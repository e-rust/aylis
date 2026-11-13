//! A área de transferência do app: texto, HTML, imagem e os formatos do próprio app (os
//! componentes copiados no designer, um desenho, um registro), num lugar só.
//!
//! Quem copia monta um [`ClipboardContent`] com tudo o que sabe dar (o texto para os outros
//! apps e o formato próprio para quem entende) e grava com [`set`]; quem cola pede o formato
//! que quer ([`text`], [`html`], [`image`], [`data`]).
//!
//! A plataforma ([`ClipboardService`]) guarda o que souber: o Windows guarda tudo; o Linux e
//! o macOS, texto, HTML e imagem; o Android, texto e HTML; o navegador, o texto. O que ela
//! não guardar fica numa cópia no próprio processo, que vale enquanto ninguém copiar outra
//! coisa (a plataforma conta as mudanças ou, sem isso, o texto dela continua o mesmo). Sem
//! serviço nenhum (testes, o designer), só a cópia do processo.

use std::borrow::Cow;

use crate::image::{Image, ImageSource};
use crate::platform::ClipboardService;
use crate::Ui;

/// O texto simples.
pub const TEXT: &str = "text/plain";
/// O HTML (a página com o que foi copiado).
pub const HTML: &str = "text/html";
/// A imagem (uma [`Image`]: pixels ou um arquivo PNG, JPEG...).
pub const IMAGE: &str = "image";
/// Os componentes copiados no designer da IDE (o JSON do form, só os nós copiados).
pub const COMPONENTS: &str = "application/x-aylis-components+json";

/// Os pixels RGBA (sem pré-multiplicar) de uma imagem: os RGBA como estão, os BGRA
/// convertidos. Um arquivo codificado fica para quem sabe decodificar (a plataforma, o
/// PaintBox): `None`.
pub fn rgba(image: &Image) -> Option<(u32, u32, Cow<'_, [u8]>)> {
    match image.source() {
        ImageSource::Rgba { width, height, pixels } => Some((*width, *height, Cow::Borrowed(pixels))),
        ImageSource::Bgra { width, height, pixels } => {
            let mut out = pixels.clone();
            out.chunks_exact_mut(4).for_each(|px| px.swap(0, 2));
            Some((*width, *height, Cow::Owned(out)))
        }
        ImageSource::Encoded(_) => None,
    }
}

/// O que vai para a área de transferência de uma vez: o que estava sai.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ClipboardContent {
    pub text: Option<String>,
    pub html: Option<String>,
    pub image: Option<Image>,
    /// Os formatos do app: o nome (um tipo MIME, como [`COMPONENTS`]) e os bytes.
    pub data: Vec<(String, Vec<u8>)>,
}

impl ClipboardContent {
    pub fn text(text: impl Into<String>) -> Self {
        Self { text: Some(text.into()), ..Default::default() }
    }

    pub fn image(image: Image) -> Self {
        Self { image: Some(image), ..Default::default() }
    }

    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn with_html(mut self, html: impl Into<String>) -> Self {
        self.html = Some(html.into());
        self
    }

    pub fn with_image(mut self, image: Image) -> Self {
        self.image = Some(image);
        self
    }

    /// Acrescenta (ou troca) um formato do app.
    pub fn with_data(mut self, format: &str, bytes: impl Into<Vec<u8>>) -> Self {
        self.data.retain(|(f, _)| f != format);
        self.data.push((format.to_string(), bytes.into()));
        self
    }

    /// Os bytes de um formato do app.
    pub fn get_data(&self, format: &str) -> Option<&[u8]> {
        self.data.iter().find(|(f, _)| f == format).map(|(_, b)| b.as_slice())
    }

    /// Se tem o formato (`TEXT`, `HTML`, `IMAGE` ou um do app).
    pub fn has(&self, format: &str) -> bool {
        match format {
            TEXT => self.text.is_some(),
            HTML => self.html.is_some(),
            IMAGE => self.image.is_some(),
            _ => self.get_data(format).is_some(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.text.is_none() && self.html.is_none() && self.image.is_none() && self.data.is_empty()
    }
}

/// A cópia no processo do que foi gravado por último, com a marca da plataforma de quando
/// foi gravado: se a marca mudou, outro app copiou por cima e a cópia não vale mais.
#[derive(Default)]
pub struct LocalClipboard {
    content: ClipboardContent,
    mark: Option<Mark>,
}

#[derive(PartialEq)]
enum Mark {
    Sequence(u64),
    Text(Option<String>),
}

fn mark(service: &mut dyn ClipboardService) -> Mark {
    match service.sequence() {
        Some(n) => Mark::Sequence(n),
        None => Mark::Text(service.get_text()),
    }
}

/// A cópia do processo, se ainda vale.
fn local(ui: &mut Ui) -> Option<&ClipboardContent> {
    let services = ui.services_mut();
    let valid = match services.clipboard.as_deref_mut() {
        Some(service) => services.clipboard_local.mark.as_ref() == Some(&mark(service)),
        None => true,
    };
    valid.then_some(&services.clipboard_local.content)
}

/// Grava `content` (o que estava sai).
pub fn set(ui: &mut Ui, content: ClipboardContent) {
    let services = ui.services_mut();
    let mark = services.clipboard.as_deref_mut().map(|service| {
        service.set_content(&content);
        mark(service)
    });
    services.clipboard_local = LocalClipboard { content, mark };
}

/// Grava só o texto.
pub fn set_text(ui: &mut Ui, text: &str) {
    set(ui, ClipboardContent::text(text));
}

/// Esvazia.
pub fn clear(ui: &mut Ui) {
    set(ui, ClipboardContent::default());
}

/// O texto (o de outro app também).
pub fn text(ui: &mut Ui) -> Option<String> {
    let native = ui.services_mut().clipboard.as_deref_mut().and_then(|s| s.get_text());
    native.or_else(|| local(ui)?.text.clone()).filter(|t| !t.is_empty())
}

/// O HTML.
pub fn html(ui: &mut Ui) -> Option<String> {
    let native = ui.services_mut().clipboard.as_deref_mut().and_then(|s| s.get_html());
    native.or_else(|| local(ui)?.html.clone())
}

/// A imagem: a gravada por este app sai igual, sem passar pela conversão da plataforma.
pub fn image(ui: &mut Ui) -> Option<Image> {
    if let Some(image) = local(ui).and_then(|c| c.image.clone()) {
        return Some(image);
    }
    ui.services_mut().clipboard.as_deref_mut().and_then(|s| s.get_image())
}

/// Os bytes de um formato do app (de outra instância também, onde a plataforma guarda).
pub fn data(ui: &mut Ui, format: &str) -> Option<Vec<u8>> {
    if let Some(bytes) = local(ui).and_then(|c| c.get_data(format)) {
        return Some(bytes.to_vec());
    }
    ui.services_mut().clipboard.as_deref_mut().and_then(|s| s.get_data(format))
}

/// Se há o formato (`TEXT`, `HTML`, `IMAGE` ou um do app) para colar.
pub fn has(ui: &mut Ui, format: &str) -> bool {
    if local(ui).is_some_and(|c| c.has(format)) {
        return true;
    }
    let Some(service) = ui.services_mut().clipboard.as_deref_mut() else { return false };
    match format {
        TEXT => service.get_text().is_some_and(|t| !t.is_empty()),
        _ => service.has(format),
    }
}
