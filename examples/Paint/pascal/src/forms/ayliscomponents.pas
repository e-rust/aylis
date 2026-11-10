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

  { Um CheckBox de um form. }
  TCheckBox = class(TAylisComponent)
  private
    function GetText: string;
    procedure SetText(Value: string);
    function GetChecked: Boolean;
    procedure SetChecked(Value: Boolean);
    function GetBoxSize: Double;
    procedure SetBoxSize(Value: Double);
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
    property Checked: Boolean read GetChecked write SetChecked;
    property BoxSize: Double read GetBoxSize write SetBoxSize;
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

  { Um Circle de um form. }
  TCircle = class(TAylisComponent)
  private
    function GetFill: string;
    procedure SetFill(Value: string);
    function GetStroke: string;
    procedure SetStroke(Value: string);
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
    property Fill: string read GetFill write SetFill;
    property Stroke: string read GetStroke write SetStroke;
    property StrokeWidth: Double read GetStrokeWidth write SetStrokeWidth;
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

  { Um ComboBox de um form. }
  TComboBox = class(TAylisComponent)
  private
    function GetItems: string;
    procedure SetItems(Value: string);
    function GetValues: string;
    procedure SetValues(Value: string);
    function GetItemIndex: Int64;
    procedure SetItemIndex(Value: Int64);
    function GetHAlign: string;
    procedure SetHAlign(Value: string);
    function GetTrimming: string;
    procedure SetTrimming(Value: string);
    function GetArrow: string;
    procedure SetArrow(Value: string);
    function GetSensitive: Boolean;
    procedure SetSensitive(Value: Boolean);
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
    property Values: string read GetValues write SetValues;
    property ItemIndex: Int64 read GetItemIndex write SetItemIndex;
    property HAlign: string read GetHAlign write SetHAlign;
    property Trimming: string read GetTrimming write SetTrimming;
    property Arrow: string read GetArrow write SetArrow;
    property Sensitive: Boolean read GetSensitive write SetSensitive;
    property Value: string read GetValue write SetValue;
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

  { Um PaintCanvas de um form. }
  TPaintCanvas = class(TAylisComponent)
  private
    function GetTool: string;
    procedure SetTool(Value: string);
    function GetColor: string;
    procedure SetColor(Value: string);
    function GetBackColor: string;
    procedure SetBackColor(Value: string);
    function GetBrushSize: Int64;
    procedure SetBrushSize(Value: Int64);
    function GetFilled: Boolean;
    procedure SetFilled(Value: Boolean);
    function GetZoom: Double;
    procedure SetZoom(Value: Double);
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
    function Undo: Boolean;
    function Redo: Boolean;
    function CanUndo: Boolean;
    function CanRedo: Boolean;
    function Size: string;
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
    property Tool: string read GetTool write SetTool;
    property Color: string read GetColor write SetColor;
    property BackColor: string read GetBackColor write SetBackColor;
    property BrushSize: Int64 read GetBrushSize write SetBrushSize;
    property Filled: Boolean read GetFilled write SetFilled;
    property Zoom: Double read GetZoom write SetZoom;
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
  public
    procedure Execute;
    property Title: string read GetTitle write SetTitle;
    property Filter: string read GetFilter write SetFilter;
    property InitialDir: string read GetInitialDir write SetInitialDir;
    property FileName: string read GetFileName write SetFileName;
    property Multiple: Boolean read GetMultiple write SetMultiple;
  end;

  { Um ScrollBox de um form. }
  TScrollBox = class(TAylisComponent)
  private
    function GetHorizontal: Boolean;
    procedure SetHorizontal(Value: Boolean);
    function GetPullToRefresh: Boolean;
    procedure SetPullToRefresh(Value: Boolean);
    function GetRefreshing: Boolean;
    procedure SetRefreshing(Value: Boolean);
    function GetWheelStep: Double;
    procedure SetWheelStep(Value: Double);
    function GetSmooth: string;
    procedure SetSmooth(Value: string);
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
    procedure ScrollTo(A1: TAylisComponent);
    function VisibleComponents(A1: TAylisComponent; A2: Boolean): string;
    property Horizontal: Boolean read GetHorizontal write SetHorizontal;
    property PullToRefresh: Boolean read GetPullToRefresh write SetPullToRefresh;
    property Refreshing: Boolean read GetRefreshing write SetRefreshing;
    property WheelStep: Double read GetWheelStep write SetWheelStep;
    property Smooth: string read GetSmooth write SetSmooth;
    property AllowedChars: string read GetAllowedChars write SetAllowedChars;
    property BlockedChars: string read GetBlockedChars write SetBlockedChars;
    property OnlyAllowed: Boolean read GetOnlyAllowed write SetOnlyAllowed;
    property CharsOverride: Boolean read GetCharsOverride write SetCharsOverride;
    property CharMap: string read GetCharMap write SetCharMap;
    property ScrollPosition: Double read GetScrollPosition write SetScrollPosition;
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
    { O Button deste form com o nome, como o FindComponent (nil se não há). }
    function FindButton(const Nome: string): TButton;
    { O Button que disparou o evento (nil se não é um Button deste form). }
    function SenderAsButton(Sender: TAylisComponent): TButton; overload;
    function SenderAsButton(Event: TJSONObject): TButton; overload;
    { O CheckBox deste form com o nome, como o FindComponent (nil se não há). }
    function FindCheckBox(const Nome: string): TCheckBox;
    { O CheckBox que disparou o evento (nil se não é um CheckBox deste form). }
    function SenderAsCheckBox(Sender: TAylisComponent): TCheckBox; overload;
    function SenderAsCheckBox(Event: TJSONObject): TCheckBox; overload;
    { O Circle deste form com o nome, como o FindComponent (nil se não há). }
    function FindCircle(const Nome: string): TCircle;
    { O Circle que disparou o evento (nil se não é um Circle deste form). }
    function SenderAsCircle(Sender: TAylisComponent): TCircle; overload;
    function SenderAsCircle(Event: TJSONObject): TCircle; overload;
    { O ComboBox deste form com o nome, como o FindComponent (nil se não há). }
    function FindComboBox(const Nome: string): TComboBox;
    { O ComboBox que disparou o evento (nil se não é um ComboBox deste form). }
    function SenderAsComboBox(Sender: TAylisComponent): TComboBox; overload;
    function SenderAsComboBox(Event: TJSONObject): TComboBox; overload;
    { O Divider deste form com o nome, como o FindComponent (nil se não há). }
    function FindDivider(const Nome: string): TDivider;
    { O Divider que disparou o evento (nil se não é um Divider deste form). }
    function SenderAsDivider(Sender: TAylisComponent): TDivider; overload;
    function SenderAsDivider(Event: TJSONObject): TDivider; overload;
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
    { O PaintCanvas deste form com o nome, como o FindComponent (nil se não há). }
    function FindPaintCanvas(const Nome: string): TPaintCanvas;
    { O PaintCanvas que disparou o evento (nil se não é um PaintCanvas deste form). }
    function SenderAsPaintCanvas(Sender: TAylisComponent): TPaintCanvas; overload;
    function SenderAsPaintCanvas(Event: TJSONObject): TPaintCanvas; overload;
    { O Panel deste form com o nome, como o FindComponent (nil se não há). }
    function FindPanel(const Nome: string): TPanel;
    { O Panel que disparou o evento (nil se não é um Panel deste form). }
    function SenderAsPanel(Sender: TAylisComponent): TPanel; overload;
    function SenderAsPanel(Event: TJSONObject): TPanel; overload;
    { O SaveDialog deste form com o nome, como o FindComponent (nil se não há). }
    function FindSaveDialog(const Nome: string): TSaveDialog;
    { O SaveDialog que disparou o evento (nil se não é um SaveDialog deste form). }
    function SenderAsSaveDialog(Sender: TAylisComponent): TSaveDialog; overload;
    function SenderAsSaveDialog(Event: TJSONObject): TSaveDialog; overload;
    { O ScrollBox deste form com o nome, como o FindComponent (nil se não há). }
    function FindScrollBox(const Nome: string): TScrollBox;
    { O ScrollBox que disparou o evento (nil se não é um ScrollBox deste form). }
    function SenderAsScrollBox(Sender: TAylisComponent): TScrollBox; overload;
    function SenderAsScrollBox(Event: TJSONObject): TScrollBox; overload;
    { O StatusBar deste form com o nome, como o FindComponent (nil se não há). }
    function FindStatusBar(const Nome: string): TStatusBar;
    { O StatusBar que disparou o evento (nil se não é um StatusBar deste form). }
    function SenderAsStatusBar(Sender: TAylisComponent): TStatusBar; overload;
    function SenderAsStatusBar(Event: TJSONObject): TStatusBar; overload;
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

function TCheckBox.GetAlign: string;
begin
  Result := AsText(GetProp('align'));
end;

procedure TCheckBox.SetAlign(Value: string);
begin
  SetProp('align', TJSONString.Create(Value));
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

function TCheckBox.GetCapturesWheel: Boolean;
begin
  Result := GetProp('captures_wheel').AsBoolean;
end;

procedure TCheckBox.SetCapturesWheel(Value: Boolean);
begin
  SetProp('captures_wheel', TJSONBoolean.Create(Value));
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

function TCheckBox.GetDragImage: string;
begin
  Result := AsText(GetProp('drag_image'));
end;

procedure TCheckBox.SetDragImage(Value: string);
begin
  SetProp('drag_image', TJSONString.Create(Value));
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

function TCheckBox.GetCursor: string;
begin
  Result := AsText(GetProp('cursor'));
end;

procedure TCheckBox.SetCursor(Value: string);
begin
  SetProp('cursor', TJSONString.Create(Value));
end;

function TCheckBox.GetOpacity: Double;
begin
  Result := GetProp('opacity').AsFloat;
end;

procedure TCheckBox.SetOpacity(Value: Double);
begin
  SetProp('opacity', TJSONFloatNumber.Create(Value));
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

function TCheckBox.GetSelfAlign: string;
begin
  Result := AsText(GetProp('self_align'));
end;

procedure TCheckBox.SetSelfAlign(Value: string);
begin
  SetProp('self_align', TJSONString.Create(Value));
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

function TCheckBox.GetShadow: string;
begin
  Result := AsText(GetProp('shadow'));
end;

procedure TCheckBox.SetShadow(Value: string);
begin
  SetProp('shadow', TJSONString.Create(Value));
end;

function TCheckBox.GetStyle: string;
begin
  Result := AsText(GetProp('style'));
end;

procedure TCheckBox.SetStyle(Value: string);
begin
  SetProp('style', TJSONString.Create(Value));
end;

function TCheckBox.GetTheme: string;
begin
  Result := AsText(GetProp('theme'));
end;

procedure TCheckBox.SetTheme(Value: string);
begin
  SetProp('theme', TJSONString.Create(Value));
end;

function TCheckBox.GetFill: string;
begin
  Result := AsText(GetProp('fill'));
end;

procedure TCheckBox.SetFill(Value: string);
begin
  SetProp('fill', TJSONString.Create(Value));
end;

function TCheckBox.GetBorderColor: string;
begin
  Result := AsText(GetProp('border_color'));
end;

procedure TCheckBox.SetBorderColor(Value: string);
begin
  SetProp('border_color', TJSONString.Create(Value));
end;

function TCheckBox.GetTextColor: string;
begin
  Result := AsText(GetProp('text_color'));
end;

procedure TCheckBox.SetTextColor(Value: string);
begin
  SetProp('text_color', TJSONString.Create(Value));
end;

function TCheckBox.GetAccentColor: string;
begin
  Result := AsText(GetProp('accent_color'));
end;

procedure TCheckBox.SetAccentColor(Value: string);
begin
  SetProp('accent_color', TJSONString.Create(Value));
end;

function TCheckBox.GetSuccessColor: string;
begin
  Result := AsText(GetProp('success_color'));
end;

procedure TCheckBox.SetSuccessColor(Value: string);
begin
  SetProp('success_color', TJSONString.Create(Value));
end;

function TCheckBox.GetWarningColor: string;
begin
  Result := AsText(GetProp('warning_color'));
end;

procedure TCheckBox.SetWarningColor(Value: string);
begin
  SetProp('warning_color', TJSONString.Create(Value));
end;

function TCheckBox.GetDangerColor: string;
begin
  Result := AsText(GetProp('danger_color'));
end;

procedure TCheckBox.SetDangerColor(Value: string);
begin
  SetProp('danger_color', TJSONString.Create(Value));
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

function TCheckBox.GetFontWeight: string;
begin
  Result := AsText(GetProp('font_weight'));
end;

procedure TCheckBox.SetFontWeight(Value: string);
begin
  SetProp('font_weight', TJSONString.Create(Value));
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

function TCheckBox.GetPopupMenu: string;
begin
  Result := AsText(GetProp('popup_menu'));
end;

procedure TCheckBox.SetPopupMenu(Value: string);
begin
  SetProp('popup_menu', TJSONString.Create(Value));
end;

procedure TCheckBox.SetFocus;
begin
  Aylis.Send('focus', TJSONObject.Create(['target', Id]));
end;

function TCircle.GetFill: string;
begin
  Result := AsText(GetProp('fill'));
end;

procedure TCircle.SetFill(Value: string);
begin
  SetProp('fill', TJSONString.Create(Value));
end;

function TCircle.GetStroke: string;
begin
  Result := AsText(GetProp('stroke'));
end;

procedure TCircle.SetStroke(Value: string);
begin
  SetProp('stroke', TJSONString.Create(Value));
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

function TCircle.GetAlign: string;
begin
  Result := AsText(GetProp('align'));
end;

procedure TCircle.SetAlign(Value: string);
begin
  SetProp('align', TJSONString.Create(Value));
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

function TCircle.GetCapturesWheel: Boolean;
begin
  Result := GetProp('captures_wheel').AsBoolean;
end;

procedure TCircle.SetCapturesWheel(Value: Boolean);
begin
  SetProp('captures_wheel', TJSONBoolean.Create(Value));
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

function TCircle.GetDragImage: string;
begin
  Result := AsText(GetProp('drag_image'));
end;

procedure TCircle.SetDragImage(Value: string);
begin
  SetProp('drag_image', TJSONString.Create(Value));
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

function TCircle.GetCursor: string;
begin
  Result := AsText(GetProp('cursor'));
end;

procedure TCircle.SetCursor(Value: string);
begin
  SetProp('cursor', TJSONString.Create(Value));
end;

function TCircle.GetOpacity: Double;
begin
  Result := GetProp('opacity').AsFloat;
end;

procedure TCircle.SetOpacity(Value: Double);
begin
  SetProp('opacity', TJSONFloatNumber.Create(Value));
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

function TCircle.GetSelfAlign: string;
begin
  Result := AsText(GetProp('self_align'));
end;

procedure TCircle.SetSelfAlign(Value: string);
begin
  SetProp('self_align', TJSONString.Create(Value));
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

function TCircle.GetShadow: string;
begin
  Result := AsText(GetProp('shadow'));
end;

procedure TCircle.SetShadow(Value: string);
begin
  SetProp('shadow', TJSONString.Create(Value));
end;

function TCircle.GetStyle: string;
begin
  Result := AsText(GetProp('style'));
end;

procedure TCircle.SetStyle(Value: string);
begin
  SetProp('style', TJSONString.Create(Value));
end;

function TCircle.GetTheme: string;
begin
  Result := AsText(GetProp('theme'));
end;

procedure TCircle.SetTheme(Value: string);
begin
  SetProp('theme', TJSONString.Create(Value));
end;

function TCircle.GetBorderColor: string;
begin
  Result := AsText(GetProp('border_color'));
end;

procedure TCircle.SetBorderColor(Value: string);
begin
  SetProp('border_color', TJSONString.Create(Value));
end;

function TCircle.GetTextColor: string;
begin
  Result := AsText(GetProp('text_color'));
end;

procedure TCircle.SetTextColor(Value: string);
begin
  SetProp('text_color', TJSONString.Create(Value));
end;

function TCircle.GetAccentColor: string;
begin
  Result := AsText(GetProp('accent_color'));
end;

procedure TCircle.SetAccentColor(Value: string);
begin
  SetProp('accent_color', TJSONString.Create(Value));
end;

function TCircle.GetSuccessColor: string;
begin
  Result := AsText(GetProp('success_color'));
end;

procedure TCircle.SetSuccessColor(Value: string);
begin
  SetProp('success_color', TJSONString.Create(Value));
end;

function TCircle.GetWarningColor: string;
begin
  Result := AsText(GetProp('warning_color'));
end;

procedure TCircle.SetWarningColor(Value: string);
begin
  SetProp('warning_color', TJSONString.Create(Value));
end;

function TCircle.GetDangerColor: string;
begin
  Result := AsText(GetProp('danger_color'));
end;

procedure TCircle.SetDangerColor(Value: string);
begin
  SetProp('danger_color', TJSONString.Create(Value));
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

function TCircle.GetFontWeight: string;
begin
  Result := AsText(GetProp('font_weight'));
end;

procedure TCircle.SetFontWeight(Value: string);
begin
  SetProp('font_weight', TJSONString.Create(Value));
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

function TCircle.GetPopupMenu: string;
begin
  Result := AsText(GetProp('popup_menu'));
end;

procedure TCircle.SetPopupMenu(Value: string);
begin
  SetProp('popup_menu', TJSONString.Create(Value));
end;

function TComboBox.GetItems: string;
begin
  Result := AsText(GetProp('items'));
end;

procedure TComboBox.SetItems(Value: string);
begin
  SetProp('items', TJSONString.Create(Value));
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

function TComboBox.GetHAlign: string;
begin
  Result := AsText(GetProp('h_align'));
end;

procedure TComboBox.SetHAlign(Value: string);
begin
  SetProp('h_align', TJSONString.Create(Value));
end;

function TComboBox.GetTrimming: string;
begin
  Result := AsText(GetProp('trimming'));
end;

procedure TComboBox.SetTrimming(Value: string);
begin
  SetProp('trimming', TJSONString.Create(Value));
end;

function TComboBox.GetArrow: string;
begin
  Result := AsText(GetProp('arrow'));
end;

procedure TComboBox.SetArrow(Value: string);
begin
  SetProp('arrow', TJSONString.Create(Value));
end;

function TComboBox.GetSensitive: Boolean;
begin
  Result := GetProp('sensitive').AsBoolean;
end;

procedure TComboBox.SetSensitive(Value: Boolean);
begin
  SetProp('sensitive', TJSONBoolean.Create(Value));
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

function TComboBox.GetAlign: string;
begin
  Result := AsText(GetProp('align'));
end;

procedure TComboBox.SetAlign(Value: string);
begin
  SetProp('align', TJSONString.Create(Value));
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

function TComboBox.GetCapturesWheel: Boolean;
begin
  Result := GetProp('captures_wheel').AsBoolean;
end;

procedure TComboBox.SetCapturesWheel(Value: Boolean);
begin
  SetProp('captures_wheel', TJSONBoolean.Create(Value));
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

function TComboBox.GetDragImage: string;
begin
  Result := AsText(GetProp('drag_image'));
end;

procedure TComboBox.SetDragImage(Value: string);
begin
  SetProp('drag_image', TJSONString.Create(Value));
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

function TComboBox.GetCursor: string;
begin
  Result := AsText(GetProp('cursor'));
end;

procedure TComboBox.SetCursor(Value: string);
begin
  SetProp('cursor', TJSONString.Create(Value));
end;

function TComboBox.GetOpacity: Double;
begin
  Result := GetProp('opacity').AsFloat;
end;

procedure TComboBox.SetOpacity(Value: Double);
begin
  SetProp('opacity', TJSONFloatNumber.Create(Value));
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

function TComboBox.GetSelfAlign: string;
begin
  Result := AsText(GetProp('self_align'));
end;

procedure TComboBox.SetSelfAlign(Value: string);
begin
  SetProp('self_align', TJSONString.Create(Value));
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

function TComboBox.GetShadow: string;
begin
  Result := AsText(GetProp('shadow'));
end;

procedure TComboBox.SetShadow(Value: string);
begin
  SetProp('shadow', TJSONString.Create(Value));
end;

function TComboBox.GetStyle: string;
begin
  Result := AsText(GetProp('style'));
end;

procedure TComboBox.SetStyle(Value: string);
begin
  SetProp('style', TJSONString.Create(Value));
end;

function TComboBox.GetTheme: string;
begin
  Result := AsText(GetProp('theme'));
end;

procedure TComboBox.SetTheme(Value: string);
begin
  SetProp('theme', TJSONString.Create(Value));
end;

function TComboBox.GetFill: string;
begin
  Result := AsText(GetProp('fill'));
end;

procedure TComboBox.SetFill(Value: string);
begin
  SetProp('fill', TJSONString.Create(Value));
end;

function TComboBox.GetBorderColor: string;
begin
  Result := AsText(GetProp('border_color'));
end;

procedure TComboBox.SetBorderColor(Value: string);
begin
  SetProp('border_color', TJSONString.Create(Value));
end;

function TComboBox.GetTextColor: string;
begin
  Result := AsText(GetProp('text_color'));
end;

procedure TComboBox.SetTextColor(Value: string);
begin
  SetProp('text_color', TJSONString.Create(Value));
end;

function TComboBox.GetAccentColor: string;
begin
  Result := AsText(GetProp('accent_color'));
end;

procedure TComboBox.SetAccentColor(Value: string);
begin
  SetProp('accent_color', TJSONString.Create(Value));
end;

function TComboBox.GetSuccessColor: string;
begin
  Result := AsText(GetProp('success_color'));
end;

procedure TComboBox.SetSuccessColor(Value: string);
begin
  SetProp('success_color', TJSONString.Create(Value));
end;

function TComboBox.GetWarningColor: string;
begin
  Result := AsText(GetProp('warning_color'));
end;

procedure TComboBox.SetWarningColor(Value: string);
begin
  SetProp('warning_color', TJSONString.Create(Value));
end;

function TComboBox.GetDangerColor: string;
begin
  Result := AsText(GetProp('danger_color'));
end;

procedure TComboBox.SetDangerColor(Value: string);
begin
  SetProp('danger_color', TJSONString.Create(Value));
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

function TComboBox.GetFontWeight: string;
begin
  Result := AsText(GetProp('font_weight'));
end;

procedure TComboBox.SetFontWeight(Value: string);
begin
  SetProp('font_weight', TJSONString.Create(Value));
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

function TComboBox.GetPopupMenu: string;
begin
  Result := AsText(GetProp('popup_menu'));
end;

procedure TComboBox.SetPopupMenu(Value: string);
begin
  SetProp('popup_menu', TJSONString.Create(Value));
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

function TPaintCanvas.GetTool: string;
begin
  Result := AsText(GetProp('tool'));
end;

procedure TPaintCanvas.SetTool(Value: string);
begin
  SetProp('tool', TJSONString.Create(Value));
end;

function TPaintCanvas.GetColor: string;
begin
  Result := AsText(GetProp('color'));
end;

procedure TPaintCanvas.SetColor(Value: string);
begin
  SetProp('color', TJSONString.Create(Value));
end;

function TPaintCanvas.GetBackColor: string;
begin
  Result := AsText(GetProp('back_color'));
end;

procedure TPaintCanvas.SetBackColor(Value: string);
begin
  SetProp('back_color', TJSONString.Create(Value));
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

function TPaintCanvas.GetAlign: string;
begin
  Result := AsText(GetProp('align'));
end;

procedure TPaintCanvas.SetAlign(Value: string);
begin
  SetProp('align', TJSONString.Create(Value));
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

function TPaintCanvas.GetCapturesWheel: Boolean;
begin
  Result := GetProp('captures_wheel').AsBoolean;
end;

procedure TPaintCanvas.SetCapturesWheel(Value: Boolean);
begin
  SetProp('captures_wheel', TJSONBoolean.Create(Value));
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

function TPaintCanvas.GetDragImage: string;
begin
  Result := AsText(GetProp('drag_image'));
end;

procedure TPaintCanvas.SetDragImage(Value: string);
begin
  SetProp('drag_image', TJSONString.Create(Value));
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

function TPaintCanvas.GetCursor: string;
begin
  Result := AsText(GetProp('cursor'));
end;

procedure TPaintCanvas.SetCursor(Value: string);
begin
  SetProp('cursor', TJSONString.Create(Value));
end;

function TPaintCanvas.GetOpacity: Double;
begin
  Result := GetProp('opacity').AsFloat;
end;

procedure TPaintCanvas.SetOpacity(Value: Double);
begin
  SetProp('opacity', TJSONFloatNumber.Create(Value));
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

function TPaintCanvas.GetSelfAlign: string;
begin
  Result := AsText(GetProp('self_align'));
end;

procedure TPaintCanvas.SetSelfAlign(Value: string);
begin
  SetProp('self_align', TJSONString.Create(Value));
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

function TPaintCanvas.GetShadow: string;
begin
  Result := AsText(GetProp('shadow'));
end;

procedure TPaintCanvas.SetShadow(Value: string);
begin
  SetProp('shadow', TJSONString.Create(Value));
end;

function TPaintCanvas.GetStyle: string;
begin
  Result := AsText(GetProp('style'));
end;

procedure TPaintCanvas.SetStyle(Value: string);
begin
  SetProp('style', TJSONString.Create(Value));
end;

function TPaintCanvas.GetTheme: string;
begin
  Result := AsText(GetProp('theme'));
end;

procedure TPaintCanvas.SetTheme(Value: string);
begin
  SetProp('theme', TJSONString.Create(Value));
end;

function TPaintCanvas.GetFill: string;
begin
  Result := AsText(GetProp('fill'));
end;

procedure TPaintCanvas.SetFill(Value: string);
begin
  SetProp('fill', TJSONString.Create(Value));
end;

function TPaintCanvas.GetBorderColor: string;
begin
  Result := AsText(GetProp('border_color'));
end;

procedure TPaintCanvas.SetBorderColor(Value: string);
begin
  SetProp('border_color', TJSONString.Create(Value));
end;

function TPaintCanvas.GetTextColor: string;
begin
  Result := AsText(GetProp('text_color'));
end;

procedure TPaintCanvas.SetTextColor(Value: string);
begin
  SetProp('text_color', TJSONString.Create(Value));
end;

function TPaintCanvas.GetAccentColor: string;
begin
  Result := AsText(GetProp('accent_color'));
end;

procedure TPaintCanvas.SetAccentColor(Value: string);
begin
  SetProp('accent_color', TJSONString.Create(Value));
end;

function TPaintCanvas.GetSuccessColor: string;
begin
  Result := AsText(GetProp('success_color'));
end;

procedure TPaintCanvas.SetSuccessColor(Value: string);
begin
  SetProp('success_color', TJSONString.Create(Value));
end;

function TPaintCanvas.GetWarningColor: string;
begin
  Result := AsText(GetProp('warning_color'));
end;

procedure TPaintCanvas.SetWarningColor(Value: string);
begin
  SetProp('warning_color', TJSONString.Create(Value));
end;

function TPaintCanvas.GetDangerColor: string;
begin
  Result := AsText(GetProp('danger_color'));
end;

procedure TPaintCanvas.SetDangerColor(Value: string);
begin
  SetProp('danger_color', TJSONString.Create(Value));
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

function TPaintCanvas.GetFontWeight: string;
begin
  Result := AsText(GetProp('font_weight'));
end;

procedure TPaintCanvas.SetFontWeight(Value: string);
begin
  SetProp('font_weight', TJSONString.Create(Value));
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

function TPaintCanvas.GetPopupMenu: string;
begin
  Result := AsText(GetProp('popup_menu'));
end;

procedure TPaintCanvas.SetPopupMenu(Value: string);
begin
  SetProp('popup_menu', TJSONString.Create(Value));
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

function TPaintCanvas.Size: string;
begin
  Result := AsText(Aylis.Call('invoke', TJSONObject.Create(['target', Id, 'method', 'size', 'args', TJSONArray.Create])));
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

procedure TSaveDialog.Execute;
begin
  Aylis.Send('execute', TJSONObject.Create(['target', Id]));
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

function TScrollBox.GetSmooth: string;
begin
  Result := AsText(GetProp('smooth'));
end;

procedure TScrollBox.SetSmooth(Value: string);
begin
  SetProp('smooth', TJSONString.Create(Value));
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

function TScrollBox.GetAlign: string;
begin
  Result := AsText(GetProp('align'));
end;

procedure TScrollBox.SetAlign(Value: string);
begin
  SetProp('align', TJSONString.Create(Value));
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

function TScrollBox.GetCapturesWheel: Boolean;
begin
  Result := GetProp('captures_wheel').AsBoolean;
end;

procedure TScrollBox.SetCapturesWheel(Value: Boolean);
begin
  SetProp('captures_wheel', TJSONBoolean.Create(Value));
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

function TScrollBox.GetDragImage: string;
begin
  Result := AsText(GetProp('drag_image'));
end;

procedure TScrollBox.SetDragImage(Value: string);
begin
  SetProp('drag_image', TJSONString.Create(Value));
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

function TScrollBox.GetCursor: string;
begin
  Result := AsText(GetProp('cursor'));
end;

procedure TScrollBox.SetCursor(Value: string);
begin
  SetProp('cursor', TJSONString.Create(Value));
end;

function TScrollBox.GetOpacity: Double;
begin
  Result := GetProp('opacity').AsFloat;
end;

procedure TScrollBox.SetOpacity(Value: Double);
begin
  SetProp('opacity', TJSONFloatNumber.Create(Value));
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

function TScrollBox.GetSelfAlign: string;
begin
  Result := AsText(GetProp('self_align'));
end;

procedure TScrollBox.SetSelfAlign(Value: string);
begin
  SetProp('self_align', TJSONString.Create(Value));
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

function TScrollBox.GetShadow: string;
begin
  Result := AsText(GetProp('shadow'));
end;

procedure TScrollBox.SetShadow(Value: string);
begin
  SetProp('shadow', TJSONString.Create(Value));
end;

function TScrollBox.GetStyle: string;
begin
  Result := AsText(GetProp('style'));
end;

procedure TScrollBox.SetStyle(Value: string);
begin
  SetProp('style', TJSONString.Create(Value));
end;

function TScrollBox.GetTheme: string;
begin
  Result := AsText(GetProp('theme'));
end;

procedure TScrollBox.SetTheme(Value: string);
begin
  SetProp('theme', TJSONString.Create(Value));
end;

function TScrollBox.GetFill: string;
begin
  Result := AsText(GetProp('fill'));
end;

procedure TScrollBox.SetFill(Value: string);
begin
  SetProp('fill', TJSONString.Create(Value));
end;

function TScrollBox.GetBorderColor: string;
begin
  Result := AsText(GetProp('border_color'));
end;

procedure TScrollBox.SetBorderColor(Value: string);
begin
  SetProp('border_color', TJSONString.Create(Value));
end;

function TScrollBox.GetTextColor: string;
begin
  Result := AsText(GetProp('text_color'));
end;

procedure TScrollBox.SetTextColor(Value: string);
begin
  SetProp('text_color', TJSONString.Create(Value));
end;

function TScrollBox.GetAccentColor: string;
begin
  Result := AsText(GetProp('accent_color'));
end;

procedure TScrollBox.SetAccentColor(Value: string);
begin
  SetProp('accent_color', TJSONString.Create(Value));
end;

function TScrollBox.GetSuccessColor: string;
begin
  Result := AsText(GetProp('success_color'));
end;

procedure TScrollBox.SetSuccessColor(Value: string);
begin
  SetProp('success_color', TJSONString.Create(Value));
end;

function TScrollBox.GetWarningColor: string;
begin
  Result := AsText(GetProp('warning_color'));
end;

procedure TScrollBox.SetWarningColor(Value: string);
begin
  SetProp('warning_color', TJSONString.Create(Value));
end;

function TScrollBox.GetDangerColor: string;
begin
  Result := AsText(GetProp('danger_color'));
end;

procedure TScrollBox.SetDangerColor(Value: string);
begin
  SetProp('danger_color', TJSONString.Create(Value));
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

function TScrollBox.GetFontWeight: string;
begin
  Result := AsText(GetProp('font_weight'));
end;

procedure TScrollBox.SetFontWeight(Value: string);
begin
  SetProp('font_weight', TJSONString.Create(Value));
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

function TScrollBox.GetPopupMenu: string;
begin
  Result := AsText(GetProp('popup_menu'));
end;

procedure TScrollBox.SetPopupMenu(Value: string);
begin
  SetProp('popup_menu', TJSONString.Create(Value));
end;

procedure TScrollBox.ScrollTo(A1: TAylisComponent);
begin
  Aylis.Send('invoke', TJSONObject.Create(['target', Id, 'method', 'scroll_to', 'args', TJSONArray.Create([A1.Id])]));
end;

function TScrollBox.VisibleComponents(A1: TAylisComponent; A2: Boolean): string;
begin
  Result := AsText(Aylis.Call('invoke', TJSONObject.Create(['target', Id, 'method', 'visible_components', 'args', TJSONArray.Create([A1.Id, A2])])));
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

function TStatusBar.GetAlign: string;
begin
  Result := AsText(GetProp('align'));
end;

procedure TStatusBar.SetAlign(Value: string);
begin
  SetProp('align', TJSONString.Create(Value));
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

function TStatusBar.GetCapturesWheel: Boolean;
begin
  Result := GetProp('captures_wheel').AsBoolean;
end;

procedure TStatusBar.SetCapturesWheel(Value: Boolean);
begin
  SetProp('captures_wheel', TJSONBoolean.Create(Value));
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

function TStatusBar.GetDragImage: string;
begin
  Result := AsText(GetProp('drag_image'));
end;

procedure TStatusBar.SetDragImage(Value: string);
begin
  SetProp('drag_image', TJSONString.Create(Value));
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

function TStatusBar.GetCursor: string;
begin
  Result := AsText(GetProp('cursor'));
end;

procedure TStatusBar.SetCursor(Value: string);
begin
  SetProp('cursor', TJSONString.Create(Value));
end;

function TStatusBar.GetOpacity: Double;
begin
  Result := GetProp('opacity').AsFloat;
end;

procedure TStatusBar.SetOpacity(Value: Double);
begin
  SetProp('opacity', TJSONFloatNumber.Create(Value));
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

function TStatusBar.GetSelfAlign: string;
begin
  Result := AsText(GetProp('self_align'));
end;

procedure TStatusBar.SetSelfAlign(Value: string);
begin
  SetProp('self_align', TJSONString.Create(Value));
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

function TStatusBar.GetShadow: string;
begin
  Result := AsText(GetProp('shadow'));
end;

procedure TStatusBar.SetShadow(Value: string);
begin
  SetProp('shadow', TJSONString.Create(Value));
end;

function TStatusBar.GetStyle: string;
begin
  Result := AsText(GetProp('style'));
end;

procedure TStatusBar.SetStyle(Value: string);
begin
  SetProp('style', TJSONString.Create(Value));
end;

function TStatusBar.GetTheme: string;
begin
  Result := AsText(GetProp('theme'));
end;

procedure TStatusBar.SetTheme(Value: string);
begin
  SetProp('theme', TJSONString.Create(Value));
end;

function TStatusBar.GetFill: string;
begin
  Result := AsText(GetProp('fill'));
end;

procedure TStatusBar.SetFill(Value: string);
begin
  SetProp('fill', TJSONString.Create(Value));
end;

function TStatusBar.GetBorderColor: string;
begin
  Result := AsText(GetProp('border_color'));
end;

procedure TStatusBar.SetBorderColor(Value: string);
begin
  SetProp('border_color', TJSONString.Create(Value));
end;

function TStatusBar.GetTextColor: string;
begin
  Result := AsText(GetProp('text_color'));
end;

procedure TStatusBar.SetTextColor(Value: string);
begin
  SetProp('text_color', TJSONString.Create(Value));
end;

function TStatusBar.GetAccentColor: string;
begin
  Result := AsText(GetProp('accent_color'));
end;

procedure TStatusBar.SetAccentColor(Value: string);
begin
  SetProp('accent_color', TJSONString.Create(Value));
end;

function TStatusBar.GetSuccessColor: string;
begin
  Result := AsText(GetProp('success_color'));
end;

procedure TStatusBar.SetSuccessColor(Value: string);
begin
  SetProp('success_color', TJSONString.Create(Value));
end;

function TStatusBar.GetWarningColor: string;
begin
  Result := AsText(GetProp('warning_color'));
end;

procedure TStatusBar.SetWarningColor(Value: string);
begin
  SetProp('warning_color', TJSONString.Create(Value));
end;

function TStatusBar.GetDangerColor: string;
begin
  Result := AsText(GetProp('danger_color'));
end;

procedure TStatusBar.SetDangerColor(Value: string);
begin
  SetProp('danger_color', TJSONString.Create(Value));
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

function TStatusBar.GetFontWeight: string;
begin
  Result := AsText(GetProp('font_weight'));
end;

procedure TStatusBar.SetFontWeight(Value: string);
begin
  SetProp('font_weight', TJSONString.Create(Value));
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

function TStatusBar.GetPopupMenu: string;
begin
  Result := AsText(GetProp('popup_menu'));
end;

procedure TStatusBar.SetPopupMenu(Value: string);
begin
  SetProp('popup_menu', TJSONString.Create(Value));
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

end.
