
/// Um MotionSensor de um form.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MotionSensor {
    pub id: ComponentId,
}

impl Handle for MotionSensor {
    fn id(&self) -> ComponentId {
        self.id
    }
}

impl MotionSensor {
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
    pub fn x(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "x").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_x(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "x", value)
    }
    pub fn y(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "y").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_y(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "y", value)
    }
    pub fn z(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "z").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_z(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "z", value)
    }
    pub fn available(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "available").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_available(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "available", value)
    }
}
