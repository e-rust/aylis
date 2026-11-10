//! Os pacotes de componente na IDE: instalar um `.aylispkg` (com a pergunta de confiança),
//! criar um a partir do form aberto e a tela "Pacotes" (as versões instaladas, a que o
//! projeto usa, usar outra e remover para a Lixeira).

use std::path::{Path, PathBuf};

use aylis_controls::{DlgKind, message_dlg};
use aylis_core::{ModalResult, Ui};
use aylis_project::packages::{ExposedEvent, ExposedProp, Manifest};
use aylis_tools::packages::{self as pkg, Trust};

use crate::app::{Ids, refresh};
use crate::dialogs;
use crate::state::Shared;

/// Ferramentas → Pacotes de componentes → Instalar pacote…: escolhe o arquivo.
pub fn install_dialog(ui: &mut Ui, state: &Shared, ids: &Ids) {
    let Some(file) = crate::files::FileDialog::new().add_filter("Pacote do Aylis", &[pkg::EXTENSION]).pick_file() else { return };
    install_file(ui, state, ids, &file);
}

fn status(ui: &mut Ui, state: &Shared, ids: &Ids, text: String) {
    state.borrow_mut().status = text;
    refresh(ui, state, ids);
}

/// Instala um `.aylispkg` (também pelos dois cliques no arquivo): mostra o que vem nele e
/// quem assinou, e só instala com o OK.
pub fn install_file(ui: &mut Ui, state: &Shared, ids: &Ids, file: &Path) {
    let package = match pkg::read(file) {
        Ok(p) => p,
        Err(e) => {
            message_dlg(ui, &format!("Não deu para abrir o pacote:\n{e}"), DlgKind::Error, &[ModalResult::Ok], |_, _| {});
            return;
        }
    };
    let m = &package.manifest;
    let trust = pkg::trust_of(&package);
    let (kind, who) = match &trust {
        Trust::Known => (DlgKind::Confirmation, "Assinado pelo autor (a mesma chave das outras vezes).".to_string()),
        Trust::NewAuthor { fingerprint } => (
            DlgKind::Confirmation,
            format!("Primeira vez deste autor. Confira com ele a impressão digital da chave:\n{fingerprint}"),
        ),
        Trust::Changed { expected, got } => (
            DlgKind::Warning,
            format!("ATENÇÃO: este autor já veio com outra chave.\nEsperada: {expected}\nVeio: {got}\nPode ser outra pessoa usando o mesmo nome."),
        ),
        Trust::Unsigned => (DlgKind::Warning, "Sem assinatura: não dá para saber quem fez o pacote.".to_string()),
    };
    // Um pacote de código roda no computador ao compilar (build.rs) e dentro do app.
    let (kind, who) = if m.is_code() {
        (DlgKind::Warning, format!("{who}\n\nEste pacote é CÓDIGO (um crate Rust): roda no seu computador ao compilar e dentro do app. Instale só de quem você confia."))
    } else {
        (kind, who)
    };
    let mut contents = package.contents();
    if contents.len() > 12 {
        let more = contents.len() - 12;
        contents.truncate(12);
        contents.push(format!("… e mais {more}"));
    }
    let text = format!(
        "Instalar o pacote {} {} de {}?\n{}\n\n{who}\n\nTipo: {} · {} propriedade(s) exposta(s)\nConteúdo:\n  {}",
        m.name,
        m.version,
        if m.author.is_empty() { "(autor sem nome)" } else { &m.author },
        m.description,
        if m.kind == "code" { "código (Rust)" } else { "composto (form)" },
        m.properties.len(),
        contents.join("\n  ")
    );
    let (state, ids) = (state.clone(), *ids);
    message_dlg(ui, &text, kind, &[ModalResult::Yes, ModalResult::No], move |ui, result| {
        if result != ModalResult::Yes {
            return;
        }
        if matches!(trust, Trust::NewAuthor { .. } | Trust::Changed { .. }) {
            let _ = pkg::trust_author(&package);
        }
        match pkg::install(&package) {
            Ok(dir) => {
                let installed = pkg::Installed { manifest: package.manifest.clone(), dir };
                offer_use(ui, &state, &ids, installed);
            }
            Err(e) => status(ui, &state, &ids, format!("⚠ pacote: {e}")),
        }
    });
}

/// Depois de instalar: usar no projeto aberto (trocando a versão, se ele já usa outra).
fn offer_use(ui: &mut Ui, state: &Shared, ids: &Ids, installed: pkg::Installed) {
    let Some(dir) = state.borrow().project.as_ref().map(|p| p.dir()) else {
        status(ui, state, ids, format!("Pacote {} {} instalado (abra um projeto para usar)", installed.manifest.name, installed.manifest.version));
        return;
    };
    let current = pkg::project_version(&dir, &installed.manifest.name);
    let question = match &current {
        Some(c) if c.version == installed.manifest.version => {
            status(ui, state, ids, format!("Pacote {} {} instalado (o projeto já usa esta versão)", installed.manifest.name, installed.manifest.version));
            return;
        }
        Some(c) => format!("O projeto usa {} {}. Atualizar este projeto para a {}?", c.name, c.version, installed.manifest.version),
        None => format!("Usar {} {} neste projeto? Ele aparece na paleta.", installed.manifest.name, installed.manifest.version),
    };
    let (state, ids) = (state.clone(), *ids);
    message_dlg(ui, &question, DlgKind::Confirmation, &[ModalResult::Yes, ModalResult::No], move |ui, result| {
        if result == ModalResult::Yes {
            use_in_project(ui, &state, &ids, &dir, &installed);
        }
    });
}

fn use_in_project(ui: &mut Ui, state: &Shared, ids: &Ids, dir: &Path, installed: &pkg::Installed) {
    match pkg::use_in_project(dir, installed) {
        Ok(notes) => {
            state.borrow_mut().doc.refresh_frames();
            if !notes.is_empty() {
                message_dlg(ui, &format!("Versão maior: confira os forms que usam o pacote.\n\n{}", notes.join("\n")), DlgKind::Warning, &[ModalResult::Ok], |_, _| {});
            }
            status(ui, state, ids, format!("{} {} no projeto (paleta: aba {})", installed.manifest.name, installed.manifest.version, if installed.manifest.category.is_empty() { "Pacotes" } else { &installed.manifest.category }));
            // O de código: a dependência entra no Cargo.toml e o crate compila agora (os
            // erros dele aparecem já, não no primeiro F9).
            if installed.manifest.is_code() {
                crate::codewindow::save_all(ui);
                crate::jobs::build(state, false);
            }
        }
        Err(e) => status(ui, state, ids, format!("⚠ pacote: {e}")),
    }
}

/// Projeto → Criar pacote deste form…: nome, versão, autor, aba da paleta e as propriedades
/// expostas (`nome = componente.propriedade : tipo`, uma por linha); grava o `.aylispkg`
/// assinado com a chave do autor.
pub fn create_dialog(ui: &mut Ui, state: &Shared, ids: &Ids) {
    let (form_name, form) = {
        let st = state.borrow();
        (st.doc.form_name(), aylis_json::save_value(&st.doc.ui, st.doc.root).ok())
    };
    let Some(form) = form else { return };
    let author = crate::actions::config_text("package_author");
    let d = dialogs::new_dialog(ui, "Criar pacote deste form", 520.0, 420.0);
    let line = dialogs::field(ui, d.body, "Nome");
    let name = dialogs::edit(ui, line, &form_name.replace(|c: char| !c.is_alphanumeric() && c != '_', ""));
    let line = dialogs::field(ui, d.body, "Versão");
    let version = dialogs::edit(ui, line, "1.0.0");
    let line = dialogs::field(ui, d.body, "Autor");
    let author_e = dialogs::edit(ui, line, &author);
    let line = dialogs::field(ui, d.body, "Aba da paleta");
    let category = dialogs::edit(ui, line, "Pacotes");
    let line = dialogs::field(ui, d.body, "Descrição");
    let description = dialogs::edit(ui, line, "");
    let line = dialogs::field(ui, d.body, "Expostas");
    let props = dialogs::edit(ui, line, "");
    let _ = ui.set_prop(props, "placeholder", "valor = pbNivel.value : number; titulo = lblTitulo.text; on_apertou = btn.on_click");
    let read = move |ui: &Ui| -> Result<Manifest, String> {
        let mut m = Manifest {
            name: dialogs::text_of(ui, name).trim().to_string(),
            version: dialogs::text_of(ui, version).trim().to_string(),
            author: dialogs::text_of(ui, author_e).trim().to_string(),
            category: dialogs::text_of(ui, category).trim().to_string(),
            description: dialogs::text_of(ui, description).trim().to_string(),
            kind: "composite".into(),
            ..Default::default()
        };
        for item in dialogs::text_of(ui, props).split([';', '\n']).map(str::trim).filter(|s| !s.is_empty()) {
            let (prop, target) = item.split_once('=').ok_or_else(|| format!("'{item}': use nome = componente.propriedade"))?;
            let (target, kind) = target.split_once(':').map_or((target, "text"), |(t, k)| (t, k.trim()));
            if !target.contains('.') {
                return Err(format!("'{item}': o alvo é componente.propriedade"));
            }
            // `on_...` é um evento exposto (o de um componente de dentro).
            if prop.trim().starts_with("on_") {
                m.events.push(ExposedEvent { name: prop.trim().to_string(), target: target.trim().to_string() });
                continue;
            }
            m.properties.push(ExposedProp { name: prop.trim().to_string(), kind: kind.to_string(), target: target.trim().to_string(), ..Default::default() });
        }
        // Valida pelo próprio formato do manifesto.
        Manifest::parse(&serde_json::to_string(&m).map_err(|e| e.to_string())?)
    };
    let check = read.clone();
    let (state, ids) = (state.clone(), *ids);
    dialogs::run(ui, d, move |ui| check(ui).map(|_| ()), move |ui| {
        let Ok(m) = read(ui) else { return };
        crate::actions::update_config("package_author", serde_json::Value::from(m.author.clone()));
        let start = state.borrow().project.as_ref().map(|p| p.dir()).unwrap_or_else(|| PathBuf::from("."));
        let Some(out) = crate::files::FileDialog::new().add_filter("Pacote do Aylis", &[pkg::EXTENSION]).set_directory(start).set_file_name(format!("{}-{}.{}", m.name, m.version, pkg::EXTENSION)).save_file() else { return };
        let mut form = form.clone();
        form["root"]["name"] = serde_json::Value::String(m.name.clone());
        let result = pkg::author_key().and_then(|key| pkg::pack(&out, &m, &form, &[], &key).map(|_| key));
        match result {
            Ok(key) => status(ui, &state, &ids, format!("Pacote {} {} criado: {} · chave {}", m.name, m.version, out.display(), pkg::fingerprint(key.verifying_key().as_bytes()))),
            Err(e) => status(ui, &state, &ids, format!("⚠ pacote: {e}")),
        }
    });
}

/// Ferramentas → Pacotes de componentes → Pacotes…: os instalados, as versões de cada um e a que o projeto usa.
pub fn manage_dialog(ui: &mut Ui, state: &Shared, ids: &Ids) {
    let project = state.borrow().project.as_ref().map(|p| p.dir());
    let groups = pkg::installed();
    let mut items = vec!["Instalar pacote…".to_string()];
    let mut targets: Vec<Option<pkg::Installed>> = vec![None];
    for (name, versions) in &groups {
        let used = project.as_ref().and_then(|d| pkg::project_version(d, name)).map(|m| m.version);
        for v in versions {
            let mark = if used.as_deref() == Some(v.manifest.version.as_str()) { "  ✓ no projeto" } else { "" };
            items.push(format!("{name} {}   — {}{mark}", v.manifest.version, if v.manifest.author.is_empty() { "?" } else { &v.manifest.author }));
            targets.push(Some(v.clone()));
        }
    }
    let (state, ids) = (state.clone(), *ids);
    crate::codewindow::quick_pick(ui, "Pacotes instalados", items, move |ui, i| {
        let Some(Some(installed)) = targets.get(i).cloned() else {
            install_dialog(ui, &state, &ids);
            return;
        };
        let mut actions = vec![ModalResult::Yes, ModalResult::No, ModalResult::Cancel];
        let in_use = project.as_ref().map(|d| pkg::forms_using(d, &installed.manifest.name)).unwrap_or_default();
        let text = format!(
            "{} {}\n\nSim: usar esta versão no projeto aberto.\nNão: remover esta versão (vai para a Lixeira).{}",
            installed.manifest.name,
            installed.manifest.version,
            if in_use.is_empty() { String::new() } else { format!("\n\nForms do projeto que usam o pacote: {}", in_use.join(", ")) }
        );
        if project.is_none() {
            actions.remove(0);
        }
        let (state, ids, project) = (state.clone(), ids, project.clone());
        message_dlg(ui, &text, DlgKind::Confirmation, &actions, move |ui, result| match result {
            ModalResult::Yes => {
                if let Some(dir) = &project {
                    use_in_project(ui, &state, &ids, dir, &installed);
                }
            }
            ModalResult::No => match pkg::remove(&installed) {
                Ok(()) => status(ui, &state, &ids, format!("{} {} removido (está na Lixeira)", installed.manifest.name, installed.manifest.version)),
                Err(e) => status(ui, &state, &ids, format!("⚠ não deu para remover: {e}")),
            },
            _ => {}
        });
    });
}
