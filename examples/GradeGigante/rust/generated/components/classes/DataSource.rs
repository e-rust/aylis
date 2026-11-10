
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

impl DataSource {
    pub fn data_set(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "data_set"))
    }
    pub fn set_data_set(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "data_set", Into::<String>::into(value))
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
}
