//! As janelas de escolher arquivo e pasta. No desktop, as do sistema (o `rfd`); na IDE lite
//! do navegador não há disco para escolher: os mesmos métodos, sempre "nada escolhido" (o
//! projeto entra e sai em ZIP, veja `lite.rs`).

#[cfg(feature = "desktop")]
pub use rfd::{AsyncFileDialog, FileDialog};

#[cfg(not(feature = "desktop"))]
pub use lite::{AsyncFileDialog, FileDialog, FileHandle};

#[cfg(not(feature = "desktop"))]
mod lite {
    use std::path::{Path, PathBuf};

    /// A escolha de arquivo síncrona (como a do `rfd`): nunca escolhe nada.
    #[derive(Default)]
    pub struct FileDialog;

    impl FileDialog {
        pub fn new() -> Self {
            Self
        }

        pub fn add_filter(self, _name: impl Into<String>, _extensions: &[impl ToString]) -> Self {
            self
        }

        pub fn set_directory<P: AsRef<Path>>(self, _dir: P) -> Self {
            self
        }

        pub fn set_file_name(self, _name: impl Into<String>) -> Self {
            self
        }

        pub fn pick_file(self) -> Option<PathBuf> {
            None
        }

        pub fn pick_folder(self) -> Option<PathBuf> {
            None
        }

        pub fn save_file(self) -> Option<PathBuf> {
            None
        }
    }

    /// O arquivo escolhido na versão assíncrona.
    pub struct FileHandle(PathBuf);

    impl FileHandle {
        pub fn path(&self) -> &Path {
            &self.0
        }
    }

    /// A escolha de arquivo assíncrona (como a do `rfd`): nunca escolhe nada.
    #[derive(Default)]
    pub struct AsyncFileDialog;

    impl AsyncFileDialog {
        pub fn new() -> Self {
            Self
        }

        pub fn add_filter(self, _name: impl Into<String>, _extensions: &[impl ToString]) -> Self {
            self
        }

        pub fn set_directory<P: AsRef<Path>>(self, _dir: P) -> Self {
            self
        }

        pub fn set_file_name(self, _name: impl Into<String>) -> Self {
            self
        }

        pub async fn pick_file(self) -> Option<FileHandle> {
            None
        }

        pub async fn pick_folder(self) -> Option<FileHandle> {
            None
        }

        pub async fn save_file(self) -> Option<FileHandle> {
            None
        }
    }
}
