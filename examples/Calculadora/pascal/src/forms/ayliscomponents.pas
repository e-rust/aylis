{ Os componentes dos forms com as propriedades tipadas e a classe base de cada form (com os
  componentes como campos published, montados quando a tela abre o form). A classe de cada
  form, com os handlers e o estado, fica na unit dele.
  Gerado pela IDE do Aylis a cada "Gerar código": não edite. }
unit ayliscomponents;

{$mode objfpc}{$H+}

interface

uses
  fpjson, aylis;

type
  { Um Button de um form. }
  TButton = class(TAylisComponent)
  private
    function GetText: string;
    procedure SetText(Value: string);
    function GetIcon: string;
    procedure SetIcon(Value: string);
    function GetCheckedIcon: string;
    procedure SetCheckedIcon(Value: string);
    function GetIconPosition: string;
    procedure SetIconPosition(Value: string);
    function GetIconSize: Double;
    procedure SetIconSize(Value: Double);
    function GetCheckable: Boolean;
    procedure SetCheckable(Value: Boolean);
    function GetChecked: Boolean;
    procedure SetChecked(Value: Boolean);
    function GetModalResult: string;
    procedure SetModalResult(Value: string);
    function GetRepeatEnter: Boolean;
    procedure SetRepeatEnter(Value: Boolean);
    function GetLeft: Double;
    procedure SetLeft(Value: Double);
    function GetTop: Double;
    procedure SetTop(Value: Double);
    function GetWidth: Double;
    procedure SetWidth(Value: Double);
    function GetHeight: Double;
    procedure SetHeight(Value: Double);
    function GetAlign: string;
    procedure SetAlign(Value: string);
    function GetVisible: Boolean;
    procedure SetVisible(Value: Boolean);
    function GetEnabled: Boolean;
    procedure SetEnabled(Value: Boolean);
    function GetHitTest: Boolean;
    procedure SetHitTest(Value: Boolean);
    function GetClipChildren: Boolean;
    procedure SetClipChildren(Value: Boolean);
    function GetCapturesWheel: Boolean;
    procedure SetCapturesWheel(Value: Boolean);
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
    function GetDragImage: string;
    procedure SetDragImage(Value: string);
    function GetDropTypes: string;
    procedure SetDropTypes(Value: string);
    function GetTabStop: Boolean;
    procedure SetTabStop(Value: Boolean);
    function GetCursor: string;
    procedure SetCursor(Value: string);
    function GetOpacity: Double;
    procedure SetOpacity(Value: Double);
    function GetCacheLayer: Boolean;
    procedure SetCacheLayer(Value: Boolean);
    function GetRotation: Double;
    procedure SetRotation(Value: Double);
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
    function GetSelfAlign: string;
    procedure SetSelfAlign(Value: string);
    function GetAutoSize: Boolean;
    procedure SetAutoSize(Value: Boolean);
    function GetColumnSpan: Int64;
    procedure SetColumnSpan(Value: Int64);
    function GetShadow: string;
    procedure SetShadow(Value: string);
    function GetStyle: string;
    procedure SetStyle(Value: string);
    function GetTheme: string;
    procedure SetTheme(Value: string);
    function GetFill: string;
    procedure SetFill(Value: string);
    function GetBorderColor: string;
    procedure SetBorderColor(Value: string);
    function GetTextColor: string;
    procedure SetTextColor(Value: string);
    function GetAccentColor: string;
    procedure SetAccentColor(Value: string);
    function GetSuccessColor: string;
    procedure SetSuccessColor(Value: string);
    function GetWarningColor: string;
    procedure SetWarningColor(Value: string);
    function GetDangerColor: string;
    procedure SetDangerColor(Value: string);
    function GetFontFamily: string;
    procedure SetFontFamily(Value: string);
    function GetFontSize: Double;
    procedure SetFontSize(Value: Double);
    function GetFontWeight: string;
    procedure SetFontWeight(Value: string);
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
    function GetPopupMenu: string;
    procedure SetPopupMenu(Value: string);
  public
    procedure SetFocus;
    property Text: string read GetText write SetText;
    property Icon: string read GetIcon write SetIcon;
    property CheckedIcon: string read GetCheckedIcon write SetCheckedIcon;
    property IconPosition: string read GetIconPosition write SetIconPosition;
    property IconSize: Double read GetIconSize write SetIconSize;
    property Checkable: Boolean read GetCheckable write SetCheckable;
    property Checked: Boolean read GetChecked write SetChecked;
    property ModalResult: string read GetModalResult write SetModalResult;
    property RepeatEnter: Boolean read GetRepeatEnter write SetRepeatEnter;
    property Left: Double read GetLeft write SetLeft;
    property Top: Double read GetTop write SetTop;
    property Width: Double read GetWidth write SetWidth;
    property Height: Double read GetHeight write SetHeight;
    property Align: string read GetAlign write SetAlign;
    property Visible: Boolean read GetVisible write SetVisible;
    property Enabled: Boolean read GetEnabled write SetEnabled;
    property HitTest: Boolean read GetHitTest write SetHitTest;
    property ClipChildren: Boolean read GetClipChildren write SetClipChildren;
    property CapturesWheel: Boolean read GetCapturesWheel write SetCapturesWheel;
    property CanFocus: Boolean read GetCanFocus write SetCanFocus;
    property Draggable: Boolean read GetDraggable write SetDraggable;
    property AcceptsDrops: Boolean read GetAcceptsDrops write SetAcceptsDrops;
    property DragData: string read GetDragData write SetDragData;
    property DragLabel: string read GetDragLabel write SetDragLabel;
    property DragImage: string read GetDragImage write SetDragImage;
    property DropTypes: string read GetDropTypes write SetDropTypes;
    property TabStop: Boolean read GetTabStop write SetTabStop;
    property Cursor: string read GetCursor write SetCursor;
    property Opacity: Double read GetOpacity write SetOpacity;
    property CacheLayer: Boolean read GetCacheLayer write SetCacheLayer;
    property Rotation: Double read GetRotation write SetRotation;
    property TranslateX: Double read GetTranslateX write SetTranslateX;
    property TranslateY: Double read GetTranslateY write SetTranslateY;
    property AspectRatio: Double read GetAspectRatio write SetAspectRatio;
    property MaxWidth: Double read GetMaxWidth write SetMaxWidth;
    property MinWidth: Double read GetMinWidth write SetMinWidth;
    property MinHeight: Double read GetMinHeight write SetMinHeight;
    property MaxHeight: Double read GetMaxHeight write SetMaxHeight;
    property SelfAlign: string read GetSelfAlign write SetSelfAlign;
    property AutoSize: Boolean read GetAutoSize write SetAutoSize;
    property ColumnSpan: Int64 read GetColumnSpan write SetColumnSpan;
    property Shadow: string read GetShadow write SetShadow;
    property Style: string read GetStyle write SetStyle;
    property Theme: string read GetTheme write SetTheme;
    property Fill: string read GetFill write SetFill;
    property BorderColor: string read GetBorderColor write SetBorderColor;
    property TextColor: string read GetTextColor write SetTextColor;
    property AccentColor: string read GetAccentColor write SetAccentColor;
    property SuccessColor: string read GetSuccessColor write SetSuccessColor;
    property WarningColor: string read GetWarningColor write SetWarningColor;
    property DangerColor: string read GetDangerColor write SetDangerColor;
    property FontFamily: string read GetFontFamily write SetFontFamily;
    property FontSize: Double read GetFontSize write SetFontSize;
    property FontWeight: string read GetFontWeight write SetFontWeight;
    property FontItalic: Boolean read GetFontItalic write SetFontItalic;
    property FontUnderline: Boolean read GetFontUnderline write SetFontUnderline;
    property FontStrikeout: Boolean read GetFontStrikeout write SetFontStrikeout;
    property Hint: string read GetHint write SetHint;
    property Tag: Int64 read GetTag write SetTag;
    property PopupMenu: string read GetPopupMenu write SetPopupMenu;
  end;

  { Um Form de um form. }
  TForm = class(TAylisForm)
  private
    function GetCaption: string;
    procedure SetCaption(Value: string);
    function GetPosition: string;
    procedure SetPosition(Value: string);
    function GetBorderStyle: string;
    procedure SetBorderStyle(Value: string);
    function GetWindowState: string;
    procedure SetWindowState(Value: string);
    function GetTransition: string;
    procedure SetTransition(Value: string);
    function GetTopmost: Boolean;
    procedure SetTopmost(Value: Boolean);
    function GetShowInTaskbar: Boolean;
    procedure SetShowInTaskbar(Value: Boolean);
    function GetActiveControl: string;
    procedure SetActiveControl(Value: string);
    function GetTabOrder: string;
    procedure SetTabOrder(Value: string);
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
    function GetLeft: Double;
    procedure SetLeft(Value: Double);
    function GetTop: Double;
    procedure SetTop(Value: Double);
    function GetWidth: Double;
    procedure SetWidth(Value: Double);
    function GetHeight: Double;
    procedure SetHeight(Value: Double);
    function GetAlign: string;
    procedure SetAlign(Value: string);
    function GetVisible: Boolean;
    procedure SetVisible(Value: Boolean);
    function GetEnabled: Boolean;
    procedure SetEnabled(Value: Boolean);
    function GetHitTest: Boolean;
    procedure SetHitTest(Value: Boolean);
    function GetClipChildren: Boolean;
    procedure SetClipChildren(Value: Boolean);
    function GetCapturesWheel: Boolean;
    procedure SetCapturesWheel(Value: Boolean);
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
    function GetDragImage: string;
    procedure SetDragImage(Value: string);
    function GetDropTypes: string;
    procedure SetDropTypes(Value: string);
    function GetTabStop: Boolean;
    procedure SetTabStop(Value: Boolean);
    function GetCursor: string;
    procedure SetCursor(Value: string);
    function GetOpacity: Double;
    procedure SetOpacity(Value: Double);
    function GetCacheLayer: Boolean;
    procedure SetCacheLayer(Value: Boolean);
    function GetRotation: Double;
    procedure SetRotation(Value: Double);
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
    function GetSelfAlign: string;
    procedure SetSelfAlign(Value: string);
    function GetAutoSize: Boolean;
    procedure SetAutoSize(Value: Boolean);
    function GetColumnSpan: Int64;
    procedure SetColumnSpan(Value: Int64);
    function GetShadow: string;
    procedure SetShadow(Value: string);
    function GetStyle: string;
    procedure SetStyle(Value: string);
    function GetTheme: string;
    procedure SetTheme(Value: string);
    function GetFill: string;
    procedure SetFill(Value: string);
    function GetBorderColor: string;
    procedure SetBorderColor(Value: string);
    function GetTextColor: string;
    procedure SetTextColor(Value: string);
    function GetAccentColor: string;
    procedure SetAccentColor(Value: string);
    function GetSuccessColor: string;
    procedure SetSuccessColor(Value: string);
    function GetWarningColor: string;
    procedure SetWarningColor(Value: string);
    function GetDangerColor: string;
    procedure SetDangerColor(Value: string);
    function GetFontFamily: string;
    procedure SetFontFamily(Value: string);
    function GetFontSize: Double;
    procedure SetFontSize(Value: Double);
    function GetFontWeight: string;
    procedure SetFontWeight(Value: string);
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
    function GetPopupMenu: string;
    procedure SetPopupMenu(Value: string);
  public
    procedure MoveToMonitor(A1: Int64);
    function Focused: string;
    procedure FocusNext;
    procedure FocusPrev;
    property Caption: string read GetCaption write SetCaption;
    property Position: string read GetPosition write SetPosition;
    property BorderStyle: string read GetBorderStyle write SetBorderStyle;
    property WindowState: string read GetWindowState write SetWindowState;
    property Transition: string read GetTransition write SetTransition;
    property Topmost: Boolean read GetTopmost write SetTopmost;
    property ShowInTaskbar: Boolean read GetShowInTaskbar write SetShowInTaskbar;
    property ActiveControl: string read GetActiveControl write SetActiveControl;
    property TabOrder: string read GetTabOrder write SetTabOrder;
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
    property Left: Double read GetLeft write SetLeft;
    property Top: Double read GetTop write SetTop;
    property Width: Double read GetWidth write SetWidth;
    property Height: Double read GetHeight write SetHeight;
    property Align: string read GetAlign write SetAlign;
    property Visible: Boolean read GetVisible write SetVisible;
    property Enabled: Boolean read GetEnabled write SetEnabled;
    property HitTest: Boolean read GetHitTest write SetHitTest;
    property ClipChildren: Boolean read GetClipChildren write SetClipChildren;
    property CapturesWheel: Boolean read GetCapturesWheel write SetCapturesWheel;
    property CanFocus: Boolean read GetCanFocus write SetCanFocus;
    property Draggable: Boolean read GetDraggable write SetDraggable;
    property AcceptsDrops: Boolean read GetAcceptsDrops write SetAcceptsDrops;
    property DragData: string read GetDragData write SetDragData;
    property DragLabel: string read GetDragLabel write SetDragLabel;
    property DragImage: string read GetDragImage write SetDragImage;
    property DropTypes: string read GetDropTypes write SetDropTypes;
    property TabStop: Boolean read GetTabStop write SetTabStop;
    property Cursor: string read GetCursor write SetCursor;
    property Opacity: Double read GetOpacity write SetOpacity;
    property CacheLayer: Boolean read GetCacheLayer write SetCacheLayer;
    property Rotation: Double read GetRotation write SetRotation;
    property TranslateX: Double read GetTranslateX write SetTranslateX;
    property TranslateY: Double read GetTranslateY write SetTranslateY;
    property AspectRatio: Double read GetAspectRatio write SetAspectRatio;
    property MaxWidth: Double read GetMaxWidth write SetMaxWidth;
    property MinWidth: Double read GetMinWidth write SetMinWidth;
    property MinHeight: Double read GetMinHeight write SetMinHeight;
    property MaxHeight: Double read GetMaxHeight write SetMaxHeight;
    property SelfAlign: string read GetSelfAlign write SetSelfAlign;
    property AutoSize: Boolean read GetAutoSize write SetAutoSize;
    property ColumnSpan: Int64 read GetColumnSpan write SetColumnSpan;
    property Shadow: string read GetShadow write SetShadow;
    property Style: string read GetStyle write SetStyle;
    property Theme: string read GetTheme write SetTheme;
    property Fill: string read GetFill write SetFill;
    property BorderColor: string read GetBorderColor write SetBorderColor;
    property TextColor: string read GetTextColor write SetTextColor;
    property AccentColor: string read GetAccentColor write SetAccentColor;
    property SuccessColor: string read GetSuccessColor write SetSuccessColor;
    property WarningColor: string read GetWarningColor write SetWarningColor;
    property DangerColor: string read GetDangerColor write SetDangerColor;
    property FontFamily: string read GetFontFamily write SetFontFamily;
    property FontSize: Double read GetFontSize write SetFontSize;
    property FontWeight: string read GetFontWeight write SetFontWeight;
    property FontItalic: Boolean read GetFontItalic write SetFontItalic;
    property FontUnderline: Boolean read GetFontUnderline write SetFontUnderline;
    property FontStrikeout: Boolean read GetFontStrikeout write SetFontStrikeout;
    property Hint: string read GetHint write SetHint;
    property Tag: Int64 read GetTag write SetTag;
    property PopupMenu: string read GetPopupMenu write SetPopupMenu;
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
    function GetAlign: string;
    procedure SetAlign(Value: string);
    function GetVisible: Boolean;
    procedure SetVisible(Value: Boolean);
    function GetEnabled: Boolean;
    procedure SetEnabled(Value: Boolean);
    function GetHitTest: Boolean;
    procedure SetHitTest(Value: Boolean);
    function GetClipChildren: Boolean;
    procedure SetClipChildren(Value: Boolean);
    function GetCapturesWheel: Boolean;
    procedure SetCapturesWheel(Value: Boolean);
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
    function GetDragImage: string;
    procedure SetDragImage(Value: string);
    function GetDropTypes: string;
    procedure SetDropTypes(Value: string);
    function GetTabStop: Boolean;
    procedure SetTabStop(Value: Boolean);
    function GetCursor: string;
    procedure SetCursor(Value: string);
    function GetOpacity: Double;
    procedure SetOpacity(Value: Double);
    function GetCacheLayer: Boolean;
    procedure SetCacheLayer(Value: Boolean);
    function GetRotation: Double;
    procedure SetRotation(Value: Double);
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
    function GetSelfAlign: string;
    procedure SetSelfAlign(Value: string);
    function GetAutoSize: Boolean;
    procedure SetAutoSize(Value: Boolean);
    function GetColumnSpan: Int64;
    procedure SetColumnSpan(Value: Int64);
    function GetShadow: string;
    procedure SetShadow(Value: string);
    function GetStyle: string;
    procedure SetStyle(Value: string);
    function GetTheme: string;
    procedure SetTheme(Value: string);
    function GetFill: string;
    procedure SetFill(Value: string);
    function GetBorderColor: string;
    procedure SetBorderColor(Value: string);
    function GetTextColor: string;
    procedure SetTextColor(Value: string);
    function GetAccentColor: string;
    procedure SetAccentColor(Value: string);
    function GetSuccessColor: string;
    procedure SetSuccessColor(Value: string);
    function GetWarningColor: string;
    procedure SetWarningColor(Value: string);
    function GetDangerColor: string;
    procedure SetDangerColor(Value: string);
    function GetFontFamily: string;
    procedure SetFontFamily(Value: string);
    function GetFontSize: Double;
    procedure SetFontSize(Value: Double);
    function GetFontWeight: string;
    procedure SetFontWeight(Value: string);
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
    function GetPopupMenu: string;
    procedure SetPopupMenu(Value: string);
  public
    property Columns: Int64 read GetColumns write SetColumns;
    property MinColumnWidth: Double read GetMinColumnWidth write SetMinColumnWidth;
    property Gap: Double read GetGap write SetGap;
    property RowHeight: Double read GetRowHeight write SetRowHeight;
    property Left: Double read GetLeft write SetLeft;
    property Top: Double read GetTop write SetTop;
    property Width: Double read GetWidth write SetWidth;
    property Height: Double read GetHeight write SetHeight;
    property Align: string read GetAlign write SetAlign;
    property Visible: Boolean read GetVisible write SetVisible;
    property Enabled: Boolean read GetEnabled write SetEnabled;
    property HitTest: Boolean read GetHitTest write SetHitTest;
    property ClipChildren: Boolean read GetClipChildren write SetClipChildren;
    property CapturesWheel: Boolean read GetCapturesWheel write SetCapturesWheel;
    property CanFocus: Boolean read GetCanFocus write SetCanFocus;
    property Draggable: Boolean read GetDraggable write SetDraggable;
    property AcceptsDrops: Boolean read GetAcceptsDrops write SetAcceptsDrops;
    property DragData: string read GetDragData write SetDragData;
    property DragLabel: string read GetDragLabel write SetDragLabel;
    property DragImage: string read GetDragImage write SetDragImage;
    property DropTypes: string read GetDropTypes write SetDropTypes;
    property TabStop: Boolean read GetTabStop write SetTabStop;
    property Cursor: string read GetCursor write SetCursor;
    property Opacity: Double read GetOpacity write SetOpacity;
    property CacheLayer: Boolean read GetCacheLayer write SetCacheLayer;
    property Rotation: Double read GetRotation write SetRotation;
    property TranslateX: Double read GetTranslateX write SetTranslateX;
    property TranslateY: Double read GetTranslateY write SetTranslateY;
    property AspectRatio: Double read GetAspectRatio write SetAspectRatio;
    property MaxWidth: Double read GetMaxWidth write SetMaxWidth;
    property MinWidth: Double read GetMinWidth write SetMinWidth;
    property MinHeight: Double read GetMinHeight write SetMinHeight;
    property MaxHeight: Double read GetMaxHeight write SetMaxHeight;
    property SelfAlign: string read GetSelfAlign write SetSelfAlign;
    property AutoSize: Boolean read GetAutoSize write SetAutoSize;
    property ColumnSpan: Int64 read GetColumnSpan write SetColumnSpan;
    property Shadow: string read GetShadow write SetShadow;
    property Style: string read GetStyle write SetStyle;
    property Theme: string read GetTheme write SetTheme;
    property Fill: string read GetFill write SetFill;
    property BorderColor: string read GetBorderColor write SetBorderColor;
    property TextColor: string read GetTextColor write SetTextColor;
    property AccentColor: string read GetAccentColor write SetAccentColor;
    property SuccessColor: string read GetSuccessColor write SetSuccessColor;
    property WarningColor: string read GetWarningColor write SetWarningColor;
    property DangerColor: string read GetDangerColor write SetDangerColor;
    property FontFamily: string read GetFontFamily write SetFontFamily;
    property FontSize: Double read GetFontSize write SetFontSize;
    property FontWeight: string read GetFontWeight write SetFontWeight;
    property FontItalic: Boolean read GetFontItalic write SetFontItalic;
    property FontUnderline: Boolean read GetFontUnderline write SetFontUnderline;
    property FontStrikeout: Boolean read GetFontStrikeout write SetFontStrikeout;
    property Hint: string read GetHint write SetHint;
    property Tag: Int64 read GetTag write SetTag;
    property PopupMenu: string read GetPopupMenu write SetPopupMenu;
  end;

  { Um Label de um form. }
  TLabel = class(TAylisComponent)
  private
    function GetText: string;
    procedure SetText(Value: string);
    function GetHAlign: string;
    procedure SetHAlign(Value: string);
    function GetVAlign: string;
    procedure SetVAlign(Value: string);
    function GetWrap: Boolean;
    procedure SetWrap(Value: Boolean);
    function GetTrimming: string;
    procedure SetTrimming(Value: string);
    function GetBold: Boolean;
    procedure SetBold(Value: Boolean);
    function GetColor: string;
    procedure SetColor(Value: string);
    function GetHighlight: string;
    procedure SetHighlight(Value: string);
    function GetLetterSpacing: Double;
    procedure SetLetterSpacing(Value: Double);
    function GetTextCase: string;
    procedure SetTextCase(Value: string);
    function GetMarkup: Boolean;
    procedure SetMarkup(Value: Boolean);
    function GetOutline: string;
    procedure SetOutline(Value: string);
    function GetOutlineWidth: Double;
    procedure SetOutlineWidth(Value: Double);
    function GetFocusControl: string;
    procedure SetFocusControl(Value: string);
    function GetLeft: Double;
    procedure SetLeft(Value: Double);
    function GetTop: Double;
    procedure SetTop(Value: Double);
    function GetWidth: Double;
    procedure SetWidth(Value: Double);
    function GetHeight: Double;
    procedure SetHeight(Value: Double);
    function GetAlign: string;
    procedure SetAlign(Value: string);
    function GetVisible: Boolean;
    procedure SetVisible(Value: Boolean);
    function GetEnabled: Boolean;
    procedure SetEnabled(Value: Boolean);
    function GetHitTest: Boolean;
    procedure SetHitTest(Value: Boolean);
    function GetClipChildren: Boolean;
    procedure SetClipChildren(Value: Boolean);
    function GetCapturesWheel: Boolean;
    procedure SetCapturesWheel(Value: Boolean);
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
    function GetDragImage: string;
    procedure SetDragImage(Value: string);
    function GetDropTypes: string;
    procedure SetDropTypes(Value: string);
    function GetTabStop: Boolean;
    procedure SetTabStop(Value: Boolean);
    function GetCursor: string;
    procedure SetCursor(Value: string);
    function GetOpacity: Double;
    procedure SetOpacity(Value: Double);
    function GetCacheLayer: Boolean;
    procedure SetCacheLayer(Value: Boolean);
    function GetRotation: Double;
    procedure SetRotation(Value: Double);
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
    function GetSelfAlign: string;
    procedure SetSelfAlign(Value: string);
    function GetAutoSize: Boolean;
    procedure SetAutoSize(Value: Boolean);
    function GetColumnSpan: Int64;
    procedure SetColumnSpan(Value: Int64);
    function GetShadow: string;
    procedure SetShadow(Value: string);
    function GetStyle: string;
    procedure SetStyle(Value: string);
    function GetTheme: string;
    procedure SetTheme(Value: string);
    function GetFill: string;
    procedure SetFill(Value: string);
    function GetBorderColor: string;
    procedure SetBorderColor(Value: string);
    function GetTextColor: string;
    procedure SetTextColor(Value: string);
    function GetAccentColor: string;
    procedure SetAccentColor(Value: string);
    function GetSuccessColor: string;
    procedure SetSuccessColor(Value: string);
    function GetWarningColor: string;
    procedure SetWarningColor(Value: string);
    function GetDangerColor: string;
    procedure SetDangerColor(Value: string);
    function GetFontFamily: string;
    procedure SetFontFamily(Value: string);
    function GetFontSize: Double;
    procedure SetFontSize(Value: Double);
    function GetFontWeight: string;
    procedure SetFontWeight(Value: string);
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
    function GetPopupMenu: string;
    procedure SetPopupMenu(Value: string);
  public
    property Text: string read GetText write SetText;
    property HAlign: string read GetHAlign write SetHAlign;
    property VAlign: string read GetVAlign write SetVAlign;
    property Wrap: Boolean read GetWrap write SetWrap;
    property Trimming: string read GetTrimming write SetTrimming;
    property Bold: Boolean read GetBold write SetBold;
    property Color: string read GetColor write SetColor;
    property Highlight: string read GetHighlight write SetHighlight;
    property LetterSpacing: Double read GetLetterSpacing write SetLetterSpacing;
    property TextCase: string read GetTextCase write SetTextCase;
    property Markup: Boolean read GetMarkup write SetMarkup;
    property Outline: string read GetOutline write SetOutline;
    property OutlineWidth: Double read GetOutlineWidth write SetOutlineWidth;
    property FocusControl: string read GetFocusControl write SetFocusControl;
    property Left: Double read GetLeft write SetLeft;
    property Top: Double read GetTop write SetTop;
    property Width: Double read GetWidth write SetWidth;
    property Height: Double read GetHeight write SetHeight;
    property Align: string read GetAlign write SetAlign;
    property Visible: Boolean read GetVisible write SetVisible;
    property Enabled: Boolean read GetEnabled write SetEnabled;
    property HitTest: Boolean read GetHitTest write SetHitTest;
    property ClipChildren: Boolean read GetClipChildren write SetClipChildren;
    property CapturesWheel: Boolean read GetCapturesWheel write SetCapturesWheel;
    property CanFocus: Boolean read GetCanFocus write SetCanFocus;
    property Draggable: Boolean read GetDraggable write SetDraggable;
    property AcceptsDrops: Boolean read GetAcceptsDrops write SetAcceptsDrops;
    property DragData: string read GetDragData write SetDragData;
    property DragLabel: string read GetDragLabel write SetDragLabel;
    property DragImage: string read GetDragImage write SetDragImage;
    property DropTypes: string read GetDropTypes write SetDropTypes;
    property TabStop: Boolean read GetTabStop write SetTabStop;
    property Cursor: string read GetCursor write SetCursor;
    property Opacity: Double read GetOpacity write SetOpacity;
    property CacheLayer: Boolean read GetCacheLayer write SetCacheLayer;
    property Rotation: Double read GetRotation write SetRotation;
    property TranslateX: Double read GetTranslateX write SetTranslateX;
    property TranslateY: Double read GetTranslateY write SetTranslateY;
    property AspectRatio: Double read GetAspectRatio write SetAspectRatio;
    property MaxWidth: Double read GetMaxWidth write SetMaxWidth;
    property MinWidth: Double read GetMinWidth write SetMinWidth;
    property MinHeight: Double read GetMinHeight write SetMinHeight;
    property MaxHeight: Double read GetMaxHeight write SetMaxHeight;
    property SelfAlign: string read GetSelfAlign write SetSelfAlign;
    property AutoSize: Boolean read GetAutoSize write SetAutoSize;
    property ColumnSpan: Int64 read GetColumnSpan write SetColumnSpan;
    property Shadow: string read GetShadow write SetShadow;
    property Style: string read GetStyle write SetStyle;
    property Theme: string read GetTheme write SetTheme;
    property Fill: string read GetFill write SetFill;
    property BorderColor: string read GetBorderColor write SetBorderColor;
    property TextColor: string read GetTextColor write SetTextColor;
    property AccentColor: string read GetAccentColor write SetAccentColor;
    property SuccessColor: string read GetSuccessColor write SetSuccessColor;
    property WarningColor: string read GetWarningColor write SetWarningColor;
    property DangerColor: string read GetDangerColor write SetDangerColor;
    property FontFamily: string read GetFontFamily write SetFontFamily;
    property FontSize: Double read GetFontSize write SetFontSize;
    property FontWeight: string read GetFontWeight write SetFontWeight;
    property FontItalic: Boolean read GetFontItalic write SetFontItalic;
    property FontUnderline: Boolean read GetFontUnderline write SetFontUnderline;
    property FontStrikeout: Boolean read GetFontStrikeout write SetFontStrikeout;
    property Hint: string read GetHint write SetHint;
    property Tag: Int64 read GetTag write SetTag;
    property PopupMenu: string read GetPopupMenu write SetPopupMenu;
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
    function GetLeft: Double;
    procedure SetLeft(Value: Double);
    function GetTop: Double;
    procedure SetTop(Value: Double);
    function GetWidth: Double;
    procedure SetWidth(Value: Double);
    function GetHeight: Double;
    procedure SetHeight(Value: Double);
    function GetAlign: string;
    procedure SetAlign(Value: string);
    function GetVisible: Boolean;
    procedure SetVisible(Value: Boolean);
    function GetEnabled: Boolean;
    procedure SetEnabled(Value: Boolean);
    function GetHitTest: Boolean;
    procedure SetHitTest(Value: Boolean);
    function GetClipChildren: Boolean;
    procedure SetClipChildren(Value: Boolean);
    function GetCapturesWheel: Boolean;
    procedure SetCapturesWheel(Value: Boolean);
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
    function GetDragImage: string;
    procedure SetDragImage(Value: string);
    function GetDropTypes: string;
    procedure SetDropTypes(Value: string);
    function GetTabStop: Boolean;
    procedure SetTabStop(Value: Boolean);
    function GetCursor: string;
    procedure SetCursor(Value: string);
    function GetOpacity: Double;
    procedure SetOpacity(Value: Double);
    function GetCacheLayer: Boolean;
    procedure SetCacheLayer(Value: Boolean);
    function GetRotation: Double;
    procedure SetRotation(Value: Double);
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
    function GetSelfAlign: string;
    procedure SetSelfAlign(Value: string);
    function GetAutoSize: Boolean;
    procedure SetAutoSize(Value: Boolean);
    function GetColumnSpan: Int64;
    procedure SetColumnSpan(Value: Int64);
    function GetShadow: string;
    procedure SetShadow(Value: string);
    function GetStyle: string;
    procedure SetStyle(Value: string);
    function GetTheme: string;
    procedure SetTheme(Value: string);
    function GetFill: string;
    procedure SetFill(Value: string);
    function GetBorderColor: string;
    procedure SetBorderColor(Value: string);
    function GetTextColor: string;
    procedure SetTextColor(Value: string);
    function GetAccentColor: string;
    procedure SetAccentColor(Value: string);
    function GetSuccessColor: string;
    procedure SetSuccessColor(Value: string);
    function GetWarningColor: string;
    procedure SetWarningColor(Value: string);
    function GetDangerColor: string;
    procedure SetDangerColor(Value: string);
    function GetFontFamily: string;
    procedure SetFontFamily(Value: string);
    function GetFontSize: Double;
    procedure SetFontSize(Value: Double);
    function GetFontWeight: string;
    procedure SetFontWeight(Value: string);
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
    function GetPopupMenu: string;
    procedure SetPopupMenu(Value: string);
  public
    property AllowedChars: string read GetAllowedChars write SetAllowedChars;
    property BlockedChars: string read GetBlockedChars write SetBlockedChars;
    property OnlyAllowed: Boolean read GetOnlyAllowed write SetOnlyAllowed;
    property CharsOverride: Boolean read GetCharsOverride write SetCharsOverride;
    property CharMap: string read GetCharMap write SetCharMap;
    property Left: Double read GetLeft write SetLeft;
    property Top: Double read GetTop write SetTop;
    property Width: Double read GetWidth write SetWidth;
    property Height: Double read GetHeight write SetHeight;
    property Align: string read GetAlign write SetAlign;
    property Visible: Boolean read GetVisible write SetVisible;
    property Enabled: Boolean read GetEnabled write SetEnabled;
    property HitTest: Boolean read GetHitTest write SetHitTest;
    property ClipChildren: Boolean read GetClipChildren write SetClipChildren;
    property CapturesWheel: Boolean read GetCapturesWheel write SetCapturesWheel;
    property CanFocus: Boolean read GetCanFocus write SetCanFocus;
    property Draggable: Boolean read GetDraggable write SetDraggable;
    property AcceptsDrops: Boolean read GetAcceptsDrops write SetAcceptsDrops;
    property DragData: string read GetDragData write SetDragData;
    property DragLabel: string read GetDragLabel write SetDragLabel;
    property DragImage: string read GetDragImage write SetDragImage;
    property DropTypes: string read GetDropTypes write SetDropTypes;
    property TabStop: Boolean read GetTabStop write SetTabStop;
    property Cursor: string read GetCursor write SetCursor;
    property Opacity: Double read GetOpacity write SetOpacity;
    property CacheLayer: Boolean read GetCacheLayer write SetCacheLayer;
    property Rotation: Double read GetRotation write SetRotation;
    property TranslateX: Double read GetTranslateX write SetTranslateX;
    property TranslateY: Double read GetTranslateY write SetTranslateY;
    property AspectRatio: Double read GetAspectRatio write SetAspectRatio;
    property MaxWidth: Double read GetMaxWidth write SetMaxWidth;
    property MinWidth: Double read GetMinWidth write SetMinWidth;
    property MinHeight: Double read GetMinHeight write SetMinHeight;
    property MaxHeight: Double read GetMaxHeight write SetMaxHeight;
    property SelfAlign: string read GetSelfAlign write SetSelfAlign;
    property AutoSize: Boolean read GetAutoSize write SetAutoSize;
    property ColumnSpan: Int64 read GetColumnSpan write SetColumnSpan;
    property Shadow: string read GetShadow write SetShadow;
    property Style: string read GetStyle write SetStyle;
    property Theme: string read GetTheme write SetTheme;
    property Fill: string read GetFill write SetFill;
    property BorderColor: string read GetBorderColor write SetBorderColor;
    property TextColor: string read GetTextColor write SetTextColor;
    property AccentColor: string read GetAccentColor write SetAccentColor;
    property SuccessColor: string read GetSuccessColor write SetSuccessColor;
    property WarningColor: string read GetWarningColor write SetWarningColor;
    property DangerColor: string read GetDangerColor write SetDangerColor;
    property FontFamily: string read GetFontFamily write SetFontFamily;
    property FontSize: Double read GetFontSize write SetFontSize;
    property FontWeight: string read GetFontWeight write SetFontWeight;
    property FontItalic: Boolean read GetFontItalic write SetFontItalic;
    property FontUnderline: Boolean read GetFontUnderline write SetFontUnderline;
    property FontStrikeout: Boolean read GetFontStrikeout write SetFontStrikeout;
    property Hint: string read GetHint write SetHint;
    property Tag: Int64 read GetTag write SetTag;
    property PopupMenu: string read GetPopupMenu write SetPopupMenu;
  end;

  { Um ListView de um form. }
  TListView = class(TAylisComponent)
  private
    function GetItems: string;
    procedure SetItems(Value: string);
    function GetItemIndex: Int64;
    procedure SetItemIndex(Value: Int64);
    function GetItemHeight: Double;
    procedure SetItemHeight(Value: Double);
    function GetAccessory: Boolean;
    procedure SetAccessory(Value: Boolean);
    function GetLeft: Double;
    procedure SetLeft(Value: Double);
    function GetTop: Double;
    procedure SetTop(Value: Double);
    function GetWidth: Double;
    procedure SetWidth(Value: Double);
    function GetHeight: Double;
    procedure SetHeight(Value: Double);
    function GetAlign: string;
    procedure SetAlign(Value: string);
    function GetVisible: Boolean;
    procedure SetVisible(Value: Boolean);
    function GetEnabled: Boolean;
    procedure SetEnabled(Value: Boolean);
    function GetHitTest: Boolean;
    procedure SetHitTest(Value: Boolean);
    function GetClipChildren: Boolean;
    procedure SetClipChildren(Value: Boolean);
    function GetCapturesWheel: Boolean;
    procedure SetCapturesWheel(Value: Boolean);
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
    function GetDragImage: string;
    procedure SetDragImage(Value: string);
    function GetDropTypes: string;
    procedure SetDropTypes(Value: string);
    function GetTabStop: Boolean;
    procedure SetTabStop(Value: Boolean);
    function GetCursor: string;
    procedure SetCursor(Value: string);
    function GetOpacity: Double;
    procedure SetOpacity(Value: Double);
    function GetCacheLayer: Boolean;
    procedure SetCacheLayer(Value: Boolean);
    function GetRotation: Double;
    procedure SetRotation(Value: Double);
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
    function GetSelfAlign: string;
    procedure SetSelfAlign(Value: string);
    function GetAutoSize: Boolean;
    procedure SetAutoSize(Value: Boolean);
    function GetColumnSpan: Int64;
    procedure SetColumnSpan(Value: Int64);
    function GetShadow: string;
    procedure SetShadow(Value: string);
    function GetStyle: string;
    procedure SetStyle(Value: string);
    function GetTheme: string;
    procedure SetTheme(Value: string);
    function GetFill: string;
    procedure SetFill(Value: string);
    function GetBorderColor: string;
    procedure SetBorderColor(Value: string);
    function GetTextColor: string;
    procedure SetTextColor(Value: string);
    function GetAccentColor: string;
    procedure SetAccentColor(Value: string);
    function GetSuccessColor: string;
    procedure SetSuccessColor(Value: string);
    function GetWarningColor: string;
    procedure SetWarningColor(Value: string);
    function GetDangerColor: string;
    procedure SetDangerColor(Value: string);
    function GetFontFamily: string;
    procedure SetFontFamily(Value: string);
    function GetFontSize: Double;
    procedure SetFontSize(Value: Double);
    function GetFontWeight: string;
    procedure SetFontWeight(Value: string);
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
    function GetPopupMenu: string;
    procedure SetPopupMenu(Value: string);
  public
    procedure SetFocus;
    property Items: string read GetItems write SetItems;
    property ItemIndex: Int64 read GetItemIndex write SetItemIndex;
    property ItemHeight: Double read GetItemHeight write SetItemHeight;
    property Accessory: Boolean read GetAccessory write SetAccessory;
    property Left: Double read GetLeft write SetLeft;
    property Top: Double read GetTop write SetTop;
    property Width: Double read GetWidth write SetWidth;
    property Height: Double read GetHeight write SetHeight;
    property Align: string read GetAlign write SetAlign;
    property Visible: Boolean read GetVisible write SetVisible;
    property Enabled: Boolean read GetEnabled write SetEnabled;
    property HitTest: Boolean read GetHitTest write SetHitTest;
    property ClipChildren: Boolean read GetClipChildren write SetClipChildren;
    property CapturesWheel: Boolean read GetCapturesWheel write SetCapturesWheel;
    property CanFocus: Boolean read GetCanFocus write SetCanFocus;
    property Draggable: Boolean read GetDraggable write SetDraggable;
    property AcceptsDrops: Boolean read GetAcceptsDrops write SetAcceptsDrops;
    property DragData: string read GetDragData write SetDragData;
    property DragLabel: string read GetDragLabel write SetDragLabel;
    property DragImage: string read GetDragImage write SetDragImage;
    property DropTypes: string read GetDropTypes write SetDropTypes;
    property TabStop: Boolean read GetTabStop write SetTabStop;
    property Cursor: string read GetCursor write SetCursor;
    property Opacity: Double read GetOpacity write SetOpacity;
    property CacheLayer: Boolean read GetCacheLayer write SetCacheLayer;
    property Rotation: Double read GetRotation write SetRotation;
    property TranslateX: Double read GetTranslateX write SetTranslateX;
    property TranslateY: Double read GetTranslateY write SetTranslateY;
    property AspectRatio: Double read GetAspectRatio write SetAspectRatio;
    property MaxWidth: Double read GetMaxWidth write SetMaxWidth;
    property MinWidth: Double read GetMinWidth write SetMinWidth;
    property MinHeight: Double read GetMinHeight write SetMinHeight;
    property MaxHeight: Double read GetMaxHeight write SetMaxHeight;
    property SelfAlign: string read GetSelfAlign write SetSelfAlign;
    property AutoSize: Boolean read GetAutoSize write SetAutoSize;
    property ColumnSpan: Int64 read GetColumnSpan write SetColumnSpan;
    property Shadow: string read GetShadow write SetShadow;
    property Style: string read GetStyle write SetStyle;
    property Theme: string read GetTheme write SetTheme;
    property Fill: string read GetFill write SetFill;
    property BorderColor: string read GetBorderColor write SetBorderColor;
    property TextColor: string read GetTextColor write SetTextColor;
    property AccentColor: string read GetAccentColor write SetAccentColor;
    property SuccessColor: string read GetSuccessColor write SetSuccessColor;
    property WarningColor: string read GetWarningColor write SetWarningColor;
    property DangerColor: string read GetDangerColor write SetDangerColor;
    property FontFamily: string read GetFontFamily write SetFontFamily;
    property FontSize: Double read GetFontSize write SetFontSize;
    property FontWeight: string read GetFontWeight write SetFontWeight;
    property FontItalic: Boolean read GetFontItalic write SetFontItalic;
    property FontUnderline: Boolean read GetFontUnderline write SetFontUnderline;
    property FontStrikeout: Boolean read GetFontStrikeout write SetFontStrikeout;
    property Hint: string read GetHint write SetHint;
    property Tag: Int64 read GetTag write SetTag;
    property PopupMenu: string read GetPopupMenu write SetPopupMenu;
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
    function GetFill: string;
    procedure SetFill(Value: string);
    function GetFill2: string;
    procedure SetFill2(Value: string);
    function GetFillMid: string;
    procedure SetFillMid(Value: string);
    function GetFillMidAt: Double;
    procedure SetFillMidAt(Value: Double);
    function GetGradientAngle: Double;
    procedure SetGradientAngle(Value: Double);
    function GetGradientRadial: Boolean;
    procedure SetGradientRadial(Value: Boolean);
    function GetStroke: string;
    procedure SetStroke(Value: string);
    function GetStrokeWidth: Double;
    procedure SetStrokeWidth(Value: Double);
    function GetRadius: Double;
    procedure SetRadius(Value: Double);
    function GetLeft: Double;
    procedure SetLeft(Value: Double);
    function GetTop: Double;
    procedure SetTop(Value: Double);
    function GetWidth: Double;
    procedure SetWidth(Value: Double);
    function GetHeight: Double;
    procedure SetHeight(Value: Double);
    function GetAlign: string;
    procedure SetAlign(Value: string);
    function GetVisible: Boolean;
    procedure SetVisible(Value: Boolean);
    function GetEnabled: Boolean;
    procedure SetEnabled(Value: Boolean);
    function GetHitTest: Boolean;
    procedure SetHitTest(Value: Boolean);
    function GetClipChildren: Boolean;
    procedure SetClipChildren(Value: Boolean);
    function GetCapturesWheel: Boolean;
    procedure SetCapturesWheel(Value: Boolean);
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
    function GetDragImage: string;
    procedure SetDragImage(Value: string);
    function GetDropTypes: string;
    procedure SetDropTypes(Value: string);
    function GetTabStop: Boolean;
    procedure SetTabStop(Value: Boolean);
    function GetCursor: string;
    procedure SetCursor(Value: string);
    function GetOpacity: Double;
    procedure SetOpacity(Value: Double);
    function GetCacheLayer: Boolean;
    procedure SetCacheLayer(Value: Boolean);
    function GetRotation: Double;
    procedure SetRotation(Value: Double);
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
    function GetSelfAlign: string;
    procedure SetSelfAlign(Value: string);
    function GetAutoSize: Boolean;
    procedure SetAutoSize(Value: Boolean);
    function GetColumnSpan: Int64;
    procedure SetColumnSpan(Value: Int64);
    function GetShadow: string;
    procedure SetShadow(Value: string);
    function GetStyle: string;
    procedure SetStyle(Value: string);
    function GetTheme: string;
    procedure SetTheme(Value: string);
    function GetBorderColor: string;
    procedure SetBorderColor(Value: string);
    function GetTextColor: string;
    procedure SetTextColor(Value: string);
    function GetAccentColor: string;
    procedure SetAccentColor(Value: string);
    function GetSuccessColor: string;
    procedure SetSuccessColor(Value: string);
    function GetWarningColor: string;
    procedure SetWarningColor(Value: string);
    function GetDangerColor: string;
    procedure SetDangerColor(Value: string);
    function GetFontFamily: string;
    procedure SetFontFamily(Value: string);
    function GetFontSize: Double;
    procedure SetFontSize(Value: Double);
    function GetFontWeight: string;
    procedure SetFontWeight(Value: string);
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
    function GetPopupMenu: string;
    procedure SetPopupMenu(Value: string);
  public
    property AllowedChars: string read GetAllowedChars write SetAllowedChars;
    property BlockedChars: string read GetBlockedChars write SetBlockedChars;
    property OnlyAllowed: Boolean read GetOnlyAllowed write SetOnlyAllowed;
    property CharsOverride: Boolean read GetCharsOverride write SetCharsOverride;
    property CharMap: string read GetCharMap write SetCharMap;
    property Fill: string read GetFill write SetFill;
    property Fill2: string read GetFill2 write SetFill2;
    property FillMid: string read GetFillMid write SetFillMid;
    property FillMidAt: Double read GetFillMidAt write SetFillMidAt;
    property GradientAngle: Double read GetGradientAngle write SetGradientAngle;
    property GradientRadial: Boolean read GetGradientRadial write SetGradientRadial;
    property Stroke: string read GetStroke write SetStroke;
    property StrokeWidth: Double read GetStrokeWidth write SetStrokeWidth;
    property Radius: Double read GetRadius write SetRadius;
    property Left: Double read GetLeft write SetLeft;
    property Top: Double read GetTop write SetTop;
    property Width: Double read GetWidth write SetWidth;
    property Height: Double read GetHeight write SetHeight;
    property Align: string read GetAlign write SetAlign;
    property Visible: Boolean read GetVisible write SetVisible;
    property Enabled: Boolean read GetEnabled write SetEnabled;
    property HitTest: Boolean read GetHitTest write SetHitTest;
    property ClipChildren: Boolean read GetClipChildren write SetClipChildren;
    property CapturesWheel: Boolean read GetCapturesWheel write SetCapturesWheel;
    property CanFocus: Boolean read GetCanFocus write SetCanFocus;
    property Draggable: Boolean read GetDraggable write SetDraggable;
    property AcceptsDrops: Boolean read GetAcceptsDrops write SetAcceptsDrops;
    property DragData: string read GetDragData write SetDragData;
    property DragLabel: string read GetDragLabel write SetDragLabel;
    property DragImage: string read GetDragImage write SetDragImage;
    property DropTypes: string read GetDropTypes write SetDropTypes;
    property TabStop: Boolean read GetTabStop write SetTabStop;
    property Cursor: string read GetCursor write SetCursor;
    property Opacity: Double read GetOpacity write SetOpacity;
    property CacheLayer: Boolean read GetCacheLayer write SetCacheLayer;
    property Rotation: Double read GetRotation write SetRotation;
    property TranslateX: Double read GetTranslateX write SetTranslateX;
    property TranslateY: Double read GetTranslateY write SetTranslateY;
    property AspectRatio: Double read GetAspectRatio write SetAspectRatio;
    property MaxWidth: Double read GetMaxWidth write SetMaxWidth;
    property MinWidth: Double read GetMinWidth write SetMinWidth;
    property MinHeight: Double read GetMinHeight write SetMinHeight;
    property MaxHeight: Double read GetMaxHeight write SetMaxHeight;
    property SelfAlign: string read GetSelfAlign write SetSelfAlign;
    property AutoSize: Boolean read GetAutoSize write SetAutoSize;
    property ColumnSpan: Int64 read GetColumnSpan write SetColumnSpan;
    property Shadow: string read GetShadow write SetShadow;
    property Style: string read GetStyle write SetStyle;
    property Theme: string read GetTheme write SetTheme;
    property BorderColor: string read GetBorderColor write SetBorderColor;
    property TextColor: string read GetTextColor write SetTextColor;
    property AccentColor: string read GetAccentColor write SetAccentColor;
    property SuccessColor: string read GetSuccessColor write SetSuccessColor;
    property WarningColor: string read GetWarningColor write SetWarningColor;
    property DangerColor: string read GetDangerColor write SetDangerColor;
    property FontFamily: string read GetFontFamily write SetFontFamily;
    property FontSize: Double read GetFontSize write SetFontSize;
    property FontWeight: string read GetFontWeight write SetFontWeight;
    property FontItalic: Boolean read GetFontItalic write SetFontItalic;
    property FontUnderline: Boolean read GetFontUnderline write SetFontUnderline;
    property FontStrikeout: Boolean read GetFontStrikeout write SetFontStrikeout;
    property Hint: string read GetHint write SetHint;
    property Tag: Int64 read GetTag write SetTag;
    property PopupMenu: string read GetPopupMenu write SetPopupMenu;
  end;

  { Um TabControl de um form. }
  TTabControl = class(TAylisComponent)
  private
    function GetTabIndex: Int64;
    procedure SetTabIndex(Value: Int64);
    function GetShowTabs: Boolean;
    procedure SetShowTabs(Value: Boolean);
    function GetTransition: string;
    procedure SetTransition(Value: string);
    function GetLeft: Double;
    procedure SetLeft(Value: Double);
    function GetTop: Double;
    procedure SetTop(Value: Double);
    function GetWidth: Double;
    procedure SetWidth(Value: Double);
    function GetHeight: Double;
    procedure SetHeight(Value: Double);
    function GetAlign: string;
    procedure SetAlign(Value: string);
    function GetVisible: Boolean;
    procedure SetVisible(Value: Boolean);
    function GetEnabled: Boolean;
    procedure SetEnabled(Value: Boolean);
    function GetHitTest: Boolean;
    procedure SetHitTest(Value: Boolean);
    function GetClipChildren: Boolean;
    procedure SetClipChildren(Value: Boolean);
    function GetCapturesWheel: Boolean;
    procedure SetCapturesWheel(Value: Boolean);
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
    function GetDragImage: string;
    procedure SetDragImage(Value: string);
    function GetDropTypes: string;
    procedure SetDropTypes(Value: string);
    function GetTabStop: Boolean;
    procedure SetTabStop(Value: Boolean);
    function GetCursor: string;
    procedure SetCursor(Value: string);
    function GetOpacity: Double;
    procedure SetOpacity(Value: Double);
    function GetCacheLayer: Boolean;
    procedure SetCacheLayer(Value: Boolean);
    function GetRotation: Double;
    procedure SetRotation(Value: Double);
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
    function GetSelfAlign: string;
    procedure SetSelfAlign(Value: string);
    function GetAutoSize: Boolean;
    procedure SetAutoSize(Value: Boolean);
    function GetColumnSpan: Int64;
    procedure SetColumnSpan(Value: Int64);
    function GetShadow: string;
    procedure SetShadow(Value: string);
    function GetStyle: string;
    procedure SetStyle(Value: string);
    function GetTheme: string;
    procedure SetTheme(Value: string);
    function GetFill: string;
    procedure SetFill(Value: string);
    function GetBorderColor: string;
    procedure SetBorderColor(Value: string);
    function GetTextColor: string;
    procedure SetTextColor(Value: string);
    function GetAccentColor: string;
    procedure SetAccentColor(Value: string);
    function GetSuccessColor: string;
    procedure SetSuccessColor(Value: string);
    function GetWarningColor: string;
    procedure SetWarningColor(Value: string);
    function GetDangerColor: string;
    procedure SetDangerColor(Value: string);
    function GetFontFamily: string;
    procedure SetFontFamily(Value: string);
    function GetFontSize: Double;
    procedure SetFontSize(Value: Double);
    function GetFontWeight: string;
    procedure SetFontWeight(Value: string);
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
    function GetPopupMenu: string;
    procedure SetPopupMenu(Value: string);
  public
    procedure SetFocus;
    property TabIndex: Int64 read GetTabIndex write SetTabIndex;
    property ShowTabs: Boolean read GetShowTabs write SetShowTabs;
    property Transition: string read GetTransition write SetTransition;
    property Left: Double read GetLeft write SetLeft;
    property Top: Double read GetTop write SetTop;
    property Width: Double read GetWidth write SetWidth;
    property Height: Double read GetHeight write SetHeight;
    property Align: string read GetAlign write SetAlign;
    property Visible: Boolean read GetVisible write SetVisible;
    property Enabled: Boolean read GetEnabled write SetEnabled;
    property HitTest: Boolean read GetHitTest write SetHitTest;
    property ClipChildren: Boolean read GetClipChildren write SetClipChildren;
    property CapturesWheel: Boolean read GetCapturesWheel write SetCapturesWheel;
    property CanFocus: Boolean read GetCanFocus write SetCanFocus;
    property Draggable: Boolean read GetDraggable write SetDraggable;
    property AcceptsDrops: Boolean read GetAcceptsDrops write SetAcceptsDrops;
    property DragData: string read GetDragData write SetDragData;
    property DragLabel: string read GetDragLabel write SetDragLabel;
    property DragImage: string read GetDragImage write SetDragImage;
    property DropTypes: string read GetDropTypes write SetDropTypes;
    property TabStop: Boolean read GetTabStop write SetTabStop;
    property Cursor: string read GetCursor write SetCursor;
    property Opacity: Double read GetOpacity write SetOpacity;
    property CacheLayer: Boolean read GetCacheLayer write SetCacheLayer;
    property Rotation: Double read GetRotation write SetRotation;
    property TranslateX: Double read GetTranslateX write SetTranslateX;
    property TranslateY: Double read GetTranslateY write SetTranslateY;
    property AspectRatio: Double read GetAspectRatio write SetAspectRatio;
    property MaxWidth: Double read GetMaxWidth write SetMaxWidth;
    property MinWidth: Double read GetMinWidth write SetMinWidth;
    property MinHeight: Double read GetMinHeight write SetMinHeight;
    property MaxHeight: Double read GetMaxHeight write SetMaxHeight;
    property SelfAlign: string read GetSelfAlign write SetSelfAlign;
    property AutoSize: Boolean read GetAutoSize write SetAutoSize;
    property ColumnSpan: Int64 read GetColumnSpan write SetColumnSpan;
    property Shadow: string read GetShadow write SetShadow;
    property Style: string read GetStyle write SetStyle;
    property Theme: string read GetTheme write SetTheme;
    property Fill: string read GetFill write SetFill;
    property BorderColor: string read GetBorderColor write SetBorderColor;
    property TextColor: string read GetTextColor write SetTextColor;
    property AccentColor: string read GetAccentColor write SetAccentColor;
    property SuccessColor: string read GetSuccessColor write SetSuccessColor;
    property WarningColor: string read GetWarningColor write SetWarningColor;
    property DangerColor: string read GetDangerColor write SetDangerColor;
    property FontFamily: string read GetFontFamily write SetFontFamily;
    property FontSize: Double read GetFontSize write SetFontSize;
    property FontWeight: string read GetFontWeight write SetFontWeight;
    property FontItalic: Boolean read GetFontItalic write SetFontItalic;
    property FontUnderline: Boolean read GetFontUnderline write SetFontUnderline;
    property FontStrikeout: Boolean read GetFontStrikeout write SetFontStrikeout;
    property Hint: string read GetHint write SetHint;
    property Tag: Int64 read GetTag write SetTag;
    property PopupMenu: string read GetPopupMenu write SetPopupMenu;
  end;

  { Um TabItem de um form. }
  TTabItem = class(TAylisComponent)
  private
    function GetCaption: string;
    procedure SetCaption(Value: string);
    function GetLeft: Double;
    procedure SetLeft(Value: Double);
    function GetTop: Double;
    procedure SetTop(Value: Double);
    function GetWidth: Double;
    procedure SetWidth(Value: Double);
    function GetHeight: Double;
    procedure SetHeight(Value: Double);
    function GetAlign: string;
    procedure SetAlign(Value: string);
    function GetVisible: Boolean;
    procedure SetVisible(Value: Boolean);
    function GetEnabled: Boolean;
    procedure SetEnabled(Value: Boolean);
    function GetHitTest: Boolean;
    procedure SetHitTest(Value: Boolean);
    function GetClipChildren: Boolean;
    procedure SetClipChildren(Value: Boolean);
    function GetCapturesWheel: Boolean;
    procedure SetCapturesWheel(Value: Boolean);
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
    function GetDragImage: string;
    procedure SetDragImage(Value: string);
    function GetDropTypes: string;
    procedure SetDropTypes(Value: string);
    function GetTabStop: Boolean;
    procedure SetTabStop(Value: Boolean);
    function GetCursor: string;
    procedure SetCursor(Value: string);
    function GetOpacity: Double;
    procedure SetOpacity(Value: Double);
    function GetCacheLayer: Boolean;
    procedure SetCacheLayer(Value: Boolean);
    function GetRotation: Double;
    procedure SetRotation(Value: Double);
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
    function GetSelfAlign: string;
    procedure SetSelfAlign(Value: string);
    function GetAutoSize: Boolean;
    procedure SetAutoSize(Value: Boolean);
    function GetColumnSpan: Int64;
    procedure SetColumnSpan(Value: Int64);
    function GetShadow: string;
    procedure SetShadow(Value: string);
    function GetStyle: string;
    procedure SetStyle(Value: string);
    function GetTheme: string;
    procedure SetTheme(Value: string);
    function GetFill: string;
    procedure SetFill(Value: string);
    function GetBorderColor: string;
    procedure SetBorderColor(Value: string);
    function GetTextColor: string;
    procedure SetTextColor(Value: string);
    function GetAccentColor: string;
    procedure SetAccentColor(Value: string);
    function GetSuccessColor: string;
    procedure SetSuccessColor(Value: string);
    function GetWarningColor: string;
    procedure SetWarningColor(Value: string);
    function GetDangerColor: string;
    procedure SetDangerColor(Value: string);
    function GetFontFamily: string;
    procedure SetFontFamily(Value: string);
    function GetFontSize: Double;
    procedure SetFontSize(Value: Double);
    function GetFontWeight: string;
    procedure SetFontWeight(Value: string);
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
    function GetPopupMenu: string;
    procedure SetPopupMenu(Value: string);
  public
    property Caption: string read GetCaption write SetCaption;
    property Left: Double read GetLeft write SetLeft;
    property Top: Double read GetTop write SetTop;
    property Width: Double read GetWidth write SetWidth;
    property Height: Double read GetHeight write SetHeight;
    property Align: string read GetAlign write SetAlign;
    property Visible: Boolean read GetVisible write SetVisible;
    property Enabled: Boolean read GetEnabled write SetEnabled;
    property HitTest: Boolean read GetHitTest write SetHitTest;
    property ClipChildren: Boolean read GetClipChildren write SetClipChildren;
    property CapturesWheel: Boolean read GetCapturesWheel write SetCapturesWheel;
    property CanFocus: Boolean read GetCanFocus write SetCanFocus;
    property Draggable: Boolean read GetDraggable write SetDraggable;
    property AcceptsDrops: Boolean read GetAcceptsDrops write SetAcceptsDrops;
    property DragData: string read GetDragData write SetDragData;
    property DragLabel: string read GetDragLabel write SetDragLabel;
    property DragImage: string read GetDragImage write SetDragImage;
    property DropTypes: string read GetDropTypes write SetDropTypes;
    property TabStop: Boolean read GetTabStop write SetTabStop;
    property Cursor: string read GetCursor write SetCursor;
    property Opacity: Double read GetOpacity write SetOpacity;
    property CacheLayer: Boolean read GetCacheLayer write SetCacheLayer;
    property Rotation: Double read GetRotation write SetRotation;
    property TranslateX: Double read GetTranslateX write SetTranslateX;
    property TranslateY: Double read GetTranslateY write SetTranslateY;
    property AspectRatio: Double read GetAspectRatio write SetAspectRatio;
    property MaxWidth: Double read GetMaxWidth write SetMaxWidth;
    property MinWidth: Double read GetMinWidth write SetMinWidth;
    property MinHeight: Double read GetMinHeight write SetMinHeight;
    property MaxHeight: Double read GetMaxHeight write SetMaxHeight;
    property SelfAlign: string read GetSelfAlign write SetSelfAlign;
    property AutoSize: Boolean read GetAutoSize write SetAutoSize;
    property ColumnSpan: Int64 read GetColumnSpan write SetColumnSpan;
    property Shadow: string read GetShadow write SetShadow;
    property Style: string read GetStyle write SetStyle;
    property Theme: string read GetTheme write SetTheme;
    property Fill: string read GetFill write SetFill;
    property BorderColor: string read GetBorderColor write SetBorderColor;
    property TextColor: string read GetTextColor write SetTextColor;
    property AccentColor: string read GetAccentColor write SetAccentColor;
    property SuccessColor: string read GetSuccessColor write SetSuccessColor;
    property WarningColor: string read GetWarningColor write SetWarningColor;
    property DangerColor: string read GetDangerColor write SetDangerColor;
    property FontFamily: string read GetFontFamily write SetFontFamily;
    property FontSize: Double read GetFontSize write SetFontSize;
    property FontWeight: string read GetFontWeight write SetFontWeight;
    property FontItalic: Boolean read GetFontItalic write SetFontItalic;
    property FontUnderline: Boolean read GetFontUnderline write SetFontUnderline;
    property FontStrikeout: Boolean read GetFontStrikeout write SetFontStrikeout;
    property Hint: string read GetHint write SetHint;
    property Tag: Int64 read GetTag write SetTag;
    property PopupMenu: string read GetPopupMenu write SetPopupMenu;
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
    { O Button deste form com o nome, como o FindComponent (nil se não há). }
    function FindButton(const Nome: string): TButton;
    { O Button que disparou o evento (nil se não é um Button deste form). }
    function SenderAsButton(Sender: TAylisComponent): TButton; overload;
    function SenderAsButton(Event: TJSONObject): TButton; overload;
    { O GridLayout deste form com o nome, como o FindComponent (nil se não há). }
    function FindGridLayout(const Nome: string): TGridLayout;
    { O GridLayout que disparou o evento (nil se não é um GridLayout deste form). }
    function SenderAsGridLayout(Sender: TAylisComponent): TGridLayout; overload;
    function SenderAsGridLayout(Event: TJSONObject): TGridLayout; overload;
    { O Label deste form com o nome, como o FindComponent (nil se não há). }
    function FindLabel(const Nome: string): TLabel;
    { O Label que disparou o evento (nil se não é um Label deste form). }
    function SenderAsLabel(Sender: TAylisComponent): TLabel; overload;
    function SenderAsLabel(Event: TJSONObject): TLabel; overload;
    { O Layout deste form com o nome, como o FindComponent (nil se não há). }
    function FindLayout(const Nome: string): TLayout;
    { O Layout que disparou o evento (nil se não é um Layout deste form). }
    function SenderAsLayout(Sender: TAylisComponent): TLayout; overload;
    function SenderAsLayout(Event: TJSONObject): TLayout; overload;
    { O ListView deste form com o nome, como o FindComponent (nil se não há). }
    function FindListView(const Nome: string): TListView;
    { O ListView que disparou o evento (nil se não é um ListView deste form). }
    function SenderAsListView(Sender: TAylisComponent): TListView; overload;
    function SenderAsListView(Event: TJSONObject): TListView; overload;
    { O Panel deste form com o nome, como o FindComponent (nil se não há). }
    function FindPanel(const Nome: string): TPanel;
    { O Panel que disparou o evento (nil se não é um Panel deste form). }
    function SenderAsPanel(Sender: TAylisComponent): TPanel; overload;
    function SenderAsPanel(Event: TJSONObject): TPanel; overload;
    { O TabControl deste form com o nome, como o FindComponent (nil se não há). }
    function FindTabControl(const Nome: string): TTabControl;
    { O TabControl que disparou o evento (nil se não é um TabControl deste form). }
    function SenderAsTabControl(Sender: TAylisComponent): TTabControl; overload;
    function SenderAsTabControl(Event: TJSONObject): TTabControl; overload;
    { O TabItem deste form com o nome, como o FindComponent (nil se não há). }
    function FindTabItem(const Nome: string): TTabItem;
    { O TabItem que disparou o evento (nil se não é um TabItem deste form). }
    function SenderAsTabItem(Sender: TAylisComponent): TTabItem; overload;
    function SenderAsTabItem(Event: TJSONObject): TTabItem; overload;
  end;

implementation

function AsText(Data: TJSONData): string;
begin
  if (Data = nil) or (Data.JSONType = jtNull) then
    Result := ''
  else
    Result := Data.AsString;
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

function TButton.GetIconPosition: string;
begin
  Result := AsText(GetProp('icon_position'));
end;

procedure TButton.SetIconPosition(Value: string);
begin
  SetProp('icon_position', TJSONString.Create(Value));
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

function TButton.GetModalResult: string;
begin
  Result := AsText(GetProp('modal_result'));
end;

procedure TButton.SetModalResult(Value: string);
begin
  SetProp('modal_result', TJSONString.Create(Value));
end;

function TButton.GetRepeatEnter: Boolean;
begin
  Result := GetProp('repeat_enter').AsBoolean;
end;

procedure TButton.SetRepeatEnter(Value: Boolean);
begin
  SetProp('repeat_enter', TJSONBoolean.Create(Value));
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

function TButton.GetAlign: string;
begin
  Result := AsText(GetProp('align'));
end;

procedure TButton.SetAlign(Value: string);
begin
  SetProp('align', TJSONString.Create(Value));
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

function TButton.GetCapturesWheel: Boolean;
begin
  Result := GetProp('captures_wheel').AsBoolean;
end;

procedure TButton.SetCapturesWheel(Value: Boolean);
begin
  SetProp('captures_wheel', TJSONBoolean.Create(Value));
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

function TButton.GetDragImage: string;
begin
  Result := AsText(GetProp('drag_image'));
end;

procedure TButton.SetDragImage(Value: string);
begin
  SetProp('drag_image', TJSONString.Create(Value));
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

function TButton.GetCursor: string;
begin
  Result := AsText(GetProp('cursor'));
end;

procedure TButton.SetCursor(Value: string);
begin
  SetProp('cursor', TJSONString.Create(Value));
end;

function TButton.GetOpacity: Double;
begin
  Result := GetProp('opacity').AsFloat;
end;

procedure TButton.SetOpacity(Value: Double);
begin
  SetProp('opacity', TJSONFloatNumber.Create(Value));
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

function TButton.GetSelfAlign: string;
begin
  Result := AsText(GetProp('self_align'));
end;

procedure TButton.SetSelfAlign(Value: string);
begin
  SetProp('self_align', TJSONString.Create(Value));
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

function TButton.GetShadow: string;
begin
  Result := AsText(GetProp('shadow'));
end;

procedure TButton.SetShadow(Value: string);
begin
  SetProp('shadow', TJSONString.Create(Value));
end;

function TButton.GetStyle: string;
begin
  Result := AsText(GetProp('style'));
end;

procedure TButton.SetStyle(Value: string);
begin
  SetProp('style', TJSONString.Create(Value));
end;

function TButton.GetTheme: string;
begin
  Result := AsText(GetProp('theme'));
end;

procedure TButton.SetTheme(Value: string);
begin
  SetProp('theme', TJSONString.Create(Value));
end;

function TButton.GetFill: string;
begin
  Result := AsText(GetProp('fill'));
end;

procedure TButton.SetFill(Value: string);
begin
  SetProp('fill', TJSONString.Create(Value));
end;

function TButton.GetBorderColor: string;
begin
  Result := AsText(GetProp('border_color'));
end;

procedure TButton.SetBorderColor(Value: string);
begin
  SetProp('border_color', TJSONString.Create(Value));
end;

function TButton.GetTextColor: string;
begin
  Result := AsText(GetProp('text_color'));
end;

procedure TButton.SetTextColor(Value: string);
begin
  SetProp('text_color', TJSONString.Create(Value));
end;

function TButton.GetAccentColor: string;
begin
  Result := AsText(GetProp('accent_color'));
end;

procedure TButton.SetAccentColor(Value: string);
begin
  SetProp('accent_color', TJSONString.Create(Value));
end;

function TButton.GetSuccessColor: string;
begin
  Result := AsText(GetProp('success_color'));
end;

procedure TButton.SetSuccessColor(Value: string);
begin
  SetProp('success_color', TJSONString.Create(Value));
end;

function TButton.GetWarningColor: string;
begin
  Result := AsText(GetProp('warning_color'));
end;

procedure TButton.SetWarningColor(Value: string);
begin
  SetProp('warning_color', TJSONString.Create(Value));
end;

function TButton.GetDangerColor: string;
begin
  Result := AsText(GetProp('danger_color'));
end;

procedure TButton.SetDangerColor(Value: string);
begin
  SetProp('danger_color', TJSONString.Create(Value));
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

function TButton.GetFontWeight: string;
begin
  Result := AsText(GetProp('font_weight'));
end;

procedure TButton.SetFontWeight(Value: string);
begin
  SetProp('font_weight', TJSONString.Create(Value));
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

function TButton.GetPopupMenu: string;
begin
  Result := AsText(GetProp('popup_menu'));
end;

procedure TButton.SetPopupMenu(Value: string);
begin
  SetProp('popup_menu', TJSONString.Create(Value));
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

function TForm.GetPosition: string;
begin
  Result := AsText(GetProp('position'));
end;

procedure TForm.SetPosition(Value: string);
begin
  SetProp('position', TJSONString.Create(Value));
end;

function TForm.GetBorderStyle: string;
begin
  Result := AsText(GetProp('border_style'));
end;

procedure TForm.SetBorderStyle(Value: string);
begin
  SetProp('border_style', TJSONString.Create(Value));
end;

function TForm.GetWindowState: string;
begin
  Result := AsText(GetProp('window_state'));
end;

procedure TForm.SetWindowState(Value: string);
begin
  SetProp('window_state', TJSONString.Create(Value));
end;

function TForm.GetTransition: string;
begin
  Result := AsText(GetProp('transition'));
end;

procedure TForm.SetTransition(Value: string);
begin
  SetProp('transition', TJSONString.Create(Value));
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

function TForm.GetActiveControl: string;
begin
  Result := AsText(GetProp('active_control'));
end;

procedure TForm.SetActiveControl(Value: string);
begin
  SetProp('active_control', TJSONString.Create(Value));
end;

function TForm.GetTabOrder: string;
begin
  Result := AsText(GetProp('tab_order'));
end;

procedure TForm.SetTabOrder(Value: string);
begin
  SetProp('tab_order', TJSONString.Create(Value));
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

function TForm.GetAlign: string;
begin
  Result := AsText(GetProp('align'));
end;

procedure TForm.SetAlign(Value: string);
begin
  SetProp('align', TJSONString.Create(Value));
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

function TForm.GetCapturesWheel: Boolean;
begin
  Result := GetProp('captures_wheel').AsBoolean;
end;

procedure TForm.SetCapturesWheel(Value: Boolean);
begin
  SetProp('captures_wheel', TJSONBoolean.Create(Value));
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

function TForm.GetDragImage: string;
begin
  Result := AsText(GetProp('drag_image'));
end;

procedure TForm.SetDragImage(Value: string);
begin
  SetProp('drag_image', TJSONString.Create(Value));
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

function TForm.GetCursor: string;
begin
  Result := AsText(GetProp('cursor'));
end;

procedure TForm.SetCursor(Value: string);
begin
  SetProp('cursor', TJSONString.Create(Value));
end;

function TForm.GetOpacity: Double;
begin
  Result := GetProp('opacity').AsFloat;
end;

procedure TForm.SetOpacity(Value: Double);
begin
  SetProp('opacity', TJSONFloatNumber.Create(Value));
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

function TForm.GetSelfAlign: string;
begin
  Result := AsText(GetProp('self_align'));
end;

procedure TForm.SetSelfAlign(Value: string);
begin
  SetProp('self_align', TJSONString.Create(Value));
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

function TForm.GetShadow: string;
begin
  Result := AsText(GetProp('shadow'));
end;

procedure TForm.SetShadow(Value: string);
begin
  SetProp('shadow', TJSONString.Create(Value));
end;

function TForm.GetStyle: string;
begin
  Result := AsText(GetProp('style'));
end;

procedure TForm.SetStyle(Value: string);
begin
  SetProp('style', TJSONString.Create(Value));
end;

function TForm.GetTheme: string;
begin
  Result := AsText(GetProp('theme'));
end;

procedure TForm.SetTheme(Value: string);
begin
  SetProp('theme', TJSONString.Create(Value));
end;

function TForm.GetFill: string;
begin
  Result := AsText(GetProp('fill'));
end;

procedure TForm.SetFill(Value: string);
begin
  SetProp('fill', TJSONString.Create(Value));
end;

function TForm.GetBorderColor: string;
begin
  Result := AsText(GetProp('border_color'));
end;

procedure TForm.SetBorderColor(Value: string);
begin
  SetProp('border_color', TJSONString.Create(Value));
end;

function TForm.GetTextColor: string;
begin
  Result := AsText(GetProp('text_color'));
end;

procedure TForm.SetTextColor(Value: string);
begin
  SetProp('text_color', TJSONString.Create(Value));
end;

function TForm.GetAccentColor: string;
begin
  Result := AsText(GetProp('accent_color'));
end;

procedure TForm.SetAccentColor(Value: string);
begin
  SetProp('accent_color', TJSONString.Create(Value));
end;

function TForm.GetSuccessColor: string;
begin
  Result := AsText(GetProp('success_color'));
end;

procedure TForm.SetSuccessColor(Value: string);
begin
  SetProp('success_color', TJSONString.Create(Value));
end;

function TForm.GetWarningColor: string;
begin
  Result := AsText(GetProp('warning_color'));
end;

procedure TForm.SetWarningColor(Value: string);
begin
  SetProp('warning_color', TJSONString.Create(Value));
end;

function TForm.GetDangerColor: string;
begin
  Result := AsText(GetProp('danger_color'));
end;

procedure TForm.SetDangerColor(Value: string);
begin
  SetProp('danger_color', TJSONString.Create(Value));
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

function TForm.GetFontWeight: string;
begin
  Result := AsText(GetProp('font_weight'));
end;

procedure TForm.SetFontWeight(Value: string);
begin
  SetProp('font_weight', TJSONString.Create(Value));
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

function TForm.GetPopupMenu: string;
begin
  Result := AsText(GetProp('popup_menu'));
end;

procedure TForm.SetPopupMenu(Value: string);
begin
  SetProp('popup_menu', TJSONString.Create(Value));
end;

procedure TForm.MoveToMonitor(A1: Int64);
begin
  Aylis.Send('invoke', TJSONObject.Create(['target', Id, 'method', 'move_to_monitor', 'args', TJSONArray.Create([A1])]));
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

function TGridLayout.GetAlign: string;
begin
  Result := AsText(GetProp('align'));
end;

procedure TGridLayout.SetAlign(Value: string);
begin
  SetProp('align', TJSONString.Create(Value));
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

function TGridLayout.GetCapturesWheel: Boolean;
begin
  Result := GetProp('captures_wheel').AsBoolean;
end;

procedure TGridLayout.SetCapturesWheel(Value: Boolean);
begin
  SetProp('captures_wheel', TJSONBoolean.Create(Value));
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

function TGridLayout.GetDragImage: string;
begin
  Result := AsText(GetProp('drag_image'));
end;

procedure TGridLayout.SetDragImage(Value: string);
begin
  SetProp('drag_image', TJSONString.Create(Value));
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

function TGridLayout.GetCursor: string;
begin
  Result := AsText(GetProp('cursor'));
end;

procedure TGridLayout.SetCursor(Value: string);
begin
  SetProp('cursor', TJSONString.Create(Value));
end;

function TGridLayout.GetOpacity: Double;
begin
  Result := GetProp('opacity').AsFloat;
end;

procedure TGridLayout.SetOpacity(Value: Double);
begin
  SetProp('opacity', TJSONFloatNumber.Create(Value));
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

function TGridLayout.GetSelfAlign: string;
begin
  Result := AsText(GetProp('self_align'));
end;

procedure TGridLayout.SetSelfAlign(Value: string);
begin
  SetProp('self_align', TJSONString.Create(Value));
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

function TGridLayout.GetShadow: string;
begin
  Result := AsText(GetProp('shadow'));
end;

procedure TGridLayout.SetShadow(Value: string);
begin
  SetProp('shadow', TJSONString.Create(Value));
end;

function TGridLayout.GetStyle: string;
begin
  Result := AsText(GetProp('style'));
end;

procedure TGridLayout.SetStyle(Value: string);
begin
  SetProp('style', TJSONString.Create(Value));
end;

function TGridLayout.GetTheme: string;
begin
  Result := AsText(GetProp('theme'));
end;

procedure TGridLayout.SetTheme(Value: string);
begin
  SetProp('theme', TJSONString.Create(Value));
end;

function TGridLayout.GetFill: string;
begin
  Result := AsText(GetProp('fill'));
end;

procedure TGridLayout.SetFill(Value: string);
begin
  SetProp('fill', TJSONString.Create(Value));
end;

function TGridLayout.GetBorderColor: string;
begin
  Result := AsText(GetProp('border_color'));
end;

procedure TGridLayout.SetBorderColor(Value: string);
begin
  SetProp('border_color', TJSONString.Create(Value));
end;

function TGridLayout.GetTextColor: string;
begin
  Result := AsText(GetProp('text_color'));
end;

procedure TGridLayout.SetTextColor(Value: string);
begin
  SetProp('text_color', TJSONString.Create(Value));
end;

function TGridLayout.GetAccentColor: string;
begin
  Result := AsText(GetProp('accent_color'));
end;

procedure TGridLayout.SetAccentColor(Value: string);
begin
  SetProp('accent_color', TJSONString.Create(Value));
end;

function TGridLayout.GetSuccessColor: string;
begin
  Result := AsText(GetProp('success_color'));
end;

procedure TGridLayout.SetSuccessColor(Value: string);
begin
  SetProp('success_color', TJSONString.Create(Value));
end;

function TGridLayout.GetWarningColor: string;
begin
  Result := AsText(GetProp('warning_color'));
end;

procedure TGridLayout.SetWarningColor(Value: string);
begin
  SetProp('warning_color', TJSONString.Create(Value));
end;

function TGridLayout.GetDangerColor: string;
begin
  Result := AsText(GetProp('danger_color'));
end;

procedure TGridLayout.SetDangerColor(Value: string);
begin
  SetProp('danger_color', TJSONString.Create(Value));
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

function TGridLayout.GetFontWeight: string;
begin
  Result := AsText(GetProp('font_weight'));
end;

procedure TGridLayout.SetFontWeight(Value: string);
begin
  SetProp('font_weight', TJSONString.Create(Value));
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

function TGridLayout.GetPopupMenu: string;
begin
  Result := AsText(GetProp('popup_menu'));
end;

procedure TGridLayout.SetPopupMenu(Value: string);
begin
  SetProp('popup_menu', TJSONString.Create(Value));
end;

function TLabel.GetText: string;
begin
  Result := AsText(GetProp('text'));
end;

procedure TLabel.SetText(Value: string);
begin
  SetProp('text', TJSONString.Create(Value));
end;

function TLabel.GetHAlign: string;
begin
  Result := AsText(GetProp('h_align'));
end;

procedure TLabel.SetHAlign(Value: string);
begin
  SetProp('h_align', TJSONString.Create(Value));
end;

function TLabel.GetVAlign: string;
begin
  Result := AsText(GetProp('v_align'));
end;

procedure TLabel.SetVAlign(Value: string);
begin
  SetProp('v_align', TJSONString.Create(Value));
end;

function TLabel.GetWrap: Boolean;
begin
  Result := GetProp('wrap').AsBoolean;
end;

procedure TLabel.SetWrap(Value: Boolean);
begin
  SetProp('wrap', TJSONBoolean.Create(Value));
end;

function TLabel.GetTrimming: string;
begin
  Result := AsText(GetProp('trimming'));
end;

procedure TLabel.SetTrimming(Value: string);
begin
  SetProp('trimming', TJSONString.Create(Value));
end;

function TLabel.GetBold: Boolean;
begin
  Result := GetProp('bold').AsBoolean;
end;

procedure TLabel.SetBold(Value: Boolean);
begin
  SetProp('bold', TJSONBoolean.Create(Value));
end;

function TLabel.GetColor: string;
begin
  Result := AsText(GetProp('color'));
end;

procedure TLabel.SetColor(Value: string);
begin
  SetProp('color', TJSONString.Create(Value));
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

function TLabel.GetTextCase: string;
begin
  Result := AsText(GetProp('text_case'));
end;

procedure TLabel.SetTextCase(Value: string);
begin
  SetProp('text_case', TJSONString.Create(Value));
end;

function TLabel.GetMarkup: Boolean;
begin
  Result := GetProp('markup').AsBoolean;
end;

procedure TLabel.SetMarkup(Value: Boolean);
begin
  SetProp('markup', TJSONBoolean.Create(Value));
end;

function TLabel.GetOutline: string;
begin
  Result := AsText(GetProp('outline'));
end;

procedure TLabel.SetOutline(Value: string);
begin
  SetProp('outline', TJSONString.Create(Value));
end;

function TLabel.GetOutlineWidth: Double;
begin
  Result := GetProp('outline_width').AsFloat;
end;

procedure TLabel.SetOutlineWidth(Value: Double);
begin
  SetProp('outline_width', TJSONFloatNumber.Create(Value));
end;

function TLabel.GetFocusControl: string;
begin
  Result := AsText(GetProp('focus_control'));
end;

procedure TLabel.SetFocusControl(Value: string);
begin
  SetProp('focus_control', TJSONString.Create(Value));
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

function TLabel.GetAlign: string;
begin
  Result := AsText(GetProp('align'));
end;

procedure TLabel.SetAlign(Value: string);
begin
  SetProp('align', TJSONString.Create(Value));
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

function TLabel.GetCapturesWheel: Boolean;
begin
  Result := GetProp('captures_wheel').AsBoolean;
end;

procedure TLabel.SetCapturesWheel(Value: Boolean);
begin
  SetProp('captures_wheel', TJSONBoolean.Create(Value));
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

function TLabel.GetDragImage: string;
begin
  Result := AsText(GetProp('drag_image'));
end;

procedure TLabel.SetDragImage(Value: string);
begin
  SetProp('drag_image', TJSONString.Create(Value));
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

function TLabel.GetCursor: string;
begin
  Result := AsText(GetProp('cursor'));
end;

procedure TLabel.SetCursor(Value: string);
begin
  SetProp('cursor', TJSONString.Create(Value));
end;

function TLabel.GetOpacity: Double;
begin
  Result := GetProp('opacity').AsFloat;
end;

procedure TLabel.SetOpacity(Value: Double);
begin
  SetProp('opacity', TJSONFloatNumber.Create(Value));
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

function TLabel.GetSelfAlign: string;
begin
  Result := AsText(GetProp('self_align'));
end;

procedure TLabel.SetSelfAlign(Value: string);
begin
  SetProp('self_align', TJSONString.Create(Value));
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

function TLabel.GetShadow: string;
begin
  Result := AsText(GetProp('shadow'));
end;

procedure TLabel.SetShadow(Value: string);
begin
  SetProp('shadow', TJSONString.Create(Value));
end;

function TLabel.GetStyle: string;
begin
  Result := AsText(GetProp('style'));
end;

procedure TLabel.SetStyle(Value: string);
begin
  SetProp('style', TJSONString.Create(Value));
end;

function TLabel.GetTheme: string;
begin
  Result := AsText(GetProp('theme'));
end;

procedure TLabel.SetTheme(Value: string);
begin
  SetProp('theme', TJSONString.Create(Value));
end;

function TLabel.GetFill: string;
begin
  Result := AsText(GetProp('fill'));
end;

procedure TLabel.SetFill(Value: string);
begin
  SetProp('fill', TJSONString.Create(Value));
end;

function TLabel.GetBorderColor: string;
begin
  Result := AsText(GetProp('border_color'));
end;

procedure TLabel.SetBorderColor(Value: string);
begin
  SetProp('border_color', TJSONString.Create(Value));
end;

function TLabel.GetTextColor: string;
begin
  Result := AsText(GetProp('text_color'));
end;

procedure TLabel.SetTextColor(Value: string);
begin
  SetProp('text_color', TJSONString.Create(Value));
end;

function TLabel.GetAccentColor: string;
begin
  Result := AsText(GetProp('accent_color'));
end;

procedure TLabel.SetAccentColor(Value: string);
begin
  SetProp('accent_color', TJSONString.Create(Value));
end;

function TLabel.GetSuccessColor: string;
begin
  Result := AsText(GetProp('success_color'));
end;

procedure TLabel.SetSuccessColor(Value: string);
begin
  SetProp('success_color', TJSONString.Create(Value));
end;

function TLabel.GetWarningColor: string;
begin
  Result := AsText(GetProp('warning_color'));
end;

procedure TLabel.SetWarningColor(Value: string);
begin
  SetProp('warning_color', TJSONString.Create(Value));
end;

function TLabel.GetDangerColor: string;
begin
  Result := AsText(GetProp('danger_color'));
end;

procedure TLabel.SetDangerColor(Value: string);
begin
  SetProp('danger_color', TJSONString.Create(Value));
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

function TLabel.GetFontWeight: string;
begin
  Result := AsText(GetProp('font_weight'));
end;

procedure TLabel.SetFontWeight(Value: string);
begin
  SetProp('font_weight', TJSONString.Create(Value));
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

function TLabel.GetPopupMenu: string;
begin
  Result := AsText(GetProp('popup_menu'));
end;

procedure TLabel.SetPopupMenu(Value: string);
begin
  SetProp('popup_menu', TJSONString.Create(Value));
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

function TLayout.GetAlign: string;
begin
  Result := AsText(GetProp('align'));
end;

procedure TLayout.SetAlign(Value: string);
begin
  SetProp('align', TJSONString.Create(Value));
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

function TLayout.GetCapturesWheel: Boolean;
begin
  Result := GetProp('captures_wheel').AsBoolean;
end;

procedure TLayout.SetCapturesWheel(Value: Boolean);
begin
  SetProp('captures_wheel', TJSONBoolean.Create(Value));
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

function TLayout.GetDragImage: string;
begin
  Result := AsText(GetProp('drag_image'));
end;

procedure TLayout.SetDragImage(Value: string);
begin
  SetProp('drag_image', TJSONString.Create(Value));
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

function TLayout.GetCursor: string;
begin
  Result := AsText(GetProp('cursor'));
end;

procedure TLayout.SetCursor(Value: string);
begin
  SetProp('cursor', TJSONString.Create(Value));
end;

function TLayout.GetOpacity: Double;
begin
  Result := GetProp('opacity').AsFloat;
end;

procedure TLayout.SetOpacity(Value: Double);
begin
  SetProp('opacity', TJSONFloatNumber.Create(Value));
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

function TLayout.GetSelfAlign: string;
begin
  Result := AsText(GetProp('self_align'));
end;

procedure TLayout.SetSelfAlign(Value: string);
begin
  SetProp('self_align', TJSONString.Create(Value));
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

function TLayout.GetShadow: string;
begin
  Result := AsText(GetProp('shadow'));
end;

procedure TLayout.SetShadow(Value: string);
begin
  SetProp('shadow', TJSONString.Create(Value));
end;

function TLayout.GetStyle: string;
begin
  Result := AsText(GetProp('style'));
end;

procedure TLayout.SetStyle(Value: string);
begin
  SetProp('style', TJSONString.Create(Value));
end;

function TLayout.GetTheme: string;
begin
  Result := AsText(GetProp('theme'));
end;

procedure TLayout.SetTheme(Value: string);
begin
  SetProp('theme', TJSONString.Create(Value));
end;

function TLayout.GetFill: string;
begin
  Result := AsText(GetProp('fill'));
end;

procedure TLayout.SetFill(Value: string);
begin
  SetProp('fill', TJSONString.Create(Value));
end;

function TLayout.GetBorderColor: string;
begin
  Result := AsText(GetProp('border_color'));
end;

procedure TLayout.SetBorderColor(Value: string);
begin
  SetProp('border_color', TJSONString.Create(Value));
end;

function TLayout.GetTextColor: string;
begin
  Result := AsText(GetProp('text_color'));
end;

procedure TLayout.SetTextColor(Value: string);
begin
  SetProp('text_color', TJSONString.Create(Value));
end;

function TLayout.GetAccentColor: string;
begin
  Result := AsText(GetProp('accent_color'));
end;

procedure TLayout.SetAccentColor(Value: string);
begin
  SetProp('accent_color', TJSONString.Create(Value));
end;

function TLayout.GetSuccessColor: string;
begin
  Result := AsText(GetProp('success_color'));
end;

procedure TLayout.SetSuccessColor(Value: string);
begin
  SetProp('success_color', TJSONString.Create(Value));
end;

function TLayout.GetWarningColor: string;
begin
  Result := AsText(GetProp('warning_color'));
end;

procedure TLayout.SetWarningColor(Value: string);
begin
  SetProp('warning_color', TJSONString.Create(Value));
end;

function TLayout.GetDangerColor: string;
begin
  Result := AsText(GetProp('danger_color'));
end;

procedure TLayout.SetDangerColor(Value: string);
begin
  SetProp('danger_color', TJSONString.Create(Value));
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

function TLayout.GetFontWeight: string;
begin
  Result := AsText(GetProp('font_weight'));
end;

procedure TLayout.SetFontWeight(Value: string);
begin
  SetProp('font_weight', TJSONString.Create(Value));
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

function TLayout.GetPopupMenu: string;
begin
  Result := AsText(GetProp('popup_menu'));
end;

procedure TLayout.SetPopupMenu(Value: string);
begin
  SetProp('popup_menu', TJSONString.Create(Value));
end;

function TListView.GetItems: string;
begin
  Result := AsText(GetProp('items'));
end;

procedure TListView.SetItems(Value: string);
begin
  SetProp('items', TJSONString.Create(Value));
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

function TListView.GetAlign: string;
begin
  Result := AsText(GetProp('align'));
end;

procedure TListView.SetAlign(Value: string);
begin
  SetProp('align', TJSONString.Create(Value));
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

function TListView.GetCapturesWheel: Boolean;
begin
  Result := GetProp('captures_wheel').AsBoolean;
end;

procedure TListView.SetCapturesWheel(Value: Boolean);
begin
  SetProp('captures_wheel', TJSONBoolean.Create(Value));
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

function TListView.GetDragImage: string;
begin
  Result := AsText(GetProp('drag_image'));
end;

procedure TListView.SetDragImage(Value: string);
begin
  SetProp('drag_image', TJSONString.Create(Value));
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

function TListView.GetCursor: string;
begin
  Result := AsText(GetProp('cursor'));
end;

procedure TListView.SetCursor(Value: string);
begin
  SetProp('cursor', TJSONString.Create(Value));
end;

function TListView.GetOpacity: Double;
begin
  Result := GetProp('opacity').AsFloat;
end;

procedure TListView.SetOpacity(Value: Double);
begin
  SetProp('opacity', TJSONFloatNumber.Create(Value));
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

function TListView.GetSelfAlign: string;
begin
  Result := AsText(GetProp('self_align'));
end;

procedure TListView.SetSelfAlign(Value: string);
begin
  SetProp('self_align', TJSONString.Create(Value));
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

function TListView.GetShadow: string;
begin
  Result := AsText(GetProp('shadow'));
end;

procedure TListView.SetShadow(Value: string);
begin
  SetProp('shadow', TJSONString.Create(Value));
end;

function TListView.GetStyle: string;
begin
  Result := AsText(GetProp('style'));
end;

procedure TListView.SetStyle(Value: string);
begin
  SetProp('style', TJSONString.Create(Value));
end;

function TListView.GetTheme: string;
begin
  Result := AsText(GetProp('theme'));
end;

procedure TListView.SetTheme(Value: string);
begin
  SetProp('theme', TJSONString.Create(Value));
end;

function TListView.GetFill: string;
begin
  Result := AsText(GetProp('fill'));
end;

procedure TListView.SetFill(Value: string);
begin
  SetProp('fill', TJSONString.Create(Value));
end;

function TListView.GetBorderColor: string;
begin
  Result := AsText(GetProp('border_color'));
end;

procedure TListView.SetBorderColor(Value: string);
begin
  SetProp('border_color', TJSONString.Create(Value));
end;

function TListView.GetTextColor: string;
begin
  Result := AsText(GetProp('text_color'));
end;

procedure TListView.SetTextColor(Value: string);
begin
  SetProp('text_color', TJSONString.Create(Value));
end;

function TListView.GetAccentColor: string;
begin
  Result := AsText(GetProp('accent_color'));
end;

procedure TListView.SetAccentColor(Value: string);
begin
  SetProp('accent_color', TJSONString.Create(Value));
end;

function TListView.GetSuccessColor: string;
begin
  Result := AsText(GetProp('success_color'));
end;

procedure TListView.SetSuccessColor(Value: string);
begin
  SetProp('success_color', TJSONString.Create(Value));
end;

function TListView.GetWarningColor: string;
begin
  Result := AsText(GetProp('warning_color'));
end;

procedure TListView.SetWarningColor(Value: string);
begin
  SetProp('warning_color', TJSONString.Create(Value));
end;

function TListView.GetDangerColor: string;
begin
  Result := AsText(GetProp('danger_color'));
end;

procedure TListView.SetDangerColor(Value: string);
begin
  SetProp('danger_color', TJSONString.Create(Value));
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

function TListView.GetFontWeight: string;
begin
  Result := AsText(GetProp('font_weight'));
end;

procedure TListView.SetFontWeight(Value: string);
begin
  SetProp('font_weight', TJSONString.Create(Value));
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

function TListView.GetPopupMenu: string;
begin
  Result := AsText(GetProp('popup_menu'));
end;

procedure TListView.SetPopupMenu(Value: string);
begin
  SetProp('popup_menu', TJSONString.Create(Value));
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

function TPanel.GetFill: string;
begin
  Result := AsText(GetProp('fill'));
end;

procedure TPanel.SetFill(Value: string);
begin
  SetProp('fill', TJSONString.Create(Value));
end;

function TPanel.GetFill2: string;
begin
  Result := AsText(GetProp('fill2'));
end;

procedure TPanel.SetFill2(Value: string);
begin
  SetProp('fill2', TJSONString.Create(Value));
end;

function TPanel.GetFillMid: string;
begin
  Result := AsText(GetProp('fill_mid'));
end;

procedure TPanel.SetFillMid(Value: string);
begin
  SetProp('fill_mid', TJSONString.Create(Value));
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

function TPanel.GetStroke: string;
begin
  Result := AsText(GetProp('stroke'));
end;

procedure TPanel.SetStroke(Value: string);
begin
  SetProp('stroke', TJSONString.Create(Value));
end;

function TPanel.GetStrokeWidth: Double;
begin
  Result := GetProp('stroke_width').AsFloat;
end;

procedure TPanel.SetStrokeWidth(Value: Double);
begin
  SetProp('stroke_width', TJSONFloatNumber.Create(Value));
end;

function TPanel.GetRadius: Double;
begin
  Result := GetProp('radius').AsFloat;
end;

procedure TPanel.SetRadius(Value: Double);
begin
  SetProp('radius', TJSONFloatNumber.Create(Value));
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

function TPanel.GetAlign: string;
begin
  Result := AsText(GetProp('align'));
end;

procedure TPanel.SetAlign(Value: string);
begin
  SetProp('align', TJSONString.Create(Value));
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

function TPanel.GetCapturesWheel: Boolean;
begin
  Result := GetProp('captures_wheel').AsBoolean;
end;

procedure TPanel.SetCapturesWheel(Value: Boolean);
begin
  SetProp('captures_wheel', TJSONBoolean.Create(Value));
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

function TPanel.GetDragImage: string;
begin
  Result := AsText(GetProp('drag_image'));
end;

procedure TPanel.SetDragImage(Value: string);
begin
  SetProp('drag_image', TJSONString.Create(Value));
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

function TPanel.GetCursor: string;
begin
  Result := AsText(GetProp('cursor'));
end;

procedure TPanel.SetCursor(Value: string);
begin
  SetProp('cursor', TJSONString.Create(Value));
end;

function TPanel.GetOpacity: Double;
begin
  Result := GetProp('opacity').AsFloat;
end;

procedure TPanel.SetOpacity(Value: Double);
begin
  SetProp('opacity', TJSONFloatNumber.Create(Value));
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

function TPanel.GetSelfAlign: string;
begin
  Result := AsText(GetProp('self_align'));
end;

procedure TPanel.SetSelfAlign(Value: string);
begin
  SetProp('self_align', TJSONString.Create(Value));
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

function TPanel.GetShadow: string;
begin
  Result := AsText(GetProp('shadow'));
end;

procedure TPanel.SetShadow(Value: string);
begin
  SetProp('shadow', TJSONString.Create(Value));
end;

function TPanel.GetStyle: string;
begin
  Result := AsText(GetProp('style'));
end;

procedure TPanel.SetStyle(Value: string);
begin
  SetProp('style', TJSONString.Create(Value));
end;

function TPanel.GetTheme: string;
begin
  Result := AsText(GetProp('theme'));
end;

procedure TPanel.SetTheme(Value: string);
begin
  SetProp('theme', TJSONString.Create(Value));
end;

function TPanel.GetBorderColor: string;
begin
  Result := AsText(GetProp('border_color'));
end;

procedure TPanel.SetBorderColor(Value: string);
begin
  SetProp('border_color', TJSONString.Create(Value));
end;

function TPanel.GetTextColor: string;
begin
  Result := AsText(GetProp('text_color'));
end;

procedure TPanel.SetTextColor(Value: string);
begin
  SetProp('text_color', TJSONString.Create(Value));
end;

function TPanel.GetAccentColor: string;
begin
  Result := AsText(GetProp('accent_color'));
end;

procedure TPanel.SetAccentColor(Value: string);
begin
  SetProp('accent_color', TJSONString.Create(Value));
end;

function TPanel.GetSuccessColor: string;
begin
  Result := AsText(GetProp('success_color'));
end;

procedure TPanel.SetSuccessColor(Value: string);
begin
  SetProp('success_color', TJSONString.Create(Value));
end;

function TPanel.GetWarningColor: string;
begin
  Result := AsText(GetProp('warning_color'));
end;

procedure TPanel.SetWarningColor(Value: string);
begin
  SetProp('warning_color', TJSONString.Create(Value));
end;

function TPanel.GetDangerColor: string;
begin
  Result := AsText(GetProp('danger_color'));
end;

procedure TPanel.SetDangerColor(Value: string);
begin
  SetProp('danger_color', TJSONString.Create(Value));
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

function TPanel.GetFontWeight: string;
begin
  Result := AsText(GetProp('font_weight'));
end;

procedure TPanel.SetFontWeight(Value: string);
begin
  SetProp('font_weight', TJSONString.Create(Value));
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

function TPanel.GetPopupMenu: string;
begin
  Result := AsText(GetProp('popup_menu'));
end;

procedure TPanel.SetPopupMenu(Value: string);
begin
  SetProp('popup_menu', TJSONString.Create(Value));
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

function TTabControl.GetTransition: string;
begin
  Result := AsText(GetProp('transition'));
end;

procedure TTabControl.SetTransition(Value: string);
begin
  SetProp('transition', TJSONString.Create(Value));
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

function TTabControl.GetAlign: string;
begin
  Result := AsText(GetProp('align'));
end;

procedure TTabControl.SetAlign(Value: string);
begin
  SetProp('align', TJSONString.Create(Value));
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

function TTabControl.GetCapturesWheel: Boolean;
begin
  Result := GetProp('captures_wheel').AsBoolean;
end;

procedure TTabControl.SetCapturesWheel(Value: Boolean);
begin
  SetProp('captures_wheel', TJSONBoolean.Create(Value));
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

function TTabControl.GetDragImage: string;
begin
  Result := AsText(GetProp('drag_image'));
end;

procedure TTabControl.SetDragImage(Value: string);
begin
  SetProp('drag_image', TJSONString.Create(Value));
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

function TTabControl.GetCursor: string;
begin
  Result := AsText(GetProp('cursor'));
end;

procedure TTabControl.SetCursor(Value: string);
begin
  SetProp('cursor', TJSONString.Create(Value));
end;

function TTabControl.GetOpacity: Double;
begin
  Result := GetProp('opacity').AsFloat;
end;

procedure TTabControl.SetOpacity(Value: Double);
begin
  SetProp('opacity', TJSONFloatNumber.Create(Value));
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

function TTabControl.GetSelfAlign: string;
begin
  Result := AsText(GetProp('self_align'));
end;

procedure TTabControl.SetSelfAlign(Value: string);
begin
  SetProp('self_align', TJSONString.Create(Value));
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

function TTabControl.GetShadow: string;
begin
  Result := AsText(GetProp('shadow'));
end;

procedure TTabControl.SetShadow(Value: string);
begin
  SetProp('shadow', TJSONString.Create(Value));
end;

function TTabControl.GetStyle: string;
begin
  Result := AsText(GetProp('style'));
end;

procedure TTabControl.SetStyle(Value: string);
begin
  SetProp('style', TJSONString.Create(Value));
end;

function TTabControl.GetTheme: string;
begin
  Result := AsText(GetProp('theme'));
end;

procedure TTabControl.SetTheme(Value: string);
begin
  SetProp('theme', TJSONString.Create(Value));
end;

function TTabControl.GetFill: string;
begin
  Result := AsText(GetProp('fill'));
end;

procedure TTabControl.SetFill(Value: string);
begin
  SetProp('fill', TJSONString.Create(Value));
end;

function TTabControl.GetBorderColor: string;
begin
  Result := AsText(GetProp('border_color'));
end;

procedure TTabControl.SetBorderColor(Value: string);
begin
  SetProp('border_color', TJSONString.Create(Value));
end;

function TTabControl.GetTextColor: string;
begin
  Result := AsText(GetProp('text_color'));
end;

procedure TTabControl.SetTextColor(Value: string);
begin
  SetProp('text_color', TJSONString.Create(Value));
end;

function TTabControl.GetAccentColor: string;
begin
  Result := AsText(GetProp('accent_color'));
end;

procedure TTabControl.SetAccentColor(Value: string);
begin
  SetProp('accent_color', TJSONString.Create(Value));
end;

function TTabControl.GetSuccessColor: string;
begin
  Result := AsText(GetProp('success_color'));
end;

procedure TTabControl.SetSuccessColor(Value: string);
begin
  SetProp('success_color', TJSONString.Create(Value));
end;

function TTabControl.GetWarningColor: string;
begin
  Result := AsText(GetProp('warning_color'));
end;

procedure TTabControl.SetWarningColor(Value: string);
begin
  SetProp('warning_color', TJSONString.Create(Value));
end;

function TTabControl.GetDangerColor: string;
begin
  Result := AsText(GetProp('danger_color'));
end;

procedure TTabControl.SetDangerColor(Value: string);
begin
  SetProp('danger_color', TJSONString.Create(Value));
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

function TTabControl.GetFontWeight: string;
begin
  Result := AsText(GetProp('font_weight'));
end;

procedure TTabControl.SetFontWeight(Value: string);
begin
  SetProp('font_weight', TJSONString.Create(Value));
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

function TTabControl.GetPopupMenu: string;
begin
  Result := AsText(GetProp('popup_menu'));
end;

procedure TTabControl.SetPopupMenu(Value: string);
begin
  SetProp('popup_menu', TJSONString.Create(Value));
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

function TTabItem.GetAlign: string;
begin
  Result := AsText(GetProp('align'));
end;

procedure TTabItem.SetAlign(Value: string);
begin
  SetProp('align', TJSONString.Create(Value));
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

function TTabItem.GetCapturesWheel: Boolean;
begin
  Result := GetProp('captures_wheel').AsBoolean;
end;

procedure TTabItem.SetCapturesWheel(Value: Boolean);
begin
  SetProp('captures_wheel', TJSONBoolean.Create(Value));
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

function TTabItem.GetDragImage: string;
begin
  Result := AsText(GetProp('drag_image'));
end;

procedure TTabItem.SetDragImage(Value: string);
begin
  SetProp('drag_image', TJSONString.Create(Value));
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

function TTabItem.GetCursor: string;
begin
  Result := AsText(GetProp('cursor'));
end;

procedure TTabItem.SetCursor(Value: string);
begin
  SetProp('cursor', TJSONString.Create(Value));
end;

function TTabItem.GetOpacity: Double;
begin
  Result := GetProp('opacity').AsFloat;
end;

procedure TTabItem.SetOpacity(Value: Double);
begin
  SetProp('opacity', TJSONFloatNumber.Create(Value));
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

function TTabItem.GetSelfAlign: string;
begin
  Result := AsText(GetProp('self_align'));
end;

procedure TTabItem.SetSelfAlign(Value: string);
begin
  SetProp('self_align', TJSONString.Create(Value));
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

function TTabItem.GetShadow: string;
begin
  Result := AsText(GetProp('shadow'));
end;

procedure TTabItem.SetShadow(Value: string);
begin
  SetProp('shadow', TJSONString.Create(Value));
end;

function TTabItem.GetStyle: string;
begin
  Result := AsText(GetProp('style'));
end;

procedure TTabItem.SetStyle(Value: string);
begin
  SetProp('style', TJSONString.Create(Value));
end;

function TTabItem.GetTheme: string;
begin
  Result := AsText(GetProp('theme'));
end;

procedure TTabItem.SetTheme(Value: string);
begin
  SetProp('theme', TJSONString.Create(Value));
end;

function TTabItem.GetFill: string;
begin
  Result := AsText(GetProp('fill'));
end;

procedure TTabItem.SetFill(Value: string);
begin
  SetProp('fill', TJSONString.Create(Value));
end;

function TTabItem.GetBorderColor: string;
begin
  Result := AsText(GetProp('border_color'));
end;

procedure TTabItem.SetBorderColor(Value: string);
begin
  SetProp('border_color', TJSONString.Create(Value));
end;

function TTabItem.GetTextColor: string;
begin
  Result := AsText(GetProp('text_color'));
end;

procedure TTabItem.SetTextColor(Value: string);
begin
  SetProp('text_color', TJSONString.Create(Value));
end;

function TTabItem.GetAccentColor: string;
begin
  Result := AsText(GetProp('accent_color'));
end;

procedure TTabItem.SetAccentColor(Value: string);
begin
  SetProp('accent_color', TJSONString.Create(Value));
end;

function TTabItem.GetSuccessColor: string;
begin
  Result := AsText(GetProp('success_color'));
end;

procedure TTabItem.SetSuccessColor(Value: string);
begin
  SetProp('success_color', TJSONString.Create(Value));
end;

function TTabItem.GetWarningColor: string;
begin
  Result := AsText(GetProp('warning_color'));
end;

procedure TTabItem.SetWarningColor(Value: string);
begin
  SetProp('warning_color', TJSONString.Create(Value));
end;

function TTabItem.GetDangerColor: string;
begin
  Result := AsText(GetProp('danger_color'));
end;

procedure TTabItem.SetDangerColor(Value: string);
begin
  SetProp('danger_color', TJSONString.Create(Value));
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

function TTabItem.GetFontWeight: string;
begin
  Result := AsText(GetProp('font_weight'));
end;

procedure TTabItem.SetFontWeight(Value: string);
begin
  SetProp('font_weight', TJSONString.Create(Value));
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

function TTabItem.GetPopupMenu: string;
begin
  Result := AsText(GetProp('popup_menu'));
end;

procedure TTabItem.SetPopupMenu(Value: string);
begin
  SetProp('popup_menu', TJSONString.Create(Value));
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

end.
