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

        public static AylisUi.FrmCalculadoraComponents frmCalculadora = null!;

        /// <summary>Registra as classes dos forms (o registro dos forms chama).</summary>
        public static void RegisterClasses()
        {
            Aylis.RegisterFormClass("frmCalculadora", (aylis, id, name) => new AylisUi.FrmCalculadoraComponents(aylis, id, name), form => frmCalculadora = (AylisUi.FrmCalculadoraComponents)form);
        }
    }
}

namespace App.Forms.AylisUi
{
    using static App.Forms.Components;

    /// <summary>Um Button de um form.</summary>
    public class Button(Aylis aylis, long id, string name) : AylisComponent(aylis, id, name)
    {
        public string Text { get => Text(GetProp("text")); set => SetProp("text", value); }
        public string Icon { get => Text(GetProp("icon")); set => SetProp("icon", value); }
        public string CheckedIcon { get => Text(GetProp("checked_icon")); set => SetProp("checked_icon", value); }
        public string IconPosition { get => Text(GetProp("icon_position")); set => SetProp("icon_position", value); }
        public double IconSize { get => Number(GetProp("icon_size")); set => SetProp("icon_size", value); }
        public bool Checkable { get => Bool(GetProp("checkable")); set => SetProp("checkable", value); }
        public bool Checked { get => Bool(GetProp("checked")); set => SetProp("checked", value); }
        public string ModalResult { get => Text(GetProp("modal_result")); set => SetProp("modal_result", value); }
        public bool RepeatEnter { get => Bool(GetProp("repeat_enter")); set => SetProp("repeat_enter", value); }
        public bool DefaultButton { get => Bool(GetProp("default_button")); set => SetProp("default_button", value); }
        public bool CancelButton { get => Bool(GetProp("cancel_button")); set => SetProp("cancel_button", value); }
        public string HAlign { get => Text(GetProp("h_align")); set => SetProp("h_align", value); }
        public long GroupIndex { get => (long)Number(GetProp("group_index")); set => SetProp("group_index", value); }
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

    /// <summary>Um GridLayout de um form.</summary>
    public class GridLayout(Aylis aylis, long id, string name) : AylisComponent(aylis, id, name)
    {
        public long Columns { get => (long)Number(GetProp("columns")); set => SetProp("columns", value); }
        public double MinColumnWidth { get => Number(GetProp("min_column_width")); set => SetProp("min_column_width", value); }
        public double Gap { get => Number(GetProp("gap")); set => SetProp("gap", value); }
        public double RowHeight { get => Number(GetProp("row_height")); set => SetProp("row_height", value); }
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

    /// <summary>Um ListView de um form.</summary>
    public class ListView(Aylis aylis, long id, string name) : AylisComponent(aylis, id, name)
    {
        public string Items { get => Text(GetProp("items")); set => SetProp("items", value); }
        public long ItemIndex { get => (long)Number(GetProp("item_index")); set => SetProp("item_index", value); }
        public double ItemHeight { get => Number(GetProp("item_height")); set => SetProp("item_height", value); }
        public bool Accessory { get => Bool(GetProp("accessory")); set => SetProp("accessory", value); }
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

    /// <summary>Um TabControl de um form.</summary>
    public class TabControl(Aylis aylis, long id, string name) : AylisComponent(aylis, id, name)
    {
        public long TabIndex { get => (long)Number(GetProp("tab_index")); set => SetProp("tab_index", value); }
        public bool ShowTabs { get => Bool(GetProp("show_tabs")); set => SetProp("show_tabs", value); }
        public string Transition { get => Text(GetProp("transition")); set => SetProp("transition", value); }
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

    /// <summary>Um TabItem de um form.</summary>
    public class TabItem(Aylis aylis, long id, string name) : AylisComponent(aylis, id, name)
    {
        public string Caption { get => Text(GetProp("caption")); set => SetProp("caption", value); }
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

    /// <summary>O form frmCalculadora e os componentes dele (resolvidos ao abrir; cada janela tem a sua instância).</summary>
    public sealed class FrmCalculadoraComponents(Aylis aylis, long id, string name) : Form(aylis, id, name)
    {
        public Panel pnlLateral = null!;
        public TabControl tabLateral = null!;
        public TabItem tabHistorico = null!;
        public Button btnLimparHistorico = null!;
        public ListView lvHistorico = null!;
        public TabItem tabMemoria = null!;
        public ListView lvMemoria = null!;
        public Layout lytCorpo = null!;
        public Layout lytTopo = null!;
        public Button btnHistorico = null!;
        public Label lblTitulo = null!;
        public Label lblExpressao = null!;
        public Label lblVisor = null!;
        public GridLayout grdMemoria = null!;
        public Button btnMC = null!;
        public Button btnMR = null!;
        public Button btnMMais = null!;
        public Button btnMMenos = null!;
        public Button btnMS = null!;
        public GridLayout grdTeclado = null!;
        public Button btnPorcento = null!;
        public Button btnCE = null!;
        public Button btnC = null!;
        public Button btnApagar = null!;
        public Button btnInverso = null!;
        public Button btnQuadrado = null!;
        public Button btnRaiz = null!;
        public Button btnDividir = null!;
        public Button btn7 = null!;
        public Button btn8 = null!;
        public Button btn9 = null!;
        public Button btnMultiplicar = null!;
        public Button btn4 = null!;
        public Button btn5 = null!;
        public Button btn6 = null!;
        public Button btnSubtrair = null!;
        public Button btn1 = null!;
        public Button btn2 = null!;
        public Button btn3 = null!;
        public Button btnSomar = null!;
        public Button btnSinal = null!;
        public Button btn0 = null!;
        public Button btnVirgula = null!;
        public Button btnIgual = null!;

        public override void Bind(JsonArray list)
        {
            pnlLateral = Resolve(list, "pnlLateral", "Panel", (i, d, n) => new Panel(i, d, n));
            tabLateral = Resolve(list, "tabLateral", "TabControl", (i, d, n) => new TabControl(i, d, n));
            tabHistorico = Resolve(list, "tabHistorico", "TabItem", (i, d, n) => new TabItem(i, d, n));
            btnLimparHistorico = Resolve(list, "btnLimparHistorico", "Button", (i, d, n) => new Button(i, d, n));
            lvHistorico = Resolve(list, "lvHistorico", "ListView", (i, d, n) => new ListView(i, d, n));
            tabMemoria = Resolve(list, "tabMemoria", "TabItem", (i, d, n) => new TabItem(i, d, n));
            lvMemoria = Resolve(list, "lvMemoria", "ListView", (i, d, n) => new ListView(i, d, n));
            lytCorpo = Resolve(list, "lytCorpo", "Layout", (i, d, n) => new Layout(i, d, n));
            lytTopo = Resolve(list, "lytTopo", "Layout", (i, d, n) => new Layout(i, d, n));
            btnHistorico = Resolve(list, "btnHistorico", "Button", (i, d, n) => new Button(i, d, n));
            lblTitulo = Resolve(list, "lblTitulo", "Label", (i, d, n) => new Label(i, d, n));
            lblExpressao = Resolve(list, "lblExpressao", "Label", (i, d, n) => new Label(i, d, n));
            lblVisor = Resolve(list, "lblVisor", "Label", (i, d, n) => new Label(i, d, n));
            grdMemoria = Resolve(list, "grdMemoria", "GridLayout", (i, d, n) => new GridLayout(i, d, n));
            btnMC = Resolve(list, "btnMC", "Button", (i, d, n) => new Button(i, d, n));
            btnMR = Resolve(list, "btnMR", "Button", (i, d, n) => new Button(i, d, n));
            btnMMais = Resolve(list, "btnMMais", "Button", (i, d, n) => new Button(i, d, n));
            btnMMenos = Resolve(list, "btnMMenos", "Button", (i, d, n) => new Button(i, d, n));
            btnMS = Resolve(list, "btnMS", "Button", (i, d, n) => new Button(i, d, n));
            grdTeclado = Resolve(list, "grdTeclado", "GridLayout", (i, d, n) => new GridLayout(i, d, n));
            btnPorcento = Resolve(list, "btnPorcento", "Button", (i, d, n) => new Button(i, d, n));
            btnCE = Resolve(list, "btnCE", "Button", (i, d, n) => new Button(i, d, n));
            btnC = Resolve(list, "btnC", "Button", (i, d, n) => new Button(i, d, n));
            btnApagar = Resolve(list, "btnApagar", "Button", (i, d, n) => new Button(i, d, n));
            btnInverso = Resolve(list, "btnInverso", "Button", (i, d, n) => new Button(i, d, n));
            btnQuadrado = Resolve(list, "btnQuadrado", "Button", (i, d, n) => new Button(i, d, n));
            btnRaiz = Resolve(list, "btnRaiz", "Button", (i, d, n) => new Button(i, d, n));
            btnDividir = Resolve(list, "btnDividir", "Button", (i, d, n) => new Button(i, d, n));
            btn7 = Resolve(list, "btn7", "Button", (i, d, n) => new Button(i, d, n));
            btn8 = Resolve(list, "btn8", "Button", (i, d, n) => new Button(i, d, n));
            btn9 = Resolve(list, "btn9", "Button", (i, d, n) => new Button(i, d, n));
            btnMultiplicar = Resolve(list, "btnMultiplicar", "Button", (i, d, n) => new Button(i, d, n));
            btn4 = Resolve(list, "btn4", "Button", (i, d, n) => new Button(i, d, n));
            btn5 = Resolve(list, "btn5", "Button", (i, d, n) => new Button(i, d, n));
            btn6 = Resolve(list, "btn6", "Button", (i, d, n) => new Button(i, d, n));
            btnSubtrair = Resolve(list, "btnSubtrair", "Button", (i, d, n) => new Button(i, d, n));
            btn1 = Resolve(list, "btn1", "Button", (i, d, n) => new Button(i, d, n));
            btn2 = Resolve(list, "btn2", "Button", (i, d, n) => new Button(i, d, n));
            btn3 = Resolve(list, "btn3", "Button", (i, d, n) => new Button(i, d, n));
            btnSomar = Resolve(list, "btnSomar", "Button", (i, d, n) => new Button(i, d, n));
            btnSinal = Resolve(list, "btnSinal", "Button", (i, d, n) => new Button(i, d, n));
            btn0 = Resolve(list, "btn0", "Button", (i, d, n) => new Button(i, d, n));
            btnVirgula = Resolve(list, "btnVirgula", "Button", (i, d, n) => new Button(i, d, n));
            btnIgual = Resolve(list, "btnIgual", "Button", (i, d, n) => new Button(i, d, n));
        }

        /// <summary>O Button deste form com o nome (como o FindComponent).</summary>
        public Button? FindButton(string name) => name switch
        {
            "btnLimparHistorico" => btnLimparHistorico,
            "btnHistorico" => btnHistorico,
            "btnMC" => btnMC,
            "btnMR" => btnMR,
            "btnMMais" => btnMMais,
            "btnMMenos" => btnMMenos,
            "btnMS" => btnMS,
            "btnPorcento" => btnPorcento,
            "btnCE" => btnCE,
            "btnC" => btnC,
            "btnApagar" => btnApagar,
            "btnInverso" => btnInverso,
            "btnQuadrado" => btnQuadrado,
            "btnRaiz" => btnRaiz,
            "btnDividir" => btnDividir,
            "btn7" => btn7,
            "btn8" => btn8,
            "btn9" => btn9,
            "btnMultiplicar" => btnMultiplicar,
            "btn4" => btn4,
            "btn5" => btn5,
            "btn6" => btn6,
            "btnSubtrair" => btnSubtrair,
            "btn1" => btn1,
            "btn2" => btn2,
            "btn3" => btn3,
            "btnSomar" => btnSomar,
            "btnSinal" => btnSinal,
            "btn0" => btn0,
            "btnVirgula" => btnVirgula,
            "btnIgual" => btnIgual,
            _ => null,
        };

        /// <summary>O Button que disparou o evento.</summary>
        public Button? SenderAsButton(AylisClient.AylisEvent ev) => FindButton(ev.SenderName);

        /// <summary>O GridLayout deste form com o nome (como o FindComponent).</summary>
        public GridLayout? FindGridLayout(string name) => name switch
        {
            "grdMemoria" => grdMemoria,
            "grdTeclado" => grdTeclado,
            _ => null,
        };

        /// <summary>O GridLayout que disparou o evento.</summary>
        public GridLayout? SenderAsGridLayout(AylisClient.AylisEvent ev) => FindGridLayout(ev.SenderName);

        /// <summary>O Label deste form com o nome (como o FindComponent).</summary>
        public Label? FindLabel(string name) => name switch
        {
            "lblTitulo" => lblTitulo,
            "lblExpressao" => lblExpressao,
            "lblVisor" => lblVisor,
            _ => null,
        };

        /// <summary>O Label que disparou o evento.</summary>
        public Label? SenderAsLabel(AylisClient.AylisEvent ev) => FindLabel(ev.SenderName);

        /// <summary>O Layout deste form com o nome (como o FindComponent).</summary>
        public Layout? FindLayout(string name) => name switch
        {
            "lytCorpo" => lytCorpo,
            "lytTopo" => lytTopo,
            _ => null,
        };

        /// <summary>O Layout que disparou o evento.</summary>
        public Layout? SenderAsLayout(AylisClient.AylisEvent ev) => FindLayout(ev.SenderName);

        /// <summary>O ListView deste form com o nome (como o FindComponent).</summary>
        public ListView? FindListView(string name) => name switch
        {
            "lvHistorico" => lvHistorico,
            "lvMemoria" => lvMemoria,
            _ => null,
        };

        /// <summary>O ListView que disparou o evento.</summary>
        public ListView? SenderAsListView(AylisClient.AylisEvent ev) => FindListView(ev.SenderName);

        /// <summary>O Panel deste form com o nome (como o FindComponent).</summary>
        public Panel? FindPanel(string name) => name switch
        {
            "pnlLateral" => pnlLateral,
            _ => null,
        };

        /// <summary>O Panel que disparou o evento.</summary>
        public Panel? SenderAsPanel(AylisClient.AylisEvent ev) => FindPanel(ev.SenderName);

        /// <summary>O TabControl deste form com o nome (como o FindComponent).</summary>
        public TabControl? FindTabControl(string name) => name switch
        {
            "tabLateral" => tabLateral,
            _ => null,
        };

        /// <summary>O TabControl que disparou o evento.</summary>
        public TabControl? SenderAsTabControl(AylisClient.AylisEvent ev) => FindTabControl(ev.SenderName);

        /// <summary>O TabItem deste form com o nome (como o FindComponent).</summary>
        public TabItem? FindTabItem(string name) => name switch
        {
            "tabHistorico" => tabHistorico,
            "tabMemoria" => tabMemoria,
            _ => null,
        };

        /// <summary>O TabItem que disparou o evento.</summary>
        public TabItem? SenderAsTabItem(AylisClient.AylisEvent ev) => FindTabItem(ev.SenderName);
    }
}
