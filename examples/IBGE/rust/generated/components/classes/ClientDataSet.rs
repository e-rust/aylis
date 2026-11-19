
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

/// Quem disparou um handler tipado (`sender: ClientDataSet`): só um ClientDataSet.
impl aylis::prelude::FromSender for ClientDataSet {
    fn from_sender(ui: &Ui, id: ComponentId) -> Option<Self> {
        (ui.class_name(id) == Some("ClientDataSet")).then_some(Self { id })
    }
}

impl ClientDataSet {
    pub fn key_fields(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "key_fields"))
    }
    pub fn set_key_fields(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "key_fields", Into::<String>::into(value))
    }
    pub fn apply_mode(self, ui: &Ui) -> aylis::prelude::ApplyMode {
        handles::read_enum::<aylis::prelude::ApplyMode>(ui, self.id, "apply_mode")
    }
    pub fn set_apply_mode(self, ui: &mut Ui, value: aylis::prelude::ApplyMode) {
        handles::write(ui, self.id, "apply_mode", PropValue::enum_value(value))
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
    pub fn indexes(self, ui: &Ui) -> Vec<IndexDef> {
        IndexDef::list_from_text(&handles::text(ui, handles::read(ui, self.id, "indexes")))
    }
    pub fn set_indexes(self, ui: &mut Ui, value: &[IndexDef]) {
        handles::write(ui, self.id, "indexes", PropValue::String(IndexDef::list_text(value)))
    }
    pub fn index_name(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "index_name"))
    }
    pub fn set_index_name(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "index_name", Into::<String>::into(value))
    }
    pub fn params(self, ui: &Ui) -> Vec<FilterParam> {
        FilterParam::list_from_text(&handles::text(ui, handles::read(ui, self.id, "params")))
    }
    pub fn set_params(self, ui: &mut Ui, value: &[FilterParam]) {
        handles::write(ui, self.id, "params", PropValue::String(FilterParam::list_text(value)))
    }
    pub fn status_filter(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "status_filter"))
    }
    pub fn set_status_filter(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "status_filter", Into::<String>::into(value))
    }
    pub fn master_source(self, ui: &Ui) -> Option<ComponentId> {
        handles::read_ref(ui, self.id, "master_source")
    }
    pub fn set_master_source(self, ui: &mut Ui, value: impl Into<handles::Ref>) {
        handles::write(ui, self.id, "master_source", Into::<handles::Ref>::into(value))
    }
    pub fn master_fields(self, ui: &Ui) -> Vec<MasterField> {
        MasterField::list_from_text(&handles::text(ui, handles::read(ui, self.id, "master_fields")))
    }
    pub fn set_master_fields(self, ui: &mut Ui, value: &[MasterField]) {
        handles::write(ui, self.id, "master_fields", PropValue::String(MasterField::list_text(value)))
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
    pub fn on_master_change(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "on_master_change"))
    }
    pub fn set_on_master_change(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "on_master_change", Into::<String>::into(value))
    }
    pub fn after_scroll(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "after_scroll"))
    }
    pub fn set_after_scroll(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "after_scroll", Into::<String>::into(value))
    }
    pub fn after_post(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "after_post"))
    }
    pub fn set_after_post(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "after_post", Into::<String>::into(value))
    }
    pub fn on_state_change(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "on_state_change"))
    }
    pub fn set_on_state_change(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "on_state_change", Into::<String>::into(value))
    }
    pub fn on_reconcile_error(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "on_reconcile_error"))
    }
    pub fn set_on_reconcile_error(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "on_reconcile_error", Into::<String>::into(value))
    }
    pub fn on_error(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "on_error"))
    }
    pub fn set_on_error(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "on_error", Into::<String>::into(value))
    }
    pub fn on_fetch(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "on_fetch"))
    }
    pub fn set_on_fetch(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "on_fetch", Into::<String>::into(value))
    }
    pub fn on_server_query(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "on_server_query"))
    }
    pub fn set_on_server_query(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "on_server_query", Into::<String>::into(value))
    }
}
