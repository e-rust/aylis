
/// Um LightSensor de um form.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LightSensor {
    pub id: ComponentId,
}

impl Handle for LightSensor {
    fn id(&self) -> ComponentId {
        self.id
    }
}

impl LightSensor {
    pub fn active(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "active").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_active(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "active", value)
    }
    pub fn interval(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "interval").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_interval(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "interval", value)
    }
    pub fn lux(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "lux").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_lux(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "lux", value)
    }
    pub fn available(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "available").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_available(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "available", value)
    }
}
