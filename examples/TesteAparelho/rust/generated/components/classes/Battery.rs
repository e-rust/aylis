
/// Um Battery de um form.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Battery {
    pub id: ComponentId,
}

impl Handle for Battery {
    fn id(&self) -> ComponentId {
        self.id
    }
}

impl Battery {
    pub fn interval(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "interval").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_interval(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "interval", value)
    }
    pub fn level(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "level").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_level(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "level", value)
    }
    pub fn charging(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "charging").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_charging(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "charging", value)
    }
    pub fn available(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "available").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_available(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "available", value)
    }
}
