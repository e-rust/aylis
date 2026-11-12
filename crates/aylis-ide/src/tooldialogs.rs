//! Diálogos do Executar: "Verificar ambiente" (com o botão Corrigir de cada item) e
//! "Publicar".

use std::path::PathBuf;

use aylis_controls::{Button, CheckBox, ComboBox, Form, Label, Layout, ScrollBox};
use aylis_core::{Align, ComponentId, Ui};
use aylis_project::{Language, Project};
use aylis_tools::Target;
use aylis_tools::env::Status;

use crate::app::{Ids, refresh};
use crate::dialogs::{combo, combo_text, edit, field, new_dialog, run, text_of};
use crate::jobs;
use crate::state::Shared;
use crate::widgets::{caption, label, muted};

/// O diálogo de ambiente aberto (para preencher quando a verificação termina).
#[derive(Clone, Copy)]
pub struct EnvView {
    pub form: ComponentId,
    pub list: ComponentId,
    pub log: ComponentId,
    pub recheck: ComponentId,
}

/// Abre o "Verificar ambiente". Com `check`, verifica de novo (senão mostra o que já foi
/// verificado, como depois de um F9 que parou por falta de algo).
pub fn environment(ui: &mut Ui, state: &Shared, ids: &Ids, check: bool) {
    if state.borrow().env_view.is_some() {
        if check {
            jobs::check_environment(state);
        }
        fill_environment(ui, state, ids);
        return;
    }
    let form = ui.add_root(Form::new("Verificar ambiente", 820.0, 600.0));
    let _ = ui.set_name(form, "frmAmbiente");
    let _ = ui.set_prop(form, "border_style", "single");
    ui.control_mut(form).unwrap().set_padding(16.0);
    let top = ui.add(Layout::default(), form);
    ui.control_mut(top).unwrap().set_align(Align::Top).set_height(34.0).set_margins([0.0, 0.0, 0.0, 8.0]);
    let recheck = ui.add(Button::new("Verificar de novo"), top);
    ui.control_mut(recheck).unwrap().set_align(Align::Right).set_width(150.0).set_style("flat");
    let language = state.borrow().project.as_ref().map_or(Language::Rust, |p| p.language);
    let intro = label(
        ui,
        top,
        &format!("O que cada plataforma precisa (projeto em {}). Corrigir mostra o comando e pergunta antes de rodar.", language.name()),
        Align::Client,
        0.0,
    );
    muted(ui, intro);
    let footer = ui.add(Layout::default(), form);
    ui.control_mut(footer).unwrap().set_align(Align::Bottom).set_height(36.0).set_margins([0.0, 12.0, 0.0, 0.0]);
    let close = ui.add(Button::new("Fechar"), footer);
    ui.control_mut(close).unwrap().set_align(Align::Right).set_width(110.0);
    let _ = ui.set_prop(close, "modal_result", "ok");
    let log = label(ui, footer, "", Align::Client, 0.0);
    muted(ui, log);
    let list = ui.add(ScrollBox::default(), form);
    ui.control_mut(list).unwrap().set_align(Align::Client);

    state.borrow_mut().env_view = Some(EnvView { form, list, log, recheck });
    {
        let (state, ids) = (state.clone(), *ids);
        ui.on_click(recheck, move |ui, _| {
            jobs::check_environment(&state);
            fill_environment(ui, &state, &ids);
        });
    }
    let (state2, ids2) = (state.clone(), *ids);
    crate::lang::translate(ui);
    let _ = ui.show_modal(form, move |ui, _| {
        state2.borrow_mut().env_view = None;
        ui.free(form);
        refresh(ui, &state2, &ids2);
    });
    if check || state.borrow().checks.is_empty() {
        jobs::check_environment(state);
    }
    fill_environment(ui, state, ids);
}

/// Refaz a lista do diálogo de ambiente (se estiver aberto).
pub fn fill_environment(ui: &mut Ui, state: &Shared, ids: &Ids) {
    let Some(view) = state.borrow().env_view else { return };
    let (checks, busy, log, run_target) = {
        let st = state.borrow();
        let busy = st.env_job.as_ref().map(|j| j.title.clone());
        (st.checks.clone(), busy, st.env_log.clone(), st.run_target)
    };
    let status = match &busy {
        Some(title) if log.is_empty() => format!("{title}…"),
        Some(title) => format!("{title}… {log}"),
        None => log,
    };
    let _ = ui.set_prop(view.log, "text", status);
    let _ = ui.set_prop(view.recheck, "enabled", busy.is_none());
    let children = ui.children(view.list).to_vec();
    ui.free_many(&children);
    if checks.is_empty() {
        let l = label(ui, view.list, "Verificando…", Align::Top, 30.0);
        muted(ui, l);
        ui.layout();
        return;
    }
    let (ok, warn, bad) = (ui.theme().color("success"), ui.theme().color("warning"), ui.theme().color("danger"));
    // A plataforma do Executar primeiro.
    let mut targets: Vec<Target> = Target::ALL.to_vec();
    targets.sort_by_key(|t| *t != run_target);
    for target in targets {
        let items: Vec<_> = checks.iter().filter(|c| c.target == target).collect();
        if items.is_empty() {
            continue;
        }
        let ready = items.iter().all(|c| c.status != Status::Missing);
        let title = format!("{}  —  {}", target.name().to_uppercase(), if ready { "pronto" } else { "falta algo" });
        let c = caption(ui, view.list, &title);
        let _ = ui.set_prop(c, "color", if ready { ok } else { bad });
        for check in items {
            let line = ui.add(Layout::default(), view.list);
            ui.control_mut(line).unwrap().set_align(Align::Top).set_height(30.0);
            let (glyph, color) = match check.status {
                Status::Ok => ("✓", ok),
                Status::Warning => ("!", warn),
                Status::Missing => ("✗", bad),
            };
            let g = label(ui, line, glyph, Align::Left, 26.0);
            let _ = ui.set_prop(g, "color", color);
            let _ = ui.set_prop(g, "bold", true);
            let n = label(ui, line, &check.name, Align::Left, 230.0);
            if check.status != Status::Ok {
                let _ = ui.set_prop(n, "bold", true);
            }
            if let (Some(fix), true) = (&check.fix, check.status != Status::Ok) {
                let b = ui.add(Button::new("Corrigir…"), line);
                ui.control_mut(b).unwrap().set_align(Align::Right).set_width(96.0).set_margins([8.0, 2.0, 0.0, 2.0]);
                let _ = ui.set_prop(b, "enabled", busy.is_none());
                // O que este botão corrige: um "Corrigir…" por item, e o MCP (`dialog_state`)
                // separa cada um pela dica (M11 do Tabernáculo).
                let _ = ui.set_prop(b, "hint", format!("{} ({})", check.name, target.name()));
                let (state, ids, fix) = (state.clone(), *ids, fix.clone());
                ui.on_click(b, move |ui, _| confirm_fix(ui, &state, &ids, fix.clone()));
            }
            let d = ui.add(Label::new(check.detail.as_str()), line);
            ui.control_mut(d).unwrap().set_align(Align::Client);
            muted(ui, d);
            let extra = match (&check.fix, &check.help) {
                (_, Some(help)) if check.status != Status::Ok => Some(help.clone()),
                (Some(fix), _) if check.status != Status::Ok => Some(format!("{}:  {}", fix.description, fix.command_line())),
                _ => None,
            };
            if let Some(text) = extra {
                let h = label(ui, view.list, &text, Align::Top, 22.0);
                ui.control_mut(h).unwrap().set_margins([26.0, 0.0, 0.0, 4.0]);
                let _ = ui.set_prop(h, "font_size", 12);
                muted(ui, h);
            }
        }
    }
    ui.layout();
}

/// Mostra o comando e pergunta antes de rodar uma correção.
fn confirm_fix(ui: &mut Ui, state: &Shared, ids: &Ids, fix: aylis_tools::env::Fix) {
    let state2 = state.clone();
    let question = format!("{}\n\nComando:\n{}\n\nRodar agora?", fix.description, fix.command_line());
    crate::actions::confirm(ui, state, ids, &question, move |_| jobs::apply_fix(&state2, fix));
}

// ---------------------------------------------------------------------------
// Publicar

/// "Publicar": plataforma, nome, versão, identificador, ícone e pasta de saída. As opções
/// ficam gravadas no `.aylisproj`.
pub fn publish(ui: &mut Ui, state: &Shared, ids: &Ids) {
    let Some(project) = state.borrow().project.clone() else {
        state.borrow_mut().status = "Abra um projeto para publicar".into();
        return;
    };
    if state.borrow().job.is_some() {
        state.borrow_mut().status = "Pare o programa em execução antes de publicar (Ctrl+F2)".into();
        return;
    }
    let d = new_dialog(ui, "Publicar", 640.0, 650.0);
    let _ = ui.set_name(d.form, "frmPublicar");
    let names: Vec<&str> = Target::ALL.iter().map(|t| t.name()).collect();
    let line = field(ui, d.body, "Plataforma");
    let target = combo(ui, line, &names, state.borrow().run_target.name());
    let line = field(ui, d.body, "Nome do aplicativo");
    let app_name = edit(ui, line, project.app_name());
    let line = field(ui, d.body, "Versão");
    let version = edit(ui, line, project.version());
    let line = field(ui, d.body, "Identificador");
    let identifier = edit(ui, line, &project.identifier());
    let line = field(ui, d.body, "Ícone (PNG)");
    let browse = ui.add(Button::new("…"), line);
    ui.control_mut(browse).unwrap().set_align(Align::Right).set_width(36.0).set_margins([6.0, 0.0, 0.0, 0.0]).set_style("flat");
    let icon = edit(ui, line, &project.publish.icon);
    let _ = ui.set_prop(icon, "placeholder", "assets/images/icone.png");
    let line = field(ui, d.body, "Pasta de saída");
    let out = edit(ui, line, "");
    let _ = ui.set_prop(out, "placeholder", "dist/<plataforma> (na pasta do projeto)");
    // Android: o formato e a chave do desenvolvedor; Windows: o instalador.
    let line = field(ui, d.body, "Servidor");
    let backend = edit(ui, line, &project.publish.backend_url);
    let _ = ui.set_prop(backend, "placeholder", "outras linguagens no celular e no navegador: wss://servidor:9443");
    let line = field(ui, d.body, "Pacote");
    let bundle = ui.add(CheckBox::new("Android: AAB (Play Store)"), line);
    ui.control_mut(bundle).unwrap().set_align(Align::Left).set_width(230.0);
    let installer = ui.add(CheckBox::new("Windows: instalador (o tipo do projeto)"), line);
    ui.control_mut(installer).unwrap().set_align(Align::Client);
    let line = field(ui, d.body, "Chave (Android)");
    let new_key = ui.add(Button::new("Criar…"), line);
    ui.control_mut(new_key).unwrap().set_align(Align::Right).set_width(80.0).set_margins([6.0, 0.0, 0.0, 0.0]).set_style("flat");
    let keystore = edit(ui, line, &project.publish.keystore);
    let _ = ui.set_prop(keystore, "placeholder", "vazio: chave de depuração (não vale na Play Store)");
    let line = field(ui, d.body, "Apelido da chave");
    let alias = edit(ui, line, &project.publish.key_alias);
    let line = field(ui, d.body, "Senha da chave");
    let password = edit(ui, line, "");
    let _ = ui.set_prop(password, "password", true);
    let _ = ui.set_prop(password, "placeholder", "não fica gravada");
    {
        let (dir, name) = (project.dir(), project.app_name().to_string());
        ui.on_click(new_key, move |ui, _| {
            let file = text_of(ui, keystore);
            let file = if file.is_empty() { "chave-android.jks".to_string() } else { file };
            let alias_text = Some(text_of(ui, alias)).filter(|a| !a.is_empty()).unwrap_or_else(|| "app".into());
            let pass = text_of(ui, password);
            let path = dir.join(&file);
            let mut log = Vec::new();
            let result = aylis_tools::publish::create_keystore(&path, &alias_text, &pass, &name, &aylis_tools::Control::new(), &mut |e| log.push(format!("{e:?}")));
            match result {
                Ok(()) => {
                    let _ = ui.set_prop(keystore, "text", file.replace('\\', "/"));
                    let _ = ui.set_prop(alias, "text", alias_text);
                    aylis_controls::toast(ui, "Chave criada: guarde o arquivo e a senha (a Play Store exige a mesma chave nas atualizações)");
                }
                Err(e) => aylis_controls::toast(ui, &format!("⚠ {e} (informe a senha antes de criar)")),
            }
        });
    }
    let hint = label(
        ui,
        d.body,
        "Rust: um executável só, APK/AAB, pacote web ou .app. Outras linguagens: desktop com o aylis-host; no celular e no navegador o app conecta no Servidor (aylis-host --bridge lá).",
        Align::Top,
        40.0,
    );
    let _ = ui.set_prop(hint, "wrap", true);
    let _ = ui.set_prop(hint, "font_size", 12);
    muted(ui, hint);
    {
        let (state, dir) = (state.clone(), project.dir());
        ui.on_click(browse, move |_, _| pick_icon(&state, dir.clone(), icon));
    }
    let original = project.clone();
    let (state2, ids2) = (state.clone(), *ids);
    run(
        ui,
        d,
        move |ui| {
            let v = text_of(ui, version);
            if v.is_empty() || !v.split('.').all(|p| !p.is_empty() && p.chars().all(|c| c.is_ascii_digit())) {
                return Err("versão no formato 1.0.0".into());
            }
            let id = text_of(ui, identifier);
            let valid_id = id.split('.').count() >= 2
                && id.split('.').all(|p| p.chars().next().is_some_and(|c| c.is_ascii_alphabetic()) && p.chars().all(|c| c.is_ascii_alphanumeric() || c == '_'));
            if !valid_id {
                return Err("identificador no formato com.empresa.app".into());
            }
            Ok(())
        },
        move |ui| {
            let target_name = combo_text(ui, target);
            let target = Target::ALL.into_iter().find(|t| t.name() == target_name).unwrap_or(Target::Desktop);
            let mut p: Project = original;
            p.publish.app_name = Some(text_of(ui, app_name)).filter(|n| n != &p.name).unwrap_or_default();
            p.publish.version = text_of(ui, version);
            p.publish.identifier = Some(text_of(ui, identifier)).filter(|i| *i != p.identifier()).unwrap_or_default();
            p.publish.icon = text_of(ui, icon).replace('\\', "/");
            p.publish.keystore = text_of(ui, keystore).replace('\\', "/");
            p.publish.key_alias = text_of(ui, alias);
            p.publish.backend_url = text_of(ui, backend);
            if let Err(e) = p.save() {
                state2.borrow_mut().status = format!("⚠ {e}");
                return;
            }
            let out = Some(text_of(ui, out)).filter(|o| !o.is_empty()).map(|o| {
                let path = PathBuf::from(o);
                if path.is_absolute() { path } else { p.dir().join(path) }
            });
            let checked = |ui: &Ui, id| matches!(ui.get_prop(id, "checked"), Ok(aylis_core::PropValue::Bool(true)));
            let extras = jobs::PublishExtras {
                bundle: checked(ui, bundle),
                installer: checked(ui, installer),
                key_password: Some(text_of(ui, password)).filter(|s| !s.is_empty()),
            };
            state2.borrow_mut().project = Some(p);
            crate::codewindow::save_all(ui);
            jobs::publish_with(&state2, target, out, extras);
            refresh(ui, &state2, &ids2);
        },
    );
}

/// Escolhe o ícone (copiado para `assets/images` se estiver fora do projeto).
fn pick_icon(state: &Shared, dir: PathBuf, edit: ComponentId) {
    let state2 = state.clone();
    crate::actions::spawn(state, async move {
        let Some(file) = crate::files::AsyncFileDialog::new().add_filter("PNG", &["png"]).set_directory(dir.join("assets/images")).pick_file().await else {
            return;
        };
        let value = crate::inspector::relative_asset(&dir, file.path(), "assets/images");
        state2.borrow_mut().after_task = Some(Box::new(move |ui: &mut Ui| match value {
            Ok(v) => {
                let _ = ui.set_prop(edit, "text", v);
            }
            Err(e) => {
                let _ = ui.set_prop(edit, "text", format!("⚠ {e}"));
            }
        }));
    });
}

// ---------------------------------------------------------------------------
// Traduções

/// "Traduções": escolhe o idioma, junta os textos dos forms e do código
/// (`locales/<idioma>.json`) e edita as traduções lado a lado.
pub fn translations(ui: &mut Ui, state: &Shared, ids: &Ids) {
    let Some(project) = state.borrow().project.clone() else {
        state.borrow_mut().status = "Abra um projeto para traduzir".into();
        return;
    };
    crate::actions::save_modified_forms(state);
    let d = new_dialog(ui, "Traduções", 820.0, 660.0);
    let _ = ui.set_name(d.form, "frmTraducoes");
    // Os idiomas do projeto numa lista, com quanto falta em cada (pedido do autor em 08/11, no
    // lugar do campo de texto), e os botões de acrescentar e tirar.
    let line = field(ui, d.body, "Idioma");
    let load = ui.add(Button::new("Atualizar textos"), line);
    ui.control_mut(load).unwrap().set_align(Align::Right).set_width(150.0).set_margins([8.0, 0.0, 0.0, 0.0]).set_style("flat");
    let remove = ui.add(Button::new("Remover"), line);
    ui.control_mut(remove).unwrap().set_align(Align::Right).set_width(100.0).set_margins([8.0, 0.0, 0.0, 0.0]).set_style("flat");
    let lang = ui.add(ComboBox::new(&[]), line);
    ui.control_mut(lang).unwrap().set_align(Align::Client);
    let add_line = field(ui, d.body, "Novo idioma");
    let add = ui.add(Button::new("Adicionar"), add_line);
    ui.control_mut(add).unwrap().set_align(Align::Right).set_width(110.0).set_margins([8.0, 0.0, 0.0, 0.0]).set_style("flat");
    let new_lang = edit(ui, add_line, "");
    let _ = ui.set_prop(new_lang, "placeholder", "en-US, es, fr...");
    let existing = aylis_project::i18n::languages(&project);
    // Traduzir para: a primeira tradução que existe; senão o idioma da IDE (se o app está em
    // outro), senão inglês (ou português, se o app já é em inglês).
    let ide_locale = crate::lang::app_locale(crate::lang::resolve(&state.borrow().language));
    let base = |l: &str| l.split(['-', '_']).next().unwrap_or_default().to_lowercase();
    let first = existing.first().cloned().unwrap_or_else(|| {
        if base(&ide_locale) != base(&project.locale) {
            ide_locale.clone()
        } else if base(&project.locale) != "en" {
            "en-US".into()
        } else {
            "pt-BR".into()
        }
    });
    // Os idiomas da lista (o código) e o escolhido.
    let languages: std::rc::Rc<std::cell::RefCell<Vec<String>>> = Default::default();
    let current = std::rc::Rc::new(std::cell::RefCell::new(first.clone()));
    let fill_languages = {
        let (languages, project) = (languages.clone(), project.clone());
        move |ui: &mut Ui, selected: &str| {
            let mut list = aylis_project::i18n::languages(&project);
            if !list.iter().any(|l| l == selected) {
                list.push(selected.to_string());
            }
            let items: Vec<String> = list
                .iter()
                .map(|l| match aylis_project::i18n::merge_locale(&project, l) {
                    Ok(m) if !m.path.exists() => format!("{l} · {}", ui.tr("novo")),
                    Ok(m) if m.missing == 0 => format!("{l} · {}", ui.tr("completo")),
                    Ok(m) => format!("{l} · {} {}", m.missing, ui.tr("sem tradução")),
                    Err(_) => l.clone(),
                })
                .collect();
            let _ = ui.set_prop(lang, "items", items.join(";"));
            let index = list.iter().position(|l| l == selected).unwrap_or(0);
            let _ = ui.set_prop(lang, "item_index", index as i64);
            *languages.borrow_mut() = list;
        }
    };
    let fill_languages = std::rc::Rc::new(fill_languages);
    fill_languages(ui, &first);
    let hint = label(
        ui,
        d.body,
        "O texto escrito no designer é a chave. Vazio = sem tradução (mostra o original). No código: ui.tr(\"texto\") / aylis.tr(\"texto\").",
        Align::Top,
        24.0,
    );
    let _ = ui.set_prop(hint, "font_size", 12);
    muted(ui, hint);
    let list = crate::widgets::framed_list(ui, d.body);
    let rows: std::rc::Rc<std::cell::RefCell<Vec<(String, ComponentId)>>> = Default::default();
    // Como o arquivo do idioma mostrado está no disco: aplicar sem mudar nada não grava.
    let saved: std::rc::Rc<std::cell::RefCell<Option<std::collections::BTreeMap<String, String>>>> = Default::default();
    let fill = {
        let (rows, saved, project) = (rows.clone(), saved.clone(), project.clone());
        move |ui: &mut Ui, language: &str| -> String {
            let children = ui.children(list).to_vec();
            ui.free_many(&children);
            rows.borrow_mut().clear();
            let merged = match aylis_project::i18n::merge_locale(&project, language) {
                Ok(r) => r,
                Err(e) => return format!("⚠ {e}"),
            };
            let (path, added, missing) = (merged.path, merged.added, merged.missing);
            *saved.borrow_mut() = path.exists().then_some(merged.saved);
            for (source, translated) in merged.map {
                let row = ui.add(Layout::default(), list);
                ui.control_mut(row).unwrap().set_align(Align::Top).set_height(32.0).set_margins([0.0, 0.0, 0.0, 4.0]);
                let l = ui.add(Label::new(source.as_str()), row);
                ui.control_mut(l).unwrap().set_align(Align::Left).set_width(360.0).set_margins([0.0, 0.0, 10.0, 0.0]);
                let e = ui.add(aylis_controls::Edit::new(translated), row);
                ui.control_mut(e).unwrap().set_align(Align::Client);
                let _ = ui.set_prop(e, "placeholder", source.as_str());
                rows.borrow_mut().push((source, e));
            }
            ui.invalidate();
            format!("{}: {} texto(s), {added} novo(s), {missing} sem tradução", path.display(), rows.borrow().len())
        }
    };
    let status = fill(ui, &first);
    state.borrow_mut().status = status;
    {
        let (fill, state, current) = (fill.clone(), state.clone(), current.clone());
        ui.on_click(load, move |ui, _| {
            let language = current.borrow().clone();
            let s = fill(ui, &language);
            state.borrow_mut().status = s;
        });
    }
    // Outro idioma na lista: os textos dele.
    {
        let (fill, state, current, languages) = (fill.clone(), state.clone(), current.clone(), languages.clone());
        ui.on(lang, "on_change", move |ui, _, _| {
            let index = ui.get::<ComboBox>(lang).map_or(0, |c| c.item_index.max(0) as usize);
            let Some(language) = languages.borrow().get(index).cloned() else { return };
            *current.borrow_mut() = language.clone();
            let s = fill(ui, &language);
            state.borrow_mut().status = s;
        });
    }
    // Um idioma novo entra na lista (o arquivo nasce ao aplicar, com alguma tradução).
    {
        let (fill, fill_languages, state, current) = (fill.clone(), fill_languages.clone(), state.clone(), current.clone());
        ui.on_click(add, move |ui, _| {
            let language = text_of(ui, new_lang).trim().to_string();
            if language.is_empty() || language.contains(['/', '\\', '.']) {
                return;
            }
            *current.borrow_mut() = language.clone();
            fill_languages(ui, &language);
            let _ = ui.set_prop(new_lang, "text", "");
            let s = fill(ui, &language);
            state.borrow_mut().status = s;
        });
    }
    // Tirar o idioma do projeto (o arquivo dele sai), com a confirmação.
    {
        let (fill, fill_languages, state, current, project) = (fill.clone(), fill_languages.clone(), state.clone(), current.clone(), project.clone());
        ui.on_click(remove, move |ui, _| {
            let language = current.borrow().clone();
            if !project.file(&aylis_project::i18n::locale_file(&language)).exists() {
                return;
            }
            let (fill, fill_languages, state, current, project) = (fill.clone(), fill_languages.clone(), state.clone(), current.clone(), project.clone());
            let title = ui.tr("Remover o idioma?");
            let text = ui.tr("As traduções de {0} saem do projeto (o arquivo locales/{0}.json é apagado).").replace("{0}", &language);
            let ok = ui.tr("Remover");
            aylis_controls::confirm(ui, &title, &text, &ok, true, move |ui, yes| {
                if !yes {
                    return;
                }
                let status = match aylis_project::i18n::remove_locale(&project, &language) {
                    Ok(()) => format!("Idioma {language} removido"),
                    Err(e) => format!("⚠ {e}"),
                };
                let next = aylis_project::i18n::languages(&project).into_iter().next().unwrap_or_else(|| "en-US".into());
                *current.borrow_mut() = next.clone();
                fill_languages(ui, &next);
                fill(ui, &next);
                state.borrow_mut().status = status;
            });
        });
    }
    let (state2, ids2) = (state.clone(), *ids);
    run(
        ui,
        d,
        |_| Ok(()),
        move |ui| {
            let language = current.borrow().clone();
            let map: std::collections::BTreeMap<String, String> =
                rows.borrow().iter().map(|(k, e)| (k.clone(), crate::dialogs::text_of(ui, *e))).collect();
            // Nada traduzido num idioma sem arquivo, ou igual ao gravado: não grava.
            let unchanged = match saved.borrow().as_ref() {
                Some(saved) => *saved == map,
                None => map.values().all(String::is_empty),
            };
            if unchanged {
                return;
            }
            state2.borrow_mut().status = match aylis_project::i18n::save_locale(&project, &language, &map) {
                Ok(p) => format!("Traduções gravadas: {} (troque o idioma em Projeto → Opções)", p.display()),
                Err(e) => format!("⚠ {e}"),
            };
            refresh(ui, &state2, &ids2);
        },
    );
}

// ---------------------------------------------------------------------------
// Configurar linguagens

/// O assistente "Configurar linguagens" aberto: a linha de cada linguagem (a caixa e a
/// situação), o log da instalação, o botão e a linha de andamento.
#[derive(Clone)]
pub struct LanguagesView {
    pub form: ComponentId,
    pub rows: Vec<(Language, ComponentId, ComponentId)>,
    pub log: ComponentId,
    pub install: ComponentId,
    pub progress: ComponentId,
}

/// A IDE abriu: confere as linguagens e, se nenhuma estiver instalada (instalação nova),
/// abre o assistente. `AYLIS_IDE_NO_WIZARD` desliga (testes, MCP).
pub fn first_run(_ui: &Ui, state: &Shared, _ids: &Ids) {
    if std::env::var_os("AYLIS_IDE_NO_WIZARD").is_some() {
        return;
    }
    state.borrow_mut().wizard_on_start = true;
    jobs::check_languages(state);
}

/// Confere de novo quais linguagens estão instaladas (o assistente atualiza sozinho).
pub fn check_languages(_ui: &mut Ui, state: &Shared) {
    jobs::check_languages(state);
}

/// "Configurar linguagens": marca as linguagens e a IDE instala o compilador, o depurador
/// e o servidor de linguagem de cada uma, sem passo manual.
pub fn languages(ui: &mut Ui, state: &Shared, ids: &Ids) {
    if state.borrow().languages_view.is_some() {
        return;
    }
    let form = ui.add_root(Form::new("Configurar linguagens", 720.0, 640.0));
    let _ = ui.set_name(form, "frmLinguagens");
    let _ = ui.set_prop(form, "border_style", "single");
    ui.control_mut(form).unwrap().set_padding(16.0);
    let intro = ui.add(
        Label::new(
            "Marque as linguagens que vai usar. A IDE instala tudo de cada uma: o compilador (ou o interpretador), o depurador e o servidor de linguagem do editor. No Windows vem pelo winget, sem perguntas; o Windows pode pedir a permissão de administrador.",
        ),
        form,
    );
    ui.control_mut(intro).unwrap().set_align(Align::Top).set_height(64.0);
    let _ = ui.set_prop(intro, "wrap", true);

    let footer = ui.add(Layout::default(), form);
    ui.control_mut(footer).unwrap().set_align(Align::Bottom).set_height(36.0).set_margins([0.0, 12.0, 0.0, 0.0]);
    let close = ui.add(Button::new("Fechar"), footer);
    ui.control_mut(close).unwrap().set_align(Align::Right).set_width(110.0).set_margins([8.0, 0.0, 0.0, 0.0]).set_style("flat");
    let _ = ui.set_prop(close, "modal_result", "cancel");
    let install = ui.add(Button::new("Instalar as marcadas"), footer);
    ui.control_mut(install).unwrap().set_align(Align::Right).set_width(190.0);
    let progress = label(ui, footer, "", Align::Client, 0.0);
    muted(ui, progress);

    let list = ui.add(Layout::default(), form);
    ui.control_mut(list).unwrap().set_align(Align::Top).set_height(Language::ALL.len() as f32 * 30.0 + 8.0);
    let mut rows = Vec::new();
    for language in Language::ALL {
        let line = ui.add(Layout::default(), list);
        ui.control_mut(line).unwrap().set_align(Align::Top).set_height(30.0);
        let check = ui.add(CheckBox::new(language.name()), line);
        ui.control_mut(check).unwrap().set_align(Align::Left).set_width(200.0);
        let status = label(ui, line, "verificando…", Align::Client, 0.0);
        muted(ui, status);
        rows.push((language, check, status));
    }
    let log_title = caption(ui, form, "Andamento");
    ui.control_mut(log_title).unwrap().set_margins([0.0, 8.0, 0.0, 4.0]);
    let log = ui.add(aylis_controls::Memo::new(""), form);
    ui.control_mut(log).unwrap().set_align(Align::Client);
    let _ = ui.set_prop(log, "read_only", true);
    let _ = ui.set_prop(log, "font_size", 12);

    state.borrow_mut().languages_view = Some(LanguagesView { form, rows: rows.clone(), log, install, progress });
    {
        let state = state.clone();
        ui.on_click(install, move |ui, _| {
            let chosen: Vec<Language> = rows
                .iter()
                .filter(|(_, check, _)| ui.get_prop(*check, "checked").ok().and_then(|v| v.to_bool().ok()).unwrap_or(false))
                .map(|(l, _, _)| *l)
                .collect();
            if chosen.is_empty() {
                let _ = ui.set_prop(progress, "text", "Marque pelo menos uma linguagem.");
                return;
            }
            state.borrow_mut().setup_log.clear();
            jobs::setup_languages(&state, chosen);
            fill_languages(ui, &state);
        });
    }
    let (state2, ids2) = (state.clone(), *ids);
    crate::lang::translate(ui);
    let _ = ui.show_modal(form, move |ui, _| {
        state2.borrow_mut().languages_view = None;
        ui.free(form);
        refresh(ui, &state2, &ids2);
    });
    if state.borrow().language_status.is_empty() {
        jobs::check_languages(state);
    }
    fill_languages(ui, state);
}

/// Atualiza o assistente (se estiver aberto): a situação de cada linguagem, o log e o
/// andamento.
pub fn fill_languages(ui: &mut Ui, state: &Shared) {
    let Some(view) = state.borrow().languages_view.clone() else { return };
    let (status, log, busy, last) = {
        let st = state.borrow();
        (st.language_status.clone(), st.setup_log.join("\n"), st.env_job.as_ref().map(|j| j.title.clone()), st.env_log.clone())
    };
    let (ok, muted_color) = (ui.theme().color("success"), ui.theme().color("text_muted"));
    let warn = ui.theme().color("warning");
    for (language, _, label) in &view.rows {
        let (text, color) = match status.iter().find(|(l, _, _)| l == language) {
            None => ("verificando…".to_string(), muted_color),
            Some((_, true, missing)) if missing.is_empty() => ("✓ pronta".to_string(), ok),
            Some((_, true, missing)) => (format!("instalada; falta: {}", missing.join(", ")), warn),
            Some((_, false, _)) => ("não instalada".to_string(), muted_color),
        };
        let _ = ui.set_prop(*label, "text", text);
        let _ = ui.set_prop(*label, "color", color);
    }
    let changed = ui.get::<aylis_controls::Memo>(view.log).is_some_and(|m| m.text != log);
    if changed {
        let lines = log.lines().count();
        if let Some(m) = ui.get_mut::<aylis_controls::Memo>(view.log) {
            m.text = log;
            m.go_to_line(lines.saturating_sub(1));
        }
    }
    let installing = busy.as_deref().is_some_and(|t| t.starts_with("Configurando"));
    let _ = ui.set_prop(view.install, "enabled", !installing);
    let progress = match busy {
        Some(title) if installing && !last.is_empty() => format!("{title}… {last}"),
        Some(title) if installing => format!("{title}…"),
        _ => String::new(),
    };
    let _ = ui.set_prop(view.progress, "text", progress);
    ui.request_repaint();
}
