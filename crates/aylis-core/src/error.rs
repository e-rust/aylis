//! Erros do aylis.

use std::fmt;

use crate::prop::PropValue;

/// Erro de qualquer operação do [`Ui`](crate::Ui) ou de propriedades.
#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    /// O id não existe ou o componente já foi liberado.
    InvalidId,
    /// O componente está temporariamente fora do `Ui` porque um método dele está rodando
    /// (o caminho dele, como `frmPedido.grdItens`).
    Busy(String),
    UnknownClass(String),
    UnknownProperty(String),
    TypeMismatch { property: String, expected: &'static str, got: &'static str },
    InvalidValue { property: String, message: String },
    ReadOnly(String),
    ComponentNotFound(String),
    InvalidName(String),
    DuplicateName(String),
    /// Tentativa de tornar um componente filho de si mesmo ou de um descendente.
    Cycle,
    /// [`Ui::create_form`](crate::Ui::create_form) não conseguiu criar o form.
    FormNotCreated { name: String, message: String },
}

impl Error {
    pub(crate) fn type_mismatch(expected: &'static str, got: &PropValue) -> Self {
        Error::TypeMismatch { property: String::new(), expected, got: got.kind_name() }
    }

    pub(crate) fn invalid(message: impl Into<String>) -> Self {
        Error::InvalidValue { property: String::new(), message: message.into() }
    }

    /// Preenche o nome da propriedade em erros que ainda não o têm.
    pub fn with_property(self, name: &str) -> Self {
        match self {
            Error::TypeMismatch { property, expected, got } if property.is_empty() => {
                Error::TypeMismatch { property: name.to_string(), expected, got }
            }
            Error::InvalidValue { property, message } if property.is_empty() => {
                Error::InvalidValue { property: name.to_string(), message }
            }
            other => other,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidId => write!(f, "componente inexistente ou já liberado"),
            Error::Busy(path) => write!(
                f,
                "{path} está ocupado: um método dele está rodando. Dentro do componente, use `self`; num componente \
                 composto, o pai lê os filhos, e o filho avisa o pai por evento"
            ),
            Error::UnknownClass(c) => write!(f, "classe '{c}' não registrada"),
            Error::UnknownProperty(p) => write!(f, "propriedade '{p}' não existe"),
            Error::TypeMismatch { property, expected, got } => {
                write!(f, "propriedade '{property}': esperado {expected}, recebido {got}")
            }
            Error::InvalidValue { property, message } => write!(f, "propriedade '{property}': {message}"),
            Error::ReadOnly(p) => write!(f, "propriedade '{p}' é somente leitura"),
            Error::ComponentNotFound(n) => write!(f, "componente '{n}' não encontrado"),
            Error::InvalidName(n) => write!(f, "nome inválido: '{n}'"),
            Error::DuplicateName(n) => write!(f, "já existe um componente chamado '{n}'"),
            Error::Cycle => {
                write!(f, "um componente não pode ser filho de si mesmo ou de um descendente")
            }
            Error::FormNotCreated { name, message } => write!(f, "form '{name}': {message}"),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T, E = Error> = std::result::Result<T, E>;
