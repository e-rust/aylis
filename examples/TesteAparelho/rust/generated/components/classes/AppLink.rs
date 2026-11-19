
/// Um AppLink de um form.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AppLink {
    pub id: ComponentId,
}

impl Handle for AppLink {
    fn id(&self) -> ComponentId {
        self.id
    }
}

/// Quem disparou um handler tipado (`sender: AppLink`): só um AppLink.
impl aylis::prelude::FromSender for AppLink {
    fn from_sender(ui: &Ui, id: ComponentId) -> Option<Self> {
        (ui.class_name(id) == Some("AppLink")).then_some(Self { id })
    }
}

impl AppLink {
    pub fn on_link(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "on_link"))
    }
    pub fn set_on_link(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "on_link", Into::<String>::into(value))
    }
    pub fn on_shared(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "on_shared"))
    }
    pub fn set_on_shared(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "on_shared", Into::<String>::into(value))
    }
    pub fn on_result(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "on_result"))
    }
    pub fn set_on_result(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "on_result", Into::<String>::into(value))
    }
    pub fn on_error(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "on_error"))
    }
    pub fn set_on_error(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "on_error", Into::<String>::into(value))
    }
    pub fn open_url(self, ui: &mut Ui, url: &str) -> Result<bool, String> {
        let v = handles::try_call(ui, self.id, "open_url", &[PropValue::String(url.to_string())])?;
        Ok(v.to_bool().unwrap_or_default())
    }
    pub fn open_app(self, ui: &mut Ui, uri: &str) -> Result<bool, String> {
        let v = handles::try_call(ui, self.id, "open_app", &[PropValue::String(uri.to_string())])?;
        Ok(v.to_bool().unwrap_or_default())
    }
    pub fn share(self, ui: &mut Ui, text: &str, files: &str) -> Result<bool, String> {
        let v = handles::try_call(ui, self.id, "share", &[PropValue::String(text.to_string()), PropValue::String(files.to_string())])?;
        Ok(v.to_bool().unwrap_or_default())
    }
    pub fn request(self, ui: &mut Ui, action: &str, data: &str) -> Result<bool, String> {
        let v = handles::try_call(ui, self.id, "request", &[PropValue::String(action.to_string()), PropValue::String(data.to_string())])?;
        Ok(v.to_bool().unwrap_or_default())
    }
}
