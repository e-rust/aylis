
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

impl Vibration {
    pub fn vibrate(self, ui: &mut Ui, ms: i64) -> Result<bool, String> {
        handles::try_call(ui, self.id, "vibrate", &[ms.into()]).map(|v| v.to_bool().unwrap_or_default())
    }
}
