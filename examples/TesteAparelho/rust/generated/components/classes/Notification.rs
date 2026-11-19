
/// Um Notification de um form.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Notification {
    pub id: ComponentId,
}

impl Handle for Notification {
    fn id(&self) -> ComponentId {
        self.id
    }
}

/// Quem disparou um handler tipado (`sender: Notification`): só um Notification.
impl aylis::prelude::FromSender for Notification {
    fn from_sender(ui: &Ui, id: ComponentId) -> Option<Self> {
        (ui.class_name(id) == Some("Notification")).then_some(Self { id })
    }
}

impl Notification {
    pub fn title(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "title"))
    }
    pub fn set_title(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "title", Into::<String>::into(value))
    }
    pub fn text(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "text"))
    }
    pub fn set_text(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "text", Into::<String>::into(value))
    }
    pub fn image(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "image"))
    }
    pub fn set_image(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "image", Into::<String>::into(value))
    }
    pub fn form(self, ui: &Ui) -> FormName {
        handles::read_enum::<FormName>(ui, self.id, "form")
    }
    pub fn set_form(self, ui: &mut Ui, value: FormName) {
        handles::write(ui, self.id, "form", PropValue::enum_value(value))
    }
    pub fn form_width(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "form_width").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_form_width(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "form_width", value)
    }
    pub fn form_height(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "form_height").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_form_height(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "form_height", value)
    }
    pub fn app_name(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "app_name"))
    }
    pub fn set_app_name(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "app_name", Into::<String>::into(value))
    }
    pub fn app_id(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "app_id"))
    }
    pub fn set_app_id(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "app_id", Into::<String>::into(value))
    }
    pub fn notification_id(self, ui: &Ui) -> i64 {
        handles::read(ui, self.id, "notification_id").and_then(|v| v.to_i64().ok()).unwrap_or_default()
    }
    pub fn set_notification_id(self, ui: &mut Ui, value: i64) {
        handles::write(ui, self.id, "notification_id", value)
    }
    pub fn channel(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "channel"))
    }
    pub fn set_channel(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "channel", Into::<String>::into(value))
    }
    pub fn channel_name(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "channel_name"))
    }
    pub fn set_channel_name(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "channel_name", Into::<String>::into(value))
    }
    pub fn priority(self, ui: &Ui) -> aylis::prelude::NotificationPriority {
        handles::read_enum::<aylis::prelude::NotificationPriority>(ui, self.id, "priority")
    }
    pub fn set_priority(self, ui: &mut Ui, value: aylis::prelude::NotificationPriority) {
        handles::write(ui, self.id, "priority", PropValue::enum_value(value))
    }
    pub fn actions(self, ui: &Ui) -> Vec<NotificationAction> {
        NotificationAction::list_from_text(&handles::text(ui, handles::read(ui, self.id, "actions")))
    }
    pub fn set_actions(self, ui: &mut Ui, value: &[NotificationAction]) {
        handles::write(ui, self.id, "actions", PropValue::String(NotificationAction::list_text(value)))
    }
    pub fn ongoing(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "ongoing").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_ongoing(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "ongoing", value)
    }
    pub fn progress(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "progress").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_progress(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "progress", value)
    }
    pub fn full_screen(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "full_screen").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_full_screen(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "full_screen", value)
    }
    pub fn on_click(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "on_click"))
    }
    pub fn set_on_click(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "on_click", Into::<String>::into(value))
    }
    pub fn on_action(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "on_action"))
    }
    pub fn set_on_action(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "on_action", Into::<String>::into(value))
    }
}
