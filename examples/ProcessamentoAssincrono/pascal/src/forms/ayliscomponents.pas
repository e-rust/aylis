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
    function GetDefaultButton: Boolean;
    procedure SetDefaultButton(Value: Boolean);
    function GetCancelButton: Boolean;
    procedure SetCancelButton(Value: Boolean);
    function GetHAlign: string;
    procedure SetHAlign(Value: string);
    function GetGroupIndex: Int64;
    procedure SetGroupIndex(Value: Int64);
    function GetRadius: string;
    procedure SetRadius(Value: string);
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
    function GetAnchors: string;
    procedure SetAnchors(Value: string);
    function GetMargins: string;
    procedure SetMargins(Value: string);
    function GetPadding: string;
    procedure SetPadding(Value: string);
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
    function GetBlendMode: string;
    procedure SetBlendMode(Value: string);
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
    function GetSelfAlign: string;
    procedure SetSelfAlign(Value: string);
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
    function GetTagString: string;
    procedure SetTagString(Value: string);
    function GetAccessibleName: string;
    procedure SetAccessibleName(Value: string);
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
    property DefaultButton: Boolean read GetDefaultButton write SetDefaultButton;
    property CancelButton: Boolean read GetCancelButton write SetCancelButton;
    property HAlign: string read GetHAlign write SetHAlign;
    property GroupIndex: Int64 read GetGroupIndex write SetGroupIndex;
    property Radius: string read GetRadius write SetRadius;
    property Left: Double read GetLeft write SetLeft;
    property Top: Double read GetTop write SetTop;
    property Width: Double read GetWidth write SetWidth;
    property Height: Double read GetHeight write SetHeight;
    property Align: string read GetAlign write SetAlign;
    property Anchors: string read GetAnchors write SetAnchors;
    property Margins: string read GetMargins write SetMargins;
    property Padding: string read GetPadding write SetPadding;
    property Visible: Boolean read GetVisible write SetVisible;
    property Enabled: Boolean read GetEnabled write SetEnabled;
    property HitTest: Boolean read GetHitTest write SetHitTest;
    property ClipChildren: Boolean read GetClipChildren write SetClipChildren;
    property InheritState: Boolean read GetInheritState write SetInheritState;
    property Translate: Boolean read GetTranslate write SetTranslate;
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
    property BlendMode: string read GetBlendMode write SetBlendMode;
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
    property SelfAlign: string read GetSelfAlign write SetSelfAlign;
    property AutoSize: Boolean read GetAutoSize write SetAutoSize;
    property ColumnSpan: Int64 read GetColumnSpan write SetColumnSpan;
    property GridRow: Int64 read GetGridRow write SetGridRow;
    property GridColumn: Int64 read GetGridColumn write SetGridColumn;
    property RowSpan: Int64 read GetRowSpan write SetRowSpan;
    property Grow: Double read GetGrow write SetGrow;
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
    property TagString: string read GetTagString write SetTagString;
    property AccessibleName: string read GetAccessibleName write SetAccessibleName;
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
    function GetWindowCorners: string;
    procedure SetWindowCorners(Value: string);
    function GetWindowState: string;
    procedure SetWindowState(Value: string);
    function GetTransition: string;
    procedure SetTransition(Value: string);
    function GetLayerCache: string;
    procedure SetLayerCache(Value: string);
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
    function GetGpu: string;
    procedure SetGpu(Value: string);
    function GetSpanMonitors: Boolean;
    procedure SetSpanMonitors(Value: Boolean);
    function GetActivateOnShow: Boolean;
    procedure SetActivateOnShow(Value: Boolean);
    function GetCloseAction: string;
    procedure SetCloseAction(Value: string);
    function GetFrameRate: Double;
    procedure SetFrameRate(Value: Double);
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
    function GetAlign: string;
    procedure SetAlign(Value: string);
    function GetAnchors: string;
    procedure SetAnchors(Value: string);
    function GetMargins: string;
    procedure SetMargins(Value: string);
    function GetPadding: string;
    procedure SetPadding(Value: string);
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
    function GetBlendMode: string;
    procedure SetBlendMode(Value: string);
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
    function GetSelfAlign: string;
    procedure SetSelfAlign(Value: string);
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
    function GetTagString: string;
    procedure SetTagString(Value: string);
    function GetAccessibleName: string;
    procedure SetAccessibleName(Value: string);
    function GetPopupMenu: string;
    procedure SetPopupMenu(Value: string);
  public
    procedure MoveToMonitor(AMonitor: Int64);
    function GpuAdapter: string;
    function GpuAdapters: string;
    function Focused: string;
    procedure FocusNext;
    procedure FocusPrev;
    property Caption: string read GetCaption write SetCaption;
    property Position: string read GetPosition write SetPosition;
    property BorderStyle: string read GetBorderStyle write SetBorderStyle;
    property WindowCorners: string read GetWindowCorners write SetWindowCorners;
    property WindowState: string read GetWindowState write SetWindowState;
    property Transition: string read GetTransition write SetTransition;
    property LayerCache: string read GetLayerCache write SetLayerCache;
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
    property Gpu: string read GetGpu write SetGpu;
    property SpanMonitors: Boolean read GetSpanMonitors write SetSpanMonitors;
    property ActivateOnShow: Boolean read GetActivateOnShow write SetActivateOnShow;
    property CloseAction: string read GetCloseAction write SetCloseAction;
    property FrameRate: Double read GetFrameRate write SetFrameRate;
    property ClientWidth: Double read GetClientWidth write SetClientWidth;
    property ClientHeight: Double read GetClientHeight write SetClientHeight;
    property Left: Double read GetLeft write SetLeft;
    property Top: Double read GetTop write SetTop;
    property Width: Double read GetWidth write SetWidth;
    property Height: Double read GetHeight write SetHeight;
    property Align: string read GetAlign write SetAlign;
    property Anchors: string read GetAnchors write SetAnchors;
    property Margins: string read GetMargins write SetMargins;
    property Padding: string read GetPadding write SetPadding;
    property Visible: Boolean read GetVisible write SetVisible;
    property Enabled: Boolean read GetEnabled write SetEnabled;
    property HitTest: Boolean read GetHitTest write SetHitTest;
    property ClipChildren: Boolean read GetClipChildren write SetClipChildren;
    property InheritState: Boolean read GetInheritState write SetInheritState;
    property Translate: Boolean read GetTranslate write SetTranslate;
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
    property BlendMode: string read GetBlendMode write SetBlendMode;
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
    property SelfAlign: string read GetSelfAlign write SetSelfAlign;
    property AutoSize: Boolean read GetAutoSize write SetAutoSize;
    property ColumnSpan: Int64 read GetColumnSpan write SetColumnSpan;
    property GridRow: Int64 read GetGridRow write SetGridRow;
    property GridColumn: Int64 read GetGridColumn write SetGridColumn;
    property RowSpan: Int64 read GetRowSpan write SetRowSpan;
    property Grow: Double read GetGrow write SetGrow;
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
    property TagString: string read GetTagString write SetTagString;
    property AccessibleName: string read GetAccessibleName write SetAccessibleName;
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
    function GetLineHeight: Double;
    procedure SetLineHeight(Value: Double);
    function GetTextShadow: string;
    procedure SetTextShadow(Value: string);
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
    function GetAnchors: string;
    procedure SetAnchors(Value: string);
    function GetMargins: string;
    procedure SetMargins(Value: string);
    function GetPadding: string;
    procedure SetPadding(Value: string);
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
    function GetBlendMode: string;
    procedure SetBlendMode(Value: string);
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
    function GetSelfAlign: string;
    procedure SetSelfAlign(Value: string);
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
    function GetTagString: string;
    procedure SetTagString(Value: string);
    function GetAccessibleName: string;
    procedure SetAccessibleName(Value: string);
    function GetPopupMenu: string;
    procedure SetPopupMenu(Value: string);
  public
    function MinContentWidth: Double;
    function MaxContentWidth: Double;
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
    property LineHeight: Double read GetLineHeight write SetLineHeight;
    property TextShadow: string read GetTextShadow write SetTextShadow;
    property FocusControl: string read GetFocusControl write SetFocusControl;
    property Left: Double read GetLeft write SetLeft;
    property Top: Double read GetTop write SetTop;
    property Width: Double read GetWidth write SetWidth;
    property Height: Double read GetHeight write SetHeight;
    property Align: string read GetAlign write SetAlign;
    property Anchors: string read GetAnchors write SetAnchors;
    property Margins: string read GetMargins write SetMargins;
    property Padding: string read GetPadding write SetPadding;
    property Visible: Boolean read GetVisible write SetVisible;
    property Enabled: Boolean read GetEnabled write SetEnabled;
    property HitTest: Boolean read GetHitTest write SetHitTest;
    property ClipChildren: Boolean read GetClipChildren write SetClipChildren;
    property InheritState: Boolean read GetInheritState write SetInheritState;
    property Translate: Boolean read GetTranslate write SetTranslate;
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
    property BlendMode: string read GetBlendMode write SetBlendMode;
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
    property SelfAlign: string read GetSelfAlign write SetSelfAlign;
    property AutoSize: Boolean read GetAutoSize write SetAutoSize;
    property ColumnSpan: Int64 read GetColumnSpan write SetColumnSpan;
    property GridRow: Int64 read GetGridRow write SetGridRow;
    property GridColumn: Int64 read GetGridColumn write SetGridColumn;
    property RowSpan: Int64 read GetRowSpan write SetRowSpan;
    property Grow: Double read GetGrow write SetGrow;
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
    property TagString: string read GetTagString write SetTagString;
    property AccessibleName: string read GetAccessibleName write SetAccessibleName;
    property PopupMenu: string read GetPopupMenu write SetPopupMenu;
  end;

  { Um ProgressBar de um form. }
  TProgressBar = class(TAylisComponent)
  private
    function GetValue: Double;
    procedure SetValue(Value: Double);
    function GetMin: Double;
    procedure SetMin(Value: Double);
    function GetMax: Double;
    procedure SetMax(Value: Double);
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
    function GetAnchors: string;
    procedure SetAnchors(Value: string);
    function GetMargins: string;
    procedure SetMargins(Value: string);
    function GetPadding: string;
    procedure SetPadding(Value: string);
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
    function GetBlendMode: string;
    procedure SetBlendMode(Value: string);
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
    function GetSelfAlign: string;
    procedure SetSelfAlign(Value: string);
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
    function GetTagString: string;
    procedure SetTagString(Value: string);
    function GetAccessibleName: string;
    procedure SetAccessibleName(Value: string);
    function GetPopupMenu: string;
    procedure SetPopupMenu(Value: string);
  public
    property Value: Double read GetValue write SetValue;
    property Min: Double read GetMin write SetMin;
    property Max: Double read GetMax write SetMax;
    property Color: string read GetColor write SetColor;
    property Left: Double read GetLeft write SetLeft;
    property Top: Double read GetTop write SetTop;
    property Width: Double read GetWidth write SetWidth;
    property Height: Double read GetHeight write SetHeight;
    property Align: string read GetAlign write SetAlign;
    property Anchors: string read GetAnchors write SetAnchors;
    property Margins: string read GetMargins write SetMargins;
    property Padding: string read GetPadding write SetPadding;
    property Visible: Boolean read GetVisible write SetVisible;
    property Enabled: Boolean read GetEnabled write SetEnabled;
    property HitTest: Boolean read GetHitTest write SetHitTest;
    property ClipChildren: Boolean read GetClipChildren write SetClipChildren;
    property InheritState: Boolean read GetInheritState write SetInheritState;
    property Translate: Boolean read GetTranslate write SetTranslate;
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
    property BlendMode: string read GetBlendMode write SetBlendMode;
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
    property SelfAlign: string read GetSelfAlign write SetSelfAlign;
    property AutoSize: Boolean read GetAutoSize write SetAutoSize;
    property ColumnSpan: Int64 read GetColumnSpan write SetColumnSpan;
    property GridRow: Int64 read GetGridRow write SetGridRow;
    property GridColumn: Int64 read GetGridColumn write SetGridColumn;
    property RowSpan: Int64 read GetRowSpan write SetRowSpan;
    property Grow: Double read GetGrow write SetGrow;
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
    property TagString: string read GetTagString write SetTagString;
    property AccessibleName: string read GetAccessibleName write SetAccessibleName;
    property PopupMenu: string read GetPopupMenu write SetPopupMenu;
  end;

  { O form frmPrincipal: a classe base com os componentes. A do form (TFrmPrincipal), com os
    handlers (métodos published) e o estado, fica na unit dele. }
  TFrmPrincipalBase = class(TForm)
  published
    lblTitulo: TLabel;
    btnFecharMes: TButton;
    lblStatus: TLabel;
  protected
    procedure BindComponents(List: TJSONArray); override;
  public
    { O Button deste form com o nome (nil se não há). }
    function FindButton(const Nome: string): TButton;
    { O Button que disparou o evento (nil se não é um Button deste form). }
    function SenderAsButton(Sender: TAylisComponent): TButton; overload;
    function SenderAsButton(Event: TJSONObject): TButton; overload;
    { O Label deste form com o nome (nil se não há). }
    function FindLabel(const Nome: string): TLabel;
    { O Label que disparou o evento (nil se não é um Label deste form). }
    function SenderAsLabel(Sender: TAylisComponent): TLabel; overload;
    function SenderAsLabel(Event: TJSONObject): TLabel; overload;
  end;

  { O form frmProgresso: a classe base com os componentes. A do form (TFrmProgresso), com os
    handlers (métodos published) e o estado, fica na unit dele. }
  TFrmProgressoBase = class(TForm)
  published
    lblEtapa: TLabel;
    pbProgresso: TProgressBar;
    btnCancelar: TButton;
  protected
    procedure BindComponents(List: TJSONArray); override;
  public
    { O Button deste form com o nome (nil se não há). }
    function FindButton(const Nome: string): TButton;
    { O Button que disparou o evento (nil se não é um Button deste form). }
    function SenderAsButton(Sender: TAylisComponent): TButton; overload;
    function SenderAsButton(Event: TJSONObject): TButton; overload;
    { O Label deste form com o nome (nil se não há). }
    function FindLabel(const Nome: string): TLabel;
    { O Label que disparou o evento (nil se não é um Label deste form). }
    function SenderAsLabel(Sender: TAylisComponent): TLabel; overload;
    function SenderAsLabel(Event: TJSONObject): TLabel; overload;
    { O ProgressBar deste form com o nome (nil se não há). }
    function FindProgressBar(const Nome: string): TProgressBar;
    { O ProgressBar que disparou o evento (nil se não é um ProgressBar deste form). }
    function SenderAsProgressBar(Sender: TAylisComponent): TProgressBar; overload;
    function SenderAsProgressBar(Event: TJSONObject): TProgressBar; overload;
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

function TButton.GetHAlign: string;
begin
  Result := AsText(GetProp('h_align'));
end;

procedure TButton.SetHAlign(Value: string);
begin
  SetProp('h_align', TJSONString.Create(Value));
end;

function TButton.GetGroupIndex: Int64;
begin
  Result := GetProp('group_index').AsInt64;
end;

procedure TButton.SetGroupIndex(Value: Int64);
begin
  SetProp('group_index', TJSONInt64Number.Create(Value));
end;

function TButton.GetRadius: string;
begin
  Result := AsText(GetProp('radius'));
end;

procedure TButton.SetRadius(Value: string);
begin
  SetProp('radius', TJSONString.Create(Value));
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

function TButton.GetAnchors: string;
begin
  Result := AsText(GetProp('anchors'));
end;

procedure TButton.SetAnchors(Value: string);
begin
  SetProp('anchors', TJSONString.Create(Value));
end;

function TButton.GetMargins: string;
begin
  Result := AsText(GetProp('margins'));
end;

procedure TButton.SetMargins(Value: string);
begin
  SetProp('margins', TJSONString.Create(Value));
end;

function TButton.GetPadding: string;
begin
  Result := AsText(GetProp('padding'));
end;

procedure TButton.SetPadding(Value: string);
begin
  SetProp('padding', TJSONString.Create(Value));
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

function TButton.GetBlendMode: string;
begin
  Result := AsText(GetProp('blend_mode'));
end;

procedure TButton.SetBlendMode(Value: string);
begin
  SetProp('blend_mode', TJSONString.Create(Value));
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

function TForm.GetWindowCorners: string;
begin
  Result := AsText(GetProp('window_corners'));
end;

procedure TForm.SetWindowCorners(Value: string);
begin
  SetProp('window_corners', TJSONString.Create(Value));
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

function TForm.GetLayerCache: string;
begin
  Result := AsText(GetProp('layer_cache'));
end;

procedure TForm.SetLayerCache(Value: string);
begin
  SetProp('layer_cache', TJSONString.Create(Value));
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

function TForm.GetGpu: string;
begin
  Result := AsText(GetProp('gpu'));
end;

procedure TForm.SetGpu(Value: string);
begin
  SetProp('gpu', TJSONString.Create(Value));
end;

function TForm.GetSpanMonitors: Boolean;
begin
  Result := GetProp('span_monitors').AsBoolean;
end;

procedure TForm.SetSpanMonitors(Value: Boolean);
begin
  SetProp('span_monitors', TJSONBoolean.Create(Value));
end;

function TForm.GetActivateOnShow: Boolean;
begin
  Result := GetProp('activate_on_show').AsBoolean;
end;

procedure TForm.SetActivateOnShow(Value: Boolean);
begin
  SetProp('activate_on_show', TJSONBoolean.Create(Value));
end;

function TForm.GetCloseAction: string;
begin
  Result := AsText(GetProp('close_action'));
end;

procedure TForm.SetCloseAction(Value: string);
begin
  SetProp('close_action', TJSONString.Create(Value));
end;

function TForm.GetFrameRate: Double;
begin
  Result := GetProp('frame_rate').AsFloat;
end;

procedure TForm.SetFrameRate(Value: Double);
begin
  SetProp('frame_rate', TJSONFloatNumber.Create(Value));
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

function TForm.GetAlign: string;
begin
  Result := AsText(GetProp('align'));
end;

procedure TForm.SetAlign(Value: string);
begin
  SetProp('align', TJSONString.Create(Value));
end;

function TForm.GetAnchors: string;
begin
  Result := AsText(GetProp('anchors'));
end;

procedure TForm.SetAnchors(Value: string);
begin
  SetProp('anchors', TJSONString.Create(Value));
end;

function TForm.GetMargins: string;
begin
  Result := AsText(GetProp('margins'));
end;

procedure TForm.SetMargins(Value: string);
begin
  SetProp('margins', TJSONString.Create(Value));
end;

function TForm.GetPadding: string;
begin
  Result := AsText(GetProp('padding'));
end;

procedure TForm.SetPadding(Value: string);
begin
  SetProp('padding', TJSONString.Create(Value));
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

function TForm.GetBlendMode: string;
begin
  Result := AsText(GetProp('blend_mode'));
end;

procedure TForm.SetBlendMode(Value: string);
begin
  SetProp('blend_mode', TJSONString.Create(Value));
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

function TForm.GetPopupMenu: string;
begin
  Result := AsText(GetProp('popup_menu'));
end;

procedure TForm.SetPopupMenu(Value: string);
begin
  SetProp('popup_menu', TJSONString.Create(Value));
end;

procedure TForm.MoveToMonitor(AMonitor: Int64);
begin
  Aylis.Send('invoke', TJSONObject.Create(['target', Id, 'method', 'move_to_monitor', 'args', TJSONArray.Create([AMonitor])]));
end;

function TForm.GpuAdapter: string;
begin
  Result := AsText(Aylis.Call('invoke', TJSONObject.Create(['target', Id, 'method', 'gpu_adapter', 'args', TJSONArray.Create])));
end;

function TForm.GpuAdapters: string;
begin
  Result := AsText(Aylis.Call('invoke', TJSONObject.Create(['target', Id, 'method', 'gpu_adapters', 'args', TJSONArray.Create])));
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

function TLabel.GetLineHeight: Double;
begin
  Result := GetProp('line_height').AsFloat;
end;

procedure TLabel.SetLineHeight(Value: Double);
begin
  SetProp('line_height', TJSONFloatNumber.Create(Value));
end;

function TLabel.GetTextShadow: string;
begin
  Result := AsText(GetProp('text_shadow'));
end;

procedure TLabel.SetTextShadow(Value: string);
begin
  SetProp('text_shadow', TJSONString.Create(Value));
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

function TLabel.GetAnchors: string;
begin
  Result := AsText(GetProp('anchors'));
end;

procedure TLabel.SetAnchors(Value: string);
begin
  SetProp('anchors', TJSONString.Create(Value));
end;

function TLabel.GetMargins: string;
begin
  Result := AsText(GetProp('margins'));
end;

procedure TLabel.SetMargins(Value: string);
begin
  SetProp('margins', TJSONString.Create(Value));
end;

function TLabel.GetPadding: string;
begin
  Result := AsText(GetProp('padding'));
end;

procedure TLabel.SetPadding(Value: string);
begin
  SetProp('padding', TJSONString.Create(Value));
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

function TLabel.GetBlendMode: string;
begin
  Result := AsText(GetProp('blend_mode'));
end;

procedure TLabel.SetBlendMode(Value: string);
begin
  SetProp('blend_mode', TJSONString.Create(Value));
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

function TLabel.GetPopupMenu: string;
begin
  Result := AsText(GetProp('popup_menu'));
end;

procedure TLabel.SetPopupMenu(Value: string);
begin
  SetProp('popup_menu', TJSONString.Create(Value));
end;

function TLabel.MinContentWidth: Double;
begin
  Result := Aylis.Call('invoke', TJSONObject.Create(['target', Id, 'method', 'min_content_width', 'args', TJSONArray.Create])).AsFloat;
end;

function TLabel.MaxContentWidth: Double;
begin
  Result := Aylis.Call('invoke', TJSONObject.Create(['target', Id, 'method', 'max_content_width', 'args', TJSONArray.Create])).AsFloat;
end;

function TProgressBar.GetValue: Double;
begin
  Result := GetProp('value').AsFloat;
end;

procedure TProgressBar.SetValue(Value: Double);
begin
  SetProp('value', TJSONFloatNumber.Create(Value));
end;

function TProgressBar.GetMin: Double;
begin
  Result := GetProp('min').AsFloat;
end;

procedure TProgressBar.SetMin(Value: Double);
begin
  SetProp('min', TJSONFloatNumber.Create(Value));
end;

function TProgressBar.GetMax: Double;
begin
  Result := GetProp('max').AsFloat;
end;

procedure TProgressBar.SetMax(Value: Double);
begin
  SetProp('max', TJSONFloatNumber.Create(Value));
end;

function TProgressBar.GetColor: string;
begin
  Result := AsText(GetProp('color'));
end;

procedure TProgressBar.SetColor(Value: string);
begin
  SetProp('color', TJSONString.Create(Value));
end;

function TProgressBar.GetLeft: Double;
begin
  Result := GetProp('left').AsFloat;
end;

procedure TProgressBar.SetLeft(Value: Double);
begin
  SetProp('left', TJSONFloatNumber.Create(Value));
end;

function TProgressBar.GetTop: Double;
begin
  Result := GetProp('top').AsFloat;
end;

procedure TProgressBar.SetTop(Value: Double);
begin
  SetProp('top', TJSONFloatNumber.Create(Value));
end;

function TProgressBar.GetWidth: Double;
begin
  Result := GetProp('width').AsFloat;
end;

procedure TProgressBar.SetWidth(Value: Double);
begin
  SetProp('width', TJSONFloatNumber.Create(Value));
end;

function TProgressBar.GetHeight: Double;
begin
  Result := GetProp('height').AsFloat;
end;

procedure TProgressBar.SetHeight(Value: Double);
begin
  SetProp('height', TJSONFloatNumber.Create(Value));
end;

function TProgressBar.GetAlign: string;
begin
  Result := AsText(GetProp('align'));
end;

procedure TProgressBar.SetAlign(Value: string);
begin
  SetProp('align', TJSONString.Create(Value));
end;

function TProgressBar.GetAnchors: string;
begin
  Result := AsText(GetProp('anchors'));
end;

procedure TProgressBar.SetAnchors(Value: string);
begin
  SetProp('anchors', TJSONString.Create(Value));
end;

function TProgressBar.GetMargins: string;
begin
  Result := AsText(GetProp('margins'));
end;

procedure TProgressBar.SetMargins(Value: string);
begin
  SetProp('margins', TJSONString.Create(Value));
end;

function TProgressBar.GetPadding: string;
begin
  Result := AsText(GetProp('padding'));
end;

procedure TProgressBar.SetPadding(Value: string);
begin
  SetProp('padding', TJSONString.Create(Value));
end;

function TProgressBar.GetVisible: Boolean;
begin
  Result := GetProp('visible').AsBoolean;
end;

procedure TProgressBar.SetVisible(Value: Boolean);
begin
  SetProp('visible', TJSONBoolean.Create(Value));
end;

function TProgressBar.GetEnabled: Boolean;
begin
  Result := GetProp('enabled').AsBoolean;
end;

procedure TProgressBar.SetEnabled(Value: Boolean);
begin
  SetProp('enabled', TJSONBoolean.Create(Value));
end;

function TProgressBar.GetHitTest: Boolean;
begin
  Result := GetProp('hit_test').AsBoolean;
end;

procedure TProgressBar.SetHitTest(Value: Boolean);
begin
  SetProp('hit_test', TJSONBoolean.Create(Value));
end;

function TProgressBar.GetClipChildren: Boolean;
begin
  Result := GetProp('clip_children').AsBoolean;
end;

procedure TProgressBar.SetClipChildren(Value: Boolean);
begin
  SetProp('clip_children', TJSONBoolean.Create(Value));
end;

function TProgressBar.GetInheritState: Boolean;
begin
  Result := GetProp('inherit_state').AsBoolean;
end;

procedure TProgressBar.SetInheritState(Value: Boolean);
begin
  SetProp('inherit_state', TJSONBoolean.Create(Value));
end;

function TProgressBar.GetTranslate: Boolean;
begin
  Result := GetProp('translate').AsBoolean;
end;

procedure TProgressBar.SetTranslate(Value: Boolean);
begin
  SetProp('translate', TJSONBoolean.Create(Value));
end;

function TProgressBar.GetCapturesWheel: Boolean;
begin
  Result := GetProp('captures_wheel').AsBoolean;
end;

procedure TProgressBar.SetCapturesWheel(Value: Boolean);
begin
  SetProp('captures_wheel', TJSONBoolean.Create(Value));
end;

function TProgressBar.GetCanFocus: Boolean;
begin
  Result := GetProp('can_focus').AsBoolean;
end;

procedure TProgressBar.SetCanFocus(Value: Boolean);
begin
  SetProp('can_focus', TJSONBoolean.Create(Value));
end;

function TProgressBar.GetDraggable: Boolean;
begin
  Result := GetProp('draggable').AsBoolean;
end;

procedure TProgressBar.SetDraggable(Value: Boolean);
begin
  SetProp('draggable', TJSONBoolean.Create(Value));
end;

function TProgressBar.GetAcceptsDrops: Boolean;
begin
  Result := GetProp('accepts_drops').AsBoolean;
end;

procedure TProgressBar.SetAcceptsDrops(Value: Boolean);
begin
  SetProp('accepts_drops', TJSONBoolean.Create(Value));
end;

function TProgressBar.GetDragData: string;
begin
  Result := AsText(GetProp('drag_data'));
end;

procedure TProgressBar.SetDragData(Value: string);
begin
  SetProp('drag_data', TJSONString.Create(Value));
end;

function TProgressBar.GetDragLabel: string;
begin
  Result := AsText(GetProp('drag_label'));
end;

procedure TProgressBar.SetDragLabel(Value: string);
begin
  SetProp('drag_label', TJSONString.Create(Value));
end;

function TProgressBar.GetDragImage: string;
begin
  Result := AsText(GetProp('drag_image'));
end;

procedure TProgressBar.SetDragImage(Value: string);
begin
  SetProp('drag_image', TJSONString.Create(Value));
end;

function TProgressBar.GetDropTypes: string;
begin
  Result := AsText(GetProp('drop_types'));
end;

procedure TProgressBar.SetDropTypes(Value: string);
begin
  SetProp('drop_types', TJSONString.Create(Value));
end;

function TProgressBar.GetTabStop: Boolean;
begin
  Result := GetProp('tab_stop').AsBoolean;
end;

procedure TProgressBar.SetTabStop(Value: Boolean);
begin
  SetProp('tab_stop', TJSONBoolean.Create(Value));
end;

function TProgressBar.GetCursor: string;
begin
  Result := AsText(GetProp('cursor'));
end;

procedure TProgressBar.SetCursor(Value: string);
begin
  SetProp('cursor', TJSONString.Create(Value));
end;

function TProgressBar.GetOpacity: Double;
begin
  Result := GetProp('opacity').AsFloat;
end;

procedure TProgressBar.SetOpacity(Value: Double);
begin
  SetProp('opacity', TJSONFloatNumber.Create(Value));
end;

function TProgressBar.GetBlendMode: string;
begin
  Result := AsText(GetProp('blend_mode'));
end;

procedure TProgressBar.SetBlendMode(Value: string);
begin
  SetProp('blend_mode', TJSONString.Create(Value));
end;

function TProgressBar.GetCacheLayer: Boolean;
begin
  Result := GetProp('cache_layer').AsBoolean;
end;

procedure TProgressBar.SetCacheLayer(Value: Boolean);
begin
  SetProp('cache_layer', TJSONBoolean.Create(Value));
end;

function TProgressBar.GetRotation: Double;
begin
  Result := GetProp('rotation').AsFloat;
end;

procedure TProgressBar.SetRotation(Value: Double);
begin
  SetProp('rotation', TJSONFloatNumber.Create(Value));
end;

function TProgressBar.GetScale: Double;
begin
  Result := GetProp('scale').AsFloat;
end;

procedure TProgressBar.SetScale(Value: Double);
begin
  SetProp('scale', TJSONFloatNumber.Create(Value));
end;

function TProgressBar.GetTranslateX: Double;
begin
  Result := GetProp('translate_x').AsFloat;
end;

procedure TProgressBar.SetTranslateX(Value: Double);
begin
  SetProp('translate_x', TJSONFloatNumber.Create(Value));
end;

function TProgressBar.GetTranslateY: Double;
begin
  Result := GetProp('translate_y').AsFloat;
end;

procedure TProgressBar.SetTranslateY(Value: Double);
begin
  SetProp('translate_y', TJSONFloatNumber.Create(Value));
end;

function TProgressBar.GetAspectRatio: Double;
begin
  Result := GetProp('aspect_ratio').AsFloat;
end;

procedure TProgressBar.SetAspectRatio(Value: Double);
begin
  SetProp('aspect_ratio', TJSONFloatNumber.Create(Value));
end;

function TProgressBar.GetMaxWidth: Double;
begin
  Result := GetProp('max_width').AsFloat;
end;

procedure TProgressBar.SetMaxWidth(Value: Double);
begin
  SetProp('max_width', TJSONFloatNumber.Create(Value));
end;

function TProgressBar.GetMinWidth: Double;
begin
  Result := GetProp('min_width').AsFloat;
end;

procedure TProgressBar.SetMinWidth(Value: Double);
begin
  SetProp('min_width', TJSONFloatNumber.Create(Value));
end;

function TProgressBar.GetMinHeight: Double;
begin
  Result := GetProp('min_height').AsFloat;
end;

procedure TProgressBar.SetMinHeight(Value: Double);
begin
  SetProp('min_height', TJSONFloatNumber.Create(Value));
end;

function TProgressBar.GetMaxHeight: Double;
begin
  Result := GetProp('max_height').AsFloat;
end;

procedure TProgressBar.SetMaxHeight(Value: Double);
begin
  SetProp('max_height', TJSONFloatNumber.Create(Value));
end;

function TProgressBar.GetSelfAlign: string;
begin
  Result := AsText(GetProp('self_align'));
end;

procedure TProgressBar.SetSelfAlign(Value: string);
begin
  SetProp('self_align', TJSONString.Create(Value));
end;

function TProgressBar.GetAutoSize: Boolean;
begin
  Result := GetProp('auto_size').AsBoolean;
end;

procedure TProgressBar.SetAutoSize(Value: Boolean);
begin
  SetProp('auto_size', TJSONBoolean.Create(Value));
end;

function TProgressBar.GetColumnSpan: Int64;
begin
  Result := GetProp('column_span').AsInt64;
end;

procedure TProgressBar.SetColumnSpan(Value: Int64);
begin
  SetProp('column_span', TJSONInt64Number.Create(Value));
end;

function TProgressBar.GetGridRow: Int64;
begin
  Result := GetProp('grid_row').AsInt64;
end;

procedure TProgressBar.SetGridRow(Value: Int64);
begin
  SetProp('grid_row', TJSONInt64Number.Create(Value));
end;

function TProgressBar.GetGridColumn: Int64;
begin
  Result := GetProp('grid_column').AsInt64;
end;

procedure TProgressBar.SetGridColumn(Value: Int64);
begin
  SetProp('grid_column', TJSONInt64Number.Create(Value));
end;

function TProgressBar.GetRowSpan: Int64;
begin
  Result := GetProp('row_span').AsInt64;
end;

procedure TProgressBar.SetRowSpan(Value: Int64);
begin
  SetProp('row_span', TJSONInt64Number.Create(Value));
end;

function TProgressBar.GetGrow: Double;
begin
  Result := GetProp('grow').AsFloat;
end;

procedure TProgressBar.SetGrow(Value: Double);
begin
  SetProp('grow', TJSONFloatNumber.Create(Value));
end;

function TProgressBar.GetShadow: string;
begin
  Result := AsText(GetProp('shadow'));
end;

procedure TProgressBar.SetShadow(Value: string);
begin
  SetProp('shadow', TJSONString.Create(Value));
end;

function TProgressBar.GetStyle: string;
begin
  Result := AsText(GetProp('style'));
end;

procedure TProgressBar.SetStyle(Value: string);
begin
  SetProp('style', TJSONString.Create(Value));
end;

function TProgressBar.GetTheme: string;
begin
  Result := AsText(GetProp('theme'));
end;

procedure TProgressBar.SetTheme(Value: string);
begin
  SetProp('theme', TJSONString.Create(Value));
end;

function TProgressBar.GetFill: string;
begin
  Result := AsText(GetProp('fill'));
end;

procedure TProgressBar.SetFill(Value: string);
begin
  SetProp('fill', TJSONString.Create(Value));
end;

function TProgressBar.GetBorderColor: string;
begin
  Result := AsText(GetProp('border_color'));
end;

procedure TProgressBar.SetBorderColor(Value: string);
begin
  SetProp('border_color', TJSONString.Create(Value));
end;

function TProgressBar.GetTextColor: string;
begin
  Result := AsText(GetProp('text_color'));
end;

procedure TProgressBar.SetTextColor(Value: string);
begin
  SetProp('text_color', TJSONString.Create(Value));
end;

function TProgressBar.GetAccentColor: string;
begin
  Result := AsText(GetProp('accent_color'));
end;

procedure TProgressBar.SetAccentColor(Value: string);
begin
  SetProp('accent_color', TJSONString.Create(Value));
end;

function TProgressBar.GetSuccessColor: string;
begin
  Result := AsText(GetProp('success_color'));
end;

procedure TProgressBar.SetSuccessColor(Value: string);
begin
  SetProp('success_color', TJSONString.Create(Value));
end;

function TProgressBar.GetWarningColor: string;
begin
  Result := AsText(GetProp('warning_color'));
end;

procedure TProgressBar.SetWarningColor(Value: string);
begin
  SetProp('warning_color', TJSONString.Create(Value));
end;

function TProgressBar.GetDangerColor: string;
begin
  Result := AsText(GetProp('danger_color'));
end;

procedure TProgressBar.SetDangerColor(Value: string);
begin
  SetProp('danger_color', TJSONString.Create(Value));
end;

function TProgressBar.GetFontFamily: string;
begin
  Result := AsText(GetProp('font_family'));
end;

procedure TProgressBar.SetFontFamily(Value: string);
begin
  SetProp('font_family', TJSONString.Create(Value));
end;

function TProgressBar.GetFontSize: Double;
begin
  Result := GetProp('font_size').AsFloat;
end;

procedure TProgressBar.SetFontSize(Value: Double);
begin
  SetProp('font_size', TJSONFloatNumber.Create(Value));
end;

function TProgressBar.GetFontWeight: string;
begin
  Result := AsText(GetProp('font_weight'));
end;

procedure TProgressBar.SetFontWeight(Value: string);
begin
  SetProp('font_weight', TJSONString.Create(Value));
end;

function TProgressBar.GetFontItalic: Boolean;
begin
  Result := GetProp('font_italic').AsBoolean;
end;

procedure TProgressBar.SetFontItalic(Value: Boolean);
begin
  SetProp('font_italic', TJSONBoolean.Create(Value));
end;

function TProgressBar.GetFontUnderline: Boolean;
begin
  Result := GetProp('font_underline').AsBoolean;
end;

procedure TProgressBar.SetFontUnderline(Value: Boolean);
begin
  SetProp('font_underline', TJSONBoolean.Create(Value));
end;

function TProgressBar.GetFontStrikeout: Boolean;
begin
  Result := GetProp('font_strikeout').AsBoolean;
end;

procedure TProgressBar.SetFontStrikeout(Value: Boolean);
begin
  SetProp('font_strikeout', TJSONBoolean.Create(Value));
end;

function TProgressBar.GetHint: string;
begin
  Result := AsText(GetProp('hint'));
end;

procedure TProgressBar.SetHint(Value: string);
begin
  SetProp('hint', TJSONString.Create(Value));
end;

function TProgressBar.GetTag: Int64;
begin
  Result := GetProp('tag').AsInt64;
end;

procedure TProgressBar.SetTag(Value: Int64);
begin
  SetProp('tag', TJSONInt64Number.Create(Value));
end;

function TProgressBar.GetTagString: string;
begin
  Result := AsText(GetProp('tag_string'));
end;

procedure TProgressBar.SetTagString(Value: string);
begin
  SetProp('tag_string', TJSONString.Create(Value));
end;

function TProgressBar.GetAccessibleName: string;
begin
  Result := AsText(GetProp('accessible_name'));
end;

procedure TProgressBar.SetAccessibleName(Value: string);
begin
  SetProp('accessible_name', TJSONString.Create(Value));
end;

function TProgressBar.GetPopupMenu: string;
begin
  Result := AsText(GetProp('popup_menu'));
end;

procedure TProgressBar.SetPopupMenu(Value: string);
begin
  SetProp('popup_menu', TJSONString.Create(Value));
end;

procedure TFrmPrincipalBase.BindComponents(List: TJSONArray);
begin
  lblTitulo := TLabel(Resolve(List, 'lblTitulo', 'Label', TLabel));
  btnFecharMes := TButton(Resolve(List, 'btnFecharMes', 'Button', TButton));
  lblStatus := TLabel(Resolve(List, 'lblStatus', 'Label', TLabel));
end;

function TFrmPrincipalBase.FindButton(const Nome: string): TButton;
begin
  Result := nil;
  if Nome = 'btnFecharMes' then
    Result := btnFecharMes;
end;

function TFrmPrincipalBase.SenderAsButton(Sender: TAylisComponent): TButton;
begin
  Result := nil;
  if Sender <> nil then
    Result := FindButton(Sender.Name);
end;

function TFrmPrincipalBase.SenderAsButton(Event: TJSONObject): TButton;
begin
  Result := FindButton(TJSONObject(Event.Find('sender')).Get('name', ''));
end;

function TFrmPrincipalBase.FindLabel(const Nome: string): TLabel;
begin
  Result := nil;
  if Nome = 'lblTitulo' then
    Result := lblTitulo;
  if Nome = 'lblStatus' then
    Result := lblStatus;
end;

function TFrmPrincipalBase.SenderAsLabel(Sender: TAylisComponent): TLabel;
begin
  Result := nil;
  if Sender <> nil then
    Result := FindLabel(Sender.Name);
end;

function TFrmPrincipalBase.SenderAsLabel(Event: TJSONObject): TLabel;
begin
  Result := FindLabel(TJSONObject(Event.Find('sender')).Get('name', ''));
end;

procedure TFrmProgressoBase.BindComponents(List: TJSONArray);
begin
  lblEtapa := TLabel(Resolve(List, 'lblEtapa', 'Label', TLabel));
  pbProgresso := TProgressBar(Resolve(List, 'pbProgresso', 'ProgressBar', TProgressBar));
  btnCancelar := TButton(Resolve(List, 'btnCancelar', 'Button', TButton));
end;

function TFrmProgressoBase.FindButton(const Nome: string): TButton;
begin
  Result := nil;
  if Nome = 'btnCancelar' then
    Result := btnCancelar;
end;

function TFrmProgressoBase.SenderAsButton(Sender: TAylisComponent): TButton;
begin
  Result := nil;
  if Sender <> nil then
    Result := FindButton(Sender.Name);
end;

function TFrmProgressoBase.SenderAsButton(Event: TJSONObject): TButton;
begin
  Result := FindButton(TJSONObject(Event.Find('sender')).Get('name', ''));
end;

function TFrmProgressoBase.FindLabel(const Nome: string): TLabel;
begin
  Result := nil;
  if Nome = 'lblEtapa' then
    Result := lblEtapa;
end;

function TFrmProgressoBase.SenderAsLabel(Sender: TAylisComponent): TLabel;
begin
  Result := nil;
  if Sender <> nil then
    Result := FindLabel(Sender.Name);
end;

function TFrmProgressoBase.SenderAsLabel(Event: TJSONObject): TLabel;
begin
  Result := FindLabel(TJSONObject(Event.Find('sender')).Get('name', ''));
end;

function TFrmProgressoBase.FindProgressBar(const Nome: string): TProgressBar;
begin
  Result := nil;
  if Nome = 'pbProgresso' then
    Result := pbProgresso;
end;

function TFrmProgressoBase.SenderAsProgressBar(Sender: TAylisComponent): TProgressBar;
begin
  Result := nil;
  if Sender <> nil then
    Result := FindProgressBar(Sender.Name);
end;

function TFrmProgressoBase.SenderAsProgressBar(Event: TJSONObject): TProgressBar;
begin
  Result := FindProgressBar(TJSONObject(Event.Find('sender')).Get('name', ''));
end;

end.
