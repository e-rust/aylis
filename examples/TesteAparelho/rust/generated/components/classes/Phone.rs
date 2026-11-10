
/// Um Phone de um form.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Phone {
    pub id: ComponentId,
}

impl Handle for Phone {
    fn id(&self) -> ComponentId {
        self.id
    }
}

impl Phone {
    pub fn watch_state(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "watch_state").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_watch_state(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "watch_state", value)
    }
    pub fn direct_call(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "direct_call").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_direct_call(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "direct_call", value)
    }
    pub fn dialer_app(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "dialer_app").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_dialer_app(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "dialer_app", value)
    }
    pub fn state(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "state"))
    }
    pub fn set_state(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "state", Into::<String>::into(value))
    }
    pub fn number(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "number"))
    }
    pub fn set_number(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "number", Into::<String>::into(value))
    }
    pub fn dial(self, ui: &mut Ui, number: &str) -> Result<bool, String> {
        handles::try_call(ui, self.id, "dial", &[PropValue::String(number.to_string())]).map(|v| v.to_bool().unwrap_or_default())
    }
    pub fn call(self, ui: &mut Ui, number: &str) -> Result<bool, String> {
        handles::try_call(ui, self.id, "call", &[PropValue::String(number.to_string())]).map(|v| v.to_bool().unwrap_or_default())
    }
    pub fn answer(self, ui: &mut Ui) -> Result<bool, String> {
        handles::try_call(ui, self.id, "answer", &[]).map(|v| v.to_bool().unwrap_or_default())
    }
    pub fn reject(self, ui: &mut Ui) -> Result<bool, String> {
        handles::try_call(ui, self.id, "reject", &[]).map(|v| v.to_bool().unwrap_or_default())
    }
    pub fn hang_up(self, ui: &mut Ui) -> Result<bool, String> {
        handles::try_call(ui, self.id, "hang_up", &[]).map(|v| v.to_bool().unwrap_or_default())
    }
    pub fn request_dialer(self, ui: &mut Ui) -> Result<bool, String> {
        handles::try_call(ui, self.id, "request_dialer", &[]).map(|v| v.to_bool().unwrap_or_default())
    }
    pub fn history(self, ui: &mut Ui, limit: i64) -> Result<String, String> {
        let v = handles::try_call(ui, self.id, "history", &[limit.into()])?;
        Ok(handles::text(ui, Some(v)))
    }
}
