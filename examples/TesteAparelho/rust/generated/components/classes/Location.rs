
/// Um Location de um form.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Location {
    pub id: ComponentId,
}

impl Handle for Location {
    fn id(&self) -> ComponentId {
        self.id
    }
}

impl Location {
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
    pub fn latitude(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "latitude").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_latitude(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "latitude", value)
    }
    pub fn longitude(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "longitude").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_longitude(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "longitude", value)
    }
    pub fn accuracy(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "accuracy").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_accuracy(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "accuracy", value)
    }
    pub fn altitude(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "altitude").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_altitude(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "altitude", value)
    }
    pub fn speed(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "speed").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_speed(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "speed", value)
    }
    pub fn available(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "available").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_available(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "available", value)
    }
}
