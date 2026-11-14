//! Compartilhar no Windows (o `share` do `AppLink`, decidido pelo autor em 05/11): a folha
//! "Compartilhar" do sistema (`DataTransferManager`), aberta sobre a janela do app, com o texto
//! e os arquivos. O sistema pede o conteúdo num evento (`DataRequested`); um `share` novo tira
//! o manipulador do anterior antes de pôr o dele.

use std::sync::Mutex;

use windows::ApplicationModel::DataTransfer::{DataRequestedEventArgs, DataTransferManager};
use windows::Foundation::TypedEventHandler;
use windows::Storage::{IStorageItem, StorageFile};
use windows::Win32::UI::Shell::IDataTransferManagerInterop;
use windows::core::{HSTRING, Interface, factory};

/// O manipulador do último `share` (para tirar antes do próximo).
static LAST: Mutex<Option<i64>> = Mutex::new(None);

pub(crate) fn share(text: &str, files: &[String]) -> Result<(), String> {
    let hwnd = crate::hello::app_window();
    if hwnd.is_invalid() {
        return Err(aylis_core::messages::t("sem janela para compartilhar"));
    }
    // Os arquivos têm de existir (o sistema os abre quando o destino pede).
    if let Some(missing) = files.iter().find(|f| !std::path::Path::new(f).is_file()) {
        return Err(format!("{missing}: {}", aylis_core::messages::t("arquivo não encontrado")));
    }
    let interop = factory::<DataTransferManager, IDataTransferManagerInterop>().map_err(|e| e.message())?;
    // SAFETY: a janela é do app (a thread da interface), e o WinRT devolve o gerente dela.
    let manager: DataTransferManager = unsafe { interop.GetForWindow(hwnd) }.map_err(|e| e.message())?;
    if let Some(previous) = LAST.lock().ok().and_then(|mut l| l.take()) {
        let _ = manager.RemoveDataRequested(previous);
    }
    let (text, files) = (text.to_string(), files.to_vec());
    let handler = TypedEventHandler::<DataTransferManager, DataRequestedEventArgs>::new(move |_, args| {
        let Some(args) = args.as_ref() else { return Ok(()) };
        let data = args.Request()?.Data()?;
        let title = text.lines().next().filter(|l| !l.trim().is_empty()).unwrap_or("Aylis");
        data.Properties()?.SetTitle(&HSTRING::from(title))?;
        if !text.is_empty() {
            data.SetText(&HSTRING::from(text.as_str()))?;
        }
        if !files.is_empty() {
            let mut items: Vec<Option<IStorageItem>> = Vec::new();
            for path in &files {
                let file = StorageFile::GetFileFromPathAsync(&HSTRING::from(path.as_str()))?.join()?;
                items.push(Some(file.cast::<IStorageItem>()?));
            }
            data.SetStorageItemsReadOnly(&windows_collections::IIterable::<IStorageItem>::from(items))?;
        }
        Ok(())
    });
    let token = manager.DataRequested(&handler).map_err(|e| e.message())?;
    if let Ok(mut last) = LAST.lock() {
        *last = Some(token);
    }
    // SAFETY: a mesma janela do app.
    unsafe { interop.ShowShareUIForWindow(hwnd) }.map_err(|e| e.message())
}
