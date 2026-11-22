//! [`ImageList`]: uma lista de imagens que outros componentes usam pelo índice ou pelo nome. A
//! coluna de ícone do `DataGrid` a usa (decidido em 22/11): o campo do registro diz qual imagem
//! (a posição ou o nome) e a grade desenha. Cada item é um ícone de pacote (`fluent:play-fill`,
//! pintado na cor do texto, como os outros ícones) ou um arquivo do projeto (a imagem como é).
//! Não visual.

use std::cell::RefCell;
use std::collections::HashMap;

use aylis_core::icons::Icon;
use aylis_core::{Component, ComponentId, EventCtx, Image, PropKind, Ui, properties};

/// O registro de um item da lista.
pub const IMAGE_LIST_ITEM: aylis_core::RecordInfo = aylis_core::RecordInfo {
    name: "ImageListItem",
    fields: &[("name", PropKind::String), ("icon", PropKind::String), ("image", PropKind::String)],
};

/// Um item: o nome (para achar pelo nome) e o ícone de pacote ou o arquivo do projeto.
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct ImageListItem {
    pub name: String,
    /// Um ícone de pacote (`fluent:play-fill`): pintado na cor do texto.
    pub icon: String,
    /// Um arquivo do projeto (`assets/images/play.png`), quando não é ícone.
    pub image: String,
}

/// O que um item da lista desenha.
#[derive(Clone, Debug, PartialEq)]
pub enum Picture {
    /// Um ícone de pacote: quem desenha escolhe a cor.
    Icon(Icon),
    /// Uma imagem (de arquivo ou de bytes).
    Image(Image),
}

/// A lista de imagens.
#[derive(Default)]
pub struct ImageList {
    /// Os itens: a lista de [`IMAGE_LIST_ITEM`] em JSON.
    pub items: String,
    parsed: RefCell<(String, Vec<ImageListItem>)>,
    /// Os arquivos já lidos (pela posição).
    files: RefCell<HashMap<usize, Option<Image>>>,
}

impl ImageList {
    fn list(&self) -> Vec<ImageListItem> {
        let mut p = self.parsed.borrow_mut();
        if p.0 != self.items {
            *p = (self.items.clone(), crate::records::parse_list(&self.items));
        }
        p.1.clone()
    }

    /// A imagem de `key`: a posição (`0`, `1`...) ou o nome de um item. `None`: não há.
    pub fn picture(ui: &Ui, id: ComponentId, key: &str) -> Option<Picture> {
        let me = ui.get::<ImageList>(id)?;
        let list = me.list();
        let key = key.trim();
        let at = key.parse::<usize>().ok().filter(|i| *i < list.len()).or_else(|| list.iter().position(|i| i.name.eq_ignore_ascii_case(key)))?;
        let item = &list[at];
        if !item.icon.trim().is_empty() {
            return ui.icon(item.icon.trim()).cloned().map(Picture::Icon);
        }
        if item.image.trim().is_empty() {
            return None;
        }
        let mut files = me.files.borrow_mut();
        files
            .entry(at)
            .or_insert_with(|| ui.services().resources.as_ref().and_then(|r| r.read(item.image.trim())).map(Image::encoded))
            .clone()
            .map(Picture::Image)
    }

    /// Quantos itens.
    pub fn count(ui: &Ui, id: ComponentId) -> usize {
        ui.get::<ImageList>(id).map_or(0, |l| l.list().len())
    }
}

impl Component for ImageList {
    fn class_name(&self) -> &'static str {
        "ImageList"
    }

    properties! {
        items: String as RecordList(&IMAGE_LIST_ITEM) => "Data",
    }

    fn property_changed(&mut self, _ctx: &mut EventCtx<'_>, name: &str) {
        if name == "items" {
            self.files.borrow_mut().clear();
        }
    }
}
