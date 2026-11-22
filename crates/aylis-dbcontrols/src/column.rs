//! [`Column`]: uma coluna do `DataGrid`, como componente filho dele. Com
//! colunas, a grade mostra só elas, na ordem; sem nenhuma, vale a lista `columns` (ou todos
//! os campos visíveis).

use aylis_core::{Color, Component, ComponentId, Error, EventCtx, PropEnum, PropInfo, PropKind, PropValue, Result, TextAlign};

/// O que a coluna mostra do campo (decidido em 22/11).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ColumnKind {
    /// O texto do campo.
    #[default]
    Text,
    /// Um ícone do `ImageList` da grade (`images`): o campo diz qual (a posição ou o nome).
    Icon,
    /// A imagem guardada no campo (um `blob` com PNG, JPEG...), na altura da linha.
    Image,
}

impl PropEnum for ColumnKind {
    const NAMES: &'static [&'static str] = &["text", "icon", "image"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Text, Self::Icon, Self::Image].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Uma coluna da grade. O que fica vazio vem do campo (`Field`): o título é o
/// `display_label`, a largura o `display_width`, o alinhamento o do campo.
#[derive(Clone)]
pub struct Column {
    /// O campo do dataset que a coluna mostra.
    pub field_name: String,
    /// O título do cabeçalho (vazio = o rótulo do campo).
    pub title: String,
    /// A largura (0 = a do campo).
    pub width: f32,
    /// O alinhamento do texto (`None` = o do campo).
    pub alignment: Option<TextAlign>,
    /// A coluna aparece na grade; escondida, sai dela (o campo continua no dataset).
    pub visible: bool,
    /// Não edita nesta coluna, mesmo com a grade editável.
    pub read_only: bool,
    /// O fundo das células.
    pub color: Option<Color>,
    /// A cor do texto das células.
    pub text_color: Option<Color>,
    /// O que a coluna mostra: o texto, um ícone do `ImageList` ou a imagem do campo.
    pub kind: ColumnKind,
}

impl Default for Column {
    fn default() -> Self {
        Self { field_name: String::new(), title: String::new(), width: 0.0, alignment: None, visible: true, read_only: false, color: None, text_color: None, kind: ColumnKind::Text }
    }
}

impl Column {
    pub fn new(field_name: &str) -> Self {
        Self { field_name: field_name.to_string(), ..Default::default() }
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }
}

/// Mensagem para a grade: uma coluna mudou (refazer as colunas). Leva a coluna, que
/// durante o próprio `property_changed` não está acessível pelo `Ui`.
pub(crate) struct ColumnsChanged(pub ComponentId, pub Column);

const COLUMN_PROPS: &[PropInfo] = &[
    PropInfo::new("field_name", PropKind::String, "Data"),
    PropInfo::new("title", PropKind::String, "Appearance"),
    PropInfo::new("width", PropKind::Float, "Appearance"),
    PropInfo::new("alignment", PropKind::Enum(<TextAlign as PropEnum>::NAMES), "Appearance").rust_type("TextAlign"),
    PropInfo::new("color", PropKind::Color, "Appearance"),
    PropInfo::new("text_color", PropKind::Color, "Appearance"),
    PropInfo::new("visible", PropKind::Bool, "Behavior"),
    PropInfo::new("read_only", PropKind::Bool, "Behavior"),
    PropInfo::new("kind", PropKind::Enum(<ColumnKind as PropEnum>::NAMES), "Appearance").rust_type("ColumnKind"),
];

impl Component for Column {
    fn class_name(&self) -> &'static str {
        "Column"
    }

    fn props(&self) -> &'static [PropInfo] {
        COLUMN_PROPS
    }

    fn get_prop(&self, name: &str) -> Option<PropValue> {
        let color = |c: Option<Color>| c.map_or(PropValue::Null, PropValue::Color);
        Some(match name {
            "field_name" => self.field_name.clone().into(),
            "title" => self.title.clone().into(),
            "width" => self.width.into(),
            "alignment" => self.alignment.map_or(PropValue::Null, PropValue::enum_value),
            "color" => color(self.color),
            "text_color" => color(self.text_color),
            "visible" => self.visible.into(),
            "read_only" => self.read_only.into(),
            "kind" => PropValue::enum_value(self.kind),
            _ => return None,
        })
    }

    fn set_prop(&mut self, name: &str, value: PropValue) -> Result<()> {
        let color = |v: PropValue| -> Result<Option<Color>> {
            match v {
                PropValue::Null => Ok(None),
                v => v.to_color().map(Some),
            }
        };
        match name {
            "field_name" => self.field_name = value.to_str()?.to_string(),
            "title" => self.title = value.to_str()?.to_string(),
            "width" => self.width = value.to_f32()?.max(0.0),
            "alignment" => {
                self.alignment = match value {
                    PropValue::Null => None,
                    v => Some(v.to_enum()?),
                }
            }
            "color" => self.color = color(value)?,
            "text_color" => self.text_color = color(value)?,
            "visible" => self.visible = value.to_bool()?,
            "read_only" => self.read_only = value.to_bool()?,
            "kind" => self.kind = value.to_enum()?,
            _ => return Err(Error::UnknownProperty(name.to_string())),
        }
        Ok(())
    }

    /// A grade (o pai) refaz as colunas.
    fn property_changed(&mut self, ctx: &mut EventCtx<'_>, _name: &str) {
        if let Some(grid) = ctx.ui.parent(ctx.id) {
            ctx.ui.send_message(grid, &ColumnsChanged(ctx.id, self.clone()));
        }
    }
}
