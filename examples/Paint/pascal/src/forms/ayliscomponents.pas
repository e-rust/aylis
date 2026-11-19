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
  TCheckBoxStyle = (Default, Round);
  {$pop}
  TCheckBoxStyleSet = set of TCheckBoxStyle;

  { Os valores de um enum do acesso tipado. }
  {$push}{$scopedenums on}
  TCircleStyle = (Default);
  {$pop}
  TCircleStyleSet = set of TCircleStyle;

  { Os valores de um enum do acesso tipado. }
  {$push}{$scopedenums on}
  TCloseAction = (Hide, Free);
  {$pop}
  TCloseActionSet = set of TCloseAction;

  { Os valores de um enum do acesso tipado. }
  {$push}{$scopedenums on}
  TComboArrow = (Chevron, Triangle);
  {$pop}
  TComboArrowSet = set of TComboArrow;

  { Os valores de um enum do acesso tipado. }
  {$push}{$scopedenums on}
  TComboBoxStyle = (Default);
  {$pop}
  TComboBoxStyleSet = set of TComboBoxStyle;

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
  TGridLayoutStyle = (Default);
  {$pop}
  TGridLayoutStyleSet = set of TGridLayoutStyle;

  { Os valores de um enum do acesso tipado. }
  {$push}{$scopedenums on}
  TIconPosition = (Left, Right, Top);
  {$pop}
  TIconPositionSet = set of TIconPosition;

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
  TOverscroll = (Auto, Glow, Stretch, Bounce, None);
  {$pop}
  TOverscrollSet = set of TOverscroll;

  { Os valores de um enum do acesso tipado. }
  {$push}{$scopedenums on}
  TPaintCanvasStyle = (Default);
  {$pop}
  TPaintCanvasStyleSet = set of TPaintCanvasStyle;

  { Os valores de um enum do acesso tipado. }
  {$push}{$scopedenums on}
  TPaintTool = (Pencil, Brush, Eraser, Line, Rectangle, Ellipse, Fill, Picker, Select);
  {$pop}
  TPaintToolSet = set of TPaintTool;

  { Os valores de um enum do acesso tipado. }
  {$push}{$scopedenums on}
  TPanelStyle = (Default, Appbar, Card, Filled, Flat, Glass, Outlined, Overlay, Reveal, Sidebar);
  {$pop}
  TPanelStyleSet = set of TPanelStyle;

  { Os valores de um enum do acesso tipado. }
  {$push}{$scopedenums on}
  TScrollBoxStyle = (Default, Horizontal, Refresh);
  {$pop}
  TScrollBoxStyleSet = set of TScrollBoxStyle;

  { Os valores de um enum do acesso tipado. }
  {$push}{$scopedenums on}
  TSelfAlign = (Stretch, Start, Center, &End);
  {$pop}
  TSelfAlignSet = set of TSelfAlign;

  { Os valores de um enum do acesso tipado. }
  {$push}{$scopedenums on}
  TSmoothScroll = (Default, &On, Off);
  {$pop}
  TSmoothScrollSet = set of TSmoothScroll;

  { Os valores de um enum do acesso tipado. }
  {$push}{$scopedenums on}
  TStatusBarStyle = (Default);
  {$pop}
  TStatusBarStyleSet = set of TStatusBarStyle;

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
  PImageSize = ^TImageSize;
  PKeyEvent = ^TKeyEvent;
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
  TImageSize = record
    Width: Int64;
    Height: Int64;
  end;
  TImageSizeArray = array of TImageSize;

  { Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos). }
  TKeyEvent = record
    Key: string;
    Ctrl: Boolean;
    Shift: Boolean;
    Alt: Boolean;
  end;
  TKeyEventArray = array of TKeyEvent;

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

  { Um CheckBox de um form. }
  TCheckBox = class(TAylisComponent)
  private
    function GetText: string;
    procedure SetText(Value: string);
    function GetChecked: Boolean;
    procedure SetChecked(Value: Boolean);
    function GetBoxSize: Double;
    procedure SetBoxSize(Value: Double);
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
    function GetStyle: TCheckBoxStyle;
    procedure SetStyle(Value: TCheckBoxStyle);
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
    property Checked: Boolean read GetChecked write SetChecked;
    property BoxSize: Double read GetBoxSize write SetBoxSize;
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
    property Style: TCheckBoxStyle read GetStyle write SetStyle;
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

  { Um Circle de um form. }
  TCircle = class(TAylisComponent)
  private
    function GetFill: TAylisColor;
    procedure SetFill(Value: TAylisColor);
    function GetStroke: TAylisColor;
    procedure SetStroke(Value: TAylisColor);
    function GetStrokeWidth: Double;
    procedure SetStrokeWidth(Value: Double);
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
    function GetStyle: TCircleStyle;
    procedure SetStyle(Value: TCircleStyle);
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
    property Fill: TAylisColor read GetFill write SetFill;
    property Stroke: TAylisColor read GetStroke write SetStroke;
    property StrokeWidth: Double read GetStrokeWidth write SetStrokeWidth;
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
    property Style: TCircleStyle read GetStyle write SetStyle;
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

  { Um ComboBox de um form. }
  TComboBox = class(TAylisComponent)
  private
    function GetItems: TStringArray;
    procedure SetItems(Value: TStringArray);
    function GetValues: string;
    procedure SetValues(Value: string);
    function GetItemIndex: Int64;
    procedure SetItemIndex(Value: Int64);
    function GetHAlign: TTextAlign;
    procedure SetHAlign(Value: TTextAlign);
    function GetTrimming: TTrimming;
    procedure SetTrimming(Value: TTrimming);
    function GetArrow: TComboArrow;
    procedure SetArrow(Value: TComboArrow);
    function GetSensitive: Boolean;
    procedure SetSensitive(Value: Boolean);
    function GetGroups: TStringArray;
    procedure SetGroups(Value: TStringArray);
    function GetDisabledItems: TStringArray;
    procedure SetDisabledItems(Value: TStringArray);
    function GetDropDownCount: Int64;
    procedure SetDropDownCount(Value: Int64);
    function GetOnChange: string;
    procedure SetOnChange(Value: string);
    function GetValue: string;
    procedure SetValue(Value: string);
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
    function GetStyle: TComboBoxStyle;
    procedure SetStyle(Value: TComboBoxStyle);
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
    property Items: TStringArray read GetItems write SetItems;
    property Values: string read GetValues write SetValues;
    property ItemIndex: Int64 read GetItemIndex write SetItemIndex;
    property HAlign: TTextAlign read GetHAlign write SetHAlign;
    property Trimming: TTrimming read GetTrimming write SetTrimming;
    property Arrow: TComboArrow read GetArrow write SetArrow;
    property Sensitive: Boolean read GetSensitive write SetSensitive;
    property Groups: TStringArray read GetGroups write SetGroups;
    property DisabledItems: TStringArray read GetDisabledItems write SetDisabledItems;
    property DropDownCount: Int64 read GetDropDownCount write SetDropDownCount;
    property OnChange: string read GetOnChange write SetOnChange;
    property Value: string read GetValue write SetValue;
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
    property Style: TComboBoxStyle read GetStyle write SetStyle;
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

  { Um GridLayout de um form. }
  TGridLayout = class(TAylisComponent)
  private
    function GetColumns: Int64;
    procedure SetColumns(Value: Int64);
    function GetMinColumnWidth: Double;
    procedure SetMinColumnWidth(Value: Double);
    function GetGap: Double;
    procedure SetGap(Value: Double);
    function GetRowHeight: Double;
    procedure SetRowHeight(Value: Double);
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
    function GetStyle: TGridLayoutStyle;
    procedure SetStyle(Value: TGridLayoutStyle);
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
    property Columns: Int64 read GetColumns write SetColumns;
    property MinColumnWidth: Double read GetMinColumnWidth write SetMinColumnWidth;
    property Gap: Double read GetGap write SetGap;
    property RowHeight: Double read GetRowHeight write SetRowHeight;
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
    property Style: TGridLayoutStyle read GetStyle write SetStyle;
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

  { Um PaintCanvas de um form. }
  TPaintCanvas = class(TAylisComponent)
  private
    function GetTool: TPaintTool;
    procedure SetTool(Value: TPaintTool);
    function GetColor: TAylisColor;
    procedure SetColor(Value: TAylisColor);
    function GetBackColor: TAylisColor;
    procedure SetBackColor(Value: TAylisColor);
    function GetBrushSize: Int64;
    procedure SetBrushSize(Value: Int64);
    function GetFilled: Boolean;
    procedure SetFilled(Value: Boolean);
    function GetZoom: Double;
    procedure SetZoom(Value: Double);
    function GetOnChange: string;
    procedure SetOnChange(Value: string);
    function GetOnPick: string;
    procedure SetOnPick(Value: string);
    function GetOnCursor: string;
    procedure SetOnCursor(Value: string);
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
    function GetStyle: TPaintCanvasStyle;
    procedure SetStyle(Value: TPaintCanvasStyle);
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
    function Undo: Boolean;
    function Redo: Boolean;
    function CanUndo: Boolean;
    function CanRedo: Boolean;
    function Size: TImageSize;
    function Checksum: string;
    function Clear: Boolean;
    function Resize(AWidth: Int64; AHeight: Int64): Boolean;
    function New(AWidth: Int64; AHeight: Int64): Boolean;
    function Open(APath: string): Boolean;
    function Save(APath: string): Boolean;
    function SelectAll: Boolean;
    function HasSelection: Boolean;
    function Copy: Boolean;
    function Cut: Boolean;
    function Paste: Boolean;
    function Delete: Boolean;
    function Deselect: Boolean;
    procedure SetFocus;
    property Tool: TPaintTool read GetTool write SetTool;
    property Color: TAylisColor read GetColor write SetColor;
    property BackColor: TAylisColor read GetBackColor write SetBackColor;
    property BrushSize: Int64 read GetBrushSize write SetBrushSize;
    property Filled: Boolean read GetFilled write SetFilled;
    property Zoom: Double read GetZoom write SetZoom;
    property OnChange: string read GetOnChange write SetOnChange;
    property OnPick: string read GetOnPick write SetOnPick;
    property OnCursor: string read GetOnCursor write SetOnCursor;
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
    property Style: TPaintCanvasStyle read GetStyle write SetStyle;
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

  { Um SaveDialog de um form. }
  TSaveDialog = class(TAylisComponent)
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

  { Um ScrollBox de um form. }
  TScrollBox = class(TAylisComponent)
  private
    function GetOverscroll: TOverscroll;
    procedure SetOverscroll(Value: TOverscroll);
    function GetHorizontal: Boolean;
    procedure SetHorizontal(Value: Boolean);
    function GetPullToRefresh: Boolean;
    procedure SetPullToRefresh(Value: Boolean);
    function GetRefreshing: Boolean;
    procedure SetRefreshing(Value: Boolean);
    function GetWheelStep: Double;
    procedure SetWheelStep(Value: Double);
    function GetSmooth: TSmoothScroll;
    procedure SetSmooth(Value: TSmoothScroll);
    function GetGutter: Boolean;
    procedure SetGutter(Value: Boolean);
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
    function GetOnEndReached: string;
    procedure SetOnEndReached(Value: string);
    function GetOnRefresh: string;
    procedure SetOnRefresh(Value: string);
    function GetOnScroll: string;
    procedure SetOnScroll(Value: string);
    function GetScrollPosition: Double;
    procedure SetScrollPosition(Value: Double);
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
    function GetStyle: TScrollBoxStyle;
    procedure SetStyle(Value: TScrollBoxStyle);
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
    procedure ScrollTo(AComponent: TAylisComponent);
    function VisibleComponents(AList: TAylisComponent; AFully: Boolean): string;
    property Overscroll: TOverscroll read GetOverscroll write SetOverscroll;
    property Horizontal: Boolean read GetHorizontal write SetHorizontal;
    property PullToRefresh: Boolean read GetPullToRefresh write SetPullToRefresh;
    property Refreshing: Boolean read GetRefreshing write SetRefreshing;
    property WheelStep: Double read GetWheelStep write SetWheelStep;
    property Smooth: TSmoothScroll read GetSmooth write SetSmooth;
    property Gutter: Boolean read GetGutter write SetGutter;
    property AllowedChars: string read GetAllowedChars write SetAllowedChars;
    property BlockedChars: string read GetBlockedChars write SetBlockedChars;
    property OnlyAllowed: Boolean read GetOnlyAllowed write SetOnlyAllowed;
    property CharsOverride: Boolean read GetCharsOverride write SetCharsOverride;
    property CharMap: string read GetCharMap write SetCharMap;
    property OnEndReached: string read GetOnEndReached write SetOnEndReached;
    property OnRefresh: string read GetOnRefresh write SetOnRefresh;
    property OnScroll: string read GetOnScroll write SetOnScroll;
    property ScrollPosition: Double read GetScrollPosition write SetScrollPosition;
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
    property Style: TScrollBoxStyle read GetStyle write SetStyle;
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

  { Um StatusBar de um form. }
  TStatusBar = class(TAylisComponent)
  private
    function GetText: string;
    procedure SetText(Value: string);
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
    function GetStyle: TStatusBarStyle;
    procedure SetStyle(Value: TStatusBarStyle);
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
    property Text: string read GetText write SetText;
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
    property Style: TStatusBarStyle read GetStyle write SetStyle;
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

  { O form frmPaint: a classe base com os componentes. A do form (TFrmPaint), com os
    handlers (métodos published) e o estado, fica na unit dele. }
  TFrmPaintBase = class(TForm)
  published
    lytTopo: TLayout;
    mnuPrincipal: TMainMenu;
    mnuArquivo: TMenuItem;
    mnuNovo: TMenuItem;
    mnuAbrir: TMenuItem;
    mnuSalvar: TMenuItem;
    mnuSalvarComo: TMenuItem;
    mnuSep1: TMenuItem;
    mnuSair: TMenuItem;
    mnuEditar: TMenuItem;
    mnuDesfazer: TMenuItem;
    mnuRefazer: TMenuItem;
    mnuSep2: TMenuItem;
    mnuRecortar: TMenuItem;
    mnuCopiar: TMenuItem;
    mnuColar: TMenuItem;
    mnuExcluir: TMenuItem;
    mnuSelecionarTudo: TMenuItem;
    mnuSep3: TMenuItem;
    mnuLimpar: TMenuItem;
    mnuExibir: TMenuItem;
    mnuAmpliar: TMenuItem;
    mnuReduzir: TMenuItem;
    mnuTamanhoReal: TMenuItem;
    divTopo: TDivider;
    btnSalvar: TButton;
    btnDesfazer: TButton;
    btnRefazer: TButton;
    pnlFaixa: TPanel;
    lytSelecao: TLayout;
    lblSelecaoGrupo: TLabel;
    btnSelecionar: TButton;
    divSelecao: TDivider;
    lytFerramentas: TLayout;
    lblFerramentasGrupo: TLabel;
    grdFerramentas: TGridLayout;
    btnLapis: TButton;
    btnBalde: TButton;
    btnBorracha: TButton;
    btnContaGotas: TButton;
    divFerramentas: TDivider;
    lytPinceis: TLayout;
    lblPinceis: TLabel;
    btnPincel: TButton;
    divPinceis: TDivider;
    lytFormas: TLayout;
    lblFormas: TLabel;
    chkPreencher: TCheckBox;
    grdFormas: TGridLayout;
    btnLinha: TButton;
    btnRetangulo: TButton;
    btnElipse: TButton;
    divFormas: TDivider;
    lytTamanho: TLayout;
    lblTamanhoGrupo: TLabel;
    cbEspessura: TComboBox;
    divTamanho: TDivider;
    lytCores: TLayout;
    lblCores: TLabel;
    btnCor1: TCircle;
    btnCor2: TCircle;
    grdPaleta: TGridLayout;
    btnPaleta01: TCircle;
    btnPaleta02: TCircle;
    btnPaleta03: TCircle;
    btnPaleta04: TCircle;
    btnPaleta05: TCircle;
    btnPaleta06: TCircle;
    btnPaleta07: TCircle;
    btnPaleta08: TCircle;
    btnPaleta09: TCircle;
    btnPaleta10: TCircle;
    btnPaleta11: TCircle;
    btnPaleta12: TCircle;
    btnPaleta13: TCircle;
    btnPaleta14: TCircle;
    btnPaleta15: TCircle;
    btnPaleta16: TCircle;
    btnPaleta17: TCircle;
    btnPaleta18: TCircle;
    btnPaleta19: TCircle;
    btnPaleta20: TCircle;
    stbStatus: TStatusBar;
    lblPosicao: TLabel;
    lblTamanho: TLabel;
    lblZoom: TLabel;
    sbxArea: TScrollBox;
    pcDesenho: TPaintCanvas;
    dlgAbrir: TOpenDialog;
    dlgSalvar: TSaveDialog;
  protected
    procedure BindComponents(List: TJSONArray); override;
  public
    { O Button deste form com o nome (nil se não há). }
    function FindButton(const Nome: string): TButton;
    { O Button que disparou o evento (nil se não é um Button deste form). }
    function SenderAsButton(Sender: TAylisComponent): TButton; overload;
    function SenderAsButton(Event: TJSONObject): TButton; overload;
    { O CheckBox deste form com o nome (nil se não há). }
    function FindCheckBox(const Nome: string): TCheckBox;
    { O CheckBox que disparou o evento (nil se não é um CheckBox deste form). }
    function SenderAsCheckBox(Sender: TAylisComponent): TCheckBox; overload;
    function SenderAsCheckBox(Event: TJSONObject): TCheckBox; overload;
    { O Circle deste form com o nome (nil se não há). }
    function FindCircle(const Nome: string): TCircle;
    { O Circle que disparou o evento (nil se não é um Circle deste form). }
    function SenderAsCircle(Sender: TAylisComponent): TCircle; overload;
    function SenderAsCircle(Event: TJSONObject): TCircle; overload;
    { O ComboBox deste form com o nome (nil se não há). }
    function FindComboBox(const Nome: string): TComboBox;
    { O ComboBox que disparou o evento (nil se não é um ComboBox deste form). }
    function SenderAsComboBox(Sender: TAylisComponent): TComboBox; overload;
    function SenderAsComboBox(Event: TJSONObject): TComboBox; overload;
    { O Divider deste form com o nome (nil se não há). }
    function FindDivider(const Nome: string): TDivider;
    { O Divider que disparou o evento (nil se não é um Divider deste form). }
    function SenderAsDivider(Sender: TAylisComponent): TDivider; overload;
    function SenderAsDivider(Event: TJSONObject): TDivider; overload;
    { O GridLayout deste form com o nome (nil se não há). }
    function FindGridLayout(const Nome: string): TGridLayout;
    { O GridLayout que disparou o evento (nil se não é um GridLayout deste form). }
    function SenderAsGridLayout(Sender: TAylisComponent): TGridLayout; overload;
    function SenderAsGridLayout(Event: TJSONObject): TGridLayout; overload;
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
    { O PaintCanvas deste form com o nome (nil se não há). }
    function FindPaintCanvas(const Nome: string): TPaintCanvas;
    { O PaintCanvas que disparou o evento (nil se não é um PaintCanvas deste form). }
    function SenderAsPaintCanvas(Sender: TAylisComponent): TPaintCanvas; overload;
    function SenderAsPaintCanvas(Event: TJSONObject): TPaintCanvas; overload;
    { O Panel deste form com o nome (nil se não há). }
    function FindPanel(const Nome: string): TPanel;
    { O Panel que disparou o evento (nil se não é um Panel deste form). }
    function SenderAsPanel(Sender: TAylisComponent): TPanel; overload;
    function SenderAsPanel(Event: TJSONObject): TPanel; overload;
    { O SaveDialog deste form com o nome (nil se não há). }
    function FindSaveDialog(const Nome: string): TSaveDialog;
    { O SaveDialog que disparou o evento (nil se não é um SaveDialog deste form). }
    function SenderAsSaveDialog(Sender: TAylisComponent): TSaveDialog; overload;
    function SenderAsSaveDialog(Event: TJSONObject): TSaveDialog; overload;
    { O ScrollBox deste form com o nome (nil se não há). }
    function FindScrollBox(const Nome: string): TScrollBox;
    { O ScrollBox que disparou o evento (nil se não é um ScrollBox deste form). }
    function SenderAsScrollBox(Sender: TAylisComponent): TScrollBox; overload;
    function SenderAsScrollBox(Event: TJSONObject): TScrollBox; overload;
    { O StatusBar deste form com o nome (nil se não há). }
    function FindStatusBar(const Nome: string): TStatusBar;
    { O StatusBar que disparou o evento (nil se não é um StatusBar deste form). }
    function SenderAsStatusBar(Sender: TAylisComponent): TStatusBar; overload;
    function SenderAsStatusBar(Event: TJSONObject): TStatusBar; overload;
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
function CheckBoxStyleOf(Data: TJSONData): TCheckBoxStyle;
function CheckBoxStyleSetOf(Data: TJSONData): TCheckBoxStyleSet;
function CheckBoxStyleSetToJSON(const S: TCheckBoxStyleSet): TJSONData;
function CircleStyleOf(Data: TJSONData): TCircleStyle;
function CircleStyleSetOf(Data: TJSONData): TCircleStyleSet;
function CircleStyleSetToJSON(const S: TCircleStyleSet): TJSONData;
function CloseActionOf(Data: TJSONData): TCloseAction;
function CloseActionSetOf(Data: TJSONData): TCloseActionSet;
function CloseActionSetToJSON(const S: TCloseActionSet): TJSONData;
function ComboArrowOf(Data: TJSONData): TComboArrow;
function ComboArrowSetOf(Data: TJSONData): TComboArrowSet;
function ComboArrowSetToJSON(const S: TComboArrowSet): TJSONData;
function ComboBoxStyleOf(Data: TJSONData): TComboBoxStyle;
function ComboBoxStyleSetOf(Data: TJSONData): TComboBoxStyleSet;
function ComboBoxStyleSetToJSON(const S: TComboBoxStyleSet): TJSONData;
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
function GridLayoutStyleOf(Data: TJSONData): TGridLayoutStyle;
function GridLayoutStyleSetOf(Data: TJSONData): TGridLayoutStyleSet;
function GridLayoutStyleSetToJSON(const S: TGridLayoutStyleSet): TJSONData;
function IconPositionOf(Data: TJSONData): TIconPosition;
function IconPositionSetOf(Data: TJSONData): TIconPositionSet;
function IconPositionSetToJSON(const S: TIconPositionSet): TJSONData;
function LabelStyleOf(Data: TJSONData): TLabelStyle;
function LabelStyleSetOf(Data: TJSONData): TLabelStyleSet;
function LabelStyleSetToJSON(const S: TLabelStyleSet): TJSONData;
function LayerCachingOf(Data: TJSONData): TLayerCaching;
function LayerCachingSetOf(Data: TJSONData): TLayerCachingSet;
function LayerCachingSetToJSON(const S: TLayerCachingSet): TJSONData;
function LayoutStyleOf(Data: TJSONData): TLayoutStyle;
function LayoutStyleSetOf(Data: TJSONData): TLayoutStyleSet;
function LayoutStyleSetToJSON(const S: TLayoutStyleSet): TJSONData;
function MainMenuStyleOf(Data: TJSONData): TMainMenuStyle;
function MainMenuStyleSetOf(Data: TJSONData): TMainMenuStyleSet;
function MainMenuStyleSetToJSON(const S: TMainMenuStyleSet): TJSONData;
function ModalResultOf(Data: TJSONData): TModalResult;
function ModalResultSetOf(Data: TJSONData): TModalResultSet;
function ModalResultSetToJSON(const S: TModalResultSet): TJSONData;
function OverscrollOf(Data: TJSONData): TOverscroll;
function OverscrollSetOf(Data: TJSONData): TOverscrollSet;
function OverscrollSetToJSON(const S: TOverscrollSet): TJSONData;
function PaintCanvasStyleOf(Data: TJSONData): TPaintCanvasStyle;
function PaintCanvasStyleSetOf(Data: TJSONData): TPaintCanvasStyleSet;
function PaintCanvasStyleSetToJSON(const S: TPaintCanvasStyleSet): TJSONData;
function PaintToolOf(Data: TJSONData): TPaintTool;
function PaintToolSetOf(Data: TJSONData): TPaintToolSet;
function PaintToolSetToJSON(const S: TPaintToolSet): TJSONData;
function PanelStyleOf(Data: TJSONData): TPanelStyle;
function PanelStyleSetOf(Data: TJSONData): TPanelStyleSet;
function PanelStyleSetToJSON(const S: TPanelStyleSet): TJSONData;
function ScrollBoxStyleOf(Data: TJSONData): TScrollBoxStyle;
function ScrollBoxStyleSetOf(Data: TJSONData): TScrollBoxStyleSet;
function ScrollBoxStyleSetToJSON(const S: TScrollBoxStyleSet): TJSONData;
function SelfAlignOf(Data: TJSONData): TSelfAlign;
function SelfAlignSetOf(Data: TJSONData): TSelfAlignSet;
function SelfAlignSetToJSON(const S: TSelfAlignSet): TJSONData;
function SmoothScrollOf(Data: TJSONData): TSmoothScroll;
function SmoothScrollSetOf(Data: TJSONData): TSmoothScrollSet;
function SmoothScrollSetToJSON(const S: TSmoothScrollSet): TJSONData;
function StatusBarStyleOf(Data: TJSONData): TStatusBarStyle;
function StatusBarStyleSetOf(Data: TJSONData): TStatusBarStyleSet;
function StatusBarStyleSetToJSON(const S: TStatusBarStyleSet): TJSONData;
function TextAlignOf(Data: TJSONData): TTextAlign;
function TextAlignSetOf(Data: TJSONData): TTextAlignSet;
function TextAlignSetToJSON(const S: TTextAlignSet): TJSONData;
function TextCaseOf(Data: TJSONData): TTextCase;
function TextCaseSetOf(Data: TJSONData): TTextCaseSet;
function TextCaseSetToJSON(const S: TTextCaseSet): TJSONData;
function ThemeNameOf(Data: TJSONData): TThemeName;
function ThemeNameSetOf(Data: TJSONData): TThemeNameSet;
function ThemeNameSetToJSON(const S: TThemeNameSet): TJSONData;
function TransitionOf(Data: TJSONData): TTransition;
function TransitionSetOf(Data: TJSONData): TTransitionSet;
function TransitionSetToJSON(const S: TTransitionSet): TJSONData;
function TrimmingOf(Data: TJSONData): TTrimming;
function TrimmingSetOf(Data: TJSONData): TTrimmingSet;
function TrimmingSetToJSON(const S: TTrimmingSet): TJSONData;
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
function ImageSizeFromJSON(Data: TJSONData): TImageSize;
function ImageSizeListFromJSON(Data: TJSONData): TImageSizeArray;
function ImageSizeFromText(Data: TJSONData): TImageSize;
function ImageSizeListFromText(Data: TJSONData): TImageSizeArray;
function ImageSizeToJSON(const R: TImageSize): TJSONObject;
function ImageSizeListToJSON(const A: array of TImageSize): TJSONArray;
function ImageSizeText(const R: TImageSize): TJSONData;
function ImageSizeListText(const A: TImageSizeArray): TJSONData;
function KeyEventFromJSON(Data: TJSONData): TKeyEvent;
function KeyEventListFromJSON(Data: TJSONData): TKeyEventArray;
function KeyEventFromText(Data: TJSONData): TKeyEvent;
function KeyEventListFromText(Data: TJSONData): TKeyEventArray;
function KeyEventToJSON(const R: TKeyEvent): TJSONObject;
function KeyEventListToJSON(const A: array of TKeyEvent): TJSONArray;
function KeyEventText(const R: TKeyEvent): TJSONData;
function KeyEventListText(const A: TKeyEventArray): TJSONData;
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
  CheckBoxStyleNames: array[TCheckBoxStyle] of string = ('', 'round');

function CheckBoxStyleOf(Data: TJSONData): TCheckBoxStyle;
var
  V: TCheckBoxStyle;
begin
  Result := Low(TCheckBoxStyle);
  if (Data <> nil) and (Data.JSONType = jtString) then
    for V := Low(TCheckBoxStyle) to High(TCheckBoxStyle) do
      if CheckBoxStyleNames[V] = Data.AsString then
        Exit(V);
end;

function CheckBoxStyleSetOf(Data: TJSONData): TCheckBoxStyleSet;
var
  I: Integer;
  V: TCheckBoxStyle;
begin
  Result := [];
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  for I := 0 to TJSONArray(Data).Count - 1 do
    for V := Low(TCheckBoxStyle) to High(TCheckBoxStyle) do
      if CheckBoxStyleNames[V] = TJSONArray(Data).Strings[I] then
        Include(Result, V);
end;

function CheckBoxStyleSetToJSON(const S: TCheckBoxStyleSet): TJSONData;
var
  V: TCheckBoxStyle;
begin
  Result := TJSONArray.Create;
  for V := Low(TCheckBoxStyle) to High(TCheckBoxStyle) do
    if V in S then
      TJSONArray(Result).Add(CheckBoxStyleNames[V]);
end;

const
  CircleStyleNames: array[TCircleStyle] of string = ('');

function CircleStyleOf(Data: TJSONData): TCircleStyle;
var
  V: TCircleStyle;
begin
  Result := Low(TCircleStyle);
  if (Data <> nil) and (Data.JSONType = jtString) then
    for V := Low(TCircleStyle) to High(TCircleStyle) do
      if CircleStyleNames[V] = Data.AsString then
        Exit(V);
end;

function CircleStyleSetOf(Data: TJSONData): TCircleStyleSet;
var
  I: Integer;
  V: TCircleStyle;
begin
  Result := [];
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  for I := 0 to TJSONArray(Data).Count - 1 do
    for V := Low(TCircleStyle) to High(TCircleStyle) do
      if CircleStyleNames[V] = TJSONArray(Data).Strings[I] then
        Include(Result, V);
end;

function CircleStyleSetToJSON(const S: TCircleStyleSet): TJSONData;
var
  V: TCircleStyle;
begin
  Result := TJSONArray.Create;
  for V := Low(TCircleStyle) to High(TCircleStyle) do
    if V in S then
      TJSONArray(Result).Add(CircleStyleNames[V]);
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
  ComboArrowNames: array[TComboArrow] of string = ('chevron', 'triangle');

function ComboArrowOf(Data: TJSONData): TComboArrow;
var
  V: TComboArrow;
begin
  Result := Low(TComboArrow);
  if (Data <> nil) and (Data.JSONType = jtString) then
    for V := Low(TComboArrow) to High(TComboArrow) do
      if ComboArrowNames[V] = Data.AsString then
        Exit(V);
end;

function ComboArrowSetOf(Data: TJSONData): TComboArrowSet;
var
  I: Integer;
  V: TComboArrow;
begin
  Result := [];
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  for I := 0 to TJSONArray(Data).Count - 1 do
    for V := Low(TComboArrow) to High(TComboArrow) do
      if ComboArrowNames[V] = TJSONArray(Data).Strings[I] then
        Include(Result, V);
end;

function ComboArrowSetToJSON(const S: TComboArrowSet): TJSONData;
var
  V: TComboArrow;
begin
  Result := TJSONArray.Create;
  for V := Low(TComboArrow) to High(TComboArrow) do
    if V in S then
      TJSONArray(Result).Add(ComboArrowNames[V]);
end;

const
  ComboBoxStyleNames: array[TComboBoxStyle] of string = ('');

function ComboBoxStyleOf(Data: TJSONData): TComboBoxStyle;
var
  V: TComboBoxStyle;
begin
  Result := Low(TComboBoxStyle);
  if (Data <> nil) and (Data.JSONType = jtString) then
    for V := Low(TComboBoxStyle) to High(TComboBoxStyle) do
      if ComboBoxStyleNames[V] = Data.AsString then
        Exit(V);
end;

function ComboBoxStyleSetOf(Data: TJSONData): TComboBoxStyleSet;
var
  I: Integer;
  V: TComboBoxStyle;
begin
  Result := [];
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  for I := 0 to TJSONArray(Data).Count - 1 do
    for V := Low(TComboBoxStyle) to High(TComboBoxStyle) do
      if ComboBoxStyleNames[V] = TJSONArray(Data).Strings[I] then
        Include(Result, V);
end;

function ComboBoxStyleSetToJSON(const S: TComboBoxStyleSet): TJSONData;
var
  V: TComboBoxStyle;
begin
  Result := TJSONArray.Create;
  for V := Low(TComboBoxStyle) to High(TComboBoxStyle) do
    if V in S then
      TJSONArray(Result).Add(ComboBoxStyleNames[V]);
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
  GridLayoutStyleNames: array[TGridLayoutStyle] of string = ('');

function GridLayoutStyleOf(Data: TJSONData): TGridLayoutStyle;
var
  V: TGridLayoutStyle;
begin
  Result := Low(TGridLayoutStyle);
  if (Data <> nil) and (Data.JSONType = jtString) then
    for V := Low(TGridLayoutStyle) to High(TGridLayoutStyle) do
      if GridLayoutStyleNames[V] = Data.AsString then
        Exit(V);
end;

function GridLayoutStyleSetOf(Data: TJSONData): TGridLayoutStyleSet;
var
  I: Integer;
  V: TGridLayoutStyle;
begin
  Result := [];
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  for I := 0 to TJSONArray(Data).Count - 1 do
    for V := Low(TGridLayoutStyle) to High(TGridLayoutStyle) do
      if GridLayoutStyleNames[V] = TJSONArray(Data).Strings[I] then
        Include(Result, V);
end;

function GridLayoutStyleSetToJSON(const S: TGridLayoutStyleSet): TJSONData;
var
  V: TGridLayoutStyle;
begin
  Result := TJSONArray.Create;
  for V := Low(TGridLayoutStyle) to High(TGridLayoutStyle) do
    if V in S then
      TJSONArray(Result).Add(GridLayoutStyleNames[V]);
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
  PaintCanvasStyleNames: array[TPaintCanvasStyle] of string = ('');

function PaintCanvasStyleOf(Data: TJSONData): TPaintCanvasStyle;
var
  V: TPaintCanvasStyle;
begin
  Result := Low(TPaintCanvasStyle);
  if (Data <> nil) and (Data.JSONType = jtString) then
    for V := Low(TPaintCanvasStyle) to High(TPaintCanvasStyle) do
      if PaintCanvasStyleNames[V] = Data.AsString then
        Exit(V);
end;

function PaintCanvasStyleSetOf(Data: TJSONData): TPaintCanvasStyleSet;
var
  I: Integer;
  V: TPaintCanvasStyle;
begin
  Result := [];
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  for I := 0 to TJSONArray(Data).Count - 1 do
    for V := Low(TPaintCanvasStyle) to High(TPaintCanvasStyle) do
      if PaintCanvasStyleNames[V] = TJSONArray(Data).Strings[I] then
        Include(Result, V);
end;

function PaintCanvasStyleSetToJSON(const S: TPaintCanvasStyleSet): TJSONData;
var
  V: TPaintCanvasStyle;
begin
  Result := TJSONArray.Create;
  for V := Low(TPaintCanvasStyle) to High(TPaintCanvasStyle) do
    if V in S then
      TJSONArray(Result).Add(PaintCanvasStyleNames[V]);
end;

const
  PaintToolNames: array[TPaintTool] of string = ('pencil', 'brush', 'eraser', 'line', 'rectangle', 'ellipse', 'fill', 'picker', 'select');

function PaintToolOf(Data: TJSONData): TPaintTool;
var
  V: TPaintTool;
begin
  Result := Low(TPaintTool);
  if (Data <> nil) and (Data.JSONType = jtString) then
    for V := Low(TPaintTool) to High(TPaintTool) do
      if PaintToolNames[V] = Data.AsString then
        Exit(V);
end;

function PaintToolSetOf(Data: TJSONData): TPaintToolSet;
var
  I: Integer;
  V: TPaintTool;
begin
  Result := [];
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  for I := 0 to TJSONArray(Data).Count - 1 do
    for V := Low(TPaintTool) to High(TPaintTool) do
      if PaintToolNames[V] = TJSONArray(Data).Strings[I] then
        Include(Result, V);
end;

function PaintToolSetToJSON(const S: TPaintToolSet): TJSONData;
var
  V: TPaintTool;
begin
  Result := TJSONArray.Create;
  for V := Low(TPaintTool) to High(TPaintTool) do
    if V in S then
      TJSONArray(Result).Add(PaintToolNames[V]);
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
  ScrollBoxStyleNames: array[TScrollBoxStyle] of string = ('', 'horizontal', 'refresh');

function ScrollBoxStyleOf(Data: TJSONData): TScrollBoxStyle;
var
  V: TScrollBoxStyle;
begin
  Result := Low(TScrollBoxStyle);
  if (Data <> nil) and (Data.JSONType = jtString) then
    for V := Low(TScrollBoxStyle) to High(TScrollBoxStyle) do
      if ScrollBoxStyleNames[V] = Data.AsString then
        Exit(V);
end;

function ScrollBoxStyleSetOf(Data: TJSONData): TScrollBoxStyleSet;
var
  I: Integer;
  V: TScrollBoxStyle;
begin
  Result := [];
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  for I := 0 to TJSONArray(Data).Count - 1 do
    for V := Low(TScrollBoxStyle) to High(TScrollBoxStyle) do
      if ScrollBoxStyleNames[V] = TJSONArray(Data).Strings[I] then
        Include(Result, V);
end;

function ScrollBoxStyleSetToJSON(const S: TScrollBoxStyleSet): TJSONData;
var
  V: TScrollBoxStyle;
begin
  Result := TJSONArray.Create;
  for V := Low(TScrollBoxStyle) to High(TScrollBoxStyle) do
    if V in S then
      TJSONArray(Result).Add(ScrollBoxStyleNames[V]);
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
  SmoothScrollNames: array[TSmoothScroll] of string = ('default', 'on', 'off');

function SmoothScrollOf(Data: TJSONData): TSmoothScroll;
var
  V: TSmoothScroll;
begin
  Result := Low(TSmoothScroll);
  if (Data <> nil) and (Data.JSONType = jtString) then
    for V := Low(TSmoothScroll) to High(TSmoothScroll) do
      if SmoothScrollNames[V] = Data.AsString then
        Exit(V);
end;

function SmoothScrollSetOf(Data: TJSONData): TSmoothScrollSet;
var
  I: Integer;
  V: TSmoothScroll;
begin
  Result := [];
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  for I := 0 to TJSONArray(Data).Count - 1 do
    for V := Low(TSmoothScroll) to High(TSmoothScroll) do
      if SmoothScrollNames[V] = TJSONArray(Data).Strings[I] then
        Include(Result, V);
end;

function SmoothScrollSetToJSON(const S: TSmoothScrollSet): TJSONData;
var
  V: TSmoothScroll;
begin
  Result := TJSONArray.Create;
  for V := Low(TSmoothScroll) to High(TSmoothScroll) do
    if V in S then
      TJSONArray(Result).Add(SmoothScrollNames[V]);
end;

const
  StatusBarStyleNames: array[TStatusBarStyle] of string = ('');

function StatusBarStyleOf(Data: TJSONData): TStatusBarStyle;
var
  V: TStatusBarStyle;
begin
  Result := Low(TStatusBarStyle);
  if (Data <> nil) and (Data.JSONType = jtString) then
    for V := Low(TStatusBarStyle) to High(TStatusBarStyle) do
      if StatusBarStyleNames[V] = Data.AsString then
        Exit(V);
end;

function StatusBarStyleSetOf(Data: TJSONData): TStatusBarStyleSet;
var
  I: Integer;
  V: TStatusBarStyle;
begin
  Result := [];
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  for I := 0 to TJSONArray(Data).Count - 1 do
    for V := Low(TStatusBarStyle) to High(TStatusBarStyle) do
      if StatusBarStyleNames[V] = TJSONArray(Data).Strings[I] then
        Include(Result, V);
end;

function StatusBarStyleSetToJSON(const S: TStatusBarStyleSet): TJSONData;
var
  V: TStatusBarStyle;
begin
  Result := TJSONArray.Create;
  for V := Low(TStatusBarStyle) to High(TStatusBarStyle) do
    if V in S then
      TJSONArray(Result).Add(StatusBarStyleNames[V]);
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

function ImageSizeFromJSON(Data: TJSONData): TImageSize;
var
  O: TJSONObject;
begin
  Result := Default(TImageSize);
  if (Data = nil) or (Data.JSONType <> jtObject) then
    Exit;
  O := TJSONObject(Data);
  Result.Width := O.Get('width', Int64(0));
  Result.Height := O.Get('height', Int64(0));
end;

function ImageSizeToJSON(const R: TImageSize): TJSONObject;
begin
  Result := TJSONObject.Create;
  Result.Add('width', R.Width);
  Result.Add('height', R.Height);
end;

function ImageSizeListFromJSON(Data: TJSONData): TImageSizeArray;
var
  I: Integer;
begin
  Result := nil;
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  SetLength(Result, Data.Count);
  for I := 0 to Data.Count - 1 do
    Result[I] := ImageSizeFromJSON(Data.Items[I]);
end;

function ImageSizeListToJSON(const A: array of TImageSize): TJSONArray;
var
  I: Integer;
begin
  Result := TJSONArray.Create;
  for I := 0 to High(A) do
    Result.Add(ImageSizeToJSON(A[I]));
end;

function ImageSizeText(const R: TImageSize): TJSONData;
var
  J: TJSONObject;
begin
  J := ImageSizeToJSON(R);
  try
    Result := TJSONString.Create(J.AsJSON);
  finally
    J.Free;
  end;
end;

function ImageSizeListText(const A: TImageSizeArray): TJSONData;
var
  J: TJSONArray;
begin
  J := ImageSizeListToJSON(A);
  try
    Result := TJSONString.Create(J.AsJSON);
  finally
    J.Free;
  end;
end;

function ImageSizeFromText(Data: TJSONData): TImageSize;
var
  J: TJSONData;
begin
  try
    J := GetJSON(AsText(Data));
  except
    J := nil;
  end;
  try
    Result := ImageSizeFromJSON(J);
  finally
    J.Free;
  end;
end;

function ImageSizeListFromText(Data: TJSONData): TImageSizeArray;
var
  J: TJSONData;
begin
  try
    J := GetJSON(AsText(Data));
  except
    J := nil;
  end;
  try
    Result := ImageSizeListFromJSON(J);
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

function TCheckBox.GetText: string;
begin
  Result := AsText(GetProp('text'));
end;

procedure TCheckBox.SetText(Value: string);
begin
  SetProp('text', TJSONString.Create(Value));
end;

function TCheckBox.GetChecked: Boolean;
begin
  Result := GetProp('checked').AsBoolean;
end;

procedure TCheckBox.SetChecked(Value: Boolean);
begin
  SetProp('checked', TJSONBoolean.Create(Value));
end;

function TCheckBox.GetBoxSize: Double;
begin
  Result := GetProp('box_size').AsFloat;
end;

procedure TCheckBox.SetBoxSize(Value: Double);
begin
  SetProp('box_size', TJSONFloatNumber.Create(Value));
end;

function TCheckBox.GetOnChange: string;
begin
  Result := AsText(GetProp('on_change'));
end;

procedure TCheckBox.SetOnChange(Value: string);
begin
  SetProp('on_change', TJSONString.Create(Value));
end;

function TCheckBox.GetLeft: Double;
begin
  Result := GetProp('left').AsFloat;
end;

procedure TCheckBox.SetLeft(Value: Double);
begin
  SetProp('left', TJSONFloatNumber.Create(Value));
end;

function TCheckBox.GetTop: Double;
begin
  Result := GetProp('top').AsFloat;
end;

procedure TCheckBox.SetTop(Value: Double);
begin
  SetProp('top', TJSONFloatNumber.Create(Value));
end;

function TCheckBox.GetWidth: Double;
begin
  Result := GetProp('width').AsFloat;
end;

procedure TCheckBox.SetWidth(Value: Double);
begin
  SetProp('width', TJSONFloatNumber.Create(Value));
end;

function TCheckBox.GetHeight: Double;
begin
  Result := GetProp('height').AsFloat;
end;

procedure TCheckBox.SetHeight(Value: Double);
begin
  SetProp('height', TJSONFloatNumber.Create(Value));
end;

function TCheckBox.GetAlign: TAlign;
begin
  Result := AlignOf(GetProp('align'));
end;

procedure TCheckBox.SetAlign(Value: TAlign);
begin
  SetProp('align', TJSONString.Create(AlignNames[Value]));
end;

function TCheckBox.GetAnchors: TAnchorSet;
begin
  Result := AnchorSetOf(GetProp('anchors'));
end;

procedure TCheckBox.SetAnchors(Value: TAnchorSet);
begin
  SetProp('anchors', AnchorSetToJSON(Value));
end;

function TCheckBox.GetMargins: TAylisBounds;
begin
  Result := AylisBoundsFromJSON(GetProp('margins'));
end;

procedure TCheckBox.SetMargins(Value: TAylisBounds);
begin
  SetProp('margins', AylisBoundsToJSON(Value));
end;

function TCheckBox.GetPadding: TAylisBounds;
begin
  Result := AylisBoundsFromJSON(GetProp('padding'));
end;

procedure TCheckBox.SetPadding(Value: TAylisBounds);
begin
  SetProp('padding', AylisBoundsToJSON(Value));
end;

function TCheckBox.GetVisible: Boolean;
begin
  Result := GetProp('visible').AsBoolean;
end;

procedure TCheckBox.SetVisible(Value: Boolean);
begin
  SetProp('visible', TJSONBoolean.Create(Value));
end;

function TCheckBox.GetEnabled: Boolean;
begin
  Result := GetProp('enabled').AsBoolean;
end;

procedure TCheckBox.SetEnabled(Value: Boolean);
begin
  SetProp('enabled', TJSONBoolean.Create(Value));
end;

function TCheckBox.GetHitTest: Boolean;
begin
  Result := GetProp('hit_test').AsBoolean;
end;

procedure TCheckBox.SetHitTest(Value: Boolean);
begin
  SetProp('hit_test', TJSONBoolean.Create(Value));
end;

function TCheckBox.GetClipChildren: Boolean;
begin
  Result := GetProp('clip_children').AsBoolean;
end;

procedure TCheckBox.SetClipChildren(Value: Boolean);
begin
  SetProp('clip_children', TJSONBoolean.Create(Value));
end;

function TCheckBox.GetInheritState: Boolean;
begin
  Result := GetProp('inherit_state').AsBoolean;
end;

procedure TCheckBox.SetInheritState(Value: Boolean);
begin
  SetProp('inherit_state', TJSONBoolean.Create(Value));
end;

function TCheckBox.GetTranslate: Boolean;
begin
  Result := GetProp('translate').AsBoolean;
end;

procedure TCheckBox.SetTranslate(Value: Boolean);
begin
  SetProp('translate', TJSONBoolean.Create(Value));
end;

function TCheckBox.GetCapturesWheel: Boolean;
begin
  Result := GetProp('captures_wheel').AsBoolean;
end;

procedure TCheckBox.SetCapturesWheel(Value: Boolean);
begin
  SetProp('captures_wheel', TJSONBoolean.Create(Value));
end;

function TCheckBox.GetCapturesGesture: Boolean;
begin
  Result := GetProp('captures_gesture').AsBoolean;
end;

procedure TCheckBox.SetCapturesGesture(Value: Boolean);
begin
  SetProp('captures_gesture', TJSONBoolean.Create(Value));
end;

function TCheckBox.GetCanFocus: Boolean;
begin
  Result := GetProp('can_focus').AsBoolean;
end;

procedure TCheckBox.SetCanFocus(Value: Boolean);
begin
  SetProp('can_focus', TJSONBoolean.Create(Value));
end;

function TCheckBox.GetDraggable: Boolean;
begin
  Result := GetProp('draggable').AsBoolean;
end;

procedure TCheckBox.SetDraggable(Value: Boolean);
begin
  SetProp('draggable', TJSONBoolean.Create(Value));
end;

function TCheckBox.GetAcceptsDrops: Boolean;
begin
  Result := GetProp('accepts_drops').AsBoolean;
end;

procedure TCheckBox.SetAcceptsDrops(Value: Boolean);
begin
  SetProp('accepts_drops', TJSONBoolean.Create(Value));
end;

function TCheckBox.GetDragData: string;
begin
  Result := AsText(GetProp('drag_data'));
end;

procedure TCheckBox.SetDragData(Value: string);
begin
  SetProp('drag_data', TJSONString.Create(Value));
end;

function TCheckBox.GetDragLabel: string;
begin
  Result := AsText(GetProp('drag_label'));
end;

procedure TCheckBox.SetDragLabel(Value: string);
begin
  SetProp('drag_label', TJSONString.Create(Value));
end;

function TCheckBox.GetDragImage: TAylisComponent;
begin
  Result := GetRef('drag_image');
end;

procedure TCheckBox.SetDragImage(Value: TAylisComponent);
begin
  SetRef('drag_image', Value);
end;

function TCheckBox.GetDropTypes: string;
begin
  Result := AsText(GetProp('drop_types'));
end;

procedure TCheckBox.SetDropTypes(Value: string);
begin
  SetProp('drop_types', TJSONString.Create(Value));
end;

function TCheckBox.GetTabStop: Boolean;
begin
  Result := GetProp('tab_stop').AsBoolean;
end;

procedure TCheckBox.SetTabStop(Value: Boolean);
begin
  SetProp('tab_stop', TJSONBoolean.Create(Value));
end;

function TCheckBox.GetCursor: TCursorShape;
begin
  Result := CursorShapeOf(GetProp('cursor'));
end;

procedure TCheckBox.SetCursor(Value: TCursorShape);
begin
  SetProp('cursor', TJSONString.Create(CursorShapeNames[Value]));
end;

function TCheckBox.GetOpacity: Double;
begin
  Result := GetProp('opacity').AsFloat;
end;

procedure TCheckBox.SetOpacity(Value: Double);
begin
  SetProp('opacity', TJSONFloatNumber.Create(Value));
end;

function TCheckBox.GetBlendMode: TBlendMode;
begin
  Result := BlendModeOf(GetProp('blend_mode'));
end;

procedure TCheckBox.SetBlendMode(Value: TBlendMode);
begin
  SetProp('blend_mode', TJSONString.Create(BlendModeNames[Value]));
end;

function TCheckBox.GetCacheLayer: Boolean;
begin
  Result := GetProp('cache_layer').AsBoolean;
end;

procedure TCheckBox.SetCacheLayer(Value: Boolean);
begin
  SetProp('cache_layer', TJSONBoolean.Create(Value));
end;

function TCheckBox.GetRotation: Double;
begin
  Result := GetProp('rotation').AsFloat;
end;

procedure TCheckBox.SetRotation(Value: Double);
begin
  SetProp('rotation', TJSONFloatNumber.Create(Value));
end;

function TCheckBox.GetScale: Double;
begin
  Result := GetProp('scale').AsFloat;
end;

procedure TCheckBox.SetScale(Value: Double);
begin
  SetProp('scale', TJSONFloatNumber.Create(Value));
end;

function TCheckBox.GetTranslateX: Double;
begin
  Result := GetProp('translate_x').AsFloat;
end;

procedure TCheckBox.SetTranslateX(Value: Double);
begin
  SetProp('translate_x', TJSONFloatNumber.Create(Value));
end;

function TCheckBox.GetTranslateY: Double;
begin
  Result := GetProp('translate_y').AsFloat;
end;

procedure TCheckBox.SetTranslateY(Value: Double);
begin
  SetProp('translate_y', TJSONFloatNumber.Create(Value));
end;

function TCheckBox.GetAspectRatio: Double;
begin
  Result := GetProp('aspect_ratio').AsFloat;
end;

procedure TCheckBox.SetAspectRatio(Value: Double);
begin
  SetProp('aspect_ratio', TJSONFloatNumber.Create(Value));
end;

function TCheckBox.GetMaxWidth: Double;
begin
  Result := GetProp('max_width').AsFloat;
end;

procedure TCheckBox.SetMaxWidth(Value: Double);
begin
  SetProp('max_width', TJSONFloatNumber.Create(Value));
end;

function TCheckBox.GetMinWidth: Double;
begin
  Result := GetProp('min_width').AsFloat;
end;

procedure TCheckBox.SetMinWidth(Value: Double);
begin
  SetProp('min_width', TJSONFloatNumber.Create(Value));
end;

function TCheckBox.GetMinHeight: Double;
begin
  Result := GetProp('min_height').AsFloat;
end;

procedure TCheckBox.SetMinHeight(Value: Double);
begin
  SetProp('min_height', TJSONFloatNumber.Create(Value));
end;

function TCheckBox.GetMaxHeight: Double;
begin
  Result := GetProp('max_height').AsFloat;
end;

procedure TCheckBox.SetMaxHeight(Value: Double);
begin
  SetProp('max_height', TJSONFloatNumber.Create(Value));
end;

function TCheckBox.GetSelfAlign: TSelfAlign;
begin
  Result := SelfAlignOf(GetProp('self_align'));
end;

procedure TCheckBox.SetSelfAlign(Value: TSelfAlign);
begin
  SetProp('self_align', TJSONString.Create(SelfAlignNames[Value]));
end;

function TCheckBox.GetAutoSize: Boolean;
begin
  Result := GetProp('auto_size').AsBoolean;
end;

procedure TCheckBox.SetAutoSize(Value: Boolean);
begin
  SetProp('auto_size', TJSONBoolean.Create(Value));
end;

function TCheckBox.GetColumnSpan: Int64;
begin
  Result := GetProp('column_span').AsInt64;
end;

procedure TCheckBox.SetColumnSpan(Value: Int64);
begin
  SetProp('column_span', TJSONInt64Number.Create(Value));
end;

function TCheckBox.GetGridRow: Int64;
begin
  Result := GetProp('grid_row').AsInt64;
end;

procedure TCheckBox.SetGridRow(Value: Int64);
begin
  SetProp('grid_row', TJSONInt64Number.Create(Value));
end;

function TCheckBox.GetGridColumn: Int64;
begin
  Result := GetProp('grid_column').AsInt64;
end;

procedure TCheckBox.SetGridColumn(Value: Int64);
begin
  SetProp('grid_column', TJSONInt64Number.Create(Value));
end;

function TCheckBox.GetRowSpan: Int64;
begin
  Result := GetProp('row_span').AsInt64;
end;

procedure TCheckBox.SetRowSpan(Value: Int64);
begin
  SetProp('row_span', TJSONInt64Number.Create(Value));
end;

function TCheckBox.GetGrow: Double;
begin
  Result := GetProp('grow').AsFloat;
end;

procedure TCheckBox.SetGrow(Value: Double);
begin
  SetProp('grow', TJSONFloatNumber.Create(Value));
end;

function TCheckBox.GetShadow: TAylisShadow;
begin
  Result := AylisShadowFromJSON(GetProp('shadow'));
end;

procedure TCheckBox.SetShadow(Value: TAylisShadow);
begin
  SetProp('shadow', AylisShadowToJSON(Value));
end;

function TCheckBox.GetStyle: TCheckBoxStyle;
begin
  Result := CheckBoxStyleOf(GetProp('style'));
end;

procedure TCheckBox.SetStyle(Value: TCheckBoxStyle);
begin
  SetProp('style', TJSONString.Create(CheckBoxStyleNames[Value]));
end;

function TCheckBox.GetTheme: TThemeName;
begin
  Result := ThemeNameOf(GetProp('theme'));
end;

procedure TCheckBox.SetTheme(Value: TThemeName);
begin
  SetProp('theme', TJSONString.Create(ThemeNameNames[Value]));
end;

function TCheckBox.GetFill: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('fill'));
end;

procedure TCheckBox.SetFill(Value: TAylisColor);
begin
  SetProp('fill', AylisColorToJSON(Value));
end;

function TCheckBox.GetBorderColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('border_color'));
end;

procedure TCheckBox.SetBorderColor(Value: TAylisColor);
begin
  SetProp('border_color', AylisColorToJSON(Value));
end;

function TCheckBox.GetTextColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('text_color'));
end;

procedure TCheckBox.SetTextColor(Value: TAylisColor);
begin
  SetProp('text_color', AylisColorToJSON(Value));
end;

function TCheckBox.GetAccentColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('accent_color'));
end;

procedure TCheckBox.SetAccentColor(Value: TAylisColor);
begin
  SetProp('accent_color', AylisColorToJSON(Value));
end;

function TCheckBox.GetSuccessColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('success_color'));
end;

procedure TCheckBox.SetSuccessColor(Value: TAylisColor);
begin
  SetProp('success_color', AylisColorToJSON(Value));
end;

function TCheckBox.GetWarningColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('warning_color'));
end;

procedure TCheckBox.SetWarningColor(Value: TAylisColor);
begin
  SetProp('warning_color', AylisColorToJSON(Value));
end;

function TCheckBox.GetDangerColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('danger_color'));
end;

procedure TCheckBox.SetDangerColor(Value: TAylisColor);
begin
  SetProp('danger_color', AylisColorToJSON(Value));
end;

function TCheckBox.GetFontFamily: string;
begin
  Result := AsText(GetProp('font_family'));
end;

procedure TCheckBox.SetFontFamily(Value: string);
begin
  SetProp('font_family', TJSONString.Create(Value));
end;

function TCheckBox.GetFontSize: Double;
begin
  Result := GetProp('font_size').AsFloat;
end;

procedure TCheckBox.SetFontSize(Value: Double);
begin
  SetProp('font_size', TJSONFloatNumber.Create(Value));
end;

function TCheckBox.GetFontWeight: TFontWeight;
begin
  Result := FontWeightOf(GetProp('font_weight'));
end;

procedure TCheckBox.SetFontWeight(Value: TFontWeight);
begin
  SetProp('font_weight', TJSONString.Create(FontWeightNames[Value]));
end;

function TCheckBox.GetFontItalic: Boolean;
begin
  Result := GetProp('font_italic').AsBoolean;
end;

procedure TCheckBox.SetFontItalic(Value: Boolean);
begin
  SetProp('font_italic', TJSONBoolean.Create(Value));
end;

function TCheckBox.GetFontUnderline: Boolean;
begin
  Result := GetProp('font_underline').AsBoolean;
end;

procedure TCheckBox.SetFontUnderline(Value: Boolean);
begin
  SetProp('font_underline', TJSONBoolean.Create(Value));
end;

function TCheckBox.GetFontStrikeout: Boolean;
begin
  Result := GetProp('font_strikeout').AsBoolean;
end;

procedure TCheckBox.SetFontStrikeout(Value: Boolean);
begin
  SetProp('font_strikeout', TJSONBoolean.Create(Value));
end;

function TCheckBox.GetHint: string;
begin
  Result := AsText(GetProp('hint'));
end;

procedure TCheckBox.SetHint(Value: string);
begin
  SetProp('hint', TJSONString.Create(Value));
end;

function TCheckBox.GetTag: Int64;
begin
  Result := GetProp('tag').AsInt64;
end;

procedure TCheckBox.SetTag(Value: Int64);
begin
  SetProp('tag', TJSONInt64Number.Create(Value));
end;

function TCheckBox.GetTagString: string;
begin
  Result := AsText(GetProp('tag_string'));
end;

procedure TCheckBox.SetTagString(Value: string);
begin
  SetProp('tag_string', TJSONString.Create(Value));
end;

function TCheckBox.GetAccessibleName: string;
begin
  Result := AsText(GetProp('accessible_name'));
end;

procedure TCheckBox.SetAccessibleName(Value: string);
begin
  SetProp('accessible_name', TJSONString.Create(Value));
end;

function TCheckBox.GetPopupMenu: TAylisComponent;
begin
  Result := GetRef('popup_menu');
end;

procedure TCheckBox.SetPopupMenu(Value: TAylisComponent);
begin
  SetRef('popup_menu', Value);
end;

function TCheckBox.GetOnClick: string;
begin
  Result := AsText(GetProp('on_click'));
end;

procedure TCheckBox.SetOnClick(Value: string);
begin
  SetProp('on_click', TJSONString.Create(Value));
end;

function TCheckBox.GetOnDblClick: string;
begin
  Result := AsText(GetProp('on_dbl_click'));
end;

procedure TCheckBox.SetOnDblClick(Value: string);
begin
  SetProp('on_dbl_click', TJSONString.Create(Value));
end;

function TCheckBox.GetOnLongPress: string;
begin
  Result := AsText(GetProp('on_long_press'));
end;

procedure TCheckBox.SetOnLongPress(Value: string);
begin
  SetProp('on_long_press', TJSONString.Create(Value));
end;

function TCheckBox.GetOnPointerDown: string;
begin
  Result := AsText(GetProp('on_pointer_down'));
end;

procedure TCheckBox.SetOnPointerDown(Value: string);
begin
  SetProp('on_pointer_down', TJSONString.Create(Value));
end;

function TCheckBox.GetOnPointerUp: string;
begin
  Result := AsText(GetProp('on_pointer_up'));
end;

procedure TCheckBox.SetOnPointerUp(Value: string);
begin
  SetProp('on_pointer_up', TJSONString.Create(Value));
end;

function TCheckBox.GetOnPointerMove: string;
begin
  Result := AsText(GetProp('on_pointer_move'));
end;

procedure TCheckBox.SetOnPointerMove(Value: string);
begin
  SetProp('on_pointer_move', TJSONString.Create(Value));
end;

function TCheckBox.GetOnPointerEnter: string;
begin
  Result := AsText(GetProp('on_pointer_enter'));
end;

procedure TCheckBox.SetOnPointerEnter(Value: string);
begin
  SetProp('on_pointer_enter', TJSONString.Create(Value));
end;

function TCheckBox.GetOnPointerLeave: string;
begin
  Result := AsText(GetProp('on_pointer_leave'));
end;

procedure TCheckBox.SetOnPointerLeave(Value: string);
begin
  SetProp('on_pointer_leave', TJSONString.Create(Value));
end;

function TCheckBox.GetOnWheel: string;
begin
  Result := AsText(GetProp('on_wheel'));
end;

procedure TCheckBox.SetOnWheel(Value: string);
begin
  SetProp('on_wheel', TJSONString.Create(Value));
end;

function TCheckBox.GetOnGesture: string;
begin
  Result := AsText(GetProp('on_gesture'));
end;

procedure TCheckBox.SetOnGesture(Value: string);
begin
  SetProp('on_gesture', TJSONString.Create(Value));
end;

function TCheckBox.GetOnSwipe: string;
begin
  Result := AsText(GetProp('on_swipe'));
end;

procedure TCheckBox.SetOnSwipe(Value: string);
begin
  SetProp('on_swipe', TJSONString.Create(Value));
end;

function TCheckBox.GetOnKeyUp: string;
begin
  Result := AsText(GetProp('on_key_up'));
end;

procedure TCheckBox.SetOnKeyUp(Value: string);
begin
  SetProp('on_key_up', TJSONString.Create(Value));
end;

function TCheckBox.GetOnEnter: string;
begin
  Result := AsText(GetProp('on_enter'));
end;

procedure TCheckBox.SetOnEnter(Value: string);
begin
  SetProp('on_enter', TJSONString.Create(Value));
end;

function TCheckBox.GetOnExit: string;
begin
  Result := AsText(GetProp('on_exit'));
end;

procedure TCheckBox.SetOnExit(Value: string);
begin
  SetProp('on_exit', TJSONString.Create(Value));
end;

function TCheckBox.GetOnResize: string;
begin
  Result := AsText(GetProp('on_resize'));
end;

procedure TCheckBox.SetOnResize(Value: string);
begin
  SetProp('on_resize', TJSONString.Create(Value));
end;

function TCheckBox.GetOnDragMove: string;
begin
  Result := AsText(GetProp('on_drag_move'));
end;

procedure TCheckBox.SetOnDragMove(Value: string);
begin
  SetProp('on_drag_move', TJSONString.Create(Value));
end;

function TCheckBox.GetOnDragLeave: string;
begin
  Result := AsText(GetProp('on_drag_leave'));
end;

procedure TCheckBox.SetOnDragLeave(Value: string);
begin
  SetProp('on_drag_leave', TJSONString.Create(Value));
end;

function TCheckBox.GetOnDragDrop: string;
begin
  Result := AsText(GetProp('on_drag_drop'));
end;

procedure TCheckBox.SetOnDragDrop(Value: string);
begin
  SetProp('on_drag_drop', TJSONString.Create(Value));
end;

function TCheckBox.GetOnDragEnd: string;
begin
  Result := AsText(GetProp('on_drag_end'));
end;

procedure TCheckBox.SetOnDragEnd(Value: string);
begin
  SetProp('on_drag_end', TJSONString.Create(Value));
end;

procedure TCheckBox.SetFocus;
begin
  Aylis.Send('focus', TJSONObject.Create(['target', Id]));
end;

function TCircle.GetFill: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('fill'));
end;

procedure TCircle.SetFill(Value: TAylisColor);
begin
  SetProp('fill', AylisColorToJSON(Value));
end;

function TCircle.GetStroke: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('stroke'));
end;

procedure TCircle.SetStroke(Value: TAylisColor);
begin
  SetProp('stroke', AylisColorToJSON(Value));
end;

function TCircle.GetStrokeWidth: Double;
begin
  Result := GetProp('stroke_width').AsFloat;
end;

procedure TCircle.SetStrokeWidth(Value: Double);
begin
  SetProp('stroke_width', TJSONFloatNumber.Create(Value));
end;

function TCircle.GetLeft: Double;
begin
  Result := GetProp('left').AsFloat;
end;

procedure TCircle.SetLeft(Value: Double);
begin
  SetProp('left', TJSONFloatNumber.Create(Value));
end;

function TCircle.GetTop: Double;
begin
  Result := GetProp('top').AsFloat;
end;

procedure TCircle.SetTop(Value: Double);
begin
  SetProp('top', TJSONFloatNumber.Create(Value));
end;

function TCircle.GetWidth: Double;
begin
  Result := GetProp('width').AsFloat;
end;

procedure TCircle.SetWidth(Value: Double);
begin
  SetProp('width', TJSONFloatNumber.Create(Value));
end;

function TCircle.GetHeight: Double;
begin
  Result := GetProp('height').AsFloat;
end;

procedure TCircle.SetHeight(Value: Double);
begin
  SetProp('height', TJSONFloatNumber.Create(Value));
end;

function TCircle.GetAlign: TAlign;
begin
  Result := AlignOf(GetProp('align'));
end;

procedure TCircle.SetAlign(Value: TAlign);
begin
  SetProp('align', TJSONString.Create(AlignNames[Value]));
end;

function TCircle.GetAnchors: TAnchorSet;
begin
  Result := AnchorSetOf(GetProp('anchors'));
end;

procedure TCircle.SetAnchors(Value: TAnchorSet);
begin
  SetProp('anchors', AnchorSetToJSON(Value));
end;

function TCircle.GetMargins: TAylisBounds;
begin
  Result := AylisBoundsFromJSON(GetProp('margins'));
end;

procedure TCircle.SetMargins(Value: TAylisBounds);
begin
  SetProp('margins', AylisBoundsToJSON(Value));
end;

function TCircle.GetPadding: TAylisBounds;
begin
  Result := AylisBoundsFromJSON(GetProp('padding'));
end;

procedure TCircle.SetPadding(Value: TAylisBounds);
begin
  SetProp('padding', AylisBoundsToJSON(Value));
end;

function TCircle.GetVisible: Boolean;
begin
  Result := GetProp('visible').AsBoolean;
end;

procedure TCircle.SetVisible(Value: Boolean);
begin
  SetProp('visible', TJSONBoolean.Create(Value));
end;

function TCircle.GetEnabled: Boolean;
begin
  Result := GetProp('enabled').AsBoolean;
end;

procedure TCircle.SetEnabled(Value: Boolean);
begin
  SetProp('enabled', TJSONBoolean.Create(Value));
end;

function TCircle.GetHitTest: Boolean;
begin
  Result := GetProp('hit_test').AsBoolean;
end;

procedure TCircle.SetHitTest(Value: Boolean);
begin
  SetProp('hit_test', TJSONBoolean.Create(Value));
end;

function TCircle.GetClipChildren: Boolean;
begin
  Result := GetProp('clip_children').AsBoolean;
end;

procedure TCircle.SetClipChildren(Value: Boolean);
begin
  SetProp('clip_children', TJSONBoolean.Create(Value));
end;

function TCircle.GetInheritState: Boolean;
begin
  Result := GetProp('inherit_state').AsBoolean;
end;

procedure TCircle.SetInheritState(Value: Boolean);
begin
  SetProp('inherit_state', TJSONBoolean.Create(Value));
end;

function TCircle.GetTranslate: Boolean;
begin
  Result := GetProp('translate').AsBoolean;
end;

procedure TCircle.SetTranslate(Value: Boolean);
begin
  SetProp('translate', TJSONBoolean.Create(Value));
end;

function TCircle.GetCapturesWheel: Boolean;
begin
  Result := GetProp('captures_wheel').AsBoolean;
end;

procedure TCircle.SetCapturesWheel(Value: Boolean);
begin
  SetProp('captures_wheel', TJSONBoolean.Create(Value));
end;

function TCircle.GetCapturesGesture: Boolean;
begin
  Result := GetProp('captures_gesture').AsBoolean;
end;

procedure TCircle.SetCapturesGesture(Value: Boolean);
begin
  SetProp('captures_gesture', TJSONBoolean.Create(Value));
end;

function TCircle.GetCanFocus: Boolean;
begin
  Result := GetProp('can_focus').AsBoolean;
end;

procedure TCircle.SetCanFocus(Value: Boolean);
begin
  SetProp('can_focus', TJSONBoolean.Create(Value));
end;

function TCircle.GetDraggable: Boolean;
begin
  Result := GetProp('draggable').AsBoolean;
end;

procedure TCircle.SetDraggable(Value: Boolean);
begin
  SetProp('draggable', TJSONBoolean.Create(Value));
end;

function TCircle.GetAcceptsDrops: Boolean;
begin
  Result := GetProp('accepts_drops').AsBoolean;
end;

procedure TCircle.SetAcceptsDrops(Value: Boolean);
begin
  SetProp('accepts_drops', TJSONBoolean.Create(Value));
end;

function TCircle.GetDragData: string;
begin
  Result := AsText(GetProp('drag_data'));
end;

procedure TCircle.SetDragData(Value: string);
begin
  SetProp('drag_data', TJSONString.Create(Value));
end;

function TCircle.GetDragLabel: string;
begin
  Result := AsText(GetProp('drag_label'));
end;

procedure TCircle.SetDragLabel(Value: string);
begin
  SetProp('drag_label', TJSONString.Create(Value));
end;

function TCircle.GetDragImage: TAylisComponent;
begin
  Result := GetRef('drag_image');
end;

procedure TCircle.SetDragImage(Value: TAylisComponent);
begin
  SetRef('drag_image', Value);
end;

function TCircle.GetDropTypes: string;
begin
  Result := AsText(GetProp('drop_types'));
end;

procedure TCircle.SetDropTypes(Value: string);
begin
  SetProp('drop_types', TJSONString.Create(Value));
end;

function TCircle.GetTabStop: Boolean;
begin
  Result := GetProp('tab_stop').AsBoolean;
end;

procedure TCircle.SetTabStop(Value: Boolean);
begin
  SetProp('tab_stop', TJSONBoolean.Create(Value));
end;

function TCircle.GetCursor: TCursorShape;
begin
  Result := CursorShapeOf(GetProp('cursor'));
end;

procedure TCircle.SetCursor(Value: TCursorShape);
begin
  SetProp('cursor', TJSONString.Create(CursorShapeNames[Value]));
end;

function TCircle.GetOpacity: Double;
begin
  Result := GetProp('opacity').AsFloat;
end;

procedure TCircle.SetOpacity(Value: Double);
begin
  SetProp('opacity', TJSONFloatNumber.Create(Value));
end;

function TCircle.GetBlendMode: TBlendMode;
begin
  Result := BlendModeOf(GetProp('blend_mode'));
end;

procedure TCircle.SetBlendMode(Value: TBlendMode);
begin
  SetProp('blend_mode', TJSONString.Create(BlendModeNames[Value]));
end;

function TCircle.GetCacheLayer: Boolean;
begin
  Result := GetProp('cache_layer').AsBoolean;
end;

procedure TCircle.SetCacheLayer(Value: Boolean);
begin
  SetProp('cache_layer', TJSONBoolean.Create(Value));
end;

function TCircle.GetRotation: Double;
begin
  Result := GetProp('rotation').AsFloat;
end;

procedure TCircle.SetRotation(Value: Double);
begin
  SetProp('rotation', TJSONFloatNumber.Create(Value));
end;

function TCircle.GetScale: Double;
begin
  Result := GetProp('scale').AsFloat;
end;

procedure TCircle.SetScale(Value: Double);
begin
  SetProp('scale', TJSONFloatNumber.Create(Value));
end;

function TCircle.GetTranslateX: Double;
begin
  Result := GetProp('translate_x').AsFloat;
end;

procedure TCircle.SetTranslateX(Value: Double);
begin
  SetProp('translate_x', TJSONFloatNumber.Create(Value));
end;

function TCircle.GetTranslateY: Double;
begin
  Result := GetProp('translate_y').AsFloat;
end;

procedure TCircle.SetTranslateY(Value: Double);
begin
  SetProp('translate_y', TJSONFloatNumber.Create(Value));
end;

function TCircle.GetAspectRatio: Double;
begin
  Result := GetProp('aspect_ratio').AsFloat;
end;

procedure TCircle.SetAspectRatio(Value: Double);
begin
  SetProp('aspect_ratio', TJSONFloatNumber.Create(Value));
end;

function TCircle.GetMaxWidth: Double;
begin
  Result := GetProp('max_width').AsFloat;
end;

procedure TCircle.SetMaxWidth(Value: Double);
begin
  SetProp('max_width', TJSONFloatNumber.Create(Value));
end;

function TCircle.GetMinWidth: Double;
begin
  Result := GetProp('min_width').AsFloat;
end;

procedure TCircle.SetMinWidth(Value: Double);
begin
  SetProp('min_width', TJSONFloatNumber.Create(Value));
end;

function TCircle.GetMinHeight: Double;
begin
  Result := GetProp('min_height').AsFloat;
end;

procedure TCircle.SetMinHeight(Value: Double);
begin
  SetProp('min_height', TJSONFloatNumber.Create(Value));
end;

function TCircle.GetMaxHeight: Double;
begin
  Result := GetProp('max_height').AsFloat;
end;

procedure TCircle.SetMaxHeight(Value: Double);
begin
  SetProp('max_height', TJSONFloatNumber.Create(Value));
end;

function TCircle.GetSelfAlign: TSelfAlign;
begin
  Result := SelfAlignOf(GetProp('self_align'));
end;

procedure TCircle.SetSelfAlign(Value: TSelfAlign);
begin
  SetProp('self_align', TJSONString.Create(SelfAlignNames[Value]));
end;

function TCircle.GetAutoSize: Boolean;
begin
  Result := GetProp('auto_size').AsBoolean;
end;

procedure TCircle.SetAutoSize(Value: Boolean);
begin
  SetProp('auto_size', TJSONBoolean.Create(Value));
end;

function TCircle.GetColumnSpan: Int64;
begin
  Result := GetProp('column_span').AsInt64;
end;

procedure TCircle.SetColumnSpan(Value: Int64);
begin
  SetProp('column_span', TJSONInt64Number.Create(Value));
end;

function TCircle.GetGridRow: Int64;
begin
  Result := GetProp('grid_row').AsInt64;
end;

procedure TCircle.SetGridRow(Value: Int64);
begin
  SetProp('grid_row', TJSONInt64Number.Create(Value));
end;

function TCircle.GetGridColumn: Int64;
begin
  Result := GetProp('grid_column').AsInt64;
end;

procedure TCircle.SetGridColumn(Value: Int64);
begin
  SetProp('grid_column', TJSONInt64Number.Create(Value));
end;

function TCircle.GetRowSpan: Int64;
begin
  Result := GetProp('row_span').AsInt64;
end;

procedure TCircle.SetRowSpan(Value: Int64);
begin
  SetProp('row_span', TJSONInt64Number.Create(Value));
end;

function TCircle.GetGrow: Double;
begin
  Result := GetProp('grow').AsFloat;
end;

procedure TCircle.SetGrow(Value: Double);
begin
  SetProp('grow', TJSONFloatNumber.Create(Value));
end;

function TCircle.GetShadow: TAylisShadow;
begin
  Result := AylisShadowFromJSON(GetProp('shadow'));
end;

procedure TCircle.SetShadow(Value: TAylisShadow);
begin
  SetProp('shadow', AylisShadowToJSON(Value));
end;

function TCircle.GetStyle: TCircleStyle;
begin
  Result := CircleStyleOf(GetProp('style'));
end;

procedure TCircle.SetStyle(Value: TCircleStyle);
begin
  SetProp('style', TJSONString.Create(CircleStyleNames[Value]));
end;

function TCircle.GetTheme: TThemeName;
begin
  Result := ThemeNameOf(GetProp('theme'));
end;

procedure TCircle.SetTheme(Value: TThemeName);
begin
  SetProp('theme', TJSONString.Create(ThemeNameNames[Value]));
end;

function TCircle.GetBorderColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('border_color'));
end;

procedure TCircle.SetBorderColor(Value: TAylisColor);
begin
  SetProp('border_color', AylisColorToJSON(Value));
end;

function TCircle.GetTextColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('text_color'));
end;

procedure TCircle.SetTextColor(Value: TAylisColor);
begin
  SetProp('text_color', AylisColorToJSON(Value));
end;

function TCircle.GetAccentColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('accent_color'));
end;

procedure TCircle.SetAccentColor(Value: TAylisColor);
begin
  SetProp('accent_color', AylisColorToJSON(Value));
end;

function TCircle.GetSuccessColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('success_color'));
end;

procedure TCircle.SetSuccessColor(Value: TAylisColor);
begin
  SetProp('success_color', AylisColorToJSON(Value));
end;

function TCircle.GetWarningColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('warning_color'));
end;

procedure TCircle.SetWarningColor(Value: TAylisColor);
begin
  SetProp('warning_color', AylisColorToJSON(Value));
end;

function TCircle.GetDangerColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('danger_color'));
end;

procedure TCircle.SetDangerColor(Value: TAylisColor);
begin
  SetProp('danger_color', AylisColorToJSON(Value));
end;

function TCircle.GetFontFamily: string;
begin
  Result := AsText(GetProp('font_family'));
end;

procedure TCircle.SetFontFamily(Value: string);
begin
  SetProp('font_family', TJSONString.Create(Value));
end;

function TCircle.GetFontSize: Double;
begin
  Result := GetProp('font_size').AsFloat;
end;

procedure TCircle.SetFontSize(Value: Double);
begin
  SetProp('font_size', TJSONFloatNumber.Create(Value));
end;

function TCircle.GetFontWeight: TFontWeight;
begin
  Result := FontWeightOf(GetProp('font_weight'));
end;

procedure TCircle.SetFontWeight(Value: TFontWeight);
begin
  SetProp('font_weight', TJSONString.Create(FontWeightNames[Value]));
end;

function TCircle.GetFontItalic: Boolean;
begin
  Result := GetProp('font_italic').AsBoolean;
end;

procedure TCircle.SetFontItalic(Value: Boolean);
begin
  SetProp('font_italic', TJSONBoolean.Create(Value));
end;

function TCircle.GetFontUnderline: Boolean;
begin
  Result := GetProp('font_underline').AsBoolean;
end;

procedure TCircle.SetFontUnderline(Value: Boolean);
begin
  SetProp('font_underline', TJSONBoolean.Create(Value));
end;

function TCircle.GetFontStrikeout: Boolean;
begin
  Result := GetProp('font_strikeout').AsBoolean;
end;

procedure TCircle.SetFontStrikeout(Value: Boolean);
begin
  SetProp('font_strikeout', TJSONBoolean.Create(Value));
end;

function TCircle.GetHint: string;
begin
  Result := AsText(GetProp('hint'));
end;

procedure TCircle.SetHint(Value: string);
begin
  SetProp('hint', TJSONString.Create(Value));
end;

function TCircle.GetTag: Int64;
begin
  Result := GetProp('tag').AsInt64;
end;

procedure TCircle.SetTag(Value: Int64);
begin
  SetProp('tag', TJSONInt64Number.Create(Value));
end;

function TCircle.GetTagString: string;
begin
  Result := AsText(GetProp('tag_string'));
end;

procedure TCircle.SetTagString(Value: string);
begin
  SetProp('tag_string', TJSONString.Create(Value));
end;

function TCircle.GetAccessibleName: string;
begin
  Result := AsText(GetProp('accessible_name'));
end;

procedure TCircle.SetAccessibleName(Value: string);
begin
  SetProp('accessible_name', TJSONString.Create(Value));
end;

function TCircle.GetPopupMenu: TAylisComponent;
begin
  Result := GetRef('popup_menu');
end;

procedure TCircle.SetPopupMenu(Value: TAylisComponent);
begin
  SetRef('popup_menu', Value);
end;

function TCircle.GetOnClick: string;
begin
  Result := AsText(GetProp('on_click'));
end;

procedure TCircle.SetOnClick(Value: string);
begin
  SetProp('on_click', TJSONString.Create(Value));
end;

function TCircle.GetOnDblClick: string;
begin
  Result := AsText(GetProp('on_dbl_click'));
end;

procedure TCircle.SetOnDblClick(Value: string);
begin
  SetProp('on_dbl_click', TJSONString.Create(Value));
end;

function TCircle.GetOnLongPress: string;
begin
  Result := AsText(GetProp('on_long_press'));
end;

procedure TCircle.SetOnLongPress(Value: string);
begin
  SetProp('on_long_press', TJSONString.Create(Value));
end;

function TCircle.GetOnPointerDown: string;
begin
  Result := AsText(GetProp('on_pointer_down'));
end;

procedure TCircle.SetOnPointerDown(Value: string);
begin
  SetProp('on_pointer_down', TJSONString.Create(Value));
end;

function TCircle.GetOnPointerUp: string;
begin
  Result := AsText(GetProp('on_pointer_up'));
end;

procedure TCircle.SetOnPointerUp(Value: string);
begin
  SetProp('on_pointer_up', TJSONString.Create(Value));
end;

function TCircle.GetOnPointerMove: string;
begin
  Result := AsText(GetProp('on_pointer_move'));
end;

procedure TCircle.SetOnPointerMove(Value: string);
begin
  SetProp('on_pointer_move', TJSONString.Create(Value));
end;

function TCircle.GetOnPointerEnter: string;
begin
  Result := AsText(GetProp('on_pointer_enter'));
end;

procedure TCircle.SetOnPointerEnter(Value: string);
begin
  SetProp('on_pointer_enter', TJSONString.Create(Value));
end;

function TCircle.GetOnPointerLeave: string;
begin
  Result := AsText(GetProp('on_pointer_leave'));
end;

procedure TCircle.SetOnPointerLeave(Value: string);
begin
  SetProp('on_pointer_leave', TJSONString.Create(Value));
end;

function TCircle.GetOnWheel: string;
begin
  Result := AsText(GetProp('on_wheel'));
end;

procedure TCircle.SetOnWheel(Value: string);
begin
  SetProp('on_wheel', TJSONString.Create(Value));
end;

function TCircle.GetOnGesture: string;
begin
  Result := AsText(GetProp('on_gesture'));
end;

procedure TCircle.SetOnGesture(Value: string);
begin
  SetProp('on_gesture', TJSONString.Create(Value));
end;

function TCircle.GetOnSwipe: string;
begin
  Result := AsText(GetProp('on_swipe'));
end;

procedure TCircle.SetOnSwipe(Value: string);
begin
  SetProp('on_swipe', TJSONString.Create(Value));
end;

function TCircle.GetOnKeyUp: string;
begin
  Result := AsText(GetProp('on_key_up'));
end;

procedure TCircle.SetOnKeyUp(Value: string);
begin
  SetProp('on_key_up', TJSONString.Create(Value));
end;

function TCircle.GetOnEnter: string;
begin
  Result := AsText(GetProp('on_enter'));
end;

procedure TCircle.SetOnEnter(Value: string);
begin
  SetProp('on_enter', TJSONString.Create(Value));
end;

function TCircle.GetOnExit: string;
begin
  Result := AsText(GetProp('on_exit'));
end;

procedure TCircle.SetOnExit(Value: string);
begin
  SetProp('on_exit', TJSONString.Create(Value));
end;

function TCircle.GetOnResize: string;
begin
  Result := AsText(GetProp('on_resize'));
end;

procedure TCircle.SetOnResize(Value: string);
begin
  SetProp('on_resize', TJSONString.Create(Value));
end;

function TCircle.GetOnDragMove: string;
begin
  Result := AsText(GetProp('on_drag_move'));
end;

procedure TCircle.SetOnDragMove(Value: string);
begin
  SetProp('on_drag_move', TJSONString.Create(Value));
end;

function TCircle.GetOnDragLeave: string;
begin
  Result := AsText(GetProp('on_drag_leave'));
end;

procedure TCircle.SetOnDragLeave(Value: string);
begin
  SetProp('on_drag_leave', TJSONString.Create(Value));
end;

function TCircle.GetOnDragDrop: string;
begin
  Result := AsText(GetProp('on_drag_drop'));
end;

procedure TCircle.SetOnDragDrop(Value: string);
begin
  SetProp('on_drag_drop', TJSONString.Create(Value));
end;

function TCircle.GetOnDragEnd: string;
begin
  Result := AsText(GetProp('on_drag_end'));
end;

procedure TCircle.SetOnDragEnd(Value: string);
begin
  SetProp('on_drag_end', TJSONString.Create(Value));
end;

function TComboBox.GetItems: TStringArray;
begin
  Result := AsList(GetProp('items'));
end;

procedure TComboBox.SetItems(Value: TStringArray);
begin
  SetProp('items', ListText(Value));
end;

function TComboBox.GetValues: string;
begin
  Result := AsText(GetProp('values'));
end;

procedure TComboBox.SetValues(Value: string);
begin
  SetProp('values', TJSONString.Create(Value));
end;

function TComboBox.GetItemIndex: Int64;
begin
  Result := GetProp('item_index').AsInt64;
end;

procedure TComboBox.SetItemIndex(Value: Int64);
begin
  SetProp('item_index', TJSONInt64Number.Create(Value));
end;

function TComboBox.GetHAlign: TTextAlign;
begin
  Result := TextAlignOf(GetProp('h_align'));
end;

procedure TComboBox.SetHAlign(Value: TTextAlign);
begin
  SetProp('h_align', TJSONString.Create(TextAlignNames[Value]));
end;

function TComboBox.GetTrimming: TTrimming;
begin
  Result := TrimmingOf(GetProp('trimming'));
end;

procedure TComboBox.SetTrimming(Value: TTrimming);
begin
  SetProp('trimming', TJSONString.Create(TrimmingNames[Value]));
end;

function TComboBox.GetArrow: TComboArrow;
begin
  Result := ComboArrowOf(GetProp('arrow'));
end;

procedure TComboBox.SetArrow(Value: TComboArrow);
begin
  SetProp('arrow', TJSONString.Create(ComboArrowNames[Value]));
end;

function TComboBox.GetSensitive: Boolean;
begin
  Result := GetProp('sensitive').AsBoolean;
end;

procedure TComboBox.SetSensitive(Value: Boolean);
begin
  SetProp('sensitive', TJSONBoolean.Create(Value));
end;

function TComboBox.GetGroups: TStringArray;
begin
  Result := AsList(GetProp('groups'));
end;

procedure TComboBox.SetGroups(Value: TStringArray);
begin
  SetProp('groups', ListText(Value));
end;

function TComboBox.GetDisabledItems: TStringArray;
begin
  Result := AsList(GetProp('disabled_items'));
end;

procedure TComboBox.SetDisabledItems(Value: TStringArray);
begin
  SetProp('disabled_items', ListText(Value));
end;

function TComboBox.GetDropDownCount: Int64;
begin
  Result := GetProp('drop_down_count').AsInt64;
end;

procedure TComboBox.SetDropDownCount(Value: Int64);
begin
  SetProp('drop_down_count', TJSONInt64Number.Create(Value));
end;

function TComboBox.GetOnChange: string;
begin
  Result := AsText(GetProp('on_change'));
end;

procedure TComboBox.SetOnChange(Value: string);
begin
  SetProp('on_change', TJSONString.Create(Value));
end;

function TComboBox.GetValue: string;
begin
  Result := AsText(GetProp('value'));
end;

procedure TComboBox.SetValue(Value: string);
begin
  SetProp('value', TJSONString.Create(Value));
end;

function TComboBox.GetLeft: Double;
begin
  Result := GetProp('left').AsFloat;
end;

procedure TComboBox.SetLeft(Value: Double);
begin
  SetProp('left', TJSONFloatNumber.Create(Value));
end;

function TComboBox.GetTop: Double;
begin
  Result := GetProp('top').AsFloat;
end;

procedure TComboBox.SetTop(Value: Double);
begin
  SetProp('top', TJSONFloatNumber.Create(Value));
end;

function TComboBox.GetWidth: Double;
begin
  Result := GetProp('width').AsFloat;
end;

procedure TComboBox.SetWidth(Value: Double);
begin
  SetProp('width', TJSONFloatNumber.Create(Value));
end;

function TComboBox.GetHeight: Double;
begin
  Result := GetProp('height').AsFloat;
end;

procedure TComboBox.SetHeight(Value: Double);
begin
  SetProp('height', TJSONFloatNumber.Create(Value));
end;

function TComboBox.GetAlign: TAlign;
begin
  Result := AlignOf(GetProp('align'));
end;

procedure TComboBox.SetAlign(Value: TAlign);
begin
  SetProp('align', TJSONString.Create(AlignNames[Value]));
end;

function TComboBox.GetAnchors: TAnchorSet;
begin
  Result := AnchorSetOf(GetProp('anchors'));
end;

procedure TComboBox.SetAnchors(Value: TAnchorSet);
begin
  SetProp('anchors', AnchorSetToJSON(Value));
end;

function TComboBox.GetMargins: TAylisBounds;
begin
  Result := AylisBoundsFromJSON(GetProp('margins'));
end;

procedure TComboBox.SetMargins(Value: TAylisBounds);
begin
  SetProp('margins', AylisBoundsToJSON(Value));
end;

function TComboBox.GetPadding: TAylisBounds;
begin
  Result := AylisBoundsFromJSON(GetProp('padding'));
end;

procedure TComboBox.SetPadding(Value: TAylisBounds);
begin
  SetProp('padding', AylisBoundsToJSON(Value));
end;

function TComboBox.GetVisible: Boolean;
begin
  Result := GetProp('visible').AsBoolean;
end;

procedure TComboBox.SetVisible(Value: Boolean);
begin
  SetProp('visible', TJSONBoolean.Create(Value));
end;

function TComboBox.GetEnabled: Boolean;
begin
  Result := GetProp('enabled').AsBoolean;
end;

procedure TComboBox.SetEnabled(Value: Boolean);
begin
  SetProp('enabled', TJSONBoolean.Create(Value));
end;

function TComboBox.GetHitTest: Boolean;
begin
  Result := GetProp('hit_test').AsBoolean;
end;

procedure TComboBox.SetHitTest(Value: Boolean);
begin
  SetProp('hit_test', TJSONBoolean.Create(Value));
end;

function TComboBox.GetClipChildren: Boolean;
begin
  Result := GetProp('clip_children').AsBoolean;
end;

procedure TComboBox.SetClipChildren(Value: Boolean);
begin
  SetProp('clip_children', TJSONBoolean.Create(Value));
end;

function TComboBox.GetInheritState: Boolean;
begin
  Result := GetProp('inherit_state').AsBoolean;
end;

procedure TComboBox.SetInheritState(Value: Boolean);
begin
  SetProp('inherit_state', TJSONBoolean.Create(Value));
end;

function TComboBox.GetTranslate: Boolean;
begin
  Result := GetProp('translate').AsBoolean;
end;

procedure TComboBox.SetTranslate(Value: Boolean);
begin
  SetProp('translate', TJSONBoolean.Create(Value));
end;

function TComboBox.GetCapturesWheel: Boolean;
begin
  Result := GetProp('captures_wheel').AsBoolean;
end;

procedure TComboBox.SetCapturesWheel(Value: Boolean);
begin
  SetProp('captures_wheel', TJSONBoolean.Create(Value));
end;

function TComboBox.GetCapturesGesture: Boolean;
begin
  Result := GetProp('captures_gesture').AsBoolean;
end;

procedure TComboBox.SetCapturesGesture(Value: Boolean);
begin
  SetProp('captures_gesture', TJSONBoolean.Create(Value));
end;

function TComboBox.GetCanFocus: Boolean;
begin
  Result := GetProp('can_focus').AsBoolean;
end;

procedure TComboBox.SetCanFocus(Value: Boolean);
begin
  SetProp('can_focus', TJSONBoolean.Create(Value));
end;

function TComboBox.GetDraggable: Boolean;
begin
  Result := GetProp('draggable').AsBoolean;
end;

procedure TComboBox.SetDraggable(Value: Boolean);
begin
  SetProp('draggable', TJSONBoolean.Create(Value));
end;

function TComboBox.GetAcceptsDrops: Boolean;
begin
  Result := GetProp('accepts_drops').AsBoolean;
end;

procedure TComboBox.SetAcceptsDrops(Value: Boolean);
begin
  SetProp('accepts_drops', TJSONBoolean.Create(Value));
end;

function TComboBox.GetDragData: string;
begin
  Result := AsText(GetProp('drag_data'));
end;

procedure TComboBox.SetDragData(Value: string);
begin
  SetProp('drag_data', TJSONString.Create(Value));
end;

function TComboBox.GetDragLabel: string;
begin
  Result := AsText(GetProp('drag_label'));
end;

procedure TComboBox.SetDragLabel(Value: string);
begin
  SetProp('drag_label', TJSONString.Create(Value));
end;

function TComboBox.GetDragImage: TAylisComponent;
begin
  Result := GetRef('drag_image');
end;

procedure TComboBox.SetDragImage(Value: TAylisComponent);
begin
  SetRef('drag_image', Value);
end;

function TComboBox.GetDropTypes: string;
begin
  Result := AsText(GetProp('drop_types'));
end;

procedure TComboBox.SetDropTypes(Value: string);
begin
  SetProp('drop_types', TJSONString.Create(Value));
end;

function TComboBox.GetTabStop: Boolean;
begin
  Result := GetProp('tab_stop').AsBoolean;
end;

procedure TComboBox.SetTabStop(Value: Boolean);
begin
  SetProp('tab_stop', TJSONBoolean.Create(Value));
end;

function TComboBox.GetCursor: TCursorShape;
begin
  Result := CursorShapeOf(GetProp('cursor'));
end;

procedure TComboBox.SetCursor(Value: TCursorShape);
begin
  SetProp('cursor', TJSONString.Create(CursorShapeNames[Value]));
end;

function TComboBox.GetOpacity: Double;
begin
  Result := GetProp('opacity').AsFloat;
end;

procedure TComboBox.SetOpacity(Value: Double);
begin
  SetProp('opacity', TJSONFloatNumber.Create(Value));
end;

function TComboBox.GetBlendMode: TBlendMode;
begin
  Result := BlendModeOf(GetProp('blend_mode'));
end;

procedure TComboBox.SetBlendMode(Value: TBlendMode);
begin
  SetProp('blend_mode', TJSONString.Create(BlendModeNames[Value]));
end;

function TComboBox.GetCacheLayer: Boolean;
begin
  Result := GetProp('cache_layer').AsBoolean;
end;

procedure TComboBox.SetCacheLayer(Value: Boolean);
begin
  SetProp('cache_layer', TJSONBoolean.Create(Value));
end;

function TComboBox.GetRotation: Double;
begin
  Result := GetProp('rotation').AsFloat;
end;

procedure TComboBox.SetRotation(Value: Double);
begin
  SetProp('rotation', TJSONFloatNumber.Create(Value));
end;

function TComboBox.GetScale: Double;
begin
  Result := GetProp('scale').AsFloat;
end;

procedure TComboBox.SetScale(Value: Double);
begin
  SetProp('scale', TJSONFloatNumber.Create(Value));
end;

function TComboBox.GetTranslateX: Double;
begin
  Result := GetProp('translate_x').AsFloat;
end;

procedure TComboBox.SetTranslateX(Value: Double);
begin
  SetProp('translate_x', TJSONFloatNumber.Create(Value));
end;

function TComboBox.GetTranslateY: Double;
begin
  Result := GetProp('translate_y').AsFloat;
end;

procedure TComboBox.SetTranslateY(Value: Double);
begin
  SetProp('translate_y', TJSONFloatNumber.Create(Value));
end;

function TComboBox.GetAspectRatio: Double;
begin
  Result := GetProp('aspect_ratio').AsFloat;
end;

procedure TComboBox.SetAspectRatio(Value: Double);
begin
  SetProp('aspect_ratio', TJSONFloatNumber.Create(Value));
end;

function TComboBox.GetMaxWidth: Double;
begin
  Result := GetProp('max_width').AsFloat;
end;

procedure TComboBox.SetMaxWidth(Value: Double);
begin
  SetProp('max_width', TJSONFloatNumber.Create(Value));
end;

function TComboBox.GetMinWidth: Double;
begin
  Result := GetProp('min_width').AsFloat;
end;

procedure TComboBox.SetMinWidth(Value: Double);
begin
  SetProp('min_width', TJSONFloatNumber.Create(Value));
end;

function TComboBox.GetMinHeight: Double;
begin
  Result := GetProp('min_height').AsFloat;
end;

procedure TComboBox.SetMinHeight(Value: Double);
begin
  SetProp('min_height', TJSONFloatNumber.Create(Value));
end;

function TComboBox.GetMaxHeight: Double;
begin
  Result := GetProp('max_height').AsFloat;
end;

procedure TComboBox.SetMaxHeight(Value: Double);
begin
  SetProp('max_height', TJSONFloatNumber.Create(Value));
end;

function TComboBox.GetSelfAlign: TSelfAlign;
begin
  Result := SelfAlignOf(GetProp('self_align'));
end;

procedure TComboBox.SetSelfAlign(Value: TSelfAlign);
begin
  SetProp('self_align', TJSONString.Create(SelfAlignNames[Value]));
end;

function TComboBox.GetAutoSize: Boolean;
begin
  Result := GetProp('auto_size').AsBoolean;
end;

procedure TComboBox.SetAutoSize(Value: Boolean);
begin
  SetProp('auto_size', TJSONBoolean.Create(Value));
end;

function TComboBox.GetColumnSpan: Int64;
begin
  Result := GetProp('column_span').AsInt64;
end;

procedure TComboBox.SetColumnSpan(Value: Int64);
begin
  SetProp('column_span', TJSONInt64Number.Create(Value));
end;

function TComboBox.GetGridRow: Int64;
begin
  Result := GetProp('grid_row').AsInt64;
end;

procedure TComboBox.SetGridRow(Value: Int64);
begin
  SetProp('grid_row', TJSONInt64Number.Create(Value));
end;

function TComboBox.GetGridColumn: Int64;
begin
  Result := GetProp('grid_column').AsInt64;
end;

procedure TComboBox.SetGridColumn(Value: Int64);
begin
  SetProp('grid_column', TJSONInt64Number.Create(Value));
end;

function TComboBox.GetRowSpan: Int64;
begin
  Result := GetProp('row_span').AsInt64;
end;

procedure TComboBox.SetRowSpan(Value: Int64);
begin
  SetProp('row_span', TJSONInt64Number.Create(Value));
end;

function TComboBox.GetGrow: Double;
begin
  Result := GetProp('grow').AsFloat;
end;

procedure TComboBox.SetGrow(Value: Double);
begin
  SetProp('grow', TJSONFloatNumber.Create(Value));
end;

function TComboBox.GetShadow: TAylisShadow;
begin
  Result := AylisShadowFromJSON(GetProp('shadow'));
end;

procedure TComboBox.SetShadow(Value: TAylisShadow);
begin
  SetProp('shadow', AylisShadowToJSON(Value));
end;

function TComboBox.GetStyle: TComboBoxStyle;
begin
  Result := ComboBoxStyleOf(GetProp('style'));
end;

procedure TComboBox.SetStyle(Value: TComboBoxStyle);
begin
  SetProp('style', TJSONString.Create(ComboBoxStyleNames[Value]));
end;

function TComboBox.GetTheme: TThemeName;
begin
  Result := ThemeNameOf(GetProp('theme'));
end;

procedure TComboBox.SetTheme(Value: TThemeName);
begin
  SetProp('theme', TJSONString.Create(ThemeNameNames[Value]));
end;

function TComboBox.GetFill: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('fill'));
end;

procedure TComboBox.SetFill(Value: TAylisColor);
begin
  SetProp('fill', AylisColorToJSON(Value));
end;

function TComboBox.GetBorderColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('border_color'));
end;

procedure TComboBox.SetBorderColor(Value: TAylisColor);
begin
  SetProp('border_color', AylisColorToJSON(Value));
end;

function TComboBox.GetTextColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('text_color'));
end;

procedure TComboBox.SetTextColor(Value: TAylisColor);
begin
  SetProp('text_color', AylisColorToJSON(Value));
end;

function TComboBox.GetAccentColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('accent_color'));
end;

procedure TComboBox.SetAccentColor(Value: TAylisColor);
begin
  SetProp('accent_color', AylisColorToJSON(Value));
end;

function TComboBox.GetSuccessColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('success_color'));
end;

procedure TComboBox.SetSuccessColor(Value: TAylisColor);
begin
  SetProp('success_color', AylisColorToJSON(Value));
end;

function TComboBox.GetWarningColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('warning_color'));
end;

procedure TComboBox.SetWarningColor(Value: TAylisColor);
begin
  SetProp('warning_color', AylisColorToJSON(Value));
end;

function TComboBox.GetDangerColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('danger_color'));
end;

procedure TComboBox.SetDangerColor(Value: TAylisColor);
begin
  SetProp('danger_color', AylisColorToJSON(Value));
end;

function TComboBox.GetFontFamily: string;
begin
  Result := AsText(GetProp('font_family'));
end;

procedure TComboBox.SetFontFamily(Value: string);
begin
  SetProp('font_family', TJSONString.Create(Value));
end;

function TComboBox.GetFontSize: Double;
begin
  Result := GetProp('font_size').AsFloat;
end;

procedure TComboBox.SetFontSize(Value: Double);
begin
  SetProp('font_size', TJSONFloatNumber.Create(Value));
end;

function TComboBox.GetFontWeight: TFontWeight;
begin
  Result := FontWeightOf(GetProp('font_weight'));
end;

procedure TComboBox.SetFontWeight(Value: TFontWeight);
begin
  SetProp('font_weight', TJSONString.Create(FontWeightNames[Value]));
end;

function TComboBox.GetFontItalic: Boolean;
begin
  Result := GetProp('font_italic').AsBoolean;
end;

procedure TComboBox.SetFontItalic(Value: Boolean);
begin
  SetProp('font_italic', TJSONBoolean.Create(Value));
end;

function TComboBox.GetFontUnderline: Boolean;
begin
  Result := GetProp('font_underline').AsBoolean;
end;

procedure TComboBox.SetFontUnderline(Value: Boolean);
begin
  SetProp('font_underline', TJSONBoolean.Create(Value));
end;

function TComboBox.GetFontStrikeout: Boolean;
begin
  Result := GetProp('font_strikeout').AsBoolean;
end;

procedure TComboBox.SetFontStrikeout(Value: Boolean);
begin
  SetProp('font_strikeout', TJSONBoolean.Create(Value));
end;

function TComboBox.GetHint: string;
begin
  Result := AsText(GetProp('hint'));
end;

procedure TComboBox.SetHint(Value: string);
begin
  SetProp('hint', TJSONString.Create(Value));
end;

function TComboBox.GetTag: Int64;
begin
  Result := GetProp('tag').AsInt64;
end;

procedure TComboBox.SetTag(Value: Int64);
begin
  SetProp('tag', TJSONInt64Number.Create(Value));
end;

function TComboBox.GetTagString: string;
begin
  Result := AsText(GetProp('tag_string'));
end;

procedure TComboBox.SetTagString(Value: string);
begin
  SetProp('tag_string', TJSONString.Create(Value));
end;

function TComboBox.GetAccessibleName: string;
begin
  Result := AsText(GetProp('accessible_name'));
end;

procedure TComboBox.SetAccessibleName(Value: string);
begin
  SetProp('accessible_name', TJSONString.Create(Value));
end;

function TComboBox.GetPopupMenu: TAylisComponent;
begin
  Result := GetRef('popup_menu');
end;

procedure TComboBox.SetPopupMenu(Value: TAylisComponent);
begin
  SetRef('popup_menu', Value);
end;

function TComboBox.GetOnClick: string;
begin
  Result := AsText(GetProp('on_click'));
end;

procedure TComboBox.SetOnClick(Value: string);
begin
  SetProp('on_click', TJSONString.Create(Value));
end;

function TComboBox.GetOnDblClick: string;
begin
  Result := AsText(GetProp('on_dbl_click'));
end;

procedure TComboBox.SetOnDblClick(Value: string);
begin
  SetProp('on_dbl_click', TJSONString.Create(Value));
end;

function TComboBox.GetOnLongPress: string;
begin
  Result := AsText(GetProp('on_long_press'));
end;

procedure TComboBox.SetOnLongPress(Value: string);
begin
  SetProp('on_long_press', TJSONString.Create(Value));
end;

function TComboBox.GetOnPointerDown: string;
begin
  Result := AsText(GetProp('on_pointer_down'));
end;

procedure TComboBox.SetOnPointerDown(Value: string);
begin
  SetProp('on_pointer_down', TJSONString.Create(Value));
end;

function TComboBox.GetOnPointerUp: string;
begin
  Result := AsText(GetProp('on_pointer_up'));
end;

procedure TComboBox.SetOnPointerUp(Value: string);
begin
  SetProp('on_pointer_up', TJSONString.Create(Value));
end;

function TComboBox.GetOnPointerMove: string;
begin
  Result := AsText(GetProp('on_pointer_move'));
end;

procedure TComboBox.SetOnPointerMove(Value: string);
begin
  SetProp('on_pointer_move', TJSONString.Create(Value));
end;

function TComboBox.GetOnPointerEnter: string;
begin
  Result := AsText(GetProp('on_pointer_enter'));
end;

procedure TComboBox.SetOnPointerEnter(Value: string);
begin
  SetProp('on_pointer_enter', TJSONString.Create(Value));
end;

function TComboBox.GetOnPointerLeave: string;
begin
  Result := AsText(GetProp('on_pointer_leave'));
end;

procedure TComboBox.SetOnPointerLeave(Value: string);
begin
  SetProp('on_pointer_leave', TJSONString.Create(Value));
end;

function TComboBox.GetOnWheel: string;
begin
  Result := AsText(GetProp('on_wheel'));
end;

procedure TComboBox.SetOnWheel(Value: string);
begin
  SetProp('on_wheel', TJSONString.Create(Value));
end;

function TComboBox.GetOnGesture: string;
begin
  Result := AsText(GetProp('on_gesture'));
end;

procedure TComboBox.SetOnGesture(Value: string);
begin
  SetProp('on_gesture', TJSONString.Create(Value));
end;

function TComboBox.GetOnSwipe: string;
begin
  Result := AsText(GetProp('on_swipe'));
end;

procedure TComboBox.SetOnSwipe(Value: string);
begin
  SetProp('on_swipe', TJSONString.Create(Value));
end;

function TComboBox.GetOnKeyUp: string;
begin
  Result := AsText(GetProp('on_key_up'));
end;

procedure TComboBox.SetOnKeyUp(Value: string);
begin
  SetProp('on_key_up', TJSONString.Create(Value));
end;

function TComboBox.GetOnEnter: string;
begin
  Result := AsText(GetProp('on_enter'));
end;

procedure TComboBox.SetOnEnter(Value: string);
begin
  SetProp('on_enter', TJSONString.Create(Value));
end;

function TComboBox.GetOnExit: string;
begin
  Result := AsText(GetProp('on_exit'));
end;

procedure TComboBox.SetOnExit(Value: string);
begin
  SetProp('on_exit', TJSONString.Create(Value));
end;

function TComboBox.GetOnResize: string;
begin
  Result := AsText(GetProp('on_resize'));
end;

procedure TComboBox.SetOnResize(Value: string);
begin
  SetProp('on_resize', TJSONString.Create(Value));
end;

function TComboBox.GetOnDragMove: string;
begin
  Result := AsText(GetProp('on_drag_move'));
end;

procedure TComboBox.SetOnDragMove(Value: string);
begin
  SetProp('on_drag_move', TJSONString.Create(Value));
end;

function TComboBox.GetOnDragLeave: string;
begin
  Result := AsText(GetProp('on_drag_leave'));
end;

procedure TComboBox.SetOnDragLeave(Value: string);
begin
  SetProp('on_drag_leave', TJSONString.Create(Value));
end;

function TComboBox.GetOnDragDrop: string;
begin
  Result := AsText(GetProp('on_drag_drop'));
end;

procedure TComboBox.SetOnDragDrop(Value: string);
begin
  SetProp('on_drag_drop', TJSONString.Create(Value));
end;

function TComboBox.GetOnDragEnd: string;
begin
  Result := AsText(GetProp('on_drag_end'));
end;

procedure TComboBox.SetOnDragEnd(Value: string);
begin
  SetProp('on_drag_end', TJSONString.Create(Value));
end;

procedure TComboBox.SetFocus;
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

function TGridLayout.GetColumns: Int64;
begin
  Result := GetProp('columns').AsInt64;
end;

procedure TGridLayout.SetColumns(Value: Int64);
begin
  SetProp('columns', TJSONInt64Number.Create(Value));
end;

function TGridLayout.GetMinColumnWidth: Double;
begin
  Result := GetProp('min_column_width').AsFloat;
end;

procedure TGridLayout.SetMinColumnWidth(Value: Double);
begin
  SetProp('min_column_width', TJSONFloatNumber.Create(Value));
end;

function TGridLayout.GetGap: Double;
begin
  Result := GetProp('gap').AsFloat;
end;

procedure TGridLayout.SetGap(Value: Double);
begin
  SetProp('gap', TJSONFloatNumber.Create(Value));
end;

function TGridLayout.GetRowHeight: Double;
begin
  Result := GetProp('row_height').AsFloat;
end;

procedure TGridLayout.SetRowHeight(Value: Double);
begin
  SetProp('row_height', TJSONFloatNumber.Create(Value));
end;

function TGridLayout.GetLeft: Double;
begin
  Result := GetProp('left').AsFloat;
end;

procedure TGridLayout.SetLeft(Value: Double);
begin
  SetProp('left', TJSONFloatNumber.Create(Value));
end;

function TGridLayout.GetTop: Double;
begin
  Result := GetProp('top').AsFloat;
end;

procedure TGridLayout.SetTop(Value: Double);
begin
  SetProp('top', TJSONFloatNumber.Create(Value));
end;

function TGridLayout.GetWidth: Double;
begin
  Result := GetProp('width').AsFloat;
end;

procedure TGridLayout.SetWidth(Value: Double);
begin
  SetProp('width', TJSONFloatNumber.Create(Value));
end;

function TGridLayout.GetHeight: Double;
begin
  Result := GetProp('height').AsFloat;
end;

procedure TGridLayout.SetHeight(Value: Double);
begin
  SetProp('height', TJSONFloatNumber.Create(Value));
end;

function TGridLayout.GetAlign: TAlign;
begin
  Result := AlignOf(GetProp('align'));
end;

procedure TGridLayout.SetAlign(Value: TAlign);
begin
  SetProp('align', TJSONString.Create(AlignNames[Value]));
end;

function TGridLayout.GetAnchors: TAnchorSet;
begin
  Result := AnchorSetOf(GetProp('anchors'));
end;

procedure TGridLayout.SetAnchors(Value: TAnchorSet);
begin
  SetProp('anchors', AnchorSetToJSON(Value));
end;

function TGridLayout.GetMargins: TAylisBounds;
begin
  Result := AylisBoundsFromJSON(GetProp('margins'));
end;

procedure TGridLayout.SetMargins(Value: TAylisBounds);
begin
  SetProp('margins', AylisBoundsToJSON(Value));
end;

function TGridLayout.GetPadding: TAylisBounds;
begin
  Result := AylisBoundsFromJSON(GetProp('padding'));
end;

procedure TGridLayout.SetPadding(Value: TAylisBounds);
begin
  SetProp('padding', AylisBoundsToJSON(Value));
end;

function TGridLayout.GetVisible: Boolean;
begin
  Result := GetProp('visible').AsBoolean;
end;

procedure TGridLayout.SetVisible(Value: Boolean);
begin
  SetProp('visible', TJSONBoolean.Create(Value));
end;

function TGridLayout.GetEnabled: Boolean;
begin
  Result := GetProp('enabled').AsBoolean;
end;

procedure TGridLayout.SetEnabled(Value: Boolean);
begin
  SetProp('enabled', TJSONBoolean.Create(Value));
end;

function TGridLayout.GetHitTest: Boolean;
begin
  Result := GetProp('hit_test').AsBoolean;
end;

procedure TGridLayout.SetHitTest(Value: Boolean);
begin
  SetProp('hit_test', TJSONBoolean.Create(Value));
end;

function TGridLayout.GetClipChildren: Boolean;
begin
  Result := GetProp('clip_children').AsBoolean;
end;

procedure TGridLayout.SetClipChildren(Value: Boolean);
begin
  SetProp('clip_children', TJSONBoolean.Create(Value));
end;

function TGridLayout.GetInheritState: Boolean;
begin
  Result := GetProp('inherit_state').AsBoolean;
end;

procedure TGridLayout.SetInheritState(Value: Boolean);
begin
  SetProp('inherit_state', TJSONBoolean.Create(Value));
end;

function TGridLayout.GetTranslate: Boolean;
begin
  Result := GetProp('translate').AsBoolean;
end;

procedure TGridLayout.SetTranslate(Value: Boolean);
begin
  SetProp('translate', TJSONBoolean.Create(Value));
end;

function TGridLayout.GetCapturesWheel: Boolean;
begin
  Result := GetProp('captures_wheel').AsBoolean;
end;

procedure TGridLayout.SetCapturesWheel(Value: Boolean);
begin
  SetProp('captures_wheel', TJSONBoolean.Create(Value));
end;

function TGridLayout.GetCapturesGesture: Boolean;
begin
  Result := GetProp('captures_gesture').AsBoolean;
end;

procedure TGridLayout.SetCapturesGesture(Value: Boolean);
begin
  SetProp('captures_gesture', TJSONBoolean.Create(Value));
end;

function TGridLayout.GetCanFocus: Boolean;
begin
  Result := GetProp('can_focus').AsBoolean;
end;

procedure TGridLayout.SetCanFocus(Value: Boolean);
begin
  SetProp('can_focus', TJSONBoolean.Create(Value));
end;

function TGridLayout.GetDraggable: Boolean;
begin
  Result := GetProp('draggable').AsBoolean;
end;

procedure TGridLayout.SetDraggable(Value: Boolean);
begin
  SetProp('draggable', TJSONBoolean.Create(Value));
end;

function TGridLayout.GetAcceptsDrops: Boolean;
begin
  Result := GetProp('accepts_drops').AsBoolean;
end;

procedure TGridLayout.SetAcceptsDrops(Value: Boolean);
begin
  SetProp('accepts_drops', TJSONBoolean.Create(Value));
end;

function TGridLayout.GetDragData: string;
begin
  Result := AsText(GetProp('drag_data'));
end;

procedure TGridLayout.SetDragData(Value: string);
begin
  SetProp('drag_data', TJSONString.Create(Value));
end;

function TGridLayout.GetDragLabel: string;
begin
  Result := AsText(GetProp('drag_label'));
end;

procedure TGridLayout.SetDragLabel(Value: string);
begin
  SetProp('drag_label', TJSONString.Create(Value));
end;

function TGridLayout.GetDragImage: TAylisComponent;
begin
  Result := GetRef('drag_image');
end;

procedure TGridLayout.SetDragImage(Value: TAylisComponent);
begin
  SetRef('drag_image', Value);
end;

function TGridLayout.GetDropTypes: string;
begin
  Result := AsText(GetProp('drop_types'));
end;

procedure TGridLayout.SetDropTypes(Value: string);
begin
  SetProp('drop_types', TJSONString.Create(Value));
end;

function TGridLayout.GetTabStop: Boolean;
begin
  Result := GetProp('tab_stop').AsBoolean;
end;

procedure TGridLayout.SetTabStop(Value: Boolean);
begin
  SetProp('tab_stop', TJSONBoolean.Create(Value));
end;

function TGridLayout.GetCursor: TCursorShape;
begin
  Result := CursorShapeOf(GetProp('cursor'));
end;

procedure TGridLayout.SetCursor(Value: TCursorShape);
begin
  SetProp('cursor', TJSONString.Create(CursorShapeNames[Value]));
end;

function TGridLayout.GetOpacity: Double;
begin
  Result := GetProp('opacity').AsFloat;
end;

procedure TGridLayout.SetOpacity(Value: Double);
begin
  SetProp('opacity', TJSONFloatNumber.Create(Value));
end;

function TGridLayout.GetBlendMode: TBlendMode;
begin
  Result := BlendModeOf(GetProp('blend_mode'));
end;

procedure TGridLayout.SetBlendMode(Value: TBlendMode);
begin
  SetProp('blend_mode', TJSONString.Create(BlendModeNames[Value]));
end;

function TGridLayout.GetCacheLayer: Boolean;
begin
  Result := GetProp('cache_layer').AsBoolean;
end;

procedure TGridLayout.SetCacheLayer(Value: Boolean);
begin
  SetProp('cache_layer', TJSONBoolean.Create(Value));
end;

function TGridLayout.GetRotation: Double;
begin
  Result := GetProp('rotation').AsFloat;
end;

procedure TGridLayout.SetRotation(Value: Double);
begin
  SetProp('rotation', TJSONFloatNumber.Create(Value));
end;

function TGridLayout.GetScale: Double;
begin
  Result := GetProp('scale').AsFloat;
end;

procedure TGridLayout.SetScale(Value: Double);
begin
  SetProp('scale', TJSONFloatNumber.Create(Value));
end;

function TGridLayout.GetTranslateX: Double;
begin
  Result := GetProp('translate_x').AsFloat;
end;

procedure TGridLayout.SetTranslateX(Value: Double);
begin
  SetProp('translate_x', TJSONFloatNumber.Create(Value));
end;

function TGridLayout.GetTranslateY: Double;
begin
  Result := GetProp('translate_y').AsFloat;
end;

procedure TGridLayout.SetTranslateY(Value: Double);
begin
  SetProp('translate_y', TJSONFloatNumber.Create(Value));
end;

function TGridLayout.GetAspectRatio: Double;
begin
  Result := GetProp('aspect_ratio').AsFloat;
end;

procedure TGridLayout.SetAspectRatio(Value: Double);
begin
  SetProp('aspect_ratio', TJSONFloatNumber.Create(Value));
end;

function TGridLayout.GetMaxWidth: Double;
begin
  Result := GetProp('max_width').AsFloat;
end;

procedure TGridLayout.SetMaxWidth(Value: Double);
begin
  SetProp('max_width', TJSONFloatNumber.Create(Value));
end;

function TGridLayout.GetMinWidth: Double;
begin
  Result := GetProp('min_width').AsFloat;
end;

procedure TGridLayout.SetMinWidth(Value: Double);
begin
  SetProp('min_width', TJSONFloatNumber.Create(Value));
end;

function TGridLayout.GetMinHeight: Double;
begin
  Result := GetProp('min_height').AsFloat;
end;

procedure TGridLayout.SetMinHeight(Value: Double);
begin
  SetProp('min_height', TJSONFloatNumber.Create(Value));
end;

function TGridLayout.GetMaxHeight: Double;
begin
  Result := GetProp('max_height').AsFloat;
end;

procedure TGridLayout.SetMaxHeight(Value: Double);
begin
  SetProp('max_height', TJSONFloatNumber.Create(Value));
end;

function TGridLayout.GetSelfAlign: TSelfAlign;
begin
  Result := SelfAlignOf(GetProp('self_align'));
end;

procedure TGridLayout.SetSelfAlign(Value: TSelfAlign);
begin
  SetProp('self_align', TJSONString.Create(SelfAlignNames[Value]));
end;

function TGridLayout.GetAutoSize: Boolean;
begin
  Result := GetProp('auto_size').AsBoolean;
end;

procedure TGridLayout.SetAutoSize(Value: Boolean);
begin
  SetProp('auto_size', TJSONBoolean.Create(Value));
end;

function TGridLayout.GetColumnSpan: Int64;
begin
  Result := GetProp('column_span').AsInt64;
end;

procedure TGridLayout.SetColumnSpan(Value: Int64);
begin
  SetProp('column_span', TJSONInt64Number.Create(Value));
end;

function TGridLayout.GetGridRow: Int64;
begin
  Result := GetProp('grid_row').AsInt64;
end;

procedure TGridLayout.SetGridRow(Value: Int64);
begin
  SetProp('grid_row', TJSONInt64Number.Create(Value));
end;

function TGridLayout.GetGridColumn: Int64;
begin
  Result := GetProp('grid_column').AsInt64;
end;

procedure TGridLayout.SetGridColumn(Value: Int64);
begin
  SetProp('grid_column', TJSONInt64Number.Create(Value));
end;

function TGridLayout.GetRowSpan: Int64;
begin
  Result := GetProp('row_span').AsInt64;
end;

procedure TGridLayout.SetRowSpan(Value: Int64);
begin
  SetProp('row_span', TJSONInt64Number.Create(Value));
end;

function TGridLayout.GetGrow: Double;
begin
  Result := GetProp('grow').AsFloat;
end;

procedure TGridLayout.SetGrow(Value: Double);
begin
  SetProp('grow', TJSONFloatNumber.Create(Value));
end;

function TGridLayout.GetShadow: TAylisShadow;
begin
  Result := AylisShadowFromJSON(GetProp('shadow'));
end;

procedure TGridLayout.SetShadow(Value: TAylisShadow);
begin
  SetProp('shadow', AylisShadowToJSON(Value));
end;

function TGridLayout.GetStyle: TGridLayoutStyle;
begin
  Result := GridLayoutStyleOf(GetProp('style'));
end;

procedure TGridLayout.SetStyle(Value: TGridLayoutStyle);
begin
  SetProp('style', TJSONString.Create(GridLayoutStyleNames[Value]));
end;

function TGridLayout.GetTheme: TThemeName;
begin
  Result := ThemeNameOf(GetProp('theme'));
end;

procedure TGridLayout.SetTheme(Value: TThemeName);
begin
  SetProp('theme', TJSONString.Create(ThemeNameNames[Value]));
end;

function TGridLayout.GetFill: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('fill'));
end;

procedure TGridLayout.SetFill(Value: TAylisColor);
begin
  SetProp('fill', AylisColorToJSON(Value));
end;

function TGridLayout.GetBorderColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('border_color'));
end;

procedure TGridLayout.SetBorderColor(Value: TAylisColor);
begin
  SetProp('border_color', AylisColorToJSON(Value));
end;

function TGridLayout.GetTextColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('text_color'));
end;

procedure TGridLayout.SetTextColor(Value: TAylisColor);
begin
  SetProp('text_color', AylisColorToJSON(Value));
end;

function TGridLayout.GetAccentColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('accent_color'));
end;

procedure TGridLayout.SetAccentColor(Value: TAylisColor);
begin
  SetProp('accent_color', AylisColorToJSON(Value));
end;

function TGridLayout.GetSuccessColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('success_color'));
end;

procedure TGridLayout.SetSuccessColor(Value: TAylisColor);
begin
  SetProp('success_color', AylisColorToJSON(Value));
end;

function TGridLayout.GetWarningColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('warning_color'));
end;

procedure TGridLayout.SetWarningColor(Value: TAylisColor);
begin
  SetProp('warning_color', AylisColorToJSON(Value));
end;

function TGridLayout.GetDangerColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('danger_color'));
end;

procedure TGridLayout.SetDangerColor(Value: TAylisColor);
begin
  SetProp('danger_color', AylisColorToJSON(Value));
end;

function TGridLayout.GetFontFamily: string;
begin
  Result := AsText(GetProp('font_family'));
end;

procedure TGridLayout.SetFontFamily(Value: string);
begin
  SetProp('font_family', TJSONString.Create(Value));
end;

function TGridLayout.GetFontSize: Double;
begin
  Result := GetProp('font_size').AsFloat;
end;

procedure TGridLayout.SetFontSize(Value: Double);
begin
  SetProp('font_size', TJSONFloatNumber.Create(Value));
end;

function TGridLayout.GetFontWeight: TFontWeight;
begin
  Result := FontWeightOf(GetProp('font_weight'));
end;

procedure TGridLayout.SetFontWeight(Value: TFontWeight);
begin
  SetProp('font_weight', TJSONString.Create(FontWeightNames[Value]));
end;

function TGridLayout.GetFontItalic: Boolean;
begin
  Result := GetProp('font_italic').AsBoolean;
end;

procedure TGridLayout.SetFontItalic(Value: Boolean);
begin
  SetProp('font_italic', TJSONBoolean.Create(Value));
end;

function TGridLayout.GetFontUnderline: Boolean;
begin
  Result := GetProp('font_underline').AsBoolean;
end;

procedure TGridLayout.SetFontUnderline(Value: Boolean);
begin
  SetProp('font_underline', TJSONBoolean.Create(Value));
end;

function TGridLayout.GetFontStrikeout: Boolean;
begin
  Result := GetProp('font_strikeout').AsBoolean;
end;

procedure TGridLayout.SetFontStrikeout(Value: Boolean);
begin
  SetProp('font_strikeout', TJSONBoolean.Create(Value));
end;

function TGridLayout.GetHint: string;
begin
  Result := AsText(GetProp('hint'));
end;

procedure TGridLayout.SetHint(Value: string);
begin
  SetProp('hint', TJSONString.Create(Value));
end;

function TGridLayout.GetTag: Int64;
begin
  Result := GetProp('tag').AsInt64;
end;

procedure TGridLayout.SetTag(Value: Int64);
begin
  SetProp('tag', TJSONInt64Number.Create(Value));
end;

function TGridLayout.GetTagString: string;
begin
  Result := AsText(GetProp('tag_string'));
end;

procedure TGridLayout.SetTagString(Value: string);
begin
  SetProp('tag_string', TJSONString.Create(Value));
end;

function TGridLayout.GetAccessibleName: string;
begin
  Result := AsText(GetProp('accessible_name'));
end;

procedure TGridLayout.SetAccessibleName(Value: string);
begin
  SetProp('accessible_name', TJSONString.Create(Value));
end;

function TGridLayout.GetPopupMenu: TAylisComponent;
begin
  Result := GetRef('popup_menu');
end;

procedure TGridLayout.SetPopupMenu(Value: TAylisComponent);
begin
  SetRef('popup_menu', Value);
end;

function TGridLayout.GetOnClick: string;
begin
  Result := AsText(GetProp('on_click'));
end;

procedure TGridLayout.SetOnClick(Value: string);
begin
  SetProp('on_click', TJSONString.Create(Value));
end;

function TGridLayout.GetOnDblClick: string;
begin
  Result := AsText(GetProp('on_dbl_click'));
end;

procedure TGridLayout.SetOnDblClick(Value: string);
begin
  SetProp('on_dbl_click', TJSONString.Create(Value));
end;

function TGridLayout.GetOnLongPress: string;
begin
  Result := AsText(GetProp('on_long_press'));
end;

procedure TGridLayout.SetOnLongPress(Value: string);
begin
  SetProp('on_long_press', TJSONString.Create(Value));
end;

function TGridLayout.GetOnPointerDown: string;
begin
  Result := AsText(GetProp('on_pointer_down'));
end;

procedure TGridLayout.SetOnPointerDown(Value: string);
begin
  SetProp('on_pointer_down', TJSONString.Create(Value));
end;

function TGridLayout.GetOnPointerUp: string;
begin
  Result := AsText(GetProp('on_pointer_up'));
end;

procedure TGridLayout.SetOnPointerUp(Value: string);
begin
  SetProp('on_pointer_up', TJSONString.Create(Value));
end;

function TGridLayout.GetOnPointerMove: string;
begin
  Result := AsText(GetProp('on_pointer_move'));
end;

procedure TGridLayout.SetOnPointerMove(Value: string);
begin
  SetProp('on_pointer_move', TJSONString.Create(Value));
end;

function TGridLayout.GetOnPointerEnter: string;
begin
  Result := AsText(GetProp('on_pointer_enter'));
end;

procedure TGridLayout.SetOnPointerEnter(Value: string);
begin
  SetProp('on_pointer_enter', TJSONString.Create(Value));
end;

function TGridLayout.GetOnPointerLeave: string;
begin
  Result := AsText(GetProp('on_pointer_leave'));
end;

procedure TGridLayout.SetOnPointerLeave(Value: string);
begin
  SetProp('on_pointer_leave', TJSONString.Create(Value));
end;

function TGridLayout.GetOnWheel: string;
begin
  Result := AsText(GetProp('on_wheel'));
end;

procedure TGridLayout.SetOnWheel(Value: string);
begin
  SetProp('on_wheel', TJSONString.Create(Value));
end;

function TGridLayout.GetOnGesture: string;
begin
  Result := AsText(GetProp('on_gesture'));
end;

procedure TGridLayout.SetOnGesture(Value: string);
begin
  SetProp('on_gesture', TJSONString.Create(Value));
end;

function TGridLayout.GetOnSwipe: string;
begin
  Result := AsText(GetProp('on_swipe'));
end;

procedure TGridLayout.SetOnSwipe(Value: string);
begin
  SetProp('on_swipe', TJSONString.Create(Value));
end;

function TGridLayout.GetOnKeyUp: string;
begin
  Result := AsText(GetProp('on_key_up'));
end;

procedure TGridLayout.SetOnKeyUp(Value: string);
begin
  SetProp('on_key_up', TJSONString.Create(Value));
end;

function TGridLayout.GetOnEnter: string;
begin
  Result := AsText(GetProp('on_enter'));
end;

procedure TGridLayout.SetOnEnter(Value: string);
begin
  SetProp('on_enter', TJSONString.Create(Value));
end;

function TGridLayout.GetOnExit: string;
begin
  Result := AsText(GetProp('on_exit'));
end;

procedure TGridLayout.SetOnExit(Value: string);
begin
  SetProp('on_exit', TJSONString.Create(Value));
end;

function TGridLayout.GetOnResize: string;
begin
  Result := AsText(GetProp('on_resize'));
end;

procedure TGridLayout.SetOnResize(Value: string);
begin
  SetProp('on_resize', TJSONString.Create(Value));
end;

function TGridLayout.GetOnDragMove: string;
begin
  Result := AsText(GetProp('on_drag_move'));
end;

procedure TGridLayout.SetOnDragMove(Value: string);
begin
  SetProp('on_drag_move', TJSONString.Create(Value));
end;

function TGridLayout.GetOnDragLeave: string;
begin
  Result := AsText(GetProp('on_drag_leave'));
end;

procedure TGridLayout.SetOnDragLeave(Value: string);
begin
  SetProp('on_drag_leave', TJSONString.Create(Value));
end;

function TGridLayout.GetOnDragDrop: string;
begin
  Result := AsText(GetProp('on_drag_drop'));
end;

procedure TGridLayout.SetOnDragDrop(Value: string);
begin
  SetProp('on_drag_drop', TJSONString.Create(Value));
end;

function TGridLayout.GetOnDragEnd: string;
begin
  Result := AsText(GetProp('on_drag_end'));
end;

procedure TGridLayout.SetOnDragEnd(Value: string);
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

function TPaintCanvas.GetTool: TPaintTool;
begin
  Result := PaintToolOf(GetProp('tool'));
end;

procedure TPaintCanvas.SetTool(Value: TPaintTool);
begin
  SetProp('tool', TJSONString.Create(PaintToolNames[Value]));
end;

function TPaintCanvas.GetColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('color'));
end;

procedure TPaintCanvas.SetColor(Value: TAylisColor);
begin
  SetProp('color', AylisColorToJSON(Value));
end;

function TPaintCanvas.GetBackColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('back_color'));
end;

procedure TPaintCanvas.SetBackColor(Value: TAylisColor);
begin
  SetProp('back_color', AylisColorToJSON(Value));
end;

function TPaintCanvas.GetBrushSize: Int64;
begin
  Result := GetProp('brush_size').AsInt64;
end;

procedure TPaintCanvas.SetBrushSize(Value: Int64);
begin
  SetProp('brush_size', TJSONInt64Number.Create(Value));
end;

function TPaintCanvas.GetFilled: Boolean;
begin
  Result := GetProp('filled').AsBoolean;
end;

procedure TPaintCanvas.SetFilled(Value: Boolean);
begin
  SetProp('filled', TJSONBoolean.Create(Value));
end;

function TPaintCanvas.GetZoom: Double;
begin
  Result := GetProp('zoom').AsFloat;
end;

procedure TPaintCanvas.SetZoom(Value: Double);
begin
  SetProp('zoom', TJSONFloatNumber.Create(Value));
end;

function TPaintCanvas.GetOnChange: string;
begin
  Result := AsText(GetProp('on_change'));
end;

procedure TPaintCanvas.SetOnChange(Value: string);
begin
  SetProp('on_change', TJSONString.Create(Value));
end;

function TPaintCanvas.GetOnPick: string;
begin
  Result := AsText(GetProp('on_pick'));
end;

procedure TPaintCanvas.SetOnPick(Value: string);
begin
  SetProp('on_pick', TJSONString.Create(Value));
end;

function TPaintCanvas.GetOnCursor: string;
begin
  Result := AsText(GetProp('on_cursor'));
end;

procedure TPaintCanvas.SetOnCursor(Value: string);
begin
  SetProp('on_cursor', TJSONString.Create(Value));
end;

function TPaintCanvas.GetLeft: Double;
begin
  Result := GetProp('left').AsFloat;
end;

procedure TPaintCanvas.SetLeft(Value: Double);
begin
  SetProp('left', TJSONFloatNumber.Create(Value));
end;

function TPaintCanvas.GetTop: Double;
begin
  Result := GetProp('top').AsFloat;
end;

procedure TPaintCanvas.SetTop(Value: Double);
begin
  SetProp('top', TJSONFloatNumber.Create(Value));
end;

function TPaintCanvas.GetWidth: Double;
begin
  Result := GetProp('width').AsFloat;
end;

procedure TPaintCanvas.SetWidth(Value: Double);
begin
  SetProp('width', TJSONFloatNumber.Create(Value));
end;

function TPaintCanvas.GetHeight: Double;
begin
  Result := GetProp('height').AsFloat;
end;

procedure TPaintCanvas.SetHeight(Value: Double);
begin
  SetProp('height', TJSONFloatNumber.Create(Value));
end;

function TPaintCanvas.GetAlign: TAlign;
begin
  Result := AlignOf(GetProp('align'));
end;

procedure TPaintCanvas.SetAlign(Value: TAlign);
begin
  SetProp('align', TJSONString.Create(AlignNames[Value]));
end;

function TPaintCanvas.GetAnchors: TAnchorSet;
begin
  Result := AnchorSetOf(GetProp('anchors'));
end;

procedure TPaintCanvas.SetAnchors(Value: TAnchorSet);
begin
  SetProp('anchors', AnchorSetToJSON(Value));
end;

function TPaintCanvas.GetMargins: TAylisBounds;
begin
  Result := AylisBoundsFromJSON(GetProp('margins'));
end;

procedure TPaintCanvas.SetMargins(Value: TAylisBounds);
begin
  SetProp('margins', AylisBoundsToJSON(Value));
end;

function TPaintCanvas.GetPadding: TAylisBounds;
begin
  Result := AylisBoundsFromJSON(GetProp('padding'));
end;

procedure TPaintCanvas.SetPadding(Value: TAylisBounds);
begin
  SetProp('padding', AylisBoundsToJSON(Value));
end;

function TPaintCanvas.GetVisible: Boolean;
begin
  Result := GetProp('visible').AsBoolean;
end;

procedure TPaintCanvas.SetVisible(Value: Boolean);
begin
  SetProp('visible', TJSONBoolean.Create(Value));
end;

function TPaintCanvas.GetEnabled: Boolean;
begin
  Result := GetProp('enabled').AsBoolean;
end;

procedure TPaintCanvas.SetEnabled(Value: Boolean);
begin
  SetProp('enabled', TJSONBoolean.Create(Value));
end;

function TPaintCanvas.GetHitTest: Boolean;
begin
  Result := GetProp('hit_test').AsBoolean;
end;

procedure TPaintCanvas.SetHitTest(Value: Boolean);
begin
  SetProp('hit_test', TJSONBoolean.Create(Value));
end;

function TPaintCanvas.GetClipChildren: Boolean;
begin
  Result := GetProp('clip_children').AsBoolean;
end;

procedure TPaintCanvas.SetClipChildren(Value: Boolean);
begin
  SetProp('clip_children', TJSONBoolean.Create(Value));
end;

function TPaintCanvas.GetInheritState: Boolean;
begin
  Result := GetProp('inherit_state').AsBoolean;
end;

procedure TPaintCanvas.SetInheritState(Value: Boolean);
begin
  SetProp('inherit_state', TJSONBoolean.Create(Value));
end;

function TPaintCanvas.GetTranslate: Boolean;
begin
  Result := GetProp('translate').AsBoolean;
end;

procedure TPaintCanvas.SetTranslate(Value: Boolean);
begin
  SetProp('translate', TJSONBoolean.Create(Value));
end;

function TPaintCanvas.GetCapturesWheel: Boolean;
begin
  Result := GetProp('captures_wheel').AsBoolean;
end;

procedure TPaintCanvas.SetCapturesWheel(Value: Boolean);
begin
  SetProp('captures_wheel', TJSONBoolean.Create(Value));
end;

function TPaintCanvas.GetCapturesGesture: Boolean;
begin
  Result := GetProp('captures_gesture').AsBoolean;
end;

procedure TPaintCanvas.SetCapturesGesture(Value: Boolean);
begin
  SetProp('captures_gesture', TJSONBoolean.Create(Value));
end;

function TPaintCanvas.GetCanFocus: Boolean;
begin
  Result := GetProp('can_focus').AsBoolean;
end;

procedure TPaintCanvas.SetCanFocus(Value: Boolean);
begin
  SetProp('can_focus', TJSONBoolean.Create(Value));
end;

function TPaintCanvas.GetDraggable: Boolean;
begin
  Result := GetProp('draggable').AsBoolean;
end;

procedure TPaintCanvas.SetDraggable(Value: Boolean);
begin
  SetProp('draggable', TJSONBoolean.Create(Value));
end;

function TPaintCanvas.GetAcceptsDrops: Boolean;
begin
  Result := GetProp('accepts_drops').AsBoolean;
end;

procedure TPaintCanvas.SetAcceptsDrops(Value: Boolean);
begin
  SetProp('accepts_drops', TJSONBoolean.Create(Value));
end;

function TPaintCanvas.GetDragData: string;
begin
  Result := AsText(GetProp('drag_data'));
end;

procedure TPaintCanvas.SetDragData(Value: string);
begin
  SetProp('drag_data', TJSONString.Create(Value));
end;

function TPaintCanvas.GetDragLabel: string;
begin
  Result := AsText(GetProp('drag_label'));
end;

procedure TPaintCanvas.SetDragLabel(Value: string);
begin
  SetProp('drag_label', TJSONString.Create(Value));
end;

function TPaintCanvas.GetDragImage: TAylisComponent;
begin
  Result := GetRef('drag_image');
end;

procedure TPaintCanvas.SetDragImage(Value: TAylisComponent);
begin
  SetRef('drag_image', Value);
end;

function TPaintCanvas.GetDropTypes: string;
begin
  Result := AsText(GetProp('drop_types'));
end;

procedure TPaintCanvas.SetDropTypes(Value: string);
begin
  SetProp('drop_types', TJSONString.Create(Value));
end;

function TPaintCanvas.GetTabStop: Boolean;
begin
  Result := GetProp('tab_stop').AsBoolean;
end;

procedure TPaintCanvas.SetTabStop(Value: Boolean);
begin
  SetProp('tab_stop', TJSONBoolean.Create(Value));
end;

function TPaintCanvas.GetCursor: TCursorShape;
begin
  Result := CursorShapeOf(GetProp('cursor'));
end;

procedure TPaintCanvas.SetCursor(Value: TCursorShape);
begin
  SetProp('cursor', TJSONString.Create(CursorShapeNames[Value]));
end;

function TPaintCanvas.GetOpacity: Double;
begin
  Result := GetProp('opacity').AsFloat;
end;

procedure TPaintCanvas.SetOpacity(Value: Double);
begin
  SetProp('opacity', TJSONFloatNumber.Create(Value));
end;

function TPaintCanvas.GetBlendMode: TBlendMode;
begin
  Result := BlendModeOf(GetProp('blend_mode'));
end;

procedure TPaintCanvas.SetBlendMode(Value: TBlendMode);
begin
  SetProp('blend_mode', TJSONString.Create(BlendModeNames[Value]));
end;

function TPaintCanvas.GetCacheLayer: Boolean;
begin
  Result := GetProp('cache_layer').AsBoolean;
end;

procedure TPaintCanvas.SetCacheLayer(Value: Boolean);
begin
  SetProp('cache_layer', TJSONBoolean.Create(Value));
end;

function TPaintCanvas.GetRotation: Double;
begin
  Result := GetProp('rotation').AsFloat;
end;

procedure TPaintCanvas.SetRotation(Value: Double);
begin
  SetProp('rotation', TJSONFloatNumber.Create(Value));
end;

function TPaintCanvas.GetScale: Double;
begin
  Result := GetProp('scale').AsFloat;
end;

procedure TPaintCanvas.SetScale(Value: Double);
begin
  SetProp('scale', TJSONFloatNumber.Create(Value));
end;

function TPaintCanvas.GetTranslateX: Double;
begin
  Result := GetProp('translate_x').AsFloat;
end;

procedure TPaintCanvas.SetTranslateX(Value: Double);
begin
  SetProp('translate_x', TJSONFloatNumber.Create(Value));
end;

function TPaintCanvas.GetTranslateY: Double;
begin
  Result := GetProp('translate_y').AsFloat;
end;

procedure TPaintCanvas.SetTranslateY(Value: Double);
begin
  SetProp('translate_y', TJSONFloatNumber.Create(Value));
end;

function TPaintCanvas.GetAspectRatio: Double;
begin
  Result := GetProp('aspect_ratio').AsFloat;
end;

procedure TPaintCanvas.SetAspectRatio(Value: Double);
begin
  SetProp('aspect_ratio', TJSONFloatNumber.Create(Value));
end;

function TPaintCanvas.GetMaxWidth: Double;
begin
  Result := GetProp('max_width').AsFloat;
end;

procedure TPaintCanvas.SetMaxWidth(Value: Double);
begin
  SetProp('max_width', TJSONFloatNumber.Create(Value));
end;

function TPaintCanvas.GetMinWidth: Double;
begin
  Result := GetProp('min_width').AsFloat;
end;

procedure TPaintCanvas.SetMinWidth(Value: Double);
begin
  SetProp('min_width', TJSONFloatNumber.Create(Value));
end;

function TPaintCanvas.GetMinHeight: Double;
begin
  Result := GetProp('min_height').AsFloat;
end;

procedure TPaintCanvas.SetMinHeight(Value: Double);
begin
  SetProp('min_height', TJSONFloatNumber.Create(Value));
end;

function TPaintCanvas.GetMaxHeight: Double;
begin
  Result := GetProp('max_height').AsFloat;
end;

procedure TPaintCanvas.SetMaxHeight(Value: Double);
begin
  SetProp('max_height', TJSONFloatNumber.Create(Value));
end;

function TPaintCanvas.GetSelfAlign: TSelfAlign;
begin
  Result := SelfAlignOf(GetProp('self_align'));
end;

procedure TPaintCanvas.SetSelfAlign(Value: TSelfAlign);
begin
  SetProp('self_align', TJSONString.Create(SelfAlignNames[Value]));
end;

function TPaintCanvas.GetAutoSize: Boolean;
begin
  Result := GetProp('auto_size').AsBoolean;
end;

procedure TPaintCanvas.SetAutoSize(Value: Boolean);
begin
  SetProp('auto_size', TJSONBoolean.Create(Value));
end;

function TPaintCanvas.GetColumnSpan: Int64;
begin
  Result := GetProp('column_span').AsInt64;
end;

procedure TPaintCanvas.SetColumnSpan(Value: Int64);
begin
  SetProp('column_span', TJSONInt64Number.Create(Value));
end;

function TPaintCanvas.GetGridRow: Int64;
begin
  Result := GetProp('grid_row').AsInt64;
end;

procedure TPaintCanvas.SetGridRow(Value: Int64);
begin
  SetProp('grid_row', TJSONInt64Number.Create(Value));
end;

function TPaintCanvas.GetGridColumn: Int64;
begin
  Result := GetProp('grid_column').AsInt64;
end;

procedure TPaintCanvas.SetGridColumn(Value: Int64);
begin
  SetProp('grid_column', TJSONInt64Number.Create(Value));
end;

function TPaintCanvas.GetRowSpan: Int64;
begin
  Result := GetProp('row_span').AsInt64;
end;

procedure TPaintCanvas.SetRowSpan(Value: Int64);
begin
  SetProp('row_span', TJSONInt64Number.Create(Value));
end;

function TPaintCanvas.GetGrow: Double;
begin
  Result := GetProp('grow').AsFloat;
end;

procedure TPaintCanvas.SetGrow(Value: Double);
begin
  SetProp('grow', TJSONFloatNumber.Create(Value));
end;

function TPaintCanvas.GetShadow: TAylisShadow;
begin
  Result := AylisShadowFromJSON(GetProp('shadow'));
end;

procedure TPaintCanvas.SetShadow(Value: TAylisShadow);
begin
  SetProp('shadow', AylisShadowToJSON(Value));
end;

function TPaintCanvas.GetStyle: TPaintCanvasStyle;
begin
  Result := PaintCanvasStyleOf(GetProp('style'));
end;

procedure TPaintCanvas.SetStyle(Value: TPaintCanvasStyle);
begin
  SetProp('style', TJSONString.Create(PaintCanvasStyleNames[Value]));
end;

function TPaintCanvas.GetTheme: TThemeName;
begin
  Result := ThemeNameOf(GetProp('theme'));
end;

procedure TPaintCanvas.SetTheme(Value: TThemeName);
begin
  SetProp('theme', TJSONString.Create(ThemeNameNames[Value]));
end;

function TPaintCanvas.GetFill: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('fill'));
end;

procedure TPaintCanvas.SetFill(Value: TAylisColor);
begin
  SetProp('fill', AylisColorToJSON(Value));
end;

function TPaintCanvas.GetBorderColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('border_color'));
end;

procedure TPaintCanvas.SetBorderColor(Value: TAylisColor);
begin
  SetProp('border_color', AylisColorToJSON(Value));
end;

function TPaintCanvas.GetTextColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('text_color'));
end;

procedure TPaintCanvas.SetTextColor(Value: TAylisColor);
begin
  SetProp('text_color', AylisColorToJSON(Value));
end;

function TPaintCanvas.GetAccentColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('accent_color'));
end;

procedure TPaintCanvas.SetAccentColor(Value: TAylisColor);
begin
  SetProp('accent_color', AylisColorToJSON(Value));
end;

function TPaintCanvas.GetSuccessColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('success_color'));
end;

procedure TPaintCanvas.SetSuccessColor(Value: TAylisColor);
begin
  SetProp('success_color', AylisColorToJSON(Value));
end;

function TPaintCanvas.GetWarningColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('warning_color'));
end;

procedure TPaintCanvas.SetWarningColor(Value: TAylisColor);
begin
  SetProp('warning_color', AylisColorToJSON(Value));
end;

function TPaintCanvas.GetDangerColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('danger_color'));
end;

procedure TPaintCanvas.SetDangerColor(Value: TAylisColor);
begin
  SetProp('danger_color', AylisColorToJSON(Value));
end;

function TPaintCanvas.GetFontFamily: string;
begin
  Result := AsText(GetProp('font_family'));
end;

procedure TPaintCanvas.SetFontFamily(Value: string);
begin
  SetProp('font_family', TJSONString.Create(Value));
end;

function TPaintCanvas.GetFontSize: Double;
begin
  Result := GetProp('font_size').AsFloat;
end;

procedure TPaintCanvas.SetFontSize(Value: Double);
begin
  SetProp('font_size', TJSONFloatNumber.Create(Value));
end;

function TPaintCanvas.GetFontWeight: TFontWeight;
begin
  Result := FontWeightOf(GetProp('font_weight'));
end;

procedure TPaintCanvas.SetFontWeight(Value: TFontWeight);
begin
  SetProp('font_weight', TJSONString.Create(FontWeightNames[Value]));
end;

function TPaintCanvas.GetFontItalic: Boolean;
begin
  Result := GetProp('font_italic').AsBoolean;
end;

procedure TPaintCanvas.SetFontItalic(Value: Boolean);
begin
  SetProp('font_italic', TJSONBoolean.Create(Value));
end;

function TPaintCanvas.GetFontUnderline: Boolean;
begin
  Result := GetProp('font_underline').AsBoolean;
end;

procedure TPaintCanvas.SetFontUnderline(Value: Boolean);
begin
  SetProp('font_underline', TJSONBoolean.Create(Value));
end;

function TPaintCanvas.GetFontStrikeout: Boolean;
begin
  Result := GetProp('font_strikeout').AsBoolean;
end;

procedure TPaintCanvas.SetFontStrikeout(Value: Boolean);
begin
  SetProp('font_strikeout', TJSONBoolean.Create(Value));
end;

function TPaintCanvas.GetHint: string;
begin
  Result := AsText(GetProp('hint'));
end;

procedure TPaintCanvas.SetHint(Value: string);
begin
  SetProp('hint', TJSONString.Create(Value));
end;

function TPaintCanvas.GetTag: Int64;
begin
  Result := GetProp('tag').AsInt64;
end;

procedure TPaintCanvas.SetTag(Value: Int64);
begin
  SetProp('tag', TJSONInt64Number.Create(Value));
end;

function TPaintCanvas.GetTagString: string;
begin
  Result := AsText(GetProp('tag_string'));
end;

procedure TPaintCanvas.SetTagString(Value: string);
begin
  SetProp('tag_string', TJSONString.Create(Value));
end;

function TPaintCanvas.GetAccessibleName: string;
begin
  Result := AsText(GetProp('accessible_name'));
end;

procedure TPaintCanvas.SetAccessibleName(Value: string);
begin
  SetProp('accessible_name', TJSONString.Create(Value));
end;

function TPaintCanvas.GetPopupMenu: TAylisComponent;
begin
  Result := GetRef('popup_menu');
end;

procedure TPaintCanvas.SetPopupMenu(Value: TAylisComponent);
begin
  SetRef('popup_menu', Value);
end;

function TPaintCanvas.GetOnClick: string;
begin
  Result := AsText(GetProp('on_click'));
end;

procedure TPaintCanvas.SetOnClick(Value: string);
begin
  SetProp('on_click', TJSONString.Create(Value));
end;

function TPaintCanvas.GetOnDblClick: string;
begin
  Result := AsText(GetProp('on_dbl_click'));
end;

procedure TPaintCanvas.SetOnDblClick(Value: string);
begin
  SetProp('on_dbl_click', TJSONString.Create(Value));
end;

function TPaintCanvas.GetOnLongPress: string;
begin
  Result := AsText(GetProp('on_long_press'));
end;

procedure TPaintCanvas.SetOnLongPress(Value: string);
begin
  SetProp('on_long_press', TJSONString.Create(Value));
end;

function TPaintCanvas.GetOnPointerDown: string;
begin
  Result := AsText(GetProp('on_pointer_down'));
end;

procedure TPaintCanvas.SetOnPointerDown(Value: string);
begin
  SetProp('on_pointer_down', TJSONString.Create(Value));
end;

function TPaintCanvas.GetOnPointerUp: string;
begin
  Result := AsText(GetProp('on_pointer_up'));
end;

procedure TPaintCanvas.SetOnPointerUp(Value: string);
begin
  SetProp('on_pointer_up', TJSONString.Create(Value));
end;

function TPaintCanvas.GetOnPointerMove: string;
begin
  Result := AsText(GetProp('on_pointer_move'));
end;

procedure TPaintCanvas.SetOnPointerMove(Value: string);
begin
  SetProp('on_pointer_move', TJSONString.Create(Value));
end;

function TPaintCanvas.GetOnPointerEnter: string;
begin
  Result := AsText(GetProp('on_pointer_enter'));
end;

procedure TPaintCanvas.SetOnPointerEnter(Value: string);
begin
  SetProp('on_pointer_enter', TJSONString.Create(Value));
end;

function TPaintCanvas.GetOnPointerLeave: string;
begin
  Result := AsText(GetProp('on_pointer_leave'));
end;

procedure TPaintCanvas.SetOnPointerLeave(Value: string);
begin
  SetProp('on_pointer_leave', TJSONString.Create(Value));
end;

function TPaintCanvas.GetOnWheel: string;
begin
  Result := AsText(GetProp('on_wheel'));
end;

procedure TPaintCanvas.SetOnWheel(Value: string);
begin
  SetProp('on_wheel', TJSONString.Create(Value));
end;

function TPaintCanvas.GetOnGesture: string;
begin
  Result := AsText(GetProp('on_gesture'));
end;

procedure TPaintCanvas.SetOnGesture(Value: string);
begin
  SetProp('on_gesture', TJSONString.Create(Value));
end;

function TPaintCanvas.GetOnSwipe: string;
begin
  Result := AsText(GetProp('on_swipe'));
end;

procedure TPaintCanvas.SetOnSwipe(Value: string);
begin
  SetProp('on_swipe', TJSONString.Create(Value));
end;

function TPaintCanvas.GetOnKeyUp: string;
begin
  Result := AsText(GetProp('on_key_up'));
end;

procedure TPaintCanvas.SetOnKeyUp(Value: string);
begin
  SetProp('on_key_up', TJSONString.Create(Value));
end;

function TPaintCanvas.GetOnEnter: string;
begin
  Result := AsText(GetProp('on_enter'));
end;

procedure TPaintCanvas.SetOnEnter(Value: string);
begin
  SetProp('on_enter', TJSONString.Create(Value));
end;

function TPaintCanvas.GetOnExit: string;
begin
  Result := AsText(GetProp('on_exit'));
end;

procedure TPaintCanvas.SetOnExit(Value: string);
begin
  SetProp('on_exit', TJSONString.Create(Value));
end;

function TPaintCanvas.GetOnResize: string;
begin
  Result := AsText(GetProp('on_resize'));
end;

procedure TPaintCanvas.SetOnResize(Value: string);
begin
  SetProp('on_resize', TJSONString.Create(Value));
end;

function TPaintCanvas.GetOnDragMove: string;
begin
  Result := AsText(GetProp('on_drag_move'));
end;

procedure TPaintCanvas.SetOnDragMove(Value: string);
begin
  SetProp('on_drag_move', TJSONString.Create(Value));
end;

function TPaintCanvas.GetOnDragLeave: string;
begin
  Result := AsText(GetProp('on_drag_leave'));
end;

procedure TPaintCanvas.SetOnDragLeave(Value: string);
begin
  SetProp('on_drag_leave', TJSONString.Create(Value));
end;

function TPaintCanvas.GetOnDragDrop: string;
begin
  Result := AsText(GetProp('on_drag_drop'));
end;

procedure TPaintCanvas.SetOnDragDrop(Value: string);
begin
  SetProp('on_drag_drop', TJSONString.Create(Value));
end;

function TPaintCanvas.GetOnDragEnd: string;
begin
  Result := AsText(GetProp('on_drag_end'));
end;

procedure TPaintCanvas.SetOnDragEnd(Value: string);
begin
  SetProp('on_drag_end', TJSONString.Create(Value));
end;

function TPaintCanvas.Undo: Boolean;
begin
  Result := Aylis.Call('invoke', TJSONObject.Create(['target', Id, 'method', 'undo', 'args', TJSONArray.Create])).AsBoolean;
end;

function TPaintCanvas.Redo: Boolean;
begin
  Result := Aylis.Call('invoke', TJSONObject.Create(['target', Id, 'method', 'redo', 'args', TJSONArray.Create])).AsBoolean;
end;

function TPaintCanvas.CanUndo: Boolean;
begin
  Result := Aylis.Call('invoke', TJSONObject.Create(['target', Id, 'method', 'can_undo', 'args', TJSONArray.Create])).AsBoolean;
end;

function TPaintCanvas.CanRedo: Boolean;
begin
  Result := Aylis.Call('invoke', TJSONObject.Create(['target', Id, 'method', 'can_redo', 'args', TJSONArray.Create])).AsBoolean;
end;

function TPaintCanvas.Size: TImageSize;
begin
  Result := ImageSizeFromText(Aylis.Call('invoke', TJSONObject.Create(['target', Id, 'method', 'size', 'args', TJSONArray.Create])));
end;

function TPaintCanvas.Checksum: string;
begin
  Result := AsText(Aylis.Call('invoke', TJSONObject.Create(['target', Id, 'method', 'checksum', 'args', TJSONArray.Create])));
end;

function TPaintCanvas.Clear: Boolean;
begin
  Result := Aylis.Call('invoke', TJSONObject.Create(['target', Id, 'method', 'clear', 'args', TJSONArray.Create])).AsBoolean;
end;

function TPaintCanvas.Resize(AWidth: Int64; AHeight: Int64): Boolean;
begin
  Result := Aylis.Call('invoke', TJSONObject.Create(['target', Id, 'method', 'resize', 'args', TJSONArray.Create([AWidth, AHeight])])).AsBoolean;
end;

function TPaintCanvas.New(AWidth: Int64; AHeight: Int64): Boolean;
begin
  Result := Aylis.Call('invoke', TJSONObject.Create(['target', Id, 'method', 'new', 'args', TJSONArray.Create([AWidth, AHeight])])).AsBoolean;
end;

function TPaintCanvas.Open(APath: string): Boolean;
begin
  Result := Aylis.Call('invoke', TJSONObject.Create(['target', Id, 'method', 'open', 'args', TJSONArray.Create([APath])])).AsBoolean;
end;

function TPaintCanvas.Save(APath: string): Boolean;
begin
  Result := Aylis.Call('invoke', TJSONObject.Create(['target', Id, 'method', 'save', 'args', TJSONArray.Create([APath])])).AsBoolean;
end;

function TPaintCanvas.SelectAll: Boolean;
begin
  Result := Aylis.Call('invoke', TJSONObject.Create(['target', Id, 'method', 'select_all', 'args', TJSONArray.Create])).AsBoolean;
end;

function TPaintCanvas.HasSelection: Boolean;
begin
  Result := Aylis.Call('invoke', TJSONObject.Create(['target', Id, 'method', 'has_selection', 'args', TJSONArray.Create])).AsBoolean;
end;

function TPaintCanvas.Copy: Boolean;
begin
  Result := Aylis.Call('invoke', TJSONObject.Create(['target', Id, 'method', 'copy', 'args', TJSONArray.Create])).AsBoolean;
end;

function TPaintCanvas.Cut: Boolean;
begin
  Result := Aylis.Call('invoke', TJSONObject.Create(['target', Id, 'method', 'cut', 'args', TJSONArray.Create])).AsBoolean;
end;

function TPaintCanvas.Paste: Boolean;
begin
  Result := Aylis.Call('invoke', TJSONObject.Create(['target', Id, 'method', 'paste', 'args', TJSONArray.Create])).AsBoolean;
end;

function TPaintCanvas.Delete: Boolean;
begin
  Result := Aylis.Call('invoke', TJSONObject.Create(['target', Id, 'method', 'delete', 'args', TJSONArray.Create])).AsBoolean;
end;

function TPaintCanvas.Deselect: Boolean;
begin
  Result := Aylis.Call('invoke', TJSONObject.Create(['target', Id, 'method', 'deselect', 'args', TJSONArray.Create])).AsBoolean;
end;

procedure TPaintCanvas.SetFocus;
begin
  Aylis.Send('focus', TJSONObject.Create(['target', Id]));
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

function TSaveDialog.GetTitle: string;
begin
  Result := AsText(GetProp('title'));
end;

procedure TSaveDialog.SetTitle(Value: string);
begin
  SetProp('title', TJSONString.Create(Value));
end;

function TSaveDialog.GetFilter: string;
begin
  Result := AsText(GetProp('filter'));
end;

procedure TSaveDialog.SetFilter(Value: string);
begin
  SetProp('filter', TJSONString.Create(Value));
end;

function TSaveDialog.GetInitialDir: string;
begin
  Result := AsText(GetProp('initial_dir'));
end;

procedure TSaveDialog.SetInitialDir(Value: string);
begin
  SetProp('initial_dir', TJSONString.Create(Value));
end;

function TSaveDialog.GetFileName: string;
begin
  Result := AsText(GetProp('file_name'));
end;

procedure TSaveDialog.SetFileName(Value: string);
begin
  SetProp('file_name', TJSONString.Create(Value));
end;

function TSaveDialog.GetMultiple: Boolean;
begin
  Result := GetProp('multiple').AsBoolean;
end;

procedure TSaveDialog.SetMultiple(Value: Boolean);
begin
  SetProp('multiple', TJSONBoolean.Create(Value));
end;

function TSaveDialog.GetOnSelect: string;
begin
  Result := AsText(GetProp('on_select'));
end;

procedure TSaveDialog.SetOnSelect(Value: string);
begin
  SetProp('on_select', TJSONString.Create(Value));
end;

function TSaveDialog.GetOnCancel: string;
begin
  Result := AsText(GetProp('on_cancel'));
end;

procedure TSaveDialog.SetOnCancel(Value: string);
begin
  SetProp('on_cancel', TJSONString.Create(Value));
end;

procedure TSaveDialog.Execute;
begin
  Aylis.Send('execute', TJSONObject.Create(['target', Id]));
end;

function TScrollBox.GetOverscroll: TOverscroll;
begin
  Result := OverscrollOf(GetProp('overscroll'));
end;

procedure TScrollBox.SetOverscroll(Value: TOverscroll);
begin
  SetProp('overscroll', TJSONString.Create(OverscrollNames[Value]));
end;

function TScrollBox.GetHorizontal: Boolean;
begin
  Result := GetProp('horizontal').AsBoolean;
end;

procedure TScrollBox.SetHorizontal(Value: Boolean);
begin
  SetProp('horizontal', TJSONBoolean.Create(Value));
end;

function TScrollBox.GetPullToRefresh: Boolean;
begin
  Result := GetProp('pull_to_refresh').AsBoolean;
end;

procedure TScrollBox.SetPullToRefresh(Value: Boolean);
begin
  SetProp('pull_to_refresh', TJSONBoolean.Create(Value));
end;

function TScrollBox.GetRefreshing: Boolean;
begin
  Result := GetProp('refreshing').AsBoolean;
end;

procedure TScrollBox.SetRefreshing(Value: Boolean);
begin
  SetProp('refreshing', TJSONBoolean.Create(Value));
end;

function TScrollBox.GetWheelStep: Double;
begin
  Result := GetProp('wheel_step').AsFloat;
end;

procedure TScrollBox.SetWheelStep(Value: Double);
begin
  SetProp('wheel_step', TJSONFloatNumber.Create(Value));
end;

function TScrollBox.GetSmooth: TSmoothScroll;
begin
  Result := SmoothScrollOf(GetProp('smooth'));
end;

procedure TScrollBox.SetSmooth(Value: TSmoothScroll);
begin
  SetProp('smooth', TJSONString.Create(SmoothScrollNames[Value]));
end;

function TScrollBox.GetGutter: Boolean;
begin
  Result := GetProp('gutter').AsBoolean;
end;

procedure TScrollBox.SetGutter(Value: Boolean);
begin
  SetProp('gutter', TJSONBoolean.Create(Value));
end;

function TScrollBox.GetAllowedChars: string;
begin
  Result := AsText(GetProp('allowed_chars'));
end;

procedure TScrollBox.SetAllowedChars(Value: string);
begin
  SetProp('allowed_chars', TJSONString.Create(Value));
end;

function TScrollBox.GetBlockedChars: string;
begin
  Result := AsText(GetProp('blocked_chars'));
end;

procedure TScrollBox.SetBlockedChars(Value: string);
begin
  SetProp('blocked_chars', TJSONString.Create(Value));
end;

function TScrollBox.GetOnlyAllowed: Boolean;
begin
  Result := GetProp('only_allowed').AsBoolean;
end;

procedure TScrollBox.SetOnlyAllowed(Value: Boolean);
begin
  SetProp('only_allowed', TJSONBoolean.Create(Value));
end;

function TScrollBox.GetCharsOverride: Boolean;
begin
  Result := GetProp('chars_override').AsBoolean;
end;

procedure TScrollBox.SetCharsOverride(Value: Boolean);
begin
  SetProp('chars_override', TJSONBoolean.Create(Value));
end;

function TScrollBox.GetCharMap: string;
begin
  Result := AsText(GetProp('char_map'));
end;

procedure TScrollBox.SetCharMap(Value: string);
begin
  SetProp('char_map', TJSONString.Create(Value));
end;

function TScrollBox.GetOnEndReached: string;
begin
  Result := AsText(GetProp('on_end_reached'));
end;

procedure TScrollBox.SetOnEndReached(Value: string);
begin
  SetProp('on_end_reached', TJSONString.Create(Value));
end;

function TScrollBox.GetOnRefresh: string;
begin
  Result := AsText(GetProp('on_refresh'));
end;

procedure TScrollBox.SetOnRefresh(Value: string);
begin
  SetProp('on_refresh', TJSONString.Create(Value));
end;

function TScrollBox.GetOnScroll: string;
begin
  Result := AsText(GetProp('on_scroll'));
end;

procedure TScrollBox.SetOnScroll(Value: string);
begin
  SetProp('on_scroll', TJSONString.Create(Value));
end;

function TScrollBox.GetScrollPosition: Double;
begin
  Result := GetProp('scroll_position').AsFloat;
end;

procedure TScrollBox.SetScrollPosition(Value: Double);
begin
  SetProp('scroll_position', TJSONFloatNumber.Create(Value));
end;

function TScrollBox.GetLeft: Double;
begin
  Result := GetProp('left').AsFloat;
end;

procedure TScrollBox.SetLeft(Value: Double);
begin
  SetProp('left', TJSONFloatNumber.Create(Value));
end;

function TScrollBox.GetTop: Double;
begin
  Result := GetProp('top').AsFloat;
end;

procedure TScrollBox.SetTop(Value: Double);
begin
  SetProp('top', TJSONFloatNumber.Create(Value));
end;

function TScrollBox.GetWidth: Double;
begin
  Result := GetProp('width').AsFloat;
end;

procedure TScrollBox.SetWidth(Value: Double);
begin
  SetProp('width', TJSONFloatNumber.Create(Value));
end;

function TScrollBox.GetHeight: Double;
begin
  Result := GetProp('height').AsFloat;
end;

procedure TScrollBox.SetHeight(Value: Double);
begin
  SetProp('height', TJSONFloatNumber.Create(Value));
end;

function TScrollBox.GetAlign: TAlign;
begin
  Result := AlignOf(GetProp('align'));
end;

procedure TScrollBox.SetAlign(Value: TAlign);
begin
  SetProp('align', TJSONString.Create(AlignNames[Value]));
end;

function TScrollBox.GetAnchors: TAnchorSet;
begin
  Result := AnchorSetOf(GetProp('anchors'));
end;

procedure TScrollBox.SetAnchors(Value: TAnchorSet);
begin
  SetProp('anchors', AnchorSetToJSON(Value));
end;

function TScrollBox.GetMargins: TAylisBounds;
begin
  Result := AylisBoundsFromJSON(GetProp('margins'));
end;

procedure TScrollBox.SetMargins(Value: TAylisBounds);
begin
  SetProp('margins', AylisBoundsToJSON(Value));
end;

function TScrollBox.GetPadding: TAylisBounds;
begin
  Result := AylisBoundsFromJSON(GetProp('padding'));
end;

procedure TScrollBox.SetPadding(Value: TAylisBounds);
begin
  SetProp('padding', AylisBoundsToJSON(Value));
end;

function TScrollBox.GetVisible: Boolean;
begin
  Result := GetProp('visible').AsBoolean;
end;

procedure TScrollBox.SetVisible(Value: Boolean);
begin
  SetProp('visible', TJSONBoolean.Create(Value));
end;

function TScrollBox.GetEnabled: Boolean;
begin
  Result := GetProp('enabled').AsBoolean;
end;

procedure TScrollBox.SetEnabled(Value: Boolean);
begin
  SetProp('enabled', TJSONBoolean.Create(Value));
end;

function TScrollBox.GetHitTest: Boolean;
begin
  Result := GetProp('hit_test').AsBoolean;
end;

procedure TScrollBox.SetHitTest(Value: Boolean);
begin
  SetProp('hit_test', TJSONBoolean.Create(Value));
end;

function TScrollBox.GetClipChildren: Boolean;
begin
  Result := GetProp('clip_children').AsBoolean;
end;

procedure TScrollBox.SetClipChildren(Value: Boolean);
begin
  SetProp('clip_children', TJSONBoolean.Create(Value));
end;

function TScrollBox.GetInheritState: Boolean;
begin
  Result := GetProp('inherit_state').AsBoolean;
end;

procedure TScrollBox.SetInheritState(Value: Boolean);
begin
  SetProp('inherit_state', TJSONBoolean.Create(Value));
end;

function TScrollBox.GetTranslate: Boolean;
begin
  Result := GetProp('translate').AsBoolean;
end;

procedure TScrollBox.SetTranslate(Value: Boolean);
begin
  SetProp('translate', TJSONBoolean.Create(Value));
end;

function TScrollBox.GetCapturesWheel: Boolean;
begin
  Result := GetProp('captures_wheel').AsBoolean;
end;

procedure TScrollBox.SetCapturesWheel(Value: Boolean);
begin
  SetProp('captures_wheel', TJSONBoolean.Create(Value));
end;

function TScrollBox.GetCapturesGesture: Boolean;
begin
  Result := GetProp('captures_gesture').AsBoolean;
end;

procedure TScrollBox.SetCapturesGesture(Value: Boolean);
begin
  SetProp('captures_gesture', TJSONBoolean.Create(Value));
end;

function TScrollBox.GetCanFocus: Boolean;
begin
  Result := GetProp('can_focus').AsBoolean;
end;

procedure TScrollBox.SetCanFocus(Value: Boolean);
begin
  SetProp('can_focus', TJSONBoolean.Create(Value));
end;

function TScrollBox.GetDraggable: Boolean;
begin
  Result := GetProp('draggable').AsBoolean;
end;

procedure TScrollBox.SetDraggable(Value: Boolean);
begin
  SetProp('draggable', TJSONBoolean.Create(Value));
end;

function TScrollBox.GetAcceptsDrops: Boolean;
begin
  Result := GetProp('accepts_drops').AsBoolean;
end;

procedure TScrollBox.SetAcceptsDrops(Value: Boolean);
begin
  SetProp('accepts_drops', TJSONBoolean.Create(Value));
end;

function TScrollBox.GetDragData: string;
begin
  Result := AsText(GetProp('drag_data'));
end;

procedure TScrollBox.SetDragData(Value: string);
begin
  SetProp('drag_data', TJSONString.Create(Value));
end;

function TScrollBox.GetDragLabel: string;
begin
  Result := AsText(GetProp('drag_label'));
end;

procedure TScrollBox.SetDragLabel(Value: string);
begin
  SetProp('drag_label', TJSONString.Create(Value));
end;

function TScrollBox.GetDragImage: TAylisComponent;
begin
  Result := GetRef('drag_image');
end;

procedure TScrollBox.SetDragImage(Value: TAylisComponent);
begin
  SetRef('drag_image', Value);
end;

function TScrollBox.GetDropTypes: string;
begin
  Result := AsText(GetProp('drop_types'));
end;

procedure TScrollBox.SetDropTypes(Value: string);
begin
  SetProp('drop_types', TJSONString.Create(Value));
end;

function TScrollBox.GetTabStop: Boolean;
begin
  Result := GetProp('tab_stop').AsBoolean;
end;

procedure TScrollBox.SetTabStop(Value: Boolean);
begin
  SetProp('tab_stop', TJSONBoolean.Create(Value));
end;

function TScrollBox.GetCursor: TCursorShape;
begin
  Result := CursorShapeOf(GetProp('cursor'));
end;

procedure TScrollBox.SetCursor(Value: TCursorShape);
begin
  SetProp('cursor', TJSONString.Create(CursorShapeNames[Value]));
end;

function TScrollBox.GetOpacity: Double;
begin
  Result := GetProp('opacity').AsFloat;
end;

procedure TScrollBox.SetOpacity(Value: Double);
begin
  SetProp('opacity', TJSONFloatNumber.Create(Value));
end;

function TScrollBox.GetBlendMode: TBlendMode;
begin
  Result := BlendModeOf(GetProp('blend_mode'));
end;

procedure TScrollBox.SetBlendMode(Value: TBlendMode);
begin
  SetProp('blend_mode', TJSONString.Create(BlendModeNames[Value]));
end;

function TScrollBox.GetCacheLayer: Boolean;
begin
  Result := GetProp('cache_layer').AsBoolean;
end;

procedure TScrollBox.SetCacheLayer(Value: Boolean);
begin
  SetProp('cache_layer', TJSONBoolean.Create(Value));
end;

function TScrollBox.GetRotation: Double;
begin
  Result := GetProp('rotation').AsFloat;
end;

procedure TScrollBox.SetRotation(Value: Double);
begin
  SetProp('rotation', TJSONFloatNumber.Create(Value));
end;

function TScrollBox.GetScale: Double;
begin
  Result := GetProp('scale').AsFloat;
end;

procedure TScrollBox.SetScale(Value: Double);
begin
  SetProp('scale', TJSONFloatNumber.Create(Value));
end;

function TScrollBox.GetTranslateX: Double;
begin
  Result := GetProp('translate_x').AsFloat;
end;

procedure TScrollBox.SetTranslateX(Value: Double);
begin
  SetProp('translate_x', TJSONFloatNumber.Create(Value));
end;

function TScrollBox.GetTranslateY: Double;
begin
  Result := GetProp('translate_y').AsFloat;
end;

procedure TScrollBox.SetTranslateY(Value: Double);
begin
  SetProp('translate_y', TJSONFloatNumber.Create(Value));
end;

function TScrollBox.GetAspectRatio: Double;
begin
  Result := GetProp('aspect_ratio').AsFloat;
end;

procedure TScrollBox.SetAspectRatio(Value: Double);
begin
  SetProp('aspect_ratio', TJSONFloatNumber.Create(Value));
end;

function TScrollBox.GetMaxWidth: Double;
begin
  Result := GetProp('max_width').AsFloat;
end;

procedure TScrollBox.SetMaxWidth(Value: Double);
begin
  SetProp('max_width', TJSONFloatNumber.Create(Value));
end;

function TScrollBox.GetMinWidth: Double;
begin
  Result := GetProp('min_width').AsFloat;
end;

procedure TScrollBox.SetMinWidth(Value: Double);
begin
  SetProp('min_width', TJSONFloatNumber.Create(Value));
end;

function TScrollBox.GetMinHeight: Double;
begin
  Result := GetProp('min_height').AsFloat;
end;

procedure TScrollBox.SetMinHeight(Value: Double);
begin
  SetProp('min_height', TJSONFloatNumber.Create(Value));
end;

function TScrollBox.GetMaxHeight: Double;
begin
  Result := GetProp('max_height').AsFloat;
end;

procedure TScrollBox.SetMaxHeight(Value: Double);
begin
  SetProp('max_height', TJSONFloatNumber.Create(Value));
end;

function TScrollBox.GetSelfAlign: TSelfAlign;
begin
  Result := SelfAlignOf(GetProp('self_align'));
end;

procedure TScrollBox.SetSelfAlign(Value: TSelfAlign);
begin
  SetProp('self_align', TJSONString.Create(SelfAlignNames[Value]));
end;

function TScrollBox.GetAutoSize: Boolean;
begin
  Result := GetProp('auto_size').AsBoolean;
end;

procedure TScrollBox.SetAutoSize(Value: Boolean);
begin
  SetProp('auto_size', TJSONBoolean.Create(Value));
end;

function TScrollBox.GetColumnSpan: Int64;
begin
  Result := GetProp('column_span').AsInt64;
end;

procedure TScrollBox.SetColumnSpan(Value: Int64);
begin
  SetProp('column_span', TJSONInt64Number.Create(Value));
end;

function TScrollBox.GetGridRow: Int64;
begin
  Result := GetProp('grid_row').AsInt64;
end;

procedure TScrollBox.SetGridRow(Value: Int64);
begin
  SetProp('grid_row', TJSONInt64Number.Create(Value));
end;

function TScrollBox.GetGridColumn: Int64;
begin
  Result := GetProp('grid_column').AsInt64;
end;

procedure TScrollBox.SetGridColumn(Value: Int64);
begin
  SetProp('grid_column', TJSONInt64Number.Create(Value));
end;

function TScrollBox.GetRowSpan: Int64;
begin
  Result := GetProp('row_span').AsInt64;
end;

procedure TScrollBox.SetRowSpan(Value: Int64);
begin
  SetProp('row_span', TJSONInt64Number.Create(Value));
end;

function TScrollBox.GetGrow: Double;
begin
  Result := GetProp('grow').AsFloat;
end;

procedure TScrollBox.SetGrow(Value: Double);
begin
  SetProp('grow', TJSONFloatNumber.Create(Value));
end;

function TScrollBox.GetShadow: TAylisShadow;
begin
  Result := AylisShadowFromJSON(GetProp('shadow'));
end;

procedure TScrollBox.SetShadow(Value: TAylisShadow);
begin
  SetProp('shadow', AylisShadowToJSON(Value));
end;

function TScrollBox.GetStyle: TScrollBoxStyle;
begin
  Result := ScrollBoxStyleOf(GetProp('style'));
end;

procedure TScrollBox.SetStyle(Value: TScrollBoxStyle);
begin
  SetProp('style', TJSONString.Create(ScrollBoxStyleNames[Value]));
end;

function TScrollBox.GetTheme: TThemeName;
begin
  Result := ThemeNameOf(GetProp('theme'));
end;

procedure TScrollBox.SetTheme(Value: TThemeName);
begin
  SetProp('theme', TJSONString.Create(ThemeNameNames[Value]));
end;

function TScrollBox.GetFill: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('fill'));
end;

procedure TScrollBox.SetFill(Value: TAylisColor);
begin
  SetProp('fill', AylisColorToJSON(Value));
end;

function TScrollBox.GetBorderColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('border_color'));
end;

procedure TScrollBox.SetBorderColor(Value: TAylisColor);
begin
  SetProp('border_color', AylisColorToJSON(Value));
end;

function TScrollBox.GetTextColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('text_color'));
end;

procedure TScrollBox.SetTextColor(Value: TAylisColor);
begin
  SetProp('text_color', AylisColorToJSON(Value));
end;

function TScrollBox.GetAccentColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('accent_color'));
end;

procedure TScrollBox.SetAccentColor(Value: TAylisColor);
begin
  SetProp('accent_color', AylisColorToJSON(Value));
end;

function TScrollBox.GetSuccessColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('success_color'));
end;

procedure TScrollBox.SetSuccessColor(Value: TAylisColor);
begin
  SetProp('success_color', AylisColorToJSON(Value));
end;

function TScrollBox.GetWarningColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('warning_color'));
end;

procedure TScrollBox.SetWarningColor(Value: TAylisColor);
begin
  SetProp('warning_color', AylisColorToJSON(Value));
end;

function TScrollBox.GetDangerColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('danger_color'));
end;

procedure TScrollBox.SetDangerColor(Value: TAylisColor);
begin
  SetProp('danger_color', AylisColorToJSON(Value));
end;

function TScrollBox.GetFontFamily: string;
begin
  Result := AsText(GetProp('font_family'));
end;

procedure TScrollBox.SetFontFamily(Value: string);
begin
  SetProp('font_family', TJSONString.Create(Value));
end;

function TScrollBox.GetFontSize: Double;
begin
  Result := GetProp('font_size').AsFloat;
end;

procedure TScrollBox.SetFontSize(Value: Double);
begin
  SetProp('font_size', TJSONFloatNumber.Create(Value));
end;

function TScrollBox.GetFontWeight: TFontWeight;
begin
  Result := FontWeightOf(GetProp('font_weight'));
end;

procedure TScrollBox.SetFontWeight(Value: TFontWeight);
begin
  SetProp('font_weight', TJSONString.Create(FontWeightNames[Value]));
end;

function TScrollBox.GetFontItalic: Boolean;
begin
  Result := GetProp('font_italic').AsBoolean;
end;

procedure TScrollBox.SetFontItalic(Value: Boolean);
begin
  SetProp('font_italic', TJSONBoolean.Create(Value));
end;

function TScrollBox.GetFontUnderline: Boolean;
begin
  Result := GetProp('font_underline').AsBoolean;
end;

procedure TScrollBox.SetFontUnderline(Value: Boolean);
begin
  SetProp('font_underline', TJSONBoolean.Create(Value));
end;

function TScrollBox.GetFontStrikeout: Boolean;
begin
  Result := GetProp('font_strikeout').AsBoolean;
end;

procedure TScrollBox.SetFontStrikeout(Value: Boolean);
begin
  SetProp('font_strikeout', TJSONBoolean.Create(Value));
end;

function TScrollBox.GetHint: string;
begin
  Result := AsText(GetProp('hint'));
end;

procedure TScrollBox.SetHint(Value: string);
begin
  SetProp('hint', TJSONString.Create(Value));
end;

function TScrollBox.GetTag: Int64;
begin
  Result := GetProp('tag').AsInt64;
end;

procedure TScrollBox.SetTag(Value: Int64);
begin
  SetProp('tag', TJSONInt64Number.Create(Value));
end;

function TScrollBox.GetTagString: string;
begin
  Result := AsText(GetProp('tag_string'));
end;

procedure TScrollBox.SetTagString(Value: string);
begin
  SetProp('tag_string', TJSONString.Create(Value));
end;

function TScrollBox.GetAccessibleName: string;
begin
  Result := AsText(GetProp('accessible_name'));
end;

procedure TScrollBox.SetAccessibleName(Value: string);
begin
  SetProp('accessible_name', TJSONString.Create(Value));
end;

function TScrollBox.GetPopupMenu: TAylisComponent;
begin
  Result := GetRef('popup_menu');
end;

procedure TScrollBox.SetPopupMenu(Value: TAylisComponent);
begin
  SetRef('popup_menu', Value);
end;

function TScrollBox.GetOnClick: string;
begin
  Result := AsText(GetProp('on_click'));
end;

procedure TScrollBox.SetOnClick(Value: string);
begin
  SetProp('on_click', TJSONString.Create(Value));
end;

function TScrollBox.GetOnDblClick: string;
begin
  Result := AsText(GetProp('on_dbl_click'));
end;

procedure TScrollBox.SetOnDblClick(Value: string);
begin
  SetProp('on_dbl_click', TJSONString.Create(Value));
end;

function TScrollBox.GetOnLongPress: string;
begin
  Result := AsText(GetProp('on_long_press'));
end;

procedure TScrollBox.SetOnLongPress(Value: string);
begin
  SetProp('on_long_press', TJSONString.Create(Value));
end;

function TScrollBox.GetOnPointerDown: string;
begin
  Result := AsText(GetProp('on_pointer_down'));
end;

procedure TScrollBox.SetOnPointerDown(Value: string);
begin
  SetProp('on_pointer_down', TJSONString.Create(Value));
end;

function TScrollBox.GetOnPointerUp: string;
begin
  Result := AsText(GetProp('on_pointer_up'));
end;

procedure TScrollBox.SetOnPointerUp(Value: string);
begin
  SetProp('on_pointer_up', TJSONString.Create(Value));
end;

function TScrollBox.GetOnPointerMove: string;
begin
  Result := AsText(GetProp('on_pointer_move'));
end;

procedure TScrollBox.SetOnPointerMove(Value: string);
begin
  SetProp('on_pointer_move', TJSONString.Create(Value));
end;

function TScrollBox.GetOnPointerEnter: string;
begin
  Result := AsText(GetProp('on_pointer_enter'));
end;

procedure TScrollBox.SetOnPointerEnter(Value: string);
begin
  SetProp('on_pointer_enter', TJSONString.Create(Value));
end;

function TScrollBox.GetOnPointerLeave: string;
begin
  Result := AsText(GetProp('on_pointer_leave'));
end;

procedure TScrollBox.SetOnPointerLeave(Value: string);
begin
  SetProp('on_pointer_leave', TJSONString.Create(Value));
end;

function TScrollBox.GetOnWheel: string;
begin
  Result := AsText(GetProp('on_wheel'));
end;

procedure TScrollBox.SetOnWheel(Value: string);
begin
  SetProp('on_wheel', TJSONString.Create(Value));
end;

function TScrollBox.GetOnGesture: string;
begin
  Result := AsText(GetProp('on_gesture'));
end;

procedure TScrollBox.SetOnGesture(Value: string);
begin
  SetProp('on_gesture', TJSONString.Create(Value));
end;

function TScrollBox.GetOnSwipe: string;
begin
  Result := AsText(GetProp('on_swipe'));
end;

procedure TScrollBox.SetOnSwipe(Value: string);
begin
  SetProp('on_swipe', TJSONString.Create(Value));
end;

function TScrollBox.GetOnKeyUp: string;
begin
  Result := AsText(GetProp('on_key_up'));
end;

procedure TScrollBox.SetOnKeyUp(Value: string);
begin
  SetProp('on_key_up', TJSONString.Create(Value));
end;

function TScrollBox.GetOnEnter: string;
begin
  Result := AsText(GetProp('on_enter'));
end;

procedure TScrollBox.SetOnEnter(Value: string);
begin
  SetProp('on_enter', TJSONString.Create(Value));
end;

function TScrollBox.GetOnExit: string;
begin
  Result := AsText(GetProp('on_exit'));
end;

procedure TScrollBox.SetOnExit(Value: string);
begin
  SetProp('on_exit', TJSONString.Create(Value));
end;

function TScrollBox.GetOnResize: string;
begin
  Result := AsText(GetProp('on_resize'));
end;

procedure TScrollBox.SetOnResize(Value: string);
begin
  SetProp('on_resize', TJSONString.Create(Value));
end;

function TScrollBox.GetOnDragMove: string;
begin
  Result := AsText(GetProp('on_drag_move'));
end;

procedure TScrollBox.SetOnDragMove(Value: string);
begin
  SetProp('on_drag_move', TJSONString.Create(Value));
end;

function TScrollBox.GetOnDragLeave: string;
begin
  Result := AsText(GetProp('on_drag_leave'));
end;

procedure TScrollBox.SetOnDragLeave(Value: string);
begin
  SetProp('on_drag_leave', TJSONString.Create(Value));
end;

function TScrollBox.GetOnDragDrop: string;
begin
  Result := AsText(GetProp('on_drag_drop'));
end;

procedure TScrollBox.SetOnDragDrop(Value: string);
begin
  SetProp('on_drag_drop', TJSONString.Create(Value));
end;

function TScrollBox.GetOnDragEnd: string;
begin
  Result := AsText(GetProp('on_drag_end'));
end;

procedure TScrollBox.SetOnDragEnd(Value: string);
begin
  SetProp('on_drag_end', TJSONString.Create(Value));
end;

procedure TScrollBox.ScrollTo(AComponent: TAylisComponent);
begin
  Aylis.Send('invoke', TJSONObject.Create(['target', Id, 'method', 'scroll_to', 'args', TJSONArray.Create([AComponent.Id])]));
end;

function TScrollBox.VisibleComponents(AList: TAylisComponent; AFully: Boolean): string;
begin
  Result := AsText(Aylis.Call('invoke', TJSONObject.Create(['target', Id, 'method', 'visible_components', 'args', TJSONArray.Create([AList.Id, AFully])])));
end;

function TStatusBar.GetText: string;
begin
  Result := AsText(GetProp('text'));
end;

procedure TStatusBar.SetText(Value: string);
begin
  SetProp('text', TJSONString.Create(Value));
end;

function TStatusBar.GetLeft: Double;
begin
  Result := GetProp('left').AsFloat;
end;

procedure TStatusBar.SetLeft(Value: Double);
begin
  SetProp('left', TJSONFloatNumber.Create(Value));
end;

function TStatusBar.GetTop: Double;
begin
  Result := GetProp('top').AsFloat;
end;

procedure TStatusBar.SetTop(Value: Double);
begin
  SetProp('top', TJSONFloatNumber.Create(Value));
end;

function TStatusBar.GetWidth: Double;
begin
  Result := GetProp('width').AsFloat;
end;

procedure TStatusBar.SetWidth(Value: Double);
begin
  SetProp('width', TJSONFloatNumber.Create(Value));
end;

function TStatusBar.GetHeight: Double;
begin
  Result := GetProp('height').AsFloat;
end;

procedure TStatusBar.SetHeight(Value: Double);
begin
  SetProp('height', TJSONFloatNumber.Create(Value));
end;

function TStatusBar.GetAlign: TAlign;
begin
  Result := AlignOf(GetProp('align'));
end;

procedure TStatusBar.SetAlign(Value: TAlign);
begin
  SetProp('align', TJSONString.Create(AlignNames[Value]));
end;

function TStatusBar.GetAnchors: TAnchorSet;
begin
  Result := AnchorSetOf(GetProp('anchors'));
end;

procedure TStatusBar.SetAnchors(Value: TAnchorSet);
begin
  SetProp('anchors', AnchorSetToJSON(Value));
end;

function TStatusBar.GetMargins: TAylisBounds;
begin
  Result := AylisBoundsFromJSON(GetProp('margins'));
end;

procedure TStatusBar.SetMargins(Value: TAylisBounds);
begin
  SetProp('margins', AylisBoundsToJSON(Value));
end;

function TStatusBar.GetPadding: TAylisBounds;
begin
  Result := AylisBoundsFromJSON(GetProp('padding'));
end;

procedure TStatusBar.SetPadding(Value: TAylisBounds);
begin
  SetProp('padding', AylisBoundsToJSON(Value));
end;

function TStatusBar.GetVisible: Boolean;
begin
  Result := GetProp('visible').AsBoolean;
end;

procedure TStatusBar.SetVisible(Value: Boolean);
begin
  SetProp('visible', TJSONBoolean.Create(Value));
end;

function TStatusBar.GetEnabled: Boolean;
begin
  Result := GetProp('enabled').AsBoolean;
end;

procedure TStatusBar.SetEnabled(Value: Boolean);
begin
  SetProp('enabled', TJSONBoolean.Create(Value));
end;

function TStatusBar.GetHitTest: Boolean;
begin
  Result := GetProp('hit_test').AsBoolean;
end;

procedure TStatusBar.SetHitTest(Value: Boolean);
begin
  SetProp('hit_test', TJSONBoolean.Create(Value));
end;

function TStatusBar.GetClipChildren: Boolean;
begin
  Result := GetProp('clip_children').AsBoolean;
end;

procedure TStatusBar.SetClipChildren(Value: Boolean);
begin
  SetProp('clip_children', TJSONBoolean.Create(Value));
end;

function TStatusBar.GetInheritState: Boolean;
begin
  Result := GetProp('inherit_state').AsBoolean;
end;

procedure TStatusBar.SetInheritState(Value: Boolean);
begin
  SetProp('inherit_state', TJSONBoolean.Create(Value));
end;

function TStatusBar.GetTranslate: Boolean;
begin
  Result := GetProp('translate').AsBoolean;
end;

procedure TStatusBar.SetTranslate(Value: Boolean);
begin
  SetProp('translate', TJSONBoolean.Create(Value));
end;

function TStatusBar.GetCapturesWheel: Boolean;
begin
  Result := GetProp('captures_wheel').AsBoolean;
end;

procedure TStatusBar.SetCapturesWheel(Value: Boolean);
begin
  SetProp('captures_wheel', TJSONBoolean.Create(Value));
end;

function TStatusBar.GetCapturesGesture: Boolean;
begin
  Result := GetProp('captures_gesture').AsBoolean;
end;

procedure TStatusBar.SetCapturesGesture(Value: Boolean);
begin
  SetProp('captures_gesture', TJSONBoolean.Create(Value));
end;

function TStatusBar.GetCanFocus: Boolean;
begin
  Result := GetProp('can_focus').AsBoolean;
end;

procedure TStatusBar.SetCanFocus(Value: Boolean);
begin
  SetProp('can_focus', TJSONBoolean.Create(Value));
end;

function TStatusBar.GetDraggable: Boolean;
begin
  Result := GetProp('draggable').AsBoolean;
end;

procedure TStatusBar.SetDraggable(Value: Boolean);
begin
  SetProp('draggable', TJSONBoolean.Create(Value));
end;

function TStatusBar.GetAcceptsDrops: Boolean;
begin
  Result := GetProp('accepts_drops').AsBoolean;
end;

procedure TStatusBar.SetAcceptsDrops(Value: Boolean);
begin
  SetProp('accepts_drops', TJSONBoolean.Create(Value));
end;

function TStatusBar.GetDragData: string;
begin
  Result := AsText(GetProp('drag_data'));
end;

procedure TStatusBar.SetDragData(Value: string);
begin
  SetProp('drag_data', TJSONString.Create(Value));
end;

function TStatusBar.GetDragLabel: string;
begin
  Result := AsText(GetProp('drag_label'));
end;

procedure TStatusBar.SetDragLabel(Value: string);
begin
  SetProp('drag_label', TJSONString.Create(Value));
end;

function TStatusBar.GetDragImage: TAylisComponent;
begin
  Result := GetRef('drag_image');
end;

procedure TStatusBar.SetDragImage(Value: TAylisComponent);
begin
  SetRef('drag_image', Value);
end;

function TStatusBar.GetDropTypes: string;
begin
  Result := AsText(GetProp('drop_types'));
end;

procedure TStatusBar.SetDropTypes(Value: string);
begin
  SetProp('drop_types', TJSONString.Create(Value));
end;

function TStatusBar.GetTabStop: Boolean;
begin
  Result := GetProp('tab_stop').AsBoolean;
end;

procedure TStatusBar.SetTabStop(Value: Boolean);
begin
  SetProp('tab_stop', TJSONBoolean.Create(Value));
end;

function TStatusBar.GetCursor: TCursorShape;
begin
  Result := CursorShapeOf(GetProp('cursor'));
end;

procedure TStatusBar.SetCursor(Value: TCursorShape);
begin
  SetProp('cursor', TJSONString.Create(CursorShapeNames[Value]));
end;

function TStatusBar.GetOpacity: Double;
begin
  Result := GetProp('opacity').AsFloat;
end;

procedure TStatusBar.SetOpacity(Value: Double);
begin
  SetProp('opacity', TJSONFloatNumber.Create(Value));
end;

function TStatusBar.GetBlendMode: TBlendMode;
begin
  Result := BlendModeOf(GetProp('blend_mode'));
end;

procedure TStatusBar.SetBlendMode(Value: TBlendMode);
begin
  SetProp('blend_mode', TJSONString.Create(BlendModeNames[Value]));
end;

function TStatusBar.GetCacheLayer: Boolean;
begin
  Result := GetProp('cache_layer').AsBoolean;
end;

procedure TStatusBar.SetCacheLayer(Value: Boolean);
begin
  SetProp('cache_layer', TJSONBoolean.Create(Value));
end;

function TStatusBar.GetRotation: Double;
begin
  Result := GetProp('rotation').AsFloat;
end;

procedure TStatusBar.SetRotation(Value: Double);
begin
  SetProp('rotation', TJSONFloatNumber.Create(Value));
end;

function TStatusBar.GetScale: Double;
begin
  Result := GetProp('scale').AsFloat;
end;

procedure TStatusBar.SetScale(Value: Double);
begin
  SetProp('scale', TJSONFloatNumber.Create(Value));
end;

function TStatusBar.GetTranslateX: Double;
begin
  Result := GetProp('translate_x').AsFloat;
end;

procedure TStatusBar.SetTranslateX(Value: Double);
begin
  SetProp('translate_x', TJSONFloatNumber.Create(Value));
end;

function TStatusBar.GetTranslateY: Double;
begin
  Result := GetProp('translate_y').AsFloat;
end;

procedure TStatusBar.SetTranslateY(Value: Double);
begin
  SetProp('translate_y', TJSONFloatNumber.Create(Value));
end;

function TStatusBar.GetAspectRatio: Double;
begin
  Result := GetProp('aspect_ratio').AsFloat;
end;

procedure TStatusBar.SetAspectRatio(Value: Double);
begin
  SetProp('aspect_ratio', TJSONFloatNumber.Create(Value));
end;

function TStatusBar.GetMaxWidth: Double;
begin
  Result := GetProp('max_width').AsFloat;
end;

procedure TStatusBar.SetMaxWidth(Value: Double);
begin
  SetProp('max_width', TJSONFloatNumber.Create(Value));
end;

function TStatusBar.GetMinWidth: Double;
begin
  Result := GetProp('min_width').AsFloat;
end;

procedure TStatusBar.SetMinWidth(Value: Double);
begin
  SetProp('min_width', TJSONFloatNumber.Create(Value));
end;

function TStatusBar.GetMinHeight: Double;
begin
  Result := GetProp('min_height').AsFloat;
end;

procedure TStatusBar.SetMinHeight(Value: Double);
begin
  SetProp('min_height', TJSONFloatNumber.Create(Value));
end;

function TStatusBar.GetMaxHeight: Double;
begin
  Result := GetProp('max_height').AsFloat;
end;

procedure TStatusBar.SetMaxHeight(Value: Double);
begin
  SetProp('max_height', TJSONFloatNumber.Create(Value));
end;

function TStatusBar.GetSelfAlign: TSelfAlign;
begin
  Result := SelfAlignOf(GetProp('self_align'));
end;

procedure TStatusBar.SetSelfAlign(Value: TSelfAlign);
begin
  SetProp('self_align', TJSONString.Create(SelfAlignNames[Value]));
end;

function TStatusBar.GetAutoSize: Boolean;
begin
  Result := GetProp('auto_size').AsBoolean;
end;

procedure TStatusBar.SetAutoSize(Value: Boolean);
begin
  SetProp('auto_size', TJSONBoolean.Create(Value));
end;

function TStatusBar.GetColumnSpan: Int64;
begin
  Result := GetProp('column_span').AsInt64;
end;

procedure TStatusBar.SetColumnSpan(Value: Int64);
begin
  SetProp('column_span', TJSONInt64Number.Create(Value));
end;

function TStatusBar.GetGridRow: Int64;
begin
  Result := GetProp('grid_row').AsInt64;
end;

procedure TStatusBar.SetGridRow(Value: Int64);
begin
  SetProp('grid_row', TJSONInt64Number.Create(Value));
end;

function TStatusBar.GetGridColumn: Int64;
begin
  Result := GetProp('grid_column').AsInt64;
end;

procedure TStatusBar.SetGridColumn(Value: Int64);
begin
  SetProp('grid_column', TJSONInt64Number.Create(Value));
end;

function TStatusBar.GetRowSpan: Int64;
begin
  Result := GetProp('row_span').AsInt64;
end;

procedure TStatusBar.SetRowSpan(Value: Int64);
begin
  SetProp('row_span', TJSONInt64Number.Create(Value));
end;

function TStatusBar.GetGrow: Double;
begin
  Result := GetProp('grow').AsFloat;
end;

procedure TStatusBar.SetGrow(Value: Double);
begin
  SetProp('grow', TJSONFloatNumber.Create(Value));
end;

function TStatusBar.GetShadow: TAylisShadow;
begin
  Result := AylisShadowFromJSON(GetProp('shadow'));
end;

procedure TStatusBar.SetShadow(Value: TAylisShadow);
begin
  SetProp('shadow', AylisShadowToJSON(Value));
end;

function TStatusBar.GetStyle: TStatusBarStyle;
begin
  Result := StatusBarStyleOf(GetProp('style'));
end;

procedure TStatusBar.SetStyle(Value: TStatusBarStyle);
begin
  SetProp('style', TJSONString.Create(StatusBarStyleNames[Value]));
end;

function TStatusBar.GetTheme: TThemeName;
begin
  Result := ThemeNameOf(GetProp('theme'));
end;

procedure TStatusBar.SetTheme(Value: TThemeName);
begin
  SetProp('theme', TJSONString.Create(ThemeNameNames[Value]));
end;

function TStatusBar.GetFill: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('fill'));
end;

procedure TStatusBar.SetFill(Value: TAylisColor);
begin
  SetProp('fill', AylisColorToJSON(Value));
end;

function TStatusBar.GetBorderColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('border_color'));
end;

procedure TStatusBar.SetBorderColor(Value: TAylisColor);
begin
  SetProp('border_color', AylisColorToJSON(Value));
end;

function TStatusBar.GetTextColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('text_color'));
end;

procedure TStatusBar.SetTextColor(Value: TAylisColor);
begin
  SetProp('text_color', AylisColorToJSON(Value));
end;

function TStatusBar.GetAccentColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('accent_color'));
end;

procedure TStatusBar.SetAccentColor(Value: TAylisColor);
begin
  SetProp('accent_color', AylisColorToJSON(Value));
end;

function TStatusBar.GetSuccessColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('success_color'));
end;

procedure TStatusBar.SetSuccessColor(Value: TAylisColor);
begin
  SetProp('success_color', AylisColorToJSON(Value));
end;

function TStatusBar.GetWarningColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('warning_color'));
end;

procedure TStatusBar.SetWarningColor(Value: TAylisColor);
begin
  SetProp('warning_color', AylisColorToJSON(Value));
end;

function TStatusBar.GetDangerColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('danger_color'));
end;

procedure TStatusBar.SetDangerColor(Value: TAylisColor);
begin
  SetProp('danger_color', AylisColorToJSON(Value));
end;

function TStatusBar.GetFontFamily: string;
begin
  Result := AsText(GetProp('font_family'));
end;

procedure TStatusBar.SetFontFamily(Value: string);
begin
  SetProp('font_family', TJSONString.Create(Value));
end;

function TStatusBar.GetFontSize: Double;
begin
  Result := GetProp('font_size').AsFloat;
end;

procedure TStatusBar.SetFontSize(Value: Double);
begin
  SetProp('font_size', TJSONFloatNumber.Create(Value));
end;

function TStatusBar.GetFontWeight: TFontWeight;
begin
  Result := FontWeightOf(GetProp('font_weight'));
end;

procedure TStatusBar.SetFontWeight(Value: TFontWeight);
begin
  SetProp('font_weight', TJSONString.Create(FontWeightNames[Value]));
end;

function TStatusBar.GetFontItalic: Boolean;
begin
  Result := GetProp('font_italic').AsBoolean;
end;

procedure TStatusBar.SetFontItalic(Value: Boolean);
begin
  SetProp('font_italic', TJSONBoolean.Create(Value));
end;

function TStatusBar.GetFontUnderline: Boolean;
begin
  Result := GetProp('font_underline').AsBoolean;
end;

procedure TStatusBar.SetFontUnderline(Value: Boolean);
begin
  SetProp('font_underline', TJSONBoolean.Create(Value));
end;

function TStatusBar.GetFontStrikeout: Boolean;
begin
  Result := GetProp('font_strikeout').AsBoolean;
end;

procedure TStatusBar.SetFontStrikeout(Value: Boolean);
begin
  SetProp('font_strikeout', TJSONBoolean.Create(Value));
end;

function TStatusBar.GetHint: string;
begin
  Result := AsText(GetProp('hint'));
end;

procedure TStatusBar.SetHint(Value: string);
begin
  SetProp('hint', TJSONString.Create(Value));
end;

function TStatusBar.GetTag: Int64;
begin
  Result := GetProp('tag').AsInt64;
end;

procedure TStatusBar.SetTag(Value: Int64);
begin
  SetProp('tag', TJSONInt64Number.Create(Value));
end;

function TStatusBar.GetTagString: string;
begin
  Result := AsText(GetProp('tag_string'));
end;

procedure TStatusBar.SetTagString(Value: string);
begin
  SetProp('tag_string', TJSONString.Create(Value));
end;

function TStatusBar.GetAccessibleName: string;
begin
  Result := AsText(GetProp('accessible_name'));
end;

procedure TStatusBar.SetAccessibleName(Value: string);
begin
  SetProp('accessible_name', TJSONString.Create(Value));
end;

function TStatusBar.GetPopupMenu: TAylisComponent;
begin
  Result := GetRef('popup_menu');
end;

procedure TStatusBar.SetPopupMenu(Value: TAylisComponent);
begin
  SetRef('popup_menu', Value);
end;

function TStatusBar.GetOnClick: string;
begin
  Result := AsText(GetProp('on_click'));
end;

procedure TStatusBar.SetOnClick(Value: string);
begin
  SetProp('on_click', TJSONString.Create(Value));
end;

function TStatusBar.GetOnDblClick: string;
begin
  Result := AsText(GetProp('on_dbl_click'));
end;

procedure TStatusBar.SetOnDblClick(Value: string);
begin
  SetProp('on_dbl_click', TJSONString.Create(Value));
end;

function TStatusBar.GetOnLongPress: string;
begin
  Result := AsText(GetProp('on_long_press'));
end;

procedure TStatusBar.SetOnLongPress(Value: string);
begin
  SetProp('on_long_press', TJSONString.Create(Value));
end;

function TStatusBar.GetOnPointerDown: string;
begin
  Result := AsText(GetProp('on_pointer_down'));
end;

procedure TStatusBar.SetOnPointerDown(Value: string);
begin
  SetProp('on_pointer_down', TJSONString.Create(Value));
end;

function TStatusBar.GetOnPointerUp: string;
begin
  Result := AsText(GetProp('on_pointer_up'));
end;

procedure TStatusBar.SetOnPointerUp(Value: string);
begin
  SetProp('on_pointer_up', TJSONString.Create(Value));
end;

function TStatusBar.GetOnPointerMove: string;
begin
  Result := AsText(GetProp('on_pointer_move'));
end;

procedure TStatusBar.SetOnPointerMove(Value: string);
begin
  SetProp('on_pointer_move', TJSONString.Create(Value));
end;

function TStatusBar.GetOnPointerEnter: string;
begin
  Result := AsText(GetProp('on_pointer_enter'));
end;

procedure TStatusBar.SetOnPointerEnter(Value: string);
begin
  SetProp('on_pointer_enter', TJSONString.Create(Value));
end;

function TStatusBar.GetOnPointerLeave: string;
begin
  Result := AsText(GetProp('on_pointer_leave'));
end;

procedure TStatusBar.SetOnPointerLeave(Value: string);
begin
  SetProp('on_pointer_leave', TJSONString.Create(Value));
end;

function TStatusBar.GetOnWheel: string;
begin
  Result := AsText(GetProp('on_wheel'));
end;

procedure TStatusBar.SetOnWheel(Value: string);
begin
  SetProp('on_wheel', TJSONString.Create(Value));
end;

function TStatusBar.GetOnGesture: string;
begin
  Result := AsText(GetProp('on_gesture'));
end;

procedure TStatusBar.SetOnGesture(Value: string);
begin
  SetProp('on_gesture', TJSONString.Create(Value));
end;

function TStatusBar.GetOnSwipe: string;
begin
  Result := AsText(GetProp('on_swipe'));
end;

procedure TStatusBar.SetOnSwipe(Value: string);
begin
  SetProp('on_swipe', TJSONString.Create(Value));
end;

function TStatusBar.GetOnKeyUp: string;
begin
  Result := AsText(GetProp('on_key_up'));
end;

procedure TStatusBar.SetOnKeyUp(Value: string);
begin
  SetProp('on_key_up', TJSONString.Create(Value));
end;

function TStatusBar.GetOnEnter: string;
begin
  Result := AsText(GetProp('on_enter'));
end;

procedure TStatusBar.SetOnEnter(Value: string);
begin
  SetProp('on_enter', TJSONString.Create(Value));
end;

function TStatusBar.GetOnExit: string;
begin
  Result := AsText(GetProp('on_exit'));
end;

procedure TStatusBar.SetOnExit(Value: string);
begin
  SetProp('on_exit', TJSONString.Create(Value));
end;

function TStatusBar.GetOnResize: string;
begin
  Result := AsText(GetProp('on_resize'));
end;

procedure TStatusBar.SetOnResize(Value: string);
begin
  SetProp('on_resize', TJSONString.Create(Value));
end;

function TStatusBar.GetOnDragMove: string;
begin
  Result := AsText(GetProp('on_drag_move'));
end;

procedure TStatusBar.SetOnDragMove(Value: string);
begin
  SetProp('on_drag_move', TJSONString.Create(Value));
end;

function TStatusBar.GetOnDragLeave: string;
begin
  Result := AsText(GetProp('on_drag_leave'));
end;

procedure TStatusBar.SetOnDragLeave(Value: string);
begin
  SetProp('on_drag_leave', TJSONString.Create(Value));
end;

function TStatusBar.GetOnDragDrop: string;
begin
  Result := AsText(GetProp('on_drag_drop'));
end;

procedure TStatusBar.SetOnDragDrop(Value: string);
begin
  SetProp('on_drag_drop', TJSONString.Create(Value));
end;

function TStatusBar.GetOnDragEnd: string;
begin
  Result := AsText(GetProp('on_drag_end'));
end;

procedure TStatusBar.SetOnDragEnd(Value: string);
begin
  SetProp('on_drag_end', TJSONString.Create(Value));
end;

procedure TFrmPaintBase.BindComponents(List: TJSONArray);
begin
  lytTopo := TLayout(Resolve(List, 'lytTopo', 'Layout', TLayout));
  mnuPrincipal := TMainMenu(Resolve(List, 'mnuPrincipal', 'MainMenu', TMainMenu));
  mnuArquivo := TMenuItem(Resolve(List, 'mnuArquivo', 'MenuItem', TMenuItem));
  mnuNovo := TMenuItem(Resolve(List, 'mnuNovo', 'MenuItem', TMenuItem));
  mnuAbrir := TMenuItem(Resolve(List, 'mnuAbrir', 'MenuItem', TMenuItem));
  mnuSalvar := TMenuItem(Resolve(List, 'mnuSalvar', 'MenuItem', TMenuItem));
  mnuSalvarComo := TMenuItem(Resolve(List, 'mnuSalvarComo', 'MenuItem', TMenuItem));
  mnuSep1 := TMenuItem(Resolve(List, 'mnuSep1', 'MenuItem', TMenuItem));
  mnuSair := TMenuItem(Resolve(List, 'mnuSair', 'MenuItem', TMenuItem));
  mnuEditar := TMenuItem(Resolve(List, 'mnuEditar', 'MenuItem', TMenuItem));
  mnuDesfazer := TMenuItem(Resolve(List, 'mnuDesfazer', 'MenuItem', TMenuItem));
  mnuRefazer := TMenuItem(Resolve(List, 'mnuRefazer', 'MenuItem', TMenuItem));
  mnuSep2 := TMenuItem(Resolve(List, 'mnuSep2', 'MenuItem', TMenuItem));
  mnuRecortar := TMenuItem(Resolve(List, 'mnuRecortar', 'MenuItem', TMenuItem));
  mnuCopiar := TMenuItem(Resolve(List, 'mnuCopiar', 'MenuItem', TMenuItem));
  mnuColar := TMenuItem(Resolve(List, 'mnuColar', 'MenuItem', TMenuItem));
  mnuExcluir := TMenuItem(Resolve(List, 'mnuExcluir', 'MenuItem', TMenuItem));
  mnuSelecionarTudo := TMenuItem(Resolve(List, 'mnuSelecionarTudo', 'MenuItem', TMenuItem));
  mnuSep3 := TMenuItem(Resolve(List, 'mnuSep3', 'MenuItem', TMenuItem));
  mnuLimpar := TMenuItem(Resolve(List, 'mnuLimpar', 'MenuItem', TMenuItem));
  mnuExibir := TMenuItem(Resolve(List, 'mnuExibir', 'MenuItem', TMenuItem));
  mnuAmpliar := TMenuItem(Resolve(List, 'mnuAmpliar', 'MenuItem', TMenuItem));
  mnuReduzir := TMenuItem(Resolve(List, 'mnuReduzir', 'MenuItem', TMenuItem));
  mnuTamanhoReal := TMenuItem(Resolve(List, 'mnuTamanhoReal', 'MenuItem', TMenuItem));
  divTopo := TDivider(Resolve(List, 'divTopo', 'Divider', TDivider));
  btnSalvar := TButton(Resolve(List, 'btnSalvar', 'Button', TButton));
  btnDesfazer := TButton(Resolve(List, 'btnDesfazer', 'Button', TButton));
  btnRefazer := TButton(Resolve(List, 'btnRefazer', 'Button', TButton));
  pnlFaixa := TPanel(Resolve(List, 'pnlFaixa', 'Panel', TPanel));
  lytSelecao := TLayout(Resolve(List, 'lytSelecao', 'Layout', TLayout));
  lblSelecaoGrupo := TLabel(Resolve(List, 'lblSelecaoGrupo', 'Label', TLabel));
  btnSelecionar := TButton(Resolve(List, 'btnSelecionar', 'Button', TButton));
  divSelecao := TDivider(Resolve(List, 'divSelecao', 'Divider', TDivider));
  lytFerramentas := TLayout(Resolve(List, 'lytFerramentas', 'Layout', TLayout));
  lblFerramentasGrupo := TLabel(Resolve(List, 'lblFerramentasGrupo', 'Label', TLabel));
  grdFerramentas := TGridLayout(Resolve(List, 'grdFerramentas', 'GridLayout', TGridLayout));
  btnLapis := TButton(Resolve(List, 'btnLapis', 'Button', TButton));
  btnBalde := TButton(Resolve(List, 'btnBalde', 'Button', TButton));
  btnBorracha := TButton(Resolve(List, 'btnBorracha', 'Button', TButton));
  btnContaGotas := TButton(Resolve(List, 'btnContaGotas', 'Button', TButton));
  divFerramentas := TDivider(Resolve(List, 'divFerramentas', 'Divider', TDivider));
  lytPinceis := TLayout(Resolve(List, 'lytPinceis', 'Layout', TLayout));
  lblPinceis := TLabel(Resolve(List, 'lblPinceis', 'Label', TLabel));
  btnPincel := TButton(Resolve(List, 'btnPincel', 'Button', TButton));
  divPinceis := TDivider(Resolve(List, 'divPinceis', 'Divider', TDivider));
  lytFormas := TLayout(Resolve(List, 'lytFormas', 'Layout', TLayout));
  lblFormas := TLabel(Resolve(List, 'lblFormas', 'Label', TLabel));
  chkPreencher := TCheckBox(Resolve(List, 'chkPreencher', 'CheckBox', TCheckBox));
  grdFormas := TGridLayout(Resolve(List, 'grdFormas', 'GridLayout', TGridLayout));
  btnLinha := TButton(Resolve(List, 'btnLinha', 'Button', TButton));
  btnRetangulo := TButton(Resolve(List, 'btnRetangulo', 'Button', TButton));
  btnElipse := TButton(Resolve(List, 'btnElipse', 'Button', TButton));
  divFormas := TDivider(Resolve(List, 'divFormas', 'Divider', TDivider));
  lytTamanho := TLayout(Resolve(List, 'lytTamanho', 'Layout', TLayout));
  lblTamanhoGrupo := TLabel(Resolve(List, 'lblTamanhoGrupo', 'Label', TLabel));
  cbEspessura := TComboBox(Resolve(List, 'cbEspessura', 'ComboBox', TComboBox));
  divTamanho := TDivider(Resolve(List, 'divTamanho', 'Divider', TDivider));
  lytCores := TLayout(Resolve(List, 'lytCores', 'Layout', TLayout));
  lblCores := TLabel(Resolve(List, 'lblCores', 'Label', TLabel));
  btnCor1 := TCircle(Resolve(List, 'btnCor1', 'Circle', TCircle));
  btnCor2 := TCircle(Resolve(List, 'btnCor2', 'Circle', TCircle));
  grdPaleta := TGridLayout(Resolve(List, 'grdPaleta', 'GridLayout', TGridLayout));
  btnPaleta01 := TCircle(Resolve(List, 'btnPaleta01', 'Circle', TCircle));
  btnPaleta02 := TCircle(Resolve(List, 'btnPaleta02', 'Circle', TCircle));
  btnPaleta03 := TCircle(Resolve(List, 'btnPaleta03', 'Circle', TCircle));
  btnPaleta04 := TCircle(Resolve(List, 'btnPaleta04', 'Circle', TCircle));
  btnPaleta05 := TCircle(Resolve(List, 'btnPaleta05', 'Circle', TCircle));
  btnPaleta06 := TCircle(Resolve(List, 'btnPaleta06', 'Circle', TCircle));
  btnPaleta07 := TCircle(Resolve(List, 'btnPaleta07', 'Circle', TCircle));
  btnPaleta08 := TCircle(Resolve(List, 'btnPaleta08', 'Circle', TCircle));
  btnPaleta09 := TCircle(Resolve(List, 'btnPaleta09', 'Circle', TCircle));
  btnPaleta10 := TCircle(Resolve(List, 'btnPaleta10', 'Circle', TCircle));
  btnPaleta11 := TCircle(Resolve(List, 'btnPaleta11', 'Circle', TCircle));
  btnPaleta12 := TCircle(Resolve(List, 'btnPaleta12', 'Circle', TCircle));
  btnPaleta13 := TCircle(Resolve(List, 'btnPaleta13', 'Circle', TCircle));
  btnPaleta14 := TCircle(Resolve(List, 'btnPaleta14', 'Circle', TCircle));
  btnPaleta15 := TCircle(Resolve(List, 'btnPaleta15', 'Circle', TCircle));
  btnPaleta16 := TCircle(Resolve(List, 'btnPaleta16', 'Circle', TCircle));
  btnPaleta17 := TCircle(Resolve(List, 'btnPaleta17', 'Circle', TCircle));
  btnPaleta18 := TCircle(Resolve(List, 'btnPaleta18', 'Circle', TCircle));
  btnPaleta19 := TCircle(Resolve(List, 'btnPaleta19', 'Circle', TCircle));
  btnPaleta20 := TCircle(Resolve(List, 'btnPaleta20', 'Circle', TCircle));
  stbStatus := TStatusBar(Resolve(List, 'stbStatus', 'StatusBar', TStatusBar));
  lblPosicao := TLabel(Resolve(List, 'lblPosicao', 'Label', TLabel));
  lblTamanho := TLabel(Resolve(List, 'lblTamanho', 'Label', TLabel));
  lblZoom := TLabel(Resolve(List, 'lblZoom', 'Label', TLabel));
  sbxArea := TScrollBox(Resolve(List, 'sbxArea', 'ScrollBox', TScrollBox));
  pcDesenho := TPaintCanvas(Resolve(List, 'pcDesenho', 'PaintCanvas', TPaintCanvas));
  dlgAbrir := TOpenDialog(Resolve(List, 'dlgAbrir', 'OpenDialog', TOpenDialog));
  dlgSalvar := TSaveDialog(Resolve(List, 'dlgSalvar', 'SaveDialog', TSaveDialog));
end;

function TFrmPaintBase.FindButton(const Nome: string): TButton;
begin
  Result := nil;
  if Nome = 'btnSalvar' then
    Result := btnSalvar;
  if Nome = 'btnDesfazer' then
    Result := btnDesfazer;
  if Nome = 'btnRefazer' then
    Result := btnRefazer;
  if Nome = 'btnSelecionar' then
    Result := btnSelecionar;
  if Nome = 'btnLapis' then
    Result := btnLapis;
  if Nome = 'btnBalde' then
    Result := btnBalde;
  if Nome = 'btnBorracha' then
    Result := btnBorracha;
  if Nome = 'btnContaGotas' then
    Result := btnContaGotas;
  if Nome = 'btnPincel' then
    Result := btnPincel;
  if Nome = 'btnLinha' then
    Result := btnLinha;
  if Nome = 'btnRetangulo' then
    Result := btnRetangulo;
  if Nome = 'btnElipse' then
    Result := btnElipse;
end;

function TFrmPaintBase.SenderAsButton(Sender: TAylisComponent): TButton;
begin
  Result := nil;
  if Sender <> nil then
    Result := FindButton(Sender.Name);
end;

function TFrmPaintBase.SenderAsButton(Event: TJSONObject): TButton;
begin
  Result := FindButton(TJSONObject(Event.Find('sender')).Get('name', ''));
end;

function TFrmPaintBase.FindCheckBox(const Nome: string): TCheckBox;
begin
  Result := nil;
  if Nome = 'chkPreencher' then
    Result := chkPreencher;
end;

function TFrmPaintBase.SenderAsCheckBox(Sender: TAylisComponent): TCheckBox;
begin
  Result := nil;
  if Sender <> nil then
    Result := FindCheckBox(Sender.Name);
end;

function TFrmPaintBase.SenderAsCheckBox(Event: TJSONObject): TCheckBox;
begin
  Result := FindCheckBox(TJSONObject(Event.Find('sender')).Get('name', ''));
end;

function TFrmPaintBase.FindCircle(const Nome: string): TCircle;
begin
  Result := nil;
  if Nome = 'btnCor1' then
    Result := btnCor1;
  if Nome = 'btnCor2' then
    Result := btnCor2;
  if Nome = 'btnPaleta01' then
    Result := btnPaleta01;
  if Nome = 'btnPaleta02' then
    Result := btnPaleta02;
  if Nome = 'btnPaleta03' then
    Result := btnPaleta03;
  if Nome = 'btnPaleta04' then
    Result := btnPaleta04;
  if Nome = 'btnPaleta05' then
    Result := btnPaleta05;
  if Nome = 'btnPaleta06' then
    Result := btnPaleta06;
  if Nome = 'btnPaleta07' then
    Result := btnPaleta07;
  if Nome = 'btnPaleta08' then
    Result := btnPaleta08;
  if Nome = 'btnPaleta09' then
    Result := btnPaleta09;
  if Nome = 'btnPaleta10' then
    Result := btnPaleta10;
  if Nome = 'btnPaleta11' then
    Result := btnPaleta11;
  if Nome = 'btnPaleta12' then
    Result := btnPaleta12;
  if Nome = 'btnPaleta13' then
    Result := btnPaleta13;
  if Nome = 'btnPaleta14' then
    Result := btnPaleta14;
  if Nome = 'btnPaleta15' then
    Result := btnPaleta15;
  if Nome = 'btnPaleta16' then
    Result := btnPaleta16;
  if Nome = 'btnPaleta17' then
    Result := btnPaleta17;
  if Nome = 'btnPaleta18' then
    Result := btnPaleta18;
  if Nome = 'btnPaleta19' then
    Result := btnPaleta19;
  if Nome = 'btnPaleta20' then
    Result := btnPaleta20;
end;

function TFrmPaintBase.SenderAsCircle(Sender: TAylisComponent): TCircle;
begin
  Result := nil;
  if Sender <> nil then
    Result := FindCircle(Sender.Name);
end;

function TFrmPaintBase.SenderAsCircle(Event: TJSONObject): TCircle;
begin
  Result := FindCircle(TJSONObject(Event.Find('sender')).Get('name', ''));
end;

function TFrmPaintBase.FindComboBox(const Nome: string): TComboBox;
begin
  Result := nil;
  if Nome = 'cbEspessura' then
    Result := cbEspessura;
end;

function TFrmPaintBase.SenderAsComboBox(Sender: TAylisComponent): TComboBox;
begin
  Result := nil;
  if Sender <> nil then
    Result := FindComboBox(Sender.Name);
end;

function TFrmPaintBase.SenderAsComboBox(Event: TJSONObject): TComboBox;
begin
  Result := FindComboBox(TJSONObject(Event.Find('sender')).Get('name', ''));
end;

function TFrmPaintBase.FindDivider(const Nome: string): TDivider;
begin
  Result := nil;
  if Nome = 'divTopo' then
    Result := divTopo;
  if Nome = 'divSelecao' then
    Result := divSelecao;
  if Nome = 'divFerramentas' then
    Result := divFerramentas;
  if Nome = 'divPinceis' then
    Result := divPinceis;
  if Nome = 'divFormas' then
    Result := divFormas;
  if Nome = 'divTamanho' then
    Result := divTamanho;
end;

function TFrmPaintBase.SenderAsDivider(Sender: TAylisComponent): TDivider;
begin
  Result := nil;
  if Sender <> nil then
    Result := FindDivider(Sender.Name);
end;

function TFrmPaintBase.SenderAsDivider(Event: TJSONObject): TDivider;
begin
  Result := FindDivider(TJSONObject(Event.Find('sender')).Get('name', ''));
end;

function TFrmPaintBase.FindGridLayout(const Nome: string): TGridLayout;
begin
  Result := nil;
  if Nome = 'grdFerramentas' then
    Result := grdFerramentas;
  if Nome = 'grdFormas' then
    Result := grdFormas;
  if Nome = 'grdPaleta' then
    Result := grdPaleta;
end;

function TFrmPaintBase.SenderAsGridLayout(Sender: TAylisComponent): TGridLayout;
begin
  Result := nil;
  if Sender <> nil then
    Result := FindGridLayout(Sender.Name);
end;

function TFrmPaintBase.SenderAsGridLayout(Event: TJSONObject): TGridLayout;
begin
  Result := FindGridLayout(TJSONObject(Event.Find('sender')).Get('name', ''));
end;

function TFrmPaintBase.FindLabel(const Nome: string): TLabel;
begin
  Result := nil;
  if Nome = 'lblSelecaoGrupo' then
    Result := lblSelecaoGrupo;
  if Nome = 'lblFerramentasGrupo' then
    Result := lblFerramentasGrupo;
  if Nome = 'lblPinceis' then
    Result := lblPinceis;
  if Nome = 'lblFormas' then
    Result := lblFormas;
  if Nome = 'lblTamanhoGrupo' then
    Result := lblTamanhoGrupo;
  if Nome = 'lblCores' then
    Result := lblCores;
  if Nome = 'lblPosicao' then
    Result := lblPosicao;
  if Nome = 'lblTamanho' then
    Result := lblTamanho;
  if Nome = 'lblZoom' then
    Result := lblZoom;
end;

function TFrmPaintBase.SenderAsLabel(Sender: TAylisComponent): TLabel;
begin
  Result := nil;
  if Sender <> nil then
    Result := FindLabel(Sender.Name);
end;

function TFrmPaintBase.SenderAsLabel(Event: TJSONObject): TLabel;
begin
  Result := FindLabel(TJSONObject(Event.Find('sender')).Get('name', ''));
end;

function TFrmPaintBase.FindLayout(const Nome: string): TLayout;
begin
  Result := nil;
  if Nome = 'lytTopo' then
    Result := lytTopo;
  if Nome = 'lytSelecao' then
    Result := lytSelecao;
  if Nome = 'lytFerramentas' then
    Result := lytFerramentas;
  if Nome = 'lytPinceis' then
    Result := lytPinceis;
  if Nome = 'lytFormas' then
    Result := lytFormas;
  if Nome = 'lytTamanho' then
    Result := lytTamanho;
  if Nome = 'lytCores' then
    Result := lytCores;
end;

function TFrmPaintBase.SenderAsLayout(Sender: TAylisComponent): TLayout;
begin
  Result := nil;
  if Sender <> nil then
    Result := FindLayout(Sender.Name);
end;

function TFrmPaintBase.SenderAsLayout(Event: TJSONObject): TLayout;
begin
  Result := FindLayout(TJSONObject(Event.Find('sender')).Get('name', ''));
end;

function TFrmPaintBase.FindMainMenu(const Nome: string): TMainMenu;
begin
  Result := nil;
  if Nome = 'mnuPrincipal' then
    Result := mnuPrincipal;
end;

function TFrmPaintBase.SenderAsMainMenu(Sender: TAylisComponent): TMainMenu;
begin
  Result := nil;
  if Sender <> nil then
    Result := FindMainMenu(Sender.Name);
end;

function TFrmPaintBase.SenderAsMainMenu(Event: TJSONObject): TMainMenu;
begin
  Result := FindMainMenu(TJSONObject(Event.Find('sender')).Get('name', ''));
end;

function TFrmPaintBase.FindMenuItem(const Nome: string): TMenuItem;
begin
  Result := nil;
  if Nome = 'mnuArquivo' then
    Result := mnuArquivo;
  if Nome = 'mnuNovo' then
    Result := mnuNovo;
  if Nome = 'mnuAbrir' then
    Result := mnuAbrir;
  if Nome = 'mnuSalvar' then
    Result := mnuSalvar;
  if Nome = 'mnuSalvarComo' then
    Result := mnuSalvarComo;
  if Nome = 'mnuSep1' then
    Result := mnuSep1;
  if Nome = 'mnuSair' then
    Result := mnuSair;
  if Nome = 'mnuEditar' then
    Result := mnuEditar;
  if Nome = 'mnuDesfazer' then
    Result := mnuDesfazer;
  if Nome = 'mnuRefazer' then
    Result := mnuRefazer;
  if Nome = 'mnuSep2' then
    Result := mnuSep2;
  if Nome = 'mnuRecortar' then
    Result := mnuRecortar;
  if Nome = 'mnuCopiar' then
    Result := mnuCopiar;
  if Nome = 'mnuColar' then
    Result := mnuColar;
  if Nome = 'mnuExcluir' then
    Result := mnuExcluir;
  if Nome = 'mnuSelecionarTudo' then
    Result := mnuSelecionarTudo;
  if Nome = 'mnuSep3' then
    Result := mnuSep3;
  if Nome = 'mnuLimpar' then
    Result := mnuLimpar;
  if Nome = 'mnuExibir' then
    Result := mnuExibir;
  if Nome = 'mnuAmpliar' then
    Result := mnuAmpliar;
  if Nome = 'mnuReduzir' then
    Result := mnuReduzir;
  if Nome = 'mnuTamanhoReal' then
    Result := mnuTamanhoReal;
end;

function TFrmPaintBase.SenderAsMenuItem(Sender: TAylisComponent): TMenuItem;
begin
  Result := nil;
  if Sender <> nil then
    Result := FindMenuItem(Sender.Name);
end;

function TFrmPaintBase.SenderAsMenuItem(Event: TJSONObject): TMenuItem;
begin
  Result := FindMenuItem(TJSONObject(Event.Find('sender')).Get('name', ''));
end;

function TFrmPaintBase.FindOpenDialog(const Nome: string): TOpenDialog;
begin
  Result := nil;
  if Nome = 'dlgAbrir' then
    Result := dlgAbrir;
end;

function TFrmPaintBase.SenderAsOpenDialog(Sender: TAylisComponent): TOpenDialog;
begin
  Result := nil;
  if Sender <> nil then
    Result := FindOpenDialog(Sender.Name);
end;

function TFrmPaintBase.SenderAsOpenDialog(Event: TJSONObject): TOpenDialog;
begin
  Result := FindOpenDialog(TJSONObject(Event.Find('sender')).Get('name', ''));
end;

function TFrmPaintBase.FindPaintCanvas(const Nome: string): TPaintCanvas;
begin
  Result := nil;
  if Nome = 'pcDesenho' then
    Result := pcDesenho;
end;

function TFrmPaintBase.SenderAsPaintCanvas(Sender: TAylisComponent): TPaintCanvas;
begin
  Result := nil;
  if Sender <> nil then
    Result := FindPaintCanvas(Sender.Name);
end;

function TFrmPaintBase.SenderAsPaintCanvas(Event: TJSONObject): TPaintCanvas;
begin
  Result := FindPaintCanvas(TJSONObject(Event.Find('sender')).Get('name', ''));
end;

function TFrmPaintBase.FindPanel(const Nome: string): TPanel;
begin
  Result := nil;
  if Nome = 'pnlFaixa' then
    Result := pnlFaixa;
end;

function TFrmPaintBase.SenderAsPanel(Sender: TAylisComponent): TPanel;
begin
  Result := nil;
  if Sender <> nil then
    Result := FindPanel(Sender.Name);
end;

function TFrmPaintBase.SenderAsPanel(Event: TJSONObject): TPanel;
begin
  Result := FindPanel(TJSONObject(Event.Find('sender')).Get('name', ''));
end;

function TFrmPaintBase.FindSaveDialog(const Nome: string): TSaveDialog;
begin
  Result := nil;
  if Nome = 'dlgSalvar' then
    Result := dlgSalvar;
end;

function TFrmPaintBase.SenderAsSaveDialog(Sender: TAylisComponent): TSaveDialog;
begin
  Result := nil;
  if Sender <> nil then
    Result := FindSaveDialog(Sender.Name);
end;

function TFrmPaintBase.SenderAsSaveDialog(Event: TJSONObject): TSaveDialog;
begin
  Result := FindSaveDialog(TJSONObject(Event.Find('sender')).Get('name', ''));
end;

function TFrmPaintBase.FindScrollBox(const Nome: string): TScrollBox;
begin
  Result := nil;
  if Nome = 'sbxArea' then
    Result := sbxArea;
end;

function TFrmPaintBase.SenderAsScrollBox(Sender: TAylisComponent): TScrollBox;
begin
  Result := nil;
  if Sender <> nil then
    Result := FindScrollBox(Sender.Name);
end;

function TFrmPaintBase.SenderAsScrollBox(Event: TJSONObject): TScrollBox;
begin
  Result := FindScrollBox(TJSONObject(Event.Find('sender')).Get('name', ''));
end;

function TFrmPaintBase.FindStatusBar(const Nome: string): TStatusBar;
begin
  Result := nil;
  if Nome = 'stbStatus' then
    Result := stbStatus;
end;

function TFrmPaintBase.SenderAsStatusBar(Sender: TAylisComponent): TStatusBar;
begin
  Result := nil;
  if Sender <> nil then
    Result := FindStatusBar(Sender.Name);
end;

function TFrmPaintBase.SenderAsStatusBar(Event: TJSONObject): TStatusBar;
begin
  Result := FindStatusBar(TJSONObject(Event.Find('sender')).Get('name', ''));
end;

initialization
  AylisFormat := DefaultFormatSettings;
  AylisFormat.DecimalSeparator := '.';
end.
