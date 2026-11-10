
/// Um ClientDataSet de um form.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ClientDataSet {
    pub id: ComponentId,
}

impl Handle for ClientDataSet {
    fn id(&self) -> ComponentId {
        self.id
    }
}

impl ClientDataSet {
    pub fn key_fields(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "key_fields"))
    }
    pub fn set_key_fields(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "key_fields", Into::<String>::into(value))
    }
    pub fn apply_mode(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "apply_mode"))
    }
    pub fn set_apply_mode(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "apply_mode", Into::<String>::into(value))
    }
    pub fn read_only(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "read_only").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_read_only(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "read_only", value)
    }
    pub fn active(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "active").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_active(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "active", value)
    }
    pub fn filter(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "filter"))
    }
    pub fn set_filter(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "filter", Into::<String>::into(value))
    }
    pub fn filtered(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "filtered").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_filtered(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "filtered", value)
    }
    pub fn index_field_names(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "index_field_names"))
    }
    pub fn set_index_field_names(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "index_field_names", Into::<String>::into(value))
    }
    pub fn indexes(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "indexes"))
    }
    pub fn set_indexes(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "indexes", Into::<String>::into(value))
    }
    pub fn index_name(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "index_name"))
    }
    pub fn set_index_name(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "index_name", Into::<String>::into(value))
    }
    pub fn params(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "params"))
    }
    pub fn set_params(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "params", Into::<String>::into(value))
    }
    pub fn status_filter(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "status_filter"))
    }
    pub fn set_status_filter(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "status_filter", Into::<String>::into(value))
    }
    pub fn master_source(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "master_source"))
    }
    pub fn set_master_source(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "master_source", Into::<String>::into(value))
    }
    pub fn master_fields(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "master_fields"))
    }
    pub fn set_master_fields(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "master_fields", Into::<String>::into(value))
    }
    pub fn apply_separately(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "apply_separately").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_apply_separately(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "apply_separately", value)
    }
    pub fn record_total(self, ui: &Ui) -> i64 {
        handles::read(ui, self.id, "record_total").and_then(|v| v.to_i64().ok()).unwrap_or_default()
    }
    pub fn set_record_total(self, ui: &mut Ui, value: i64) {
        handles::write(ui, self.id, "record_total", value)
    }
    pub fn max_records(self, ui: &Ui) -> i64 {
        handles::read(ui, self.id, "max_records").and_then(|v| v.to_i64().ok()).unwrap_or_default()
    }
    pub fn set_max_records(self, ui: &mut Ui, value: i64) {
        handles::write(ui, self.id, "max_records", value)
    }
    pub fn partial_notice(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "partial_notice"))
    }
}
