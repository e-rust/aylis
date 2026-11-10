
/// Um DBChart de um form.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DBChart {
    pub id: ComponentId,
}

impl Handle for DBChart {
    fn id(&self) -> ComponentId {
        self.id
    }
}

impl DBChart {
    pub fn data_source(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "data_source"))
    }
    pub fn set_data_source(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "data_source", Into::<String>::into(value))
    }
    pub fn label_field(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "label_field"))
    }
    pub fn set_label_field(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "label_field", Into::<String>::into(value))
    }
    pub fn value_fields(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "value_fields"))
    }
    pub fn set_value_fields(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "value_fields", Into::<String>::into(value))
    }
    pub fn kind(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "kind"))
    }
    pub fn set_kind(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "kind", Into::<String>::into(value))
    }
    pub fn title(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "title"))
    }
    pub fn set_title(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "title", Into::<String>::into(value))
    }
    pub fn labels(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "labels"))
    }
    pub fn set_labels(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "labels", Into::<String>::into(value))
    }
    pub fn series(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "series"))
    }
    pub fn set_series(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "series", Into::<String>::into(value))
    }
    pub fn json(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "json"))
    }
    pub fn set_json(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "json", Into::<String>::into(value))
    }
    pub fn legend(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "legend").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_legend(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "legend", value)
    }
    pub fn stepped(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "stepped").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_stepped(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "stepped", value)
    }
    pub fn tension(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "tension").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_tension(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "tension", value)
    }
    pub fn animate(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "animate").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_animate(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "animate", value)
    }
    pub fn zoom(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "zoom").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_zoom(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "zoom", value)
    }
    pub fn auto_scale(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "auto_scale").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_auto_scale(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "auto_scale", value)
    }
    pub fn y_min(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "y_min").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_y_min(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "y_min", value)
    }
    pub fn y_max(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "y_max").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_y_max(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "y_max", value)
    }
    pub fn log_scale(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "log_scale").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_log_scale(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "log_scale", value)
    }
    pub fn reverse(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "reverse").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_reverse(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "reverse", value)
    }
    pub fn time_axis(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "time_axis").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_time_axis(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "time_axis", value)
    }
    pub fn clicked_series(self, ui: &Ui) -> i64 {
        handles::read(ui, self.id, "clicked_series").and_then(|v| v.to_i64().ok()).unwrap_or_default()
    }
    pub fn set_clicked_series(self, ui: &mut Ui, value: i64) {
        handles::write(ui, self.id, "clicked_series", value)
    }
    pub fn clicked_index(self, ui: &Ui) -> i64 {
        handles::read(ui, self.id, "clicked_index").and_then(|v| v.to_i64().ok()).unwrap_or_default()
    }
    pub fn set_clicked_index(self, ui: &mut Ui, value: i64) {
        handles::write(ui, self.id, "clicked_index", value)
    }
    pub fn left(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "left").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_left(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "left", value)
    }
    pub fn top(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "top").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_top(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "top", value)
    }
    pub fn width(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "width").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_width(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "width", value)
    }
    pub fn height(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "height").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_height(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "height", value)
    }
    pub fn align(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "align"))
    }
    pub fn set_align(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "align", Into::<String>::into(value))
    }
    pub fn visible(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "visible").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_visible(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "visible", value)
    }
    pub fn enabled(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "enabled").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_enabled(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "enabled", value)
    }
    pub fn hit_test(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "hit_test").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_hit_test(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "hit_test", value)
    }
    pub fn clip_children(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "clip_children").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_clip_children(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "clip_children", value)
    }
    pub fn captures_wheel(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "captures_wheel").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_captures_wheel(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "captures_wheel", value)
    }
    pub fn can_focus(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "can_focus").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_can_focus(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "can_focus", value)
    }
    pub fn draggable(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "draggable").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_draggable(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "draggable", value)
    }
    pub fn accepts_drops(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "accepts_drops").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_accepts_drops(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "accepts_drops", value)
    }
    pub fn drag_data(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "drag_data"))
    }
    pub fn set_drag_data(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "drag_data", Into::<String>::into(value))
    }
    pub fn drag_label(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "drag_label"))
    }
    pub fn set_drag_label(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "drag_label", Into::<String>::into(value))
    }
    pub fn drag_image(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "drag_image"))
    }
    pub fn set_drag_image(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "drag_image", Into::<String>::into(value))
    }
    pub fn drop_types(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "drop_types"))
    }
    pub fn set_drop_types(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "drop_types", Into::<String>::into(value))
    }
    pub fn tab_stop(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "tab_stop").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_tab_stop(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "tab_stop", value)
    }
    pub fn cursor(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "cursor"))
    }
    pub fn set_cursor(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "cursor", Into::<String>::into(value))
    }
    pub fn opacity(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "opacity").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_opacity(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "opacity", value)
    }
    pub fn cache_layer(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "cache_layer").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_cache_layer(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "cache_layer", value)
    }
    pub fn rotation(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "rotation").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_rotation(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "rotation", value)
    }
    pub fn translate_x(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "translate_x").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_translate_x(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "translate_x", value)
    }
    pub fn translate_y(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "translate_y").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_translate_y(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "translate_y", value)
    }
    pub fn aspect_ratio(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "aspect_ratio").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_aspect_ratio(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "aspect_ratio", value)
    }
    pub fn max_width(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "max_width").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_max_width(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "max_width", value)
    }
    pub fn min_width(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "min_width").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_min_width(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "min_width", value)
    }
    pub fn min_height(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "min_height").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_min_height(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "min_height", value)
    }
    pub fn max_height(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "max_height").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_max_height(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "max_height", value)
    }
    pub fn self_align(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "self_align"))
    }
    pub fn set_self_align(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "self_align", Into::<String>::into(value))
    }
    pub fn auto_size(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "auto_size").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_auto_size(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "auto_size", value)
    }
    pub fn column_span(self, ui: &Ui) -> i64 {
        handles::read(ui, self.id, "column_span").and_then(|v| v.to_i64().ok()).unwrap_or_default()
    }
    pub fn set_column_span(self, ui: &mut Ui, value: i64) {
        handles::write(ui, self.id, "column_span", value)
    }
    pub fn shadow(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "shadow"))
    }
    pub fn set_shadow(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "shadow", Into::<String>::into(value))
    }
    pub fn style(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "style"))
    }
    pub fn set_style(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "style", Into::<String>::into(value))
    }
    pub fn theme(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "theme"))
    }
    pub fn set_theme(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "theme", Into::<String>::into(value))
    }
    pub fn fill(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "fill"))
    }
    pub fn set_fill(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "fill", Into::<String>::into(value))
    }
    pub fn border_color(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "border_color"))
    }
    pub fn set_border_color(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "border_color", Into::<String>::into(value))
    }
    pub fn text_color(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "text_color"))
    }
    pub fn set_text_color(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "text_color", Into::<String>::into(value))
    }
    pub fn accent_color(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "accent_color"))
    }
    pub fn set_accent_color(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "accent_color", Into::<String>::into(value))
    }
    pub fn success_color(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "success_color"))
    }
    pub fn set_success_color(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "success_color", Into::<String>::into(value))
    }
    pub fn warning_color(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "warning_color"))
    }
    pub fn set_warning_color(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "warning_color", Into::<String>::into(value))
    }
    pub fn danger_color(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "danger_color"))
    }
    pub fn set_danger_color(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "danger_color", Into::<String>::into(value))
    }
    pub fn font_family(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "font_family"))
    }
    pub fn set_font_family(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "font_family", Into::<String>::into(value))
    }
    pub fn font_size(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "font_size").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_font_size(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "font_size", value)
    }
    pub fn font_weight(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "font_weight"))
    }
    pub fn set_font_weight(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "font_weight", Into::<String>::into(value))
    }
    pub fn font_italic(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "font_italic").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_font_italic(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "font_italic", value)
    }
    pub fn font_underline(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "font_underline").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_font_underline(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "font_underline", value)
    }
    pub fn font_strikeout(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "font_strikeout").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_font_strikeout(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "font_strikeout", value)
    }
    pub fn hint(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "hint"))
    }
    pub fn set_hint(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "hint", Into::<String>::into(value))
    }
    pub fn tag(self, ui: &Ui) -> i64 {
        handles::read(ui, self.id, "tag").and_then(|v| v.to_i64().ok()).unwrap_or_default()
    }
    pub fn set_tag(self, ui: &mut Ui, value: i64) {
        handles::write(ui, self.id, "tag", value)
    }
    pub fn popup_menu(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "popup_menu"))
    }
    pub fn set_popup_menu(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "popup_menu", Into::<String>::into(value))
    }
}
