//! O `Clipboard`: a área de transferência pelo nome, para o JSON, o protocolo e os clientes
//! das outras linguagens (no Rust, o [`aylis_core::clipboard`] direto faz o mesmo).

use aylis_core::clipboard::{self, ClipboardContent};
use aylis_core::prop::PropKind;
use aylis_core::{Component, ComponentId, EventCtx, MethodInfo, PropValue, Ui, properties};

use crate::ImageView;

/// A área de transferência do sistema, não visual: copiar e colar texto, HTML, imagem e os
/// formatos do app (um tipo MIME do app com o conteúdo em texto, como um registro em JSON).
/// Copiar grava tudo o que for passado de uma vez: com `copy_data`, o `text` vai junto para
/// os outros apps.
///
/// | Método | O quê |
/// |---|---|
/// | `copy_text(text)`, `paste_text()` | o texto |
/// | `copy_html(html, text)`, `paste_html()` | o HTML, com o texto para quem não entende HTML |
/// | `copy_data(format, data, text)`, `paste_data(format)` | um formato do app |
/// | `copy_image(source)`, `paste_image(target)` | a imagem de um `Image` (ou a foto de outro controle) e a colada num `Image` |
/// | `has_format(format)` | se há `text/plain`, `text/html`, `image` ou o formato do app para colar |
/// | `clear()` | esvazia |
///
/// | Plataforma | Guarda no sistema | Fica no app (até outro copiar) |
/// |---|---|---|
/// | Windows | tudo (outra instância cola o formato do app) | nada |
/// | Linux, macOS | texto, HTML, imagem | o formato do app |
/// | Android | texto, HTML | imagem, o formato do app |
/// | Navegador | texto, HTML | imagem, o formato do app |
#[derive(Default)]
pub struct Clipboard {
    pub tag: i64,
}

impl Clipboard {
    /// A imagem de `source`: a de um `Image`; de outro controle, a foto dele.
    fn image_of(ui: &Ui, source: ComponentId) -> Option<aylis_core::Image> {
        match ui.get::<ImageView>(source) {
            Some(view) => view.current_image(ui),
            None => ui.snapshot(source, 1.0),
        }
    }
}

fn component(args: &[PropValue], i: usize) -> Result<ComponentId, String> {
    args.get(i).and_then(|v| v.to_component().ok().flatten()).ok_or_else(|| aylis_core::messages::t("informe o componente"))
}

impl Component for Clipboard {
    fn class_name(&self) -> &'static str {
        "Clipboard"
    }

    properties! {
        tag: i64 => "Misc",
    }

    fn methods(&self) -> &'static [MethodInfo] {
        use PropKind::{Bool, Component, String};
        const METHODS: &[MethodInfo] = &[
            MethodInfo::new("copy_text", &[String], Some(Bool)).params(&["text"]),
            MethodInfo::new("copy_html", &[String, String], Some(Bool)).params(&["html", "text"]),
            MethodInfo::new("copy_data", &[String, String, String], Some(Bool)).params(&["format", "data", "text"]),
            MethodInfo::new("copy_image", &[Component], Some(Bool)).params(&["source"]),
            MethodInfo::new("paste_text", &[], Some(String)),
            MethodInfo::new("paste_html", &[], Some(String)),
            MethodInfo::new("paste_data", &[String], Some(String)).params(&["format"]),
            MethodInfo::new("paste_image", &[Component], Some(Bool)).params(&["target"]),
            MethodInfo::new("has_format", &[String], Some(Bool)).params(&["format"]),
            MethodInfo::new("clear", &[], None),
        ];
        METHODS
    }

    fn invoke(&mut self, ctx: &mut EventCtx<'_>, method: &str, args: &[PropValue]) -> Result<PropValue, String> {
        let text = |i: usize| args.get(i).and_then(|v| v.to_str().ok()).unwrap_or_default().to_string();
        let ui = &mut *ctx.ui;
        let copied = |ui: &mut Ui, content: ClipboardContent| {
            clipboard::set(ui, content);
            Ok(PropValue::Bool(true))
        };
        match method {
            "copy_text" => copied(ui, ClipboardContent::text(text(0))),
            "copy_html" => copied(ui, ClipboardContent::text(text(1)).with_html(text(0))),
            "copy_data" => {
                let format = text(0);
                if format.trim().is_empty() {
                    return Err(aylis_core::messages::t("informe o formato"));
                }
                let mut content = ClipboardContent::default().with_data(&format, text(1).into_bytes());
                if !text(2).is_empty() {
                    content = content.with_text(text(2));
                }
                copied(ui, content)
            }
            "copy_image" => {
                let Some(image) = Self::image_of(ui, component(args, 0)?) else { return Ok(PropValue::Bool(false)) };
                copied(ui, ClipboardContent::image(image))
            }
            "paste_text" => Ok(PropValue::String(clipboard::text(ui).unwrap_or_default())),
            "paste_html" => Ok(PropValue::String(clipboard::html(ui).unwrap_or_default())),
            "paste_data" => {
                let bytes = clipboard::data(ui, &text(0)).unwrap_or_default();
                Ok(PropValue::String(String::from_utf8_lossy(&bytes).into_owned()))
            }
            "paste_image" => {
                let target = component(args, 0)?;
                let Some(image) = clipboard::image(ui) else { return Ok(PropValue::Bool(false)) };
                let Some(view) = ui.get_mut::<ImageView>(target) else {
                    return Err(aylis_core::messages::t("a imagem só é colada num Image"));
                };
                view.set_picture(Some(image));
                ui.damage(target);
                Ok(PropValue::Bool(true))
            }
            "has_format" => Ok(PropValue::Bool(clipboard::has(ui, &text(0)))),
            "clear" => {
                clipboard::clear(ui);
                Ok(PropValue::Null)
            }
            _ => Err(format!("Clipboard não tem o método '{method}'")),
        }
    }
}
