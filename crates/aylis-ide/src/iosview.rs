//! Projeto → Opções do projeto → iPhone (GitHub): o app de iOS compilado numa máquina macOS do
//! GitHub, sem Mac. A tela é a interface do `aylis_tools::github_ci` (o mesmo do `aylis-tool
//! ci-ios`): mostra o repositório e o workflow, cria ou atualiza o workflow, abre as páginas do
//! GitHub e, no assinado, converte o certificado e o perfil em base64 e copia o valor para colar
//! nos segredos (nada de comando para decorar).

use std::path::{Path, PathBuf};
use std::rc::Rc;

use aylis_core::{Align, ComponentId, PropValue, Ui};
use aylis_project::Project;
use aylis_tools::github_ci;

use crate::app::Ids;
use crate::dialogs::{self, combo, field};
use crate::state::Shared;
use crate::widgets::{bar, caption, label, muted, tool};

/// As duas formas de assinar: sem a assinatura da Apple (o Sideloadly assina) ou com os
/// segredos do repositório.
const SIGNING: [&str; 2] = ["Sem assinatura: instalar pelo Sideloadly (conta gratuita)", "Assinado com os segredos do repositório (conta paga)"];

/// Um texto apagado que quebra linha.
fn note(ui: &mut Ui, parent: ComponentId, text: &str, height: f32) -> ComponentId {
    let l = label(ui, parent, text, Align::Top, height);
    muted(ui, l);
    let _ = ui.set_prop(l, "wrap", true);
    let _ = ui.set_prop(l, "v_align", PropValue::Enum("leading".into()));
    l
}

/// O valor de uma linha "rótulo: valor" (que quebra linha).
fn value(ui: &mut Ui, parent: ComponentId, title: &str, height: f32) -> ComponentId {
    let line = field(ui, parent, title);
    ui.control_mut(line).unwrap().set_height(height);
    // O rótulo no alto, na linha do valor (que pode ter duas linhas).
    if let Some(&title) = ui.children(line).first() {
        let _ = ui.set_prop(title, "v_align", PropValue::Enum("leading".into()));
    }
    let l = label(ui, line, "", Align::Client, 0.0);
    let _ = ui.set_prop(l, "wrap", true);
    let _ = ui.set_prop(l, "v_align", PropValue::Enum("leading".into()));
    l
}

/// O nome de um segredo, como se digita no GitHub.
fn secret_name(ui: &mut Ui, parent: ComponentId, name: &str) {
    let l = label(ui, parent, name, Align::Top, 24.0);
    ui.control_mut(l).unwrap().set_margins([0.0, 6.0, 0.0, 0.0]);
    let _ = ui.set_prop(l, "font_weight", PropValue::Enum("bold".into()));
}

/// O que a tela mostra do repositório e do workflow agora.
struct Situation {
    root: Option<PathBuf>,
    repository: Option<String>,
    workflow: Option<PathBuf>,
    /// O texto do workflow gravado, se existe.
    written: Option<String>,
}

fn situation(project: &Project) -> Situation {
    let root = github_ci::git_root(&project.dir());
    let repository = root.as_deref().and_then(github_ci::github_repository);
    let workflow = github_ci::workflow_path(project);
    let written = workflow.as_deref().and_then(|w| std::fs::read_to_string(w).ok());
    Situation { root, repository, workflow, written }
}

fn open_url(ui: &mut Ui, error: ComponentId, url: &str) {
    if let Err(e) = aylis_tools::shell::open(url) {
        let _ = ui.set_prop(error, "text", format!("⚠ {e}"));
    }
}

pub fn open(ui: &mut Ui, state: &Shared, _ids: &Ids) {
    let Some(project) = state.borrow().project.clone() else { return };
    let d = dialogs::new_dialog(ui, "iPhone (GitHub)", 760.0, 900.0);
    let error = d.error;
    let body = d.body;

    caption(ui, body, "COMO FUNCIONA");
    note(
        ui,
        body,
        "1. Crie o workflow aqui.  2. Envie o projeto para o GitHub (commit e push).  3. No GitHub, abra o workflow e clique em Run workflow: uma máquina macOS compila o app.  4. Baixe o .ipa em Artifacts, na página da execução.  5. Instale no iPhone.",
        62.0,
    );

    caption(ui, body, "SITUAÇÃO");
    let repo_label = value(ui, body, "Repositório", 40.0);
    let workflow_label = value(ui, body, "Workflow", 40.0);

    caption(ui, body, "ASSINATURA");
    let line = field(ui, body, "Assinatura");
    let signing = combo(ui, line, &SIGNING, SIGNING[0]);
    let actions = bar(ui, body, Align::Top, 34.0);
    ui.control_mut(actions).unwrap().set_margins([0.0, 4.0, 0.0, 8.0]);
    let create = tool(ui, actions, "Criar o workflow", 200.0, "");
    let open_actions = tool(ui, actions, "Abrir no GitHub", 170.0, "secondary");
    let open_folder = tool(ui, actions, "Abrir a pasta", 150.0, "flat");
    let done = note(ui, body, "", 40.0);

    // Sem assinatura: o Sideloadly.
    let unsigned_box = ui.add(aylis_controls::Layout::default(), body);
    ui.control_mut(unsigned_box).unwrap().set_align(Align::Top).set_height(150.0);
    caption(ui, unsigned_box, "INSTALAR NO IPHONE");
    note(
        ui,
        unsigned_box,
        "O .ipa sai sem a assinatura da Apple. No Windows, abra o Sideloadly, ligue o iPhone pelo cabo, entre com o Apple ID de quem vai usar e arraste o .ipa: ele assina e instala. Com a conta gratuita, o app para de abrir em 7 dias (é só instalar de novo). No iPhone, confie no desenvolvedor em Ajustes → Geral → VPN e Gerenciamento de Dispositivos.",
        84.0,
    );
    let sideload_bar = bar(ui, unsigned_box, Align::Top, 34.0);
    let sideloadly = tool(ui, sideload_bar, "Página do Sideloadly", 200.0, "secondary");

    // Assinado: os segredos.
    let signed_box = ui.add(aylis_controls::Layout::default(), body);
    ui.control_mut(signed_box).unwrap().set_align(Align::Top).set_height(380.0);
    caption(ui, signed_box, "SEGREDOS DO REPOSITÓRIO");
    note(
        ui,
        signed_box,
        "Cadastre três segredos em Settings → Secrets and variables → Actions (New repository secret). Os botões abaixo leem o arquivo, convertem para base64 e copiam o valor: cole no campo Secret do GitHub. Exporte o certificado pelo Acesso às Chaves de um Mac (ou gere no site da Apple) e baixe o perfil no site da Apple.",
        62.0,
    );
    secret_name(ui, signed_box, "IOS_CERTIFICATE_P12");
    let line = bar(ui, signed_box, Align::Top, 34.0);
    let copy_p12 = tool(ui, line, "Copiar de um .p12…", 260.0, "secondary");
    let p12_label = label(ui, line, "", Align::Client, 0.0);
    ui.control_mut(p12_label).unwrap().set_margins([8.0, 0.0, 0.0, 0.0]);
    muted(ui, p12_label);
    secret_name(ui, signed_box, "IOS_CERTIFICATE_PASSWORD");
    note(ui, signed_box, "A senha escolhida ao exportar o .p12 (digite direto no GitHub).", 24.0);
    secret_name(ui, signed_box, "IOS_PROFILE");
    // Duas linhas: o "copiado" e o aviso do perfil que não serve ou a validade.
    let line = bar(ui, signed_box, Align::Top, 44.0);
    let copy_profile = tool(ui, line, "Copiar de um .mobileprovision…", 260.0, "secondary");
    let profile_label = label(ui, line, "", Align::Client, 0.0);
    ui.control_mut(profile_label).unwrap().set_margins([8.0, 0.0, 0.0, 0.0]);
    let _ = ui.set_prop(profile_label, "wrap", true);
    muted(ui, profile_label);
    let secrets_bar = bar(ui, signed_box, Align::Top, 34.0);
    ui.control_mut(secrets_bar).unwrap().set_margins([0.0, 12.0, 0.0, 0.0]);
    let open_secrets = tool(ui, secrets_bar, "Abrir os segredos no GitHub", 260.0, "secondary");

    let project = Rc::new(project);
    // Mostra a situação e o que cada escolha deixa fazer.
    let show = {
        let project = project.clone();
        Rc::new(move |ui: &mut Ui| {
            let s = situation(&project);
            let repo_text = match (&s.root, &s.repository) {
                (None, _) => ui.tr("⚠ O projeto não está num repositório git. Crie um (git init) e ligue-o a um repositório do GitHub."),
                (Some(root), None) => ui.tr("⚠ {0} não está ligado ao GitHub (sem o remote origin no github.com).").replace("{0}", &root.display().to_string()),
                (Some(_), Some(repo)) => format!("github.com/{repo}"),
            };
            let _ = ui.set_prop(repo_label, "text", repo_text);
            let workflow_text = match (&s.workflow, &s.written) {
                (None, _) => "—".to_string(),
                (Some(_), None) => ui.tr("Ainda não criado."),
                (Some(path), Some(text)) => {
                    let kind = if github_ci::is_signed(text) { ui.tr("assinado") } else { ui.tr("sem assinatura") };
                    let version = github_ci::workflow_aylis_ref(text).unwrap_or_default();
                    format!("{} ({kind}, Aylis {version})", path.display())
                }
            };
            let _ = ui.set_prop(workflow_label, "text", workflow_text);
            let exists = s.written.is_some();
            let _ = ui.set_prop(create, "text", ui.tr(if exists { "Atualizar o workflow" } else { "Criar o workflow" }));
            ui.control_mut(create).unwrap().set_enabled(s.root.is_some());
            ui.control_mut(open_actions).unwrap().set_enabled(s.repository.is_some() && exists);
            ui.control_mut(open_folder).unwrap().set_enabled(exists);
            ui.control_mut(open_secrets).unwrap().set_enabled(s.repository.is_some());
            let signed = ui.get::<aylis_controls::ComboBox>(signing).is_some_and(|c| c.item_index == 1);
            ui.control_mut(unsigned_box).unwrap().set_visible(!signed);
            ui.control_mut(signed_box).unwrap().set_visible(signed);
        })
    };
    // A escolha começa pela do workflow gravado.
    if situation(&project).written.as_deref().is_some_and(github_ci::is_signed) {
        let _ = ui.set_prop(signing, "item_index", 1i64);
    }
    show(ui);
    {
        let show = show.clone();
        ui.on(signing, "on_change", move |ui, _, _| show(ui));
    }
    let is_signed = move |ui: &Ui| ui.get::<aylis_controls::ComboBox>(signing).is_some_and(|c| c.item_index == 1);

    {
        let (project, show) = (project.clone(), show.clone());
        ui.on_click(create, move |ui, _| {
            let _ = ui.set_prop(error, "text", "");
            match github_ci::write_ios_workflow(&project, is_signed(ui)) {
                Ok(path) => {
                    let text = ui.tr("Workflow gravado em {0}. Agora faça o commit e o push dele, e rode em Abrir no GitHub → Run workflow.").replace("{0}", &path.display().to_string());
                    let _ = ui.set_prop(done, "text", text);
                }
                Err(e) => {
                    let _ = ui.set_prop(error, "text", format!("⚠ {e}"));
                }
            }
            show(ui);
        });
    }
    {
        let project = project.clone();
        ui.on_click(open_actions, move |ui, _| {
            let s = situation(&project);
            if let (Some(repo), Some(workflow)) = (s.repository, s.workflow) {
                open_url(ui, error, &github_ci::actions_url(&repo, &workflow));
            }
        });
    }
    {
        let project = project.clone();
        ui.on_click(open_folder, move |ui, _| {
            if let Some(dir) = github_ci::workflow_path(&project).and_then(|w| w.parent().map(Path::to_path_buf)) {
                open_url(ui, error, &dir.display().to_string());
            }
        });
    }
    {
        let project = project.clone();
        ui.on_click(open_secrets, move |ui, _| {
            if let Some(repo) = situation(&project).repository {
                open_url(ui, error, &github_ci::secrets_url(&repo));
            }
        });
    }
    ui.on_click(sideloadly, move |ui, _| open_url(ui, error, "https://sideloadly.io/"));

    // Os dois arquivos dos segredos: escolher, converter e copiar.
    {
        let state = state.clone();
        ui.on_click(copy_p12, move |_, _| {
            let state2 = state.clone();
            crate::actions::spawn(&state, async move {
                let Some(file) = crate::files::AsyncFileDialog::new().add_filter("Certificado", &["p12", "pfx"]).pick_file().await else { return };
                let path = file.path().to_path_buf();
                state2.borrow_mut().after_task = Some(Box::new(move |ui: &mut Ui| match github_ci::base64_file(&path) {
                    Ok(text) => {
                        crate::messages::copy_text(ui, &text);
                        let name = path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
                        let _ = ui.set_prop(p12_label, "text", ui.tr("{0}: copiado. Cole no segredo IOS_CERTIFICATE_P12.").replace("{0}", &name));
                    }
                    Err(e) => {
                        let _ = ui.set_prop(error, "text", format!("⚠ {e}"));
                    }
                }));
            });
        });
    }
    {
        let (state, project) = (state.clone(), project.clone());
        ui.on_click(copy_profile, move |_, _| {
            let (state2, project) = (state.clone(), project.clone());
            crate::actions::spawn(&state, async move {
                let Some(file) = crate::files::AsyncFileDialog::new().add_filter("Perfil de provisionamento", &["mobileprovision"]).pick_file().await else { return };
                let path = file.path().to_path_buf();
                state2.borrow_mut().after_task = Some(Box::new(move |ui: &mut Ui| {
                    // O perfil é conferido antes de copiar: o bundle id e a validade.
                    let profile = match aylis_tools::ios_device::read_profile(&path) {
                        Ok(p) => p,
                        Err(e) => {
                            let _ = ui.set_prop(error, "text", format!("⚠ {e}"));
                            return;
                        }
                    };
                    let bundle = project.identifier();
                    let problem = aylis_tools::ios_device::profile_problem(&profile, &bundle, None, false, &aylis_tools::ios_device::now_iso());
                    let text = match github_ci::base64_file(&path) {
                        Ok(text) => text,
                        Err(e) => {
                            let _ = ui.set_prop(error, "text", format!("⚠ {e}"));
                            return;
                        }
                    };
                    crate::messages::copy_text(ui, &text);
                    let mut info = ui.tr("{0}: copiado. Cole no segredo IOS_PROFILE.").replace("{0}", &profile.name);
                    match problem {
                        Some(p) => info.push_str(&format!("\n⚠ {} {p}", ui.tr("Este perfil não serve para o app:"))),
                        None => info.push_str(&format!("\n{} {}", ui.tr("Vale até"), profile.expires)),
                    }
                    let _ = ui.set_prop(profile_label, "text", info);
                }));
            });
        });
    }

    // Nada a aplicar no OK: cada botão já faz o que diz.
    dialogs::run(ui, d, |_| Ok(()), |_| {});
}
