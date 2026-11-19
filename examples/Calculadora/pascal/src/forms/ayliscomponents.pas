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
  TListViewStyle = (Default, Flat);
  {$pop}
  TListViewStyleSet = set of TListViewStyle;

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
  TSelfAlign = (Stretch, Start, Center, &End);
  {$pop}
  TSelfAlignSet = set of TSelfAlign;

  { Os valores de um enum do acesso tipado. }
  {$push}{$scopedenums on}
  TTabControlStyle = (Default);
  {$pop}
  TTabControlStyleSet = set of TTabControlStyle;

  { Os valores de um enum do acesso tipado. }
  {$push}{$scopedenums on}
  TTabItemStyle = (Default);
  {$pop}
  TTabItemStyleSet = set of TTabItemStyle;

  { Os valores de um enum do acesso tipado. }
  {$push}{$scopedenums on}
  TTabPosition = (Top, Bottom, Left, Right);
  {$pop}
  TTabPositionSet = set of TTabPosition;

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
  PKeyEvent = ^TKeyEvent;
  PListViewItem = ^TListViewItem;
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

  { Um TabControl de um form. }
  TTabControl = class(TAylisComponent)
  private
    function GetTabIndex: Int64;
    procedure SetTabIndex(Value: Int64);
    function GetShowTabs: Boolean;
    procedure SetShowTabs(Value: Boolean);
    function GetTransition: TTransition;
    procedure SetTransition(Value: TTransition);
    function GetSwipe: Boolean;
    procedure SetSwipe(Value: Boolean);
    function GetTabPosition: TTabPosition;
    procedure SetTabPosition(Value: TTabPosition);
    function GetTabWidth: Double;
    procedure SetTabWidth(Value: Double);
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
    function GetStyle: TTabControlStyle;
    procedure SetStyle(Value: TTabControlStyle);
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
    property TabIndex: Int64 read GetTabIndex write SetTabIndex;
    property ShowTabs: Boolean read GetShowTabs write SetShowTabs;
    property Transition: TTransition read GetTransition write SetTransition;
    property Swipe: Boolean read GetSwipe write SetSwipe;
    property TabPosition: TTabPosition read GetTabPosition write SetTabPosition;
    property TabWidth: Double read GetTabWidth write SetTabWidth;
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
    property Style: TTabControlStyle read GetStyle write SetStyle;
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

  { Um TabItem de um form. }
  TTabItem = class(TAylisComponent)
  private
    function GetCaption: string;
    procedure SetCaption(Value: string);
    function GetOnShow: string;
    procedure SetOnShow(Value: string);
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
    function GetStyle: TTabItemStyle;
    procedure SetStyle(Value: TTabItemStyle);
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
    property OnShow: string read GetOnShow write SetOnShow;
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
    property Style: TTabItemStyle read GetStyle write SetStyle;
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

  { O form frmCalculadora: a classe base com os componentes. A do form (TFrmCalculadora), com os
    handlers (métodos published) e o estado, fica na unit dele. }
  TFrmCalculadoraBase = class(TForm)
  published
    pnlLateral: TPanel;
    tabLateral: TTabControl;
    tabHistorico: TTabItem;
    btnLimparHistorico: TButton;
    lvHistorico: TListView;
    tabMemoria: TTabItem;
    lvMemoria: TListView;
    lytCorpo: TLayout;
    lytTopo: TLayout;
    btnHistorico: TButton;
    lblTitulo: TLabel;
    lblExpressao: TLabel;
    lblVisor: TLabel;
    grdMemoria: TGridLayout;
    btnMC: TButton;
    btnMR: TButton;
    btnMMais: TButton;
    btnMMenos: TButton;
    btnMS: TButton;
    grdTeclado: TGridLayout;
    btnPorcento: TButton;
    btnCE: TButton;
    btnC: TButton;
    btnApagar: TButton;
    btnInverso: TButton;
    btnQuadrado: TButton;
    btnRaiz: TButton;
    btnDividir: TButton;
    btn7: TButton;
    btn8: TButton;
    btn9: TButton;
    btnMultiplicar: TButton;
    btn4: TButton;
    btn5: TButton;
    btn6: TButton;
    btnSubtrair: TButton;
    btn1: TButton;
    btn2: TButton;
    btn3: TButton;
    btnSomar: TButton;
    btnSinal: TButton;
    btn0: TButton;
    btnVirgula: TButton;
    btnIgual: TButton;
  protected
    procedure BindComponents(List: TJSONArray); override;
  public
    { O Button deste form com o nome (nil se não há). }
    function FindButton(const Nome: string): TButton;
    { O Button que disparou o evento (nil se não é um Button deste form). }
    function SenderAsButton(Sender: TAylisComponent): TButton; overload;
    function SenderAsButton(Event: TJSONObject): TButton; overload;
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
    { O ListView deste form com o nome (nil se não há). }
    function FindListView(const Nome: string): TListView;
    { O ListView que disparou o evento (nil se não é um ListView deste form). }
    function SenderAsListView(Sender: TAylisComponent): TListView; overload;
    function SenderAsListView(Event: TJSONObject): TListView; overload;
    { O Panel deste form com o nome (nil se não há). }
    function FindPanel(const Nome: string): TPanel;
    { O Panel que disparou o evento (nil se não é um Panel deste form). }
    function SenderAsPanel(Sender: TAylisComponent): TPanel; overload;
    function SenderAsPanel(Event: TJSONObject): TPanel; overload;
    { O TabControl deste form com o nome (nil se não há). }
    function FindTabControl(const Nome: string): TTabControl;
    { O TabControl que disparou o evento (nil se não é um TabControl deste form). }
    function SenderAsTabControl(Sender: TAylisComponent): TTabControl; overload;
    function SenderAsTabControl(Event: TJSONObject): TTabControl; overload;
    { O TabItem deste form com o nome (nil se não há). }
    function FindTabItem(const Nome: string): TTabItem;
    { O TabItem que disparou o evento (nil se não é um TabItem deste form). }
    function SenderAsTabItem(Sender: TAylisComponent): TTabItem; overload;
    function SenderAsTabItem(Event: TJSONObject): TTabItem; overload;
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
function ListViewStyleOf(Data: TJSONData): TListViewStyle;
function ListViewStyleSetOf(Data: TJSONData): TListViewStyleSet;
function ListViewStyleSetToJSON(const S: TListViewStyleSet): TJSONData;
function ModalResultOf(Data: TJSONData): TModalResult;
function ModalResultSetOf(Data: TJSONData): TModalResultSet;
function ModalResultSetToJSON(const S: TModalResultSet): TJSONData;
function PanelStyleOf(Data: TJSONData): TPanelStyle;
function PanelStyleSetOf(Data: TJSONData): TPanelStyleSet;
function PanelStyleSetToJSON(const S: TPanelStyleSet): TJSONData;
function SelfAlignOf(Data: TJSONData): TSelfAlign;
function SelfAlignSetOf(Data: TJSONData): TSelfAlignSet;
function SelfAlignSetToJSON(const S: TSelfAlignSet): TJSONData;
function TabControlStyleOf(Data: TJSONData): TTabControlStyle;
function TabControlStyleSetOf(Data: TJSONData): TTabControlStyleSet;
function TabControlStyleSetToJSON(const S: TTabControlStyleSet): TJSONData;
function TabItemStyleOf(Data: TJSONData): TTabItemStyle;
function TabItemStyleSetOf(Data: TJSONData): TTabItemStyleSet;
function TabItemStyleSetToJSON(const S: TTabItemStyleSet): TJSONData;
function TabPositionOf(Data: TJSONData): TTabPosition;
function TabPositionSetOf(Data: TJSONData): TTabPositionSet;
function TabPositionSetToJSON(const S: TTabPositionSet): TJSONData;
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
  TabControlStyleNames: array[TTabControlStyle] of string = ('');

function TabControlStyleOf(Data: TJSONData): TTabControlStyle;
var
  V: TTabControlStyle;
begin
  Result := Low(TTabControlStyle);
  if (Data <> nil) and (Data.JSONType = jtString) then
    for V := Low(TTabControlStyle) to High(TTabControlStyle) do
      if TabControlStyleNames[V] = Data.AsString then
        Exit(V);
end;

function TabControlStyleSetOf(Data: TJSONData): TTabControlStyleSet;
var
  I: Integer;
  V: TTabControlStyle;
begin
  Result := [];
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  for I := 0 to TJSONArray(Data).Count - 1 do
    for V := Low(TTabControlStyle) to High(TTabControlStyle) do
      if TabControlStyleNames[V] = TJSONArray(Data).Strings[I] then
        Include(Result, V);
end;

function TabControlStyleSetToJSON(const S: TTabControlStyleSet): TJSONData;
var
  V: TTabControlStyle;
begin
  Result := TJSONArray.Create;
  for V := Low(TTabControlStyle) to High(TTabControlStyle) do
    if V in S then
      TJSONArray(Result).Add(TabControlStyleNames[V]);
end;

const
  TabItemStyleNames: array[TTabItemStyle] of string = ('');

function TabItemStyleOf(Data: TJSONData): TTabItemStyle;
var
  V: TTabItemStyle;
begin
  Result := Low(TTabItemStyle);
  if (Data <> nil) and (Data.JSONType = jtString) then
    for V := Low(TTabItemStyle) to High(TTabItemStyle) do
      if TabItemStyleNames[V] = Data.AsString then
        Exit(V);
end;

function TabItemStyleSetOf(Data: TJSONData): TTabItemStyleSet;
var
  I: Integer;
  V: TTabItemStyle;
begin
  Result := [];
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  for I := 0 to TJSONArray(Data).Count - 1 do
    for V := Low(TTabItemStyle) to High(TTabItemStyle) do
      if TabItemStyleNames[V] = TJSONArray(Data).Strings[I] then
        Include(Result, V);
end;

function TabItemStyleSetToJSON(const S: TTabItemStyleSet): TJSONData;
var
  V: TTabItemStyle;
begin
  Result := TJSONArray.Create;
  for V := Low(TTabItemStyle) to High(TTabItemStyle) do
    if V in S then
      TJSONArray(Result).Add(TabItemStyleNames[V]);
end;

const
  TabPositionNames: array[TTabPosition] of string = ('top', 'bottom', 'left', 'right');

function TabPositionOf(Data: TJSONData): TTabPosition;
var
  V: TTabPosition;
begin
  Result := Low(TTabPosition);
  if (Data <> nil) and (Data.JSONType = jtString) then
    for V := Low(TTabPosition) to High(TTabPosition) do
      if TabPositionNames[V] = Data.AsString then
        Exit(V);
end;

function TabPositionSetOf(Data: TJSONData): TTabPositionSet;
var
  I: Integer;
  V: TTabPosition;
begin
  Result := [];
  if (Data = nil) or (Data.JSONType <> jtArray) then
    Exit;
  for I := 0 to TJSONArray(Data).Count - 1 do
    for V := Low(TTabPosition) to High(TTabPosition) do
      if TabPositionNames[V] = TJSONArray(Data).Strings[I] then
        Include(Result, V);
end;

function TabPositionSetToJSON(const S: TTabPositionSet): TJSONData;
var
  V: TTabPosition;
begin
  Result := TJSONArray.Create;
  for V := Low(TTabPosition) to High(TTabPosition) do
    if V in S then
      TJSONArray(Result).Add(TabPositionNames[V]);
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

function TTabControl.GetTabIndex: Int64;
begin
  Result := GetProp('tab_index').AsInt64;
end;

procedure TTabControl.SetTabIndex(Value: Int64);
begin
  SetProp('tab_index', TJSONInt64Number.Create(Value));
end;

function TTabControl.GetShowTabs: Boolean;
begin
  Result := GetProp('show_tabs').AsBoolean;
end;

procedure TTabControl.SetShowTabs(Value: Boolean);
begin
  SetProp('show_tabs', TJSONBoolean.Create(Value));
end;

function TTabControl.GetTransition: TTransition;
begin
  Result := TransitionOf(GetProp('transition'));
end;

procedure TTabControl.SetTransition(Value: TTransition);
begin
  SetProp('transition', TJSONString.Create(TransitionNames[Value]));
end;

function TTabControl.GetSwipe: Boolean;
begin
  Result := GetProp('swipe').AsBoolean;
end;

procedure TTabControl.SetSwipe(Value: Boolean);
begin
  SetProp('swipe', TJSONBoolean.Create(Value));
end;

function TTabControl.GetTabPosition: TTabPosition;
begin
  Result := TabPositionOf(GetProp('tab_position'));
end;

procedure TTabControl.SetTabPosition(Value: TTabPosition);
begin
  SetProp('tab_position', TJSONString.Create(TabPositionNames[Value]));
end;

function TTabControl.GetTabWidth: Double;
begin
  Result := GetProp('tab_width').AsFloat;
end;

procedure TTabControl.SetTabWidth(Value: Double);
begin
  SetProp('tab_width', TJSONFloatNumber.Create(Value));
end;

function TTabControl.GetOnChange: string;
begin
  Result := AsText(GetProp('on_change'));
end;

procedure TTabControl.SetOnChange(Value: string);
begin
  SetProp('on_change', TJSONString.Create(Value));
end;

function TTabControl.GetLeft: Double;
begin
  Result := GetProp('left').AsFloat;
end;

procedure TTabControl.SetLeft(Value: Double);
begin
  SetProp('left', TJSONFloatNumber.Create(Value));
end;

function TTabControl.GetTop: Double;
begin
  Result := GetProp('top').AsFloat;
end;

procedure TTabControl.SetTop(Value: Double);
begin
  SetProp('top', TJSONFloatNumber.Create(Value));
end;

function TTabControl.GetWidth: Double;
begin
  Result := GetProp('width').AsFloat;
end;

procedure TTabControl.SetWidth(Value: Double);
begin
  SetProp('width', TJSONFloatNumber.Create(Value));
end;

function TTabControl.GetHeight: Double;
begin
  Result := GetProp('height').AsFloat;
end;

procedure TTabControl.SetHeight(Value: Double);
begin
  SetProp('height', TJSONFloatNumber.Create(Value));
end;

function TTabControl.GetAlign: TAlign;
begin
  Result := AlignOf(GetProp('align'));
end;

procedure TTabControl.SetAlign(Value: TAlign);
begin
  SetProp('align', TJSONString.Create(AlignNames[Value]));
end;

function TTabControl.GetAnchors: TAnchorSet;
begin
  Result := AnchorSetOf(GetProp('anchors'));
end;

procedure TTabControl.SetAnchors(Value: TAnchorSet);
begin
  SetProp('anchors', AnchorSetToJSON(Value));
end;

function TTabControl.GetMargins: TAylisBounds;
begin
  Result := AylisBoundsFromJSON(GetProp('margins'));
end;

procedure TTabControl.SetMargins(Value: TAylisBounds);
begin
  SetProp('margins', AylisBoundsToJSON(Value));
end;

function TTabControl.GetPadding: TAylisBounds;
begin
  Result := AylisBoundsFromJSON(GetProp('padding'));
end;

procedure TTabControl.SetPadding(Value: TAylisBounds);
begin
  SetProp('padding', AylisBoundsToJSON(Value));
end;

function TTabControl.GetVisible: Boolean;
begin
  Result := GetProp('visible').AsBoolean;
end;

procedure TTabControl.SetVisible(Value: Boolean);
begin
  SetProp('visible', TJSONBoolean.Create(Value));
end;

function TTabControl.GetEnabled: Boolean;
begin
  Result := GetProp('enabled').AsBoolean;
end;

procedure TTabControl.SetEnabled(Value: Boolean);
begin
  SetProp('enabled', TJSONBoolean.Create(Value));
end;

function TTabControl.GetHitTest: Boolean;
begin
  Result := GetProp('hit_test').AsBoolean;
end;

procedure TTabControl.SetHitTest(Value: Boolean);
begin
  SetProp('hit_test', TJSONBoolean.Create(Value));
end;

function TTabControl.GetClipChildren: Boolean;
begin
  Result := GetProp('clip_children').AsBoolean;
end;

procedure TTabControl.SetClipChildren(Value: Boolean);
begin
  SetProp('clip_children', TJSONBoolean.Create(Value));
end;

function TTabControl.GetInheritState: Boolean;
begin
  Result := GetProp('inherit_state').AsBoolean;
end;

procedure TTabControl.SetInheritState(Value: Boolean);
begin
  SetProp('inherit_state', TJSONBoolean.Create(Value));
end;

function TTabControl.GetTranslate: Boolean;
begin
  Result := GetProp('translate').AsBoolean;
end;

procedure TTabControl.SetTranslate(Value: Boolean);
begin
  SetProp('translate', TJSONBoolean.Create(Value));
end;

function TTabControl.GetCapturesWheel: Boolean;
begin
  Result := GetProp('captures_wheel').AsBoolean;
end;

procedure TTabControl.SetCapturesWheel(Value: Boolean);
begin
  SetProp('captures_wheel', TJSONBoolean.Create(Value));
end;

function TTabControl.GetCapturesGesture: Boolean;
begin
  Result := GetProp('captures_gesture').AsBoolean;
end;

procedure TTabControl.SetCapturesGesture(Value: Boolean);
begin
  SetProp('captures_gesture', TJSONBoolean.Create(Value));
end;

function TTabControl.GetCanFocus: Boolean;
begin
  Result := GetProp('can_focus').AsBoolean;
end;

procedure TTabControl.SetCanFocus(Value: Boolean);
begin
  SetProp('can_focus', TJSONBoolean.Create(Value));
end;

function TTabControl.GetDraggable: Boolean;
begin
  Result := GetProp('draggable').AsBoolean;
end;

procedure TTabControl.SetDraggable(Value: Boolean);
begin
  SetProp('draggable', TJSONBoolean.Create(Value));
end;

function TTabControl.GetAcceptsDrops: Boolean;
begin
  Result := GetProp('accepts_drops').AsBoolean;
end;

procedure TTabControl.SetAcceptsDrops(Value: Boolean);
begin
  SetProp('accepts_drops', TJSONBoolean.Create(Value));
end;

function TTabControl.GetDragData: string;
begin
  Result := AsText(GetProp('drag_data'));
end;

procedure TTabControl.SetDragData(Value: string);
begin
  SetProp('drag_data', TJSONString.Create(Value));
end;

function TTabControl.GetDragLabel: string;
begin
  Result := AsText(GetProp('drag_label'));
end;

procedure TTabControl.SetDragLabel(Value: string);
begin
  SetProp('drag_label', TJSONString.Create(Value));
end;

function TTabControl.GetDragImage: TAylisComponent;
begin
  Result := GetRef('drag_image');
end;

procedure TTabControl.SetDragImage(Value: TAylisComponent);
begin
  SetRef('drag_image', Value);
end;

function TTabControl.GetDropTypes: string;
begin
  Result := AsText(GetProp('drop_types'));
end;

procedure TTabControl.SetDropTypes(Value: string);
begin
  SetProp('drop_types', TJSONString.Create(Value));
end;

function TTabControl.GetTabStop: Boolean;
begin
  Result := GetProp('tab_stop').AsBoolean;
end;

procedure TTabControl.SetTabStop(Value: Boolean);
begin
  SetProp('tab_stop', TJSONBoolean.Create(Value));
end;

function TTabControl.GetCursor: TCursorShape;
begin
  Result := CursorShapeOf(GetProp('cursor'));
end;

procedure TTabControl.SetCursor(Value: TCursorShape);
begin
  SetProp('cursor', TJSONString.Create(CursorShapeNames[Value]));
end;

function TTabControl.GetOpacity: Double;
begin
  Result := GetProp('opacity').AsFloat;
end;

procedure TTabControl.SetOpacity(Value: Double);
begin
  SetProp('opacity', TJSONFloatNumber.Create(Value));
end;

function TTabControl.GetBlendMode: TBlendMode;
begin
  Result := BlendModeOf(GetProp('blend_mode'));
end;

procedure TTabControl.SetBlendMode(Value: TBlendMode);
begin
  SetProp('blend_mode', TJSONString.Create(BlendModeNames[Value]));
end;

function TTabControl.GetCacheLayer: Boolean;
begin
  Result := GetProp('cache_layer').AsBoolean;
end;

procedure TTabControl.SetCacheLayer(Value: Boolean);
begin
  SetProp('cache_layer', TJSONBoolean.Create(Value));
end;

function TTabControl.GetRotation: Double;
begin
  Result := GetProp('rotation').AsFloat;
end;

procedure TTabControl.SetRotation(Value: Double);
begin
  SetProp('rotation', TJSONFloatNumber.Create(Value));
end;

function TTabControl.GetScale: Double;
begin
  Result := GetProp('scale').AsFloat;
end;

procedure TTabControl.SetScale(Value: Double);
begin
  SetProp('scale', TJSONFloatNumber.Create(Value));
end;

function TTabControl.GetTranslateX: Double;
begin
  Result := GetProp('translate_x').AsFloat;
end;

procedure TTabControl.SetTranslateX(Value: Double);
begin
  SetProp('translate_x', TJSONFloatNumber.Create(Value));
end;

function TTabControl.GetTranslateY: Double;
begin
  Result := GetProp('translate_y').AsFloat;
end;

procedure TTabControl.SetTranslateY(Value: Double);
begin
  SetProp('translate_y', TJSONFloatNumber.Create(Value));
end;

function TTabControl.GetAspectRatio: Double;
begin
  Result := GetProp('aspect_ratio').AsFloat;
end;

procedure TTabControl.SetAspectRatio(Value: Double);
begin
  SetProp('aspect_ratio', TJSONFloatNumber.Create(Value));
end;

function TTabControl.GetMaxWidth: Double;
begin
  Result := GetProp('max_width').AsFloat;
end;

procedure TTabControl.SetMaxWidth(Value: Double);
begin
  SetProp('max_width', TJSONFloatNumber.Create(Value));
end;

function TTabControl.GetMinWidth: Double;
begin
  Result := GetProp('min_width').AsFloat;
end;

procedure TTabControl.SetMinWidth(Value: Double);
begin
  SetProp('min_width', TJSONFloatNumber.Create(Value));
end;

function TTabControl.GetMinHeight: Double;
begin
  Result := GetProp('min_height').AsFloat;
end;

procedure TTabControl.SetMinHeight(Value: Double);
begin
  SetProp('min_height', TJSONFloatNumber.Create(Value));
end;

function TTabControl.GetMaxHeight: Double;
begin
  Result := GetProp('max_height').AsFloat;
end;

procedure TTabControl.SetMaxHeight(Value: Double);
begin
  SetProp('max_height', TJSONFloatNumber.Create(Value));
end;

function TTabControl.GetSelfAlign: TSelfAlign;
begin
  Result := SelfAlignOf(GetProp('self_align'));
end;

procedure TTabControl.SetSelfAlign(Value: TSelfAlign);
begin
  SetProp('self_align', TJSONString.Create(SelfAlignNames[Value]));
end;

function TTabControl.GetAutoSize: Boolean;
begin
  Result := GetProp('auto_size').AsBoolean;
end;

procedure TTabControl.SetAutoSize(Value: Boolean);
begin
  SetProp('auto_size', TJSONBoolean.Create(Value));
end;

function TTabControl.GetColumnSpan: Int64;
begin
  Result := GetProp('column_span').AsInt64;
end;

procedure TTabControl.SetColumnSpan(Value: Int64);
begin
  SetProp('column_span', TJSONInt64Number.Create(Value));
end;

function TTabControl.GetGridRow: Int64;
begin
  Result := GetProp('grid_row').AsInt64;
end;

procedure TTabControl.SetGridRow(Value: Int64);
begin
  SetProp('grid_row', TJSONInt64Number.Create(Value));
end;

function TTabControl.GetGridColumn: Int64;
begin
  Result := GetProp('grid_column').AsInt64;
end;

procedure TTabControl.SetGridColumn(Value: Int64);
begin
  SetProp('grid_column', TJSONInt64Number.Create(Value));
end;

function TTabControl.GetRowSpan: Int64;
begin
  Result := GetProp('row_span').AsInt64;
end;

procedure TTabControl.SetRowSpan(Value: Int64);
begin
  SetProp('row_span', TJSONInt64Number.Create(Value));
end;

function TTabControl.GetGrow: Double;
begin
  Result := GetProp('grow').AsFloat;
end;

procedure TTabControl.SetGrow(Value: Double);
begin
  SetProp('grow', TJSONFloatNumber.Create(Value));
end;

function TTabControl.GetShadow: TAylisShadow;
begin
  Result := AylisShadowFromJSON(GetProp('shadow'));
end;

procedure TTabControl.SetShadow(Value: TAylisShadow);
begin
  SetProp('shadow', AylisShadowToJSON(Value));
end;

function TTabControl.GetStyle: TTabControlStyle;
begin
  Result := TabControlStyleOf(GetProp('style'));
end;

procedure TTabControl.SetStyle(Value: TTabControlStyle);
begin
  SetProp('style', TJSONString.Create(TabControlStyleNames[Value]));
end;

function TTabControl.GetTheme: TThemeName;
begin
  Result := ThemeNameOf(GetProp('theme'));
end;

procedure TTabControl.SetTheme(Value: TThemeName);
begin
  SetProp('theme', TJSONString.Create(ThemeNameNames[Value]));
end;

function TTabControl.GetFill: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('fill'));
end;

procedure TTabControl.SetFill(Value: TAylisColor);
begin
  SetProp('fill', AylisColorToJSON(Value));
end;

function TTabControl.GetBorderColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('border_color'));
end;

procedure TTabControl.SetBorderColor(Value: TAylisColor);
begin
  SetProp('border_color', AylisColorToJSON(Value));
end;

function TTabControl.GetTextColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('text_color'));
end;

procedure TTabControl.SetTextColor(Value: TAylisColor);
begin
  SetProp('text_color', AylisColorToJSON(Value));
end;

function TTabControl.GetAccentColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('accent_color'));
end;

procedure TTabControl.SetAccentColor(Value: TAylisColor);
begin
  SetProp('accent_color', AylisColorToJSON(Value));
end;

function TTabControl.GetSuccessColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('success_color'));
end;

procedure TTabControl.SetSuccessColor(Value: TAylisColor);
begin
  SetProp('success_color', AylisColorToJSON(Value));
end;

function TTabControl.GetWarningColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('warning_color'));
end;

procedure TTabControl.SetWarningColor(Value: TAylisColor);
begin
  SetProp('warning_color', AylisColorToJSON(Value));
end;

function TTabControl.GetDangerColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('danger_color'));
end;

procedure TTabControl.SetDangerColor(Value: TAylisColor);
begin
  SetProp('danger_color', AylisColorToJSON(Value));
end;

function TTabControl.GetFontFamily: string;
begin
  Result := AsText(GetProp('font_family'));
end;

procedure TTabControl.SetFontFamily(Value: string);
begin
  SetProp('font_family', TJSONString.Create(Value));
end;

function TTabControl.GetFontSize: Double;
begin
  Result := GetProp('font_size').AsFloat;
end;

procedure TTabControl.SetFontSize(Value: Double);
begin
  SetProp('font_size', TJSONFloatNumber.Create(Value));
end;

function TTabControl.GetFontWeight: TFontWeight;
begin
  Result := FontWeightOf(GetProp('font_weight'));
end;

procedure TTabControl.SetFontWeight(Value: TFontWeight);
begin
  SetProp('font_weight', TJSONString.Create(FontWeightNames[Value]));
end;

function TTabControl.GetFontItalic: Boolean;
begin
  Result := GetProp('font_italic').AsBoolean;
end;

procedure TTabControl.SetFontItalic(Value: Boolean);
begin
  SetProp('font_italic', TJSONBoolean.Create(Value));
end;

function TTabControl.GetFontUnderline: Boolean;
begin
  Result := GetProp('font_underline').AsBoolean;
end;

procedure TTabControl.SetFontUnderline(Value: Boolean);
begin
  SetProp('font_underline', TJSONBoolean.Create(Value));
end;

function TTabControl.GetFontStrikeout: Boolean;
begin
  Result := GetProp('font_strikeout').AsBoolean;
end;

procedure TTabControl.SetFontStrikeout(Value: Boolean);
begin
  SetProp('font_strikeout', TJSONBoolean.Create(Value));
end;

function TTabControl.GetHint: string;
begin
  Result := AsText(GetProp('hint'));
end;

procedure TTabControl.SetHint(Value: string);
begin
  SetProp('hint', TJSONString.Create(Value));
end;

function TTabControl.GetTag: Int64;
begin
  Result := GetProp('tag').AsInt64;
end;

procedure TTabControl.SetTag(Value: Int64);
begin
  SetProp('tag', TJSONInt64Number.Create(Value));
end;

function TTabControl.GetTagString: string;
begin
  Result := AsText(GetProp('tag_string'));
end;

procedure TTabControl.SetTagString(Value: string);
begin
  SetProp('tag_string', TJSONString.Create(Value));
end;

function TTabControl.GetAccessibleName: string;
begin
  Result := AsText(GetProp('accessible_name'));
end;

procedure TTabControl.SetAccessibleName(Value: string);
begin
  SetProp('accessible_name', TJSONString.Create(Value));
end;

function TTabControl.GetPopupMenu: TAylisComponent;
begin
  Result := GetRef('popup_menu');
end;

procedure TTabControl.SetPopupMenu(Value: TAylisComponent);
begin
  SetRef('popup_menu', Value);
end;

function TTabControl.GetOnClick: string;
begin
  Result := AsText(GetProp('on_click'));
end;

procedure TTabControl.SetOnClick(Value: string);
begin
  SetProp('on_click', TJSONString.Create(Value));
end;

function TTabControl.GetOnDblClick: string;
begin
  Result := AsText(GetProp('on_dbl_click'));
end;

procedure TTabControl.SetOnDblClick(Value: string);
begin
  SetProp('on_dbl_click', TJSONString.Create(Value));
end;

function TTabControl.GetOnLongPress: string;
begin
  Result := AsText(GetProp('on_long_press'));
end;

procedure TTabControl.SetOnLongPress(Value: string);
begin
  SetProp('on_long_press', TJSONString.Create(Value));
end;

function TTabControl.GetOnPointerDown: string;
begin
  Result := AsText(GetProp('on_pointer_down'));
end;

procedure TTabControl.SetOnPointerDown(Value: string);
begin
  SetProp('on_pointer_down', TJSONString.Create(Value));
end;

function TTabControl.GetOnPointerUp: string;
begin
  Result := AsText(GetProp('on_pointer_up'));
end;

procedure TTabControl.SetOnPointerUp(Value: string);
begin
  SetProp('on_pointer_up', TJSONString.Create(Value));
end;

function TTabControl.GetOnPointerMove: string;
begin
  Result := AsText(GetProp('on_pointer_move'));
end;

procedure TTabControl.SetOnPointerMove(Value: string);
begin
  SetProp('on_pointer_move', TJSONString.Create(Value));
end;

function TTabControl.GetOnPointerEnter: string;
begin
  Result := AsText(GetProp('on_pointer_enter'));
end;

procedure TTabControl.SetOnPointerEnter(Value: string);
begin
  SetProp('on_pointer_enter', TJSONString.Create(Value));
end;

function TTabControl.GetOnPointerLeave: string;
begin
  Result := AsText(GetProp('on_pointer_leave'));
end;

procedure TTabControl.SetOnPointerLeave(Value: string);
begin
  SetProp('on_pointer_leave', TJSONString.Create(Value));
end;

function TTabControl.GetOnWheel: string;
begin
  Result := AsText(GetProp('on_wheel'));
end;

procedure TTabControl.SetOnWheel(Value: string);
begin
  SetProp('on_wheel', TJSONString.Create(Value));
end;

function TTabControl.GetOnGesture: string;
begin
  Result := AsText(GetProp('on_gesture'));
end;

procedure TTabControl.SetOnGesture(Value: string);
begin
  SetProp('on_gesture', TJSONString.Create(Value));
end;

function TTabControl.GetOnSwipe: string;
begin
  Result := AsText(GetProp('on_swipe'));
end;

procedure TTabControl.SetOnSwipe(Value: string);
begin
  SetProp('on_swipe', TJSONString.Create(Value));
end;

function TTabControl.GetOnKeyUp: string;
begin
  Result := AsText(GetProp('on_key_up'));
end;

procedure TTabControl.SetOnKeyUp(Value: string);
begin
  SetProp('on_key_up', TJSONString.Create(Value));
end;

function TTabControl.GetOnEnter: string;
begin
  Result := AsText(GetProp('on_enter'));
end;

procedure TTabControl.SetOnEnter(Value: string);
begin
  SetProp('on_enter', TJSONString.Create(Value));
end;

function TTabControl.GetOnExit: string;
begin
  Result := AsText(GetProp('on_exit'));
end;

procedure TTabControl.SetOnExit(Value: string);
begin
  SetProp('on_exit', TJSONString.Create(Value));
end;

function TTabControl.GetOnResize: string;
begin
  Result := AsText(GetProp('on_resize'));
end;

procedure TTabControl.SetOnResize(Value: string);
begin
  SetProp('on_resize', TJSONString.Create(Value));
end;

function TTabControl.GetOnDragMove: string;
begin
  Result := AsText(GetProp('on_drag_move'));
end;

procedure TTabControl.SetOnDragMove(Value: string);
begin
  SetProp('on_drag_move', TJSONString.Create(Value));
end;

function TTabControl.GetOnDragLeave: string;
begin
  Result := AsText(GetProp('on_drag_leave'));
end;

procedure TTabControl.SetOnDragLeave(Value: string);
begin
  SetProp('on_drag_leave', TJSONString.Create(Value));
end;

function TTabControl.GetOnDragDrop: string;
begin
  Result := AsText(GetProp('on_drag_drop'));
end;

procedure TTabControl.SetOnDragDrop(Value: string);
begin
  SetProp('on_drag_drop', TJSONString.Create(Value));
end;

function TTabControl.GetOnDragEnd: string;
begin
  Result := AsText(GetProp('on_drag_end'));
end;

procedure TTabControl.SetOnDragEnd(Value: string);
begin
  SetProp('on_drag_end', TJSONString.Create(Value));
end;

procedure TTabControl.SetFocus;
begin
  Aylis.Send('focus', TJSONObject.Create(['target', Id]));
end;

function TTabItem.GetCaption: string;
begin
  Result := AsText(GetProp('caption'));
end;

procedure TTabItem.SetCaption(Value: string);
begin
  SetProp('caption', TJSONString.Create(Value));
end;

function TTabItem.GetOnShow: string;
begin
  Result := AsText(GetProp('on_show'));
end;

procedure TTabItem.SetOnShow(Value: string);
begin
  SetProp('on_show', TJSONString.Create(Value));
end;

function TTabItem.GetLeft: Double;
begin
  Result := GetProp('left').AsFloat;
end;

procedure TTabItem.SetLeft(Value: Double);
begin
  SetProp('left', TJSONFloatNumber.Create(Value));
end;

function TTabItem.GetTop: Double;
begin
  Result := GetProp('top').AsFloat;
end;

procedure TTabItem.SetTop(Value: Double);
begin
  SetProp('top', TJSONFloatNumber.Create(Value));
end;

function TTabItem.GetWidth: Double;
begin
  Result := GetProp('width').AsFloat;
end;

procedure TTabItem.SetWidth(Value: Double);
begin
  SetProp('width', TJSONFloatNumber.Create(Value));
end;

function TTabItem.GetHeight: Double;
begin
  Result := GetProp('height').AsFloat;
end;

procedure TTabItem.SetHeight(Value: Double);
begin
  SetProp('height', TJSONFloatNumber.Create(Value));
end;

function TTabItem.GetAlign: TAlign;
begin
  Result := AlignOf(GetProp('align'));
end;

procedure TTabItem.SetAlign(Value: TAlign);
begin
  SetProp('align', TJSONString.Create(AlignNames[Value]));
end;

function TTabItem.GetAnchors: TAnchorSet;
begin
  Result := AnchorSetOf(GetProp('anchors'));
end;

procedure TTabItem.SetAnchors(Value: TAnchorSet);
begin
  SetProp('anchors', AnchorSetToJSON(Value));
end;

function TTabItem.GetMargins: TAylisBounds;
begin
  Result := AylisBoundsFromJSON(GetProp('margins'));
end;

procedure TTabItem.SetMargins(Value: TAylisBounds);
begin
  SetProp('margins', AylisBoundsToJSON(Value));
end;

function TTabItem.GetPadding: TAylisBounds;
begin
  Result := AylisBoundsFromJSON(GetProp('padding'));
end;

procedure TTabItem.SetPadding(Value: TAylisBounds);
begin
  SetProp('padding', AylisBoundsToJSON(Value));
end;

function TTabItem.GetVisible: Boolean;
begin
  Result := GetProp('visible').AsBoolean;
end;

procedure TTabItem.SetVisible(Value: Boolean);
begin
  SetProp('visible', TJSONBoolean.Create(Value));
end;

function TTabItem.GetEnabled: Boolean;
begin
  Result := GetProp('enabled').AsBoolean;
end;

procedure TTabItem.SetEnabled(Value: Boolean);
begin
  SetProp('enabled', TJSONBoolean.Create(Value));
end;

function TTabItem.GetHitTest: Boolean;
begin
  Result := GetProp('hit_test').AsBoolean;
end;

procedure TTabItem.SetHitTest(Value: Boolean);
begin
  SetProp('hit_test', TJSONBoolean.Create(Value));
end;

function TTabItem.GetClipChildren: Boolean;
begin
  Result := GetProp('clip_children').AsBoolean;
end;

procedure TTabItem.SetClipChildren(Value: Boolean);
begin
  SetProp('clip_children', TJSONBoolean.Create(Value));
end;

function TTabItem.GetInheritState: Boolean;
begin
  Result := GetProp('inherit_state').AsBoolean;
end;

procedure TTabItem.SetInheritState(Value: Boolean);
begin
  SetProp('inherit_state', TJSONBoolean.Create(Value));
end;

function TTabItem.GetTranslate: Boolean;
begin
  Result := GetProp('translate').AsBoolean;
end;

procedure TTabItem.SetTranslate(Value: Boolean);
begin
  SetProp('translate', TJSONBoolean.Create(Value));
end;

function TTabItem.GetCapturesWheel: Boolean;
begin
  Result := GetProp('captures_wheel').AsBoolean;
end;

procedure TTabItem.SetCapturesWheel(Value: Boolean);
begin
  SetProp('captures_wheel', TJSONBoolean.Create(Value));
end;

function TTabItem.GetCapturesGesture: Boolean;
begin
  Result := GetProp('captures_gesture').AsBoolean;
end;

procedure TTabItem.SetCapturesGesture(Value: Boolean);
begin
  SetProp('captures_gesture', TJSONBoolean.Create(Value));
end;

function TTabItem.GetCanFocus: Boolean;
begin
  Result := GetProp('can_focus').AsBoolean;
end;

procedure TTabItem.SetCanFocus(Value: Boolean);
begin
  SetProp('can_focus', TJSONBoolean.Create(Value));
end;

function TTabItem.GetDraggable: Boolean;
begin
  Result := GetProp('draggable').AsBoolean;
end;

procedure TTabItem.SetDraggable(Value: Boolean);
begin
  SetProp('draggable', TJSONBoolean.Create(Value));
end;

function TTabItem.GetAcceptsDrops: Boolean;
begin
  Result := GetProp('accepts_drops').AsBoolean;
end;

procedure TTabItem.SetAcceptsDrops(Value: Boolean);
begin
  SetProp('accepts_drops', TJSONBoolean.Create(Value));
end;

function TTabItem.GetDragData: string;
begin
  Result := AsText(GetProp('drag_data'));
end;

procedure TTabItem.SetDragData(Value: string);
begin
  SetProp('drag_data', TJSONString.Create(Value));
end;

function TTabItem.GetDragLabel: string;
begin
  Result := AsText(GetProp('drag_label'));
end;

procedure TTabItem.SetDragLabel(Value: string);
begin
  SetProp('drag_label', TJSONString.Create(Value));
end;

function TTabItem.GetDragImage: TAylisComponent;
begin
  Result := GetRef('drag_image');
end;

procedure TTabItem.SetDragImage(Value: TAylisComponent);
begin
  SetRef('drag_image', Value);
end;

function TTabItem.GetDropTypes: string;
begin
  Result := AsText(GetProp('drop_types'));
end;

procedure TTabItem.SetDropTypes(Value: string);
begin
  SetProp('drop_types', TJSONString.Create(Value));
end;

function TTabItem.GetTabStop: Boolean;
begin
  Result := GetProp('tab_stop').AsBoolean;
end;

procedure TTabItem.SetTabStop(Value: Boolean);
begin
  SetProp('tab_stop', TJSONBoolean.Create(Value));
end;

function TTabItem.GetCursor: TCursorShape;
begin
  Result := CursorShapeOf(GetProp('cursor'));
end;

procedure TTabItem.SetCursor(Value: TCursorShape);
begin
  SetProp('cursor', TJSONString.Create(CursorShapeNames[Value]));
end;

function TTabItem.GetOpacity: Double;
begin
  Result := GetProp('opacity').AsFloat;
end;

procedure TTabItem.SetOpacity(Value: Double);
begin
  SetProp('opacity', TJSONFloatNumber.Create(Value));
end;

function TTabItem.GetBlendMode: TBlendMode;
begin
  Result := BlendModeOf(GetProp('blend_mode'));
end;

procedure TTabItem.SetBlendMode(Value: TBlendMode);
begin
  SetProp('blend_mode', TJSONString.Create(BlendModeNames[Value]));
end;

function TTabItem.GetCacheLayer: Boolean;
begin
  Result := GetProp('cache_layer').AsBoolean;
end;

procedure TTabItem.SetCacheLayer(Value: Boolean);
begin
  SetProp('cache_layer', TJSONBoolean.Create(Value));
end;

function TTabItem.GetRotation: Double;
begin
  Result := GetProp('rotation').AsFloat;
end;

procedure TTabItem.SetRotation(Value: Double);
begin
  SetProp('rotation', TJSONFloatNumber.Create(Value));
end;

function TTabItem.GetScale: Double;
begin
  Result := GetProp('scale').AsFloat;
end;

procedure TTabItem.SetScale(Value: Double);
begin
  SetProp('scale', TJSONFloatNumber.Create(Value));
end;

function TTabItem.GetTranslateX: Double;
begin
  Result := GetProp('translate_x').AsFloat;
end;

procedure TTabItem.SetTranslateX(Value: Double);
begin
  SetProp('translate_x', TJSONFloatNumber.Create(Value));
end;

function TTabItem.GetTranslateY: Double;
begin
  Result := GetProp('translate_y').AsFloat;
end;

procedure TTabItem.SetTranslateY(Value: Double);
begin
  SetProp('translate_y', TJSONFloatNumber.Create(Value));
end;

function TTabItem.GetAspectRatio: Double;
begin
  Result := GetProp('aspect_ratio').AsFloat;
end;

procedure TTabItem.SetAspectRatio(Value: Double);
begin
  SetProp('aspect_ratio', TJSONFloatNumber.Create(Value));
end;

function TTabItem.GetMaxWidth: Double;
begin
  Result := GetProp('max_width').AsFloat;
end;

procedure TTabItem.SetMaxWidth(Value: Double);
begin
  SetProp('max_width', TJSONFloatNumber.Create(Value));
end;

function TTabItem.GetMinWidth: Double;
begin
  Result := GetProp('min_width').AsFloat;
end;

procedure TTabItem.SetMinWidth(Value: Double);
begin
  SetProp('min_width', TJSONFloatNumber.Create(Value));
end;

function TTabItem.GetMinHeight: Double;
begin
  Result := GetProp('min_height').AsFloat;
end;

procedure TTabItem.SetMinHeight(Value: Double);
begin
  SetProp('min_height', TJSONFloatNumber.Create(Value));
end;

function TTabItem.GetMaxHeight: Double;
begin
  Result := GetProp('max_height').AsFloat;
end;

procedure TTabItem.SetMaxHeight(Value: Double);
begin
  SetProp('max_height', TJSONFloatNumber.Create(Value));
end;

function TTabItem.GetSelfAlign: TSelfAlign;
begin
  Result := SelfAlignOf(GetProp('self_align'));
end;

procedure TTabItem.SetSelfAlign(Value: TSelfAlign);
begin
  SetProp('self_align', TJSONString.Create(SelfAlignNames[Value]));
end;

function TTabItem.GetAutoSize: Boolean;
begin
  Result := GetProp('auto_size').AsBoolean;
end;

procedure TTabItem.SetAutoSize(Value: Boolean);
begin
  SetProp('auto_size', TJSONBoolean.Create(Value));
end;

function TTabItem.GetColumnSpan: Int64;
begin
  Result := GetProp('column_span').AsInt64;
end;

procedure TTabItem.SetColumnSpan(Value: Int64);
begin
  SetProp('column_span', TJSONInt64Number.Create(Value));
end;

function TTabItem.GetGridRow: Int64;
begin
  Result := GetProp('grid_row').AsInt64;
end;

procedure TTabItem.SetGridRow(Value: Int64);
begin
  SetProp('grid_row', TJSONInt64Number.Create(Value));
end;

function TTabItem.GetGridColumn: Int64;
begin
  Result := GetProp('grid_column').AsInt64;
end;

procedure TTabItem.SetGridColumn(Value: Int64);
begin
  SetProp('grid_column', TJSONInt64Number.Create(Value));
end;

function TTabItem.GetRowSpan: Int64;
begin
  Result := GetProp('row_span').AsInt64;
end;

procedure TTabItem.SetRowSpan(Value: Int64);
begin
  SetProp('row_span', TJSONInt64Number.Create(Value));
end;

function TTabItem.GetGrow: Double;
begin
  Result := GetProp('grow').AsFloat;
end;

procedure TTabItem.SetGrow(Value: Double);
begin
  SetProp('grow', TJSONFloatNumber.Create(Value));
end;

function TTabItem.GetShadow: TAylisShadow;
begin
  Result := AylisShadowFromJSON(GetProp('shadow'));
end;

procedure TTabItem.SetShadow(Value: TAylisShadow);
begin
  SetProp('shadow', AylisShadowToJSON(Value));
end;

function TTabItem.GetStyle: TTabItemStyle;
begin
  Result := TabItemStyleOf(GetProp('style'));
end;

procedure TTabItem.SetStyle(Value: TTabItemStyle);
begin
  SetProp('style', TJSONString.Create(TabItemStyleNames[Value]));
end;

function TTabItem.GetTheme: TThemeName;
begin
  Result := ThemeNameOf(GetProp('theme'));
end;

procedure TTabItem.SetTheme(Value: TThemeName);
begin
  SetProp('theme', TJSONString.Create(ThemeNameNames[Value]));
end;

function TTabItem.GetFill: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('fill'));
end;

procedure TTabItem.SetFill(Value: TAylisColor);
begin
  SetProp('fill', AylisColorToJSON(Value));
end;

function TTabItem.GetBorderColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('border_color'));
end;

procedure TTabItem.SetBorderColor(Value: TAylisColor);
begin
  SetProp('border_color', AylisColorToJSON(Value));
end;

function TTabItem.GetTextColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('text_color'));
end;

procedure TTabItem.SetTextColor(Value: TAylisColor);
begin
  SetProp('text_color', AylisColorToJSON(Value));
end;

function TTabItem.GetAccentColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('accent_color'));
end;

procedure TTabItem.SetAccentColor(Value: TAylisColor);
begin
  SetProp('accent_color', AylisColorToJSON(Value));
end;

function TTabItem.GetSuccessColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('success_color'));
end;

procedure TTabItem.SetSuccessColor(Value: TAylisColor);
begin
  SetProp('success_color', AylisColorToJSON(Value));
end;

function TTabItem.GetWarningColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('warning_color'));
end;

procedure TTabItem.SetWarningColor(Value: TAylisColor);
begin
  SetProp('warning_color', AylisColorToJSON(Value));
end;

function TTabItem.GetDangerColor: TAylisColor;
begin
  Result := AylisColorFromJSON(GetProp('danger_color'));
end;

procedure TTabItem.SetDangerColor(Value: TAylisColor);
begin
  SetProp('danger_color', AylisColorToJSON(Value));
end;

function TTabItem.GetFontFamily: string;
begin
  Result := AsText(GetProp('font_family'));
end;

procedure TTabItem.SetFontFamily(Value: string);
begin
  SetProp('font_family', TJSONString.Create(Value));
end;

function TTabItem.GetFontSize: Double;
begin
  Result := GetProp('font_size').AsFloat;
end;

procedure TTabItem.SetFontSize(Value: Double);
begin
  SetProp('font_size', TJSONFloatNumber.Create(Value));
end;

function TTabItem.GetFontWeight: TFontWeight;
begin
  Result := FontWeightOf(GetProp('font_weight'));
end;

procedure TTabItem.SetFontWeight(Value: TFontWeight);
begin
  SetProp('font_weight', TJSONString.Create(FontWeightNames[Value]));
end;

function TTabItem.GetFontItalic: Boolean;
begin
  Result := GetProp('font_italic').AsBoolean;
end;

procedure TTabItem.SetFontItalic(Value: Boolean);
begin
  SetProp('font_italic', TJSONBoolean.Create(Value));
end;

function TTabItem.GetFontUnderline: Boolean;
begin
  Result := GetProp('font_underline').AsBoolean;
end;

procedure TTabItem.SetFontUnderline(Value: Boolean);
begin
  SetProp('font_underline', TJSONBoolean.Create(Value));
end;

function TTabItem.GetFontStrikeout: Boolean;
begin
  Result := GetProp('font_strikeout').AsBoolean;
end;

procedure TTabItem.SetFontStrikeout(Value: Boolean);
begin
  SetProp('font_strikeout', TJSONBoolean.Create(Value));
end;

function TTabItem.GetHint: string;
begin
  Result := AsText(GetProp('hint'));
end;

procedure TTabItem.SetHint(Value: string);
begin
  SetProp('hint', TJSONString.Create(Value));
end;

function TTabItem.GetTag: Int64;
begin
  Result := GetProp('tag').AsInt64;
end;

procedure TTabItem.SetTag(Value: Int64);
begin
  SetProp('tag', TJSONInt64Number.Create(Value));
end;

function TTabItem.GetTagString: string;
begin
  Result := AsText(GetProp('tag_string'));
end;

procedure TTabItem.SetTagString(Value: string);
begin
  SetProp('tag_string', TJSONString.Create(Value));
end;

function TTabItem.GetAccessibleName: string;
begin
  Result := AsText(GetProp('accessible_name'));
end;

procedure TTabItem.SetAccessibleName(Value: string);
begin
  SetProp('accessible_name', TJSONString.Create(Value));
end;

function TTabItem.GetPopupMenu: TAylisComponent;
begin
  Result := GetRef('popup_menu');
end;

procedure TTabItem.SetPopupMenu(Value: TAylisComponent);
begin
  SetRef('popup_menu', Value);
end;

function TTabItem.GetOnClick: string;
begin
  Result := AsText(GetProp('on_click'));
end;

procedure TTabItem.SetOnClick(Value: string);
begin
  SetProp('on_click', TJSONString.Create(Value));
end;

function TTabItem.GetOnDblClick: string;
begin
  Result := AsText(GetProp('on_dbl_click'));
end;

procedure TTabItem.SetOnDblClick(Value: string);
begin
  SetProp('on_dbl_click', TJSONString.Create(Value));
end;

function TTabItem.GetOnLongPress: string;
begin
  Result := AsText(GetProp('on_long_press'));
end;

procedure TTabItem.SetOnLongPress(Value: string);
begin
  SetProp('on_long_press', TJSONString.Create(Value));
end;

function TTabItem.GetOnPointerDown: string;
begin
  Result := AsText(GetProp('on_pointer_down'));
end;

procedure TTabItem.SetOnPointerDown(Value: string);
begin
  SetProp('on_pointer_down', TJSONString.Create(Value));
end;

function TTabItem.GetOnPointerUp: string;
begin
  Result := AsText(GetProp('on_pointer_up'));
end;

procedure TTabItem.SetOnPointerUp(Value: string);
begin
  SetProp('on_pointer_up', TJSONString.Create(Value));
end;

function TTabItem.GetOnPointerMove: string;
begin
  Result := AsText(GetProp('on_pointer_move'));
end;

procedure TTabItem.SetOnPointerMove(Value: string);
begin
  SetProp('on_pointer_move', TJSONString.Create(Value));
end;

function TTabItem.GetOnPointerEnter: string;
begin
  Result := AsText(GetProp('on_pointer_enter'));
end;

procedure TTabItem.SetOnPointerEnter(Value: string);
begin
  SetProp('on_pointer_enter', TJSONString.Create(Value));
end;

function TTabItem.GetOnPointerLeave: string;
begin
  Result := AsText(GetProp('on_pointer_leave'));
end;

procedure TTabItem.SetOnPointerLeave(Value: string);
begin
  SetProp('on_pointer_leave', TJSONString.Create(Value));
end;

function TTabItem.GetOnWheel: string;
begin
  Result := AsText(GetProp('on_wheel'));
end;

procedure TTabItem.SetOnWheel(Value: string);
begin
  SetProp('on_wheel', TJSONString.Create(Value));
end;

function TTabItem.GetOnGesture: string;
begin
  Result := AsText(GetProp('on_gesture'));
end;

procedure TTabItem.SetOnGesture(Value: string);
begin
  SetProp('on_gesture', TJSONString.Create(Value));
end;

function TTabItem.GetOnSwipe: string;
begin
  Result := AsText(GetProp('on_swipe'));
end;

procedure TTabItem.SetOnSwipe(Value: string);
begin
  SetProp('on_swipe', TJSONString.Create(Value));
end;

function TTabItem.GetOnKeyUp: string;
begin
  Result := AsText(GetProp('on_key_up'));
end;

procedure TTabItem.SetOnKeyUp(Value: string);
begin
  SetProp('on_key_up', TJSONString.Create(Value));
end;

function TTabItem.GetOnEnter: string;
begin
  Result := AsText(GetProp('on_enter'));
end;

procedure TTabItem.SetOnEnter(Value: string);
begin
  SetProp('on_enter', TJSONString.Create(Value));
end;

function TTabItem.GetOnExit: string;
begin
  Result := AsText(GetProp('on_exit'));
end;

procedure TTabItem.SetOnExit(Value: string);
begin
  SetProp('on_exit', TJSONString.Create(Value));
end;

function TTabItem.GetOnResize: string;
begin
  Result := AsText(GetProp('on_resize'));
end;

procedure TTabItem.SetOnResize(Value: string);
begin
  SetProp('on_resize', TJSONString.Create(Value));
end;

function TTabItem.GetOnDragMove: string;
begin
  Result := AsText(GetProp('on_drag_move'));
end;

procedure TTabItem.SetOnDragMove(Value: string);
begin
  SetProp('on_drag_move', TJSONString.Create(Value));
end;

function TTabItem.GetOnDragLeave: string;
begin
  Result := AsText(GetProp('on_drag_leave'));
end;

procedure TTabItem.SetOnDragLeave(Value: string);
begin
  SetProp('on_drag_leave', TJSONString.Create(Value));
end;

function TTabItem.GetOnDragDrop: string;
begin
  Result := AsText(GetProp('on_drag_drop'));
end;

procedure TTabItem.SetOnDragDrop(Value: string);
begin
  SetProp('on_drag_drop', TJSONString.Create(Value));
end;

function TTabItem.GetOnDragEnd: string;
begin
  Result := AsText(GetProp('on_drag_end'));
end;

procedure TTabItem.SetOnDragEnd(Value: string);
begin
  SetProp('on_drag_end', TJSONString.Create(Value));
end;

procedure TFrmCalculadoraBase.BindComponents(List: TJSONArray);
begin
  pnlLateral := TPanel(Resolve(List, 'pnlLateral', 'Panel', TPanel));
  tabLateral := TTabControl(Resolve(List, 'tabLateral', 'TabControl', TTabControl));
  tabHistorico := TTabItem(Resolve(List, 'tabHistorico', 'TabItem', TTabItem));
  btnLimparHistorico := TButton(Resolve(List, 'btnLimparHistorico', 'Button', TButton));
  lvHistorico := TListView(Resolve(List, 'lvHistorico', 'ListView', TListView));
  tabMemoria := TTabItem(Resolve(List, 'tabMemoria', 'TabItem', TTabItem));
  lvMemoria := TListView(Resolve(List, 'lvMemoria', 'ListView', TListView));
  lytCorpo := TLayout(Resolve(List, 'lytCorpo', 'Layout', TLayout));
  lytTopo := TLayout(Resolve(List, 'lytTopo', 'Layout', TLayout));
  btnHistorico := TButton(Resolve(List, 'btnHistorico', 'Button', TButton));
  lblTitulo := TLabel(Resolve(List, 'lblTitulo', 'Label', TLabel));
  lblExpressao := TLabel(Resolve(List, 'lblExpressao', 'Label', TLabel));
  lblVisor := TLabel(Resolve(List, 'lblVisor', 'Label', TLabel));
  grdMemoria := TGridLayout(Resolve(List, 'grdMemoria', 'GridLayout', TGridLayout));
  btnMC := TButton(Resolve(List, 'btnMC', 'Button', TButton));
  btnMR := TButton(Resolve(List, 'btnMR', 'Button', TButton));
  btnMMais := TButton(Resolve(List, 'btnMMais', 'Button', TButton));
  btnMMenos := TButton(Resolve(List, 'btnMMenos', 'Button', TButton));
  btnMS := TButton(Resolve(List, 'btnMS', 'Button', TButton));
  grdTeclado := TGridLayout(Resolve(List, 'grdTeclado', 'GridLayout', TGridLayout));
  btnPorcento := TButton(Resolve(List, 'btnPorcento', 'Button', TButton));
  btnCE := TButton(Resolve(List, 'btnCE', 'Button', TButton));
  btnC := TButton(Resolve(List, 'btnC', 'Button', TButton));
  btnApagar := TButton(Resolve(List, 'btnApagar', 'Button', TButton));
  btnInverso := TButton(Resolve(List, 'btnInverso', 'Button', TButton));
  btnQuadrado := TButton(Resolve(List, 'btnQuadrado', 'Button', TButton));
  btnRaiz := TButton(Resolve(List, 'btnRaiz', 'Button', TButton));
  btnDividir := TButton(Resolve(List, 'btnDividir', 'Button', TButton));
  btn7 := TButton(Resolve(List, 'btn7', 'Button', TButton));
  btn8 := TButton(Resolve(List, 'btn8', 'Button', TButton));
  btn9 := TButton(Resolve(List, 'btn9', 'Button', TButton));
  btnMultiplicar := TButton(Resolve(List, 'btnMultiplicar', 'Button', TButton));
  btn4 := TButton(Resolve(List, 'btn4', 'Button', TButton));
  btn5 := TButton(Resolve(List, 'btn5', 'Button', TButton));
  btn6 := TButton(Resolve(List, 'btn6', 'Button', TButton));
  btnSubtrair := TButton(Resolve(List, 'btnSubtrair', 'Button', TButton));
  btn1 := TButton(Resolve(List, 'btn1', 'Button', TButton));
  btn2 := TButton(Resolve(List, 'btn2', 'Button', TButton));
  btn3 := TButton(Resolve(List, 'btn3', 'Button', TButton));
  btnSomar := TButton(Resolve(List, 'btnSomar', 'Button', TButton));
  btnSinal := TButton(Resolve(List, 'btnSinal', 'Button', TButton));
  btn0 := TButton(Resolve(List, 'btn0', 'Button', TButton));
  btnVirgula := TButton(Resolve(List, 'btnVirgula', 'Button', TButton));
  btnIgual := TButton(Resolve(List, 'btnIgual', 'Button', TButton));
end;

function TFrmCalculadoraBase.FindButton(const Nome: string): TButton;
begin
  Result := nil;
  if Nome = 'btnLimparHistorico' then
    Result := btnLimparHistorico;
  if Nome = 'btnHistorico' then
    Result := btnHistorico;
  if Nome = 'btnMC' then
    Result := btnMC;
  if Nome = 'btnMR' then
    Result := btnMR;
  if Nome = 'btnMMais' then
    Result := btnMMais;
  if Nome = 'btnMMenos' then
    Result := btnMMenos;
  if Nome = 'btnMS' then
    Result := btnMS;
  if Nome = 'btnPorcento' then
    Result := btnPorcento;
  if Nome = 'btnCE' then
    Result := btnCE;
  if Nome = 'btnC' then
    Result := btnC;
  if Nome = 'btnApagar' then
    Result := btnApagar;
  if Nome = 'btnInverso' then
    Result := btnInverso;
  if Nome = 'btnQuadrado' then
    Result := btnQuadrado;
  if Nome = 'btnRaiz' then
    Result := btnRaiz;
  if Nome = 'btnDividir' then
    Result := btnDividir;
  if Nome = 'btn7' then
    Result := btn7;
  if Nome = 'btn8' then
    Result := btn8;
  if Nome = 'btn9' then
    Result := btn9;
  if Nome = 'btnMultiplicar' then
    Result := btnMultiplicar;
  if Nome = 'btn4' then
    Result := btn4;
  if Nome = 'btn5' then
    Result := btn5;
  if Nome = 'btn6' then
    Result := btn6;
  if Nome = 'btnSubtrair' then
    Result := btnSubtrair;
  if Nome = 'btn1' then
    Result := btn1;
  if Nome = 'btn2' then
    Result := btn2;
  if Nome = 'btn3' then
    Result := btn3;
  if Nome = 'btnSomar' then
    Result := btnSomar;
  if Nome = 'btnSinal' then
    Result := btnSinal;
  if Nome = 'btn0' then
    Result := btn0;
  if Nome = 'btnVirgula' then
    Result := btnVirgula;
  if Nome = 'btnIgual' then
    Result := btnIgual;
end;

function TFrmCalculadoraBase.SenderAsButton(Sender: TAylisComponent): TButton;
begin
  Result := nil;
  if Sender <> nil then
    Result := FindButton(Sender.Name);
end;

function TFrmCalculadoraBase.SenderAsButton(Event: TJSONObject): TButton;
begin
  Result := FindButton(TJSONObject(Event.Find('sender')).Get('name', ''));
end;

function TFrmCalculadoraBase.FindGridLayout(const Nome: string): TGridLayout;
begin
  Result := nil;
  if Nome = 'grdMemoria' then
    Result := grdMemoria;
  if Nome = 'grdTeclado' then
    Result := grdTeclado;
end;

function TFrmCalculadoraBase.SenderAsGridLayout(Sender: TAylisComponent): TGridLayout;
begin
  Result := nil;
  if Sender <> nil then
    Result := FindGridLayout(Sender.Name);
end;

function TFrmCalculadoraBase.SenderAsGridLayout(Event: TJSONObject): TGridLayout;
begin
  Result := FindGridLayout(TJSONObject(Event.Find('sender')).Get('name', ''));
end;

function TFrmCalculadoraBase.FindLabel(const Nome: string): TLabel;
begin
  Result := nil;
  if Nome = 'lblTitulo' then
    Result := lblTitulo;
  if Nome = 'lblExpressao' then
    Result := lblExpressao;
  if Nome = 'lblVisor' then
    Result := lblVisor;
end;

function TFrmCalculadoraBase.SenderAsLabel(Sender: TAylisComponent): TLabel;
begin
  Result := nil;
  if Sender <> nil then
    Result := FindLabel(Sender.Name);
end;

function TFrmCalculadoraBase.SenderAsLabel(Event: TJSONObject): TLabel;
begin
  Result := FindLabel(TJSONObject(Event.Find('sender')).Get('name', ''));
end;

function TFrmCalculadoraBase.FindLayout(const Nome: string): TLayout;
begin
  Result := nil;
  if Nome = 'lytCorpo' then
    Result := lytCorpo;
  if Nome = 'lytTopo' then
    Result := lytTopo;
end;

function TFrmCalculadoraBase.SenderAsLayout(Sender: TAylisComponent): TLayout;
begin
  Result := nil;
  if Sender <> nil then
    Result := FindLayout(Sender.Name);
end;

function TFrmCalculadoraBase.SenderAsLayout(Event: TJSONObject): TLayout;
begin
  Result := FindLayout(TJSONObject(Event.Find('sender')).Get('name', ''));
end;

function TFrmCalculadoraBase.FindListView(const Nome: string): TListView;
begin
  Result := nil;
  if Nome = 'lvHistorico' then
    Result := lvHistorico;
  if Nome = 'lvMemoria' then
    Result := lvMemoria;
end;

function TFrmCalculadoraBase.SenderAsListView(Sender: TAylisComponent): TListView;
begin
  Result := nil;
  if Sender <> nil then
    Result := FindListView(Sender.Name);
end;

function TFrmCalculadoraBase.SenderAsListView(Event: TJSONObject): TListView;
begin
  Result := FindListView(TJSONObject(Event.Find('sender')).Get('name', ''));
end;

function TFrmCalculadoraBase.FindPanel(const Nome: string): TPanel;
begin
  Result := nil;
  if Nome = 'pnlLateral' then
    Result := pnlLateral;
end;

function TFrmCalculadoraBase.SenderAsPanel(Sender: TAylisComponent): TPanel;
begin
  Result := nil;
  if Sender <> nil then
    Result := FindPanel(Sender.Name);
end;

function TFrmCalculadoraBase.SenderAsPanel(Event: TJSONObject): TPanel;
begin
  Result := FindPanel(TJSONObject(Event.Find('sender')).Get('name', ''));
end;

function TFrmCalculadoraBase.FindTabControl(const Nome: string): TTabControl;
begin
  Result := nil;
  if Nome = 'tabLateral' then
    Result := tabLateral;
end;

function TFrmCalculadoraBase.SenderAsTabControl(Sender: TAylisComponent): TTabControl;
begin
  Result := nil;
  if Sender <> nil then
    Result := FindTabControl(Sender.Name);
end;

function TFrmCalculadoraBase.SenderAsTabControl(Event: TJSONObject): TTabControl;
begin
  Result := FindTabControl(TJSONObject(Event.Find('sender')).Get('name', ''));
end;

function TFrmCalculadoraBase.FindTabItem(const Nome: string): TTabItem;
begin
  Result := nil;
  if Nome = 'tabHistorico' then
    Result := tabHistorico;
  if Nome = 'tabMemoria' then
    Result := tabMemoria;
end;

function TFrmCalculadoraBase.SenderAsTabItem(Sender: TAylisComponent): TTabItem;
begin
  Result := nil;
  if Sender <> nil then
    Result := FindTabItem(Sender.Name);
end;

function TFrmCalculadoraBase.SenderAsTabItem(Event: TJSONObject): TTabItem;
begin
  Result := FindTabItem(TJSONObject(Event.Find('sender')).Get('name', ''));
end;

initialization
  AylisFormat := DefaultFormatSettings;
  AylisFormat.DecimalSeparator := '.';
end.
