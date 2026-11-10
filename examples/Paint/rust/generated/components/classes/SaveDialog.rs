
/// Um SaveDialog de um form.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SaveDialog {
    pub id: ComponentId,
}

impl Handle for SaveDialog {
    fn id(&self) -> ComponentId {
        self.id
    }
}

impl SaveDialog {
    pub fn title(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "title"))
    }
    pub fn set_title(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "title", Into::<String>::into(value))
    }
    pub fn filter(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "filter"))
    }
    pub fn set_filter(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "filter", Into::<String>::into(value))
    }
    pub fn initial_dir(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "initial_dir"))
    }
    pub fn set_initial_dir(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "initial_dir", Into::<String>::into(value))
    }
    pub fn file_name(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "file_name"))
    }
    pub fn set_file_name(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "file_name", Into::<String>::into(value))
    }
    pub fn multiple(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "multiple").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_multiple(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "multiple", value)
    }
    pub fn execute(self, ui: &mut Ui) {
        aylis::prelude::SaveDialog::execute(ui, self.id);
    }
}
