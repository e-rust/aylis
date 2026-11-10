//! # aylis-host
//!
//! Abre telas do aylis controladas por um backend em **outro processo**, em qualquer
//! linguagem. O host inicia o backend e conversa com ele por stdin/stdout, uma mensagem
//! JSON por linha (o protocolo do `aylis-remote`):
//!
//! ```text
//! aylis-host [--cpu|--gpu] [--png arquivo.png] [--project app.aylisproj] [--dev] -- <comando do backend> [argumentos...]
//! aylis-host -- python clients/python/registration_backend.py
//! aylis-host --project Vendas/Vendas.aylisproj -- python src/main.py
//! aylis-host --connect ws://servidor:9000/aylis      # o backend é um servidor WebSocket
//! aylis-host --connect wss://servidor/aylis          # com TLS (raízes do sistema)
//! aylis-host --connect wss://interno:9443 --tls-ca empresa.pem   # + uma autoridade interna
//! aylis-host --bridge 0.0.0.0:9443 --tls-cert host.pem --tls-key host.key -- python src/main.py
//! aylis-host --bridge 0.0.0.0:9000 --sessions -- python src/main.py   # um backend para todas as telas
//! ```
//!
//! Por WebSocket, o backend pode estar em outra máquina: cada mensagem de texto é uma
//! mensagem JSON do mesmo protocolo (veja `transport.rs`). A tela sempre conecta no servidor
//! (`--connect`); o host não espera conexões de backend.
//!
//! Com `--project`, o host abre o projeto antes (tema, idioma, assets, forms "criar no
//! início"), inicia o backend na pasta do projeto, e o `run` sem `form` abre o form
//! principal. O backend cria os outros com `{"cmd": "create_form", "name": "frmClientes"}`.
//!
//! 1. O backend escreve comandos (`load`, `create`, `set`, `data`...) e, quando a tela
//!    estiver pronta, `{"cmd": "run", "form": "NomeDoForm"}`.
//! 2. O host abre a janela. A partir daí, eventos com handler por nome vão para o
//!    backend (no stdin dele) e os comandos que ele mandar são aplicados na hora.
//! 3. Ao fechar a janela, o backend recebe `{"event": "closed"}` e o stdin dele é
//!    fechado. `{"cmd": "close"}` fecha a janela pelo backend.
//!
//! Com `--png`, não abre janela: segue lendo comandos até `close` (ou o fim) e grava a
//! tela num PNG — útil para testes automatizados.
//!
//! Com `--dev` (a IDE ao executar), o host lê do stdin uma linha JSON: o alvo do canal de
//! desenvolvimento (`"dev": {"address", "ide_public", "secret"}`, veja `aylis-devlink`), com que
//! conecta na IDE (inspetor, hot reload dos forms salvos, rastro de eventos), e, nos testes, um
//! roteiro (`"script"`, `"output"`, veja `aylis_remote::script`), tocado sem janela. Sem
//! `--dev`, nada disso.
//!
//! O que o backend escreve no stderr aparece no console do host.

mod bridge;
mod transport;

use std::process::{Child, Command, ExitCode, Stdio};
use std::sync::mpsc::{self, TryRecvError};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use aylis_core::{ComponentId, Ui};
use aylis_platform_winit::{Options, Renderer};
use aylis_remote::{forward_events, handle_message, register_all, resolve};
use serde_json::{Value, json};

type Error = Box<dyn std::error::Error>;
type Output = Arc<Mutex<Box<dyn transport::Outgoing>>>;

/// Quantos eventos foram para o backend (o roteiro de teste espera eles assentarem).
static EVENTS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
/// A tela encerrou sem esperar o backend (`Ui::terminate`).
static TERMINATED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

struct Args {
    png: Option<String>,
    /// `--bridge endereço`: servidor dos apps publicados (um backend por conexão).
    bridge: Option<String>,
    /// `--sessions`: na ponte, um backend só para todas as telas (cada mensagem com o número
    /// da sessão; veja `bridge.rs`).
    sessions: bool,
    project: Option<String>,
    renderer: Renderer,
    backend: Vec<String>,
    /// WebSocket: conectar no backend, que é o servidor (`ws://...`).
    connect: Option<String>,
    /// TLS: autoridade extra para `wss://` no `--connect`; certificado e chave na `--bridge`.
    tls_ca: Option<String>,
    tls_cert: Option<String>,
    tls_key: Option<String>,
    /// Os limites da `--bridge`.
    limits: bridge::Limits,
    /// `--dev`: o canal com a IDE (o alvo chega pelo stdin).
    dev: bool,
}

fn main() -> ExitCode {
    match parse_args().and_then(run) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("aylis-host: {e}");
            ExitCode::FAILURE
        }
    }
}

fn parse_args() -> Result<Args, Error> {
    let mut args = Args { png: None, bridge: None, project: None, renderer: Renderer::Auto, backend: Vec::new(), connect: None, tls_ca: None, tls_cert: None, tls_key: None, limits: bridge::Limits::default(), dev: false, sessions: false };
    let number = |value: Option<String>, option: &str| -> Result<usize, Error> {
        value.and_then(|v| v.parse().ok()).filter(|n| *n > 0).ok_or_else(|| format!("informe um número maior que zero depois de {option}").into())
    };
    let mut it = std::env::args().skip(1);
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "--" => {
                args.backend = it.collect();
                break;
            }
            "--png" => args.png = Some(it.next().ok_or("informe o arquivo depois de --png")?),
            "--dev" => args.dev = true,
            "--sessions" => args.sessions = true,
            "--project" => args.project = Some(it.next().ok_or("informe o .aylisproj depois de --project")?),
            "--connect" => args.connect = Some(it.next().ok_or("informe a URL depois de --connect (ws://...)")?),
            "--bridge" => args.bridge = Some(it.next().ok_or("informe o endereço depois de --bridge (0.0.0.0:9000)")?),
            "--tls-ca" => args.tls_ca = Some(it.next().ok_or("informe o arquivo PEM depois de --tls-ca")?),
            "--tls-cert" => args.tls_cert = Some(it.next().ok_or("informe o arquivo PEM depois de --tls-cert")?),
            "--tls-key" => args.tls_key = Some(it.next().ok_or("informe o arquivo PEM depois de --tls-key")?),
            "--max-connections" => args.limits.max_connections = number(it.next(), "--max-connections")?,
            "--max-per-ip" => args.limits.max_per_ip = number(it.next(), "--max-per-ip")?,
            "--allow-origin" => args.limits.origins.push(it.next().ok_or("informe a origem depois de --allow-origin (https://app.empresa.com)")?),
            "--cpu" => args.renderer = Renderer::Cpu,
            "--gpu" => args.renderer = Renderer::Gpu,
            other => return Err(format!("opção desconhecida '{other}'\n{USAGE}").into()),
        }
    }
    if args.backend.is_empty() && args.connect.is_none() {
        return Err(USAGE.into());
    }
    if args.tls_cert.is_some() != args.tls_key.is_some() {
        return Err("--tls-cert e --tls-key vão juntos".into());
    }
    if args.tls_cert.is_some() && args.bridge.is_none() {
        return Err("--tls-cert e --tls-key valem só com --bridge".into());
    }
    Ok(args)
}

const USAGE: &str = "uso: aylis-host [--cpu|--gpu] [--png arquivo.png] [--project app.aylisproj] \
                     (-- <comando do backend> [argumentos...] | --connect ws[s]://host:porta [--tls-ca ca.pem]) \
                     | aylis-host --bridge endereço:porta [--sessions] [--project app.aylisproj] [--tls-cert ... --tls-key ...] \
                     [--allow-origin https://app.empresa.com]... [--max-connections 200] [--max-per-ip 50] -- <comando do backend>";

fn send(out: &Output, message: &Value) {
    if let Ok(mut channel) = out.lock() {
        // O backend pode ter terminado; nesse caso não há a quem avisar.
        let _ = channel.send_line(&message.to_string());
    }
}

fn parse(line: &str) -> Option<Value> {
    if line.trim().is_empty() {
        return None;
    }
    match serde_json::from_str(line) {
        Ok(v) => Some(v),
        Err(e) => {
            eprintln!("aylis-host: linha inválida do backend ({e}): {line}");
            None
        }
    }
}

fn run(args: Args) -> Result<(), Error> {
    // Ponte: sem tela aqui; as telas são os apps que conectam.
    if let Some(address) = &args.bridge {
        let tls = match (&args.tls_cert, &args.tls_key) {
            (Some(cert), Some(key)) => Some(transport::Tls { cert: std::path::Path::new(cert), key: std::path::Path::new(key) }),
            _ => None,
        };
        let cwd = args.project.as_deref().map(aylis_project::Project::open).transpose()?.map(|p| backend_dir(&p));
        return Ok(bridge::run(address, tls, &args.backend, cwd, args.limits.clone(), args.sessions)?);
    }
    let mut ui = Ui::new();
    register_all(&mut ui);
    // Projeto: aberto antes do backend, que roda na pasta dele.
    let project = args.project.as_deref().map(aylis_project::Project::open).transpose()?;
    let main_form = match &project {
        Some(p) => Some(aylis_project::install(&mut ui, p)?),
        None => None,
    };
    // Desenvolvimento (a IDE executa com `--dev`): form salvo aparece sem reiniciar, e o
    // canal cifrado com a IDE (inspetor, rastro de eventos).
    let start = if args.dev { aylis_remote::script::start_line().ok_or("--dev: o stdin não trouxe a linha de início")? } else { Value::Null };
    let script = aylis_remote::script::from_start(&start);
    if args.dev && script.is_none() {
        if let Some(p) = &project {
            aylis_project::hot_reload(&mut ui, p);
        }
    }
    if let Some(target) = start.get("dev").and_then(aylis_devlink::Target::from_json) {
        aylis::dev_connect(&ui, target);
    }
    let connection = match &args.connect {
        Some(url) => transport::connect(url, args.tls_ca.as_deref().map(std::path::Path::new))?,
        None => {
            let mut command = Command::new(&args.backend[0]);
            if let Some(p) = &project {
                command.current_dir(backend_dir(p));
            }
            let child = command
                .args(&args.backend[1..])
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::inherit())
                .spawn()
                .map_err(|e| format!("não consegui iniciar o backend '{}': {e}", args.backend[0]))?;
            // Depurar na IDE: o depurador da linguagem se anexa a este processo.
            if std::env::var_os("AYLIS_DEBUG_PID").is_some() {
                eprintln!("AYLIS_BACKEND_PID={}", child.id());
            }
            transport::stdio(child)
        }
    };
    let transport::Connection { incoming: mut lines, outgoing, child } = connection;
    let out: Output = Arc::new(Mutex::new(outgoing));

    let events = out.clone();
    forward_events(&mut ui, move |_, event| {
        EVENTS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        send(&events, &event)
    });

    // A saída de emergência (o "não está respondendo" do segundo X): o backend travado sai
    // junto, sem os 3 s de espera.
    ui.on_terminate(|_| TERMINATED.store(true, std::sync::atomic::Ordering::Relaxed));
    let form = startup(&mut ui, &mut lines, &out, main_form)?;
    let result = match (script, &args.png) {
        (Some((script, output)), _) => scripted(ui, form, lines, &out, script, output),
        (None, Some(path)) => headless(ui, form, lines, &out, path),
        (None, None) => window(ui, form, lines, out.clone(), args.renderer),
    };
    send(&out, &json!({ "event": "closed" }));
    drop(out);
    if let Some(mut child) = child {
        finish(&mut child);
    }
    result
}

/// Processa os comandos iniciais até o backend mandar `run` (sem `form`, abre o principal
/// do projeto).
fn startup(
    ui: &mut Ui,
    lines: &mut transport::Incoming,
    out: &Output,
    main_form: Option<ComponentId>,
) -> Result<ComponentId, Error> {
    // A primeira mensagem tem de ser o `hello` (a versão do protocolo); antes dele, nada roda.
    let mut handshake = aylis_remote::Handshake::default();
    for line in lines.by_ref() {
        let Some(message) = parse(&line?) else { continue };
        if let Some(refused) = handshake.check(&message) {
            send(out, &refused);
            continue;
        }
        if message["cmd"] == "run" {
            let form = match (message.get("form").filter(|f| !f.is_null()), main_form) {
                (Some(f), _) => resolve(ui, f).map_err(|e| format!("run: {e}"))?,
                (None, Some(main)) => main,
                (None, None) => return Err("run: informe o form (ou use --project)".into()),
            };
            if let Some(id) = message.get("id") {
                send(out, &json!({ "id": id, "result": null }));
            }
            return Ok(form);
        }
        if let Some(reply) = handle_message(ui, &message) {
            send(out, &reply);
        }
    }
    Err("o backend terminou sem mandar {\"cmd\": \"run\", \"form\": ...}".into())
}

/// Abre a janela; os comandos chegam por uma thread que lê o stdout do backend.
fn window(ui: Ui, form: ComponentId, lines: transport::Incoming, out: Output, renderer: Renderer) -> Result<(), Error> {
    aylis_platform_winit::run_with_pump(ui, form, Options { renderer, ..Default::default() }, move |waker| {
        let (tx, rx) = mpsc::channel::<Value>();
        std::thread::spawn(move || {
            for line in lines {
                let Ok(line) = line else { break };
                if let Some(message) = parse(&line) {
                    if tx.send(message).is_err() {
                        break;
                    }
                    waker.wake();
                }
            }
            drop(tx); // backend terminou: a bomba fecha a janela
            waker.wake();
        });
        Box::new(move |ui: &mut Ui| loop {
            match rx.try_recv() {
                Ok(message) => {
                    if let Some(reply) = handle_message(ui, &message) {
                        send(&out, &reply);
                    }
                }
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => {
                    ui.request_close();
                    break;
                }
            }
        })
    })
}

/// Sem janela: segue até `close` (ou o fim do backend) e grava a tela em PNG.
fn headless(mut ui: Ui, form: ComponentId, lines: transport::Incoming, out: &Output, path: &str) -> Result<(), Error> {
    let renderer = aylis_skia::SkiaRenderer::new();
    ui.services_mut().text = Some(Box::new(renderer.text_service()));
    // Como na janela: o form abre (o on_show dele chega ao backend).
    let _ = ui.show(form);
    ui.process_events();
    ui.layout();
    for line in lines {
        let Some(message) = parse(&line?) else { continue };
        if let Some(reply) = handle_message(&mut ui, &message) {
            send(out, &reply);
        }
        if ui.close_requested() {
            break;
        }
    }
    let (w, h) = ui.control(form).map_or((800.0, 600.0), |cb| (cb.width(), cb.height()));
    let background = ui.theme().color("background");
    let png = renderer
        .render_png(w as u32, h as u32, 1.0, background, |canvas| ui.paint(form, canvas))
        .ok_or("falha ao gerar o PNG")?;
    std::fs::write(path, png)?;
    eprintln!("aylis-host: tela gravada em {path}");
    Ok(())
}

/// Onde o backend roda: na pasta atual, se ela está dentro do projeto (quem chamou já
/// escolheu a pasta da linguagem); senão, na pasta da linguagem ativa do projeto.
fn backend_dir(project: &aylis_project::Project) -> std::path::PathBuf {
    match std::env::current_dir() {
        Ok(cwd) if cwd.starts_with(project.dir()) => cwd,
        _ if project.code_dir().is_dir() => project.code_dir(),
        _ => project.dir(),
    }
}

/// Com um roteiro de teste (na linha do stdin, `--dev`): sem janela, toca os passos e,
/// depois de cada um, manda `{"event": "ping"}` e aplica os comandos do backend até o
/// `{"cmd": "pong"}` (o backend responde depois de tratar o que veio antes). Grava o estado
/// em `output` e a tela num PNG com o mesmo nome.
fn scripted(mut ui: Ui, form: ComponentId, lines: transport::Incoming, out: &Output, script: Value, output: Option<std::path::PathBuf>) -> Result<(), Error> {
    let renderer = aylis_skia::SkiaRenderer::new();
    ui.services_mut().text = Some(Box::new(renderer.text_service()));
    let _ = ui.show(form);
    ui.process_events();
    let (tx, rx) = mpsc::channel::<String>();
    std::thread::spawn(move || {
        for line in lines {
            let Ok(line) = line else { break };
            if tx.send(line).is_err() {
                break;
            }
        }
    });
    // Um ping por rodada; se a rodada mandou eventos ao backend (um on_error de um
    // play, por exemplo), outra rodada, até assentar.
    let mut pump = |ui: &mut Ui| loop {
        let before = EVENTS.load(std::sync::atomic::Ordering::Relaxed);
        send(out, &json!({ "event": "ping" }));
        let mut answered = false;
        while let Ok(line) = rx.recv_timeout(Duration::from_secs(60)) {
            let Some(message) = parse(&line) else { continue };
            if message["cmd"] == "pong" {
                answered = true;
                break;
            }
            if let Some(reply) = handle_message(ui, &message) {
                send(out, &reply);
            }
        }
        // Assentou: nenhum evento novo foi para o backend nesta rodada.
        if !answered || EVENTS.load(std::sync::atomic::Ordering::Relaxed) == before {
            break;
        }
    };
    aylis_remote::script::play(&mut ui, form, &script, &mut pump)?;
    let state = aylis_remote::script::state(&mut ui, &script);
    aylis_remote::script::write_state(&state, output.as_deref())?;
    if let Some(path) = output {
        let png = path.with_extension("png");
        let (w, h) = ui.control(form).map_or((800.0, 600.0), |cb| (cb.width(), cb.height()));
        let background = ui.theme().color("background");
        if let Some(bytes) = renderer.render_png(w as u32, h as u32, 1.0, background, |canvas| ui.paint(form, canvas)) {
            std::fs::write(png, bytes)?;
        }
    }
    Ok(())
}

/// Dá até 3 s para o backend terminar sozinho; depois encerra.
fn finish(child: &mut Child) {
    if TERMINATED.load(std::sync::atomic::Ordering::Relaxed) {
        let _ = child.kill();
        let _ = child.wait();
        return;
    }
    let deadline = Instant::now() + Duration::from_secs(3);
    while Instant::now() < deadline {
        if let Ok(Some(_)) = child.try_wait() {
            return;
        }
        std::thread::sleep(Duration::from_millis(50));
    }
    let _ = child.kill();
    let _ = child.wait();
}
