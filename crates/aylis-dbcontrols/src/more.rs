//! A imagem de um campo (o `DBImage`).

use std::any::Any;

use aylis_controls::ImageView;
use aylis_core::{Component, ComponentId, EventCtx, Image, Ui, properties};
use aylis_data::{DataEvent, FieldLink, Value};

use crate::{data_message, link_of};

// ---------------------------------------------------------------------------

/// Imagem de um campo (como o `TDBImage`): campo `blob` com os bytes da imagem (PNG,
/// JPEG...) ou campo de texto com o caminho de um arquivo do projeto.
#[derive(Default)]
pub struct DBImage {
    pub base: ImageView,
    pub data_source: Option<ComponentId>,
    pub data_field: String,
}

impl DBImage {
    pub fn new(data_source: ComponentId, data_field: &str) -> Self {
        Self { data_source: Some(data_source), data_field: data_field.to_string(), ..Default::default() }
    }

    fn link(&self) -> FieldLink {
        link_of(self.data_source, &self.data_field)
    }

    fn load(&mut self, ui: &Ui) {
        match self.link().value(ui) {
            Value::Blob(bytes) if !bytes.is_empty() => {
                self.base.set_picture(Some(Image::encoded(bytes)));
                self.base.image.clear();
            }
            Value::String(path) => {
                self.base.set_picture(None);
                self.base.image = path;
            }
            _ => {
                self.base.set_picture(None);
                self.base.image.clear();
            }
        }
    }
}

impl Component for DBImage {
    fn class_name(&self) -> &'static str {
        "DBImage"
    }

    fn inner(&self) -> Option<&dyn Component> {
        Some(&self.base)
    }

    fn inner_mut(&mut self) -> Option<&mut dyn Component> {
        Some(&mut self.base)
    }

    properties! {
        data_source: Option<ComponentId> => "Data",
        data_field: String => "Data",
    }

    fn message(&mut self, ctx: &mut EventCtx<'_>, message: &dyn Any) {
        let Some(m) = data_message(message) else { return };
        if !self.link().concerns(ctx.ui, m) || m.event == DataEvent::UpdateData {
            return;
        }
        self.load(ctx.ui);
        ctx.repaint();
    }
}
