
/// Um Timer de um form.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Timer {
    pub id: ComponentId,
}

impl Handle for Timer {
    fn id(&self) -> ComponentId {
        self.id
    }
}

impl Timer {
    pub fn interval(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "interval").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_interval(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "interval", value)
    }
    pub fn enabled(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "enabled").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_enabled(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "enabled", value)
    }
    pub fn precision(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "precision"))
    }
    pub fn set_precision(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "precision", Into::<String>::into(value))
    }
    pub fn background(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "background").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_background(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "background", value)
    }
    pub fn one_shot(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "one_shot").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_one_shot(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "one_shot", value)
    }
    pub fn drift(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "drift").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_drift(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "drift", value)
    }
    pub fn restart(self, ui: &mut Ui) -> Result<(), String> {
        handles::try_call(ui, self.id, "restart", &[]).map(|_| ())
    }
}
