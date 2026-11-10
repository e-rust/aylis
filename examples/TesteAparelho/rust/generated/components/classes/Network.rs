
/// Um Network de um form.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Network {
    pub id: ComponentId,
}

impl Handle for Network {
    fn id(&self) -> ComponentId {
        self.id
    }
}

impl Network {
    pub fn interval(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "interval").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_interval(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "interval", value)
    }
    pub fn connected(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "connected").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_connected(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "connected", value)
    }
    pub fn kind(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "kind"))
    }
    pub fn set_kind(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "kind", Into::<String>::into(value))
    }
    pub fn available(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "available").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_available(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "available", value)
    }
}
