//! Aylis IDE.
//!
//! ```text
//! aylis-ide [projeto.aylisproj | layout.json]   abre a IDE (com o projeto ou form, se informado)
//! aylis-ide --preview layout.json [--dark] [--assets pasta]
//!                                             executa a tela numa janela (o "Prévia do form")
//! aylis-ide --png saida.png [projeto|layout] [--gallery]
//!                                             desenha a IDE (ou a galeria de componentes) num PNG
//!                [--recording arquivo.aylisrec]  ou o visualizador de uma gravação
//! aylis-ide --format-forms projeto.aylisproj    regrava os forms no formato que a IDE salva
//! aylis-ide --headless --mcp-stdio [projeto]    a IDE sem janela, com o MCP no stdin/stdout
//! ```

// Programa de janela no Windows: abrir com dois cliques não traz um console junto.
#![windows_subsystem = "windows"]

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use aylis_ide::document::{Document, new_document_ui};
use aylis_ide::state::shared;
use aylis_ide::{actions, app};

fn main() -> ExitCode {
    attach_console();
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("aylis-ide: {e}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let flag_value = |flag: &str| args.iter().position(|a| a == flag).and_then(|i| args.get(i + 1)).cloned();

    // O gancho do Claude Code: decide e sai, sem abrir nada (roda antes de cada gravação).
    #[cfg(feature = "desktop")]
    if args.iter().any(|a| a == aylis_ide::edit_guard::FLAG) {
        std::process::exit(aylis_ide::edit_guard::run());
    }

    if let Some(path) = flag_value("--format-forms") {
        let project = aylis_project::Project::open(&path).map_err(|e| e.to_string())?;
        let changed = actions::format_forms(&project)?;
        println!("{} de {} forms regravados no formato da IDE", changed.len(), project.forms.len());
        return Ok(());
    }
    if let Some(path) = flag_value("--preview") {
        return preview(Path::new(&path), args.iter().any(|a| a == "--dark"), flag_value("--assets"));
    }
    let png = flag_value("--png");
    let recording = flag_value("--recording");
    let device = flag_value("--device");
    let target = args.iter().find(|a| !a.starts_with("--") && Some(*a) != png.as_ref() && Some(*a) != recording.as_ref() && Some(*a) != device.as_ref()).cloned();

    // As compilações substituídas ou esquecidas saem do target enquanto a IDE abre.
    // E todo o Rust da máquina passa a compilar na mesma pasta (o padrão; desliga nas Opções).
    std::thread::spawn(|| {
        let _ = aylis_tools::cargo_config::apply();
        aylis_tools::cleanup::automatic(&aylis_tools::shared_target_dir())
    });
    // Dois cliques num projeto já aberto em outra IDE (ou com uma na tela inicial): ele vai
    // para lá e esta não abre. `--new-window` (Arquivo → Nova janela) abre sempre.
    let headless = args.iter().any(|a| a == "--headless");
    if headless && !args.iter().any(|a| a == "--mcp-stdio") {
        return Err("--headless só existe com --mcp-stdio (a IDE sem janela comandada pelo MCP no stdio)".into());
    }
    if let Some(target) = &target
        && !args.iter().any(|a| a == "--new-window")
        && png.is_none()
        && !headless
        && aylis_tools::instances::send(Path::new(target))
    {
        return Ok(());
    }
    let state = shared(Document::new());
    actions::load_config(&state);
    // Projetos apagados saem da lista dos confiáveis.
    aylis_tools::trust::prune();
    // Antes da gravação desta sessão começar: a anterior quebrou?
    let crash = aylis_ide::reportview::previous_crash(std::time::SystemTime::now());
    let mut ide = app::build(state.clone());
    // Aberta presa ao processo do agente (o job do cliente encerra os processos dele ao fechar):
    // fica na gravação e na barra de situação, porque se o cliente reiniciar a ponte a IDE some
    // sem erro nem registro do Windows (a IDE que sumiu na conferência de 08/11).
    if aylis_tools::dap::in_kill_on_close_job() {
        ide.ui.report(&aylis_core::activity::Activity::Custom { name: "ide_no_job_do_cliente", value: "kill_on_close" });
        state.borrow_mut().status = "⚠ A IDE foi aberta presa ao programa do agente: se ele fechar ou reiniciar a conexão, a IDE fecha junto (salve com frequência)".into();
    }
    // A janela e os painéis como estavam da última vez.
    actions::restore_layout(&mut ide.ui, &ide.ids);
    if let Some(target) = &target {
        actions::open_recent(&mut ide.ui, &state, &ide.ids, Path::new(target));
        app::refresh(&mut ide.ui, &state, &ide.ids);
    }

    // A IDE sem janela, com o MCP no stdio (aylis_ide::headless): na CI, por um agente ou pelo
    // `aylis-tool ide-script`.
    #[cfg(feature = "desktop")]
    if headless {
        return aylis_ide::headless::run_mcp_stdio(ide).map_err(Into::into);
    }

    if let Some(out) = png {
        let renderer = aylis_skia::SkiaRenderer::new();
        ide.ui.services_mut().text = Some(Box::new(renderer.text_service()));
        // Seleciona um botão, se houver, para mostrar o Object Inspector preenchido.
        {
            let mut st = ide.state.borrow_mut();
            let button = st.doc.tree().into_iter().map(|(id, _)| id).find(|&id| st.doc.ui.class_name(id) == Some("Button"));
            if let Some(b) = button {
                st.doc.select(Some(b));
            }
            // --device "iPhone 15;deitado": o form na moldura do aparelho.
            if let Some(spec) = flag_value("--device") {
                st.doc.show_device(aylis_core::devices::parse(&spec));
            }
        }
        ide.ui.layout();
        app::refresh(&mut ide.ui, &ide.state, &ide.ids);
        ide.ui.layout();
        let background = ide.ui.theme().color("background");
        // --gallery: desenha a galeria de componentes em vez da janela principal.
        let form = if args.iter().any(|a| a == "--gallery") {
            let gallery = aylis_ide::gallery::open(&mut ide.ui, &ide.state, &ide.ids);
            ide.ui.layout();
            gallery
        } else if let Some(file) = flag_value("--recording") {
            // --recording arquivo.aylisrec: o visualizador da gravação.
            ide.ui.show(ide.form)?;
            let viewer = aylis_ide::recordings::show(&mut ide.ui, Path::new(&file)).ok_or("gravação inválida")?;
            ide.ui.layout();
            viewer
        } else {
            ide.form
        };
        let ui = &mut ide.ui;
        let (w, h) = ui.control(form).map_or((1440.0, 900.0), |cb| (cb.width(), cb.height()));
        let png = renderer.render_png(w as u32, h as u32, 1.0, background, |c| ui.paint(form, c)).ok_or("falha ao gerar o PNG")?;
        std::fs::write(&out, png)?;
        eprintln!("aylis-ide: IDE desenhada em {out}");
        return Ok(());
    }

    // Com a bomba, os diálogos do sistema (abrir, salvar, confirmar) não travam a janela e a
    // saída do programa em execução chega às mensagens.
    let (pump_state, ids) = (ide.state.clone(), ide.ids.clone());
    // No Windows, o Direct2D (o padrão dos apps): o OpenGL do Skia derrubou a IDE no driver de
    // vídeo (atio6axx.dll da AMD, em 08/11), e uma queda no driver não dá para segurar. O
    // `AYLIS_RENDERER` (cpu, gpu, d2d) ainda escolhe outro.
    let renderer = if cfg!(windows) && std::env::var_os("AYLIS_RENDERER").is_none() { aylis_platform_winit::Renderer::D2d } else { aylis_platform_winit::Renderer::Auto };
    let options = aylis_platform_winit::Options { icon: window_icon(), renderer, ..Default::default() };
    // A abertura (o lingote em 3D) vem antes e troca para a IDE; desligada nas Opções da IDE,
    // e as medições de desempenho e as capturas vão direto para a IDE.
    let diagnostics = ["AYLIS_BENCH", "AYLIS_SNAPSHOT"].iter().any(|v| std::env::var_os(v).is_some());
    // A sessão anterior caiu com forms alterados: oferece recuperar.
    // Com a pergunta, a IDE abre direto (a abertura e um diálogo juntos travavam).
    let pending = aylis_ide::recovery::start();
    let splash = aylis_ide::splash::enabled() && !diagnostics && ide.ui.open_forms().is_empty() && pending.is_empty() && crash.is_none();
    let first = if splash { aylis_ide::splash::show(&mut ide.ui, ide.form)? } else { ide.form };
    aylis_ide::recovery::offer(&mut ide.ui, &ide.state, &ide.ids, pending);
    // A sessão anterior quebrou: oferece relatar, com a gravação dela.
    if let Some(crash) = crash.filter(|_| !diagnostics) {
        aylis_ide::reportview::offer_after_crash(&mut ide.ui, &ide.state, &ide.ids, crash);
    }
    // No registro das IDEs abertas (o duplo clique e o MCP acham esta).
    aylis_ide::instance::start(&mut ide.ui, &ide.state, &ide.ids);
    // Versão nova do Aylis: a consulta do dia (fora das medições e das capturas).
    if !diagnostics {
        aylis_ide::updates::start(&mut ide.ui);
    }
    aylis_platform_winit::run_with_pump(ide.ui, first, options, move |waker| actions::pump(pump_state, ids, waker))?;
    aylis_ide::recovery::finish();
    aylis_ide::instance::stop();
    Ok(())
}

/// Roda a tela desenhada numa janela; eventos com handler aparecem no console.
fn preview(path: &Path, dark: bool, assets: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    let json = std::fs::read_to_string(path)?;
    let mut ui = new_document_ui(dark);
    ui.set_design_mode(false);
    if let Some(dir) = assets {
        // As fontes e os pacotes de ícones do projeto (assets/fonts, assets/icons).
        aylis_project::install_assets(&mut ui, &aylis_project::Files::Dir(PathBuf::from(&dir)));
        ui.services_mut().resources = Some(Box::new(aylis_project::DirResources(dir.into())));
    }
    // Os ícones de pacote citados no form vêm dos pacotes da IDE: o escolhido agora ainda não
    // foi para assets/icons (isso é no "Gerar código"), e um form solto nem tem a pasta.
    for (pack, names) in aylis_project::codegen::icons_cited_in(&json) {
        let file = aylis_project::codegen::resources_dir().join("icons").join(format!("{pack}.txt"));
        let Some(parsed) = std::fs::read_to_string(file).ok().and_then(|t| aylis_core::icons::parse_pack(&t).ok()) else { continue };
        for name in names {
            let full = format!("{pack}:{name}");
            if ui.icon(&full).is_none() {
                if let Some(icon) = parsed.icon(&name) {
                    ui.register_icon(&full, icon);
                }
            }
        }
    }
    ui.set_event_sink(|ui, sender, event, handler, _| {
        eprintln!("evento {event} de {} → {handler}", ui.path_of(sender));
    });
    let form = aylis_json::load(&mut ui, &json, None)?;
    aylis_platform_winit::run(ui, form)?;
    Ok(())
}

/// O ícone da janela e da barra de tarefas: a maior imagem do `.ico` da IDE (o mesmo que
/// vai no executável pelo `build.rs`).
fn window_icon() -> Option<Vec<u8>> {
    aylis_build::ico_largest_png(include_bytes!("../resources/aylis-ide.ico")).map(<[u8]>::to_vec)
}

/// Aberta de um terminal (`aylis-ide --png ...`), as mensagens vão para o console dele; com
/// dois cliques não há console e nada aparece. A saída redirecionada (pipe, arquivo) já
/// funciona sem isso.
#[cfg(windows)]
fn attach_console() {
    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn AttachConsole(process_id: u32) -> i32;
    }
    const ATTACH_PARENT_PROCESS: u32 = u32::MAX;
    // SAFETY: só liga o processo ao console do pai, se houver; falhar não tem efeito.
    unsafe {
        AttachConsole(ATTACH_PARENT_PROCESS);
    }
}

#[cfg(not(windows))]
fn attach_console() {}
