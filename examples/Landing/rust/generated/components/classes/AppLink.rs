
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

impl AppLink {
    pub fn open_url(self, ui: &mut Ui, url: &str) -> Result<bool, String> {
        handles::try_call(ui, self.id, "open_url", &[PropValue::String(url.to_string())]).map(|v| v.to_bool().unwrap_or_default())
    }
    pub fn open_app(self, ui: &mut Ui, uri: &str) -> Result<bool, String> {
        handles::try_call(ui, self.id, "open_app", &[PropValue::String(uri.to_string())]).map(|v| v.to_bool().unwrap_or_default())
    }
    pub fn share(self, ui: &mut Ui, text: &str, files: &str) -> Result<bool, String> {
        handles::try_call(ui, self.id, "share", &[PropValue::String(text.to_string()), PropValue::String(files.to_string())]).map(|v| v.to_bool().unwrap_or_default())
    }
    pub fn request(self, ui: &mut Ui, action: &str, data: &str) -> Result<bool, String> {
        handles::try_call(ui, self.id, "request", &[PropValue::String(action.to_string()), PropValue::String(data.to_string())]).map(|v| v.to_bool().unwrap_or_default())
    }
}
