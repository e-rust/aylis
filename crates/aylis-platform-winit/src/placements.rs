//! O `remember_placement` dos forms: onde estava a janela de cada um (o monitor, a posição,
//! o tamanho e se estava maximizada) num arquivo do app, na pasta de dados do usuário
//! (`%LOCALAPPDATA%\<app>\placements.json` no Windows), para reabrir igual. Gravado no máximo
//! uma vez por segundo enquanto a janela se move e ao fechar.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use aylis_core::WindowPlacement;
use aylis_core::monitors::SavedPlacement;
use serde_json::{Value, json};

pub(crate) struct Placements {
    path: Option<PathBuf>,
    saved: HashMap<String, SavedPlacement>,
    dirty: bool,
    last_write: Option<Instant>,
}

/// A pasta de dados do usuário para o app (pelo nome do executável).
fn app_dir() -> Option<PathBuf> {
    let app = std::env::current_exe().ok()?.file_stem()?.to_string_lossy().to_string();
    let base = if cfg!(windows) {
        std::env::var_os("LOCALAPPDATA").map(PathBuf::from)
    } else if cfg!(target_os = "macos") {
        std::env::var_os("HOME").map(|h| PathBuf::from(h).join("Library/Application Support"))
    } else {
        std::env::var_os("XDG_CONFIG_HOME").map(PathBuf::from).or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".config")))
    }?;
    Some(base.join(app))
}

impl Placements {
    /// As do app (o arquivo pode ainda não existir).
    pub(crate) fn load() -> Self {
        Self::at(app_dir().map(|d| d.join("placements.json")))
    }

    pub(crate) fn at(path: Option<PathBuf>) -> Self {
        let saved = path.as_deref().and_then(read).unwrap_or_default();
        Self { path, saved, dirty: false, last_write: None }
    }

    pub(crate) fn get(&self, form: &str) -> Option<&SavedPlacement> {
        self.saved.get(form)
    }

    pub(crate) fn set(&mut self, form: &str, saved: SavedPlacement) {
        if self.saved.get(form) != Some(&saved) {
            self.saved.insert(form.to_string(), saved);
            self.dirty = true;
        }
    }

    /// Grava se mudou e já passou um segundo da última vez.
    pub(crate) fn flush_if_due(&mut self) {
        if self.dirty && self.last_write.is_none_or(|t| t.elapsed() >= Duration::from_secs(1)) {
            self.flush();
        }
    }

    pub(crate) fn flush(&mut self) {
        if !self.dirty {
            return;
        }
        self.dirty = false;
        self.last_write = Some(Instant::now());
        let Some(path) = &self.path else { return };
        let forms: serde_json::Map<String, Value> = self
            .saved
            .iter()
            .map(|(form, s)| {
                let p = s.placement;
                (form.clone(), json!({ "monitor": s.monitor, "x": p.x, "y": p.y, "width": p.width, "height": p.height, "maximized": p.maximized }))
            })
            .collect();
        if let Some(dir) = path.parent() {
            let _ = std::fs::create_dir_all(dir);
        }
        let _ = std::fs::write(path, serde_json::to_string_pretty(&json!({ "forms": forms })).unwrap_or_default());
    }
}

fn read(path: &Path) -> Option<HashMap<String, SavedPlacement>> {
    let doc: Value = serde_json::from_str(&std::fs::read_to_string(path).ok()?).ok()?;
    let forms = doc.get("forms")?.as_object()?;
    Some(
        forms
            .iter()
            .filter_map(|(name, v)| {
                let int = |k: &str| v.get(k).and_then(Value::as_i64);
                let placement = WindowPlacement {
                    x: int("x")? as i32,
                    y: int("y")? as i32,
                    width: int("width")?.max(0) as u32,
                    height: int("height")?.max(0) as u32,
                    maximized: v.get("maximized").and_then(Value::as_bool).unwrap_or(false),
                };
                Some((name.clone(), SavedPlacement { monitor: v.get("monitor")?.as_str()?.to_string(), placement }))
            })
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn placements_survive_a_round_trip_through_the_file() {
        let dir = std::env::temp_dir().join(format!("aylis-placements-{}", std::process::id()));
        let path = dir.join("placements.json");
        let mut store = Placements::at(Some(path.clone()));
        let saved = SavedPlacement { monitor: "\\\\.\\DISPLAY2".into(), placement: WindowPlacement { x: -3000, y: 100, width: 1200, height: 800, maximized: true } };
        store.set("frmPrincipal", saved.clone());
        store.flush();
        let again = Placements::at(Some(path));
        assert_eq!(again.get("frmPrincipal"), Some(&saved));
        let _ = std::fs::remove_dir_all(dir);
    }
}
