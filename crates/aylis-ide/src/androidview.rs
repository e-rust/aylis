//! Projeto → Opções do projeto → Android: o que vai no manifesto (versão, SDK, orientação, tela, permissões,
//! recursos e links que abrem o app) e o manifesto final como sai no publicar. As permissões
//! que os componentes pedem aparecem com o motivo e não saem; as que a Play Store restringe
//! têm aviso. O XML a mais fica em `android/manifest-extra.xml` (mesclado e conferido).

use aylis_controls::{CheckBox, DlgKind, message_dlg};
use aylis_core::{Align, ComponentId, ModalResult, PropValue, Ui};
use aylis_project::AndroidOptions;
use aylis_tools::publish::{ANDROID_MANIFEST_EXTRA, ANDROID_RESTRICTED, android_manifest_preview, android_required_permissions};

use crate::app::{Ids, refresh};
use crate::dialogs::{self, combo, edit, field, text_of};
use crate::state::Shared;
use crate::widgets::{label, muted};

const ORIENTATIONS: [&str; 6] = ["(gira com o aparelho)", "portrait", "landscape", "sensorPortrait", "sensorLandscape", "fullSensor"];

fn check(ui: &mut Ui, parent: ComponentId, text: &str, on: bool) -> ComponentId {
    let c = ui.add(CheckBox::new(text), parent);
    ui.control_mut(c).unwrap().set_align(Align::Client);
    let _ = ui.set_prop(c, "checked", on);
    c
}

fn checked(ui: &Ui, id: ComponentId) -> bool {
    matches!(ui.get_prop(id, "checked"), Ok(PropValue::Bool(true)))
}

fn number(text: &str) -> Result<u32, String> {
    let t = text.trim();
    if t.is_empty() { Ok(0) } else { t.parse().map_err(|_| format!("'{t}' não é um número")) }
}

/// A lista separada por `;` (sem vazios).
fn list(text: &str) -> Vec<String> {
    text.split(';').map(str::trim).filter(|s| !s.is_empty()).map(str::to_string).collect()
}

pub fn open(ui: &mut Ui, state: &Shared, ids: &Ids) {
    let Some(project) = state.borrow().project.clone() else { return };
    let a = project.publish.android.clone();
    let d = dialogs::new_dialog(ui, "Android", 680.0, 720.0);
    let line = field(ui, d.body, "Identificador");
    let id = label(ui, line, &format!("{}   (Projeto → Opções)", project.identifier()), Align::Client, 0.0);
    muted(ui, id);
    let line = field(ui, d.body, "versionCode");
    let version_code = edit(ui, line, &if a.version_code == 0 { String::new() } else { a.version_code.to_string() });
    let _ = ui.set_prop(version_code, "placeholder", format!("automático ({})", project.version()));
    let line = field(ui, d.body, "minSdk / targetSdk");
    let min_sdk = edit(ui, line, &if a.min_sdk == 0 { String::new() } else { a.min_sdk.to_string() });
    let _ = ui.set_prop(min_sdk, "placeholder", "26");
    let target_sdk = edit(ui, line, &if a.target_sdk == 0 { String::new() } else { a.target_sdk.to_string() });
    let _ = ui.set_prop(target_sdk, "placeholder", "34");
    let line = field(ui, d.body, "Orientação");
    let current = if a.orientation.is_empty() { ORIENTATIONS[0] } else { &a.orientation };
    let orientation = combo(ui, line, &ORIENTATIONS, current);
    let line = field(ui, d.body, "Tela");
    let fullscreen = check(ui, line, "Tela cheia", a.fullscreen);
    let line = field(ui, d.body, "");
    let keep_on = check(ui, line, "Manter a tela acesa", a.keep_screen_on);
    let line = field(ui, d.body, "");
    let tablet = check(ui, line, "Só tablets", a.tablet_only);
    // As dos componentes: marcadas e travadas, com o motivo.
    let required = android_required_permissions(&project);
    let line = field(ui, d.body, "Permissões dos componentes");
    let text: Vec<String> = required.iter().map(|(p, why)| format!("☑ {} — {why}", p.trim_start_matches("android.permission."))).collect();
    let l = label(ui, line, &text.join("\n"), Align::Client, 0.0);
    let _ = ui.set_prop(l, "wrap", true);
    muted(ui, l);
    ui.control_mut(line).unwrap().set_height((text.len() as f32 * 18.0).max(30.0));
    let line = field(ui, d.body, "Mais permissões");
    let permissions = edit(ui, line, &a.permissions.join("; "));
    let _ = ui.set_prop(permissions, "placeholder", "CALL_PHONE; VIBRATE");
    let line = field(ui, d.body, "Recursos (uses-feature)");
    let features = edit(ui, line, &a.features.iter().map(|(n, r)| format!("{n}{}", if *r { "" } else { ":opcional" })).collect::<Vec<_>>().join("; "));
    let _ = ui.set_prop(features, "placeholder", "android.hardware.nfc; android.hardware.telephony:opcional");
    let line = field(ui, d.body, "Links que abrem o app");
    let links = edit(ui, line, &a.deep_links.join("; "));
    let _ = ui.set_prop(links, "placeholder", "https://empresa.com/pedido; meuapp://");
    let line = field(ui, d.body, "XML a mais");
    let extra = label(ui, line, &format!("{ANDROID_MANIFEST_EXTRA} (mesclado no publicar)"), Align::Client, 0.0);
    muted(ui, extra);
    let line = field(ui, d.body, "");
    let show = ui.add(aylis_controls::Button::new("Ver o manifesto final"), line);
    ui.control_mut(show).unwrap().set_align(Align::Left).set_width(200.0);

    let abis = a.abis.clone();
    let read = move |ui: &Ui| -> Result<AndroidOptions, String> {
        let orientation = dialogs::combo_text(ui, orientation);
        let features = list(&text_of(ui, features))
            .into_iter()
            .map(|f| match f.strip_suffix(":opcional") {
                Some(name) => (name.trim().to_string(), false),
                None => (f, true),
            })
            .collect();
        Ok(AndroidOptions {
            version_code: number(&text_of(ui, version_code))?,
            min_sdk: number(&text_of(ui, min_sdk))?,
            target_sdk: number(&text_of(ui, target_sdk))?,
            orientation: if orientation == ORIENTATIONS[0] { String::new() } else { orientation },
            fullscreen: checked(ui, fullscreen),
            keep_screen_on: checked(ui, keep_on),
            tablet_only: checked(ui, tablet),
            permissions: list(&text_of(ui, permissions)),
            features,
            deep_links: list(&text_of(ui, links)),
            // As do projeto (`publish.android.abis`, pelo `set_project_setting`) ficam como estão.
            abis: abis.clone(),
        })
    };
    {
        let (read, project) = (read.clone(), project.clone());
        ui.on_click(show, move |ui, _| {
            let mut p = project.clone();
            match read(ui) {
                Ok(a) => p.publish.android = a,
                Err(e) => {
                    message_dlg(ui, &e, DlgKind::Error, &[ModalResult::Ok], |_, _| {});
                    return;
                }
            }
            let (text, kind) = match android_manifest_preview(&p) {
                Ok(m) => (m, DlgKind::Information),
                Err(e) => (e, DlgKind::Error),
            };
            message_dlg(ui, &text, kind, &[ModalResult::Ok], |_, _| {});
        });
    }
    let (state, ids, validate) = (state.clone(), *ids, read.clone());
    dialogs::run(
        ui,
        d,
        move |ui| {
            let a = validate(ui)?;
            let full = |p: &str| if p.contains('.') { p.to_string() } else { format!("android.permission.{p}") };
            let restricted: Vec<String> = a.permissions.iter().map(|p| full(p)).filter(|p| ANDROID_RESTRICTED.contains(&p.as_str())).collect();
            if !restricted.is_empty() {
                // Só avisa: fora da Play Store vale sem restrição.
                eprintln!("aylis-ide: permissões restritas pela Play Store: {}", restricted.join(", "));
            }
            Ok(())
        },
        move |ui| {
            let Ok(a) = read(ui) else { return };
            let restricted: Vec<String> = a
                .permissions
                .iter()
                .map(|p| if p.contains('.') { p.clone() } else { format!("android.permission.{p}") })
                .filter(|p| ANDROID_RESTRICTED.contains(&p.as_str()))
                .collect();
            let mut st = state.borrow_mut();
            let Some(mut p) = st.project.clone() else { return };
            p.publish.android = a;
            st.status = match p.save() {
                Ok(()) if restricted.is_empty() => "Opções do Android salvas".into(),
                Ok(()) => format!("⚠ A Play Store restringe {} (só aceita em apps que precisam; fora dela vale)", restricted.join(", ").replace("android.permission.", "")),
                Err(e) => format!("⚠ {e}"),
            };
            st.project = Some(p);
            drop(st);
            refresh(ui, &state, &ids);
        },
    );
}
