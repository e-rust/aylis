
/// Um Field de um form.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Field {
    pub id: ComponentId,
}

impl Handle for Field {
    fn id(&self) -> ComponentId {
        self.id
    }
}

impl Field {
    pub fn field_name(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "field_name"))
    }
    pub fn set_field_name(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "field_name", Into::<String>::into(value))
    }
    pub fn r#type(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "type"))
    }
    pub fn set_type(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "type", Into::<String>::into(value))
    }
    pub fn size(self, ui: &Ui) -> i64 {
        handles::read(ui, self.id, "size").and_then(|v| v.to_i64().ok()).unwrap_or_default()
    }
    pub fn set_size(self, ui: &mut Ui, value: i64) {
        handles::write(ui, self.id, "size", value)
    }
    pub fn scale(self, ui: &Ui) -> i64 {
        handles::read(ui, self.id, "scale").and_then(|v| v.to_i64().ok()).unwrap_or_default()
    }
    pub fn set_scale(self, ui: &mut Ui, value: i64) {
        handles::write(ui, self.id, "scale", value)
    }
    pub fn rounding(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "rounding"))
    }
    pub fn set_rounding(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "rounding", Into::<String>::into(value))
    }
    pub fn display_label(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "display_label"))
    }
    pub fn set_display_label(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "display_label", Into::<String>::into(value))
    }
    pub fn display_width(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "display_width").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_display_width(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "display_width", value)
    }
    pub fn display_format(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "display_format"))
    }
    pub fn set_display_format(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "display_format", Into::<String>::into(value))
    }
    pub fn edit_format(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "edit_format"))
    }
    pub fn set_edit_format(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "edit_format", Into::<String>::into(value))
    }
    pub fn alignment(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "alignment"))
    }
    pub fn set_alignment(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "alignment", Into::<String>::into(value))
    }
    pub fn required(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "required").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_required(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "required", value)
    }
    pub fn read_only(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "read_only").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_read_only(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "read_only", value)
    }
    pub fn visible(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "visible").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_visible(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "visible", value)
    }
    pub fn min(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "min").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_min(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "min", value)
    }
    pub fn max(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "max").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_max(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "max", value)
    }
    pub fn default_value(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "default_value"))
    }
    pub fn set_default_value(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "default_value", Into::<String>::into(value))
    }
    pub fn display_values(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "display_values"))
    }
    pub fn set_display_values(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "display_values", Into::<String>::into(value))
    }
    pub fn char_case(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "char_case"))
    }
    pub fn set_char_case(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "char_case", Into::<String>::into(value))
    }
    pub fn edit_mask(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "edit_mask"))
    }
    pub fn set_edit_mask(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "edit_mask", Into::<String>::into(value))
    }
    pub fn sensitive(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "sensitive").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_sensitive(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "sensitive", value)
    }
    pub fn kind(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "kind"))
    }
    pub fn set_kind(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "kind", Into::<String>::into(value))
    }
    pub fn expression(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "expression"))
    }
    pub fn set_expression(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "expression", Into::<String>::into(value))
    }
    pub fn lookup_data_set(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "lookup_data_set"))
    }
    pub fn set_lookup_data_set(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "lookup_data_set", Into::<String>::into(value))
    }
    pub fn key_fields(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "key_fields"))
    }
    pub fn set_key_fields(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "key_fields", Into::<String>::into(value))
    }
    pub fn lookup_key_fields(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "lookup_key_fields"))
    }
    pub fn set_lookup_key_fields(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "lookup_key_fields", Into::<String>::into(value))
    }
    pub fn lookup_result_field(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "lookup_result_field"))
    }
    pub fn set_lookup_result_field(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "lookup_result_field", Into::<String>::into(value))
    }
}
