//! A IDE lite do navegador ("Experimentar" na página do aylis): a mesma IDE, sem a feature
//! `desktop`. Ficam a paleta, o designer, o inspector, a estrutura, os temas, as vistas de
//! dispositivo e a prévia; não há compilar, executar, depurar, servidor de linguagem,
//! publicar nem pacotes de código (os menus seguem [`can`]).
//!
//! Os arquivos passam pelo [`Workspace`]: uma pasta no desktop, o armazenamento do navegador
//! na lite. O form em edição é guardado sozinho e volta ao recarregar a página; "Baixar
//! projeto (ZIP)" entrega um projeto do aylis que a IDE de desktop abre.

use std::path::PathBuf;

/// O que a IDE consegue fazer onde está rodando (a mesma regra da tela inicial e dos menus).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Capability {
    /// Compilar e executar o projeto (F9, Ctrl+F9).
    Build,
    /// Depurar (Ctrl+F5) e os pontos de parada.
    Debug,
    /// Publicar para as plataformas.
    Publish,
    /// O servidor de linguagem e o editor de código.
    Code,
    /// Pastas e arquivos do disco (abrir projeto, salvar como, assets do disco).
    Disk,
    /// Pacotes de componente (instalar, criar).
    Packages,
}

/// A IDE pode fazer `what` aqui? No desktop, tudo; no navegador, nada disso.
pub fn can(what: Capability) -> bool {
    let _ = what;
    cfg!(feature = "desktop")
}

/// Do que um item de menu precisa (pelo texto, em português, como foi criado). `None`:
/// vale em toda parte (o designer, o inspector, a prévia, os temas).
pub fn needs(item: &str) -> Option<Capability> {
    use Capability::*;
    Some(match item {
        "Novo projeto…" | "Abrir projeto…" | "Abrir form…" | "Nova janela" | "Abrir gravação…" | "Recentes" | "Salvar como…" | "Salvar tudo"
        | "Fechar projeto" | "Sair" | "Abrir pasta do projeto" | "Confiar neste projeto" | "Adicionar form existente…" | "Ícone do app…" | "Android…"
        | "Gravar o uso da IDE" | "Abrir pasta das gravações" | "Relatar problema…" | "Opções da IDE…" => Disk,
        "Gerar código" | "Abrir código do form" | "Abrir fonte…" | "Pendências entre linguagens…" | "Pesquisar no projeto…" | "Depurar no aparelho" => Code,
        "Pacotes de componentes" | "Pacotes…" | "Instalar pacote…" | "Criar pacote deste form…" => Packages,
        "Executar" | "Executar sem depurar" | "Compilar e executar em release" | "Compilar" | "Compilar em release" | "Parar" | "Inspetor do app em execução…"
        | "Profiler…" | "Rastro de eventos" | "Servidor MCP para agentes de IA (liga/desliga)" | "Verificar ambiente…" | "Configurar linguagens…" => Build,
        "Depurar na IDE" | "Próxima linha" | "Entrar na função" | "Sair da função" => Debug,
        "Publicar…" => Publish,
        _ => return None,
    })
}

/// Onde os arquivos do projeto ficam: uma pasta no desktop, o armazenamento do navegador na
/// lite. Os caminhos são relativos à raiz do projeto, com `/` (`forms/principal.json`).
pub trait Workspace {
    fn read(&self, path: &str) -> Option<String>;
    fn write(&self, path: &str, text: &str) -> Result<(), String>;
    /// Os caminhos guardados que começam com `prefix`.
    fn list(&self, prefix: &str) -> Vec<String>;
}

/// Uma pasta do disco.
pub struct DirWorkspace(pub PathBuf);

impl Workspace for DirWorkspace {
    fn read(&self, path: &str) -> Option<String> {
        std::fs::read_to_string(self.0.join(path)).ok()
    }

    fn write(&self, path: &str, text: &str) -> Result<(), String> {
        let file = self.0.join(path);
        if let Some(parent) = file.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        std::fs::write(&file, text).map_err(|e| format!("{}: {e}", file.display()))
    }

    fn list(&self, prefix: &str) -> Vec<String> {
        fn walk(root: &std::path::Path, dir: &std::path::Path, out: &mut Vec<String>) {
            for e in std::fs::read_dir(dir).into_iter().flatten().flatten() {
                let p = e.path();
                if p.is_dir() {
                    walk(root, &p, out);
                } else if let Ok(rel) = p.strip_prefix(root) {
                    out.push(rel.to_string_lossy().replace('\\', "/"));
                }
            }
        }
        let mut out = Vec::new();
        walk(&self.0, &self.0, &mut out);
        out.retain(|p| p.starts_with(prefix));
        out.sort();
        out
    }
}

/// O armazenamento do navegador: o IndexedDB (banco `aylis-ide`, tabela `files`), que guarda
/// bem mais que o `localStorage` (os 5 MB dele não cabiam um projeto com temas e traduções).
/// O IndexedDB só responde depois; o [`Workspace`] é síncrono, então a lite carrega tudo para a
/// memória antes de abrir ([`BrowserWorkspace::load`]) e cada gravação vai para a memória e,
/// em seguida, para o banco.
#[cfg(target_arch = "wasm32")]
pub struct BrowserWorkspace;

#[cfg(target_arch = "wasm32")]
thread_local! {
    static FILES: std::cell::RefCell<std::collections::BTreeMap<String, String>> = std::cell::RefCell::default();
}

#[cfg(target_arch = "wasm32")]
impl BrowserWorkspace {
    /// O prefixo das versões antigas, no `localStorage` (migradas ao abrir).
    const OLD_PREFIX: &'static str = "aylis-ide/";
    /// Abre o banco (criando a tabela na primeira vez) e devolve a promessa do pedido.
    const OPEN: &'static str = "return new Promise((ok, fail) => { const r = indexedDB.open('aylis-ide', 1); \
        r.onupgradeneeded = () => r.result.createObjectStore('files'); r.onerror = () => fail(r.error); r.onsuccess = () => ok(r.result); });";

    /// Carrega o banco para a memória e passa para ele o que estava no `localStorage`.
    pub async fn load() -> Result<(), String> {
        let read = js_sys::Function::new_no_args(&format!(
            "{} ", Self::OPEN.replace("ok(r.result)", "{ const t = r.result.transaction('files'); const s = t.objectStore('files'); const k = s.getAllKeys(); const v = s.getAll(); t.oncomplete = () => ok([k.result, v.result]); t.onerror = () => fail(t.error); }")
        ));
        let promise: js_sys::Promise = read.call0(&wasm_bindgen::JsValue::NULL).map_err(|e| format!("{e:?}"))?.into();
        let pair: js_sys::Array = wasm_bindgen_futures::JsFuture::from(promise).await.map_err(|e| format!("IndexedDB: {e:?}"))?.into();
        let (keys, values): (js_sys::Array, js_sys::Array) = (pair.get(0).into(), pair.get(1).into());
        FILES.with(|f| {
            let mut f = f.borrow_mut();
            for i in 0..keys.length() {
                if let (Some(k), Some(v)) = (keys.get(i).as_string(), values.get(i).as_string()) {
                    f.insert(k, v);
                }
            }
        });
        // As versões antigas guardavam no `localStorage`: passa para o banco e apaga de lá.
        if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok().flatten()) {
            let old: Vec<String> = (0..storage.length().unwrap_or(0)).filter_map(|i| storage.key(i).ok().flatten()).filter(|k| k.starts_with(Self::OLD_PREFIX)).collect();
            for key in old {
                if let Ok(Some(text)) = storage.get_item(&key) {
                    let path = &key[Self::OLD_PREFIX.len()..];
                    if FILES.with(|f| !f.borrow().contains_key(path)) {
                        let _ = BrowserWorkspace.write(path, &text);
                    }
                }
                let _ = storage.remove_item(&key);
            }
        }
        Ok(())
    }
}

#[cfg(target_arch = "wasm32")]
impl Workspace for BrowserWorkspace {
    fn read(&self, path: &str) -> Option<String> {
        FILES.with(|f| f.borrow().get(path).cloned())
    }

    fn write(&self, path: &str, text: &str) -> Result<(), String> {
        FILES.with(|f| f.borrow_mut().insert(path.to_string(), text.to_string()));
        let put = js_sys::Function::new_with_args(
            "k, v",
            &Self::OPEN.replace("ok(r.result)", "{ const t = r.result.transaction('files', 'readwrite'); t.objectStore('files').put(v, k); t.oncomplete = () => ok(); t.onerror = () => fail(t.error); }"),
        );
        let promise: js_sys::Promise = put.call2(&wasm_bindgen::JsValue::NULL, &path.into(), &text.into()).map_err(|e| format!("{e:?}"))?.into();
        // Grava em seguida; uma falha (o espaço acabou) aparece no console.
        wasm_bindgen_futures::spawn_local(async move {
            if let Err(e) = wasm_bindgen_futures::JsFuture::from(promise).await {
                web_sys::console::error_1(&e);
            }
        });
        Ok(())
    }

    fn list(&self, prefix: &str) -> Vec<String> {
        FILES.with(|f| f.borrow().keys().filter(|k| k.starts_with(prefix)).cloned().collect())
    }
}

/// O form em edição na lite (o projeto de um form só).
pub const FORM_PATH: &str = "forms/principal.json";

/// Um projeto do aylis em ZIP com o form: o `.aylisproj`, o form e o que mais estiver no
/// workspace (temas, traduções). A IDE de desktop abre depois de descompactar.
pub fn project_zip(workspace: &dyn Workspace, name: &str, form_json: &str) -> Result<Vec<u8>, String> {
    use std::io::Write;
    let form_name = serde_json::from_str::<serde_json::Value>(form_json)
        .ok()
        .and_then(|v| v["root"]["name"].as_str().map(str::to_string))
        .unwrap_or_else(|| "frmPrincipal".into());
    let project = serde_json::json!({
        "aylis_project": aylis_project::FORMAT_VERSION,
        "name": name,
        "language": "rust",
        "main_form": form_name,
        "forms": [{ "name": form_name, "file": FORM_PATH, "auto_create": true }],
        "locale": "pt-BR"
    });
    let mut zip = zip::ZipWriter::new(std::io::Cursor::new(Vec::new()));
    let opts = zip::write::SimpleFileOptions::default();
    let mut add = |path: &str, text: &str| -> Result<(), String> {
        zip.start_file(format!("{name}/{path}"), opts).map_err(|e| e.to_string())?;
        zip.write_all(text.as_bytes()).map_err(|e| e.to_string())
    };
    add(&format!("{name}.aylisproj"), &serde_json::to_string_pretty(&project).map_err(|e| e.to_string())?)?;
    add(FORM_PATH, form_json)?;
    for path in workspace.list("") {
        if path == FORM_PATH {
            continue;
        }
        if let Some(text) = workspace.read(&path) {
            add(&path, &text)?;
        }
    }
    Ok(zip.finish().map_err(|e| e.to_string())?.into_inner())
}

/// O caminho do ZIP de um projeto: o `.aylisproj` (em qualquer pasta dele) marca a raiz; o
/// form principal vira o form em edição ([`FORM_PATH`]) e os outros arquivos de texto (temas,
/// traduções, outros forms) vão para o workspace com o caminho a partir da raiz. Devolve o
/// JSON do form principal e quantos arquivos binários (imagens) ficaram de fora.
pub fn import_zip(workspace: &dyn Workspace, bytes: &[u8]) -> Result<(String, usize), String> {
    use std::io::Read;
    let mut zip = zip::ZipArchive::new(std::io::Cursor::new(bytes)).map_err(|e| format!("não é um ZIP: {e}"))?;
    // Os caminhos com `/` (o Compress-Archive do Windows grava com `\`); `index` acha o nome
    // original no ZIP.
    let original: Vec<String> = zip.file_names().filter_map(Result::ok).map(|n| n.to_string()).collect();
    let names: Vec<String> = original.iter().map(|n| n.replace('\\', "/")).collect();
    let index = |name: &str| names.iter().position(|n| n == name).map(|i| original[i].clone());
    let project = names.iter().filter(|n| n.ends_with(".aylisproj")).min_by_key(|n| n.matches('/').count()).ok_or("o ZIP não tem um .aylisproj")?;
    let root = project.rsplit_once('/').map_or(String::new(), |(dir, _)| format!("{dir}/"));
    let mut read = |name: &str| -> Option<Vec<u8>> {
        let mut file = zip.by_name(&index(name)?).ok()?;
        let mut data = Vec::new();
        file.read_to_end(&mut data).ok()?;
        Some(data)
    };
    let project_json: serde_json::Value = serde_json::from_slice(&read(project).ok_or("não consegui ler o .aylisproj")?).map_err(|e| format!(".aylisproj: {e}"))?;
    let main = project_json["main_form"].as_str().unwrap_or_default();
    let forms = project_json["forms"].as_array().cloned().unwrap_or_default();
    let main_file = forms
        .iter()
        .find(|f| f["name"].as_str() == Some(main))
        .or_else(|| forms.first())
        .and_then(|f| f["file"].as_str())
        .ok_or("o projeto não tem forms")?
        .replace('\\', "/");
    let form_json = String::from_utf8(read(&format!("{root}{main_file}")).ok_or_else(|| format!("o ZIP não tem {main_file}"))?).map_err(|_| format!("{main_file} não é texto"))?;
    let mut skipped = 0;
    for name in &names {
        let Some(rel) = name.strip_prefix(root.as_str()) else { continue };
        if name.ends_with('/') || name == project || rel == main_file {
            continue;
        }
        match read(name).map(String::from_utf8) {
            Some(Ok(text)) => workspace.write(rel, &text)?,
            _ => skipped += 1,
        }
    }
    workspace.write(FORM_PATH, &form_json)?;
    Ok((form_json, skipped))
}

/// Arquivo → Abrir projeto (ZIP): o navegador escolhe o arquivo e o projeto vira o form em
/// edição (o mesmo ZIP que "Baixar projeto" entrega, ou um projeto da IDE de desktop).
pub fn open_zip(ui: &mut aylis_core::Ui, state: &crate::state::Shared, ids: &crate::app::Ids) {
    PENDING.with(|p| *p.borrow_mut() = Some((state.clone(), *ids)));
    let poster = ui.poster();
    let done = Box::new(move |paths: Vec<String>| {
        poster.post(move |ui| {
            let Some((state, ids)) = PENDING.with(|p| p.borrow_mut().take()) else { return };
            let Some(path) = paths.first() else { return };
            let result = aylis_core::files::read(path).map_err(|e| e.to_string()).and_then(|bytes| {
                let ws = workspace().ok_or("sem armazenamento")?;
                import_zip(ws.as_ref(), &bytes)
            });
            let result = result.and_then(|(json, skipped)| crate::document::Document::from_json(&json).map(|doc| (doc, skipped)).map_err(|e| e.to_string()));
            {
                let mut st = state.borrow_mut();
                match result {
                    Ok((mut doc, skipped)) => {
                        doc.set_dark(st.doc.dark);
                        st.close_all();
                        st.doc = doc;
                        st.status = match skipped {
                            0 => "Projeto aberto do ZIP".into(),
                            n => format!("Projeto aberto do ZIP ({n} imagens e outros binários ficaram de fora)"),
                        };
                    }
                    Err(e) => st.status = format!("⚠ {e}"),
                }
            }
            crate::app::refresh(ui, &state, &ids);
        });
    });
    let options = aylis_core::platform::FileDialogOptions { title: "Abrir projeto (ZIP)".into(), filters: vec![("ZIP".into(), vec!["zip".into()])], ..Default::default() };
    match ui.services_mut().file_dialogs.as_mut() {
        Some(service) => service.show(options, done),
        None => done(Vec::new()),
    }
}

thread_local! {
    /// O estado da IDE à espera do ZIP escolhido (o diálogo responde de outra volta).
    static PENDING: std::cell::RefCell<Option<(crate::state::Shared, crate::app::Ids)>> = const { std::cell::RefCell::new(None) };
}

/// Entrega `bytes` ao navegador como um arquivo para baixar.
#[cfg(target_arch = "wasm32")]
pub fn download(name: &str, bytes: &[u8]) -> Result<(), String> {
    use wasm_bindgen::JsCast;
    let array = js_sys::Uint8Array::from(bytes);
    let parts = js_sys::Array::of1(&array);
    let options = web_sys::BlobPropertyBag::new();
    options.set_type("application/zip");
    let blob = web_sys::Blob::new_with_u8_array_sequence_and_options(&parts, &options).map_err(|_| "Blob")?;
    let url = web_sys::Url::create_object_url_with_blob(&blob).map_err(|_| "URL")?;
    let document = web_sys::window().and_then(|w| w.document()).ok_or("sem documento")?;
    let a: web_sys::HtmlAnchorElement = document.create_element("a").map_err(|_| "a")?.dyn_into().map_err(|_| "a")?;
    a.set_href(&url);
    a.set_download(name);
    a.click();
    let _ = web_sys::Url::revoke_object_url(&url);
    Ok(())
}

/// O workspace de onde a IDE está: o armazenamento do navegador na lite.
fn workspace() -> Option<Box<dyn Workspace>> {
    #[cfg(target_arch = "wasm32")]
    return Some(Box::new(BrowserWorkspace));
    #[cfg(not(target_arch = "wasm32"))]
    None
}

/// Guarda o form em edição no workspace (o "Salvar" da lite e a cópia automática).
/// Devolve se mudou.
pub fn save(state: &crate::state::Shared) -> bool {
    let Some(ws) = workspace() else { return false };
    let json = match state.borrow().doc.to_json() {
        Ok(j) => j,
        Err(e) => {
            state.borrow_mut().status = format!("⚠ {e}");
            return false;
        }
    };
    if ws.read(FORM_PATH).as_deref() == Some(json.as_str()) {
        return false;
    }
    let result = ws.write(FORM_PATH, &json);
    state.borrow_mut().status = match result {
        Ok(()) => "Guardado no navegador (Arquivo → Baixar projeto leva o ZIP)".into(),
        Err(e) => format!("⚠ {e}"),
    };
    true
}

/// Arquivo → Baixar projeto (ZIP): o form e o resto do workspace num projeto do aylis.
pub fn download_project(_ui: &mut aylis_core::Ui, state: &crate::state::Shared, _ids: &crate::app::Ids) {
    let Some(ws) = workspace() else { return };
    let json = state.borrow().doc.to_json();
    let result = json.and_then(|json| project_zip(ws.as_ref(), "MeuApp", &json));
    #[cfg(target_arch = "wasm32")]
    let result = result.and_then(|bytes| download("MeuApp.zip", &bytes));
    #[cfg(not(target_arch = "wasm32"))]
    let result = result.map(|_| ());
    state.borrow_mut().status = match result {
        Ok(()) => "Projeto baixado (MeuApp.zip): descompacte e abra o MeuApp.aylisproj na IDE".into(),
        Err(e) => format!("⚠ {e}"),
    };
}

/// A prévia do form na lite: no navegador não há outro processo nem outra janela, então o
/// form abre por cima da IDE, funcionando (fora do modo de design), com o tema dele.
/// Fechar volta ao designer.
pub fn preview(ui: &mut aylis_core::Ui, state: &crate::state::Shared) {
    let (json, dark) = {
        let st = state.borrow();
        (st.doc.to_json(), st.doc.dark)
    };
    let loaded = json.and_then(|json| aylis_json::load(ui, &json, None).map_err(|e| e.to_string()));
    match loaded {
        Ok(form) => {
            let _ = ui.set_prop(form, "theme", if dark { "Aylis Dark" } else { "Aylis Light" });
            // No navegador a prévia ocupa a página: o botão de voltar fica numa barra em cima,
            // e o form cresce a altura dela (senão a barra empurra e corta o fim do form).
            const BAR: f32 = 40.0;
            if let Some(cb) = ui.control_mut(form) {
                let height = cb.height();
                cb.set_height(height + BAR);
            }
            let bar = ui.add(aylis_controls::Panel::new(0.0, BAR), form);
            if let Some(cb) = ui.control_mut(bar) {
                cb.set_align(aylis_core::Align::MostTop).set_padding(4.0);
            }
            let back = ui.add(aylis_controls::Button::new(ui.tr("◀ Voltar ao designer")), bar);
            if let Some(cb) = ui.control_mut(back) {
                cb.set_align(aylis_core::Align::Left).set_width(190.0);
            }
            let _ = ui.set_prop(back, "modal_result", "ok");
            let _ = ui.show_modal(form, move |ui, _| {
                ui.free(form);
            });
            state.borrow_mut().status = "Prévia do form (feche para voltar ao designer)".into();
        }
        Err(e) => state.borrow_mut().status = format!("⚠ prévia: {e}"),
    }
}

/// Sobe a IDE lite (a página chama, pelo `aylis-ide-web`): primeiro carrega o que está guardado
/// no navegador, depois abre.
#[cfg(target_arch = "wasm32")]
pub fn run() -> Result<(), String> {
    wasm_bindgen_futures::spawn_local(async {
        if let Err(e) = BrowserWorkspace::load().await {
            web_sys::console::error_1(&format!("aylis-ide: {e} (o form não fica guardado)").into());
        }
        if let Err(e) = start() {
            web_sys::console::error_1(&format!("aylis-ide: {e}").into());
        }
    });
    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn start() -> Result<(), String> {
    use crate::document::Document;
    let workspace = BrowserWorkspace;
    // O form da última visita, se houver.
    let doc = workspace.read(FORM_PATH).and_then(|json| Document::from_json(&json).ok()).unwrap_or_else(Document::new);
    let state = crate::state::shared(doc);
    // Sem a tela inicial (ela abre e cria projetos no disco): direto ao designer.
    state.borrow_mut().home = false;
    let mut ide = crate::app::build(state.clone());
    // A cópia automática no navegador, a cada 3 s se o form mudou.
    let timer = ide.ui.add(aylis_controls::Timer::new(3000), ide.form);
    {
        let state = state.clone();
        ide.ui.on(timer, "on_timer", move |_, _, _| {
            save(&state);
        });
    }
    let (pump_state, ids) = (ide.state.clone(), ide.ids);
    aylis_platform_winit::run_with_pump(ide.ui, ide.form, Default::default(), move |waker| crate::actions::pump(pump_state, ids, waker)).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_zip_is_a_project_the_desktop_ide_opens() {
        let dir = std::env::temp_dir().join(format!("aylis-lite-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let ws = DirWorkspace(dir.clone());
        ws.write("themes/meu.json", "{}").unwrap();
        assert_eq!(ws.list("themes/"), vec!["themes/meu.json".to_string()]);
        let form = r#"{"aylis":1,"root":{"class":"Form","name":"frmTeste","width":300,"height":200}}"#;
        let bytes = project_zip(&ws, "Teste", form).unwrap();
        // Abre o ZIP extraído como projeto.
        let out = dir.join("out");
        zip::ZipArchive::new(std::io::Cursor::new(bytes.clone())).unwrap().extract(&out).unwrap();
        let project = aylis_project::Project::open(out.join("Teste/Teste.aylisproj")).unwrap();
        assert_eq!(project.main_form, "frmTeste");
        assert!(out.join("Teste/themes/meu.json").is_file());
        // E o ZIP volta para outra lite: o form em edição e o tema.
        let other = DirWorkspace(dir.join("other"));
        let (json, skipped) = import_zip(&other, &bytes).unwrap();
        assert_eq!((json.as_str(), skipped), (form, 0));
        assert_eq!(other.read(FORM_PATH).as_deref(), Some(form));
        assert_eq!(other.read("themes/meu.json").as_deref(), Some("{}"));
        assert!(import_zip(&other, b"nada").is_err());
        let _ = std::fs::remove_dir_all(&dir);
    }
}
