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

        public static AylisUi.FrmPaintComponents frmPaint = null!;

        /// <summary>Registra as classes dos forms (o registro dos forms chama).</summary>
        public static void RegisterClasses()
        {
            Aylis.RegisterFormClass("frmPaint", (aylis, id, name) => new AylisUi.FrmPaintComponents(aylis, id, name), form => frmPaint = (AylisUi.FrmPaintComponents)form);
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
        public double Left { get => Number(GetProp("left")); set => SetProp("left", value); }
        public double Top { get => Number(GetProp("top")); set => SetProp("top", value); }
        public double Width { get => Number(GetProp("width")); set => SetProp("width", value); }
        public double Height { get => Number(GetProp("height")); set => SetProp("height", value); }
        public string Align { get => Text(GetProp("align")); set => SetProp("align", value); }
        public bool Visible { get => Bool(GetProp("visible")); set => SetProp("visible", value); }
        public bool Enabled { get => Bool(GetProp("enabled")); set => SetProp("enabled", value); }
        public bool HitTest { get => Bool(GetProp("hit_test")); set => SetProp("hit_test", value); }
        public bool ClipChildren { get => Bool(GetProp("clip_children")); set => SetProp("clip_children", value); }
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
        public string PopupMenu { get => Text(GetProp("popup_menu")); set => SetProp("popup_menu", value); }
    }

    /// <summary>Um CheckBox de um form.</summary>
    public class CheckBox(Aylis aylis, long id, string name) : AylisComponent(aylis, id, name)
    {
        public string Text { get => Text(GetProp("text")); set => SetProp("text", value); }
        public bool Checked { get => Bool(GetProp("checked")); set => SetProp("checked", value); }
        public double BoxSize { get => Number(GetProp("box_size")); set => SetProp("box_size", value); }
        public double Left { get => Number(GetProp("left")); set => SetProp("left", value); }
        public double Top { get => Number(GetProp("top")); set => SetProp("top", value); }
        public double Width { get => Number(GetProp("width")); set => SetProp("width", value); }
        public double Height { get => Number(GetProp("height")); set => SetProp("height", value); }
        public string Align { get => Text(GetProp("align")); set => SetProp("align", value); }
        public bool Visible { get => Bool(GetProp("visible")); set => SetProp("visible", value); }
        public bool Enabled { get => Bool(GetProp("enabled")); set => SetProp("enabled", value); }
        public bool HitTest { get => Bool(GetProp("hit_test")); set => SetProp("hit_test", value); }
        public bool ClipChildren { get => Bool(GetProp("clip_children")); set => SetProp("clip_children", value); }
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
        public string PopupMenu { get => Text(GetProp("popup_menu")); set => SetProp("popup_menu", value); }
    }

    /// <summary>Um Circle de um form.</summary>
    public class Circle(Aylis aylis, long id, string name) : AylisComponent(aylis, id, name)
    {
        public string Fill { get => Text(GetProp("fill")); set => SetProp("fill", value); }
        public string Stroke { get => Text(GetProp("stroke")); set => SetProp("stroke", value); }
        public double StrokeWidth { get => Number(GetProp("stroke_width")); set => SetProp("stroke_width", value); }
        public double Left { get => Number(GetProp("left")); set => SetProp("left", value); }
        public double Top { get => Number(GetProp("top")); set => SetProp("top", value); }
        public double Width { get => Number(GetProp("width")); set => SetProp("width", value); }
        public double Height { get => Number(GetProp("height")); set => SetProp("height", value); }
        public string Align { get => Text(GetProp("align")); set => SetProp("align", value); }
        public bool Visible { get => Bool(GetProp("visible")); set => SetProp("visible", value); }
        public bool Enabled { get => Bool(GetProp("enabled")); set => SetProp("enabled", value); }
        public bool HitTest { get => Bool(GetProp("hit_test")); set => SetProp("hit_test", value); }
        public bool ClipChildren { get => Bool(GetProp("clip_children")); set => SetProp("clip_children", value); }
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
        public string PopupMenu { get => Text(GetProp("popup_menu")); set => SetProp("popup_menu", value); }
    }

    /// <summary>Um ComboBox de um form.</summary>
    public class ComboBox(Aylis aylis, long id, string name) : AylisComponent(aylis, id, name)
    {
        public string Items { get => Text(GetProp("items")); set => SetProp("items", value); }
        public string Values { get => Text(GetProp("values")); set => SetProp("values", value); }
        public long ItemIndex { get => (long)Number(GetProp("item_index")); set => SetProp("item_index", value); }
        public string HAlign { get => Text(GetProp("h_align")); set => SetProp("h_align", value); }
        public string Trimming { get => Text(GetProp("trimming")); set => SetProp("trimming", value); }
        public string Arrow { get => Text(GetProp("arrow")); set => SetProp("arrow", value); }
        public bool Sensitive { get => Bool(GetProp("sensitive")); set => SetProp("sensitive", value); }
        public string Value { get => Text(GetProp("value")); set => SetProp("value", value); }
        public double Left { get => Number(GetProp("left")); set => SetProp("left", value); }
        public double Top { get => Number(GetProp("top")); set => SetProp("top", value); }
        public double Width { get => Number(GetProp("width")); set => SetProp("width", value); }
        public double Height { get => Number(GetProp("height")); set => SetProp("height", value); }
        public string Align { get => Text(GetProp("align")); set => SetProp("align", value); }
        public bool Visible { get => Bool(GetProp("visible")); set => SetProp("visible", value); }
        public bool Enabled { get => Bool(GetProp("enabled")); set => SetProp("enabled", value); }
        public bool HitTest { get => Bool(GetProp("hit_test")); set => SetProp("hit_test", value); }
        public bool ClipChildren { get => Bool(GetProp("clip_children")); set => SetProp("clip_children", value); }
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
        public bool Visible { get => Bool(GetProp("visible")); set => SetProp("visible", value); }
        public bool Enabled { get => Bool(GetProp("enabled")); set => SetProp("enabled", value); }
        public bool HitTest { get => Bool(GetProp("hit_test")); set => SetProp("hit_test", value); }
        public bool ClipChildren { get => Bool(GetProp("clip_children")); set => SetProp("clip_children", value); }
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
        public double Left { get => Number(GetProp("left")); set => SetProp("left", value); }
        public double Top { get => Number(GetProp("top")); set => SetProp("top", value); }
        public double Width { get => Number(GetProp("width")); set => SetProp("width", value); }
        public double Height { get => Number(GetProp("height")); set => SetProp("height", value); }
        public string Align { get => Text(GetProp("align")); set => SetProp("align", value); }
        public bool Visible { get => Bool(GetProp("visible")); set => SetProp("visible", value); }
        public bool Enabled { get => Bool(GetProp("enabled")); set => SetProp("enabled", value); }
        public bool HitTest { get => Bool(GetProp("hit_test")); set => SetProp("hit_test", value); }
        public bool ClipChildren { get => Bool(GetProp("clip_children")); set => SetProp("clip_children", value); }
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
        public string PopupMenu { get => Text(GetProp("popup_menu")); set => SetProp("popup_menu", value); }
        public void MoveToMonitor(long a1) => Aylis.Send("invoke", new JsonObject { ["target"] = Id, ["method"] = "move_to_monitor", ["args"] = new JsonArray(a1) });
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
        public bool Visible { get => Bool(GetProp("visible")); set => SetProp("visible", value); }
        public bool Enabled { get => Bool(GetProp("enabled")); set => SetProp("enabled", value); }
        public bool HitTest { get => Bool(GetProp("hit_test")); set => SetProp("hit_test", value); }
        public bool ClipChildren { get => Bool(GetProp("clip_children")); set => SetProp("clip_children", value); }
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
        public string FocusControl { get => Text(GetProp("focus_control")); set => SetProp("focus_control", value); }
        public double Left { get => Number(GetProp("left")); set => SetProp("left", value); }
        public double Top { get => Number(GetProp("top")); set => SetProp("top", value); }
        public double Width { get => Number(GetProp("width")); set => SetProp("width", value); }
        public double Height { get => Number(GetProp("height")); set => SetProp("height", value); }
        public string Align { get => Text(GetProp("align")); set => SetProp("align", value); }
        public bool Visible { get => Bool(GetProp("visible")); set => SetProp("visible", value); }
        public bool Enabled { get => Bool(GetProp("enabled")); set => SetProp("enabled", value); }
        public bool HitTest { get => Bool(GetProp("hit_test")); set => SetProp("hit_test", value); }
        public bool ClipChildren { get => Bool(GetProp("clip_children")); set => SetProp("clip_children", value); }
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
        public string PopupMenu { get => Text(GetProp("popup_menu")); set => SetProp("popup_menu", value); }
    }

    /// <summary>Um Layout de um form.</summary>
    public class Layout(Aylis aylis, long id, string name) : AylisComponent(aylis, id, name)
    {
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
        public bool Visible { get => Bool(GetProp("visible")); set => SetProp("visible", value); }
        public bool Enabled { get => Bool(GetProp("enabled")); set => SetProp("enabled", value); }
        public bool HitTest { get => Bool(GetProp("hit_test")); set => SetProp("hit_test", value); }
        public bool ClipChildren { get => Bool(GetProp("clip_children")); set => SetProp("clip_children", value); }
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
        public bool Visible { get => Bool(GetProp("visible")); set => SetProp("visible", value); }
        public bool Enabled { get => Bool(GetProp("enabled")); set => SetProp("enabled", value); }
        public bool HitTest { get => Bool(GetProp("hit_test")); set => SetProp("hit_test", value); }
        public bool ClipChildren { get => Bool(GetProp("clip_children")); set => SetProp("clip_children", value); }
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

    /// <summary>Um PaintCanvas de um form.</summary>
    public class PaintCanvas(Aylis aylis, long id, string name) : AylisComponent(aylis, id, name)
    {
        public string Tool { get => Text(GetProp("tool")); set => SetProp("tool", value); }
        public string Color { get => Text(GetProp("color")); set => SetProp("color", value); }
        public string BackColor { get => Text(GetProp("back_color")); set => SetProp("back_color", value); }
        public long BrushSize { get => (long)Number(GetProp("brush_size")); set => SetProp("brush_size", value); }
        public bool Filled { get => Bool(GetProp("filled")); set => SetProp("filled", value); }
        public double Zoom { get => Number(GetProp("zoom")); set => SetProp("zoom", value); }
        public double Left { get => Number(GetProp("left")); set => SetProp("left", value); }
        public double Top { get => Number(GetProp("top")); set => SetProp("top", value); }
        public double Width { get => Number(GetProp("width")); set => SetProp("width", value); }
        public double Height { get => Number(GetProp("height")); set => SetProp("height", value); }
        public string Align { get => Text(GetProp("align")); set => SetProp("align", value); }
        public bool Visible { get => Bool(GetProp("visible")); set => SetProp("visible", value); }
        public bool Enabled { get => Bool(GetProp("enabled")); set => SetProp("enabled", value); }
        public bool HitTest { get => Bool(GetProp("hit_test")); set => SetProp("hit_test", value); }
        public bool ClipChildren { get => Bool(GetProp("clip_children")); set => SetProp("clip_children", value); }
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
        public string PopupMenu { get => Text(GetProp("popup_menu")); set => SetProp("popup_menu", value); }
        public bool Undo() => Bool(Aylis.Call("invoke", new JsonObject { ["target"] = Id, ["method"] = "undo", ["args"] = new JsonArray() }));
        public bool Redo() => Bool(Aylis.Call("invoke", new JsonObject { ["target"] = Id, ["method"] = "redo", ["args"] = new JsonArray() }));
        public bool CanUndo() => Bool(Aylis.Call("invoke", new JsonObject { ["target"] = Id, ["method"] = "can_undo", ["args"] = new JsonArray() }));
        public bool CanRedo() => Bool(Aylis.Call("invoke", new JsonObject { ["target"] = Id, ["method"] = "can_redo", ["args"] = new JsonArray() }));
        public string Size() => Text(Aylis.Call("invoke", new JsonObject { ["target"] = Id, ["method"] = "size", ["args"] = new JsonArray() }));
        public string Checksum() => Text(Aylis.Call("invoke", new JsonObject { ["target"] = Id, ["method"] = "checksum", ["args"] = new JsonArray() }));
        public bool Clear() => Bool(Aylis.Call("invoke", new JsonObject { ["target"] = Id, ["method"] = "clear", ["args"] = new JsonArray() }));
        public bool Resize(long width, long height) => Bool(Aylis.Call("invoke", new JsonObject { ["target"] = Id, ["method"] = "resize", ["args"] = new JsonArray(width, height) }));
        public bool New(long width, long height) => Bool(Aylis.Call("invoke", new JsonObject { ["target"] = Id, ["method"] = "new", ["args"] = new JsonArray(width, height) }));
        public bool Open(string path) => Bool(Aylis.Call("invoke", new JsonObject { ["target"] = Id, ["method"] = "open", ["args"] = new JsonArray(path) }));
        public bool Save(string path) => Bool(Aylis.Call("invoke", new JsonObject { ["target"] = Id, ["method"] = "save", ["args"] = new JsonArray(path) }));
        public bool SelectAll() => Bool(Aylis.Call("invoke", new JsonObject { ["target"] = Id, ["method"] = "select_all", ["args"] = new JsonArray() }));
        public bool HasSelection() => Bool(Aylis.Call("invoke", new JsonObject { ["target"] = Id, ["method"] = "has_selection", ["args"] = new JsonArray() }));
        public bool Copy() => Bool(Aylis.Call("invoke", new JsonObject { ["target"] = Id, ["method"] = "copy", ["args"] = new JsonArray() }));
        public bool Cut() => Bool(Aylis.Call("invoke", new JsonObject { ["target"] = Id, ["method"] = "cut", ["args"] = new JsonArray() }));
        public bool Paste() => Bool(Aylis.Call("invoke", new JsonObject { ["target"] = Id, ["method"] = "paste", ["args"] = new JsonArray() }));
        public bool Delete() => Bool(Aylis.Call("invoke", new JsonObject { ["target"] = Id, ["method"] = "delete", ["args"] = new JsonArray() }));
        public bool Deselect() => Bool(Aylis.Call("invoke", new JsonObject { ["target"] = Id, ["method"] = "deselect", ["args"] = new JsonArray() }));
    }

    /// <summary>Um Panel de um form.</summary>
    public class Panel(Aylis aylis, long id, string name) : AylisComponent(aylis, id, name)
    {
        public string AllowedChars { get => Text(GetProp("allowed_chars")); set => SetProp("allowed_chars", value); }
        public string BlockedChars { get => Text(GetProp("blocked_chars")); set => SetProp("blocked_chars", value); }
        public bool OnlyAllowed { get => Bool(GetProp("only_allowed")); set => SetProp("only_allowed", value); }
        public bool CharsOverride { get => Bool(GetProp("chars_override")); set => SetProp("chars_override", value); }
        public string CharMap { get => Text(GetProp("char_map")); set => SetProp("char_map", value); }
        public string Fill { get => Text(GetProp("fill")); set => SetProp("fill", value); }
        public string Fill2 { get => Text(GetProp("fill2")); set => SetProp("fill2", value); }
        public string FillMid { get => Text(GetProp("fill_mid")); set => SetProp("fill_mid", value); }
        public double FillMidAt { get => Number(GetProp("fill_mid_at")); set => SetProp("fill_mid_at", value); }
        public double GradientAngle { get => Number(GetProp("gradient_angle")); set => SetProp("gradient_angle", value); }
        public bool GradientRadial { get => Bool(GetProp("gradient_radial")); set => SetProp("gradient_radial", value); }
        public string Stroke { get => Text(GetProp("stroke")); set => SetProp("stroke", value); }
        public double StrokeWidth { get => Number(GetProp("stroke_width")); set => SetProp("stroke_width", value); }
        public double Radius { get => Number(GetProp("radius")); set => SetProp("radius", value); }
        public double Left { get => Number(GetProp("left")); set => SetProp("left", value); }
        public double Top { get => Number(GetProp("top")); set => SetProp("top", value); }
        public double Width { get => Number(GetProp("width")); set => SetProp("width", value); }
        public double Height { get => Number(GetProp("height")); set => SetProp("height", value); }
        public string Align { get => Text(GetProp("align")); set => SetProp("align", value); }
        public bool Visible { get => Bool(GetProp("visible")); set => SetProp("visible", value); }
        public bool Enabled { get => Bool(GetProp("enabled")); set => SetProp("enabled", value); }
        public bool HitTest { get => Bool(GetProp("hit_test")); set => SetProp("hit_test", value); }
        public bool ClipChildren { get => Bool(GetProp("clip_children")); set => SetProp("clip_children", value); }
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
        public string PopupMenu { get => Text(GetProp("popup_menu")); set => SetProp("popup_menu", value); }
    }

    /// <summary>Um SaveDialog de um form.</summary>
    public class SaveDialog(Aylis aylis, long id, string name) : AylisComponent(aylis, id, name)
    {
        public string Title { get => Text(GetProp("title")); set => SetProp("title", value); }
        public string Filter { get => Text(GetProp("filter")); set => SetProp("filter", value); }
        public string InitialDir { get => Text(GetProp("initial_dir")); set => SetProp("initial_dir", value); }
        public string FileName { get => Text(GetProp("file_name")); set => SetProp("file_name", value); }
        public bool Multiple { get => Bool(GetProp("multiple")); set => SetProp("multiple", value); }
        public void Execute() => Aylis.Send("execute", new JsonObject { ["target"] = Id });
    }

    /// <summary>Um ScrollBox de um form.</summary>
    public class ScrollBox(Aylis aylis, long id, string name) : AylisComponent(aylis, id, name)
    {
        public bool Horizontal { get => Bool(GetProp("horizontal")); set => SetProp("horizontal", value); }
        public bool PullToRefresh { get => Bool(GetProp("pull_to_refresh")); set => SetProp("pull_to_refresh", value); }
        public bool Refreshing { get => Bool(GetProp("refreshing")); set => SetProp("refreshing", value); }
        public double WheelStep { get => Number(GetProp("wheel_step")); set => SetProp("wheel_step", value); }
        public string Smooth { get => Text(GetProp("smooth")); set => SetProp("smooth", value); }
        public string AllowedChars { get => Text(GetProp("allowed_chars")); set => SetProp("allowed_chars", value); }
        public string BlockedChars { get => Text(GetProp("blocked_chars")); set => SetProp("blocked_chars", value); }
        public bool OnlyAllowed { get => Bool(GetProp("only_allowed")); set => SetProp("only_allowed", value); }
        public bool CharsOverride { get => Bool(GetProp("chars_override")); set => SetProp("chars_override", value); }
        public string CharMap { get => Text(GetProp("char_map")); set => SetProp("char_map", value); }
        public double ScrollPosition { get => Number(GetProp("scroll_position")); set => SetProp("scroll_position", value); }
        public double Left { get => Number(GetProp("left")); set => SetProp("left", value); }
        public double Top { get => Number(GetProp("top")); set => SetProp("top", value); }
        public double Width { get => Number(GetProp("width")); set => SetProp("width", value); }
        public double Height { get => Number(GetProp("height")); set => SetProp("height", value); }
        public string Align { get => Text(GetProp("align")); set => SetProp("align", value); }
        public bool Visible { get => Bool(GetProp("visible")); set => SetProp("visible", value); }
        public bool Enabled { get => Bool(GetProp("enabled")); set => SetProp("enabled", value); }
        public bool HitTest { get => Bool(GetProp("hit_test")); set => SetProp("hit_test", value); }
        public bool ClipChildren { get => Bool(GetProp("clip_children")); set => SetProp("clip_children", value); }
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
        public string PopupMenu { get => Text(GetProp("popup_menu")); set => SetProp("popup_menu", value); }
        public void ScrollTo(AylisComponent a1) => Aylis.Send("invoke", new JsonObject { ["target"] = Id, ["method"] = "scroll_to", ["args"] = new JsonArray(a1.Id) });
        public string VisibleComponents(AylisComponent a1, bool a2) => Text(Aylis.Call("invoke", new JsonObject { ["target"] = Id, ["method"] = "visible_components", ["args"] = new JsonArray(a1.Id, a2) }));
    }

    /// <summary>Um StatusBar de um form.</summary>
    public class StatusBar(Aylis aylis, long id, string name) : AylisComponent(aylis, id, name)
    {
        public string Text { get => Text(GetProp("text")); set => SetProp("text", value); }
        public double Left { get => Number(GetProp("left")); set => SetProp("left", value); }
        public double Top { get => Number(GetProp("top")); set => SetProp("top", value); }
        public double Width { get => Number(GetProp("width")); set => SetProp("width", value); }
        public double Height { get => Number(GetProp("height")); set => SetProp("height", value); }
        public string Align { get => Text(GetProp("align")); set => SetProp("align", value); }
        public bool Visible { get => Bool(GetProp("visible")); set => SetProp("visible", value); }
        public bool Enabled { get => Bool(GetProp("enabled")); set => SetProp("enabled", value); }
        public bool HitTest { get => Bool(GetProp("hit_test")); set => SetProp("hit_test", value); }
        public bool ClipChildren { get => Bool(GetProp("clip_children")); set => SetProp("clip_children", value); }
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
        public string PopupMenu { get => Text(GetProp("popup_menu")); set => SetProp("popup_menu", value); }
    }

    /// <summary>O form frmPaint e os componentes dele (resolvidos ao abrir; cada janela tem a sua instância).</summary>
    public sealed class FrmPaintComponents(Aylis aylis, long id, string name) : Form(aylis, id, name)
    {
        public Layout lytTopo = null!;
        public MainMenu mnuPrincipal = null!;
        public MenuItem mnuArquivo = null!;
        public MenuItem mnuNovo = null!;
        public MenuItem mnuAbrir = null!;
        public MenuItem mnuSalvar = null!;
        public MenuItem mnuSalvarComo = null!;
        public MenuItem mnuSep1 = null!;
        public MenuItem mnuSair = null!;
        public MenuItem mnuEditar = null!;
        public MenuItem mnuDesfazer = null!;
        public MenuItem mnuRefazer = null!;
        public MenuItem mnuSep2 = null!;
        public MenuItem mnuRecortar = null!;
        public MenuItem mnuCopiar = null!;
        public MenuItem mnuColar = null!;
        public MenuItem mnuExcluir = null!;
        public MenuItem mnuSelecionarTudo = null!;
        public MenuItem mnuSep3 = null!;
        public MenuItem mnuLimpar = null!;
        public MenuItem mnuExibir = null!;
        public MenuItem mnuAmpliar = null!;
        public MenuItem mnuReduzir = null!;
        public MenuItem mnuTamanhoReal = null!;
        public Divider divTopo = null!;
        public Button btnSalvar = null!;
        public Button btnDesfazer = null!;
        public Button btnRefazer = null!;
        public Panel pnlFaixa = null!;
        public Layout lytSelecao = null!;
        public Label lblSelecaoGrupo = null!;
        public Button btnSelecionar = null!;
        public Divider divSelecao = null!;
        public Layout lytFerramentas = null!;
        public Label lblFerramentasGrupo = null!;
        public GridLayout grdFerramentas = null!;
        public Button btnLapis = null!;
        public Button btnBalde = null!;
        public Button btnBorracha = null!;
        public Button btnContaGotas = null!;
        public Divider divFerramentas = null!;
        public Layout lytPinceis = null!;
        public Label lblPinceis = null!;
        public Button btnPincel = null!;
        public Divider divPinceis = null!;
        public Layout lytFormas = null!;
        public Label lblFormas = null!;
        public CheckBox chkPreencher = null!;
        public GridLayout grdFormas = null!;
        public Button btnLinha = null!;
        public Button btnRetangulo = null!;
        public Button btnElipse = null!;
        public Divider divFormas = null!;
        public Layout lytTamanho = null!;
        public Label lblTamanhoGrupo = null!;
        public ComboBox cbEspessura = null!;
        public Divider divTamanho = null!;
        public Layout lytCores = null!;
        public Label lblCores = null!;
        public Circle btnCor1 = null!;
        public Circle btnCor2 = null!;
        public GridLayout grdPaleta = null!;
        public Circle btnPaleta01 = null!;
        public Circle btnPaleta02 = null!;
        public Circle btnPaleta03 = null!;
        public Circle btnPaleta04 = null!;
        public Circle btnPaleta05 = null!;
        public Circle btnPaleta06 = null!;
        public Circle btnPaleta07 = null!;
        public Circle btnPaleta08 = null!;
        public Circle btnPaleta09 = null!;
        public Circle btnPaleta10 = null!;
        public Circle btnPaleta11 = null!;
        public Circle btnPaleta12 = null!;
        public Circle btnPaleta13 = null!;
        public Circle btnPaleta14 = null!;
        public Circle btnPaleta15 = null!;
        public Circle btnPaleta16 = null!;
        public Circle btnPaleta17 = null!;
        public Circle btnPaleta18 = null!;
        public Circle btnPaleta19 = null!;
        public Circle btnPaleta20 = null!;
        public StatusBar stbStatus = null!;
        public Label lblPosicao = null!;
        public Label lblTamanho = null!;
        public Label lblZoom = null!;
        public ScrollBox sbxArea = null!;
        public PaintCanvas pcDesenho = null!;
        public OpenDialog dlgAbrir = null!;
        public SaveDialog dlgSalvar = null!;

        public override void Bind(JsonArray list)
        {
            lytTopo = Resolve(list, "lytTopo", "Layout", (i, d, n) => new Layout(i, d, n));
            mnuPrincipal = Resolve(list, "mnuPrincipal", "MainMenu", (i, d, n) => new MainMenu(i, d, n));
            mnuArquivo = Resolve(list, "mnuArquivo", "MenuItem", (i, d, n) => new MenuItem(i, d, n));
            mnuNovo = Resolve(list, "mnuNovo", "MenuItem", (i, d, n) => new MenuItem(i, d, n));
            mnuAbrir = Resolve(list, "mnuAbrir", "MenuItem", (i, d, n) => new MenuItem(i, d, n));
            mnuSalvar = Resolve(list, "mnuSalvar", "MenuItem", (i, d, n) => new MenuItem(i, d, n));
            mnuSalvarComo = Resolve(list, "mnuSalvarComo", "MenuItem", (i, d, n) => new MenuItem(i, d, n));
            mnuSep1 = Resolve(list, "mnuSep1", "MenuItem", (i, d, n) => new MenuItem(i, d, n));
            mnuSair = Resolve(list, "mnuSair", "MenuItem", (i, d, n) => new MenuItem(i, d, n));
            mnuEditar = Resolve(list, "mnuEditar", "MenuItem", (i, d, n) => new MenuItem(i, d, n));
            mnuDesfazer = Resolve(list, "mnuDesfazer", "MenuItem", (i, d, n) => new MenuItem(i, d, n));
            mnuRefazer = Resolve(list, "mnuRefazer", "MenuItem", (i, d, n) => new MenuItem(i, d, n));
            mnuSep2 = Resolve(list, "mnuSep2", "MenuItem", (i, d, n) => new MenuItem(i, d, n));
            mnuRecortar = Resolve(list, "mnuRecortar", "MenuItem", (i, d, n) => new MenuItem(i, d, n));
            mnuCopiar = Resolve(list, "mnuCopiar", "MenuItem", (i, d, n) => new MenuItem(i, d, n));
            mnuColar = Resolve(list, "mnuColar", "MenuItem", (i, d, n) => new MenuItem(i, d, n));
            mnuExcluir = Resolve(list, "mnuExcluir", "MenuItem", (i, d, n) => new MenuItem(i, d, n));
            mnuSelecionarTudo = Resolve(list, "mnuSelecionarTudo", "MenuItem", (i, d, n) => new MenuItem(i, d, n));
            mnuSep3 = Resolve(list, "mnuSep3", "MenuItem", (i, d, n) => new MenuItem(i, d, n));
            mnuLimpar = Resolve(list, "mnuLimpar", "MenuItem", (i, d, n) => new MenuItem(i, d, n));
            mnuExibir = Resolve(list, "mnuExibir", "MenuItem", (i, d, n) => new MenuItem(i, d, n));
            mnuAmpliar = Resolve(list, "mnuAmpliar", "MenuItem", (i, d, n) => new MenuItem(i, d, n));
            mnuReduzir = Resolve(list, "mnuReduzir", "MenuItem", (i, d, n) => new MenuItem(i, d, n));
            mnuTamanhoReal = Resolve(list, "mnuTamanhoReal", "MenuItem", (i, d, n) => new MenuItem(i, d, n));
            divTopo = Resolve(list, "divTopo", "Divider", (i, d, n) => new Divider(i, d, n));
            btnSalvar = Resolve(list, "btnSalvar", "Button", (i, d, n) => new Button(i, d, n));
            btnDesfazer = Resolve(list, "btnDesfazer", "Button", (i, d, n) => new Button(i, d, n));
            btnRefazer = Resolve(list, "btnRefazer", "Button", (i, d, n) => new Button(i, d, n));
            pnlFaixa = Resolve(list, "pnlFaixa", "Panel", (i, d, n) => new Panel(i, d, n));
            lytSelecao = Resolve(list, "lytSelecao", "Layout", (i, d, n) => new Layout(i, d, n));
            lblSelecaoGrupo = Resolve(list, "lblSelecaoGrupo", "Label", (i, d, n) => new Label(i, d, n));
            btnSelecionar = Resolve(list, "btnSelecionar", "Button", (i, d, n) => new Button(i, d, n));
            divSelecao = Resolve(list, "divSelecao", "Divider", (i, d, n) => new Divider(i, d, n));
            lytFerramentas = Resolve(list, "lytFerramentas", "Layout", (i, d, n) => new Layout(i, d, n));
            lblFerramentasGrupo = Resolve(list, "lblFerramentasGrupo", "Label", (i, d, n) => new Label(i, d, n));
            grdFerramentas = Resolve(list, "grdFerramentas", "GridLayout", (i, d, n) => new GridLayout(i, d, n));
            btnLapis = Resolve(list, "btnLapis", "Button", (i, d, n) => new Button(i, d, n));
            btnBalde = Resolve(list, "btnBalde", "Button", (i, d, n) => new Button(i, d, n));
            btnBorracha = Resolve(list, "btnBorracha", "Button", (i, d, n) => new Button(i, d, n));
            btnContaGotas = Resolve(list, "btnContaGotas", "Button", (i, d, n) => new Button(i, d, n));
            divFerramentas = Resolve(list, "divFerramentas", "Divider", (i, d, n) => new Divider(i, d, n));
            lytPinceis = Resolve(list, "lytPinceis", "Layout", (i, d, n) => new Layout(i, d, n));
            lblPinceis = Resolve(list, "lblPinceis", "Label", (i, d, n) => new Label(i, d, n));
            btnPincel = Resolve(list, "btnPincel", "Button", (i, d, n) => new Button(i, d, n));
            divPinceis = Resolve(list, "divPinceis", "Divider", (i, d, n) => new Divider(i, d, n));
            lytFormas = Resolve(list, "lytFormas", "Layout", (i, d, n) => new Layout(i, d, n));
            lblFormas = Resolve(list, "lblFormas", "Label", (i, d, n) => new Label(i, d, n));
            chkPreencher = Resolve(list, "chkPreencher", "CheckBox", (i, d, n) => new CheckBox(i, d, n));
            grdFormas = Resolve(list, "grdFormas", "GridLayout", (i, d, n) => new GridLayout(i, d, n));
            btnLinha = Resolve(list, "btnLinha", "Button", (i, d, n) => new Button(i, d, n));
            btnRetangulo = Resolve(list, "btnRetangulo", "Button", (i, d, n) => new Button(i, d, n));
            btnElipse = Resolve(list, "btnElipse", "Button", (i, d, n) => new Button(i, d, n));
            divFormas = Resolve(list, "divFormas", "Divider", (i, d, n) => new Divider(i, d, n));
            lytTamanho = Resolve(list, "lytTamanho", "Layout", (i, d, n) => new Layout(i, d, n));
            lblTamanhoGrupo = Resolve(list, "lblTamanhoGrupo", "Label", (i, d, n) => new Label(i, d, n));
            cbEspessura = Resolve(list, "cbEspessura", "ComboBox", (i, d, n) => new ComboBox(i, d, n));
            divTamanho = Resolve(list, "divTamanho", "Divider", (i, d, n) => new Divider(i, d, n));
            lytCores = Resolve(list, "lytCores", "Layout", (i, d, n) => new Layout(i, d, n));
            lblCores = Resolve(list, "lblCores", "Label", (i, d, n) => new Label(i, d, n));
            btnCor1 = Resolve(list, "btnCor1", "Circle", (i, d, n) => new Circle(i, d, n));
            btnCor2 = Resolve(list, "btnCor2", "Circle", (i, d, n) => new Circle(i, d, n));
            grdPaleta = Resolve(list, "grdPaleta", "GridLayout", (i, d, n) => new GridLayout(i, d, n));
            btnPaleta01 = Resolve(list, "btnPaleta01", "Circle", (i, d, n) => new Circle(i, d, n));
            btnPaleta02 = Resolve(list, "btnPaleta02", "Circle", (i, d, n) => new Circle(i, d, n));
            btnPaleta03 = Resolve(list, "btnPaleta03", "Circle", (i, d, n) => new Circle(i, d, n));
            btnPaleta04 = Resolve(list, "btnPaleta04", "Circle", (i, d, n) => new Circle(i, d, n));
            btnPaleta05 = Resolve(list, "btnPaleta05", "Circle", (i, d, n) => new Circle(i, d, n));
            btnPaleta06 = Resolve(list, "btnPaleta06", "Circle", (i, d, n) => new Circle(i, d, n));
            btnPaleta07 = Resolve(list, "btnPaleta07", "Circle", (i, d, n) => new Circle(i, d, n));
            btnPaleta08 = Resolve(list, "btnPaleta08", "Circle", (i, d, n) => new Circle(i, d, n));
            btnPaleta09 = Resolve(list, "btnPaleta09", "Circle", (i, d, n) => new Circle(i, d, n));
            btnPaleta10 = Resolve(list, "btnPaleta10", "Circle", (i, d, n) => new Circle(i, d, n));
            btnPaleta11 = Resolve(list, "btnPaleta11", "Circle", (i, d, n) => new Circle(i, d, n));
            btnPaleta12 = Resolve(list, "btnPaleta12", "Circle", (i, d, n) => new Circle(i, d, n));
            btnPaleta13 = Resolve(list, "btnPaleta13", "Circle", (i, d, n) => new Circle(i, d, n));
            btnPaleta14 = Resolve(list, "btnPaleta14", "Circle", (i, d, n) => new Circle(i, d, n));
            btnPaleta15 = Resolve(list, "btnPaleta15", "Circle", (i, d, n) => new Circle(i, d, n));
            btnPaleta16 = Resolve(list, "btnPaleta16", "Circle", (i, d, n) => new Circle(i, d, n));
            btnPaleta17 = Resolve(list, "btnPaleta17", "Circle", (i, d, n) => new Circle(i, d, n));
            btnPaleta18 = Resolve(list, "btnPaleta18", "Circle", (i, d, n) => new Circle(i, d, n));
            btnPaleta19 = Resolve(list, "btnPaleta19", "Circle", (i, d, n) => new Circle(i, d, n));
            btnPaleta20 = Resolve(list, "btnPaleta20", "Circle", (i, d, n) => new Circle(i, d, n));
            stbStatus = Resolve(list, "stbStatus", "StatusBar", (i, d, n) => new StatusBar(i, d, n));
            lblPosicao = Resolve(list, "lblPosicao", "Label", (i, d, n) => new Label(i, d, n));
            lblTamanho = Resolve(list, "lblTamanho", "Label", (i, d, n) => new Label(i, d, n));
            lblZoom = Resolve(list, "lblZoom", "Label", (i, d, n) => new Label(i, d, n));
            sbxArea = Resolve(list, "sbxArea", "ScrollBox", (i, d, n) => new ScrollBox(i, d, n));
            pcDesenho = Resolve(list, "pcDesenho", "PaintCanvas", (i, d, n) => new PaintCanvas(i, d, n));
            dlgAbrir = Resolve(list, "dlgAbrir", "OpenDialog", (i, d, n) => new OpenDialog(i, d, n));
            dlgSalvar = Resolve(list, "dlgSalvar", "SaveDialog", (i, d, n) => new SaveDialog(i, d, n));
        }

        /// <summary>O Button deste form com o nome (como o FindComponent).</summary>
        public Button? FindButton(string name) => name switch
        {
            "btnSalvar" => btnSalvar,
            "btnDesfazer" => btnDesfazer,
            "btnRefazer" => btnRefazer,
            "btnSelecionar" => btnSelecionar,
            "btnLapis" => btnLapis,
            "btnBalde" => btnBalde,
            "btnBorracha" => btnBorracha,
            "btnContaGotas" => btnContaGotas,
            "btnPincel" => btnPincel,
            "btnLinha" => btnLinha,
            "btnRetangulo" => btnRetangulo,
            "btnElipse" => btnElipse,
            _ => null,
        };

        /// <summary>O Button que disparou o evento.</summary>
        public Button? SenderAsButton(AylisClient.AylisEvent ev) => FindButton(ev.SenderName);

        /// <summary>O CheckBox deste form com o nome (como o FindComponent).</summary>
        public CheckBox? FindCheckBox(string name) => name switch
        {
            "chkPreencher" => chkPreencher,
            _ => null,
        };

        /// <summary>O CheckBox que disparou o evento.</summary>
        public CheckBox? SenderAsCheckBox(AylisClient.AylisEvent ev) => FindCheckBox(ev.SenderName);

        /// <summary>O Circle deste form com o nome (como o FindComponent).</summary>
        public Circle? FindCircle(string name) => name switch
        {
            "btnCor1" => btnCor1,
            "btnCor2" => btnCor2,
            "btnPaleta01" => btnPaleta01,
            "btnPaleta02" => btnPaleta02,
            "btnPaleta03" => btnPaleta03,
            "btnPaleta04" => btnPaleta04,
            "btnPaleta05" => btnPaleta05,
            "btnPaleta06" => btnPaleta06,
            "btnPaleta07" => btnPaleta07,
            "btnPaleta08" => btnPaleta08,
            "btnPaleta09" => btnPaleta09,
            "btnPaleta10" => btnPaleta10,
            "btnPaleta11" => btnPaleta11,
            "btnPaleta12" => btnPaleta12,
            "btnPaleta13" => btnPaleta13,
            "btnPaleta14" => btnPaleta14,
            "btnPaleta15" => btnPaleta15,
            "btnPaleta16" => btnPaleta16,
            "btnPaleta17" => btnPaleta17,
            "btnPaleta18" => btnPaleta18,
            "btnPaleta19" => btnPaleta19,
            "btnPaleta20" => btnPaleta20,
            _ => null,
        };

        /// <summary>O Circle que disparou o evento.</summary>
        public Circle? SenderAsCircle(AylisClient.AylisEvent ev) => FindCircle(ev.SenderName);

        /// <summary>O ComboBox deste form com o nome (como o FindComponent).</summary>
        public ComboBox? FindComboBox(string name) => name switch
        {
            "cbEspessura" => cbEspessura,
            _ => null,
        };

        /// <summary>O ComboBox que disparou o evento.</summary>
        public ComboBox? SenderAsComboBox(AylisClient.AylisEvent ev) => FindComboBox(ev.SenderName);

        /// <summary>O Divider deste form com o nome (como o FindComponent).</summary>
        public Divider? FindDivider(string name) => name switch
        {
            "divTopo" => divTopo,
            "divSelecao" => divSelecao,
            "divFerramentas" => divFerramentas,
            "divPinceis" => divPinceis,
            "divFormas" => divFormas,
            "divTamanho" => divTamanho,
            _ => null,
        };

        /// <summary>O Divider que disparou o evento.</summary>
        public Divider? SenderAsDivider(AylisClient.AylisEvent ev) => FindDivider(ev.SenderName);

        /// <summary>O GridLayout deste form com o nome (como o FindComponent).</summary>
        public GridLayout? FindGridLayout(string name) => name switch
        {
            "grdFerramentas" => grdFerramentas,
            "grdFormas" => grdFormas,
            "grdPaleta" => grdPaleta,
            _ => null,
        };

        /// <summary>O GridLayout que disparou o evento.</summary>
        public GridLayout? SenderAsGridLayout(AylisClient.AylisEvent ev) => FindGridLayout(ev.SenderName);

        /// <summary>O Label deste form com o nome (como o FindComponent).</summary>
        public Label? FindLabel(string name) => name switch
        {
            "lblSelecaoGrupo" => lblSelecaoGrupo,
            "lblFerramentasGrupo" => lblFerramentasGrupo,
            "lblPinceis" => lblPinceis,
            "lblFormas" => lblFormas,
            "lblTamanhoGrupo" => lblTamanhoGrupo,
            "lblCores" => lblCores,
            "lblPosicao" => lblPosicao,
            "lblTamanho" => lblTamanho,
            "lblZoom" => lblZoom,
            _ => null,
        };

        /// <summary>O Label que disparou o evento.</summary>
        public Label? SenderAsLabel(AylisClient.AylisEvent ev) => FindLabel(ev.SenderName);

        /// <summary>O Layout deste form com o nome (como o FindComponent).</summary>
        public Layout? FindLayout(string name) => name switch
        {
            "lytTopo" => lytTopo,
            "lytSelecao" => lytSelecao,
            "lytFerramentas" => lytFerramentas,
            "lytPinceis" => lytPinceis,
            "lytFormas" => lytFormas,
            "lytTamanho" => lytTamanho,
            "lytCores" => lytCores,
            _ => null,
        };

        /// <summary>O Layout que disparou o evento.</summary>
        public Layout? SenderAsLayout(AylisClient.AylisEvent ev) => FindLayout(ev.SenderName);

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
            "mnuNovo" => mnuNovo,
            "mnuAbrir" => mnuAbrir,
            "mnuSalvar" => mnuSalvar,
            "mnuSalvarComo" => mnuSalvarComo,
            "mnuSep1" => mnuSep1,
            "mnuSair" => mnuSair,
            "mnuEditar" => mnuEditar,
            "mnuDesfazer" => mnuDesfazer,
            "mnuRefazer" => mnuRefazer,
            "mnuSep2" => mnuSep2,
            "mnuRecortar" => mnuRecortar,
            "mnuCopiar" => mnuCopiar,
            "mnuColar" => mnuColar,
            "mnuExcluir" => mnuExcluir,
            "mnuSelecionarTudo" => mnuSelecionarTudo,
            "mnuSep3" => mnuSep3,
            "mnuLimpar" => mnuLimpar,
            "mnuExibir" => mnuExibir,
            "mnuAmpliar" => mnuAmpliar,
            "mnuReduzir" => mnuReduzir,
            "mnuTamanhoReal" => mnuTamanhoReal,
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

        /// <summary>O PaintCanvas deste form com o nome (como o FindComponent).</summary>
        public PaintCanvas? FindPaintCanvas(string name) => name switch
        {
            "pcDesenho" => pcDesenho,
            _ => null,
        };

        /// <summary>O PaintCanvas que disparou o evento.</summary>
        public PaintCanvas? SenderAsPaintCanvas(AylisClient.AylisEvent ev) => FindPaintCanvas(ev.SenderName);

        /// <summary>O Panel deste form com o nome (como o FindComponent).</summary>
        public Panel? FindPanel(string name) => name switch
        {
            "pnlFaixa" => pnlFaixa,
            _ => null,
        };

        /// <summary>O Panel que disparou o evento.</summary>
        public Panel? SenderAsPanel(AylisClient.AylisEvent ev) => FindPanel(ev.SenderName);

        /// <summary>O SaveDialog deste form com o nome (como o FindComponent).</summary>
        public SaveDialog? FindSaveDialog(string name) => name switch
        {
            "dlgSalvar" => dlgSalvar,
            _ => null,
        };

        /// <summary>O SaveDialog que disparou o evento.</summary>
        public SaveDialog? SenderAsSaveDialog(AylisClient.AylisEvent ev) => FindSaveDialog(ev.SenderName);

        /// <summary>O ScrollBox deste form com o nome (como o FindComponent).</summary>
        public ScrollBox? FindScrollBox(string name) => name switch
        {
            "sbxArea" => sbxArea,
            _ => null,
        };

        /// <summary>O ScrollBox que disparou o evento.</summary>
        public ScrollBox? SenderAsScrollBox(AylisClient.AylisEvent ev) => FindScrollBox(ev.SenderName);

        /// <summary>O StatusBar deste form com o nome (como o FindComponent).</summary>
        public StatusBar? FindStatusBar(string name) => name switch
        {
            "stbStatus" => stbStatus,
            _ => null,
        };

        /// <summary>O StatusBar que disparou o evento.</summary>
        public StatusBar? SenderAsStatusBar(AylisClient.AylisEvent ev) => FindStatusBar(ev.SenderName);
    }
}
