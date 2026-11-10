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
    property Vertical: Boolean read GetVertical write SetVertical;
    property Text: string read GetText write SetText;
    property Thickness: Double read GetThickness write SetThickness;
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

  { Um Icon de um form. }
  TIcon = class(TAylisComponent)
  private
    function GetIcon: string;
    procedure SetIcon(Value: string);
    function GetColor: string;
    procedure SetColor(Value: string);
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
    property Icon: string read GetIcon write SetIcon;
    property Color: string read GetColor write SetColor;
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
  public
    property Text: string read GetText write SetText;
    property Shortcut: string read GetShortcut write SetShortcut;
    property Enabled: Boolean read GetEnabled write SetEnabled;
    property Checked: Boolean read GetChecked write SetChecked;
    property AutoCheck: Boolean read GetAutoCheck write SetAutoCheck;
    property Icon: string read GetIcon write SetIcon;
    property Visible: Boolean read GetVisible write SetVisible;
    property Danger: Boolean read GetDanger write SetDanger;
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
  public
    procedure Execute;
    property Title: string read GetTitle write SetTitle;
    property Filter: string read GetFilter write SetFilter;
    property InitialDir: string read GetInitialDir write SetInitialDir;
    property FileName: string read GetFileName write SetFileName;
    property Multiple: Boolean read GetMultiple write SetMultiple;
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

  { Um SeekBar de um form. }
  TSeekBar = class(TAylisComponent)
  private
    function GetPosition: Double;
    procedure SetPosition(Value: Double);
    function GetDuration: Double;
    procedure SetDuration(Value: Double);
    function GetBuffered: Double;
    procedure SetBuffered(Value: Double);
    function GetMarkers: string;
    procedure SetMarkers(Value: string);
    function GetShowTime: Boolean;
    procedure SetShowTime(Value: Boolean);
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
    property Position: Double read GetPosition write SetPosition;
    property Duration: Double read GetDuration write SetDuration;
    property Buffered: Double read GetBuffered write SetBuffered;
    property Markers: string read GetMarkers write SetMarkers;
    property ShowTime: Boolean read GetShowTime write SetShowTime;
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

  { Um Timer de um form. }
  TTimer = class(TAylisComponent)
  private
    function GetInterval: Double;
    procedure SetInterval(Value: Double);
    function GetEnabled: Boolean;
    procedure SetEnabled(Value: Boolean);
    function GetPrecision: string;
    procedure SetPrecision(Value: string);
    function GetBackground: Boolean;
    procedure SetBackground(Value: Boolean);
    function GetDrift: Double;
    procedure SetDrift(Value: Double);
  public
    property Interval: Double read GetInterval write SetInterval;
    property Enabled: Boolean read GetEnabled write SetEnabled;
    property Precision: string read GetPrecision write SetPrecision;
    property Background: Boolean read GetBackground write SetBackground;
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
    property Value: Double read GetValue write SetValue;
    property Min: Double read GetMin write SetMin;
    property Max: Double read GetMax write SetMax;
    property Frequency: Double read GetFrequency write SetFrequency;
    property Vertical: Boolean read GetVertical write SetVertical;
    property ThumbSize: Double read GetThumbSize write SetThumbSize;
    property TrackSize: Double read GetTrackSize write SetTrackSize;
    property SolidThumb: Boolean read GetSolidThumb write SetSolidThumb;
    property Limit: Double read GetLimit write SetLimit;
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
    function GetFit: string;
    procedure SetFit(Value: string);
    function GetRadius: Double;
    procedure SetRadius(Value: Double);
    function GetRate: Double;
    procedure SetRate(Value: Double);
    function GetOutput: string;
    procedure SetOutput(Value: string);
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
    property Fit: string read GetFit write SetFit;
    property Radius: Double read GetRadius write SetRadius;
    property Rate: Double read GetRate write SetRate;
    property Output: string read GetOutput write SetOutput;
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
    { O Button deste form com o nome, como o FindComponent (nil se não há). }
    function FindButton(const Nome: string): TButton;
    { O Button que disparou o evento (nil se não é um Button deste form). }
    function SenderAsButton(Sender: TAylisComponent): TButton; overload;
    function SenderAsButton(Event: TJSONObject): TButton; overload;
    { O Divider deste form com o nome, como o FindComponent (nil se não há). }
    function FindDivider(const Nome: string): TDivider;
    { O Divider que disparou o evento (nil se não é um Divider deste form). }
    function SenderAsDivider(Sender: TAylisComponent): TDivider; overload;
    function SenderAsDivider(Event: TJSONObject): TDivider; overload;
    { O Icon deste form com o nome, como o FindComponent (nil se não há). }
    function FindIcon(const Nome: string): TIcon;
    { O Icon que disparou o evento (nil se não é um Icon deste form). }
    function SenderAsIcon(Sender: TAylisComponent): TIcon; overload;
    function SenderAsIcon(Event: TJSONObject): TIcon; overload;
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
    { O MainMenu deste form com o nome, como o FindComponent (nil se não há). }
    function FindMainMenu(const Nome: string): TMainMenu;
    { O MainMenu que disparou o evento (nil se não é um MainMenu deste form). }
    function SenderAsMainMenu(Sender: TAylisComponent): TMainMenu; overload;
    function SenderAsMainMenu(Event: TJSONObject): TMainMenu; overload;
    { O MenuItem deste form com o nome, como o FindComponent (nil se não há). }
    function FindMenuItem(const Nome: string): TMenuItem;
    { O MenuItem que disparou o evento (nil se não é um MenuItem deste form). }
    function SenderAsMenuItem(Sender: TAylisComponent): TMenuItem; overload;
    function SenderAsMenuItem(Event: TJSONObject): TMenuItem; overload;
    { O OpenDialog deste form com o nome, como o FindComponent (nil se não há). }
    function FindOpenDialog(const Nome: string): TOpenDialog;
    { O OpenDialog que disparou o evento (nil se não é um OpenDialog deste form). }
    function SenderAsOpenDialog(Sender: TAylisComponent): TOpenDialog; overload;
    function SenderAsOpenDialog(Event: TJSONObject): TOpenDialog; overload;
    { O Panel deste form com o nome, como o FindComponent (nil se não há). }
    function FindPanel(const Nome: string): TPanel;
    { O Panel que disparou o evento (nil se não é um Panel deste form). }
    function SenderAsPanel(Sender: TAylisComponent): TPanel; overload;
    function SenderAsPanel(Event: TJSONObject): TPanel; overload;
    { O SeekBar deste form com o nome, como o FindComponent (nil se não há). }
    function FindSeekBar(const Nome: string): TSeekBar;
    { O SeekBar que disparou o evento (nil se não é um SeekBar deste form). }
    function SenderAsSeekBar(Sender: TAylisComponent): TSeekBar; overload;
    function SenderAsSeekBar(Event: TJSONObject): TSeekBar; overload;
    { O Timer deste form com o nome, como o FindComponent (nil se não há). }
    function FindTimer(const Nome: string): TTimer;
    { O Timer que disparou o evento (nil se não é um Timer deste form). }
    function SenderAsTimer(Sender: TAylisComponent): TTimer; overload;
    function SenderAsTimer(Event: TJSONObject): TTimer; overload;
    { O TrackBar deste form com o nome, como o FindComponent (nil se não há). }
    function FindTrackBar(const Nome: string): TTrackBar;
    { O TrackBar que disparou o evento (nil se não é um TrackBar deste form). }
    function SenderAsTrackBar(Sender: TAylisComponent): TTrackBar; overload;
    function SenderAsTrackBar(Event: TJSONObject): TTrackBar; overload;
    { O VideoPlayer deste form com o nome, como o FindComponent (nil se não há). }
    function FindVideoPlayer(const Nome: string): TVideoPlayer;
    { O VideoPlayer que disparou o evento (nil se não é um VideoPlayer deste form). }
    function SenderAsVideoPlayer(Sender: TAylisComponent): TVideoPlayer; overload;
    function SenderAsVideoPlayer(Event: TJSONObject): TVideoPlayer; overload;
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

function TDivider.GetAlign: string;
begin
  Result := AsText(GetProp('align'));
end;

procedure TDivider.SetAlign(Value: string);
begin
  SetProp('align', TJSONString.Create(Value));
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

function TDivider.GetCapturesWheel: Boolean;
begin
  Result := GetProp('captures_wheel').AsBoolean;
end;

procedure TDivider.SetCapturesWheel(Value: Boolean);
begin
  SetProp('captures_wheel', TJSONBoolean.Create(Value));
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

function TDivider.GetDragImage: string;
begin
  Result := AsText(GetProp('drag_image'));
end;

procedure TDivider.SetDragImage(Value: string);
begin
  SetProp('drag_image', TJSONString.Create(Value));
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

function TDivider.GetCursor: string;
begin
  Result := AsText(GetProp('cursor'));
end;

procedure TDivider.SetCursor(Value: string);
begin
  SetProp('cursor', TJSONString.Create(Value));
end;

function TDivider.GetOpacity: Double;
begin
  Result := GetProp('opacity').AsFloat;
end;

procedure TDivider.SetOpacity(Value: Double);
begin
  SetProp('opacity', TJSONFloatNumber.Create(Value));
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

function TDivider.GetSelfAlign: string;
begin
  Result := AsText(GetProp('self_align'));
end;

procedure TDivider.SetSelfAlign(Value: string);
begin
  SetProp('self_align', TJSONString.Create(Value));
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

function TDivider.GetShadow: string;
begin
  Result := AsText(GetProp('shadow'));
end;

procedure TDivider.SetShadow(Value: string);
begin
  SetProp('shadow', TJSONString.Create(Value));
end;

function TDivider.GetStyle: string;
begin
  Result := AsText(GetProp('style'));
end;

procedure TDivider.SetStyle(Value: string);
begin
  SetProp('style', TJSONString.Create(Value));
end;

function TDivider.GetTheme: string;
begin
  Result := AsText(GetProp('theme'));
end;

procedure TDivider.SetTheme(Value: string);
begin
  SetProp('theme', TJSONString.Create(Value));
end;

function TDivider.GetFill: string;
begin
  Result := AsText(GetProp('fill'));
end;

procedure TDivider.SetFill(Value: string);
begin
  SetProp('fill', TJSONString.Create(Value));
end;

function TDivider.GetBorderColor: string;
begin
  Result := AsText(GetProp('border_color'));
end;

procedure TDivider.SetBorderColor(Value: string);
begin
  SetProp('border_color', TJSONString.Create(Value));
end;

function TDivider.GetTextColor: string;
begin
  Result := AsText(GetProp('text_color'));
end;

procedure TDivider.SetTextColor(Value: string);
begin
  SetProp('text_color', TJSONString.Create(Value));
end;

function TDivider.GetAccentColor: string;
begin
  Result := AsText(GetProp('accent_color'));
end;

procedure TDivider.SetAccentColor(Value: string);
begin
  SetProp('accent_color', TJSONString.Create(Value));
end;

function TDivider.GetSuccessColor: string;
begin
  Result := AsText(GetProp('success_color'));
end;

procedure TDivider.SetSuccessColor(Value: string);
begin
  SetProp('success_color', TJSONString.Create(Value));
end;

function TDivider.GetWarningColor: string;
begin
  Result := AsText(GetProp('warning_color'));
end;

procedure TDivider.SetWarningColor(Value: string);
begin
  SetProp('warning_color', TJSONString.Create(Value));
end;

function TDivider.GetDangerColor: string;
begin
  Result := AsText(GetProp('danger_color'));
end;

procedure TDivider.SetDangerColor(Value: string);
begin
  SetProp('danger_color', TJSONString.Create(Value));
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

function TDivider.GetFontWeight: string;
begin
  Result := AsText(GetProp('font_weight'));
end;

procedure TDivider.SetFontWeight(Value: string);
begin
  SetProp('font_weight', TJSONString.Create(Value));
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

function TDivider.GetPopupMenu: string;
begin
  Result := AsText(GetProp('popup_menu'));
end;

procedure TDivider.SetPopupMenu(Value: string);
begin
  SetProp('popup_menu', TJSONString.Create(Value));
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

function TIcon.GetIcon: string;
begin
  Result := AsText(GetProp('icon'));
end;

procedure TIcon.SetIcon(Value: string);
begin
  SetProp('icon', TJSONString.Create(Value));
end;

function TIcon.GetColor: string;
begin
  Result := AsText(GetProp('color'));
end;

procedure TIcon.SetColor(Value: string);
begin
  SetProp('color', TJSONString.Create(Value));
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

function TIcon.GetAlign: string;
begin
  Result := AsText(GetProp('align'));
end;

procedure TIcon.SetAlign(Value: string);
begin
  SetProp('align', TJSONString.Create(Value));
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

function TIcon.GetCapturesWheel: Boolean;
begin
  Result := GetProp('captures_wheel').AsBoolean;
end;

procedure TIcon.SetCapturesWheel(Value: Boolean);
begin
  SetProp('captures_wheel', TJSONBoolean.Create(Value));
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

function TIcon.GetDragImage: string;
begin
  Result := AsText(GetProp('drag_image'));
end;

procedure TIcon.SetDragImage(Value: string);
begin
  SetProp('drag_image', TJSONString.Create(Value));
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

function TIcon.GetCursor: string;
begin
  Result := AsText(GetProp('cursor'));
end;

procedure TIcon.SetCursor(Value: string);
begin
  SetProp('cursor', TJSONString.Create(Value));
end;

function TIcon.GetOpacity: Double;
begin
  Result := GetProp('opacity').AsFloat;
end;

procedure TIcon.SetOpacity(Value: Double);
begin
  SetProp('opacity', TJSONFloatNumber.Create(Value));
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

function TIcon.GetSelfAlign: string;
begin
  Result := AsText(GetProp('self_align'));
end;

procedure TIcon.SetSelfAlign(Value: string);
begin
  SetProp('self_align', TJSONString.Create(Value));
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

function TIcon.GetShadow: string;
begin
  Result := AsText(GetProp('shadow'));
end;

procedure TIcon.SetShadow(Value: string);
begin
  SetProp('shadow', TJSONString.Create(Value));
end;

function TIcon.GetStyle: string;
begin
  Result := AsText(GetProp('style'));
end;

procedure TIcon.SetStyle(Value: string);
begin
  SetProp('style', TJSONString.Create(Value));
end;

function TIcon.GetTheme: string;
begin
  Result := AsText(GetProp('theme'));
end;

procedure TIcon.SetTheme(Value: string);
begin
  SetProp('theme', TJSONString.Create(Value));
end;

function TIcon.GetFill: string;
begin
  Result := AsText(GetProp('fill'));
end;

procedure TIcon.SetFill(Value: string);
begin
  SetProp('fill', TJSONString.Create(Value));
end;

function TIcon.GetBorderColor: string;
begin
  Result := AsText(GetProp('border_color'));
end;

procedure TIcon.SetBorderColor(Value: string);
begin
  SetProp('border_color', TJSONString.Create(Value));
end;

function TIcon.GetTextColor: string;
begin
  Result := AsText(GetProp('text_color'));
end;

procedure TIcon.SetTextColor(Value: string);
begin
  SetProp('text_color', TJSONString.Create(Value));
end;

function TIcon.GetAccentColor: string;
begin
  Result := AsText(GetProp('accent_color'));
end;

procedure TIcon.SetAccentColor(Value: string);
begin
  SetProp('accent_color', TJSONString.Create(Value));
end;

function TIcon.GetSuccessColor: string;
begin
  Result := AsText(GetProp('success_color'));
end;

procedure TIcon.SetSuccessColor(Value: string);
begin
  SetProp('success_color', TJSONString.Create(Value));
end;

function TIcon.GetWarningColor: string;
begin
  Result := AsText(GetProp('warning_color'));
end;

procedure TIcon.SetWarningColor(Value: string);
begin
  SetProp('warning_color', TJSONString.Create(Value));
end;

function TIcon.GetDangerColor: string;
begin
  Result := AsText(GetProp('danger_color'));
end;

procedure TIcon.SetDangerColor(Value: string);
begin
  SetProp('danger_color', TJSONString.Create(Value));
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

function TIcon.GetFontWeight: string;
begin
  Result := AsText(GetProp('font_weight'));
end;

procedure TIcon.SetFontWeight(Value: string);
begin
  SetProp('font_weight', TJSONString.Create(Value));
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

function TIcon.GetPopupMenu: string;
begin
  Result := AsText(GetProp('popup_menu'));
end;

procedure TIcon.SetPopupMenu(Value: string);
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

function TMainMenu.GetAlign: string;
begin
  Result := AsText(GetProp('align'));
end;

procedure TMainMenu.SetAlign(Value: string);
begin
  SetProp('align', TJSONString.Create(Value));
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

function TMainMenu.GetCapturesWheel: Boolean;
begin
  Result := GetProp('captures_wheel').AsBoolean;
end;

procedure TMainMenu.SetCapturesWheel(Value: Boolean);
begin
  SetProp('captures_wheel', TJSONBoolean.Create(Value));
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

function TMainMenu.GetDragImage: string;
begin
  Result := AsText(GetProp('drag_image'));
end;

procedure TMainMenu.SetDragImage(Value: string);
begin
  SetProp('drag_image', TJSONString.Create(Value));
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

function TMainMenu.GetCursor: string;
begin
  Result := AsText(GetProp('cursor'));
end;

procedure TMainMenu.SetCursor(Value: string);
begin
  SetProp('cursor', TJSONString.Create(Value));
end;

function TMainMenu.GetOpacity: Double;
begin
  Result := GetProp('opacity').AsFloat;
end;

procedure TMainMenu.SetOpacity(Value: Double);
begin
  SetProp('opacity', TJSONFloatNumber.Create(Value));
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

function TMainMenu.GetSelfAlign: string;
begin
  Result := AsText(GetProp('self_align'));
end;

procedure TMainMenu.SetSelfAlign(Value: string);
begin
  SetProp('self_align', TJSONString.Create(Value));
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

function TMainMenu.GetShadow: string;
begin
  Result := AsText(GetProp('shadow'));
end;

procedure TMainMenu.SetShadow(Value: string);
begin
  SetProp('shadow', TJSONString.Create(Value));
end;

function TMainMenu.GetStyle: string;
begin
  Result := AsText(GetProp('style'));
end;

procedure TMainMenu.SetStyle(Value: string);
begin
  SetProp('style', TJSONString.Create(Value));
end;

function TMainMenu.GetTheme: string;
begin
  Result := AsText(GetProp('theme'));
end;

procedure TMainMenu.SetTheme(Value: string);
begin
  SetProp('theme', TJSONString.Create(Value));
end;

function TMainMenu.GetFill: string;
begin
  Result := AsText(GetProp('fill'));
end;

procedure TMainMenu.SetFill(Value: string);
begin
  SetProp('fill', TJSONString.Create(Value));
end;

function TMainMenu.GetBorderColor: string;
begin
  Result := AsText(GetProp('border_color'));
end;

procedure TMainMenu.SetBorderColor(Value: string);
begin
  SetProp('border_color', TJSONString.Create(Value));
end;

function TMainMenu.GetTextColor: string;
begin
  Result := AsText(GetProp('text_color'));
end;

procedure TMainMenu.SetTextColor(Value: string);
begin
  SetProp('text_color', TJSONString.Create(Value));
end;

function TMainMenu.GetAccentColor: string;
begin
  Result := AsText(GetProp('accent_color'));
end;

procedure TMainMenu.SetAccentColor(Value: string);
begin
  SetProp('accent_color', TJSONString.Create(Value));
end;

function TMainMenu.GetSuccessColor: string;
begin
  Result := AsText(GetProp('success_color'));
end;

procedure TMainMenu.SetSuccessColor(Value: string);
begin
  SetProp('success_color', TJSONString.Create(Value));
end;

function TMainMenu.GetWarningColor: string;
begin
  Result := AsText(GetProp('warning_color'));
end;

procedure TMainMenu.SetWarningColor(Value: string);
begin
  SetProp('warning_color', TJSONString.Create(Value));
end;

function TMainMenu.GetDangerColor: string;
begin
  Result := AsText(GetProp('danger_color'));
end;

procedure TMainMenu.SetDangerColor(Value: string);
begin
  SetProp('danger_color', TJSONString.Create(Value));
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

function TMainMenu.GetFontWeight: string;
begin
  Result := AsText(GetProp('font_weight'));
end;

procedure TMainMenu.SetFontWeight(Value: string);
begin
  SetProp('font_weight', TJSONString.Create(Value));
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

function TMainMenu.GetPopupMenu: string;
begin
  Result := AsText(GetProp('popup_menu'));
end;

procedure TMainMenu.SetPopupMenu(Value: string);
begin
  SetProp('popup_menu', TJSONString.Create(Value));
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

function TSeekBar.GetMarkers: string;
begin
  Result := AsText(GetProp('markers'));
end;

procedure TSeekBar.SetMarkers(Value: string);
begin
  SetProp('markers', TJSONString.Create(Value));
end;

function TSeekBar.GetShowTime: Boolean;
begin
  Result := GetProp('show_time').AsBoolean;
end;

procedure TSeekBar.SetShowTime(Value: Boolean);
begin
  SetProp('show_time', TJSONBoolean.Create(Value));
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

function TSeekBar.GetAlign: string;
begin
  Result := AsText(GetProp('align'));
end;

procedure TSeekBar.SetAlign(Value: string);
begin
  SetProp('align', TJSONString.Create(Value));
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

function TSeekBar.GetCapturesWheel: Boolean;
begin
  Result := GetProp('captures_wheel').AsBoolean;
end;

procedure TSeekBar.SetCapturesWheel(Value: Boolean);
begin
  SetProp('captures_wheel', TJSONBoolean.Create(Value));
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

function TSeekBar.GetDragImage: string;
begin
  Result := AsText(GetProp('drag_image'));
end;

procedure TSeekBar.SetDragImage(Value: string);
begin
  SetProp('drag_image', TJSONString.Create(Value));
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

function TSeekBar.GetCursor: string;
begin
  Result := AsText(GetProp('cursor'));
end;

procedure TSeekBar.SetCursor(Value: string);
begin
  SetProp('cursor', TJSONString.Create(Value));
end;

function TSeekBar.GetOpacity: Double;
begin
  Result := GetProp('opacity').AsFloat;
end;

procedure TSeekBar.SetOpacity(Value: Double);
begin
  SetProp('opacity', TJSONFloatNumber.Create(Value));
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

function TSeekBar.GetSelfAlign: string;
begin
  Result := AsText(GetProp('self_align'));
end;

procedure TSeekBar.SetSelfAlign(Value: string);
begin
  SetProp('self_align', TJSONString.Create(Value));
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

function TSeekBar.GetShadow: string;
begin
  Result := AsText(GetProp('shadow'));
end;

procedure TSeekBar.SetShadow(Value: string);
begin
  SetProp('shadow', TJSONString.Create(Value));
end;

function TSeekBar.GetStyle: string;
begin
  Result := AsText(GetProp('style'));
end;

procedure TSeekBar.SetStyle(Value: string);
begin
  SetProp('style', TJSONString.Create(Value));
end;

function TSeekBar.GetTheme: string;
begin
  Result := AsText(GetProp('theme'));
end;

procedure TSeekBar.SetTheme(Value: string);
begin
  SetProp('theme', TJSONString.Create(Value));
end;

function TSeekBar.GetFill: string;
begin
  Result := AsText(GetProp('fill'));
end;

procedure TSeekBar.SetFill(Value: string);
begin
  SetProp('fill', TJSONString.Create(Value));
end;

function TSeekBar.GetBorderColor: string;
begin
  Result := AsText(GetProp('border_color'));
end;

procedure TSeekBar.SetBorderColor(Value: string);
begin
  SetProp('border_color', TJSONString.Create(Value));
end;

function TSeekBar.GetTextColor: string;
begin
  Result := AsText(GetProp('text_color'));
end;

procedure TSeekBar.SetTextColor(Value: string);
begin
  SetProp('text_color', TJSONString.Create(Value));
end;

function TSeekBar.GetAccentColor: string;
begin
  Result := AsText(GetProp('accent_color'));
end;

procedure TSeekBar.SetAccentColor(Value: string);
begin
  SetProp('accent_color', TJSONString.Create(Value));
end;

function TSeekBar.GetSuccessColor: string;
begin
  Result := AsText(GetProp('success_color'));
end;

procedure TSeekBar.SetSuccessColor(Value: string);
begin
  SetProp('success_color', TJSONString.Create(Value));
end;

function TSeekBar.GetWarningColor: string;
begin
  Result := AsText(GetProp('warning_color'));
end;

procedure TSeekBar.SetWarningColor(Value: string);
begin
  SetProp('warning_color', TJSONString.Create(Value));
end;

function TSeekBar.GetDangerColor: string;
begin
  Result := AsText(GetProp('danger_color'));
end;

procedure TSeekBar.SetDangerColor(Value: string);
begin
  SetProp('danger_color', TJSONString.Create(Value));
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

function TSeekBar.GetFontWeight: string;
begin
  Result := AsText(GetProp('font_weight'));
end;

procedure TSeekBar.SetFontWeight(Value: string);
begin
  SetProp('font_weight', TJSONString.Create(Value));
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

function TSeekBar.GetPopupMenu: string;
begin
  Result := AsText(GetProp('popup_menu'));
end;

procedure TSeekBar.SetPopupMenu(Value: string);
begin
  SetProp('popup_menu', TJSONString.Create(Value));
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

function TTimer.GetPrecision: string;
begin
  Result := AsText(GetProp('precision'));
end;

procedure TTimer.SetPrecision(Value: string);
begin
  SetProp('precision', TJSONString.Create(Value));
end;

function TTimer.GetBackground: Boolean;
begin
  Result := GetProp('background').AsBoolean;
end;

procedure TTimer.SetBackground(Value: Boolean);
begin
  SetProp('background', TJSONBoolean.Create(Value));
end;

function TTimer.GetDrift: Double;
begin
  Result := GetProp('drift').AsFloat;
end;

procedure TTimer.SetDrift(Value: Double);
begin
  SetProp('drift', TJSONFloatNumber.Create(Value));
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

function TTrackBar.GetAlign: string;
begin
  Result := AsText(GetProp('align'));
end;

procedure TTrackBar.SetAlign(Value: string);
begin
  SetProp('align', TJSONString.Create(Value));
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

function TTrackBar.GetCapturesWheel: Boolean;
begin
  Result := GetProp('captures_wheel').AsBoolean;
end;

procedure TTrackBar.SetCapturesWheel(Value: Boolean);
begin
  SetProp('captures_wheel', TJSONBoolean.Create(Value));
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

function TTrackBar.GetDragImage: string;
begin
  Result := AsText(GetProp('drag_image'));
end;

procedure TTrackBar.SetDragImage(Value: string);
begin
  SetProp('drag_image', TJSONString.Create(Value));
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

function TTrackBar.GetCursor: string;
begin
  Result := AsText(GetProp('cursor'));
end;

procedure TTrackBar.SetCursor(Value: string);
begin
  SetProp('cursor', TJSONString.Create(Value));
end;

function TTrackBar.GetOpacity: Double;
begin
  Result := GetProp('opacity').AsFloat;
end;

procedure TTrackBar.SetOpacity(Value: Double);
begin
  SetProp('opacity', TJSONFloatNumber.Create(Value));
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

function TTrackBar.GetSelfAlign: string;
begin
  Result := AsText(GetProp('self_align'));
end;

procedure TTrackBar.SetSelfAlign(Value: string);
begin
  SetProp('self_align', TJSONString.Create(Value));
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

function TTrackBar.GetShadow: string;
begin
  Result := AsText(GetProp('shadow'));
end;

procedure TTrackBar.SetShadow(Value: string);
begin
  SetProp('shadow', TJSONString.Create(Value));
end;

function TTrackBar.GetStyle: string;
begin
  Result := AsText(GetProp('style'));
end;

procedure TTrackBar.SetStyle(Value: string);
begin
  SetProp('style', TJSONString.Create(Value));
end;

function TTrackBar.GetTheme: string;
begin
  Result := AsText(GetProp('theme'));
end;

procedure TTrackBar.SetTheme(Value: string);
begin
  SetProp('theme', TJSONString.Create(Value));
end;

function TTrackBar.GetFill: string;
begin
  Result := AsText(GetProp('fill'));
end;

procedure TTrackBar.SetFill(Value: string);
begin
  SetProp('fill', TJSONString.Create(Value));
end;

function TTrackBar.GetBorderColor: string;
begin
  Result := AsText(GetProp('border_color'));
end;

procedure TTrackBar.SetBorderColor(Value: string);
begin
  SetProp('border_color', TJSONString.Create(Value));
end;

function TTrackBar.GetTextColor: string;
begin
  Result := AsText(GetProp('text_color'));
end;

procedure TTrackBar.SetTextColor(Value: string);
begin
  SetProp('text_color', TJSONString.Create(Value));
end;

function TTrackBar.GetAccentColor: string;
begin
  Result := AsText(GetProp('accent_color'));
end;

procedure TTrackBar.SetAccentColor(Value: string);
begin
  SetProp('accent_color', TJSONString.Create(Value));
end;

function TTrackBar.GetSuccessColor: string;
begin
  Result := AsText(GetProp('success_color'));
end;

procedure TTrackBar.SetSuccessColor(Value: string);
begin
  SetProp('success_color', TJSONString.Create(Value));
end;

function TTrackBar.GetWarningColor: string;
begin
  Result := AsText(GetProp('warning_color'));
end;

procedure TTrackBar.SetWarningColor(Value: string);
begin
  SetProp('warning_color', TJSONString.Create(Value));
end;

function TTrackBar.GetDangerColor: string;
begin
  Result := AsText(GetProp('danger_color'));
end;

procedure TTrackBar.SetDangerColor(Value: string);
begin
  SetProp('danger_color', TJSONString.Create(Value));
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

function TTrackBar.GetFontWeight: string;
begin
  Result := AsText(GetProp('font_weight'));
end;

procedure TTrackBar.SetFontWeight(Value: string);
begin
  SetProp('font_weight', TJSONString.Create(Value));
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

function TTrackBar.GetPopupMenu: string;
begin
  Result := AsText(GetProp('popup_menu'));
end;

procedure TTrackBar.SetPopupMenu(Value: string);
begin
  SetProp('popup_menu', TJSONString.Create(Value));
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

function TVideoPlayer.GetFit: string;
begin
  Result := AsText(GetProp('fit'));
end;

procedure TVideoPlayer.SetFit(Value: string);
begin
  SetProp('fit', TJSONString.Create(Value));
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

function TVideoPlayer.GetOutput: string;
begin
  Result := AsText(GetProp('output'));
end;

procedure TVideoPlayer.SetOutput(Value: string);
begin
  SetProp('output', TJSONString.Create(Value));
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

function TVideoPlayer.GetAlign: string;
begin
  Result := AsText(GetProp('align'));
end;

procedure TVideoPlayer.SetAlign(Value: string);
begin
  SetProp('align', TJSONString.Create(Value));
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

function TVideoPlayer.GetCapturesWheel: Boolean;
begin
  Result := GetProp('captures_wheel').AsBoolean;
end;

procedure TVideoPlayer.SetCapturesWheel(Value: Boolean);
begin
  SetProp('captures_wheel', TJSONBoolean.Create(Value));
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

function TVideoPlayer.GetDragImage: string;
begin
  Result := AsText(GetProp('drag_image'));
end;

procedure TVideoPlayer.SetDragImage(Value: string);
begin
  SetProp('drag_image', TJSONString.Create(Value));
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

function TVideoPlayer.GetCursor: string;
begin
  Result := AsText(GetProp('cursor'));
end;

procedure TVideoPlayer.SetCursor(Value: string);
begin
  SetProp('cursor', TJSONString.Create(Value));
end;

function TVideoPlayer.GetOpacity: Double;
begin
  Result := GetProp('opacity').AsFloat;
end;

procedure TVideoPlayer.SetOpacity(Value: Double);
begin
  SetProp('opacity', TJSONFloatNumber.Create(Value));
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

function TVideoPlayer.GetSelfAlign: string;
begin
  Result := AsText(GetProp('self_align'));
end;

procedure TVideoPlayer.SetSelfAlign(Value: string);
begin
  SetProp('self_align', TJSONString.Create(Value));
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

function TVideoPlayer.GetShadow: string;
begin
  Result := AsText(GetProp('shadow'));
end;

procedure TVideoPlayer.SetShadow(Value: string);
begin
  SetProp('shadow', TJSONString.Create(Value));
end;

function TVideoPlayer.GetStyle: string;
begin
  Result := AsText(GetProp('style'));
end;

procedure TVideoPlayer.SetStyle(Value: string);
begin
  SetProp('style', TJSONString.Create(Value));
end;

function TVideoPlayer.GetTheme: string;
begin
  Result := AsText(GetProp('theme'));
end;

procedure TVideoPlayer.SetTheme(Value: string);
begin
  SetProp('theme', TJSONString.Create(Value));
end;

function TVideoPlayer.GetFill: string;
begin
  Result := AsText(GetProp('fill'));
end;

procedure TVideoPlayer.SetFill(Value: string);
begin
  SetProp('fill', TJSONString.Create(Value));
end;

function TVideoPlayer.GetBorderColor: string;
begin
  Result := AsText(GetProp('border_color'));
end;

procedure TVideoPlayer.SetBorderColor(Value: string);
begin
  SetProp('border_color', TJSONString.Create(Value));
end;

function TVideoPlayer.GetTextColor: string;
begin
  Result := AsText(GetProp('text_color'));
end;

procedure TVideoPlayer.SetTextColor(Value: string);
begin
  SetProp('text_color', TJSONString.Create(Value));
end;

function TVideoPlayer.GetAccentColor: string;
begin
  Result := AsText(GetProp('accent_color'));
end;

procedure TVideoPlayer.SetAccentColor(Value: string);
begin
  SetProp('accent_color', TJSONString.Create(Value));
end;

function TVideoPlayer.GetSuccessColor: string;
begin
  Result := AsText(GetProp('success_color'));
end;

procedure TVideoPlayer.SetSuccessColor(Value: string);
begin
  SetProp('success_color', TJSONString.Create(Value));
end;

function TVideoPlayer.GetWarningColor: string;
begin
  Result := AsText(GetProp('warning_color'));
end;

procedure TVideoPlayer.SetWarningColor(Value: string);
begin
  SetProp('warning_color', TJSONString.Create(Value));
end;

function TVideoPlayer.GetDangerColor: string;
begin
  Result := AsText(GetProp('danger_color'));
end;

procedure TVideoPlayer.SetDangerColor(Value: string);
begin
  SetProp('danger_color', TJSONString.Create(Value));
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

function TVideoPlayer.GetFontWeight: string;
begin
  Result := AsText(GetProp('font_weight'));
end;

procedure TVideoPlayer.SetFontWeight(Value: string);
begin
  SetProp('font_weight', TJSONString.Create(Value));
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

function TVideoPlayer.GetPopupMenu: string;
begin
  Result := AsText(GetProp('popup_menu'));
end;

procedure TVideoPlayer.SetPopupMenu(Value: string);
begin
  SetProp('popup_menu', TJSONString.Create(Value));
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

end.
