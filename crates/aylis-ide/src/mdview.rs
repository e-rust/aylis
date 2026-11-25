//! O Markdown aberto no editor embutido, formatado: o "Design" de um `.md` mostra o texto como
//! fica (o componente `Markdown`), e o "Código", o texto original. A visão escolhida fica por
//! arquivo, e a formatada lê o texto do editor, não o do disco (o que ainda não foi salvo já
//! aparece). As imagens relativas saem da pasta do arquivo; um link para outro `.md` abre ele
//! na IDE, formatado, e um endereço abre no navegador.

use std::cell::{Cell, RefCell};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use aylis_core::{Align, ComponentId, EventArgs, Ui};
use aylis_markdown::Markdown;

thread_local! {
    /// Os arquivos com a visão formatada escolhida (o resto mostra o texto).
    static FORMATTED: RefCell<HashSet<PathBuf>> = RefCell::new(HashSet::new());
    /// O componente que mostra o texto formatado (no centro, no lugar do editor).
    static VIEW: Cell<Option<ComponentId>> = const { Cell::new(None) };
    /// O arquivo à vista na visão formatada (as imagens relativas partem da pasta dele).
    static SHOWN: RefCell<Option<PathBuf>> = const { RefCell::new(None) };
}

/// A pasta de onde as imagens relativas do texto partem (a do arquivo à vista), lida pelo
/// serviço de recursos do `Ui` da IDE.
type Base = Arc<Mutex<PathBuf>>;

/// É um arquivo Markdown (pela extensão).
pub fn is_markdown(file: &Path) -> bool {
    file.extension().and_then(|e| e.to_str()).is_some_and(|e| e.eq_ignore_ascii_case("md") || e.eq_ignore_ascii_case("markdown"))
}

/// Cria a visão formatada no centro (escondida) e liga os links. `refresh` refaz a IDE (o
/// link que abre outro arquivo).
pub fn create(ui: &mut Ui, parent: ComponentId, refresh: impl Fn(&mut Ui) + 'static) -> ComponentId {
    let view = ui.add(Markdown::new(""), parent);
    ui.control_mut(view).unwrap().set_align(Align::Client).set_padding([24.0, 16.0, 24.0, 16.0]).set_visible(false);
    let _ = ui.set_name(view, "mdPreview");
    // Os blocos de código com as cores do editor, como na dica do mouse.
    let _ = ui.set_prop(view, "editor_look", true);
    VIEW.with(|v| v.set(Some(view)));
    // As imagens do texto: o `Ui` da IDE não tinha serviço de recursos (os forms têm o deles).
    if ui.services().resources.is_none() {
        let base: Base = Arc::default();
        ui.services_mut().resources = Some(Box::new(Images(base.clone())));
        BASE.with(|b| *b.borrow_mut() = Some(base));
    }
    ui.on(view, "on_link", move |ui, _, args| {
        if let EventArgs::Text(url) = args {
            if follow(ui, url) {
                refresh(ui);
            }
        }
    });
    view
}

thread_local! {
    static BASE: RefCell<Option<Base>> = const { RefCell::new(None) };
}

/// O componente da visão formatada.
pub fn view() -> Option<ComponentId> {
    VIEW.with(Cell::get)
}

/// O arquivo do editor embutido à vista, se é Markdown.
pub fn active_markdown(ui: &Ui) -> Option<PathBuf> {
    crate::codewindow::embedded_active(ui).filter(|f| is_markdown(f))
}

/// O arquivo Markdown à vista com a visão formatada escolhida.
pub fn formatted_active(ui: &Ui) -> Option<PathBuf> {
    active_markdown(ui).filter(|f| FORMATTED.with(|s| s.borrow().contains(f)))
}

/// Escolhe a visão do arquivo: formatada (`true`) ou o texto.
pub fn set_formatted(file: &Path, on: bool) {
    FORMATTED.with(|s| {
        let mut s = s.borrow_mut();
        if on {
            s.insert(file.to_path_buf());
        } else {
            s.remove(file);
        }
    });
}

/// Mostra ou esconde a visão formatada (o `home::sync` chama): à vista, com o texto de agora
/// do editor. Devolve se ficou à vista.
pub fn sync(ui: &mut Ui, code_view: bool) -> bool {
    let Some(view) = view() else { return false };
    let file = if code_view { formatted_active(ui) } else { None };
    let show = file.is_some();
    if ui.control(view).is_some_and(|c| c.visible() != show) {
        ui.control_mut(view).unwrap().set_visible(show);
    }
    let Some(file) = file else { return false };
    let text = crate::codewindow::text_of(ui, &file).unwrap_or_default();
    let other_file = SHOWN.with(|s| s.borrow().as_ref() != Some(&file));
    if other_file {
        SHOWN.with(|s| *s.borrow_mut() = Some(file.clone()));
        if let Some(base) = BASE.with(|b| b.borrow().clone()) {
            if let Ok(mut dir) = base.lock() {
                *dir = file.parent().map(Path::to_path_buf).unwrap_or_default();
            }
        }
        // Outro arquivo: pela propriedade (volta ao começo e esquece a seleção).
        let _ = ui.set_prop(view, "text", text);
    } else if ui.get::<Markdown>(view).is_some_and(|m| m.text != text) {
        // O mesmo arquivo editado: só o texto, sem perder a rolagem (o layout se refaz pelo
        // texto novo).
        if let Some(m) = ui.get_mut::<Markdown>(view) {
            m.text = text;
        }
        ui.request_repaint();
    }
    apply_pending_anchor(ui, view);
    true
}

/// Um link clicado no texto formatado: `#título` rola até ele; um endereço abre no navegador;
/// um arquivo relativo abre na IDE (um `.md`, formatado). Devolve se abriu um arquivo (a IDE
/// se refaz).
fn follow(ui: &mut Ui, url: &str) -> bool {
    if let Some(anchor) = url.strip_prefix('#') {
        if let Some(view) = view() {
            let _ = ui.set_prop(view, "anchor", anchor);
        }
        return false;
    }
    if url.contains("://") || url.starts_with("mailto:") {
        open_url(ui, url);
        return false;
    }
    let Some(current) = SHOWN.with(|s| s.borrow().clone()) else { return false };
    let (path, anchor) = url.split_once('#').unwrap_or((url, ""));
    let Some(target) = resolve(&current, path) else { return false };
    if let Err(e) = crate::codewindow::open(ui, &target, 1) {
        crate::codewindow::set_status(ui, &format!("⚠ não consegui abrir {}: {e}", target.display()));
        return false;
    }
    if is_markdown(&target) {
        set_formatted(&target, true);
        // O título do link, depois de mostrar o arquivo (o refresh da IDE troca o texto).
        if !anchor.is_empty() {
            PENDING_ANCHOR.with(|p| *p.borrow_mut() = Some(anchor.to_string()));
        }
    }
    true
}

thread_local! {
    /// O título para onde rolar quando o arquivo do link aparecer.
    static PENDING_ANCHOR: RefCell<Option<String>> = const { RefCell::new(None) };
}

/// Rola até o título que o último link pediu (depois que o texto do arquivo foi posto).
fn apply_pending_anchor(ui: &mut Ui, view: ComponentId) {
    if let Some(anchor) = PENDING_ANCHOR.with(|p| p.borrow_mut().take()) {
        let _ = ui.set_prop(view, "anchor", anchor);
    }
}

/// O caminho de um link relativo ao arquivo `current` (sem a parte `#título`); `None` se vazio.
fn resolve(current: &Path, link: &str) -> Option<PathBuf> {
    let link = link.trim();
    if link.is_empty() {
        return None;
    }
    let decoded = link.replace("%20", " ");
    let mut path = current.parent().map(Path::to_path_buf).unwrap_or_default();
    for part in decoded.split(['/', '\\']) {
        match part {
            "" | "." => {}
            ".." => {
                path.pop();
            }
            p => path.push(p),
        }
    }
    Some(path)
}

/// Abre o endereço no navegador (o serviço de links da plataforma; sem ele, o do sistema).
fn open_url(ui: &mut Ui, url: &str) {
    let result = match ui.services_mut().links.as_mut() {
        Some(links) => links.open_url(url),
        None => open_with_system(url),
    };
    if let Err(e) = result {
        crate::codewindow::set_status(ui, &format!("⚠ não consegui abrir {url}: {e}"));
    }
}

#[cfg(feature = "desktop")]
fn open_with_system(url: &str) -> Result<(), String> {
    aylis_tools::shell::open(url)
}

#[cfg(not(feature = "desktop"))]
fn open_with_system(_url: &str) -> Result<(), String> {
    Err("sem navegador para abrir".into())
}

/// As imagens do texto formatado: o caminho relativo à pasta do arquivo à vista. Só imagens (o
/// componente só pede imagens, e nada mais do disco sai por aqui).
struct Images(Base);

const IMAGE_EXTENSIONS: [&str; 8] = ["png", "jpg", "jpeg", "gif", "svg", "webp", "bmp", "ico"];

impl aylis_core::platform::ResourceService for Images {
    fn read(&self, path: &str) -> Option<Vec<u8>> {
        if path.contains("://") {
            return None;
        }
        let decoded = path.replace("%20", " ");
        let ext = Path::new(&decoded).extension()?.to_str()?.to_ascii_lowercase();
        if !IMAGE_EXTENSIONS.contains(&ext.as_str()) {
            return None;
        }
        let dir = self.0.lock().ok()?.clone();
        if dir.as_os_str().is_empty() {
            return None;
        }
        std::fs::read(dir.join(decoded)).ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn markdown_files_are_recognized_by_extension() {
        assert!(is_markdown(Path::new("docs/README.md")));
        assert!(is_markdown(Path::new("notas.MARKDOWN")));
        assert!(!is_markdown(Path::new("src/main.rs")));
        assert!(!is_markdown(Path::new("LEIAME")));
    }

    #[test]
    fn relative_links_resolve_from_the_file_folder() {
        let current = Path::new("/proj/docs/guias/inicio.md");
        assert_eq!(resolve(current, "outro.md"), Some(PathBuf::from("/proj/docs/guias/outro.md")));
        assert_eq!(resolve(current, "../README.md"), Some(PathBuf::from("/proj/docs/README.md")));
        assert_eq!(resolve(current, "./img/com%20espaco.md"), Some(PathBuf::from("/proj/docs/guias/img/com espaco.md")));
        assert_eq!(resolve(current, " "), None);
    }

    #[test]
    fn only_images_are_read_from_the_file_folder() {
        let dir = std::env::temp_dir().join(format!("aylis-mdview-{}", std::process::id()));
        std::fs::create_dir_all(dir.join("img")).unwrap();
        std::fs::write(dir.join("img").join("logo.png"), b"png").unwrap();
        std::fs::write(dir.join("segredo.txt"), b"nao").unwrap();
        let images = Images(Arc::new(Mutex::new(dir.clone())));
        use aylis_core::platform::ResourceService;
        assert_eq!(images.read("img/logo.png").as_deref(), Some(&b"png"[..]));
        assert_eq!(images.read("segredo.txt"), None);
        assert_eq!(images.read("https://exemplo.com/a.png"), None);
        let _ = std::fs::remove_dir_all(dir);
    }
}
