//! Ferramentas → Opções da IDE (como o Tools → Options do Delphi): a gravação do uso, os
//! projetos confiáveis, onde está cada compilador e ferramenta (uma aba por linguagem: vazio é
//! o automático, o PATH e as pastas padrão), o editor externo e o MCP (o servidor, o token e a
//! configuração do Claude Code e do Claude Desktop). Vale no Windows, no macOS e no Linux.

use std::path::{Path, PathBuf};

use aylis_controls::{Button, CheckBox, Edit, Form, Layout, Memo, TabControl};
use aylis_core::{Align, ComponentId, Ui};
use aylis_project::Language;
use aylis_tools::hostlang;

use crate::app::Ids;
use crate::state::Shared;
use crate::widgets::{caption, label, muted};

/// As abas das ferramentas: o título e as chaves de [`aylis_tools::settings`].
const TOOL_TABS: &[(&str, &[&str])] = &[
    ("TypeScript", &["bun", "node"]),
    ("C#", &["dotnet"]),
    ("Java", &["javac", "java"]),
    ("Go", &["go"]),
    ("C++", &["vcvars", "gxx"]),
    ("Pascal", &["fpc", "fpc_units"]),
    ("Publicação", &["makensis", "bundletool", "wix"]),
    ("Editor externo", &["vscode"]),
];

/// O que está em uso para a chave (o caminho escolhido ou o achado no PATH e nas pastas
/// padrão).
fn in_use(key: &str) -> Option<PathBuf> {
    let found = |language| hostlang::find(language).ok();
    match key {
        "bun" | "node" => aylis_tools::settings::tool(key).or_else(|| hostlang::which(key)),
        "dotnet" => found(Language::CSharp).map(|t| t.program),
        "javac" => found(Language::Java).map(|t| t.program),
        "java" => found(Language::Java).and_then(|t| t.extra),
        "go" => found(Language::Go).map(|t| t.program),
        "vcvars" | "gxx" => found(Language::Cpp).map(|t| t.program).filter(|p| (key == "vcvars") == p.extension().is_some_and(|e| e.eq_ignore_ascii_case("bat"))),
        "fpc" => found(Language::Pascal).map(|t| t.program),
        "fpc_units" => found(Language::Pascal).and_then(|t| t.extra),
        "makensis" => aylis_tools::installer::makensis(),
        "bundletool" => aylis_tools::publish::bundletool(),
        "wix" => aylis_tools::publish::wix(),
        "vscode" => aylis_tools::shell::vscode(),
        _ => None,
    }
}

fn describe(key: &str) -> &'static str {
    aylis_tools::settings::TOOLS.iter().find(|(k, _)| *k == key).map_or("", |(_, d)| d)
}

fn in_use_text(ui: &Ui, key: &str) -> String {
    match in_use(key) {
        Some(p) => format!("{}: {}", ui.tr("em uso"), p.display()),
        None => ui.tr("não encontrado"),
    }
}

/// Uma linha de ferramenta: o nome, o caminho escolhido (vazio: automático) com "Procurar…" e
/// o que está em uso.
fn tool_row(ui: &mut Ui, state: &Shared, page: ComponentId, key: &'static str) -> (ComponentId, ComponentId) {
    let title = caption(ui, page, describe(key));
    ui.control_mut(title).unwrap().set_margins([0.0, 10.0, 0.0, 2.0]);
    let line = ui.add(Layout::default(), page);
    ui.control_mut(line).unwrap().set_align(Align::Top).set_height(32.0);
    let browse = ui.add(Button::new("Procurar…"), line);
    ui.control_mut(browse).unwrap().set_align(Align::Right).set_width(110.0).set_margins([8.0, 0.0, 0.0, 0.0]).set_style("flat");
    let edit = ui.add(Edit::new(aylis_tools::settings::tool_text(key)), line);
    ui.control_mut(edit).unwrap().set_align(Align::Client);
    let _ = ui.set_prop(edit, "placeholder", ui.tr("automático (PATH e pastas padrão)"));
    let used = label(ui, page, &in_use_text(ui, key), Align::Top, 22.0);
    muted(ui, used);
    let state2 = state.clone();
    ui.on_click(browse, move |_, _| {
        let state3 = state2.clone();
        crate::actions::spawn(&state2, async move {
            let picked = if key == "fpc_units" { crate::files::AsyncFileDialog::new().pick_folder().await } else { crate::files::AsyncFileDialog::new().pick_file().await };
            let Some(file) = picked else { return };
            let path = file.path().display().to_string();
            state3.borrow_mut().after_task = Some(Box::new(move |ui: &mut Ui| {
                let _ = ui.set_prop(edit, "text", path);
            }));
        });
    });
    (edit, used)
}

/// Os campos da pasta de compilação: a pasta, se vale para todo o Rust da máquina e os dias do
/// que é de um projeto e do compartilhado.
struct BuildFolder {
    dir: ComponentId,
    global: ComponentId,
    project_days: ComponentId,
    shared_days: ComponentId,
}

/// A pasta de compilação dos projetos em Rust (uma para todos) e os prazos da limpeza
/// (`aylis_tools::cleanup`), com o "Limpar agora".
fn build_folder(ui: &mut Ui, state: &Shared, page: ComponentId) -> BuildFolder {
    let title = caption(ui, page, "Pasta de compilação (todos os projetos em Rust)");
    ui.control_mut(title).unwrap().set_margins([0.0, 14.0, 0.0, 2.0]);
    let line = ui.add(Layout::default(), page);
    ui.control_mut(line).unwrap().set_align(Align::Top).set_height(32.0);
    let browse = ui.add(Button::new("Procurar…"), line);
    ui.control_mut(browse).unwrap().set_align(Align::Right).set_width(110.0).set_margins([8.0, 0.0, 0.0, 0.0]).set_style("flat");
    let dir = ui.add(Edit::new(aylis_tools::settings::tool_text("target_dir")), line);
    ui.control_mut(dir).unwrap().set_align(Align::Client);
    let _ = ui.set_prop(dir, "placeholder", format!("{}: {}", ui.tr("automático"), aylis_tools::default_target_dir().display()));
    let state2 = state.clone();
    ui.on_click(browse, move |_, _| {
        let state3 = state2.clone();
        crate::actions::spawn(&state2, async move {
            let Some(folder) = crate::files::AsyncFileDialog::new().pick_folder().await else { return };
            let path = folder.path().display().to_string();
            state3.borrow_mut().after_task = Some(Box::new(move |ui: &mut Ui| {
                let _ = ui.set_prop(dir, "text", path);
            }));
        });
    });
    // `aylis_tools::cargo_config`: o padrão é ligado.
    let global = ui.add(CheckBox::new("Usar esta pasta em todo o Rust desta máquina (terminal, editor, outros projetos)"), page);
    ui.control_mut(global).unwrap().set_align(Align::Top).set_height(30.0).set_margins([0.0, 8.0, 0.0, 0.0]);
    let _ = ui.set_prop(global, "checked", aylis_tools::cargo_config::enabled());
    let place = aylis_tools::cargo_config::global_path().map(|p| p.display().to_string()).unwrap_or_default();
    let about_global = label(ui, page, &format!("{} {place}", ui.tr("Grava a pasta e as dependências sem depuração (o target/ cai à metade) em")), Align::Top, 22.0);
    muted(ui, about_global);
    let days = |ui: &mut Ui, text: &str, key: &str, default: u64| {
        let line = ui.add(Layout::default(), page);
        ui.control_mut(line).unwrap().set_align(Align::Top).set_height(34.0).set_margins([0.0, 8.0, 0.0, 0.0]);
        let l = label(ui, line, text, Align::Left, 260.0);
        muted(ui, l);
        let value = aylis_tools::settings::number(key).unwrap_or(default);
        let e = crate::dialogs::edit(ui, line, &value.to_string());
        ui.control_mut(e).unwrap().set_align(Align::Left).set_width(80.0);
        e
    };
    let project_days = days(ui, "Dias do que é só de um projeto", "cleanup_project_days", aylis_tools::cleanup::DEFAULT_PROJECT_DAYS);
    let shared_days = days(ui, "Dias do compartilhado", "cleanup_shared_days", aylis_tools::cleanup::DEFAULT_SHARED_DAYS);
    let about = label(
        ui,
        page,
        "O que é só de um projeto (o código dele, os testes e os exemplos compilados) sai depois desses dias sem compilar o projeto (cada projeto pode ter o seu prazo em Projeto → Opções); o compartilhado (o aylis, as dependências), depois do último projeto que o usou. O que nenhum projeto usa mais (outra versão do Rust ou de uma dependência, um projeto apagado) sai na limpeza automática, uma vez por hora.",
        Align::Top,
        66.0,
    );
    let _ = ui.set_prop(about, "wrap", true);
    muted(ui, about);
    let buttons = ui.add(Layout::default(), page);
    ui.control_mut(buttons).unwrap().set_align(Align::Top).set_height(32.0).set_margins([0.0, 8.0, 0.0, 0.0]);
    let clean = ui.add(Button::new("Limpar agora"), buttons);
    ui.control_mut(clean).unwrap().set_align(Align::Left).set_width(150.0).set_style("flat");
    let state2 = state.clone();
    ui.on_click(clean, move |_, _| {
        crate::jobs::start(&state2, crate::jobs::Slot::Main, "Limpando a pasta de compilação", |_, _| {
            let target = aylis_tools::shared_target_dir();
            let r = aylis_tools::cleanup::clean(&target, aylis_tools::cleanup::DEFAULT_DAYS);
            Ok(format!("{}: {} unidades, {} MB", target.display(), r.units, r.bytes / (1024 * 1024)))
        });
    });
    BuildFolder { dir, global, project_days, shared_days }
}

/// Grava os campos da pasta de compilação.
fn apply_build_folder(ui: &Ui, b: &BuildFolder, errors: &mut Vec<String>) {
    let dir = crate::dialogs::text_of(ui, b.dir);
    let days = |id| crate::dialogs::text_of(ui, id).parse::<u64>().ok().filter(|d| *d > 0).map(|d| d.to_string());
    for (key, value) in [("target_dir", Some(dir)), ("cleanup_project_days", days(b.project_days)), ("cleanup_shared_days", days(b.shared_days))] {
        if let Err(e) = aylis_tools::settings::set_tool(key, value.as_deref()) {
            errors.push(e);
        }
    }
    // Depois da pasta: o arquivo global leva a pasta nova.
    let global = ui.get_prop(b.global, "checked").ok().and_then(|v| v.to_bool().ok()).unwrap_or(true);
    if let Err(e) = aylis_tools::cargo_config::set_enabled(global) {
        errors.push(e);
    }
}

/// O executável `aylis-tool` ao lado da IDE (a ponte do MCP).
fn aylis_tool() -> Result<PathBuf, String> {
    let exe = std::env::current_exe().map_err(|e| e.to_string())?;
    let tool = exe.with_file_name(format!("aylis-tool{}", std::env::consts::EXE_SUFFIX));
    if tool.is_file() { Ok(tool) } else { Err(format!("o aylis-tool não está ao lado da IDE ({})", tool.display())) }
}

/// Onde está o `claude`, para a aba MCP: no PATH, fora dele (o caminho completo, que um
/// terminal comum não acha sozinho) ou não encontrado.
fn claude_code_place(ui: &Ui) -> String {
    match aylis_remote::mcp::claude_executable() {
        Some(c) if c.in_path => format!("Claude Code: {}", c.path.display()),
        Some(c) => format!("{} {}", ui.tr("Claude Code fora do PATH (num terminal, use o caminho completo):"), c.path.display()),
        None => ui.tr("Claude Code não encontrado (nem no PATH, nem com o Claude Desktop)"),
    }
}

/// Registra o aylis no Claude Code (`claude mcp add`), chamado direto (sem `cmd`), pelo caminho
/// completo do `claude` (o do Claude Desktop não fica no PATH).
fn configure_claude_code() -> Result<String, String> {
    let tool = aylis_tool()?;
    let args = aylis_tools::mcp::claude_code_args(&tool);
    let Some(claude) = aylis_remote::mcp::claude_executable().map(|c| c.path) else {
        return Err(format!("Claude Code não encontrado (nem no PATH, nem com o Claude Desktop); instale e rode: claude {}", args.join(" ")));
    };
    let out = aylis_tools::process::quiet(std::process::Command::new(&claude).args(&args)).output().map_err(|e| e.to_string())?;
    if out.status.success() {
        Ok("Claude Code configurado: o servidor aylis aparece no /mcp".into())
    } else {
        Err(String::from_utf8_lossy(&out.stderr).trim().to_string())
    }
}

/// Grava o aylis na configuração do Claude Desktop (inclusive na cópia do pacote MSIX).
fn configure_claude_desktop() -> Result<String, String> {
    let tool = aylis_tool()?;
    let configs = aylis_tools::mcp::claude_desktop_configs();
    if configs.is_empty() {
        return Err("o Claude Desktop não existe neste sistema (use o Claude Code)".into());
    }
    // Onde o Claude já tem a pasta dele; sem nenhuma, a padrão.
    let mut targets: Vec<&Path> = configs.iter().filter(|p| p.parent().is_some_and(Path::is_dir)).map(PathBuf::as_path).collect();
    if targets.is_empty() {
        targets.push(&configs[0]);
    }
    for path in &targets {
        aylis_tools::mcp::add_to_claude_desktop(path, &tool)?;
    }
    Ok(format!("Claude Desktop configurado ({}); reinicie o Claude Desktop", targets.iter().map(|p| p.display().to_string()).collect::<Vec<_>>().join(", ")))
}

/// Abre a tela de opções.
pub fn open(ui: &mut Ui, state: &Shared, ids: &Ids) {
    let form = ui.add_root(Form::new("Opções da IDE", 940.0, 600.0));
    let _ = ui.set_name(form, "frmOpcoesIde");
    let _ = ui.set_prop(form, "border_style", "single");
    ui.control_mut(form).unwrap().set_padding(16.0);

    let footer = ui.add(Layout::default(), form);
    ui.control_mut(footer).unwrap().set_align(Align::Bottom).set_height(36.0).set_margins([0.0, 12.0, 0.0, 0.0]);
    let cancel = ui.add(Button::new("Cancelar"), footer);
    ui.control_mut(cancel).unwrap().set_align(Align::Right).set_width(110.0).set_margins([8.0, 0.0, 0.0, 0.0]).set_style("flat");
    let _ = ui.set_prop(cancel, "modal_result", "cancel");
    let ok = ui.add(Button::new("OK"), footer);
    ui.control_mut(ok).unwrap().set_align(Align::Right).set_width(110.0);
    let _ = ui.set_prop(ok, "modal_result", "ok");

    let tabs = ui.add(TabControl::default(), form);
    ui.control_mut(tabs).unwrap().set_align(Align::Client);

    // Geral: a gravação do uso e os projetos confiáveis.
    let general = TabControl::add_tab(ui, tabs, "Geral");
    ui.control_mut(general).unwrap().set_padding(12.0);
    let record = ui.add(CheckBox::new("Gravar o uso da IDE para investigar problemas"), general);
    ui.control_mut(record).unwrap().set_align(Align::Top).set_height(30.0);
    let _ = ui.set_prop(record, "checked", crate::recordings::recording(ui, ids));
    let note = label(ui, general, "As gravações ficam só nesta máquina, com o que se clica e digita (as senhas, mascaradas).", Align::Top, 40.0);
    let _ = ui.set_prop(note, "wrap", true);
    muted(ui, note);
    let mask = ui.add(CheckBox::new("Mascarar o texto do editor de código nas gravações"), general);
    ui.control_mut(mask).unwrap().set_align(Align::Top).set_height(30.0);
    let _ = ui.set_prop(mask, "checked", crate::actions::config_flag("mask_editor_text", false));
    let splash = ui.add(CheckBox::new("Mostrar a tela de abertura ao iniciar a IDE"), general);
    ui.control_mut(splash).unwrap().set_align(Align::Top).set_height(30.0);
    let _ = ui.set_prop(splash, "checked", crate::splash::enabled());
    let trusted_title = caption(ui, general, "Projetos confiáveis (compilam e executam sem perguntar)");
    ui.control_mut(trusted_title).unwrap().set_margins([0.0, 14.0, 0.0, 2.0]);
    let trusted = ui.add(Memo::new(aylis_tools::trust::list().join("\n")), general);
    ui.control_mut(trusted).unwrap().set_align(Align::Client);
    let _ = ui.set_prop(trusted, "read_only", true);

    // Uma aba por linguagem (e a das ferramentas de publicação e o editor externo).
    let mut fields: Vec<(&'static str, ComponentId, ComponentId)> = Vec::new();
    for (title, keys) in TOOL_TABS {
        let page = TabControl::add_tab(ui, tabs, title);
        ui.control_mut(page).unwrap().set_padding(12.0);
        for key in keys.iter() {
            if *key == "vcvars" && !cfg!(windows) {
                continue;
            }
            let (edit, used) = tool_row(ui, state, page, key);
            fields.push((key, edit, used));
        }
    }
    // Rust e Python: o que está em uso (instalados pelo Configurar linguagens).
    let page = TabControl::add_tab(ui, tabs, "Rust e Python");
    ui.control_mut(page).unwrap().set_padding(12.0);
    let cargo = hostlang::which("cargo").map_or_else(|| ui.tr("não encontrado"), |p| p.display().to_string());
    let python = aylis_tools::env::python().map_or_else(|| ui.tr("não encontrado"), |(p, _)| p);
    label(ui, page, &format!("cargo: {cargo}"), Align::Top, 26.0);
    label(ui, page, &format!("python: {python}"), Align::Top, 26.0);
    let hint = label(ui, page, "Instale ou atualize por Ferramentas → Configurar linguagens.", Align::Top, 26.0);
    muted(ui, hint);
    let build = build_folder(ui, state, page);

    // MCP: o servidor, a porta, o token e a configuração dos clientes.
    let mcp = TabControl::add_tab(ui, tabs, "MCP");
    ui.control_mut(mcp).unwrap().set_padding(12.0);
    let mcp_on = ui.add(CheckBox::new("Ligar o servidor MCP (agentes de IA comandando a IDE)"), mcp);
    ui.control_mut(mcp_on).unwrap().set_align(Align::Top).set_height(30.0);
    let _ = ui.set_prop(mcp_on, "checked", state.borrow().mcp.is_some());
    let port_line = crate::dialogs::field(ui, mcp, "Porta");
    let port = crate::dialogs::edit(ui, port_line, &aylis_tools::mcp::access().map(|a| a.port).unwrap_or(aylis_tools::mcp::DEFAULT_PORT).to_string());
    let about = label(ui, mcp, "Só programas com o token (guardado nesta máquina, fora dos projetos) usam o servidor. Os clientes conectam pelo aylis-tool mcp, que lê o token sozinho.", Align::Top, 44.0);
    let _ = ui.set_prop(about, "wrap", true);
    muted(ui, about);
    let buttons = ui.add(Layout::default(), mcp);
    ui.control_mut(buttons).unwrap().set_align(Align::Top).set_height(34.0).set_margins([0.0, 8.0, 0.0, 0.0]);
    let code = ui.add(Button::new("Configurar o Claude Code"), buttons);
    ui.control_mut(code).unwrap().set_align(Align::Left).set_width(210.0).set_margins([0.0, 0.0, 8.0, 0.0]);
    let desktop = (!cfg!(target_os = "linux")).then(|| {
        let b = ui.add(Button::new("Configurar o Claude Desktop"), buttons);
        ui.control_mut(b).unwrap().set_align(Align::Left).set_width(220.0).set_margins([0.0, 0.0, 8.0, 0.0]);
        b
    });
    let token = ui.add(Button::new("Gerar novo token"), buttons);
    ui.control_mut(token).unwrap().set_align(Align::Left).set_width(160.0).set_style("flat");
    let place = claude_code_place(ui);
    let claude_line = label(ui, mcp, &place, Align::Top, 40.0);
    let _ = ui.set_prop(claude_line, "wrap", true);
    muted(ui, claude_line);
    let result = label(ui, mcp, "", Align::Top, 48.0);
    let _ = ui.set_prop(result, "wrap", true);
    let show = move |ui: &mut Ui, r: Result<String, String>| {
        let (text, color) = match r {
            Ok(t) => (t, ui.theme().color("success")),
            Err(e) => (format!("⚠ {e}"), ui.theme().color("error")),
        };
        let _ = ui.set_prop(result, "text", text);
        let _ = ui.set_prop(result, "color", color);
    };
    ui.on_click(code, move |ui, _| show(ui, configure_claude_code()));
    if let Some(desktop) = desktop {
        ui.on_click(desktop, move |ui, _| show(ui, configure_claude_desktop()));
    }
    {
        let state = state.clone();
        ui.on_click(token, move |ui, _| {
            let r = aylis_tools::mcp::new_token().map(|_| "Token novo gerado: o antigo parou de valer".to_string());
            // O servidor ligado passa a pedir o novo.
            if r.is_ok() && state.borrow().mcp.is_some() {
                crate::actions::set_mcp(ui, &state, true, 0, false);
            }
            show(ui, r);
        });
    }

    TabControl::select(ui, tabs, 0);
    crate::lang::translate(ui);
    let (state2, ids2) = (state.clone(), *ids);
    let _ = ui.show_modal(form, move |ui, result| {
        if result == aylis_core::ModalResult::Ok {
            apply(ui, &state2, &ids2, &fields, [record, mask, splash], &build, mcp_on, port);
        }
        ui.free(form);
        crate::app::refresh(ui, &state2, &ids2);
    });
}

/// O OK: grava tudo. `general` são as caixas da aba Geral: gravar o uso, mascarar o editor e
/// a tela de abertura.
#[allow(clippy::too_many_arguments)]
fn apply(ui: &mut Ui, state: &Shared, ids: &Ids, fields: &[(&'static str, ComponentId, ComponentId)], general: [ComponentId; 3], build: &BuildFolder, mcp_on: ComponentId, port: ComponentId) {
    let [record, mask, splash] = general;
    let checked = |ui: &Ui, id| ui.get_prop(id, "checked").ok().and_then(|v| v.to_bool().ok()).unwrap_or(false);
    let mut errors = Vec::new();
    for (key, edit, _) in fields {
        let text = crate::dialogs::text_of(ui, *edit);
        if let Err(e) = aylis_tools::settings::set_tool(key, Some(&text)) {
            errors.push(e);
        }
    }
    apply_build_folder(ui, build, &mut errors);
    let on = checked(ui, record);
    if on != crate::recordings::recording(ui, ids) {
        crate::recordings::set_recording(ui, state, ids, on);
    }
    let masked = checked(ui, mask);
    crate::actions::update_config("mask_editor_text", serde_json::Value::from(masked));
    crate::codewindow::set_sensitive(ui, masked);
    crate::splash::set_enabled(checked(ui, splash));
    let port: u16 = crate::dialogs::text_of(ui, port).parse().unwrap_or(aylis_tools::mcp::DEFAULT_PORT);
    let old_port = aylis_tools::mcp::access().map(|a| a.port).unwrap_or(aylis_tools::mcp::DEFAULT_PORT);
    if port != old_port {
        if let Err(e) = aylis_tools::mcp::set_port(port) {
            errors.push(e);
        }
    }
    let want = checked(ui, mcp_on);
    if want != state.borrow().mcp.is_some() || (want && port != old_port) {
        crate::actions::set_mcp(ui, state, want, 0, true);
    }
    if !errors.is_empty() {
        state.borrow_mut().status = format!("⚠ {}", errors.join("; "));
    }
}
