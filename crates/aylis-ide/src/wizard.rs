//! O assistente de novo projeto (Arquivo → Novo projeto), em passos: o projeto e a
//! linguagem (com a instalação do que faltar), as plataformas de destino, o modelo, as
//! configurações iniciais (tema, renderizador, ícone, nome e id do app, permissões) e um
//! resumo antes de criar.

use std::cell::{Cell, RefCell};
use std::path::PathBuf;
use std::rc::Rc;

use aylis_controls::{Button, CheckBox, ComboBox, Label, Layout, ScrollBox};
use aylis_core::platform::Permission;
use aylis_core::{Align, ComponentId, PropValue, Ui};
use aylis_project::new_project::{NewProjectOptions, TARGETS, check_identifier, suggested_identifier};
use aylis_project::{Language, RenderOptions};

use crate::app::Ids;
use crate::dialogs::{combo, combo_text, edit, field, new_dialog, render_section, render_value, run, text_of};
use crate::state::Shared;
use crate::widgets::{caption, label, muted};

/// Os passos, na ordem.
const STEPS: [&str; 5] = ["Projeto e linguagem", "Modelo", "Plataformas", "Configurações", "Resumo"];

/// Os temas que o assistente oferece: o texto e o valor do `theme` no projeto.
const THEMES: [(&str, &str); 2] = [("Aylis claro", ""), ("Aylis escuro", "dark")];

/// O texto de cada permissão na lista.
fn permission_title(p: Permission) -> &'static str {
    match p {
        Permission::Camera => "Câmera",
        Permission::Microphone => "Microfone",
        Permission::Location => "Localização",
        Permission::Notifications => "Notificações",
        Permission::Photos => "Fotos e vídeos",
        Permission::Contacts => "Contatos",
        Permission::Calendar => "Agenda",
        Permission::Bluetooth => "Bluetooth",
    }
}

/// O `aylis-tool` ao lado da IDE (a ponte do MCP que o `.mcp.json` do projeto chama); sem ele
/// (a IDE do navegador), o `.mcp.json` chama pelo nome.
pub fn aylis_tool() -> Option<PathBuf> {
    let exe = std::env::current_exe().ok()?;
    let tool = exe.with_file_name(format!("aylis-tool{}", std::env::consts::EXE_SUFFIX));
    tool.is_file().then_some(tool)
}

fn checked(ui: &Ui, id: ComponentId) -> bool {
    ui.get::<CheckBox>(id).is_some_and(|c| c.checked)
}

fn page(ui: &mut Ui, parent: ComponentId) -> ComponentId {
    let p = ui.add(ScrollBox::default(), parent);
    ui.control_mut(p).unwrap().set_align(Align::Client);
    p
}

fn note(ui: &mut Ui, parent: ComponentId, text: &str, height: f32) -> ComponentId {
    let l = label(ui, parent, text, Align::Top, height);
    muted(ui, l);
    let _ = ui.set_prop(l, "wrap", true);
    let _ = ui.set_prop(l, "font_size", 12);
    let _ = ui.set_prop(l, "v_align", PropValue::Enum("leading".into()));
    l
}

/// Abre o assistente. `on_ok` recebe o nome, a pasta, a linguagem e as escolhas.
pub fn new_project(
    ui: &mut Ui,
    state: &Shared,
    ids: &Ids,
    location: PathBuf,
    on_ok: impl FnOnce(&mut Ui, String, PathBuf, Language, NewProjectOptions) + 'static,
) {
    let d = new_dialog(ui, "Novo projeto", 700.0, 660.0);
    // O rodapé do assistente: Cancelar, Voltar, Avançar e, no último passo, Criar.
    let footer_buttons: Vec<ComponentId> = ui.children(d.footer).iter().copied().filter(|c| ui.class_name(*c) == Some("Button")).collect();
    let create = footer_buttons.iter().copied().find(|b| ui.get_prop(*b, "modal_result").ok() == Some(PropValue::Enum("ok".into())));
    if let Some(create) = create {
        let _ = ui.set_prop(create, "text", "Criar");
    }
    let next = ui.add(Button::new("Avançar"), d.footer);
    ui.control_mut(next).unwrap().set_align(Align::Right).set_width(110.0).set_margins([8.0, 0.0, 0.0, 0.0]);
    let back = ui.add(Button::new("Voltar"), d.footer);
    ui.control_mut(back).unwrap().set_align(Align::Right).set_width(110.0).set_margins([8.0, 0.0, 0.0, 0.0]).set_style("flat");

    let header = ui.add(Layout::default(), d.body);
    ui.control_mut(header).unwrap().set_align(Align::Top).set_height(52.0);
    let title = label(ui, header, "", Align::Top, 26.0);
    let _ = ui.set_prop(title, "font_size", 16);
    let _ = ui.set_prop(title, "font_weight", PropValue::Enum("bold".into()));
    let steps_line = label(ui, header, &STEPS.join("  ›  "), Align::Top, 20.0);
    muted(ui, steps_line);
    let _ = ui.set_prop(steps_line, "font_size", 12);
    let pages_box = ui.add(Layout::default(), d.body);
    ui.control_mut(pages_box).unwrap().set_align(Align::Client);
    let pages: Vec<ComponentId> = STEPS.iter().map(|_| page(ui, pages_box)).collect();

    // 1. Projeto e linguagem.
    let p = pages[0];
    let line = field(ui, p, "Nome");
    let name = edit(ui, line, "MeuProjeto");
    let line = field(ui, p, "Criar em");
    let browse = ui.add(Button::new("…"), line);
    ui.control_mut(browse).unwrap().set_align(Align::Right).set_width(36.0).set_margins([6.0, 0.0, 0.0, 0.0]).set_style("flat");
    let dir = edit(ui, line, &location.display().to_string());
    let line = field(ui, p, "Linguagem");
    let install = ui.add(Button::new("Instalar…"), line);
    ui.control_mut(install).unwrap().set_align(Align::Right).set_width(110.0).set_margins([6.0, 0.0, 0.0, 0.0]).set_style("flat");
    let names: Vec<&str> = Language::ALL.iter().map(|l| l.name()).collect();
    let lang = combo(ui, line, &names, "Rust");
    let lang_status = note(ui, p, "", 40.0);
    let folder_hint = note(ui, p, "", 40.0);

    // 2. Modelo: em branco, já configurado para onde o app vai rodar (decidido pelo autor em
    // 08/11): ele marca as plataformas do passo seguinte, o tamanho do form e o aparelho.
    let p = pages[1];
    let line = field(ui, p, "Modelo");
    let templates: Vec<&str> = aylis_project::TEMPLATES.iter().map(|t| t.name).collect();
    let template = combo(ui, line, &templates, templates[0]);
    let template_note = note(ui, p, aylis_project::TEMPLATES[0].description, 44.0);

    // 3. Plataformas.
    let p = pages[2];
    note(ui, p, "Onde o aplicativo vai rodar. A IDE avisa quando um componente não funciona numa delas, e dá para mudar depois em Projeto → Opções.", 44.0);
    let mut platform_checks = Vec::new();
    for (key, title) in TARGETS {
        let c = ui.add(CheckBox::new(title), p);
        ui.control_mut(c).unwrap().set_align(Align::Top).set_height(30.0);
        let on = aylis_project::TEMPLATES[0].platforms.contains(&key);
        let _ = ui.set_prop(c, "checked", on);
        platform_checks.push((key, c));
    }

    // 4. Configurações.
    let p = pages[3];
    caption(ui, p, "APARÊNCIA");
    let line = field(ui, p, "Tema");
    let theme_names: Vec<&str> = THEMES.iter().map(|(t, _)| *t).collect();
    let theme = combo(ui, line, &theme_names, theme_names[0]);
    let render = render_section(ui, p, RenderOptions::default());
    caption(ui, p, "APLICATIVO");
    let line = field(ui, p, "Nome do app");
    let app_name = edit(ui, line, "MeuProjeto");
    let line = field(ui, p, "Id do app");
    let identifier = edit(ui, line, &suggested_identifier("MeuProjeto"));
    let line = field(ui, p, "Ícone");
    let pick = ui.add(Button::new("…"), line);
    ui.control_mut(pick).unwrap().set_align(Align::Right).set_width(36.0).set_margins([6.0, 0.0, 0.0, 0.0]).set_style("flat");
    let icon = edit(ui, line, "");
    let _ = ui.set_prop(icon, "placeholder", "PNG de 1024 px ou SVG (opcional)");
    caption(ui, p, "PERMISSÕES (o app pede com prmApp.request_all; a publicação declara)");
    let mut permission_checks = Vec::new();
    for chunk in Permission::ALL.chunks(4) {
        let row = ui.add(Layout::default(), p);
        ui.control_mut(row).unwrap().set_align(Align::Top).set_height(30.0);
        for perm in chunk {
            let c = ui.add(CheckBox::new(permission_title(*perm)), row);
            ui.control_mut(c).unwrap().set_align(Align::Left).set_width(150.0);
            permission_checks.push((*perm, c));
        }
    }

    caption(ui, p, "AGENTES DE IA");
    let agent_guide = ui.add(CheckBox::new("Guia para agentes de IA (AGENTS.md, CLAUDE.md e .mcp.json)"), p);
    ui.control_mut(agent_guide).unwrap().set_align(Align::Top).set_height(30.0);
    let _ = ui.set_prop(agent_guide, "checked", true);
    note(ui, p, "As regras do código do projeto para o Claude Code e outros agentes, e a ligação deles com a IDE pelo MCP.", 36.0);

    // 5. Resumo.
    let summary = ui.add(Label::new(""), pages[4]);
    ui.control_mut(summary).unwrap().set_align(Align::Top).set_height(420.0);
    let _ = ui.set_prop(summary, "wrap", true);
    let _ = ui.set_prop(summary, "v_align", PropValue::Enum("leading".into()));

    let platform_checks = Rc::new(platform_checks);
    let permission_checks = Rc::new(permission_checks);
    let render = Rc::new(render);
    // O que está escolhido agora.
    let options = {
        let (platform_checks, permission_checks, render) = (platform_checks.clone(), permission_checks.clone(), render.clone());
        move |ui: &Ui| NewProjectOptions {
            template: {
                let index = ui.get::<ComboBox>(template).map_or(0, |c| c.item_index.max(0) as usize);
                aylis_project::TEMPLATES.get(index).map_or(String::new(), |t| t.name.to_string())
            },
            platforms: platform_checks.iter().filter(|(_, c)| checked(ui, *c)).map(|(k, _)| k.to_string()).collect(),
            theme: THEMES.get(ui.get::<ComboBox>(theme).map_or(0, |c| c.item_index.max(0) as usize)).map_or("", |(_, v)| v).to_string(),
            render: render_value(ui, &render),
            locale: String::new(),
            icon: Some(PathBuf::from(text_of(ui, icon))).filter(|p| !p.as_os_str().is_empty()),
            app_name: text_of(ui, app_name),
            identifier: text_of(ui, identifier),
            permissions: permission_checks.iter().filter(|(_, c)| checked(ui, *c)).map(|(p, _)| p.name().to_string()).collect(),
            agent_guide: checked(ui, agent_guide),
            aylis_tool: aylis_tool(),
        }
    };
    let options = Rc::new(options);

    // O que valida cada passo antes de avançar.
    let validate_step = {
        let options = options.clone();
        move |ui: &Ui, step: usize| -> Result<(), String> {
            match step {
                0 => {
                    let n = text_of(ui, name);
                    if n.is_empty() {
                        return Err("informe o nome do projeto".into());
                    }
                    if !PathBuf::from(text_of(ui, dir)).is_dir() {
                        return Err("a pasta escolhida não existe".into());
                    }
                    if PathBuf::from(text_of(ui, dir)).join(&n).read_dir().is_ok_and(|mut e| e.next().is_some()) {
                        return Err(format!("a pasta {n} já existe e não está vazia"));
                    }
                    Ok(())
                }
                2 if options(ui).platforms.is_empty() => Err("marque ao menos uma plataforma".into()),
                3 => {
                    let o = options(ui);
                    check_identifier(&o.identifier)?;
                    match &o.icon {
                        Some(p) if !p.is_file() => Err(format!("o ícone {} não existe", p.display())),
                        _ => Ok(()),
                    }
                }
                _ => Ok(()),
            }
        }
    };
    let validate_step = Rc::new(validate_step);

    let step = Rc::new(Cell::new(0usize));
    let error = d.error;
    let show = {
        let (step, options, pages) = (step.clone(), options.clone(), pages.clone());
        move |ui: &mut Ui| {
            let s = step.get();
            for (i, p) in pages.iter().enumerate() {
                ui.control_mut(*p).unwrap().set_visible(i == s);
            }
            let _ = ui.set_prop(title, "text", format!("{} de {} · {}", s + 1, STEPS.len(), ui.tr(STEPS[s])));
            ui.control_mut(back).unwrap().set_visible(s > 0);
            ui.control_mut(next).unwrap().set_visible(s + 1 < STEPS.len());
            if let Some(create) = create {
                ui.control_mut(create).unwrap().set_visible(s + 1 == STEPS.len());
            }
            let _ = ui.set_prop(error, "text", "");
            if s + 1 == STEPS.len() {
                let o = options(ui);
                let platforms: Vec<&str> = TARGETS.iter().filter(|(k, _)| o.platforms.iter().any(|p| p == k)).map(|(_, t)| *t).collect();
                let permissions: Vec<&str> = Permission::ALL.iter().filter(|p| o.permissions.iter().any(|q| q == p.name())).map(|p| permission_title(*p)).collect();
                let none = ui.tr("nenhuma");
                let text = [
                    format!("{}: {}", ui.tr("Pasta"), PathBuf::from(text_of(ui, dir)).join(text_of(ui, name)).display()),
                    format!("{}: {}", ui.tr("Linguagem"), combo_text(ui, lang)),
                    format!("{}: {}", ui.tr("Plataformas"), platforms.join(", ")),
                    format!("{}: {}", ui.tr("Modelo"), combo_text(ui, template)),
                    format!("{}: {}", ui.tr("Tema"), combo_text(ui, theme)),
                    format!("{}: {} ({})", ui.tr("Aplicativo"), o.app_name, o.identifier),
                    format!("{}: {}", ui.tr("Ícone"), o.icon.as_ref().map_or(ui.tr("nenhum"), |p| p.display().to_string())),
                    format!("{}: {}", ui.tr("Permissões"), if permissions.is_empty() { none } else { permissions.join(", ") }),
                    format!("{}: {}", ui.tr("Guia para agentes de IA"), if o.agent_guide { aylis_project::agents::FILES.join(", ") } else { ui.tr("não") }),
                ]
                .join("\n\n");
                let _ = ui.set_prop(summary, "text", text);
            }
        }
    };
    let show = Rc::new(show);
    show(ui);

    {
        let (step, show, validate_step) = (step.clone(), show.clone(), validate_step.clone());
        ui.on_click(next, move |ui, _| {
            if let Err(e) = validate_step(ui, step.get()) {
                let _ = ui.set_prop(error, "text", format!("⚠ {}", ui.tr(&e)));
                return;
            }
            step.set((step.get() + 1).min(STEPS.len() - 1));
            show(ui);
        });
    }
    {
        let (step, show) = (step.clone(), show.clone());
        ui.on_click(back, move |ui, _| {
            step.set(step.get().saturating_sub(1));
            show(ui);
        });
    }

    // A linguagem: pronta ou o que falta (e o botão de instalar).
    // Trocar o modelo marca as plataformas dele e mostra o que ele traz.
    {
        let platform_checks = platform_checks.clone();
        ui.on(template, "on_change", move |ui, _, _| {
            let index = ui.get::<ComboBox>(template).map_or(0, |c| c.item_index.max(0) as usize);
            let Some(t) = aylis_project::TEMPLATES.get(index) else { return };
            for (key, c) in platform_checks.iter() {
                let _ = ui.set_prop(*c, "checked", t.platforms.contains(key));
            }
            let text = ui.tr(t.description);
            let _ = ui.set_prop(template_note, "text", text);
        });
    }
    let language_status = move |ui: &mut Ui| {
        let language = Language::from_name(&combo_text(ui, lang)).unwrap_or_default();
        // Na IDE do navegador não há compilador a instalar.
        let ready = !cfg!(feature = "desktop") || aylis_tools::setup::language_ready(language);
        let text = if ready {
            ui.tr("Pronta: o compilador está instalado.")
        } else {
            ui.tr("Falta instalar o compilador desta linguagem: \"Instalar…\" baixa e configura (dá para criar o projeto antes).")
        };
        let _ = ui.set_prop(lang_status, "text", text);
        ui.control_mut(install).unwrap().set_visible(!ready);
    };
    language_status(ui);
    ui.on(lang, "on_change", move |ui, _, _| language_status(ui));
    {
        let (state, ids) = (state.clone(), *ids);
        ui.on_click(install, move |ui, _| crate::tooldialogs::languages(ui, &state, &ids));
    }
    // O nome do projeto sugere o nome e o id do app (até a pessoa mudar).
    let follows = Rc::new(RefCell::new((text_of(ui, name), true, true)));
    let update = move |ui: &mut Ui| {
        let n = text_of(ui, name);
        let path = PathBuf::from(text_of(ui, dir)).join(&n);
        let _ = ui.set_prop(folder_hint, "text", ui.tr("A pasta {0} será criada com forms/, assets/, themes/ e o código.").replace("{0}", &path.display().to_string()));
        let mut f = follows.borrow_mut();
        f.1 = f.1 && text_of(ui, app_name) == f.0;
        f.2 = f.2 && text_of(ui, identifier) == suggested_identifier(&f.0);
        if f.1 {
            let _ = ui.set_prop(app_name, "text", n.as_str());
        }
        if f.2 {
            let _ = ui.set_prop(identifier, "text", suggested_identifier(&n));
        }
        f.0 = n;
    };
    let update = Rc::new(update);
    update(ui);
    {
        let update = update.clone();
        ui.on(name, "on_change", move |ui, _, _| update(ui));
    }
    {
        let update = update.clone();
        ui.on(dir, "on_change", move |ui, _, _| update(ui));
    }
    {
        let state = state.clone();
        let update = update.clone();
        ui.on_click(browse, move |_, _| {
            let (state2, update) = (state.clone(), update.clone());
            crate::actions::spawn(&state, async move {
                if let Some(folder) = crate::files::AsyncFileDialog::new().pick_folder().await {
                    state2.borrow_mut().after_task = Some(Box::new(move |ui: &mut Ui| {
                        let _ = ui.set_prop(dir, "text", folder.path().display().to_string());
                        update(ui);
                    }));
                }
            });
        });
    }
    {
        let state = state.clone();
        ui.on_click(pick, move |_, _| {
            let state2 = state.clone();
            crate::actions::spawn(&state, async move {
                if let Some(file) = crate::files::AsyncFileDialog::new().add_filter("Imagem", &["png", "svg"]).pick_file().await {
                    state2.borrow_mut().after_task = Some(Box::new(move |ui: &mut Ui| {
                        let _ = ui.set_prop(icon, "text", file.path().display().to_string());
                    }));
                }
            });
        });
    }

    let validate_all = {
        let validate_step = validate_step.clone();
        move |ui: &mut Ui| (0..STEPS.len()).try_for_each(|s| validate_step(ui, s))
    };
    run(ui, d, validate_all, move |ui| {
        let language = Language::from_name(&combo_text(ui, lang)).unwrap_or_default();
        let o = options(ui);
        on_ok(ui, text_of(ui, name), PathBuf::from(text_of(ui, dir)), language, o);
    });
    ui.set_focus(name);
}
