// Os componentes dos forms com as propriedades tipadas (frmX.componente.Propriedade).
// Gerado pela IDE do Aylis a cada "Gerar código": não edite.

using System.Text.Json.Nodes;
using AylisClient;

namespace App.Forms
{
    /// <summary>Os forms (<c>using static App.Forms.Components;</c>): cada frmX é a janela do
    /// form cujo evento está rodando (a primeira, fora dos handlers).</summary>
    public static class Components
    {
        internal static bool Bool(JsonNode? v) => v is JsonValue j && j.TryGetValue<bool>(out var b) && b;
        internal static double Number(JsonNode? v) => v is JsonValue j && j.TryGetValue<double>(out var d) ? d : 0;
        internal static string Text(JsonNode? v) => v is JsonValue j && j.TryGetValue<string>(out var t) ? t : v?.ToJsonString() ?? "";

        public static AylisUi.FrmIBGEComponents frmIBGE = null!;

        /// <summary>Registra as classes dos forms (o registro dos forms chama).</summary>
        public static void RegisterClasses()
        {
            Aylis.RegisterFormClass("frmIBGE", (aylis, id, name) => new AylisUi.FrmIBGEComponents(aylis, id, name), form => frmIBGE = (AylisUi.FrmIBGEComponents)form);
        }
    }
}

namespace App.Forms.AylisUi
{
    using static App.Forms.Components;

    /// <summary>Um ClientDataSet de um form.</summary>
    public class ClientDataSet(Aylis aylis, long id, string name) : AylisComponent(aylis, id, name)
    {
        public string KeyFields { get => Text(GetProp("key_fields")); set => SetProp("key_fields", value); }
        public string ApplyMode { get => Text(GetProp("apply_mode")); set => SetProp("apply_mode", value); }
        public bool ReadOnly { get => Bool(GetProp("read_only")); set => SetProp("read_only", value); }
        public bool Active { get => Bool(GetProp("active")); set => SetProp("active", value); }
        public string Filter { get => Text(GetProp("filter")); set => SetProp("filter", value); }
        public bool Filtered { get => Bool(GetProp("filtered")); set => SetProp("filtered", value); }
        public string IndexFieldNames { get => Text(GetProp("index_field_names")); set => SetProp("index_field_names", value); }
        public string Indexes { get => Text(GetProp("indexes")); set => SetProp("indexes", value); }
        public string IndexName { get => Text(GetProp("index_name")); set => SetProp("index_name", value); }
        public string Params { get => Text(GetProp("params")); set => SetProp("params", value); }
        public string StatusFilter { get => Text(GetProp("status_filter")); set => SetProp("status_filter", value); }
        public string MasterSource { get => Text(GetProp("master_source")); set => SetProp("master_source", value); }
        public string MasterFields { get => Text(GetProp("master_fields")); set => SetProp("master_fields", value); }
        public bool ApplySeparately { get => Bool(GetProp("apply_separately")); set => SetProp("apply_separately", value); }
        public long RecordTotal { get => (long)Number(GetProp("record_total")); set => SetProp("record_total", value); }
        public long MaxRecords { get => (long)Number(GetProp("max_records")); set => SetProp("max_records", value); }
        public string PartialNotice => Text(GetProp("partial_notice"));
    }

    /// <summary>Um DBChart de um form.</summary>
    public class DBChart(Aylis aylis, long id, string name) : AylisComponent(aylis, id, name)
    {
        public string DataSource { get => Text(GetProp("data_source")); set => SetProp("data_source", value); }
        public string LabelField { get => Text(GetProp("label_field")); set => SetProp("label_field", value); }
        public string ValueFields { get => Text(GetProp("value_fields")); set => SetProp("value_fields", value); }
        public string Kind { get => Text(GetProp("kind")); set => SetProp("kind", value); }
        public string Title { get => Text(GetProp("title")); set => SetProp("title", value); }
        public string Labels { get => Text(GetProp("labels")); set => SetProp("labels", value); }
        public string Series { get => Text(GetProp("series")); set => SetProp("series", value); }
        public string Json { get => Text(GetProp("json")); set => SetProp("json", value); }
        public bool Legend { get => Bool(GetProp("legend")); set => SetProp("legend", value); }
        public bool Stepped { get => Bool(GetProp("stepped")); set => SetProp("stepped", value); }
        public double Tension { get => Number(GetProp("tension")); set => SetProp("tension", value); }
        public bool Animate { get => Bool(GetProp("animate")); set => SetProp("animate", value); }
        public bool Zoom { get => Bool(GetProp("zoom")); set => SetProp("zoom", value); }
        public bool AutoScale { get => Bool(GetProp("auto_scale")); set => SetProp("auto_scale", value); }
        public double YMin { get => Number(GetProp("y_min")); set => SetProp("y_min", value); }
        public double YMax { get => Number(GetProp("y_max")); set => SetProp("y_max", value); }
        public bool LogScale { get => Bool(GetProp("log_scale")); set => SetProp("log_scale", value); }
        public bool Reverse { get => Bool(GetProp("reverse")); set => SetProp("reverse", value); }
        public bool TimeAxis { get => Bool(GetProp("time_axis")); set => SetProp("time_axis", value); }
        public long ClickedSeries { get => (long)Number(GetProp("clicked_series")); set => SetProp("clicked_series", value); }
        public long ClickedIndex { get => (long)Number(GetProp("clicked_index")); set => SetProp("clicked_index", value); }
        public string ValueFormat { get => Text(GetProp("value_format")); set => SetProp("value_format", value); }
        public double LabelAngle { get => Number(GetProp("label_angle")); set => SetProp("label_angle", value); }
        public string TitleAlign { get => Text(GetProp("title_align")); set => SetProp("title_align", value); }
        public bool TitleBold { get => Bool(GetProp("title_bold")); set => SetProp("title_bold", value); }
        public double AreaOpacity { get => Number(GetProp("area_opacity")); set => SetProp("area_opacity", value); }
        public string LegendPosition { get => Text(GetProp("legend_position")); set => SetProp("legend_position", value); }
        public bool Stacked { get => Bool(GetProp("stacked")); set => SetProp("stacked", value); }
        public string Colors { get => Text(GetProp("colors")); set => SetProp("colors", value); }
        public string CenterText { get => Text(GetProp("center_text")); set => SetProp("center_text", value); }
        public string SliceLabels { get => Text(GetProp("slice_labels")); set => SetProp("slice_labels", value); }
        public double Left { get => Number(GetProp("left")); set => SetProp("left", value); }
        public double Top { get => Number(GetProp("top")); set => SetProp("top", value); }
        public double Width { get => Number(GetProp("width")); set => SetProp("width", value); }
        public double Height { get => Number(GetProp("height")); set => SetProp("height", value); }
        public string Align { get => Text(GetProp("align")); set => SetProp("align", value); }
        public string Anchors { get => Text(GetProp("anchors")); set => SetProp("anchors", value); }
        public string Margins { get => Text(GetProp("margins")); set => SetProp("margins", value); }
        public string Padding { get => Text(GetProp("padding")); set => SetProp("padding", value); }
        public bool Visible { get => Bool(GetProp("visible")); set => SetProp("visible", value); }
        public bool Enabled { get => Bool(GetProp("enabled")); set => SetProp("enabled", value); }
        public bool HitTest { get => Bool(GetProp("hit_test")); set => SetProp("hit_test", value); }
        public bool ClipChildren { get => Bool(GetProp("clip_children")); set => SetProp("clip_children", value); }
        public bool InheritState { get => Bool(GetProp("inherit_state")); set => SetProp("inherit_state", value); }
        public bool Translate { get => Bool(GetProp("translate")); set => SetProp("translate", value); }
        public bool CapturesWheel { get => Bool(GetProp("captures_wheel")); set => SetProp("captures_wheel", value); }
        public bool CanFocus { get => Bool(GetProp("can_focus")); set => SetProp("can_focus", value); }
        public bool Draggable { get => Bool(GetProp("draggable")); set => SetProp("draggable", value); }
        public bool AcceptsDrops { get => Bool(GetProp("accepts_drops")); set => SetProp("accepts_drops", value); }
        public string DragData { get => Text(GetProp("drag_data")); set => SetProp("drag_data", value); }
        public string DragLabel { get => Text(GetProp("drag_label")); set => SetProp("drag_label", value); }
        public string DragImage { get => Text(GetProp("drag_image")); set => SetProp("drag_image", value); }
        public string DropTypes { get => Text(GetProp("drop_types")); set => SetProp("drop_types", value); }
        public bool TabStop { get => Bool(GetProp("tab_stop")); set => SetProp("tab_stop", value); }
        public string Cursor { get => Text(GetProp("cursor")); set => SetProp("cursor", value); }
        public double Opacity { get => Number(GetProp("opacity")); set => SetProp("opacity", value); }
        public bool CacheLayer { get => Bool(GetProp("cache_layer")); set => SetProp("cache_layer", value); }
        public double Rotation { get => Number(GetProp("rotation")); set => SetProp("rotation", value); }
        public double TranslateX { get => Number(GetProp("translate_x")); set => SetProp("translate_x", value); }
        public double TranslateY { get => Number(GetProp("translate_y")); set => SetProp("translate_y", value); }
        public double AspectRatio { get => Number(GetProp("aspect_ratio")); set => SetProp("aspect_ratio", value); }
        public double MaxWidth { get => Number(GetProp("max_width")); set => SetProp("max_width", value); }
        public double MinWidth { get => Number(GetProp("min_width")); set => SetProp("min_width", value); }
        public double MinHeight { get => Number(GetProp("min_height")); set => SetProp("min_height", value); }
        public double MaxHeight { get => Number(GetProp("max_height")); set => SetProp("max_height", value); }
        public string SelfAlign { get => Text(GetProp("self_align")); set => SetProp("self_align", value); }
        public bool AutoSize { get => Bool(GetProp("auto_size")); set => SetProp("auto_size", value); }
        public long ColumnSpan { get => (long)Number(GetProp("column_span")); set => SetProp("column_span", value); }
        public long GridRow { get => (long)Number(GetProp("grid_row")); set => SetProp("grid_row", value); }
        public long GridColumn { get => (long)Number(GetProp("grid_column")); set => SetProp("grid_column", value); }
        public long RowSpan { get => (long)Number(GetProp("row_span")); set => SetProp("row_span", value); }
        public double Grow { get => Number(GetProp("grow")); set => SetProp("grow", value); }
        public string Shadow { get => Text(GetProp("shadow")); set => SetProp("shadow", value); }
        public string Style { get => Text(GetProp("style")); set => SetProp("style", value); }
        public string Theme { get => Text(GetProp("theme")); set => SetProp("theme", value); }
        public string Fill { get => Text(GetProp("fill")); set => SetProp("fill", value); }
        public string BorderColor { get => Text(GetProp("border_color")); set => SetProp("border_color", value); }
        public string TextColor { get => Text(GetProp("text_color")); set => SetProp("text_color", value); }
        public string AccentColor { get => Text(GetProp("accent_color")); set => SetProp("accent_color", value); }
        public string SuccessColor { get => Text(GetProp("success_color")); set => SetProp("success_color", value); }
        public string WarningColor { get => Text(GetProp("warning_color")); set => SetProp("warning_color", value); }
        public string DangerColor { get => Text(GetProp("danger_color")); set => SetProp("danger_color", value); }
        public string FontFamily { get => Text(GetProp("font_family")); set => SetProp("font_family", value); }
        public double FontSize { get => Number(GetProp("font_size")); set => SetProp("font_size", value); }
        public string FontWeight { get => Text(GetProp("font_weight")); set => SetProp("font_weight", value); }
        public bool FontItalic { get => Bool(GetProp("font_italic")); set => SetProp("font_italic", value); }
        public bool FontUnderline { get => Bool(GetProp("font_underline")); set => SetProp("font_underline", value); }
        public bool FontStrikeout { get => Bool(GetProp("font_strikeout")); set => SetProp("font_strikeout", value); }
        public string Hint { get => Text(GetProp("hint")); set => SetProp("hint", value); }
        public long Tag { get => (long)Number(GetProp("tag")); set => SetProp("tag", value); }
        public string TagString { get => Text(GetProp("tag_string")); set => SetProp("tag_string", value); }
        public string AccessibleName { get => Text(GetProp("accessible_name")); set => SetProp("accessible_name", value); }
        public string PopupMenu { get => Text(GetProp("popup_menu")); set => SetProp("popup_menu", value); }
    }

    /// <summary>Um DataGrid de um form.</summary>
    public class DataGrid(Aylis aylis, long id, string name) : AylisComponent(aylis, id, name)
    {
        public string DataSource { get => Text(GetProp("data_source")); set => SetProp("data_source", value); }
        public string Columns { get => Text(GetProp("columns")); set => SetProp("columns", value); }
        public double RowHeight { get => Number(GetProp("row_height")); set => SetProp("row_height", value); }
        public bool ReadOnly { get => Bool(GetProp("read_only")); set => SetProp("read_only", value); }
        public bool Sortable { get => Bool(GetProp("sortable")); set => SetProp("sortable", value); }
        public bool ColumnMove { get => Bool(GetProp("column_move")); set => SetProp("column_move", value); }
        public long FixedColumns { get => (long)Number(GetProp("fixed_columns")); set => SetProp("fixed_columns", value); }
        public bool FilterRow { get => Bool(GetProp("filter_row")); set => SetProp("filter_row", value); }
        public bool MultiSelect { get => Bool(GetProp("multi_select")); set => SetProp("multi_select", value); }
        public string Footer { get => Text(GetProp("footer")); set => SetProp("footer", value); }
        public bool IncrementalSearch { get => Bool(GetProp("incremental_search")); set => SetProp("incremental_search", value); }
        public string CursorColor { get => Text(GetProp("cursor_color")); set => SetProp("cursor_color", value); }
        public bool Indicator { get => Bool(GetProp("indicator")); set => SetProp("indicator", value); }
        public bool HighlightCurrent { get => Bool(GetProp("highlight_current")); set => SetProp("highlight_current", value); }
        public double Left { get => Number(GetProp("left")); set => SetProp("left", value); }
        public double Top { get => Number(GetProp("top")); set => SetProp("top", value); }
        public double Width { get => Number(GetProp("width")); set => SetProp("width", value); }
        public double Height { get => Number(GetProp("height")); set => SetProp("height", value); }
        public string Align { get => Text(GetProp("align")); set => SetProp("align", value); }
        public string Anchors { get => Text(GetProp("anchors")); set => SetProp("anchors", value); }
        public string Margins { get => Text(GetProp("margins")); set => SetProp("margins", value); }
        public string Padding { get => Text(GetProp("padding")); set => SetProp("padding", value); }
        public bool Visible { get => Bool(GetProp("visible")); set => SetProp("visible", value); }
        public bool Enabled { get => Bool(GetProp("enabled")); set => SetProp("enabled", value); }
        public bool HitTest { get => Bool(GetProp("hit_test")); set => SetProp("hit_test", value); }
        public bool ClipChildren { get => Bool(GetProp("clip_children")); set => SetProp("clip_children", value); }
        public bool InheritState { get => Bool(GetProp("inherit_state")); set => SetProp("inherit_state", value); }
        public bool Translate { get => Bool(GetProp("translate")); set => SetProp("translate", value); }
        public bool CapturesWheel { get => Bool(GetProp("captures_wheel")); set => SetProp("captures_wheel", value); }
        public bool CanFocus { get => Bool(GetProp("can_focus")); set => SetProp("can_focus", value); }
        public bool Draggable { get => Bool(GetProp("draggable")); set => SetProp("draggable", value); }
        public bool AcceptsDrops { get => Bool(GetProp("accepts_drops")); set => SetProp("accepts_drops", value); }
        public string DragData { get => Text(GetProp("drag_data")); set => SetProp("drag_data", value); }
        public string DragLabel { get => Text(GetProp("drag_label")); set => SetProp("drag_label", value); }
        public string DragImage { get => Text(GetProp("drag_image")); set => SetProp("drag_image", value); }
        public string DropTypes { get => Text(GetProp("drop_types")); set => SetProp("drop_types", value); }
        public bool TabStop { get => Bool(GetProp("tab_stop")); set => SetProp("tab_stop", value); }
        public string Cursor { get => Text(GetProp("cursor")); set => SetProp("cursor", value); }
        public double Opacity { get => Number(GetProp("opacity")); set => SetProp("opacity", value); }
        public bool CacheLayer { get => Bool(GetProp("cache_layer")); set => SetProp("cache_layer", value); }
        public double Rotation { get => Number(GetProp("rotation")); set => SetProp("rotation", value); }
        public double TranslateX { get => Number(GetProp("translate_x")); set => SetProp("translate_x", value); }
        public double TranslateY { get => Number(GetProp("translate_y")); set => SetProp("translate_y", value); }
        public double AspectRatio { get => Number(GetProp("aspect_ratio")); set => SetProp("aspect_ratio", value); }
        public double MaxWidth { get => Number(GetProp("max_width")); set => SetProp("max_width", value); }
        public double MinWidth { get => Number(GetProp("min_width")); set => SetProp("min_width", value); }
        public double MinHeight { get => Number(GetProp("min_height")); set => SetProp("min_height", value); }
        public double MaxHeight { get => Number(GetProp("max_height")); set => SetProp("max_height", value); }
        public string SelfAlign { get => Text(GetProp("self_align")); set => SetProp("self_align", value); }
        public bool AutoSize { get => Bool(GetProp("auto_size")); set => SetProp("auto_size", value); }
        public long ColumnSpan { get => (long)Number(GetProp("column_span")); set => SetProp("column_span", value); }
        public long GridRow { get => (long)Number(GetProp("grid_row")); set => SetProp("grid_row", value); }
        public long GridColumn { get => (long)Number(GetProp("grid_column")); set => SetProp("grid_column", value); }
        public long RowSpan { get => (long)Number(GetProp("row_span")); set => SetProp("row_span", value); }
        public double Grow { get => Number(GetProp("grow")); set => SetProp("grow", value); }
        public string Shadow { get => Text(GetProp("shadow")); set => SetProp("shadow", value); }
        public string Style { get => Text(GetProp("style")); set => SetProp("style", value); }
        public string Theme { get => Text(GetProp("theme")); set => SetProp("theme", value); }
        public string Fill { get => Text(GetProp("fill")); set => SetProp("fill", value); }
        public string BorderColor { get => Text(GetProp("border_color")); set => SetProp("border_color", value); }
        public string TextColor { get => Text(GetProp("text_color")); set => SetProp("text_color", value); }
        public string AccentColor { get => Text(GetProp("accent_color")); set => SetProp("accent_color", value); }
        public string SuccessColor { get => Text(GetProp("success_color")); set => SetProp("success_color", value); }
        public string WarningColor { get => Text(GetProp("warning_color")); set => SetProp("warning_color", value); }
        public string DangerColor { get => Text(GetProp("danger_color")); set => SetProp("danger_color", value); }
        public string FontFamily { get => Text(GetProp("font_family")); set => SetProp("font_family", value); }
        public double FontSize { get => Number(GetProp("font_size")); set => SetProp("font_size", value); }
        public string FontWeight { get => Text(GetProp("font_weight")); set => SetProp("font_weight", value); }
        public bool FontItalic { get => Bool(GetProp("font_italic")); set => SetProp("font_italic", value); }
        public bool FontUnderline { get => Bool(GetProp("font_underline")); set => SetProp("font_underline", value); }
        public bool FontStrikeout { get => Bool(GetProp("font_strikeout")); set => SetProp("font_strikeout", value); }
        public string Hint { get => Text(GetProp("hint")); set => SetProp("hint", value); }
        public long Tag { get => (long)Number(GetProp("tag")); set => SetProp("tag", value); }
        public string TagString { get => Text(GetProp("tag_string")); set => SetProp("tag_string", value); }
        public string AccessibleName { get => Text(GetProp("accessible_name")); set => SetProp("accessible_name", value); }
        public string PopupMenu { get => Text(GetProp("popup_menu")); set => SetProp("popup_menu", value); }
    }

    /// <summary>Um DataSource de um form.</summary>
    public class DataSource(Aylis aylis, long id, string name) : AylisComponent(aylis, id, name)
    {
        public string DataSet { get => Text(GetProp("data_set")); set => SetProp("data_set", value); }
        public bool AutoEdit { get => Bool(GetProp("auto_edit")); set => SetProp("auto_edit", value); }
        public bool Enabled { get => Bool(GetProp("enabled")); set => SetProp("enabled", value); }
    }

    /// <summary>Um Field de um form.</summary>
    public class Field(Aylis aylis, long id, string name) : AylisComponent(aylis, id, name)
    {
        public string FieldName { get => Text(GetProp("field_name")); set => SetProp("field_name", value); }
        public string Type { get => Text(GetProp("type")); set => SetProp("type", value); }
        public long Size { get => (long)Number(GetProp("size")); set => SetProp("size", value); }
        public long Scale { get => (long)Number(GetProp("scale")); set => SetProp("scale", value); }
        public string Rounding { get => Text(GetProp("rounding")); set => SetProp("rounding", value); }
        public string DisplayLabel { get => Text(GetProp("display_label")); set => SetProp("display_label", value); }
        public double DisplayWidth { get => Number(GetProp("display_width")); set => SetProp("display_width", value); }
        public string DisplayFormat { get => Text(GetProp("display_format")); set => SetProp("display_format", value); }
        public string EditFormat { get => Text(GetProp("edit_format")); set => SetProp("edit_format", value); }
        public string Alignment { get => Text(GetProp("alignment")); set => SetProp("alignment", value); }
        public bool Required { get => Bool(GetProp("required")); set => SetProp("required", value); }
        public bool ReadOnly { get => Bool(GetProp("read_only")); set => SetProp("read_only", value); }
        public bool Visible { get => Bool(GetProp("visible")); set => SetProp("visible", value); }
        public double Min { get => Number(GetProp("min")); set => SetProp("min", value); }
        public double Max { get => Number(GetProp("max")); set => SetProp("max", value); }
        public string DefaultValue { get => Text(GetProp("default_value")); set => SetProp("default_value", value); }
        public string DisplayValues { get => Text(GetProp("display_values")); set => SetProp("display_values", value); }
        public string CharCase { get => Text(GetProp("char_case")); set => SetProp("char_case", value); }
        public string EditMask { get => Text(GetProp("edit_mask")); set => SetProp("edit_mask", value); }
        public bool Sensitive { get => Bool(GetProp("sensitive")); set => SetProp("sensitive", value); }
        public string Kind { get => Text(GetProp("kind")); set => SetProp("kind", value); }
        public string Expression { get => Text(GetProp("expression")); set => SetProp("expression", value); }
        public string LookupDataSet { get => Text(GetProp("lookup_data_set")); set => SetProp("lookup_data_set", value); }
        public string KeyFields { get => Text(GetProp("key_fields")); set => SetProp("key_fields", value); }
        public string LookupKeyFields { get => Text(GetProp("lookup_key_fields")); set => SetProp("lookup_key_fields", value); }
        public string LookupResultField { get => Text(GetProp("lookup_result_field")); set => SetProp("lookup_result_field", value); }
    }

    /// <summary>Um Form de um form.</summary>
    public class Form(Aylis aylis, long id, string name) : AylisForm(aylis, id, name)
    {
        public string Caption { get => Text(GetProp("caption")); set => SetProp("caption", value); }
        public string Position { get => Text(GetProp("position")); set => SetProp("position", value); }
        public string BorderStyle { get => Text(GetProp("border_style")); set => SetProp("border_style", value); }
        public string WindowState { get => Text(GetProp("window_state")); set => SetProp("window_state", value); }
        public string Transition { get => Text(GetProp("transition")); set => SetProp("transition", value); }
        public bool Topmost { get => Bool(GetProp("topmost")); set => SetProp("topmost", value); }
        public bool ShowInTaskbar { get => Bool(GetProp("show_in_taskbar")); set => SetProp("show_in_taskbar", value); }
        public string ActiveControl { get => Text(GetProp("active_control")); set => SetProp("active_control", value); }
        public string TabOrder { get => Text(GetProp("tab_order")); set => SetProp("tab_order", value); }
        public string TabKeys { get => Text(GetProp("tab_keys")); set => SetProp("tab_keys", value); }
        public string BackTabKeys { get => Text(GetProp("back_tab_keys")); set => SetProp("back_tab_keys", value); }
        public string AllowedChars { get => Text(GetProp("allowed_chars")); set => SetProp("allowed_chars", value); }
        public string BlockedChars { get => Text(GetProp("blocked_chars")); set => SetProp("blocked_chars", value); }
        public bool OnlyAllowed { get => Bool(GetProp("only_allowed")); set => SetProp("only_allowed", value); }
        public bool CharsOverride { get => Bool(GetProp("chars_override")); set => SetProp("chars_override", value); }
        public string CharMap { get => Text(GetProp("char_map")); set => SetProp("char_map", value); }
        public bool KeyPreview { get => Bool(GetProp("key_preview")); set => SetProp("key_preview", value); }
        public double Zoom { get => Number(GetProp("zoom")); set => SetProp("zoom", value); }
        public long Monitor { get => (long)Number(GetProp("monitor")); set => SetProp("monitor", value); }
        public bool RememberPlacement { get => Bool(GetProp("remember_placement")); set => SetProp("remember_placement", value); }
        public string Gpu { get => Text(GetProp("gpu")); set => SetProp("gpu", value); }
        public bool ActivateOnShow { get => Bool(GetProp("activate_on_show")); set => SetProp("activate_on_show", value); }
        public string CloseAction { get => Text(GetProp("close_action")); set => SetProp("close_action", value); }
        public double FrameRate { get => Number(GetProp("frame_rate")); set => SetProp("frame_rate", value); }
        public double Left { get => Number(GetProp("left")); set => SetProp("left", value); }
        public double Top { get => Number(GetProp("top")); set => SetProp("top", value); }
        public double Width { get => Number(GetProp("width")); set => SetProp("width", value); }
        public double Height { get => Number(GetProp("height")); set => SetProp("height", value); }
        public string Align { get => Text(GetProp("align")); set => SetProp("align", value); }
        public string Anchors { get => Text(GetProp("anchors")); set => SetProp("anchors", value); }
        public string Margins { get => Text(GetProp("margins")); set => SetProp("margins", value); }
        public string Padding { get => Text(GetProp("padding")); set => SetProp("padding", value); }
        public bool Visible { get => Bool(GetProp("visible")); set => SetProp("visible", value); }
        public bool Enabled { get => Bool(GetProp("enabled")); set => SetProp("enabled", value); }
        public bool HitTest { get => Bool(GetProp("hit_test")); set => SetProp("hit_test", value); }
        public bool ClipChildren { get => Bool(GetProp("clip_children")); set => SetProp("clip_children", value); }
        public bool InheritState { get => Bool(GetProp("inherit_state")); set => SetProp("inherit_state", value); }
        public bool Translate { get => Bool(GetProp("translate")); set => SetProp("translate", value); }
        public bool CapturesWheel { get => Bool(GetProp("captures_wheel")); set => SetProp("captures_wheel", value); }
        public bool CanFocus { get => Bool(GetProp("can_focus")); set => SetProp("can_focus", value); }
        public bool Draggable { get => Bool(GetProp("draggable")); set => SetProp("draggable", value); }
        public bool AcceptsDrops { get => Bool(GetProp("accepts_drops")); set => SetProp("accepts_drops", value); }
        public string DragData { get => Text(GetProp("drag_data")); set => SetProp("drag_data", value); }
        public string DragLabel { get => Text(GetProp("drag_label")); set => SetProp("drag_label", value); }
        public string DragImage { get => Text(GetProp("drag_image")); set => SetProp("drag_image", value); }
        public string DropTypes { get => Text(GetProp("drop_types")); set => SetProp("drop_types", value); }
        public bool TabStop { get => Bool(GetProp("tab_stop")); set => SetProp("tab_stop", value); }
        public string Cursor { get => Text(GetProp("cursor")); set => SetProp("cursor", value); }
        public double Opacity { get => Number(GetProp("opacity")); set => SetProp("opacity", value); }
        public bool CacheLayer { get => Bool(GetProp("cache_layer")); set => SetProp("cache_layer", value); }
        public double Rotation { get => Number(GetProp("rotation")); set => SetProp("rotation", value); }
        public double TranslateX { get => Number(GetProp("translate_x")); set => SetProp("translate_x", value); }
        public double TranslateY { get => Number(GetProp("translate_y")); set => SetProp("translate_y", value); }
        public double AspectRatio { get => Number(GetProp("aspect_ratio")); set => SetProp("aspect_ratio", value); }
        public double MaxWidth { get => Number(GetProp("max_width")); set => SetProp("max_width", value); }
        public double MinWidth { get => Number(GetProp("min_width")); set => SetProp("min_width", value); }
        public double MinHeight { get => Number(GetProp("min_height")); set => SetProp("min_height", value); }
        public double MaxHeight { get => Number(GetProp("max_height")); set => SetProp("max_height", value); }
        public string SelfAlign { get => Text(GetProp("self_align")); set => SetProp("self_align", value); }
        public bool AutoSize { get => Bool(GetProp("auto_size")); set => SetProp("auto_size", value); }
        public long ColumnSpan { get => (long)Number(GetProp("column_span")); set => SetProp("column_span", value); }
        public long GridRow { get => (long)Number(GetProp("grid_row")); set => SetProp("grid_row", value); }
        public long GridColumn { get => (long)Number(GetProp("grid_column")); set => SetProp("grid_column", value); }
        public long RowSpan { get => (long)Number(GetProp("row_span")); set => SetProp("row_span", value); }
        public double Grow { get => Number(GetProp("grow")); set => SetProp("grow", value); }
        public string Shadow { get => Text(GetProp("shadow")); set => SetProp("shadow", value); }
        public string Style { get => Text(GetProp("style")); set => SetProp("style", value); }
        public string Theme { get => Text(GetProp("theme")); set => SetProp("theme", value); }
        public string Fill { get => Text(GetProp("fill")); set => SetProp("fill", value); }
        public string BorderColor { get => Text(GetProp("border_color")); set => SetProp("border_color", value); }
        public string TextColor { get => Text(GetProp("text_color")); set => SetProp("text_color", value); }
        public string AccentColor { get => Text(GetProp("accent_color")); set => SetProp("accent_color", value); }
        public string SuccessColor { get => Text(GetProp("success_color")); set => SetProp("success_color", value); }
        public string WarningColor { get => Text(GetProp("warning_color")); set => SetProp("warning_color", value); }
        public string DangerColor { get => Text(GetProp("danger_color")); set => SetProp("danger_color", value); }
        public string FontFamily { get => Text(GetProp("font_family")); set => SetProp("font_family", value); }
        public double FontSize { get => Number(GetProp("font_size")); set => SetProp("font_size", value); }
        public string FontWeight { get => Text(GetProp("font_weight")); set => SetProp("font_weight", value); }
        public bool FontItalic { get => Bool(GetProp("font_italic")); set => SetProp("font_italic", value); }
        public bool FontUnderline { get => Bool(GetProp("font_underline")); set => SetProp("font_underline", value); }
        public bool FontStrikeout { get => Bool(GetProp("font_strikeout")); set => SetProp("font_strikeout", value); }
        public string Hint { get => Text(GetProp("hint")); set => SetProp("hint", value); }
        public long Tag { get => (long)Number(GetProp("tag")); set => SetProp("tag", value); }
        public string TagString { get => Text(GetProp("tag_string")); set => SetProp("tag_string", value); }
        public string AccessibleName { get => Text(GetProp("accessible_name")); set => SetProp("accessible_name", value); }
        public string PopupMenu { get => Text(GetProp("popup_menu")); set => SetProp("popup_menu", value); }
        public void MoveToMonitor(long monitor) => Aylis.Send("invoke", new JsonObject { ["target"] = Id, ["method"] = "move_to_monitor", ["args"] = new JsonArray(monitor) });
        public string GpuAdapter() => Text(Aylis.Call("invoke", new JsonObject { ["target"] = Id, ["method"] = "gpu_adapter", ["args"] = new JsonArray() }));
        public string GpuAdapters() => Text(Aylis.Call("invoke", new JsonObject { ["target"] = Id, ["method"] = "gpu_adapters", ["args"] = new JsonArray() }));
        public string Focused() => Text(Aylis.Call("focused", new JsonObject { ["target"] = Id }));
        public void FocusNext() => Aylis.Send("focus_next", new JsonObject { ["target"] = Id });
        public void FocusPrev() => Aylis.Send("focus_prev", new JsonObject { ["target"] = Id });
    }

    /// <summary>Um GroupBox de um form.</summary>
    public class GroupBox(Aylis aylis, long id, string name) : AylisComponent(aylis, id, name)
    {
        public string Caption { get => Text(GetProp("caption")); set => SetProp("caption", value); }
        public string AllowedChars { get => Text(GetProp("allowed_chars")); set => SetProp("allowed_chars", value); }
        public string BlockedChars { get => Text(GetProp("blocked_chars")); set => SetProp("blocked_chars", value); }
        public bool OnlyAllowed { get => Bool(GetProp("only_allowed")); set => SetProp("only_allowed", value); }
        public bool CharsOverride { get => Bool(GetProp("chars_override")); set => SetProp("chars_override", value); }
        public string CharMap { get => Text(GetProp("char_map")); set => SetProp("char_map", value); }
        public double Left { get => Number(GetProp("left")); set => SetProp("left", value); }
        public double Top { get => Number(GetProp("top")); set => SetProp("top", value); }
        public double Width { get => Number(GetProp("width")); set => SetProp("width", value); }
        public double Height { get => Number(GetProp("height")); set => SetProp("height", value); }
        public string Align { get => Text(GetProp("align")); set => SetProp("align", value); }
        public string Anchors { get => Text(GetProp("anchors")); set => SetProp("anchors", value); }
        public string Margins { get => Text(GetProp("margins")); set => SetProp("margins", value); }
        public string Padding { get => Text(GetProp("padding")); set => SetProp("padding", value); }
        public bool Visible { get => Bool(GetProp("visible")); set => SetProp("visible", value); }
        public bool Enabled { get => Bool(GetProp("enabled")); set => SetProp("enabled", value); }
        public bool HitTest { get => Bool(GetProp("hit_test")); set => SetProp("hit_test", value); }
        public bool ClipChildren { get => Bool(GetProp("clip_children")); set => SetProp("clip_children", value); }
        public bool InheritState { get => Bool(GetProp("inherit_state")); set => SetProp("inherit_state", value); }
        public bool Translate { get => Bool(GetProp("translate")); set => SetProp("translate", value); }
        public bool CapturesWheel { get => Bool(GetProp("captures_wheel")); set => SetProp("captures_wheel", value); }
        public bool CanFocus { get => Bool(GetProp("can_focus")); set => SetProp("can_focus", value); }
        public bool Draggable { get => Bool(GetProp("draggable")); set => SetProp("draggable", value); }
        public bool AcceptsDrops { get => Bool(GetProp("accepts_drops")); set => SetProp("accepts_drops", value); }
        public string DragData { get => Text(GetProp("drag_data")); set => SetProp("drag_data", value); }
        public string DragLabel { get => Text(GetProp("drag_label")); set => SetProp("drag_label", value); }
        public string DragImage { get => Text(GetProp("drag_image")); set => SetProp("drag_image", value); }
        public string DropTypes { get => Text(GetProp("drop_types")); set => SetProp("drop_types", value); }
        public bool TabStop { get => Bool(GetProp("tab_stop")); set => SetProp("tab_stop", value); }
        public string Cursor { get => Text(GetProp("cursor")); set => SetProp("cursor", value); }
        public double Opacity { get => Number(GetProp("opacity")); set => SetProp("opacity", value); }
        public bool CacheLayer { get => Bool(GetProp("cache_layer")); set => SetProp("cache_layer", value); }
        public double Rotation { get => Number(GetProp("rotation")); set => SetProp("rotation", value); }
        public double TranslateX { get => Number(GetProp("translate_x")); set => SetProp("translate_x", value); }
        public double TranslateY { get => Number(GetProp("translate_y")); set => SetProp("translate_y", value); }
        public double AspectRatio { get => Number(GetProp("aspect_ratio")); set => SetProp("aspect_ratio", value); }
        public double MaxWidth { get => Number(GetProp("max_width")); set => SetProp("max_width", value); }
        public double MinWidth { get => Number(GetProp("min_width")); set => SetProp("min_width", value); }
        public double MinHeight { get => Number(GetProp("min_height")); set => SetProp("min_height", value); }
        public double MaxHeight { get => Number(GetProp("max_height")); set => SetProp("max_height", value); }
        public string SelfAlign { get => Text(GetProp("self_align")); set => SetProp("self_align", value); }
        public bool AutoSize { get => Bool(GetProp("auto_size")); set => SetProp("auto_size", value); }
        public long ColumnSpan { get => (long)Number(GetProp("column_span")); set => SetProp("column_span", value); }
        public long GridRow { get => (long)Number(GetProp("grid_row")); set => SetProp("grid_row", value); }
        public long GridColumn { get => (long)Number(GetProp("grid_column")); set => SetProp("grid_column", value); }
        public long RowSpan { get => (long)Number(GetProp("row_span")); set => SetProp("row_span", value); }
        public double Grow { get => Number(GetProp("grow")); set => SetProp("grow", value); }
        public string Shadow { get => Text(GetProp("shadow")); set => SetProp("shadow", value); }
        public string Style { get => Text(GetProp("style")); set => SetProp("style", value); }
        public string Theme { get => Text(GetProp("theme")); set => SetProp("theme", value); }
        public string Fill { get => Text(GetProp("fill")); set => SetProp("fill", value); }
        public string BorderColor { get => Text(GetProp("border_color")); set => SetProp("border_color", value); }
        public string TextColor { get => Text(GetProp("text_color")); set => SetProp("text_color", value); }
        public string AccentColor { get => Text(GetProp("accent_color")); set => SetProp("accent_color", value); }
        public string SuccessColor { get => Text(GetProp("success_color")); set => SetProp("success_color", value); }
        public string WarningColor { get => Text(GetProp("warning_color")); set => SetProp("warning_color", value); }
        public string DangerColor { get => Text(GetProp("danger_color")); set => SetProp("danger_color", value); }
        public string FontFamily { get => Text(GetProp("font_family")); set => SetProp("font_family", value); }
        public double FontSize { get => Number(GetProp("font_size")); set => SetProp("font_size", value); }
        public string FontWeight { get => Text(GetProp("font_weight")); set => SetProp("font_weight", value); }
        public bool FontItalic { get => Bool(GetProp("font_italic")); set => SetProp("font_italic", value); }
        public bool FontUnderline { get => Bool(GetProp("font_underline")); set => SetProp("font_underline", value); }
        public bool FontStrikeout { get => Bool(GetProp("font_strikeout")); set => SetProp("font_strikeout", value); }
        public string Hint { get => Text(GetProp("hint")); set => SetProp("hint", value); }
        public long Tag { get => (long)Number(GetProp("tag")); set => SetProp("tag", value); }
        public string TagString { get => Text(GetProp("tag_string")); set => SetProp("tag_string", value); }
        public string AccessibleName { get => Text(GetProp("accessible_name")); set => SetProp("accessible_name", value); }
        public string PopupMenu { get => Text(GetProp("popup_menu")); set => SetProp("popup_menu", value); }
    }

    /// <summary>Um Image de um form.</summary>
    public class Image(Aylis aylis, long id, string name) : AylisComponent(aylis, id, name)
    {
        public string ImageValue { get => Text(GetProp("image")); set => SetProp("image", value); }
        public string Fit { get => Text(GetProp("fit")); set => SetProp("fit", value); }
        public double Radius { get => Number(GetProp("radius")); set => SetProp("radius", value); }
        public double Left { get => Number(GetProp("left")); set => SetProp("left", value); }
        public double Top { get => Number(GetProp("top")); set => SetProp("top", value); }
        public double Width { get => Number(GetProp("width")); set => SetProp("width", value); }
        public double Height { get => Number(GetProp("height")); set => SetProp("height", value); }
        public string Align { get => Text(GetProp("align")); set => SetProp("align", value); }
        public string Anchors { get => Text(GetProp("anchors")); set => SetProp("anchors", value); }
        public string Margins { get => Text(GetProp("margins")); set => SetProp("margins", value); }
        public string Padding { get => Text(GetProp("padding")); set => SetProp("padding", value); }
        public bool Visible { get => Bool(GetProp("visible")); set => SetProp("visible", value); }
        public bool Enabled { get => Bool(GetProp("enabled")); set => SetProp("enabled", value); }
        public bool HitTest { get => Bool(GetProp("hit_test")); set => SetProp("hit_test", value); }
        public bool ClipChildren { get => Bool(GetProp("clip_children")); set => SetProp("clip_children", value); }
        public bool InheritState { get => Bool(GetProp("inherit_state")); set => SetProp("inherit_state", value); }
        public bool Translate { get => Bool(GetProp("translate")); set => SetProp("translate", value); }
        public bool CapturesWheel { get => Bool(GetProp("captures_wheel")); set => SetProp("captures_wheel", value); }
        public bool CanFocus { get => Bool(GetProp("can_focus")); set => SetProp("can_focus", value); }
        public bool Draggable { get => Bool(GetProp("draggable")); set => SetProp("draggable", value); }
        public bool AcceptsDrops { get => Bool(GetProp("accepts_drops")); set => SetProp("accepts_drops", value); }
        public string DragData { get => Text(GetProp("drag_data")); set => SetProp("drag_data", value); }
        public string DragLabel { get => Text(GetProp("drag_label")); set => SetProp("drag_label", value); }
        public string DragImage { get => Text(GetProp("drag_image")); set => SetProp("drag_image", value); }
        public string DropTypes { get => Text(GetProp("drop_types")); set => SetProp("drop_types", value); }
        public bool TabStop { get => Bool(GetProp("tab_stop")); set => SetProp("tab_stop", value); }
        public string Cursor { get => Text(GetProp("cursor")); set => SetProp("cursor", value); }
        public double Opacity { get => Number(GetProp("opacity")); set => SetProp("opacity", value); }
        public bool CacheLayer { get => Bool(GetProp("cache_layer")); set => SetProp("cache_layer", value); }
        public double Rotation { get => Number(GetProp("rotation")); set => SetProp("rotation", value); }
        public double TranslateX { get => Number(GetProp("translate_x")); set => SetProp("translate_x", value); }
        public double TranslateY { get => Number(GetProp("translate_y")); set => SetProp("translate_y", value); }
        public double AspectRatio { get => Number(GetProp("aspect_ratio")); set => SetProp("aspect_ratio", value); }
        public double MaxWidth { get => Number(GetProp("max_width")); set => SetProp("max_width", value); }
        public double MinWidth { get => Number(GetProp("min_width")); set => SetProp("min_width", value); }
        public double MinHeight { get => Number(GetProp("min_height")); set => SetProp("min_height", value); }
        public double MaxHeight { get => Number(GetProp("max_height")); set => SetProp("max_height", value); }
        public string SelfAlign { get => Text(GetProp("self_align")); set => SetProp("self_align", value); }
        public bool AutoSize { get => Bool(GetProp("auto_size")); set => SetProp("auto_size", value); }
        public long ColumnSpan { get => (long)Number(GetProp("column_span")); set => SetProp("column_span", value); }
        public long GridRow { get => (long)Number(GetProp("grid_row")); set => SetProp("grid_row", value); }
        public long GridColumn { get => (long)Number(GetProp("grid_column")); set => SetProp("grid_column", value); }
        public long RowSpan { get => (long)Number(GetProp("row_span")); set => SetProp("row_span", value); }
        public double Grow { get => Number(GetProp("grow")); set => SetProp("grow", value); }
        public string Shadow { get => Text(GetProp("shadow")); set => SetProp("shadow", value); }
        public string Style { get => Text(GetProp("style")); set => SetProp("style", value); }
        public string Theme { get => Text(GetProp("theme")); set => SetProp("theme", value); }
        public string Fill { get => Text(GetProp("fill")); set => SetProp("fill", value); }
        public string BorderColor { get => Text(GetProp("border_color")); set => SetProp("border_color", value); }
        public string TextColor { get => Text(GetProp("text_color")); set => SetProp("text_color", value); }
        public string AccentColor { get => Text(GetProp("accent_color")); set => SetProp("accent_color", value); }
        public string SuccessColor { get => Text(GetProp("success_color")); set => SetProp("success_color", value); }
        public string WarningColor { get => Text(GetProp("warning_color")); set => SetProp("warning_color", value); }
        public string DangerColor { get => Text(GetProp("danger_color")); set => SetProp("danger_color", value); }
        public string FontFamily { get => Text(GetProp("font_family")); set => SetProp("font_family", value); }
        public double FontSize { get => Number(GetProp("font_size")); set => SetProp("font_size", value); }
        public string FontWeight { get => Text(GetProp("font_weight")); set => SetProp("font_weight", value); }
        public bool FontItalic { get => Bool(GetProp("font_italic")); set => SetProp("font_italic", value); }
        public bool FontUnderline { get => Bool(GetProp("font_underline")); set => SetProp("font_underline", value); }
        public bool FontStrikeout { get => Bool(GetProp("font_strikeout")); set => SetProp("font_strikeout", value); }
        public string Hint { get => Text(GetProp("hint")); set => SetProp("hint", value); }
        public long Tag { get => (long)Number(GetProp("tag")); set => SetProp("tag", value); }
        public string TagString { get => Text(GetProp("tag_string")); set => SetProp("tag_string", value); }
        public string AccessibleName { get => Text(GetProp("accessible_name")); set => SetProp("accessible_name", value); }
        public string PopupMenu { get => Text(GetProp("popup_menu")); set => SetProp("popup_menu", value); }
    }

    /// <summary>Um Label de um form.</summary>
    public class Label(Aylis aylis, long id, string name) : AylisComponent(aylis, id, name)
    {
        public string Text { get => Text(GetProp("text")); set => SetProp("text", value); }
        public string HAlign { get => Text(GetProp("h_align")); set => SetProp("h_align", value); }
        public string VAlign { get => Text(GetProp("v_align")); set => SetProp("v_align", value); }
        public bool Wrap { get => Bool(GetProp("wrap")); set => SetProp("wrap", value); }
        public string Trimming { get => Text(GetProp("trimming")); set => SetProp("trimming", value); }
        public bool Bold { get => Bool(GetProp("bold")); set => SetProp("bold", value); }
        public string Color { get => Text(GetProp("color")); set => SetProp("color", value); }
        public string Highlight { get => Text(GetProp("highlight")); set => SetProp("highlight", value); }
        public double LetterSpacing { get => Number(GetProp("letter_spacing")); set => SetProp("letter_spacing", value); }
        public string TextCase { get => Text(GetProp("text_case")); set => SetProp("text_case", value); }
        public bool Markup { get => Bool(GetProp("markup")); set => SetProp("markup", value); }
        public string Outline { get => Text(GetProp("outline")); set => SetProp("outline", value); }
        public double OutlineWidth { get => Number(GetProp("outline_width")); set => SetProp("outline_width", value); }
        public double LineHeight { get => Number(GetProp("line_height")); set => SetProp("line_height", value); }
        public string TextShadow { get => Text(GetProp("text_shadow")); set => SetProp("text_shadow", value); }
        public string FocusControl { get => Text(GetProp("focus_control")); set => SetProp("focus_control", value); }
        public double Left { get => Number(GetProp("left")); set => SetProp("left", value); }
        public double Top { get => Number(GetProp("top")); set => SetProp("top", value); }
        public double Width { get => Number(GetProp("width")); set => SetProp("width", value); }
        public double Height { get => Number(GetProp("height")); set => SetProp("height", value); }
        public string Align { get => Text(GetProp("align")); set => SetProp("align", value); }
        public string Anchors { get => Text(GetProp("anchors")); set => SetProp("anchors", value); }
        public string Margins { get => Text(GetProp("margins")); set => SetProp("margins", value); }
        public string Padding { get => Text(GetProp("padding")); set => SetProp("padding", value); }
        public bool Visible { get => Bool(GetProp("visible")); set => SetProp("visible", value); }
        public bool Enabled { get => Bool(GetProp("enabled")); set => SetProp("enabled", value); }
        public bool HitTest { get => Bool(GetProp("hit_test")); set => SetProp("hit_test", value); }
        public bool ClipChildren { get => Bool(GetProp("clip_children")); set => SetProp("clip_children", value); }
        public bool InheritState { get => Bool(GetProp("inherit_state")); set => SetProp("inherit_state", value); }
        public bool Translate { get => Bool(GetProp("translate")); set => SetProp("translate", value); }
        public bool CapturesWheel { get => Bool(GetProp("captures_wheel")); set => SetProp("captures_wheel", value); }
        public bool CanFocus { get => Bool(GetProp("can_focus")); set => SetProp("can_focus", value); }
        public bool Draggable { get => Bool(GetProp("draggable")); set => SetProp("draggable", value); }
        public bool AcceptsDrops { get => Bool(GetProp("accepts_drops")); set => SetProp("accepts_drops", value); }
        public string DragData { get => Text(GetProp("drag_data")); set => SetProp("drag_data", value); }
        public string DragLabel { get => Text(GetProp("drag_label")); set => SetProp("drag_label", value); }
        public string DragImage { get => Text(GetProp("drag_image")); set => SetProp("drag_image", value); }
        public string DropTypes { get => Text(GetProp("drop_types")); set => SetProp("drop_types", value); }
        public bool TabStop { get => Bool(GetProp("tab_stop")); set => SetProp("tab_stop", value); }
        public string Cursor { get => Text(GetProp("cursor")); set => SetProp("cursor", value); }
        public double Opacity { get => Number(GetProp("opacity")); set => SetProp("opacity", value); }
        public bool CacheLayer { get => Bool(GetProp("cache_layer")); set => SetProp("cache_layer", value); }
        public double Rotation { get => Number(GetProp("rotation")); set => SetProp("rotation", value); }
        public double TranslateX { get => Number(GetProp("translate_x")); set => SetProp("translate_x", value); }
        public double TranslateY { get => Number(GetProp("translate_y")); set => SetProp("translate_y", value); }
        public double AspectRatio { get => Number(GetProp("aspect_ratio")); set => SetProp("aspect_ratio", value); }
        public double MaxWidth { get => Number(GetProp("max_width")); set => SetProp("max_width", value); }
        public double MinWidth { get => Number(GetProp("min_width")); set => SetProp("min_width", value); }
        public double MinHeight { get => Number(GetProp("min_height")); set => SetProp("min_height", value); }
        public double MaxHeight { get => Number(GetProp("max_height")); set => SetProp("max_height", value); }
        public string SelfAlign { get => Text(GetProp("self_align")); set => SetProp("self_align", value); }
        public bool AutoSize { get => Bool(GetProp("auto_size")); set => SetProp("auto_size", value); }
        public long ColumnSpan { get => (long)Number(GetProp("column_span")); set => SetProp("column_span", value); }
        public long GridRow { get => (long)Number(GetProp("grid_row")); set => SetProp("grid_row", value); }
        public long GridColumn { get => (long)Number(GetProp("grid_column")); set => SetProp("grid_column", value); }
        public long RowSpan { get => (long)Number(GetProp("row_span")); set => SetProp("row_span", value); }
        public double Grow { get => Number(GetProp("grow")); set => SetProp("grow", value); }
        public string Shadow { get => Text(GetProp("shadow")); set => SetProp("shadow", value); }
        public string Style { get => Text(GetProp("style")); set => SetProp("style", value); }
        public string Theme { get => Text(GetProp("theme")); set => SetProp("theme", value); }
        public string Fill { get => Text(GetProp("fill")); set => SetProp("fill", value); }
        public string BorderColor { get => Text(GetProp("border_color")); set => SetProp("border_color", value); }
        public string TextColor { get => Text(GetProp("text_color")); set => SetProp("text_color", value); }
        public string AccentColor { get => Text(GetProp("accent_color")); set => SetProp("accent_color", value); }
        public string SuccessColor { get => Text(GetProp("success_color")); set => SetProp("success_color", value); }
        public string WarningColor { get => Text(GetProp("warning_color")); set => SetProp("warning_color", value); }
        public string DangerColor { get => Text(GetProp("danger_color")); set => SetProp("danger_color", value); }
        public string FontFamily { get => Text(GetProp("font_family")); set => SetProp("font_family", value); }
        public double FontSize { get => Number(GetProp("font_size")); set => SetProp("font_size", value); }
        public string FontWeight { get => Text(GetProp("font_weight")); set => SetProp("font_weight", value); }
        public bool FontItalic { get => Bool(GetProp("font_italic")); set => SetProp("font_italic", value); }
        public bool FontUnderline { get => Bool(GetProp("font_underline")); set => SetProp("font_underline", value); }
        public bool FontStrikeout { get => Bool(GetProp("font_strikeout")); set => SetProp("font_strikeout", value); }
        public string Hint { get => Text(GetProp("hint")); set => SetProp("hint", value); }
        public long Tag { get => (long)Number(GetProp("tag")); set => SetProp("tag", value); }
        public string TagString { get => Text(GetProp("tag_string")); set => SetProp("tag_string", value); }
        public string AccessibleName { get => Text(GetProp("accessible_name")); set => SetProp("accessible_name", value); }
        public string PopupMenu { get => Text(GetProp("popup_menu")); set => SetProp("popup_menu", value); }
        public double MinContentWidth() => Number(Aylis.Call("invoke", new JsonObject { ["target"] = Id, ["method"] = "min_content_width", ["args"] = new JsonArray() }));
        public double MaxContentWidth() => Number(Aylis.Call("invoke", new JsonObject { ["target"] = Id, ["method"] = "max_content_width", ["args"] = new JsonArray() }));
    }

    /// <summary>Um Layout de um form.</summary>
    public class Layout(Aylis aylis, long id, string name) : AylisComponent(aylis, id, name)
    {
        public string AllowedChars { get => Text(GetProp("allowed_chars")); set => SetProp("allowed_chars", value); }
        public string BlockedChars { get => Text(GetProp("blocked_chars")); set => SetProp("blocked_chars", value); }
        public bool OnlyAllowed { get => Bool(GetProp("only_allowed")); set => SetProp("only_allowed", value); }
        public bool CharsOverride { get => Bool(GetProp("chars_override")); set => SetProp("chars_override", value); }
        public string CharMap { get => Text(GetProp("char_map")); set => SetProp("char_map", value); }
        public double Gap { get => Number(GetProp("gap")); set => SetProp("gap", value); }
        public double Left { get => Number(GetProp("left")); set => SetProp("left", value); }
        public double Top { get => Number(GetProp("top")); set => SetProp("top", value); }
        public double Width { get => Number(GetProp("width")); set => SetProp("width", value); }
        public double Height { get => Number(GetProp("height")); set => SetProp("height", value); }
        public string Align { get => Text(GetProp("align")); set => SetProp("align", value); }
        public string Anchors { get => Text(GetProp("anchors")); set => SetProp("anchors", value); }
        public string Margins { get => Text(GetProp("margins")); set => SetProp("margins", value); }
        public string Padding { get => Text(GetProp("padding")); set => SetProp("padding", value); }
        public bool Visible { get => Bool(GetProp("visible")); set => SetProp("visible", value); }
        public bool Enabled { get => Bool(GetProp("enabled")); set => SetProp("enabled", value); }
        public bool HitTest { get => Bool(GetProp("hit_test")); set => SetProp("hit_test", value); }
        public bool ClipChildren { get => Bool(GetProp("clip_children")); set => SetProp("clip_children", value); }
        public bool InheritState { get => Bool(GetProp("inherit_state")); set => SetProp("inherit_state", value); }
        public bool Translate { get => Bool(GetProp("translate")); set => SetProp("translate", value); }
        public bool CapturesWheel { get => Bool(GetProp("captures_wheel")); set => SetProp("captures_wheel", value); }
        public bool CanFocus { get => Bool(GetProp("can_focus")); set => SetProp("can_focus", value); }
        public bool Draggable { get => Bool(GetProp("draggable")); set => SetProp("draggable", value); }
        public bool AcceptsDrops { get => Bool(GetProp("accepts_drops")); set => SetProp("accepts_drops", value); }
        public string DragData { get => Text(GetProp("drag_data")); set => SetProp("drag_data", value); }
        public string DragLabel { get => Text(GetProp("drag_label")); set => SetProp("drag_label", value); }
        public string DragImage { get => Text(GetProp("drag_image")); set => SetProp("drag_image", value); }
        public string DropTypes { get => Text(GetProp("drop_types")); set => SetProp("drop_types", value); }
        public bool TabStop { get => Bool(GetProp("tab_stop")); set => SetProp("tab_stop", value); }
        public string Cursor { get => Text(GetProp("cursor")); set => SetProp("cursor", value); }
        public double Opacity { get => Number(GetProp("opacity")); set => SetProp("opacity", value); }
        public bool CacheLayer { get => Bool(GetProp("cache_layer")); set => SetProp("cache_layer", value); }
        public double Rotation { get => Number(GetProp("rotation")); set => SetProp("rotation", value); }
        public double TranslateX { get => Number(GetProp("translate_x")); set => SetProp("translate_x", value); }
        public double TranslateY { get => Number(GetProp("translate_y")); set => SetProp("translate_y", value); }
        public double AspectRatio { get => Number(GetProp("aspect_ratio")); set => SetProp("aspect_ratio", value); }
        public double MaxWidth { get => Number(GetProp("max_width")); set => SetProp("max_width", value); }
        public double MinWidth { get => Number(GetProp("min_width")); set => SetProp("min_width", value); }
        public double MinHeight { get => Number(GetProp("min_height")); set => SetProp("min_height", value); }
        public double MaxHeight { get => Number(GetProp("max_height")); set => SetProp("max_height", value); }
        public string SelfAlign { get => Text(GetProp("self_align")); set => SetProp("self_align", value); }
        public bool AutoSize { get => Bool(GetProp("auto_size")); set => SetProp("auto_size", value); }
        public long ColumnSpan { get => (long)Number(GetProp("column_span")); set => SetProp("column_span", value); }
        public long GridRow { get => (long)Number(GetProp("grid_row")); set => SetProp("grid_row", value); }
        public long GridColumn { get => (long)Number(GetProp("grid_column")); set => SetProp("grid_column", value); }
        public long RowSpan { get => (long)Number(GetProp("row_span")); set => SetProp("row_span", value); }
        public double Grow { get => Number(GetProp("grow")); set => SetProp("grow", value); }
        public string Shadow { get => Text(GetProp("shadow")); set => SetProp("shadow", value); }
        public string Style { get => Text(GetProp("style")); set => SetProp("style", value); }
        public string Theme { get => Text(GetProp("theme")); set => SetProp("theme", value); }
        public string Fill { get => Text(GetProp("fill")); set => SetProp("fill", value); }
        public string BorderColor { get => Text(GetProp("border_color")); set => SetProp("border_color", value); }
        public string TextColor { get => Text(GetProp("text_color")); set => SetProp("text_color", value); }
        public string AccentColor { get => Text(GetProp("accent_color")); set => SetProp("accent_color", value); }
        public string SuccessColor { get => Text(GetProp("success_color")); set => SetProp("success_color", value); }
        public string WarningColor { get => Text(GetProp("warning_color")); set => SetProp("warning_color", value); }
        public string DangerColor { get => Text(GetProp("danger_color")); set => SetProp("danger_color", value); }
        public string FontFamily { get => Text(GetProp("font_family")); set => SetProp("font_family", value); }
        public double FontSize { get => Number(GetProp("font_size")); set => SetProp("font_size", value); }
        public string FontWeight { get => Text(GetProp("font_weight")); set => SetProp("font_weight", value); }
        public bool FontItalic { get => Bool(GetProp("font_italic")); set => SetProp("font_italic", value); }
        public bool FontUnderline { get => Bool(GetProp("font_underline")); set => SetProp("font_underline", value); }
        public bool FontStrikeout { get => Bool(GetProp("font_strikeout")); set => SetProp("font_strikeout", value); }
        public string Hint { get => Text(GetProp("hint")); set => SetProp("hint", value); }
        public long Tag { get => (long)Number(GetProp("tag")); set => SetProp("tag", value); }
        public string TagString { get => Text(GetProp("tag_string")); set => SetProp("tag_string", value); }
        public string AccessibleName { get => Text(GetProp("accessible_name")); set => SetProp("accessible_name", value); }
        public string PopupMenu { get => Text(GetProp("popup_menu")); set => SetProp("popup_menu", value); }
    }

    /// <summary>Um Panel de um form.</summary>
    public class Panel(Aylis aylis, long id, string name) : AylisComponent(aylis, id, name)
    {
        public string AllowedChars { get => Text(GetProp("allowed_chars")); set => SetProp("allowed_chars", value); }
        public string BlockedChars { get => Text(GetProp("blocked_chars")); set => SetProp("blocked_chars", value); }
        public bool OnlyAllowed { get => Bool(GetProp("only_allowed")); set => SetProp("only_allowed", value); }
        public bool CharsOverride { get => Bool(GetProp("chars_override")); set => SetProp("chars_override", value); }
        public string CharMap { get => Text(GetProp("char_map")); set => SetProp("char_map", value); }
        public double Gap { get => Number(GetProp("gap")); set => SetProp("gap", value); }
        public string Fill { get => Text(GetProp("fill")); set => SetProp("fill", value); }
        public string Fill2 { get => Text(GetProp("fill2")); set => SetProp("fill2", value); }
        public string FillMid { get => Text(GetProp("fill_mid")); set => SetProp("fill_mid", value); }
        public double FillMidAt { get => Number(GetProp("fill_mid_at")); set => SetProp("fill_mid_at", value); }
        public double GradientAngle { get => Number(GetProp("gradient_angle")); set => SetProp("gradient_angle", value); }
        public bool GradientRadial { get => Bool(GetProp("gradient_radial")); set => SetProp("gradient_radial", value); }
        public string Stroke { get => Text(GetProp("stroke")); set => SetProp("stroke", value); }
        public double StrokeWidth { get => Number(GetProp("stroke_width")); set => SetProp("stroke_width", value); }
        public string Radius { get => Text(GetProp("radius")); set => SetProp("radius", value); }
        public double Left { get => Number(GetProp("left")); set => SetProp("left", value); }
        public double Top { get => Number(GetProp("top")); set => SetProp("top", value); }
        public double Width { get => Number(GetProp("width")); set => SetProp("width", value); }
        public double Height { get => Number(GetProp("height")); set => SetProp("height", value); }
        public string Align { get => Text(GetProp("align")); set => SetProp("align", value); }
        public string Anchors { get => Text(GetProp("anchors")); set => SetProp("anchors", value); }
        public string Margins { get => Text(GetProp("margins")); set => SetProp("margins", value); }
        public string Padding { get => Text(GetProp("padding")); set => SetProp("padding", value); }
        public bool Visible { get => Bool(GetProp("visible")); set => SetProp("visible", value); }
        public bool Enabled { get => Bool(GetProp("enabled")); set => SetProp("enabled", value); }
        public bool HitTest { get => Bool(GetProp("hit_test")); set => SetProp("hit_test", value); }
        public bool ClipChildren { get => Bool(GetProp("clip_children")); set => SetProp("clip_children", value); }
        public bool InheritState { get => Bool(GetProp("inherit_state")); set => SetProp("inherit_state", value); }
        public bool Translate { get => Bool(GetProp("translate")); set => SetProp("translate", value); }
        public bool CapturesWheel { get => Bool(GetProp("captures_wheel")); set => SetProp("captures_wheel", value); }
        public bool CanFocus { get => Bool(GetProp("can_focus")); set => SetProp("can_focus", value); }
        public bool Draggable { get => Bool(GetProp("draggable")); set => SetProp("draggable", value); }
        public bool AcceptsDrops { get => Bool(GetProp("accepts_drops")); set => SetProp("accepts_drops", value); }
        public string DragData { get => Text(GetProp("drag_data")); set => SetProp("drag_data", value); }
        public string DragLabel { get => Text(GetProp("drag_label")); set => SetProp("drag_label", value); }
        public string DragImage { get => Text(GetProp("drag_image")); set => SetProp("drag_image", value); }
        public string DropTypes { get => Text(GetProp("drop_types")); set => SetProp("drop_types", value); }
        public bool TabStop { get => Bool(GetProp("tab_stop")); set => SetProp("tab_stop", value); }
        public string Cursor { get => Text(GetProp("cursor")); set => SetProp("cursor", value); }
        public double Opacity { get => Number(GetProp("opacity")); set => SetProp("opacity", value); }
        public bool CacheLayer { get => Bool(GetProp("cache_layer")); set => SetProp("cache_layer", value); }
        public double Rotation { get => Number(GetProp("rotation")); set => SetProp("rotation", value); }
        public double TranslateX { get => Number(GetProp("translate_x")); set => SetProp("translate_x", value); }
        public double TranslateY { get => Number(GetProp("translate_y")); set => SetProp("translate_y", value); }
        public double AspectRatio { get => Number(GetProp("aspect_ratio")); set => SetProp("aspect_ratio", value); }
        public double MaxWidth { get => Number(GetProp("max_width")); set => SetProp("max_width", value); }
        public double MinWidth { get => Number(GetProp("min_width")); set => SetProp("min_width", value); }
        public double MinHeight { get => Number(GetProp("min_height")); set => SetProp("min_height", value); }
        public double MaxHeight { get => Number(GetProp("max_height")); set => SetProp("max_height", value); }
        public string SelfAlign { get => Text(GetProp("self_align")); set => SetProp("self_align", value); }
        public bool AutoSize { get => Bool(GetProp("auto_size")); set => SetProp("auto_size", value); }
        public long ColumnSpan { get => (long)Number(GetProp("column_span")); set => SetProp("column_span", value); }
        public long GridRow { get => (long)Number(GetProp("grid_row")); set => SetProp("grid_row", value); }
        public long GridColumn { get => (long)Number(GetProp("grid_column")); set => SetProp("grid_column", value); }
        public long RowSpan { get => (long)Number(GetProp("row_span")); set => SetProp("row_span", value); }
        public double Grow { get => Number(GetProp("grow")); set => SetProp("grow", value); }
        public string Shadow { get => Text(GetProp("shadow")); set => SetProp("shadow", value); }
        public string Style { get => Text(GetProp("style")); set => SetProp("style", value); }
        public string Theme { get => Text(GetProp("theme")); set => SetProp("theme", value); }
        public string BorderColor { get => Text(GetProp("border_color")); set => SetProp("border_color", value); }
        public string TextColor { get => Text(GetProp("text_color")); set => SetProp("text_color", value); }
        public string AccentColor { get => Text(GetProp("accent_color")); set => SetProp("accent_color", value); }
        public string SuccessColor { get => Text(GetProp("success_color")); set => SetProp("success_color", value); }
        public string WarningColor { get => Text(GetProp("warning_color")); set => SetProp("warning_color", value); }
        public string DangerColor { get => Text(GetProp("danger_color")); set => SetProp("danger_color", value); }
        public string FontFamily { get => Text(GetProp("font_family")); set => SetProp("font_family", value); }
        public double FontSize { get => Number(GetProp("font_size")); set => SetProp("font_size", value); }
        public string FontWeight { get => Text(GetProp("font_weight")); set => SetProp("font_weight", value); }
        public bool FontItalic { get => Bool(GetProp("font_italic")); set => SetProp("font_italic", value); }
        public bool FontUnderline { get => Bool(GetProp("font_underline")); set => SetProp("font_underline", value); }
        public bool FontStrikeout { get => Bool(GetProp("font_strikeout")); set => SetProp("font_strikeout", value); }
        public string Hint { get => Text(GetProp("hint")); set => SetProp("hint", value); }
        public long Tag { get => (long)Number(GetProp("tag")); set => SetProp("tag", value); }
        public string TagString { get => Text(GetProp("tag_string")); set => SetProp("tag_string", value); }
        public string AccessibleName { get => Text(GetProp("accessible_name")); set => SetProp("accessible_name", value); }
        public string PopupMenu { get => Text(GetProp("popup_menu")); set => SetProp("popup_menu", value); }
    }

    /// <summary>Um Splitter de um form.</summary>
    public class Splitter(Aylis aylis, long id, string name) : AylisComponent(aylis, id, name)
    {
        public double MinSize { get => Number(GetProp("min_size")); set => SetProp("min_size", value); }
        public double Left { get => Number(GetProp("left")); set => SetProp("left", value); }
        public double Top { get => Number(GetProp("top")); set => SetProp("top", value); }
        public double Width { get => Number(GetProp("width")); set => SetProp("width", value); }
        public double Height { get => Number(GetProp("height")); set => SetProp("height", value); }
        public string Align { get => Text(GetProp("align")); set => SetProp("align", value); }
        public string Anchors { get => Text(GetProp("anchors")); set => SetProp("anchors", value); }
        public string Margins { get => Text(GetProp("margins")); set => SetProp("margins", value); }
        public string Padding { get => Text(GetProp("padding")); set => SetProp("padding", value); }
        public bool Visible { get => Bool(GetProp("visible")); set => SetProp("visible", value); }
        public bool Enabled { get => Bool(GetProp("enabled")); set => SetProp("enabled", value); }
        public bool HitTest { get => Bool(GetProp("hit_test")); set => SetProp("hit_test", value); }
        public bool ClipChildren { get => Bool(GetProp("clip_children")); set => SetProp("clip_children", value); }
        public bool InheritState { get => Bool(GetProp("inherit_state")); set => SetProp("inherit_state", value); }
        public bool Translate { get => Bool(GetProp("translate")); set => SetProp("translate", value); }
        public bool CapturesWheel { get => Bool(GetProp("captures_wheel")); set => SetProp("captures_wheel", value); }
        public bool CanFocus { get => Bool(GetProp("can_focus")); set => SetProp("can_focus", value); }
        public bool Draggable { get => Bool(GetProp("draggable")); set => SetProp("draggable", value); }
        public bool AcceptsDrops { get => Bool(GetProp("accepts_drops")); set => SetProp("accepts_drops", value); }
        public string DragData { get => Text(GetProp("drag_data")); set => SetProp("drag_data", value); }
        public string DragLabel { get => Text(GetProp("drag_label")); set => SetProp("drag_label", value); }
        public string DragImage { get => Text(GetProp("drag_image")); set => SetProp("drag_image", value); }
        public string DropTypes { get => Text(GetProp("drop_types")); set => SetProp("drop_types", value); }
        public bool TabStop { get => Bool(GetProp("tab_stop")); set => SetProp("tab_stop", value); }
        public string Cursor { get => Text(GetProp("cursor")); set => SetProp("cursor", value); }
        public double Opacity { get => Number(GetProp("opacity")); set => SetProp("opacity", value); }
        public bool CacheLayer { get => Bool(GetProp("cache_layer")); set => SetProp("cache_layer", value); }
        public double Rotation { get => Number(GetProp("rotation")); set => SetProp("rotation", value); }
        public double TranslateX { get => Number(GetProp("translate_x")); set => SetProp("translate_x", value); }
        public double TranslateY { get => Number(GetProp("translate_y")); set => SetProp("translate_y", value); }
        public double AspectRatio { get => Number(GetProp("aspect_ratio")); set => SetProp("aspect_ratio", value); }
        public double MaxWidth { get => Number(GetProp("max_width")); set => SetProp("max_width", value); }
        public double MinWidth { get => Number(GetProp("min_width")); set => SetProp("min_width", value); }
        public double MinHeight { get => Number(GetProp("min_height")); set => SetProp("min_height", value); }
        public double MaxHeight { get => Number(GetProp("max_height")); set => SetProp("max_height", value); }
        public string SelfAlign { get => Text(GetProp("self_align")); set => SetProp("self_align", value); }
        public bool AutoSize { get => Bool(GetProp("auto_size")); set => SetProp("auto_size", value); }
        public long ColumnSpan { get => (long)Number(GetProp("column_span")); set => SetProp("column_span", value); }
        public long GridRow { get => (long)Number(GetProp("grid_row")); set => SetProp("grid_row", value); }
        public long GridColumn { get => (long)Number(GetProp("grid_column")); set => SetProp("grid_column", value); }
        public long RowSpan { get => (long)Number(GetProp("row_span")); set => SetProp("row_span", value); }
        public double Grow { get => Number(GetProp("grow")); set => SetProp("grow", value); }
        public string Shadow { get => Text(GetProp("shadow")); set => SetProp("shadow", value); }
        public string Style { get => Text(GetProp("style")); set => SetProp("style", value); }
        public string Theme { get => Text(GetProp("theme")); set => SetProp("theme", value); }
        public string Fill { get => Text(GetProp("fill")); set => SetProp("fill", value); }
        public string BorderColor { get => Text(GetProp("border_color")); set => SetProp("border_color", value); }
        public string TextColor { get => Text(GetProp("text_color")); set => SetProp("text_color", value); }
        public string AccentColor { get => Text(GetProp("accent_color")); set => SetProp("accent_color", value); }
        public string SuccessColor { get => Text(GetProp("success_color")); set => SetProp("success_color", value); }
        public string WarningColor { get => Text(GetProp("warning_color")); set => SetProp("warning_color", value); }
        public string DangerColor { get => Text(GetProp("danger_color")); set => SetProp("danger_color", value); }
        public string FontFamily { get => Text(GetProp("font_family")); set => SetProp("font_family", value); }
        public double FontSize { get => Number(GetProp("font_size")); set => SetProp("font_size", value); }
        public string FontWeight { get => Text(GetProp("font_weight")); set => SetProp("font_weight", value); }
        public bool FontItalic { get => Bool(GetProp("font_italic")); set => SetProp("font_italic", value); }
        public bool FontUnderline { get => Bool(GetProp("font_underline")); set => SetProp("font_underline", value); }
        public bool FontStrikeout { get => Bool(GetProp("font_strikeout")); set => SetProp("font_strikeout", value); }
        public string Hint { get => Text(GetProp("hint")); set => SetProp("hint", value); }
        public long Tag { get => (long)Number(GetProp("tag")); set => SetProp("tag", value); }
        public string TagString { get => Text(GetProp("tag_string")); set => SetProp("tag_string", value); }
        public string AccessibleName { get => Text(GetProp("accessible_name")); set => SetProp("accessible_name", value); }
        public string PopupMenu { get => Text(GetProp("popup_menu")); set => SetProp("popup_menu", value); }
    }

    /// <summary>Um Svg de um form.</summary>
    public class Svg(Aylis aylis, long id, string name) : AylisComponent(aylis, id, name)
    {
        public string Source { get => Text(GetProp("source")); set => SetProp("source", value); }
        public string Color { get => Text(GetProp("color")); set => SetProp("color", value); }
        public string Fit { get => Text(GetProp("fit")); set => SetProp("fit", value); }
        public string Highlight { get => Text(GetProp("highlight")); set => SetProp("highlight", value); }
        public string HighlightColor { get => Text(GetProp("highlight_color")); set => SetProp("highlight_color", value); }
        public string ZoomTo { get => Text(GetProp("zoom_to")); set => SetProp("zoom_to", value); }
        public double Left { get => Number(GetProp("left")); set => SetProp("left", value); }
        public double Top { get => Number(GetProp("top")); set => SetProp("top", value); }
        public double Width { get => Number(GetProp("width")); set => SetProp("width", value); }
        public double Height { get => Number(GetProp("height")); set => SetProp("height", value); }
        public string Align { get => Text(GetProp("align")); set => SetProp("align", value); }
        public string Anchors { get => Text(GetProp("anchors")); set => SetProp("anchors", value); }
        public string Margins { get => Text(GetProp("margins")); set => SetProp("margins", value); }
        public string Padding { get => Text(GetProp("padding")); set => SetProp("padding", value); }
        public bool Visible { get => Bool(GetProp("visible")); set => SetProp("visible", value); }
        public bool Enabled { get => Bool(GetProp("enabled")); set => SetProp("enabled", value); }
        public bool HitTest { get => Bool(GetProp("hit_test")); set => SetProp("hit_test", value); }
        public bool ClipChildren { get => Bool(GetProp("clip_children")); set => SetProp("clip_children", value); }
        public bool InheritState { get => Bool(GetProp("inherit_state")); set => SetProp("inherit_state", value); }
        public bool Translate { get => Bool(GetProp("translate")); set => SetProp("translate", value); }
        public bool CapturesWheel { get => Bool(GetProp("captures_wheel")); set => SetProp("captures_wheel", value); }
        public bool CanFocus { get => Bool(GetProp("can_focus")); set => SetProp("can_focus", value); }
        public bool Draggable { get => Bool(GetProp("draggable")); set => SetProp("draggable", value); }
        public bool AcceptsDrops { get => Bool(GetProp("accepts_drops")); set => SetProp("accepts_drops", value); }
        public string DragData { get => Text(GetProp("drag_data")); set => SetProp("drag_data", value); }
        public string DragLabel { get => Text(GetProp("drag_label")); set => SetProp("drag_label", value); }
        public string DragImage { get => Text(GetProp("drag_image")); set => SetProp("drag_image", value); }
        public string DropTypes { get => Text(GetProp("drop_types")); set => SetProp("drop_types", value); }
        public bool TabStop { get => Bool(GetProp("tab_stop")); set => SetProp("tab_stop", value); }
        public string Cursor { get => Text(GetProp("cursor")); set => SetProp("cursor", value); }
        public double Opacity { get => Number(GetProp("opacity")); set => SetProp("opacity", value); }
        public bool CacheLayer { get => Bool(GetProp("cache_layer")); set => SetProp("cache_layer", value); }
        public double Rotation { get => Number(GetProp("rotation")); set => SetProp("rotation", value); }
        public double TranslateX { get => Number(GetProp("translate_x")); set => SetProp("translate_x", value); }
        public double TranslateY { get => Number(GetProp("translate_y")); set => SetProp("translate_y", value); }
        public double AspectRatio { get => Number(GetProp("aspect_ratio")); set => SetProp("aspect_ratio", value); }
        public double MaxWidth { get => Number(GetProp("max_width")); set => SetProp("max_width", value); }
        public double MinWidth { get => Number(GetProp("min_width")); set => SetProp("min_width", value); }
        public double MinHeight { get => Number(GetProp("min_height")); set => SetProp("min_height", value); }
        public double MaxHeight { get => Number(GetProp("max_height")); set => SetProp("max_height", value); }
        public string SelfAlign { get => Text(GetProp("self_align")); set => SetProp("self_align", value); }
        public bool AutoSize { get => Bool(GetProp("auto_size")); set => SetProp("auto_size", value); }
        public long ColumnSpan { get => (long)Number(GetProp("column_span")); set => SetProp("column_span", value); }
        public long GridRow { get => (long)Number(GetProp("grid_row")); set => SetProp("grid_row", value); }
        public long GridColumn { get => (long)Number(GetProp("grid_column")); set => SetProp("grid_column", value); }
        public long RowSpan { get => (long)Number(GetProp("row_span")); set => SetProp("row_span", value); }
        public double Grow { get => Number(GetProp("grow")); set => SetProp("grow", value); }
        public string Shadow { get => Text(GetProp("shadow")); set => SetProp("shadow", value); }
        public string Style { get => Text(GetProp("style")); set => SetProp("style", value); }
        public string Theme { get => Text(GetProp("theme")); set => SetProp("theme", value); }
        public string Fill { get => Text(GetProp("fill")); set => SetProp("fill", value); }
        public string BorderColor { get => Text(GetProp("border_color")); set => SetProp("border_color", value); }
        public string TextColor { get => Text(GetProp("text_color")); set => SetProp("text_color", value); }
        public string AccentColor { get => Text(GetProp("accent_color")); set => SetProp("accent_color", value); }
        public string SuccessColor { get => Text(GetProp("success_color")); set => SetProp("success_color", value); }
        public string WarningColor { get => Text(GetProp("warning_color")); set => SetProp("warning_color", value); }
        public string DangerColor { get => Text(GetProp("danger_color")); set => SetProp("danger_color", value); }
        public string FontFamily { get => Text(GetProp("font_family")); set => SetProp("font_family", value); }
        public double FontSize { get => Number(GetProp("font_size")); set => SetProp("font_size", value); }
        public string FontWeight { get => Text(GetProp("font_weight")); set => SetProp("font_weight", value); }
        public bool FontItalic { get => Bool(GetProp("font_italic")); set => SetProp("font_italic", value); }
        public bool FontUnderline { get => Bool(GetProp("font_underline")); set => SetProp("font_underline", value); }
        public bool FontStrikeout { get => Bool(GetProp("font_strikeout")); set => SetProp("font_strikeout", value); }
        public string Hint { get => Text(GetProp("hint")); set => SetProp("hint", value); }
        public long Tag { get => (long)Number(GetProp("tag")); set => SetProp("tag", value); }
        public string TagString { get => Text(GetProp("tag_string")); set => SetProp("tag_string", value); }
        public string AccessibleName { get => Text(GetProp("accessible_name")); set => SetProp("accessible_name", value); }
        public string PopupMenu { get => Text(GetProp("popup_menu")); set => SetProp("popup_menu", value); }
    }

    /// <summary>O form frmIBGE e os componentes dele (resolvidos ao abrir; cada janela tem a sua instância).</summary>
    public sealed class FrmIBGEComponents(Aylis aylis, long id, string name) : Form(aylis, id, name)
    {
        public ClientDataSet cdsEstados = null!;
        public Field cdsEstadosId = null!;
        public Field cdsEstadosSigla = null!;
        public Field cdsEstadosNome = null!;
        public Field cdsEstadosRegiao = null!;
        public Field cdsEstadosPopulacao = null!;
        public Field cdsEstadosArea = null!;
        public Field cdsEstadosDensidade = null!;
        public Field cdsEstadosPib = null!;
        public Field cdsEstadosPibPerCapita = null!;
        public Field cdsEstadosEstimativa = null!;
        public Field cdsEstadosEstimativaAno = null!;
        public Field cdsEstadosMunicipios = null!;
        public Field cdsEstadosBandeira = null!;
        public DataSource dsEstados = null!;
        public ClientDataSet cdsMunicipios = null!;
        public Field cdsMunicipiosId = null!;
        public Field cdsMunicipiosUfId = null!;
        public Field cdsMunicipiosUf = null!;
        public Field cdsMunicipiosNome = null!;
        public Field cdsMunicipiosMesorregiao = null!;
        public Field cdsMunicipiosMicrorregiao = null!;
        public Field cdsMunicipiosRegiaoIntermediaria = null!;
        public Field cdsMunicipiosRegiaoImediata = null!;
        public Field cdsMunicipiosPopulacao = null!;
        public Field cdsMunicipiosArea = null!;
        public Field cdsMunicipiosDensidade = null!;
        public Field cdsMunicipiosPib = null!;
        public Field cdsMunicipiosPibParticipacao = null!;
        public Field cdsMunicipiosPibPerCapita = null!;
        public Field cdsMunicipiosEstimativa = null!;
        public Field cdsMunicipiosEstimativaAno = null!;
        public Field cdsMunicipiosDistritos = null!;
        public Field cdsMunicipiosBandeira = null!;
        public DataSource dsMunicipios = null!;
        public ClientDataSet cdsDistritos = null!;
        public Field cdsDistritosId = null!;
        public Field cdsDistritosMunicipioId = null!;
        public Field cdsDistritosNome = null!;
        public DataSource dsDistritos = null!;
        public ClientDataSet cdsPopulacao = null!;
        public Field cdsPopulacaoAno = null!;
        public Field cdsPopulacaoHabitantes = null!;
        public DataSource dsPopulacao = null!;
        public ClientDataSet cdsMaiores = null!;
        public Field cdsMaioresNome = null!;
        public Field cdsMaioresHabitantes = null!;
        public DataSource dsMaiores = null!;
        public Label lblFonte = null!;
        public Layout lytListas = null!;
        public GroupBox gbEstados = null!;
        public DataGrid grdEstados = null!;
        public GroupBox gbDistritos = null!;
        public DataGrid grdDistritos = null!;
        public GroupBox gbMunicipios = null!;
        public DataGrid grdMunicipios = null!;
        public Splitter splListas = null!;
        public Layout lytPainel = null!;
        public Panel pnlEstado = null!;
        public Svg svgBrasil = null!;
        public Layout lytBandeiraEstado = null!;
        public Label lblEstado = null!;
        public Label lblRegiao = null!;
        public Image imgBandeiraEstado = null!;
        public Layout lytNumerosEstado = null!;
        public Label dbtEstadoPopulacao = null!;
        public Label lblEstadoEstimativa = null!;
        public Label dbtEstadoEstimativa = null!;
        public Label dbtEstadoArea = null!;
        public Label dbtEstadoDensidade = null!;
        public Label dbtEstadoPib = null!;
        public Label dbtEstadoPibPerCapita = null!;
        public Label dbtEstadoMunicipios = null!;
        public Panel pnlMunicipio = null!;
        public Svg svgMunicipio = null!;
        public Svg svgEstado = null!;
        public Layout lytBandeiraMunicipio = null!;
        public Label lblMunicipio = null!;
        public Label lblCodigo = null!;
        public Image imgBandeiraMunicipio = null!;
        public Label lblSemBandeira = null!;
        public Layout lytNumerosMunicipio = null!;
        public Label dbtMesorregiao = null!;
        public Label dbtMicrorregiao = null!;
        public Label dbtRegiaoImediata = null!;
        public Label dbtPopulacao = null!;
        public Label lblEstimativa = null!;
        public Label dbtEstimativa = null!;
        public Label dbtArea = null!;
        public Label dbtDensidade = null!;
        public Label dbtPib = null!;
        public Label dbtPibPerCapita = null!;
        public Label dbtPibParticipacao = null!;
        public Layout lytGraficos = null!;
        public DBChart dbcPopulacao = null!;
        public DBChart dbcMaiores = null!;
        public DBChart dbcEstados = null!;

        public override void Bind(JsonArray list)
        {
            cdsEstados = Resolve(list, "cdsEstados", "ClientDataSet", (i, d, n) => new ClientDataSet(i, d, n));
            cdsEstadosId = Resolve(list, "cdsEstadosId", "Field", (i, d, n) => new Field(i, d, n));
            cdsEstadosSigla = Resolve(list, "cdsEstadosSigla", "Field", (i, d, n) => new Field(i, d, n));
            cdsEstadosNome = Resolve(list, "cdsEstadosNome", "Field", (i, d, n) => new Field(i, d, n));
            cdsEstadosRegiao = Resolve(list, "cdsEstadosRegiao", "Field", (i, d, n) => new Field(i, d, n));
            cdsEstadosPopulacao = Resolve(list, "cdsEstadosPopulacao", "Field", (i, d, n) => new Field(i, d, n));
            cdsEstadosArea = Resolve(list, "cdsEstadosArea", "Field", (i, d, n) => new Field(i, d, n));
            cdsEstadosDensidade = Resolve(list, "cdsEstadosDensidade", "Field", (i, d, n) => new Field(i, d, n));
            cdsEstadosPib = Resolve(list, "cdsEstadosPib", "Field", (i, d, n) => new Field(i, d, n));
            cdsEstadosPibPerCapita = Resolve(list, "cdsEstadosPibPerCapita", "Field", (i, d, n) => new Field(i, d, n));
            cdsEstadosEstimativa = Resolve(list, "cdsEstadosEstimativa", "Field", (i, d, n) => new Field(i, d, n));
            cdsEstadosEstimativaAno = Resolve(list, "cdsEstadosEstimativaAno", "Field", (i, d, n) => new Field(i, d, n));
            cdsEstadosMunicipios = Resolve(list, "cdsEstadosMunicipios", "Field", (i, d, n) => new Field(i, d, n));
            cdsEstadosBandeira = Resolve(list, "cdsEstadosBandeira", "Field", (i, d, n) => new Field(i, d, n));
            dsEstados = Resolve(list, "dsEstados", "DataSource", (i, d, n) => new DataSource(i, d, n));
            cdsMunicipios = Resolve(list, "cdsMunicipios", "ClientDataSet", (i, d, n) => new ClientDataSet(i, d, n));
            cdsMunicipiosId = Resolve(list, "cdsMunicipiosId", "Field", (i, d, n) => new Field(i, d, n));
            cdsMunicipiosUfId = Resolve(list, "cdsMunicipiosUfId", "Field", (i, d, n) => new Field(i, d, n));
            cdsMunicipiosUf = Resolve(list, "cdsMunicipiosUf", "Field", (i, d, n) => new Field(i, d, n));
            cdsMunicipiosNome = Resolve(list, "cdsMunicipiosNome", "Field", (i, d, n) => new Field(i, d, n));
            cdsMunicipiosMesorregiao = Resolve(list, "cdsMunicipiosMesorregiao", "Field", (i, d, n) => new Field(i, d, n));
            cdsMunicipiosMicrorregiao = Resolve(list, "cdsMunicipiosMicrorregiao", "Field", (i, d, n) => new Field(i, d, n));
            cdsMunicipiosRegiaoIntermediaria = Resolve(list, "cdsMunicipiosRegiaoIntermediaria", "Field", (i, d, n) => new Field(i, d, n));
            cdsMunicipiosRegiaoImediata = Resolve(list, "cdsMunicipiosRegiaoImediata", "Field", (i, d, n) => new Field(i, d, n));
            cdsMunicipiosPopulacao = Resolve(list, "cdsMunicipiosPopulacao", "Field", (i, d, n) => new Field(i, d, n));
            cdsMunicipiosArea = Resolve(list, "cdsMunicipiosArea", "Field", (i, d, n) => new Field(i, d, n));
            cdsMunicipiosDensidade = Resolve(list, "cdsMunicipiosDensidade", "Field", (i, d, n) => new Field(i, d, n));
            cdsMunicipiosPib = Resolve(list, "cdsMunicipiosPib", "Field", (i, d, n) => new Field(i, d, n));
            cdsMunicipiosPibParticipacao = Resolve(list, "cdsMunicipiosPibParticipacao", "Field", (i, d, n) => new Field(i, d, n));
            cdsMunicipiosPibPerCapita = Resolve(list, "cdsMunicipiosPibPerCapita", "Field", (i, d, n) => new Field(i, d, n));
            cdsMunicipiosEstimativa = Resolve(list, "cdsMunicipiosEstimativa", "Field", (i, d, n) => new Field(i, d, n));
            cdsMunicipiosEstimativaAno = Resolve(list, "cdsMunicipiosEstimativaAno", "Field", (i, d, n) => new Field(i, d, n));
            cdsMunicipiosDistritos = Resolve(list, "cdsMunicipiosDistritos", "Field", (i, d, n) => new Field(i, d, n));
            cdsMunicipiosBandeira = Resolve(list, "cdsMunicipiosBandeira", "Field", (i, d, n) => new Field(i, d, n));
            dsMunicipios = Resolve(list, "dsMunicipios", "DataSource", (i, d, n) => new DataSource(i, d, n));
            cdsDistritos = Resolve(list, "cdsDistritos", "ClientDataSet", (i, d, n) => new ClientDataSet(i, d, n));
            cdsDistritosId = Resolve(list, "cdsDistritosId", "Field", (i, d, n) => new Field(i, d, n));
            cdsDistritosMunicipioId = Resolve(list, "cdsDistritosMunicipioId", "Field", (i, d, n) => new Field(i, d, n));
            cdsDistritosNome = Resolve(list, "cdsDistritosNome", "Field", (i, d, n) => new Field(i, d, n));
            dsDistritos = Resolve(list, "dsDistritos", "DataSource", (i, d, n) => new DataSource(i, d, n));
            cdsPopulacao = Resolve(list, "cdsPopulacao", "ClientDataSet", (i, d, n) => new ClientDataSet(i, d, n));
            cdsPopulacaoAno = Resolve(list, "cdsPopulacaoAno", "Field", (i, d, n) => new Field(i, d, n));
            cdsPopulacaoHabitantes = Resolve(list, "cdsPopulacaoHabitantes", "Field", (i, d, n) => new Field(i, d, n));
            dsPopulacao = Resolve(list, "dsPopulacao", "DataSource", (i, d, n) => new DataSource(i, d, n));
            cdsMaiores = Resolve(list, "cdsMaiores", "ClientDataSet", (i, d, n) => new ClientDataSet(i, d, n));
            cdsMaioresNome = Resolve(list, "cdsMaioresNome", "Field", (i, d, n) => new Field(i, d, n));
            cdsMaioresHabitantes = Resolve(list, "cdsMaioresHabitantes", "Field", (i, d, n) => new Field(i, d, n));
            dsMaiores = Resolve(list, "dsMaiores", "DataSource", (i, d, n) => new DataSource(i, d, n));
            lblFonte = Resolve(list, "lblFonte", "Label", (i, d, n) => new Label(i, d, n));
            lytListas = Resolve(list, "lytListas", "Layout", (i, d, n) => new Layout(i, d, n));
            gbEstados = Resolve(list, "gbEstados", "GroupBox", (i, d, n) => new GroupBox(i, d, n));
            grdEstados = Resolve(list, "grdEstados", "DataGrid", (i, d, n) => new DataGrid(i, d, n));
            gbDistritos = Resolve(list, "gbDistritos", "GroupBox", (i, d, n) => new GroupBox(i, d, n));
            grdDistritos = Resolve(list, "grdDistritos", "DataGrid", (i, d, n) => new DataGrid(i, d, n));
            gbMunicipios = Resolve(list, "gbMunicipios", "GroupBox", (i, d, n) => new GroupBox(i, d, n));
            grdMunicipios = Resolve(list, "grdMunicipios", "DataGrid", (i, d, n) => new DataGrid(i, d, n));
            splListas = Resolve(list, "splListas", "Splitter", (i, d, n) => new Splitter(i, d, n));
            lytPainel = Resolve(list, "lytPainel", "Layout", (i, d, n) => new Layout(i, d, n));
            pnlEstado = Resolve(list, "pnlEstado", "Panel", (i, d, n) => new Panel(i, d, n));
            svgBrasil = Resolve(list, "svgBrasil", "Svg", (i, d, n) => new Svg(i, d, n));
            lytBandeiraEstado = Resolve(list, "lytBandeiraEstado", "Layout", (i, d, n) => new Layout(i, d, n));
            lblEstado = Resolve(list, "lblEstado", "Label", (i, d, n) => new Label(i, d, n));
            lblRegiao = Resolve(list, "lblRegiao", "Label", (i, d, n) => new Label(i, d, n));
            imgBandeiraEstado = Resolve(list, "imgBandeiraEstado", "Image", (i, d, n) => new Image(i, d, n));
            lytNumerosEstado = Resolve(list, "lytNumerosEstado", "Layout", (i, d, n) => new Layout(i, d, n));
            dbtEstadoPopulacao = Resolve(list, "dbtEstadoPopulacao", "Label", (i, d, n) => new Label(i, d, n));
            lblEstadoEstimativa = Resolve(list, "lblEstadoEstimativa", "Label", (i, d, n) => new Label(i, d, n));
            dbtEstadoEstimativa = Resolve(list, "dbtEstadoEstimativa", "Label", (i, d, n) => new Label(i, d, n));
            dbtEstadoArea = Resolve(list, "dbtEstadoArea", "Label", (i, d, n) => new Label(i, d, n));
            dbtEstadoDensidade = Resolve(list, "dbtEstadoDensidade", "Label", (i, d, n) => new Label(i, d, n));
            dbtEstadoPib = Resolve(list, "dbtEstadoPib", "Label", (i, d, n) => new Label(i, d, n));
            dbtEstadoPibPerCapita = Resolve(list, "dbtEstadoPibPerCapita", "Label", (i, d, n) => new Label(i, d, n));
            dbtEstadoMunicipios = Resolve(list, "dbtEstadoMunicipios", "Label", (i, d, n) => new Label(i, d, n));
            pnlMunicipio = Resolve(list, "pnlMunicipio", "Panel", (i, d, n) => new Panel(i, d, n));
            svgMunicipio = Resolve(list, "svgMunicipio", "Svg", (i, d, n) => new Svg(i, d, n));
            svgEstado = Resolve(list, "svgEstado", "Svg", (i, d, n) => new Svg(i, d, n));
            lytBandeiraMunicipio = Resolve(list, "lytBandeiraMunicipio", "Layout", (i, d, n) => new Layout(i, d, n));
            lblMunicipio = Resolve(list, "lblMunicipio", "Label", (i, d, n) => new Label(i, d, n));
            lblCodigo = Resolve(list, "lblCodigo", "Label", (i, d, n) => new Label(i, d, n));
            imgBandeiraMunicipio = Resolve(list, "imgBandeiraMunicipio", "Image", (i, d, n) => new Image(i, d, n));
            lblSemBandeira = Resolve(list, "lblSemBandeira", "Label", (i, d, n) => new Label(i, d, n));
            lytNumerosMunicipio = Resolve(list, "lytNumerosMunicipio", "Layout", (i, d, n) => new Layout(i, d, n));
            dbtMesorregiao = Resolve(list, "dbtMesorregiao", "Label", (i, d, n) => new Label(i, d, n));
            dbtMicrorregiao = Resolve(list, "dbtMicrorregiao", "Label", (i, d, n) => new Label(i, d, n));
            dbtRegiaoImediata = Resolve(list, "dbtRegiaoImediata", "Label", (i, d, n) => new Label(i, d, n));
            dbtPopulacao = Resolve(list, "dbtPopulacao", "Label", (i, d, n) => new Label(i, d, n));
            lblEstimativa = Resolve(list, "lblEstimativa", "Label", (i, d, n) => new Label(i, d, n));
            dbtEstimativa = Resolve(list, "dbtEstimativa", "Label", (i, d, n) => new Label(i, d, n));
            dbtArea = Resolve(list, "dbtArea", "Label", (i, d, n) => new Label(i, d, n));
            dbtDensidade = Resolve(list, "dbtDensidade", "Label", (i, d, n) => new Label(i, d, n));
            dbtPib = Resolve(list, "dbtPib", "Label", (i, d, n) => new Label(i, d, n));
            dbtPibPerCapita = Resolve(list, "dbtPibPerCapita", "Label", (i, d, n) => new Label(i, d, n));
            dbtPibParticipacao = Resolve(list, "dbtPibParticipacao", "Label", (i, d, n) => new Label(i, d, n));
            lytGraficos = Resolve(list, "lytGraficos", "Layout", (i, d, n) => new Layout(i, d, n));
            dbcPopulacao = Resolve(list, "dbcPopulacao", "DBChart", (i, d, n) => new DBChart(i, d, n));
            dbcMaiores = Resolve(list, "dbcMaiores", "DBChart", (i, d, n) => new DBChart(i, d, n));
            dbcEstados = Resolve(list, "dbcEstados", "DBChart", (i, d, n) => new DBChart(i, d, n));
        }

        /// <summary>O ClientDataSet deste form com o nome (como o FindComponent).</summary>
        public ClientDataSet? FindClientDataSet(string name) => name switch
        {
            "cdsEstados" => cdsEstados,
            "cdsMunicipios" => cdsMunicipios,
            "cdsDistritos" => cdsDistritos,
            "cdsPopulacao" => cdsPopulacao,
            "cdsMaiores" => cdsMaiores,
            _ => null,
        };

        /// <summary>O ClientDataSet que disparou o evento.</summary>
        public ClientDataSet? SenderAsClientDataSet(AylisClient.AylisEvent ev) => FindClientDataSet(ev.SenderName);

        /// <summary>O DBChart deste form com o nome (como o FindComponent).</summary>
        public DBChart? FindDBChart(string name) => name switch
        {
            "dbcPopulacao" => dbcPopulacao,
            "dbcMaiores" => dbcMaiores,
            "dbcEstados" => dbcEstados,
            _ => null,
        };

        /// <summary>O DBChart que disparou o evento.</summary>
        public DBChart? SenderAsDBChart(AylisClient.AylisEvent ev) => FindDBChart(ev.SenderName);

        /// <summary>O DataGrid deste form com o nome (como o FindComponent).</summary>
        public DataGrid? FindDataGrid(string name) => name switch
        {
            "grdEstados" => grdEstados,
            "grdDistritos" => grdDistritos,
            "grdMunicipios" => grdMunicipios,
            _ => null,
        };

        /// <summary>O DataGrid que disparou o evento.</summary>
        public DataGrid? SenderAsDataGrid(AylisClient.AylisEvent ev) => FindDataGrid(ev.SenderName);

        /// <summary>O DataSource deste form com o nome (como o FindComponent).</summary>
        public DataSource? FindDataSource(string name) => name switch
        {
            "dsEstados" => dsEstados,
            "dsMunicipios" => dsMunicipios,
            "dsDistritos" => dsDistritos,
            "dsPopulacao" => dsPopulacao,
            "dsMaiores" => dsMaiores,
            _ => null,
        };

        /// <summary>O DataSource que disparou o evento.</summary>
        public DataSource? SenderAsDataSource(AylisClient.AylisEvent ev) => FindDataSource(ev.SenderName);

        /// <summary>O Field deste form com o nome (como o FindComponent).</summary>
        public Field? FindField(string name) => name switch
        {
            "cdsEstadosId" => cdsEstadosId,
            "cdsEstadosSigla" => cdsEstadosSigla,
            "cdsEstadosNome" => cdsEstadosNome,
            "cdsEstadosRegiao" => cdsEstadosRegiao,
            "cdsEstadosPopulacao" => cdsEstadosPopulacao,
            "cdsEstadosArea" => cdsEstadosArea,
            "cdsEstadosDensidade" => cdsEstadosDensidade,
            "cdsEstadosPib" => cdsEstadosPib,
            "cdsEstadosPibPerCapita" => cdsEstadosPibPerCapita,
            "cdsEstadosEstimativa" => cdsEstadosEstimativa,
            "cdsEstadosEstimativaAno" => cdsEstadosEstimativaAno,
            "cdsEstadosMunicipios" => cdsEstadosMunicipios,
            "cdsEstadosBandeira" => cdsEstadosBandeira,
            "cdsMunicipiosId" => cdsMunicipiosId,
            "cdsMunicipiosUfId" => cdsMunicipiosUfId,
            "cdsMunicipiosUf" => cdsMunicipiosUf,
            "cdsMunicipiosNome" => cdsMunicipiosNome,
            "cdsMunicipiosMesorregiao" => cdsMunicipiosMesorregiao,
            "cdsMunicipiosMicrorregiao" => cdsMunicipiosMicrorregiao,
            "cdsMunicipiosRegiaoIntermediaria" => cdsMunicipiosRegiaoIntermediaria,
            "cdsMunicipiosRegiaoImediata" => cdsMunicipiosRegiaoImediata,
            "cdsMunicipiosPopulacao" => cdsMunicipiosPopulacao,
            "cdsMunicipiosArea" => cdsMunicipiosArea,
            "cdsMunicipiosDensidade" => cdsMunicipiosDensidade,
            "cdsMunicipiosPib" => cdsMunicipiosPib,
            "cdsMunicipiosPibParticipacao" => cdsMunicipiosPibParticipacao,
            "cdsMunicipiosPibPerCapita" => cdsMunicipiosPibPerCapita,
            "cdsMunicipiosEstimativa" => cdsMunicipiosEstimativa,
            "cdsMunicipiosEstimativaAno" => cdsMunicipiosEstimativaAno,
            "cdsMunicipiosDistritos" => cdsMunicipiosDistritos,
            "cdsMunicipiosBandeira" => cdsMunicipiosBandeira,
            "cdsDistritosId" => cdsDistritosId,
            "cdsDistritosMunicipioId" => cdsDistritosMunicipioId,
            "cdsDistritosNome" => cdsDistritosNome,
            "cdsPopulacaoAno" => cdsPopulacaoAno,
            "cdsPopulacaoHabitantes" => cdsPopulacaoHabitantes,
            "cdsMaioresNome" => cdsMaioresNome,
            "cdsMaioresHabitantes" => cdsMaioresHabitantes,
            _ => null,
        };

        /// <summary>O Field que disparou o evento.</summary>
        public Field? SenderAsField(AylisClient.AylisEvent ev) => FindField(ev.SenderName);

        /// <summary>O GroupBox deste form com o nome (como o FindComponent).</summary>
        public GroupBox? FindGroupBox(string name) => name switch
        {
            "gbEstados" => gbEstados,
            "gbDistritos" => gbDistritos,
            "gbMunicipios" => gbMunicipios,
            _ => null,
        };

        /// <summary>O GroupBox que disparou o evento.</summary>
        public GroupBox? SenderAsGroupBox(AylisClient.AylisEvent ev) => FindGroupBox(ev.SenderName);

        /// <summary>O Image deste form com o nome (como o FindComponent).</summary>
        public Image? FindImage(string name) => name switch
        {
            "imgBandeiraEstado" => imgBandeiraEstado,
            "imgBandeiraMunicipio" => imgBandeiraMunicipio,
            _ => null,
        };

        /// <summary>O Image que disparou o evento.</summary>
        public Image? SenderAsImage(AylisClient.AylisEvent ev) => FindImage(ev.SenderName);

        /// <summary>O Label deste form com o nome (como o FindComponent).</summary>
        public Label? FindLabel(string name) => name switch
        {
            "lblFonte" => lblFonte,
            "lblEstado" => lblEstado,
            "lblRegiao" => lblRegiao,
            "dbtEstadoPopulacao" => dbtEstadoPopulacao,
            "lblEstadoEstimativa" => lblEstadoEstimativa,
            "dbtEstadoEstimativa" => dbtEstadoEstimativa,
            "dbtEstadoArea" => dbtEstadoArea,
            "dbtEstadoDensidade" => dbtEstadoDensidade,
            "dbtEstadoPib" => dbtEstadoPib,
            "dbtEstadoPibPerCapita" => dbtEstadoPibPerCapita,
            "dbtEstadoMunicipios" => dbtEstadoMunicipios,
            "lblMunicipio" => lblMunicipio,
            "lblCodigo" => lblCodigo,
            "lblSemBandeira" => lblSemBandeira,
            "dbtMesorregiao" => dbtMesorregiao,
            "dbtMicrorregiao" => dbtMicrorregiao,
            "dbtRegiaoImediata" => dbtRegiaoImediata,
            "dbtPopulacao" => dbtPopulacao,
            "lblEstimativa" => lblEstimativa,
            "dbtEstimativa" => dbtEstimativa,
            "dbtArea" => dbtArea,
            "dbtDensidade" => dbtDensidade,
            "dbtPib" => dbtPib,
            "dbtPibPerCapita" => dbtPibPerCapita,
            "dbtPibParticipacao" => dbtPibParticipacao,
            _ => null,
        };

        /// <summary>O Label que disparou o evento.</summary>
        public Label? SenderAsLabel(AylisClient.AylisEvent ev) => FindLabel(ev.SenderName);

        /// <summary>O Layout deste form com o nome (como o FindComponent).</summary>
        public Layout? FindLayout(string name) => name switch
        {
            "lytListas" => lytListas,
            "lytPainel" => lytPainel,
            "lytBandeiraEstado" => lytBandeiraEstado,
            "lytNumerosEstado" => lytNumerosEstado,
            "lytBandeiraMunicipio" => lytBandeiraMunicipio,
            "lytNumerosMunicipio" => lytNumerosMunicipio,
            "lytGraficos" => lytGraficos,
            _ => null,
        };

        /// <summary>O Layout que disparou o evento.</summary>
        public Layout? SenderAsLayout(AylisClient.AylisEvent ev) => FindLayout(ev.SenderName);

        /// <summary>O Panel deste form com o nome (como o FindComponent).</summary>
        public Panel? FindPanel(string name) => name switch
        {
            "pnlEstado" => pnlEstado,
            "pnlMunicipio" => pnlMunicipio,
            _ => null,
        };

        /// <summary>O Panel que disparou o evento.</summary>
        public Panel? SenderAsPanel(AylisClient.AylisEvent ev) => FindPanel(ev.SenderName);

        /// <summary>O Splitter deste form com o nome (como o FindComponent).</summary>
        public Splitter? FindSplitter(string name) => name switch
        {
            "splListas" => splListas,
            _ => null,
        };

        /// <summary>O Splitter que disparou o evento.</summary>
        public Splitter? SenderAsSplitter(AylisClient.AylisEvent ev) => FindSplitter(ev.SenderName);

        /// <summary>O Svg deste form com o nome (como o FindComponent).</summary>
        public Svg? FindSvg(string name) => name switch
        {
            "svgBrasil" => svgBrasil,
            "svgMunicipio" => svgMunicipio,
            "svgEstado" => svgEstado,
            _ => null,
        };

        /// <summary>O Svg que disparou o evento.</summary>
        public Svg? SenderAsSvg(AylisClient.AylisEvent ev) => FindSvg(ev.SenderName);
    }
}
