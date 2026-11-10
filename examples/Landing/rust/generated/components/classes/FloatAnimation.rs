
/// Um FloatAnimation de um form.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FloatAnimation {
    pub id: ComponentId,
}

impl Handle for FloatAnimation {
    fn id(&self) -> ComponentId {
        self.id
    }
}

impl FloatAnimation {
    pub fn property(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "property"))
    }
    pub fn set_property(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "property", Into::<String>::into(value))
    }
    pub fn start_value(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "start_value").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_start_value(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "start_value", value)
    }
    pub fn stop_value(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "stop_value").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_stop_value(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "stop_value", value)
    }
    pub fn duration(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "duration").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_duration(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "duration", value)
    }
    pub fn delay(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "delay").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_delay(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "delay", value)
    }
    pub fn easing(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "easing"))
    }
    pub fn set_easing(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "easing", Into::<String>::into(value))
    }
    pub fn trigger(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "trigger"))
    }
    pub fn set_trigger(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "trigger", Into::<String>::into(value))
    }
    pub fn auto_reverse(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "auto_reverse").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_auto_reverse(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "auto_reverse", value)
    }
    pub fn looping(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "looping").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_looping(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "looping", value)
    }
    pub fn enabled(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "enabled").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_enabled(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "enabled", value)
    }
    pub fn keyframes(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "keyframes"))
    }
    pub fn set_keyframes(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "keyframes", Into::<String>::into(value))
    }
}
