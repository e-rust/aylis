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

        public static AylisUi.FrmPlayerComponents frmPlayer = null!;

        /// <summary>Registra as classes dos forms (o registro dos forms chama).</summary>
        public static void RegisterClasses()
        {
            Aylis.RegisterFormClass("frmPlayer", (aylis, id, name) => new AylisUi.FrmPlayerComponents(aylis, id, name), form => frmPlayer = (AylisUi.FrmPlayerComponents)form);
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

    /// <summary>Um Divider de um form.</summary>
    public class Divider(Aylis aylis, long id, string name) : AylisComponent(aylis, id, name)
    {
        public bool Vertical { get => Bool(GetProp("vertical")); set => SetProp("vertical", value); }
        public string Text { get => Text(GetProp("text")); set => SetProp("text", value); }
        public double Thickness { get => Number(GetProp("thickness")); set => SetProp("thickness", value); }
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

    /// <summary>Um Icon de um form.</summary>
    public class Icon(Aylis aylis, long id, string name) : AylisComponent(aylis, id, name)
    {
        public string IconValue { get => Text(GetProp("icon")); set => SetProp("icon", value); }
        public string Color { get => Text(GetProp("color")); set => SetProp("color", value); }
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

    /// <summary>Um MainMenu de um form.</summary>
    public class MainMenu(Aylis aylis, long id, string name) : AylisComponent(aylis, id, name)
    {
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

    /// <summary>Um MenuItem de um form.</summary>
    public class MenuItem(Aylis aylis, long id, string name) : AylisComponent(aylis, id, name)
    {
        public string Text { get => Text(GetProp("text")); set => SetProp("text", value); }
        public string Shortcut { get => Text(GetProp("shortcut")); set => SetProp("shortcut", value); }
        public bool Enabled { get => Bool(GetProp("enabled")); set => SetProp("enabled", value); }
        public bool Checked { get => Bool(GetProp("checked")); set => SetProp("checked", value); }
        public bool AutoCheck { get => Bool(GetProp("auto_check")); set => SetProp("auto_check", value); }
        public string Icon { get => Text(GetProp("icon")); set => SetProp("icon", value); }
        public bool Visible { get => Bool(GetProp("visible")); set => SetProp("visible", value); }
        public bool Danger { get => Bool(GetProp("danger")); set => SetProp("danger", value); }
        public bool Header { get => Bool(GetProp("header")); set => SetProp("header", value); }
    }

    /// <summary>Um OpenDialog de um form.</summary>
    public class OpenDialog(Aylis aylis, long id, string name) : AylisComponent(aylis, id, name)
    {
        public string Title { get => Text(GetProp("title")); set => SetProp("title", value); }
        public string Filter { get => Text(GetProp("filter")); set => SetProp("filter", value); }
        public string InitialDir { get => Text(GetProp("initial_dir")); set => SetProp("initial_dir", value); }
        public string FileName { get => Text(GetProp("file_name")); set => SetProp("file_name", value); }
        public bool Multiple { get => Bool(GetProp("multiple")); set => SetProp("multiple", value); }
        public void Execute() => Aylis.Send("execute", new JsonObject { ["target"] = Id });
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

    /// <summary>Um SeekBar de um form.</summary>
    public class SeekBar(Aylis aylis, long id, string name) : AylisComponent(aylis, id, name)
    {
        public double Position { get => Number(GetProp("position")); set => SetProp("position", value); }
        public double Duration { get => Number(GetProp("duration")); set => SetProp("duration", value); }
        public double Buffered { get => Number(GetProp("buffered")); set => SetProp("buffered", value); }
        public string Markers { get => Text(GetProp("markers")); set => SetProp("markers", value); }
        public bool ShowTime { get => Bool(GetProp("show_time")); set => SetProp("show_time", value); }
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

    /// <summary>Um Timer de um form.</summary>
    public class Timer(Aylis aylis, long id, string name) : AylisComponent(aylis, id, name)
    {
        public double Interval { get => Number(GetProp("interval")); set => SetProp("interval", value); }
        public bool Enabled { get => Bool(GetProp("enabled")); set => SetProp("enabled", value); }
        public string Precision { get => Text(GetProp("precision")); set => SetProp("precision", value); }
        public bool Background { get => Bool(GetProp("background")); set => SetProp("background", value); }
        public bool OneShot { get => Bool(GetProp("one_shot")); set => SetProp("one_shot", value); }
        public double Drift { get => Number(GetProp("drift")); set => SetProp("drift", value); }
        public void Restart() => Aylis.Send("invoke", new JsonObject { ["target"] = Id, ["method"] = "restart", ["args"] = new JsonArray() });
    }

    /// <summary>Um TrackBar de um form.</summary>
    public class TrackBar(Aylis aylis, long id, string name) : AylisComponent(aylis, id, name)
    {
        public double Value { get => Number(GetProp("value")); set => SetProp("value", value); }
        public double Min { get => Number(GetProp("min")); set => SetProp("min", value); }
        public double Max { get => Number(GetProp("max")); set => SetProp("max", value); }
        public double Frequency { get => Number(GetProp("frequency")); set => SetProp("frequency", value); }
        public bool Vertical { get => Bool(GetProp("vertical")); set => SetProp("vertical", value); }
        public double ThumbSize { get => Number(GetProp("thumb_size")); set => SetProp("thumb_size", value); }
        public double TrackSize { get => Number(GetProp("track_size")); set => SetProp("track_size", value); }
        public bool SolidThumb { get => Bool(GetProp("solid_thumb")); set => SetProp("solid_thumb", value); }
        public double Limit { get => Number(GetProp("limit")); set => SetProp("limit", value); }
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

    /// <summary>Um VideoPlayer de um form.</summary>
    public class VideoPlayer(Aylis aylis, long id, string name) : AylisComponent(aylis, id, name)
    {
        public string File { get => Text(GetProp("file")); set => SetProp("file", value); }
        public bool Autoplay { get => Bool(GetProp("autoplay")); set => SetProp("autoplay", value); }
        public bool Looping { get => Bool(GetProp("looping")); set => SetProp("looping", value); }
        public double Volume { get => Number(GetProp("volume")); set => SetProp("volume", value); }
        public bool Muted { get => Bool(GetProp("muted")); set => SetProp("muted", value); }
        public string Fit { get => Text(GetProp("fit")); set => SetProp("fit", value); }
        public double Radius { get => Number(GetProp("radius")); set => SetProp("radius", value); }
        public double Rate { get => Number(GetProp("rate")); set => SetProp("rate", value); }
        public string Output { get => Text(GetProp("output")); set => SetProp("output", value); }
        public bool ShowVideo { get => Bool(GetProp("show_video")); set => SetProp("show_video", value); }
        public long FramesAhead { get => (long)Number(GetProp("frames_ahead")); set => SetProp("frames_ahead", value); }
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
        public void Play() => Aylis.Send("media", new JsonObject { ["target"] = Id, ["op"] = "play" });
        public void Pause() => Aylis.Send("media", new JsonObject { ["target"] = Id, ["op"] = "pause" });
        public void Stop() => Aylis.Send("media", new JsonObject { ["target"] = Id, ["op"] = "stop" });
        public void Seek(double seconds) => Aylis.Send("media", new JsonObject { ["target"] = Id, ["op"] = "seek", ["seconds"] = seconds });
        public double Position() => Number(Aylis.Call("media", new JsonObject { ["target"] = Id, ["op"] = "position" }));
        public double Duration() => Number(Aylis.Call("media", new JsonObject { ["target"] = Id, ["op"] = "duration" }));
    }

    /// <summary>O form frmPlayer e os componentes dele (resolvidos ao abrir; cada janela tem a sua instância).</summary>
    public sealed class FrmPlayerComponents(Aylis aylis, long id, string name) : Form(aylis, id, name)
    {
        public MainMenu mnuPrincipal = null!;
        public MenuItem mnuArquivo = null!;
        public MenuItem mnuAbrir = null!;
        public MenuItem mnuLimparFila = null!;
        public MenuItem mnuSep1 = null!;
        public MenuItem mnuSair = null!;
        public MenuItem mnuReproduzir = null!;
        public MenuItem mnuReproduzirPausar = null!;
        public MenuItem mnuParar = null!;
        public MenuItem mnuAnterior = null!;
        public MenuItem mnuProxima = null!;
        public MenuItem mnuSep2 = null!;
        public MenuItem mnuEmbaralhar = null!;
        public MenuItem mnuRepetir = null!;
        public MenuItem mnuExibir = null!;
        public MenuItem mnuAgora = null!;
        public MenuItem mnuFila = null!;
        public Panel pnlControles = null!;
        public Layout lytBarra = null!;
        public Label lblTempo = null!;
        public Label lblDuracao = null!;
        public SeekBar sbPosicao = null!;
        public Layout lytInfo = null!;
        public Icon icoCapa = null!;
        public Layout lytTextos = null!;
        public Label lblTitulo = null!;
        public Label lblDetalhe = null!;
        public Layout lytVolume = null!;
        public Button btnMudo = null!;
        public TrackBar tbVolume = null!;
        public Layout lytBotoes = null!;
        public Layout lytBotoesCentro = null!;
        public Button btnEmbaralhar = null!;
        public Button btnAnterior = null!;
        public Button btnReproduzir = null!;
        public Button btnProxima = null!;
        public Button btnRepetir = null!;
        public Layout lytNavegacao = null!;
        public Button btnAgora = null!;
        public Button btnFila = null!;
        public Divider divNavegacao = null!;
        public Button btnAbrirArquivos = null!;
        public Panel pnlAgora = null!;
        public VideoPlayer vpVideo = null!;
        public Panel pnlFila = null!;
        public Label lblFila = null!;
        public Layout lytFilaBotoes = null!;
        public Button btnAdicionar = null!;
        public Button btnLimparFila = null!;
        public ListView lvFila = null!;
        public OpenDialog dlgAbrir = null!;
        public Timer tmrPosicao = null!;

        public override void Bind(JsonArray list)
        {
            mnuPrincipal = Resolve(list, "mnuPrincipal", "MainMenu", (i, d, n) => new MainMenu(i, d, n));
            mnuArquivo = Resolve(list, "mnuArquivo", "MenuItem", (i, d, n) => new MenuItem(i, d, n));
            mnuAbrir = Resolve(list, "mnuAbrir", "MenuItem", (i, d, n) => new MenuItem(i, d, n));
            mnuLimparFila = Resolve(list, "mnuLimparFila", "MenuItem", (i, d, n) => new MenuItem(i, d, n));
            mnuSep1 = Resolve(list, "mnuSep1", "MenuItem", (i, d, n) => new MenuItem(i, d, n));
            mnuSair = Resolve(list, "mnuSair", "MenuItem", (i, d, n) => new MenuItem(i, d, n));
            mnuReproduzir = Resolve(list, "mnuReproduzir", "MenuItem", (i, d, n) => new MenuItem(i, d, n));
            mnuReproduzirPausar = Resolve(list, "mnuReproduzirPausar", "MenuItem", (i, d, n) => new MenuItem(i, d, n));
            mnuParar = Resolve(list, "mnuParar", "MenuItem", (i, d, n) => new MenuItem(i, d, n));
            mnuAnterior = Resolve(list, "mnuAnterior", "MenuItem", (i, d, n) => new MenuItem(i, d, n));
            mnuProxima = Resolve(list, "mnuProxima", "MenuItem", (i, d, n) => new MenuItem(i, d, n));
            mnuSep2 = Resolve(list, "mnuSep2", "MenuItem", (i, d, n) => new MenuItem(i, d, n));
            mnuEmbaralhar = Resolve(list, "mnuEmbaralhar", "MenuItem", (i, d, n) => new MenuItem(i, d, n));
            mnuRepetir = Resolve(list, "mnuRepetir", "MenuItem", (i, d, n) => new MenuItem(i, d, n));
            mnuExibir = Resolve(list, "mnuExibir", "MenuItem", (i, d, n) => new MenuItem(i, d, n));
            mnuAgora = Resolve(list, "mnuAgora", "MenuItem", (i, d, n) => new MenuItem(i, d, n));
            mnuFila = Resolve(list, "mnuFila", "MenuItem", (i, d, n) => new MenuItem(i, d, n));
            pnlControles = Resolve(list, "pnlControles", "Panel", (i, d, n) => new Panel(i, d, n));
            lytBarra = Resolve(list, "lytBarra", "Layout", (i, d, n) => new Layout(i, d, n));
            lblTempo = Resolve(list, "lblTempo", "Label", (i, d, n) => new Label(i, d, n));
            lblDuracao = Resolve(list, "lblDuracao", "Label", (i, d, n) => new Label(i, d, n));
            sbPosicao = Resolve(list, "sbPosicao", "SeekBar", (i, d, n) => new SeekBar(i, d, n));
            lytInfo = Resolve(list, "lytInfo", "Layout", (i, d, n) => new Layout(i, d, n));
            icoCapa = Resolve(list, "icoCapa", "Icon", (i, d, n) => new Icon(i, d, n));
            lytTextos = Resolve(list, "lytTextos", "Layout", (i, d, n) => new Layout(i, d, n));
            lblTitulo = Resolve(list, "lblTitulo", "Label", (i, d, n) => new Label(i, d, n));
            lblDetalhe = Resolve(list, "lblDetalhe", "Label", (i, d, n) => new Label(i, d, n));
            lytVolume = Resolve(list, "lytVolume", "Layout", (i, d, n) => new Layout(i, d, n));
            btnMudo = Resolve(list, "btnMudo", "Button", (i, d, n) => new Button(i, d, n));
            tbVolume = Resolve(list, "tbVolume", "TrackBar", (i, d, n) => new TrackBar(i, d, n));
            lytBotoes = Resolve(list, "lytBotoes", "Layout", (i, d, n) => new Layout(i, d, n));
            lytBotoesCentro = Resolve(list, "lytBotoesCentro", "Layout", (i, d, n) => new Layout(i, d, n));
            btnEmbaralhar = Resolve(list, "btnEmbaralhar", "Button", (i, d, n) => new Button(i, d, n));
            btnAnterior = Resolve(list, "btnAnterior", "Button", (i, d, n) => new Button(i, d, n));
            btnReproduzir = Resolve(list, "btnReproduzir", "Button", (i, d, n) => new Button(i, d, n));
            btnProxima = Resolve(list, "btnProxima", "Button", (i, d, n) => new Button(i, d, n));
            btnRepetir = Resolve(list, "btnRepetir", "Button", (i, d, n) => new Button(i, d, n));
            lytNavegacao = Resolve(list, "lytNavegacao", "Layout", (i, d, n) => new Layout(i, d, n));
            btnAgora = Resolve(list, "btnAgora", "Button", (i, d, n) => new Button(i, d, n));
            btnFila = Resolve(list, "btnFila", "Button", (i, d, n) => new Button(i, d, n));
            divNavegacao = Resolve(list, "divNavegacao", "Divider", (i, d, n) => new Divider(i, d, n));
            btnAbrirArquivos = Resolve(list, "btnAbrirArquivos", "Button", (i, d, n) => new Button(i, d, n));
            pnlAgora = Resolve(list, "pnlAgora", "Panel", (i, d, n) => new Panel(i, d, n));
            vpVideo = Resolve(list, "vpVideo", "VideoPlayer", (i, d, n) => new VideoPlayer(i, d, n));
            pnlFila = Resolve(list, "pnlFila", "Panel", (i, d, n) => new Panel(i, d, n));
            lblFila = Resolve(list, "lblFila", "Label", (i, d, n) => new Label(i, d, n));
            lytFilaBotoes = Resolve(list, "lytFilaBotoes", "Layout", (i, d, n) => new Layout(i, d, n));
            btnAdicionar = Resolve(list, "btnAdicionar", "Button", (i, d, n) => new Button(i, d, n));
            btnLimparFila = Resolve(list, "btnLimparFila", "Button", (i, d, n) => new Button(i, d, n));
            lvFila = Resolve(list, "lvFila", "ListView", (i, d, n) => new ListView(i, d, n));
            dlgAbrir = Resolve(list, "dlgAbrir", "OpenDialog", (i, d, n) => new OpenDialog(i, d, n));
            tmrPosicao = Resolve(list, "tmrPosicao", "Timer", (i, d, n) => new Timer(i, d, n));
        }

        /// <summary>O Button deste form com o nome (como o FindComponent).</summary>
        public Button? FindButton(string name) => name switch
        {
            "btnMudo" => btnMudo,
            "btnEmbaralhar" => btnEmbaralhar,
            "btnAnterior" => btnAnterior,
            "btnReproduzir" => btnReproduzir,
            "btnProxima" => btnProxima,
            "btnRepetir" => btnRepetir,
            "btnAgora" => btnAgora,
            "btnFila" => btnFila,
            "btnAbrirArquivos" => btnAbrirArquivos,
            "btnAdicionar" => btnAdicionar,
            "btnLimparFila" => btnLimparFila,
            _ => null,
        };

        /// <summary>O Button que disparou o evento.</summary>
        public Button? SenderAsButton(AylisClient.AylisEvent ev) => FindButton(ev.SenderName);

        /// <summary>O Divider deste form com o nome (como o FindComponent).</summary>
        public Divider? FindDivider(string name) => name switch
        {
            "divNavegacao" => divNavegacao,
            _ => null,
        };

        /// <summary>O Divider que disparou o evento.</summary>
        public Divider? SenderAsDivider(AylisClient.AylisEvent ev) => FindDivider(ev.SenderName);

        /// <summary>O Icon deste form com o nome (como o FindComponent).</summary>
        public Icon? FindIcon(string name) => name switch
        {
            "icoCapa" => icoCapa,
            _ => null,
        };

        /// <summary>O Icon que disparou o evento.</summary>
        public Icon? SenderAsIcon(AylisClient.AylisEvent ev) => FindIcon(ev.SenderName);

        /// <summary>O Label deste form com o nome (como o FindComponent).</summary>
        public Label? FindLabel(string name) => name switch
        {
            "lblTempo" => lblTempo,
            "lblDuracao" => lblDuracao,
            "lblTitulo" => lblTitulo,
            "lblDetalhe" => lblDetalhe,
            "lblFila" => lblFila,
            _ => null,
        };

        /// <summary>O Label que disparou o evento.</summary>
        public Label? SenderAsLabel(AylisClient.AylisEvent ev) => FindLabel(ev.SenderName);

        /// <summary>O Layout deste form com o nome (como o FindComponent).</summary>
        public Layout? FindLayout(string name) => name switch
        {
            "lytBarra" => lytBarra,
            "lytInfo" => lytInfo,
            "lytTextos" => lytTextos,
            "lytVolume" => lytVolume,
            "lytBotoes" => lytBotoes,
            "lytBotoesCentro" => lytBotoesCentro,
            "lytNavegacao" => lytNavegacao,
            "lytFilaBotoes" => lytFilaBotoes,
            _ => null,
        };

        /// <summary>O Layout que disparou o evento.</summary>
        public Layout? SenderAsLayout(AylisClient.AylisEvent ev) => FindLayout(ev.SenderName);

        /// <summary>O ListView deste form com o nome (como o FindComponent).</summary>
        public ListView? FindListView(string name) => name switch
        {
            "lvFila" => lvFila,
            _ => null,
        };

        /// <summary>O ListView que disparou o evento.</summary>
        public ListView? SenderAsListView(AylisClient.AylisEvent ev) => FindListView(ev.SenderName);

        /// <summary>O MainMenu deste form com o nome (como o FindComponent).</summary>
        public MainMenu? FindMainMenu(string name) => name switch
        {
            "mnuPrincipal" => mnuPrincipal,
            _ => null,
        };

        /// <summary>O MainMenu que disparou o evento.</summary>
        public MainMenu? SenderAsMainMenu(AylisClient.AylisEvent ev) => FindMainMenu(ev.SenderName);

        /// <summary>O MenuItem deste form com o nome (como o FindComponent).</summary>
        public MenuItem? FindMenuItem(string name) => name switch
        {
            "mnuArquivo" => mnuArquivo,
            "mnuAbrir" => mnuAbrir,
            "mnuLimparFila" => mnuLimparFila,
            "mnuSep1" => mnuSep1,
            "mnuSair" => mnuSair,
            "mnuReproduzir" => mnuReproduzir,
            "mnuReproduzirPausar" => mnuReproduzirPausar,
            "mnuParar" => mnuParar,
            "mnuAnterior" => mnuAnterior,
            "mnuProxima" => mnuProxima,
            "mnuSep2" => mnuSep2,
            "mnuEmbaralhar" => mnuEmbaralhar,
            "mnuRepetir" => mnuRepetir,
            "mnuExibir" => mnuExibir,
            "mnuAgora" => mnuAgora,
            "mnuFila" => mnuFila,
            _ => null,
        };

        /// <summary>O MenuItem que disparou o evento.</summary>
        public MenuItem? SenderAsMenuItem(AylisClient.AylisEvent ev) => FindMenuItem(ev.SenderName);

        /// <summary>O OpenDialog deste form com o nome (como o FindComponent).</summary>
        public OpenDialog? FindOpenDialog(string name) => name switch
        {
            "dlgAbrir" => dlgAbrir,
            _ => null,
        };

        /// <summary>O OpenDialog que disparou o evento.</summary>
        public OpenDialog? SenderAsOpenDialog(AylisClient.AylisEvent ev) => FindOpenDialog(ev.SenderName);

        /// <summary>O Panel deste form com o nome (como o FindComponent).</summary>
        public Panel? FindPanel(string name) => name switch
        {
            "pnlControles" => pnlControles,
            "pnlAgora" => pnlAgora,
            "pnlFila" => pnlFila,
            _ => null,
        };

        /// <summary>O Panel que disparou o evento.</summary>
        public Panel? SenderAsPanel(AylisClient.AylisEvent ev) => FindPanel(ev.SenderName);

        /// <summary>O SeekBar deste form com o nome (como o FindComponent).</summary>
        public SeekBar? FindSeekBar(string name) => name switch
        {
            "sbPosicao" => sbPosicao,
            _ => null,
        };

        /// <summary>O SeekBar que disparou o evento.</summary>
        public SeekBar? SenderAsSeekBar(AylisClient.AylisEvent ev) => FindSeekBar(ev.SenderName);

        /// <summary>O Timer deste form com o nome (como o FindComponent).</summary>
        public Timer? FindTimer(string name) => name switch
        {
            "tmrPosicao" => tmrPosicao,
            _ => null,
        };

        /// <summary>O Timer que disparou o evento.</summary>
        public Timer? SenderAsTimer(AylisClient.AylisEvent ev) => FindTimer(ev.SenderName);

        /// <summary>O TrackBar deste form com o nome (como o FindComponent).</summary>
        public TrackBar? FindTrackBar(string name) => name switch
        {
            "tbVolume" => tbVolume,
            _ => null,
        };

        /// <summary>O TrackBar que disparou o evento.</summary>
        public TrackBar? SenderAsTrackBar(AylisClient.AylisEvent ev) => FindTrackBar(ev.SenderName);

        /// <summary>O VideoPlayer deste form com o nome (como o FindComponent).</summary>
        public VideoPlayer? FindVideoPlayer(string name) => name switch
        {
            "vpVideo" => vpVideo,
            _ => null,
        };

        /// <summary>O VideoPlayer que disparou o evento.</summary>
        public VideoPlayer? SenderAsVideoPlayer(AylisClient.AylisEvent ev) => FindVideoPlayer(ev.SenderName);
    }
}
