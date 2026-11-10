//! # aylis-docview
//!
//! O componente [`DocViewer`]: mostra documentos dentro do form, sem programa externo.
//!
//! | Formato | Como |
//! |---|---|
//! | PDF | `hayro` (Rust puro): cada página rasterizada numa thread, na escala da tela |
//! | XLSX, XLSM, XLS, XLSB, ODS | `calamine`: planilha com cabeçalho de colunas e abas |
//! | CSV, TSV | separador detectado (`;`, `,`, tabulação) |
//! | DOCX, ODT | o texto do documento: parágrafos, títulos e tabelas (sem imagens) |

pub mod office;
pub mod pdf;
mod viewer;

pub use viewer::DocViewer;

/// Registra o `DocViewer` na paleta (categoria "Additional").
pub fn register(ui: &mut aylis_core::Ui) {
    ui.register_class::<DocViewer>("Additional");
}
