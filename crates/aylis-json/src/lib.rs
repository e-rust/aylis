//! # aylis-json
//!
//! Layouts e temas do aylis em JSON — o formato padrão de arquivo, de tema e do
//! protocolo com o backend.
//!
//! ```json
//! {
//!   "aylis": 1,
//!   "root": {
//!     "class": "Form",
//!     "name": "FrmClientes",
//!     "caption": "Clientes",
//!     "children": [
//!       { "class": "Button", "name": "btnSalvar", "align": "left", "text": "Salvar", "on_click": "btn_salvar_click" }
//!     ]
//!   }
//! }
//! ```
//!
//! Regras:
//! - chaves reservadas: `class`, `name` (opcional) e `children`; todo o resto é
//!   propriedade, inclusive eventos (`on_*`, com o nome do handler);
//! - ao salvar, só vai o que difere do valor padrão da classe;
//! - `margins`/`padding`: `8`, `[h, v]`, `[l, t, r, b]` ou `{ "left": 4 }`; também
//!   `"margins.left": 4`;
//! - cores: `"#RRGGBB"`, `"#RRGGBBAA"` ou o nome de um token do tema (`"primary"`);
//! - referências a outros componentes: o nome, resolvido depois de criar tudo (a ordem
//!   no arquivo não importa).
//!
//! As classes usadas precisam estar registradas no [`Ui`](aylis_core::Ui)
//! (`aylis_controls::register(&mut ui)` registra as padrão).

mod format;
mod layout;
mod props;
mod reload;
mod schema;
mod theme;
mod value;

use std::fmt;

pub use format::to_form_text;
pub use layout::{load, load_value, save, save_value, validate};
pub use props::{prop_from_json, prop_to_json, set_props};
pub use reload::{ReloadReport, reload};
pub use schema::{catalog, class_info, schema};
pub use theme::{load_theme, save_theme, theme_from_value, theme_to_value};

/// Versão do formato gravada em `"aylis"`.
pub const FORMAT_VERSION: u64 = 1;

/// Erro ao ler ou gravar JSON. `path` mostra onde, como `FrmClientes > pnlTopo > Button`.
#[derive(Debug)]
pub enum Error {
    /// JSON malformado (com linha e coluna).
    Json(serde_json::Error),
    /// Versão do formato mais nova que esta biblioteca.
    Version(u64),
    /// Estrutura inválida (falta `class`, `children` não é lista...).
    Structure { path: String, message: String },
    /// Falha numa propriedade (desconhecida, tipo errado, valor inválido...).
    Property { path: String, property: String, message: String },
    /// Falha ao criar o componente (classe não registrada...).
    Component { path: String, source: aylis_core::Error },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Json(e) => write!(f, "JSON inválido: {e}"),
            Error::Version(v) => {
                if *v > FORMAT_VERSION {
                    write!(f, "formato {v} é mais novo que este aylis (que lê o {FORMAT_VERSION}): atualize o aylis")
                } else {
                    write!(f, "formato {v} é de uma versão anterior do Aylis (este lê o {FORMAT_VERSION}): rode o aylis-migrate no projeto")
                }
            }
            Error::Structure { path, message } => write!(f, "{}{message}", at(path)),
            Error::Property { path, property, message } => {
                write!(f, "{}propriedade '{property}': {message}", at(path))
            }
            Error::Component { path, source } => write!(f, "{}{source}", at(path)),
        }
    }
}

fn at(path: &str) -> String {
    if path.is_empty() { String::new() } else { format!("{path}: ") }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Json(e) => Some(e),
            Error::Component { source, .. } => Some(source),
            _ => None,
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Json(e)
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
