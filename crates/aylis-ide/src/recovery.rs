//! Recuperação dos forms não salvos, como a pasta `__recovery` do Delphi: a cada 30 s a IDE
//! guarda na memória o JSON dos forms alterados e, de 2 em 2 minutos (ou num pânico), grava
//! em `%LOCALAPPDATA%\aylis\recovery`. Ao fechar normalmente, a pasta é limpa; se a IDE caiu,
//! na próxima abertura ela pergunta se recupera.

use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::{Duration, Instant};

use aylis_core::Ui;
use serde_json::{Value, json};

use crate::document::Document;
use crate::app::Ids;
use crate::state::Shared;

/// De quanto em quanto tempo a cópia em memória vai para o disco.
const WRITE_EVERY: Duration = Duration::from_secs(120);

/// Um form guardado: o arquivo original (sem ele, um form novo) e o layout.
#[derive(Clone, Debug, PartialEq)]
pub struct Saved {
    pub path: Option<PathBuf>,
    pub form: String,
    pub layout: String,
}

/// A última cópia (em memória, para o gancho de pânico gravar) e quando foi ao disco.
struct Copies {
    forms: Vec<Saved>,
    written: Option<Instant>,
    dirty: bool,
}

static COPIES: Mutex<Copies> = Mutex::new(Copies { forms: Vec::new(), written: None, dirty: false });

/// Ligada por [`start`] (a IDE de verdade): os testes montam a IDE sem gravar nada.
static ACTIVE: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// A pasta da recuperação.
pub fn dir() -> Option<PathBuf> {
    Some(aylis_tools::dap::tools_dir()?.parent()?.join("recovery"))
}

fn lock_file() -> Option<PathBuf> {
    Some(dir()?.join("session.lock"))
}

/// A IDE abriu: marca a sessão (some ao fechar normalmente) e liga o gancho de pânico.
/// Devolve os forms de uma sessão anterior que não terminou bem, se houver.
pub fn start() -> Vec<Saved> {
    let pending = if lock_file().is_some_and(|l| l.exists()) { read_all() } else { Vec::new() };
    if let Some(dir) = dir() {
        let _ = std::fs::create_dir_all(&dir);
        if pending.is_empty() {
            clear_files(&dir);
        }
        let _ = std::fs::write(dir.join("session.lock"), std::process::id().to_string());
    }
    install_panic_hook();
    ACTIVE.store(true, std::sync::atomic::Ordering::Relaxed);
    pending
}

/// A IDE fechou normalmente: nada a recuperar.
pub fn finish() {
    if let Some(dir) = dir() {
        clear_files(&dir);
        let _ = std::fs::remove_file(dir.join("session.lock"));
    }
}

fn clear_files(dir: &Path) {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for e in entries.flatten() {
            if e.path().extension().is_some_and(|x| x == "json") {
                let _ = std::fs::remove_file(e.path());
            }
        }
    }
}

/// Guarda os forms alterados (no timer da IDE). O disco é atualizado a cada [`WRITE_EVERY`].
pub fn snapshot(state: &Shared) {
    if !ACTIVE.load(std::sync::atomic::Ordering::Relaxed) {
        return;
    }
    let forms = {
        let st = state.borrow();
        (0..st.tab_count())
            .filter_map(|i| st.tab(i))
            .filter(|d| d.modified)
            .filter_map(|d| Some(Saved { path: d.path.clone(), form: d.form_name(), layout: d.to_json().ok()? }))
            .collect::<Vec<_>>()
    };
    let Ok(mut copies) = COPIES.lock() else { return };
    if copies.forms != forms {
        copies.forms = forms;
        copies.dirty = true;
    }
    if copies.dirty && copies.written.is_none_or(|t| t.elapsed() >= WRITE_EVERY) {
        write_all(&copies.forms);
        copies.dirty = false;
        copies.written = Some(Instant::now());
    }
}

fn write_all(forms: &[Saved]) {
    let Some(dir) = dir() else { return };
    let _ = std::fs::create_dir_all(&dir);
    clear_files(&dir);
    for (i, f) in forms.iter().enumerate() {
        let doc = json!({ "path": f.path.as_ref().map(|p| p.display().to_string()), "form": f.form, "layout": f.layout });
        let _ = std::fs::write(dir.join(format!("{i:02}.json")), doc.to_string());
    }
}

fn read_all() -> Vec<Saved> {
    let Some(dir) = dir() else { return Vec::new() };
    let Ok(entries) = std::fs::read_dir(&dir) else { return Vec::new() };
    let mut files: Vec<PathBuf> = entries.flatten().map(|e| e.path()).filter(|p| p.extension().is_some_and(|x| x == "json")).collect();
    files.sort();
    files
        .iter()
        .filter_map(|p| serde_json::from_str::<Value>(&std::fs::read_to_string(p).ok()?).ok())
        .filter_map(|v| {
            Some(Saved {
                path: v["path"].as_str().map(PathBuf::from),
                form: v["form"].as_str().unwrap_or_default().to_string(),
                layout: v["layout"].as_str()?.to_string(),
            })
        })
        .collect()
}

/// Num pânico, a última cópia em memória vai para o disco. Se o estado estiver travado (o
/// pânico no meio de uma gravação), desiste em silêncio: vale a cópia anterior do disco.
fn install_panic_hook() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        let previous = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            if let Ok(copies) = COPIES.try_lock() {
                if !copies.forms.is_empty() {
                    write_all(&copies.forms);
                }
            }
            previous(info);
        }));
    });
}

/// Pergunta se recupera os forms da sessão que caiu; recuperar abre cada um com o conteúdo
/// guardado (alterado, para salvar), descartar apaga as cópias.
pub fn offer(ui: &mut Ui, state: &Shared, ids: &Ids, forms: Vec<Saved>) {
    if forms.is_empty() {
        return;
    }
    let names: Vec<String> = forms.iter().map(|f| f.form.clone()).collect();
    let text = format!("{}\n\n{}", ui.tr("A IDE fechou de forma inesperada. Recuperar os forms não salvos?"), names.join(", "));
    let title = ui.tr("Recuperar forms");
    let ok = ui.tr("Recuperar");
    let (state, ids) = (state.clone(), *ids);
    aylis_controls::confirm(ui, &title, &text, &ok, false, move |ui, yes| {
        if yes {
            restore(&state, &forms);
        }
        if let Some(dir) = dir() {
            clear_files(&dir);
        }
        crate::app::refresh(ui, &state, &ids);
    });
}

fn restore(state: &Shared, forms: &[Saved]) {
    let mut restored = 0;
    for f in forms {
        match &f.path {
            Some(path) if path.exists() => {
                crate::actions::open_form_path(state, path);
                let mut st = state.borrow_mut();
                if st.doc.replace_json(&f.layout).is_ok() {
                    st.doc.modified = true;
                    restored += 1;
                }
            }
            _ => {
                if let Ok(mut doc) = Document::from_json(&f.layout) {
                    doc.modified = true;
                    state.borrow_mut().open_tab(doc);
                    restored += 1;
                }
            }
        }
    }
    state.borrow_mut().status = format!("{restored} form(s) recuperado(s): confira e salve");
}
