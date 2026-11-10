
/// Um BackgroundService de um form.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BackgroundService {
    pub id: ComponentId,
}

impl Handle for BackgroundService {
    fn id(&self) -> ComponentId {
        self.id
    }
}

impl BackgroundService {
    pub fn active(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "active").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_active(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "active", value)
    }
    pub fn title(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "title"))
    }
    pub fn set_title(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "title", Into::<String>::into(value))
    }
    pub fn text(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "text"))
    }
    pub fn set_text(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "text", Into::<String>::into(value))
    }
    pub fn kind(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "kind"))
    }
    pub fn set_kind(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "kind", Into::<String>::into(value))
    }
    pub fn keep_alive(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "keep_alive").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_keep_alive(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "keep_alive", value)
    }
    pub fn send(self, ui: &mut Ui, a1: &str) -> Result<(), String> {
        handles::try_call(ui, self.id, "send", &[PropValue::String(a1.to_string())]).map(|_| ())
    }
    pub fn running(self, ui: &mut Ui) -> Result<bool, String> {
        handles::try_call(ui, self.id, "running", &[]).map(|v| v.to_bool().unwrap_or_default())
    }
}
