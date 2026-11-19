
/// Um DataSource de um form.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DataSource {
    pub id: ComponentId,
}

impl Handle for DataSource {
    fn id(&self) -> ComponentId {
        self.id
    }
}

/// Quem disparou um handler tipado (`sender: DataSource`): só um DataSource.
impl aylis::prelude::FromSender for DataSource {
    fn from_sender(ui: &Ui, id: ComponentId) -> Option<Self> {
        (ui.class_name(id) == Some("DataSource")).then_some(Self { id })
    }
}

impl DataSource {
    pub fn data_set(self, ui: &Ui) -> Option<ComponentId> {
        handles::read_ref(ui, self.id, "data_set")
    }
    pub fn set_data_set(self, ui: &mut Ui, value: impl Into<handles::Ref>) {
        handles::write(ui, self.id, "data_set", Into::<handles::Ref>::into(value))
    }
    pub fn auto_edit(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "auto_edit").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_auto_edit(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "auto_edit", value)
    }
    pub fn enabled(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "enabled").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_enabled(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "enabled", value)
    }
    pub fn on_data_change(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "on_data_change"))
    }
    pub fn set_on_data_change(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "on_data_change", Into::<String>::into(value))
    }
    pub fn on_state_change(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "on_state_change"))
    }
    pub fn set_on_state_change(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "on_state_change", Into::<String>::into(value))
    }
}
