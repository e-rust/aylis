
/// Um Vibration de um form.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Vibration {
    pub id: ComponentId,
}

impl Handle for Vibration {
    fn id(&self) -> ComponentId {
        self.id
    }
}

/// Quem disparou um handler tipado (`sender: Vibration`): só um Vibration.
impl aylis::prelude::FromSender for Vibration {
    fn from_sender(ui: &Ui, id: ComponentId) -> Option<Self> {
        (ui.class_name(id) == Some("Vibration")).then_some(Self { id })
    }
}

impl Vibration {
    pub fn on_error(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "on_error"))
    }
    pub fn set_on_error(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "on_error", Into::<String>::into(value))
    }
    pub fn vibrate(self, ui: &mut Ui, ms: i64) -> Result<bool, String> {
        let v = handles::try_call(ui, self.id, "vibrate", &[ms.into()])?;
        Ok(v.to_bool().unwrap_or_default())
    }
}
