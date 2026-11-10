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
  { Um ClientDataSet de um form. }
  TClientDataSet = class(TAylisComponent)
  private
    function GetKeyFields: string;
    procedure SetKeyFields(Value: string);
    function GetApplyMode: string;
    procedure SetApplyMode(Value: string);
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
    function GetIndexes: string;
    procedure SetIndexes(Value: string);
    function GetIndexName: string;
    procedure SetIndexName(Value: string);
    function GetParams: string;
    procedure SetParams(Value: string);
    function GetStatusFilter: string;
    procedure SetStatusFilter(Value: string);
    function GetMasterSource: string;
    procedure SetMasterSource(Value: string);
    function GetMasterFields: string;
    procedure SetMasterFields(Value: string);
    function GetApplySeparately: Boolean;
    procedure SetApplySeparately(Value: Boolean);
    function GetRecordTotal: Int64;
    procedure SetRecordTotal(Value: Int64);
    function GetMaxRecords: Int64;
    procedure SetMaxRecords(Value: Int64);
    function GetPartialNotice: string;
  public
    property KeyFields: string read GetKeyFields write SetKeyFields;
    property ApplyMode: string read GetApplyMode write SetApplyMode;
    property ReadOnly: Boolean read GetReadOnly write SetReadOnly;
    property Active: Boolean read GetActive write SetActive;
    property Filter: string read GetFilter write SetFilter;
    property Filtered: Boolean read GetFiltered write SetFiltered;
    property IndexFieldNames: string read GetIndexFieldNames write SetIndexFieldNames;
    property Indexes: string read GetIndexes write SetIndexes;
    property IndexName: string read GetIndexName write SetIndexName;
    property Params: string read GetParams write SetParams;
    property StatusFilter: string read GetStatusFilter write SetStatusFilter;
    property MasterSource: string read GetMasterSource write SetMasterSource;
    property MasterFields: string read GetMasterFields write SetMasterFields;
    property ApplySeparately: Boolean read GetApplySeparately write SetApplySeparately;
    property RecordTotal: Int64 read GetRecordTotal write SetRecordTotal;
    property MaxRecords: Int64 read GetMaxRecords write SetMaxRecords;
    property PartialNotice: string read GetPartialNotice;
  end;

  { Um DBChart de um form. }
  TDBChart = class(TAylisComponent)
  private
    function GetDataSource: string;
    procedure SetDataSource(Value: string);
    function GetLabelField: string;
    procedure SetLabelField(Value: string);
    function GetValueFields: string;
    procedure SetValueFields(Value: string);
    function GetKind: string;
    procedure SetKind(Value: string);
    function GetTitle: string;
    procedure SetTitle(Value: string);
    function GetLabels: string;
    procedure SetLabels(Value: string);
    function GetSeries: string;
    procedure SetSeries(Value: string);
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
    property DataSource: string read GetDataSource write SetDataSource;
    property LabelField: string read GetLabelField write SetLabelField;
    property ValueFields: string read GetValueFields write SetValueFields;
    property Kind: string read GetKind write SetKind;
    property Title: string read GetTitle write SetTitle;
    property Labels: string read GetLabels write SetLabels;
    property Series: string read GetSeries write SetSeries;
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

  { Um DataGrid de um form. }
  TDataGrid = class(TAylisComponent)
  private
    function GetDataSource: string;
    procedure SetDataSource(Value: string);
    function GetColumns: string;
    procedure SetColumns(Value: string);
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
    function GetFooter: string;
    procedure SetFooter(Value: string);
    function GetIncrementalSearch: Boolean;
    procedure SetIncrementalSearch(Value: Boolean);
    function GetCursorColor: string;
    procedure SetCursorColor(Value: string);
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
    property DataSource: string read GetDataSource write SetDataSource;
    property Columns: string read GetColumns write SetColumns;
    property RowHeight: Double read GetRowHeight write SetRowHeight;
    property ReadOnly: Boolean read GetReadOnly write SetReadOnly;
    property Sortable: Boolean read GetSortable write SetSortable;
    property ColumnMove: Boolean read GetColumnMove write SetColumnMove;
    property FixedColumns: Int64 read GetFixedColumns write SetFixedColumns;
    property FilterRow: Boolean read GetFilterRow write SetFilterRow;
    property MultiSelect: Boolean read GetMultiSelect write SetMultiSelect;
    property Footer: string read GetFooter write SetFooter;
    property IncrementalSearch: Boolean read GetIncrementalSearch write SetIncrementalSearch;
    property CursorColor: string read GetCursorColor write SetCursorColor;
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

  { Um DataSource de um form. }
  TDataSource = class(TAylisComponent)
  private
    function GetDataSet: string;
    procedure SetDataSet(Value: string);
    function GetAutoEdit: Boolean;
    procedure SetAutoEdit(Value: Boolean);
    function GetEnabled: Boolean;
    procedure SetEnabled(Value: Boolean);
  public
    property DataSet: string read GetDataSet write SetDataSet;
    property AutoEdit: Boolean read GetAutoEdit write SetAutoEdit;
    property Enabled: Boolean read GetEnabled write SetEnabled;
  end;

  { Um Field de um form. }
  TField = class(TAylisComponent)
  private
    function GetFieldName: string;
    procedure SetFieldName(Value: string);
    function GetType: string;
    procedure SetType(Value: string);
    function GetSize: Int64;
    procedure SetSize(Value: Int64);
    function GetScale: Int64;
    procedure SetScale(Value: Int64);
    function GetRounding: string;
    procedure SetRounding(Value: string);
    function GetDisplayLabel: string;
    procedure SetDisplayLabel(Value: string);
    function GetDisplayWidth: Double;
    procedure SetDisplayWidth(Value: Double);
    function GetDisplayFormat: string;
    procedure SetDisplayFormat(Value: string);
    function GetEditFormat: string;
    procedure SetEditFormat(Value: string);
    function GetAlignment: string;
    procedure SetAlignment(Value: string);
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
    function GetCharCase: string;
    procedure SetCharCase(Value: string);
    function GetEditMask: string;
    procedure SetEditMask(Value: string);
    function GetSensitive: Boolean;
    procedure SetSensitive(Value: Boolean);
    function GetKind: string;
    procedure SetKind(Value: string);
    function GetExpression: string;
    procedure SetExpression(Value: string);
    function GetLookupDataSet: string;
    procedure SetLookupDataSet(Value: string);
    function GetKeyFields: string;
    procedure SetKeyFields(Value: string);
    function GetLookupKeyFields: string;
    procedure SetLookupKeyFields(Value: string);
    function GetLookupResultField: string;
    procedure SetLookupResultField(Value: string);
  public
    property FieldName: string read GetFieldName write SetFieldName;
    property &Type: string read GetType write SetType;
    property Size: Int64 read GetSize write SetSize;
    property Scale: Int64 read GetScale write SetScale;
    property Rounding: string read GetRounding write SetRounding;
    property DisplayLabel: string read GetDisplayLabel write SetDisplayLabel;
    property DisplayWidth: Double read GetDisplayWidth write SetDisplayWidth;
    property DisplayFormat: string read GetDisplayFormat write SetDisplayFormat;
    property EditFormat: string read GetEditFormat write SetEditFormat;
    property Alignment: string read GetAlignment write SetAlignment;
    property Required: Boolean read GetRequired write SetRequired;
    property ReadOnly: Boolean read GetReadOnly write SetReadOnly;
    property Visible: Boolean read GetVisible write SetVisible;
    property Min: Double read GetMin write SetMin;
    property Max: Double read GetMax write SetMax;
    property DefaultValue: string read GetDefaultValue write SetDefaultValue;
    property DisplayValues: string read GetDisplayValues write SetDisplayValues;
    property CharCase: string read GetCharCase write SetCharCase;
    property EditMask: string read GetEditMask write SetEditMask;
    property Sensitive: Boolean read GetSensitive write SetSensitive;
    property Kind: string read GetKind write SetKind;
    property Expression: string read GetExpression write SetExpression;
    property LookupDataSet: string read GetLookupDataSet write SetLookupDataSet;
    property KeyFields: string read GetKeyFields write SetKeyFields;
    property LookupKeyFields: string read GetLookupKeyFields write SetLookupKeyFields;
    property LookupResultField: string read GetLookupResultField write SetLookupResultField;
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

  { Um Image de um form. }
  TImage = class(TAylisComponent)
  private
    function GetImage: string;
    procedure SetImage(Value: string);
    function GetFit: string;
    procedure SetFit(Value: string);
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
    property Image: string read GetImage write SetImage;
    property Fit: string read GetFit write SetFit;
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
    property MinSize: Double read GetMinSize write SetMinSize;
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

  { Um Svg de um form. }
  TSvg = class(TAylisComponent)
  private
    function GetSource: string;
    procedure SetSource(Value: string);
    function GetColor: string;
    procedure SetColor(Value: string);
    function GetFit: string;
    procedure SetFit(Value: string);
    function GetHighlight: string;
    procedure SetHighlight(Value: string);
    function GetHighlightColor: string;
    procedure SetHighlightColor(Value: string);
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
    property Source: string read GetSource write SetSource;
    property Color: string read GetColor write SetColor;
    property Fit: string read GetFit write SetFit;
    property Highlight: string read GetHighlight write SetHighlight;
    property HighlightColor: string read GetHighlightColor write SetHighlightColor;
    property ZoomTo: string read GetZoomTo write SetZoomTo;
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
    { O ClientDataSet deste form com o nome, como o FindComponent (nil se não há). }
    function FindClientDataSet(const Nome: string): TClientDataSet;
    { O ClientDataSet que disparou o evento (nil se não é um ClientDataSet deste form). }
    function SenderAsClientDataSet(Sender: TAylisComponent): TClientDataSet; overload;
    function SenderAsClientDataSet(Event: TJSONObject): TClientDataSet; overload;
    { O DBChart deste form com o nome, como o FindComponent (nil se não há). }
    function FindDBChart(const Nome: string): TDBChart;
    { O DBChart que disparou o evento (nil se não é um DBChart deste form). }
    function SenderAsDBChart(Sender: TAylisComponent): TDBChart; overload;
    function SenderAsDBChart(Event: TJSONObject): TDBChart; overload;
    { O DataGrid deste form com o nome, como o FindComponent (nil se não há). }
    function FindDataGrid(const Nome: string): TDataGrid;
    { O DataGrid que disparou o evento (nil se não é um DataGrid deste form). }
    function SenderAsDataGrid(Sender: TAylisComponent): TDataGrid; overload;
    function SenderAsDataGrid(Event: TJSONObject): TDataGrid; overload;
    { O DataSource deste form com o nome, como o FindComponent (nil se não há). }
    function FindDataSource(const Nome: string): TDataSource;
    { O DataSource que disparou o evento (nil se não é um DataSource deste form). }
    function SenderAsDataSource(Sender: TAylisComponent): TDataSource; overload;
    function SenderAsDataSource(Event: TJSONObject): TDataSource; overload;
    { O Field deste form com o nome, como o FindComponent (nil se não há). }
    function FindField(const Nome: string): TField;
    { O Field que disparou o evento (nil se não é um Field deste form). }
    function SenderAsField(Sender: TAylisComponent): TField; overload;
    function SenderAsField(Event: TJSONObject): TField; overload;
    { O GroupBox deste form com o nome, como o FindComponent (nil se não há). }
    function FindGroupBox(const Nome: string): TGroupBox;
    { O GroupBox que disparou o evento (nil se não é um GroupBox deste form). }
    function SenderAsGroupBox(Sender: TAylisComponent): TGroupBox; overload;
    function SenderAsGroupBox(Event: TJSONObject): TGroupBox; overload;
    { O Image deste form com o nome, como o FindComponent (nil se não há). }
    function FindImage(const Nome: string): TImage;
    { O Image que disparou o evento (nil se não é um Image deste form). }
    function SenderAsImage(Sender: TAylisComponent): TImage; overload;
    function SenderAsImage(Event: TJSONObject): TImage; overload;
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
    { O Panel deste form com o nome, como o FindComponent (nil se não há). }
    function FindPanel(const Nome: string): TPanel;
    { O Panel que disparou o evento (nil se não é um Panel deste form). }
    function SenderAsPanel(Sender: TAylisComponent): TPanel; overload;
    function SenderAsPanel(Event: TJSONObject): TPanel; overload;
    { O Splitter deste form com o nome, como o FindComponent (nil se não há). }
    function FindSplitter(const Nome: string): TSplitter;
    { O Splitter que disparou o evento (nil se não é um Splitter deste form). }
    function SenderAsSplitter(Sender: TAylisComponent): TSplitter; overload;
    function SenderAsSplitter(Event: TJSONObject): TSplitter; overload;
    { O Svg deste form com o nome, como o FindComponent (nil se não há). }
    function FindSvg(const Nome: string): TSvg;
    { O Svg que disparou o evento (nil se não é um Svg deste form). }
    function SenderAsSvg(Sender: TAylisComponent): TSvg; overload;
    function SenderAsSvg(Event: TJSONObject): TSvg; overload;
  end;

implementation

function AsText(Data: TJSONData): string;
begin
  if (Data = nil) or (Data.JSONType = jtNull) then
    Result := ''
  else
    Result := Data.AsString;
end;

function TClientDataSet.GetKeyFields: string;
begin
  Result := AsText(GetProp('key_fields'));
end;

procedure TClientDataSet.SetKeyFields(Value: string);
begin
  SetProp('key_fields', TJSONString.Create(Value));
end;

function TClientDataSet.GetApplyMode: string;
begin
  Result := AsText(GetProp('apply_mode'));
end;

procedure TClientDataSet.SetApplyMode(Value: string);
begin
  SetProp('apply_mode', TJSONString.Create(Value));
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

function TClientDataSet.GetIndexes: string;
begin
  Result := AsText(GetProp('indexes'));
end;

procedure TClientDataSet.SetIndexes(Value: string);
begin
  SetProp('indexes', TJSONString.Create(Value));
end;

function TClientDataSet.GetIndexName: string;
begin
  Result := AsText(GetProp('index_name'));
end;

procedure TClientDataSet.SetIndexName(Value: string);
begin
  SetProp('index_name', TJSONString.Create(Value));
end;

function TClientDataSet.GetParams: string;
begin
  Result := AsText(GetProp('params'));
end;

procedure TClientDataSet.SetParams(Value: string);
begin
  SetProp('params', TJSONString.Create(Value));
end;

function TClientDataSet.GetStatusFilter: string;
begin
  Result := AsText(GetProp('status_filter'));
end;

procedure TClientDataSet.SetStatusFilter(Value: string);
begin
  SetProp('status_filter', TJSONString.Create(Value));
end;

function TClientDataSet.GetMasterSource: string;
begin
  Result := AsText(GetProp('master_source'));
end;

procedure TClientDataSet.SetMasterSource(Value: string);
begin
  SetProp('master_source', TJSONString.Create(Value));
end;

function TClientDataSet.GetMasterFields: string;
begin
  Result := AsText(GetProp('master_fields'));
end;

procedure TClientDataSet.SetMasterFields(Value: string);
begin
  SetProp('master_fields', TJSONString.Create(Value));
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

function TDBChart.GetDataSource: string;
begin
  Result := AsText(GetProp('data_source'));
end;

procedure TDBChart.SetDataSource(Value: string);
begin
  SetProp('data_source', TJSONString.Create(Value));
end;

function TDBChart.GetLabelField: string;
begin
  Result := AsText(GetProp('label_field'));
end;

procedure TDBChart.SetLabelField(Value: string);
begin
  SetProp('label_field', TJSONString.Create(Value));
end;

function TDBChart.GetValueFields: string;
begin
  Result := AsText(GetProp('value_fields'));
end;

procedure TDBChart.SetValueFields(Value: string);
begin
  SetProp('value_fields', TJSONString.Create(Value));
end;

function TDBChart.GetKind: string;
begin
  Result := AsText(GetProp('kind'));
end;

procedure TDBChart.SetKind(Value: string);
begin
  SetProp('kind', TJSONString.Create(Value));
end;

function TDBChart.GetTitle: string;
begin
  Result := AsText(GetProp('title'));
end;

procedure TDBChart.SetTitle(Value: string);
begin
  SetProp('title', TJSONString.Create(Value));
end;

function TDBChart.GetLabels: string;
begin
  Result := AsText(GetProp('labels'));
end;

procedure TDBChart.SetLabels(Value: string);
begin
  SetProp('labels', TJSONString.Create(Value));
end;

function TDBChart.GetSeries: string;
begin
  Result := AsText(GetProp('series'));
end;

procedure TDBChart.SetSeries(Value: string);
begin
  SetProp('series', TJSONString.Create(Value));
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

function TDBChart.GetAlign: string;
begin
  Result := AsText(GetProp('align'));
end;

procedure TDBChart.SetAlign(Value: string);
begin
  SetProp('align', TJSONString.Create(Value));
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

function TDBChart.GetCapturesWheel: Boolean;
begin
  Result := GetProp('captures_wheel').AsBoolean;
end;

procedure TDBChart.SetCapturesWheel(Value: Boolean);
begin
  SetProp('captures_wheel', TJSONBoolean.Create(Value));
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

function TDBChart.GetDragImage: string;
begin
  Result := AsText(GetProp('drag_image'));
end;

procedure TDBChart.SetDragImage(Value: string);
begin
  SetProp('drag_image', TJSONString.Create(Value));
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

function TDBChart.GetCursor: string;
begin
  Result := AsText(GetProp('cursor'));
end;

procedure TDBChart.SetCursor(Value: string);
begin
  SetProp('cursor', TJSONString.Create(Value));
end;

function TDBChart.GetOpacity: Double;
begin
  Result := GetProp('opacity').AsFloat;
end;

procedure TDBChart.SetOpacity(Value: Double);
begin
  SetProp('opacity', TJSONFloatNumber.Create(Value));
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

function TDBChart.GetSelfAlign: string;
begin
  Result := AsText(GetProp('self_align'));
end;

procedure TDBChart.SetSelfAlign(Value: string);
begin
  SetProp('self_align', TJSONString.Create(Value));
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

function TDBChart.GetShadow: string;
begin
  Result := AsText(GetProp('shadow'));
end;

procedure TDBChart.SetShadow(Value: string);
begin
  SetProp('shadow', TJSONString.Create(Value));
end;

function TDBChart.GetStyle: string;
begin
  Result := AsText(GetProp('style'));
end;

procedure TDBChart.SetStyle(Value: string);
begin
  SetProp('style', TJSONString.Create(Value));
end;

function TDBChart.GetTheme: string;
begin
  Result := AsText(GetProp('theme'));
end;

procedure TDBChart.SetTheme(Value: string);
begin
  SetProp('theme', TJSONString.Create(Value));
end;

function TDBChart.GetFill: string;
begin
  Result := AsText(GetProp('fill'));
end;

procedure TDBChart.SetFill(Value: string);
begin
  SetProp('fill', TJSONString.Create(Value));
end;

function TDBChart.GetBorderColor: string;
begin
  Result := AsText(GetProp('border_color'));
end;

procedure TDBChart.SetBorderColor(Value: string);
begin
  SetProp('border_color', TJSONString.Create(Value));
end;

function TDBChart.GetTextColor: string;
begin
  Result := AsText(GetProp('text_color'));
end;

procedure TDBChart.SetTextColor(Value: string);
begin
  SetProp('text_color', TJSONString.Create(Value));
end;

function TDBChart.GetAccentColor: string;
begin
  Result := AsText(GetProp('accent_color'));
end;

procedure TDBChart.SetAccentColor(Value: string);
begin
  SetProp('accent_color', TJSONString.Create(Value));
end;

function TDBChart.GetSuccessColor: string;
begin
  Result := AsText(GetProp('success_color'));
end;

procedure TDBChart.SetSuccessColor(Value: string);
begin
  SetProp('success_color', TJSONString.Create(Value));
end;

function TDBChart.GetWarningColor: string;
begin
  Result := AsText(GetProp('warning_color'));
end;

procedure TDBChart.SetWarningColor(Value: string);
begin
  SetProp('warning_color', TJSONString.Create(Value));
end;

function TDBChart.GetDangerColor: string;
begin
  Result := AsText(GetProp('danger_color'));
end;

procedure TDBChart.SetDangerColor(Value: string);
begin
  SetProp('danger_color', TJSONString.Create(Value));
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

function TDBChart.GetFontWeight: string;
begin
  Result := AsText(GetProp('font_weight'));
end;

procedure TDBChart.SetFontWeight(Value: string);
begin
  SetProp('font_weight', TJSONString.Create(Value));
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

function TDBChart.GetPopupMenu: string;
begin
  Result := AsText(GetProp('popup_menu'));
end;

procedure TDBChart.SetPopupMenu(Value: string);
begin
  SetProp('popup_menu', TJSONString.Create(Value));
end;

function TDataGrid.GetDataSource: string;
begin
  Result := AsText(GetProp('data_source'));
end;

procedure TDataGrid.SetDataSource(Value: string);
begin
  SetProp('data_source', TJSONString.Create(Value));
end;

function TDataGrid.GetColumns: string;
begin
  Result := AsText(GetProp('columns'));
end;

procedure TDataGrid.SetColumns(Value: string);
begin
  SetProp('columns', TJSONString.Create(Value));
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

function TDataGrid.GetFooter: string;
begin
  Result := AsText(GetProp('footer'));
end;

procedure TDataGrid.SetFooter(Value: string);
begin
  SetProp('footer', TJSONString.Create(Value));
end;

function TDataGrid.GetIncrementalSearch: Boolean;
begin
  Result := GetProp('incremental_search').AsBoolean;
end;

procedure TDataGrid.SetIncrementalSearch(Value: Boolean);
begin
  SetProp('incremental_search', TJSONBoolean.Create(Value));
end;

function TDataGrid.GetCursorColor: string;
begin
  Result := AsText(GetProp('cursor_color'));
end;

procedure TDataGrid.SetCursorColor(Value: string);
begin
  SetProp('cursor_color', TJSONString.Create(Value));
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

function TDataGrid.GetAlign: string;
begin
  Result := AsText(GetProp('align'));
end;

procedure TDataGrid.SetAlign(Value: string);
begin
  SetProp('align', TJSONString.Create(Value));
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

function TDataGrid.GetCapturesWheel: Boolean;
begin
  Result := GetProp('captures_wheel').AsBoolean;
end;

procedure TDataGrid.SetCapturesWheel(Value: Boolean);
begin
  SetProp('captures_wheel', TJSONBoolean.Create(Value));
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

function TDataGrid.GetDragImage: string;
begin
  Result := AsText(GetProp('drag_image'));
end;

procedure TDataGrid.SetDragImage(Value: string);
begin
  SetProp('drag_image', TJSONString.Create(Value));
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

function TDataGrid.GetCursor: string;
begin
  Result := AsText(GetProp('cursor'));
end;

procedure TDataGrid.SetCursor(Value: string);
begin
  SetProp('cursor', TJSONString.Create(Value));
end;

function TDataGrid.GetOpacity: Double;
begin
  Result := GetProp('opacity').AsFloat;
end;

procedure TDataGrid.SetOpacity(Value: Double);
begin
  SetProp('opacity', TJSONFloatNumber.Create(Value));
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

function TDataGrid.GetSelfAlign: string;
begin
  Result := AsText(GetProp('self_align'));
end;

procedure TDataGrid.SetSelfAlign(Value: string);
begin
  SetProp('self_align', TJSONString.Create(Value));
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

function TDataGrid.GetShadow: string;
begin
  Result := AsText(GetProp('shadow'));
end;

procedure TDataGrid.SetShadow(Value: string);
begin
  SetProp('shadow', TJSONString.Create(Value));
end;

function TDataGrid.GetStyle: string;
begin
  Result := AsText(GetProp('style'));
end;

procedure TDataGrid.SetStyle(Value: string);
begin
  SetProp('style', TJSONString.Create(Value));
end;

function TDataGrid.GetTheme: string;
begin
  Result := AsText(GetProp('theme'));
end;

procedure TDataGrid.SetTheme(Value: string);
begin
  SetProp('theme', TJSONString.Create(Value));
end;

function TDataGrid.GetFill: string;
begin
  Result := AsText(GetProp('fill'));
end;

procedure TDataGrid.SetFill(Value: string);
begin
  SetProp('fill', TJSONString.Create(Value));
end;

function TDataGrid.GetBorderColor: string;
begin
  Result := AsText(GetProp('border_color'));
end;

procedure TDataGrid.SetBorderColor(Value: string);
begin
  SetProp('border_color', TJSONString.Create(Value));
end;

function TDataGrid.GetTextColor: string;
begin
  Result := AsText(GetProp('text_color'));
end;

procedure TDataGrid.SetTextColor(Value: string);
begin
  SetProp('text_color', TJSONString.Create(Value));
end;

function TDataGrid.GetAccentColor: string;
begin
  Result := AsText(GetProp('accent_color'));
end;

procedure TDataGrid.SetAccentColor(Value: string);
begin
  SetProp('accent_color', TJSONString.Create(Value));
end;

function TDataGrid.GetSuccessColor: string;
begin
  Result := AsText(GetProp('success_color'));
end;

procedure TDataGrid.SetSuccessColor(Value: string);
begin
  SetProp('success_color', TJSONString.Create(Value));
end;

function TDataGrid.GetWarningColor: string;
begin
  Result := AsText(GetProp('warning_color'));
end;

procedure TDataGrid.SetWarningColor(Value: string);
begin
  SetProp('warning_color', TJSONString.Create(Value));
end;

function TDataGrid.GetDangerColor: string;
begin
  Result := AsText(GetProp('danger_color'));
end;

procedure TDataGrid.SetDangerColor(Value: string);
begin
  SetProp('danger_color', TJSONString.Create(Value));
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

function TDataGrid.GetFontWeight: string;
begin
  Result := AsText(GetProp('font_weight'));
end;

procedure TDataGrid.SetFontWeight(Value: string);
begin
  SetProp('font_weight', TJSONString.Create(Value));
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

function TDataGrid.GetPopupMenu: string;
begin
  Result := AsText(GetProp('popup_menu'));
end;

procedure TDataGrid.SetPopupMenu(Value: string);
begin
  SetProp('popup_menu', TJSONString.Create(Value));
end;

procedure TDataGrid.SetFocus;
begin
  Aylis.Send('focus', TJSONObject.Create(['target', Id]));
end;

function TDataSource.GetDataSet: string;
begin
  Result := AsText(GetProp('data_set'));
end;

procedure TDataSource.SetDataSet(Value: string);
begin
  SetProp('data_set', TJSONString.Create(Value));
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

function TField.GetFieldName: string;
begin
  Result := AsText(GetProp('field_name'));
end;

procedure TField.SetFieldName(Value: string);
begin
  SetProp('field_name', TJSONString.Create(Value));
end;

function TField.GetType: string;
begin
  Result := AsText(GetProp('type'));
end;

procedure TField.SetType(Value: string);
begin
  SetProp('type', TJSONString.Create(Value));
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

function TField.GetRounding: string;
begin
  Result := AsText(GetProp('rounding'));
end;

procedure TField.SetRounding(Value: string);
begin
  SetProp('rounding', TJSONString.Create(Value));
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

function TField.GetAlignment: string;
begin
  Result := AsText(GetProp('alignment'));
end;

procedure TField.SetAlignment(Value: string);
begin
  SetProp('alignment', TJSONString.Create(Value));
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

function TField.GetCharCase: string;
begin
  Result := AsText(GetProp('char_case'));
end;

procedure TField.SetCharCase(Value: string);
begin
  SetProp('char_case', TJSONString.Create(Value));
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

function TField.GetKind: string;
begin
  Result := AsText(GetProp('kind'));
end;

procedure TField.SetKind(Value: string);
begin
  SetProp('kind', TJSONString.Create(Value));
end;

function TField.GetExpression: string;
begin
  Result := AsText(GetProp('expression'));
end;

procedure TField.SetExpression(Value: string);
begin
  SetProp('expression', TJSONString.Create(Value));
end;

function TField.GetLookupDataSet: string;
begin
  Result := AsText(GetProp('lookup_data_set'));
end;

procedure TField.SetLookupDataSet(Value: string);
begin
  SetProp('lookup_data_set', TJSONString.Create(Value));
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

function TGroupBox.GetAlign: string;
begin
  Result := AsText(GetProp('align'));
end;

procedure TGroupBox.SetAlign(Value: string);
begin
  SetProp('align', TJSONString.Create(Value));
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

function TGroupBox.GetCapturesWheel: Boolean;
begin
  Result := GetProp('captures_wheel').AsBoolean;
end;

procedure TGroupBox.SetCapturesWheel(Value: Boolean);
begin
  SetProp('captures_wheel', TJSONBoolean.Create(Value));
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

function TGroupBox.GetDragImage: string;
begin
  Result := AsText(GetProp('drag_image'));
end;

procedure TGroupBox.SetDragImage(Value: string);
begin
  SetProp('drag_image', TJSONString.Create(Value));
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

function TGroupBox.GetCursor: string;
begin
  Result := AsText(GetProp('cursor'));
end;

procedure TGroupBox.SetCursor(Value: string);
begin
  SetProp('cursor', TJSONString.Create(Value));
end;

function TGroupBox.GetOpacity: Double;
begin
  Result := GetProp('opacity').AsFloat;
end;

procedure TGroupBox.SetOpacity(Value: Double);
begin
  SetProp('opacity', TJSONFloatNumber.Create(Value));
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

function TGroupBox.GetSelfAlign: string;
begin
  Result := AsText(GetProp('self_align'));
end;

procedure TGroupBox.SetSelfAlign(Value: string);
begin
  SetProp('self_align', TJSONString.Create(Value));
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

function TGroupBox.GetShadow: string;
begin
  Result := AsText(GetProp('shadow'));
end;

procedure TGroupBox.SetShadow(Value: string);
begin
  SetProp('shadow', TJSONString.Create(Value));
end;

function TGroupBox.GetStyle: string;
begin
  Result := AsText(GetProp('style'));
end;

procedure TGroupBox.SetStyle(Value: string);
begin
  SetProp('style', TJSONString.Create(Value));
end;

function TGroupBox.GetTheme: string;
begin
  Result := AsText(GetProp('theme'));
end;

procedure TGroupBox.SetTheme(Value: string);
begin
  SetProp('theme', TJSONString.Create(Value));
end;

function TGroupBox.GetFill: string;
begin
  Result := AsText(GetProp('fill'));
end;

procedure TGroupBox.SetFill(Value: string);
begin
  SetProp('fill', TJSONString.Create(Value));
end;

function TGroupBox.GetBorderColor: string;
begin
  Result := AsText(GetProp('border_color'));
end;

procedure TGroupBox.SetBorderColor(Value: string);
begin
  SetProp('border_color', TJSONString.Create(Value));
end;

function TGroupBox.GetTextColor: string;
begin
  Result := AsText(GetProp('text_color'));
end;

procedure TGroupBox.SetTextColor(Value: string);
begin
  SetProp('text_color', TJSONString.Create(Value));
end;

function TGroupBox.GetAccentColor: string;
begin
  Result := AsText(GetProp('accent_color'));
end;

procedure TGroupBox.SetAccentColor(Value: string);
begin
  SetProp('accent_color', TJSONString.Create(Value));
end;

function TGroupBox.GetSuccessColor: string;
begin
  Result := AsText(GetProp('success_color'));
end;

procedure TGroupBox.SetSuccessColor(Value: string);
begin
  SetProp('success_color', TJSONString.Create(Value));
end;

function TGroupBox.GetWarningColor: string;
begin
  Result := AsText(GetProp('warning_color'));
end;

procedure TGroupBox.SetWarningColor(Value: string);
begin
  SetProp('warning_color', TJSONString.Create(Value));
end;

function TGroupBox.GetDangerColor: string;
begin
  Result := AsText(GetProp('danger_color'));
end;

procedure TGroupBox.SetDangerColor(Value: string);
begin
  SetProp('danger_color', TJSONString.Create(Value));
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

function TGroupBox.GetFontWeight: string;
begin
  Result := AsText(GetProp('font_weight'));
end;

procedure TGroupBox.SetFontWeight(Value: string);
begin
  SetProp('font_weight', TJSONString.Create(Value));
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

function TGroupBox.GetPopupMenu: string;
begin
  Result := AsText(GetProp('popup_menu'));
end;

procedure TGroupBox.SetPopupMenu(Value: string);
begin
  SetProp('popup_menu', TJSONString.Create(Value));
end;

function TImage.GetImage: string;
begin
  Result := AsText(GetProp('image'));
end;

procedure TImage.SetImage(Value: string);
begin
  SetProp('image', TJSONString.Create(Value));
end;

function TImage.GetFit: string;
begin
  Result := AsText(GetProp('fit'));
end;

procedure TImage.SetFit(Value: string);
begin
  SetProp('fit', TJSONString.Create(Value));
end;

function TImage.GetRadius: Double;
begin
  Result := GetProp('radius').AsFloat;
end;

procedure TImage.SetRadius(Value: Double);
begin
  SetProp('radius', TJSONFloatNumber.Create(Value));
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

function TImage.GetAlign: string;
begin
  Result := AsText(GetProp('align'));
end;

procedure TImage.SetAlign(Value: string);
begin
  SetProp('align', TJSONString.Create(Value));
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

function TImage.GetCapturesWheel: Boolean;
begin
  Result := GetProp('captures_wheel').AsBoolean;
end;

procedure TImage.SetCapturesWheel(Value: Boolean);
begin
  SetProp('captures_wheel', TJSONBoolean.Create(Value));
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

function TImage.GetDragImage: string;
begin
  Result := AsText(GetProp('drag_image'));
end;

procedure TImage.SetDragImage(Value: string);
begin
  SetProp('drag_image', TJSONString.Create(Value));
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

function TImage.GetCursor: string;
begin
  Result := AsText(GetProp('cursor'));
end;

procedure TImage.SetCursor(Value: string);
begin
  SetProp('cursor', TJSONString.Create(Value));
end;

function TImage.GetOpacity: Double;
begin
  Result := GetProp('opacity').AsFloat;
end;

procedure TImage.SetOpacity(Value: Double);
begin
  SetProp('opacity', TJSONFloatNumber.Create(Value));
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

function TImage.GetSelfAlign: string;
begin
  Result := AsText(GetProp('self_align'));
end;

procedure TImage.SetSelfAlign(Value: string);
begin
  SetProp('self_align', TJSONString.Create(Value));
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

function TImage.GetShadow: string;
begin
  Result := AsText(GetProp('shadow'));
end;

procedure TImage.SetShadow(Value: string);
begin
  SetProp('shadow', TJSONString.Create(Value));
end;

function TImage.GetStyle: string;
begin
  Result := AsText(GetProp('style'));
end;

procedure TImage.SetStyle(Value: string);
begin
  SetProp('style', TJSONString.Create(Value));
end;

function TImage.GetTheme: string;
begin
  Result := AsText(GetProp('theme'));
end;

procedure TImage.SetTheme(Value: string);
begin
  SetProp('theme', TJSONString.Create(Value));
end;

function TImage.GetFill: string;
begin
  Result := AsText(GetProp('fill'));
end;

procedure TImage.SetFill(Value: string);
begin
  SetProp('fill', TJSONString.Create(Value));
end;

function TImage.GetBorderColor: string;
begin
  Result := AsText(GetProp('border_color'));
end;

procedure TImage.SetBorderColor(Value: string);
begin
  SetProp('border_color', TJSONString.Create(Value));
end;

function TImage.GetTextColor: string;
begin
  Result := AsText(GetProp('text_color'));
end;

procedure TImage.SetTextColor(Value: string);
begin
  SetProp('text_color', TJSONString.Create(Value));
end;

function TImage.GetAccentColor: string;
begin
  Result := AsText(GetProp('accent_color'));
end;

procedure TImage.SetAccentColor(Value: string);
begin
  SetProp('accent_color', TJSONString.Create(Value));
end;

function TImage.GetSuccessColor: string;
begin
  Result := AsText(GetProp('success_color'));
end;

procedure TImage.SetSuccessColor(Value: string);
begin
  SetProp('success_color', TJSONString.Create(Value));
end;

function TImage.GetWarningColor: string;
begin
  Result := AsText(GetProp('warning_color'));
end;

procedure TImage.SetWarningColor(Value: string);
begin
  SetProp('warning_color', TJSONString.Create(Value));
end;

function TImage.GetDangerColor: string;
begin
  Result := AsText(GetProp('danger_color'));
end;

procedure TImage.SetDangerColor(Value: string);
begin
  SetProp('danger_color', TJSONString.Create(Value));
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

function TImage.GetFontWeight: string;
begin
  Result := AsText(GetProp('font_weight'));
end;

procedure TImage.SetFontWeight(Value: string);
begin
  SetProp('font_weight', TJSONString.Create(Value));
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

function TImage.GetPopupMenu: string;
begin
  Result := AsText(GetProp('popup_menu'));
end;

procedure TImage.SetPopupMenu(Value: string);
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

function TSplitter.GetAlign: string;
begin
  Result := AsText(GetProp('align'));
end;

procedure TSplitter.SetAlign(Value: string);
begin
  SetProp('align', TJSONString.Create(Value));
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

function TSplitter.GetCapturesWheel: Boolean;
begin
  Result := GetProp('captures_wheel').AsBoolean;
end;

procedure TSplitter.SetCapturesWheel(Value: Boolean);
begin
  SetProp('captures_wheel', TJSONBoolean.Create(Value));
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

function TSplitter.GetDragImage: string;
begin
  Result := AsText(GetProp('drag_image'));
end;

procedure TSplitter.SetDragImage(Value: string);
begin
  SetProp('drag_image', TJSONString.Create(Value));
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

function TSplitter.GetCursor: string;
begin
  Result := AsText(GetProp('cursor'));
end;

procedure TSplitter.SetCursor(Value: string);
begin
  SetProp('cursor', TJSONString.Create(Value));
end;

function TSplitter.GetOpacity: Double;
begin
  Result := GetProp('opacity').AsFloat;
end;

procedure TSplitter.SetOpacity(Value: Double);
begin
  SetProp('opacity', TJSONFloatNumber.Create(Value));
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

function TSplitter.GetSelfAlign: string;
begin
  Result := AsText(GetProp('self_align'));
end;

procedure TSplitter.SetSelfAlign(Value: string);
begin
  SetProp('self_align', TJSONString.Create(Value));
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

function TSplitter.GetShadow: string;
begin
  Result := AsText(GetProp('shadow'));
end;

procedure TSplitter.SetShadow(Value: string);
begin
  SetProp('shadow', TJSONString.Create(Value));
end;

function TSplitter.GetStyle: string;
begin
  Result := AsText(GetProp('style'));
end;

procedure TSplitter.SetStyle(Value: string);
begin
  SetProp('style', TJSONString.Create(Value));
end;

function TSplitter.GetTheme: string;
begin
  Result := AsText(GetProp('theme'));
end;

procedure TSplitter.SetTheme(Value: string);
begin
  SetProp('theme', TJSONString.Create(Value));
end;

function TSplitter.GetFill: string;
begin
  Result := AsText(GetProp('fill'));
end;

procedure TSplitter.SetFill(Value: string);
begin
  SetProp('fill', TJSONString.Create(Value));
end;

function TSplitter.GetBorderColor: string;
begin
  Result := AsText(GetProp('border_color'));
end;

procedure TSplitter.SetBorderColor(Value: string);
begin
  SetProp('border_color', TJSONString.Create(Value));
end;

function TSplitter.GetTextColor: string;
begin
  Result := AsText(GetProp('text_color'));
end;

procedure TSplitter.SetTextColor(Value: string);
begin
  SetProp('text_color', TJSONString.Create(Value));
end;

function TSplitter.GetAccentColor: string;
begin
  Result := AsText(GetProp('accent_color'));
end;

procedure TSplitter.SetAccentColor(Value: string);
begin
  SetProp('accent_color', TJSONString.Create(Value));
end;

function TSplitter.GetSuccessColor: string;
begin
  Result := AsText(GetProp('success_color'));
end;

procedure TSplitter.SetSuccessColor(Value: string);
begin
  SetProp('success_color', TJSONString.Create(Value));
end;

function TSplitter.GetWarningColor: string;
begin
  Result := AsText(GetProp('warning_color'));
end;

procedure TSplitter.SetWarningColor(Value: string);
begin
  SetProp('warning_color', TJSONString.Create(Value));
end;

function TSplitter.GetDangerColor: string;
begin
  Result := AsText(GetProp('danger_color'));
end;

procedure TSplitter.SetDangerColor(Value: string);
begin
  SetProp('danger_color', TJSONString.Create(Value));
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

function TSplitter.GetFontWeight: string;
begin
  Result := AsText(GetProp('font_weight'));
end;

procedure TSplitter.SetFontWeight(Value: string);
begin
  SetProp('font_weight', TJSONString.Create(Value));
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

function TSplitter.GetPopupMenu: string;
begin
  Result := AsText(GetProp('popup_menu'));
end;

procedure TSplitter.SetPopupMenu(Value: string);
begin
  SetProp('popup_menu', TJSONString.Create(Value));
end;

function TSvg.GetSource: string;
begin
  Result := AsText(GetProp('source'));
end;

procedure TSvg.SetSource(Value: string);
begin
  SetProp('source', TJSONString.Create(Value));
end;

function TSvg.GetColor: string;
begin
  Result := AsText(GetProp('color'));
end;

procedure TSvg.SetColor(Value: string);
begin
  SetProp('color', TJSONString.Create(Value));
end;

function TSvg.GetFit: string;
begin
  Result := AsText(GetProp('fit'));
end;

procedure TSvg.SetFit(Value: string);
begin
  SetProp('fit', TJSONString.Create(Value));
end;

function TSvg.GetHighlight: string;
begin
  Result := AsText(GetProp('highlight'));
end;

procedure TSvg.SetHighlight(Value: string);
begin
  SetProp('highlight', TJSONString.Create(Value));
end;

function TSvg.GetHighlightColor: string;
begin
  Result := AsText(GetProp('highlight_color'));
end;

procedure TSvg.SetHighlightColor(Value: string);
begin
  SetProp('highlight_color', TJSONString.Create(Value));
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

function TSvg.GetAlign: string;
begin
  Result := AsText(GetProp('align'));
end;

procedure TSvg.SetAlign(Value: string);
begin
  SetProp('align', TJSONString.Create(Value));
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

function TSvg.GetCapturesWheel: Boolean;
begin
  Result := GetProp('captures_wheel').AsBoolean;
end;

procedure TSvg.SetCapturesWheel(Value: Boolean);
begin
  SetProp('captures_wheel', TJSONBoolean.Create(Value));
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

function TSvg.GetDragImage: string;
begin
  Result := AsText(GetProp('drag_image'));
end;

procedure TSvg.SetDragImage(Value: string);
begin
  SetProp('drag_image', TJSONString.Create(Value));
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

function TSvg.GetCursor: string;
begin
  Result := AsText(GetProp('cursor'));
end;

procedure TSvg.SetCursor(Value: string);
begin
  SetProp('cursor', TJSONString.Create(Value));
end;

function TSvg.GetOpacity: Double;
begin
  Result := GetProp('opacity').AsFloat;
end;

procedure TSvg.SetOpacity(Value: Double);
begin
  SetProp('opacity', TJSONFloatNumber.Create(Value));
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

function TSvg.GetSelfAlign: string;
begin
  Result := AsText(GetProp('self_align'));
end;

procedure TSvg.SetSelfAlign(Value: string);
begin
  SetProp('self_align', TJSONString.Create(Value));
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

function TSvg.GetShadow: string;
begin
  Result := AsText(GetProp('shadow'));
end;

procedure TSvg.SetShadow(Value: string);
begin
  SetProp('shadow', TJSONString.Create(Value));
end;

function TSvg.GetStyle: string;
begin
  Result := AsText(GetProp('style'));
end;

procedure TSvg.SetStyle(Value: string);
begin
  SetProp('style', TJSONString.Create(Value));
end;

function TSvg.GetTheme: string;
begin
  Result := AsText(GetProp('theme'));
end;

procedure TSvg.SetTheme(Value: string);
begin
  SetProp('theme', TJSONString.Create(Value));
end;

function TSvg.GetFill: string;
begin
  Result := AsText(GetProp('fill'));
end;

procedure TSvg.SetFill(Value: string);
begin
  SetProp('fill', TJSONString.Create(Value));
end;

function TSvg.GetBorderColor: string;
begin
  Result := AsText(GetProp('border_color'));
end;

procedure TSvg.SetBorderColor(Value: string);
begin
  SetProp('border_color', TJSONString.Create(Value));
end;

function TSvg.GetTextColor: string;
begin
  Result := AsText(GetProp('text_color'));
end;

procedure TSvg.SetTextColor(Value: string);
begin
  SetProp('text_color', TJSONString.Create(Value));
end;

function TSvg.GetAccentColor: string;
begin
  Result := AsText(GetProp('accent_color'));
end;

procedure TSvg.SetAccentColor(Value: string);
begin
  SetProp('accent_color', TJSONString.Create(Value));
end;

function TSvg.GetSuccessColor: string;
begin
  Result := AsText(GetProp('success_color'));
end;

procedure TSvg.SetSuccessColor(Value: string);
begin
  SetProp('success_color', TJSONString.Create(Value));
end;

function TSvg.GetWarningColor: string;
begin
  Result := AsText(GetProp('warning_color'));
end;

procedure TSvg.SetWarningColor(Value: string);
begin
  SetProp('warning_color', TJSONString.Create(Value));
end;

function TSvg.GetDangerColor: string;
begin
  Result := AsText(GetProp('danger_color'));
end;

procedure TSvg.SetDangerColor(Value: string);
begin
  SetProp('danger_color', TJSONString.Create(Value));
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

function TSvg.GetFontWeight: string;
begin
  Result := AsText(GetProp('font_weight'));
end;

procedure TSvg.SetFontWeight(Value: string);
begin
  SetProp('font_weight', TJSONString.Create(Value));
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

function TSvg.GetPopupMenu: string;
begin
  Result := AsText(GetProp('popup_menu'));
end;

procedure TSvg.SetPopupMenu(Value: string);
begin
  SetProp('popup_menu', TJSONString.Create(Value));
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

end.
