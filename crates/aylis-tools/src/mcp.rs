//! O acesso ao servidor MCP da IDE (agentes de IA comandando a IDE): a porta e o token
//! (`Authorization: Bearer ...`), guardados em `%LOCALAPPDATA%\aylis\mcp.json` fora de
//! qualquer projeto; a ponte stdio → HTTP (`aylis-tool mcp`), que o Claude Desktop e o Claude
//! Code iniciam e que lê o token sozinha (ele nunca fica escrito na configuração deles); e a
//! configuração automática dos dois.

use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::time::Duration;

use serde_json::{Value, json};

/// A porta padrão do servidor MCP.
pub const DEFAULT_PORT: u16 = 7341;

/// As instruções que o servidor manda ao cliente no `initialize` (a IDE e a ponte, quando a IDE
/// está fechada, mandam o mesmo texto). Elas fazem o papel de uma skill: valem em qualquer
/// cliente (o chat do Claude Desktop, o Claude Code...) sem instalar nada além do servidor, e
/// por isso o Aylis não tem plugin (decidido com o autor em 08/11).
pub const INSTRUCTIONS: &str = "\
IDE do Aylis: framework de interface com designer, forms em JSON e o código dos eventos numa \
pasta por linguagem do projeto (<projeto>/rust, <projeto>/python...).

Começo da conversa: chame session_title com um título curto do que ela vai fazer (aparece na \
barra da janela da IDE, para o usuário saber quem está nela). A primeira chamada de ferramenta \
abre a IDE, se ela não estiver aberta, e essa IDE fica ligada a esta conversa. Depois, \
project_info; sem projeto, open_project (caminho do .aylisproj) ou new_project.

Regra principal: com o projeto aberto na IDE, NÃO grave nem edite os arquivos dele com as suas \
ferramentas de arquivo (forms/*.json, temas, .aylisproj, código). A IDE guarda o que está aberto \
na memória, não vê a mudança no disco e a perde ao salvar. Mude tudo por aqui:
- código: read_code (o texto do editor, com o que não foi salvo) e edit_code (troca um trecho \
exato; o usuário desfaz com Ctrl+Z); open_code abre um fonte;
- forms: open_form, list_components, get_component, set_property, add_component, \
delete_component, get_layout e set_layout;
- salvar: save (com all, tudo o que mudou);
- generated/ é somente leitura: generate_code refaz.

Compilar e executar: build compila e espera terminar (pode levar minutos); a resposta diz se \
compilou, com os erros e as últimas linhas. Corrija com edit_code e chame build de novo. \
run_without_debug executa, run executa no depurador; output mostra as mensagens e status a \
situação. Com o app aberto, app_command lê e mexe na tela dele ({\"cmd\": \"tree\"}, \
{\"cmd\": \"click\", \"target\": \"btnSalvar\"}).

Como o código do Aylis é escrito: componentes pelo acesso tipado (self.btnSalvar, \
frmCliente.edtNome), nunca procurando pelo nome em texto; handlers devolvem Result e a recusa é \
um erro; mensagem ao usuário pelo MessageDialog do Aylis, nunca a caixa do sistema; nada de banco \
na tela (o ClientDataSet guarda os dados e o delta, o backend aplica); thread volta à tela por \
post/post_to (no navegador, aylis::background).

O que não estiver nas ferramentas: ide_menus lista o que o usuário faz pelos menus; menu escolhe \
um item e os dialog_* preenchem o diálogo que abrir. usages acha onde um componente, campo ou \
handler é usado.";

/// A porta e o token do servidor.
#[derive(Clone, Debug, PartialEq)]
pub struct Access {
    pub port: u16,
    pub token: String,
}

fn file() -> Option<PathBuf> {
    Some(crate::dap::tools_dir()?.parent()?.join("mcp.json"))
}

fn random_token() -> Result<String, String> {
    let mut bytes = [0u8; 32];
    crate::random_bytes(&mut bytes)?;
    Ok(bytes.iter().map(|b| format!("{b:02x}")).collect())
}

fn save(access: &Access) -> Result<(), String> {
    let path = file().ok_or("sem pasta de dados do usuário")?;
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    }
    let text = serde_json::to_string_pretty(&json!({ "port": access.port, "token": access.token })).map_err(|e| e.to_string())?;
    std::fs::write(&path, text + "\n").map_err(|e| format!("{}: {e}", path.display()))
}

/// A porta e o token; na primeira vez, gera o token e grava.
pub fn access() -> Result<Access, String> {
    let saved = file().and_then(|p| std::fs::read_to_string(p).ok()).and_then(|t| serde_json::from_str::<Value>(&t).ok());
    let port = saved.as_ref().and_then(|v| v["port"].as_u64()).and_then(|p| u16::try_from(p).ok()).filter(|p| *p != 0).unwrap_or(DEFAULT_PORT);
    match saved.as_ref().and_then(|v| v["token"].as_str()).filter(|t| t.len() >= 32) {
        Some(token) => Ok(Access { port, token: token.to_string() }),
        None => {
            let access = Access { port, token: random_token()? };
            save(&access)?;
            Ok(access)
        }
    }
}

/// Troca a porta (o token fica).
pub fn set_port(port: u16) -> Result<(), String> {
    let mut a = access()?;
    a.port = if port == 0 { DEFAULT_PORT } else { port };
    save(&a)
}

/// Gera um token novo: o antigo para de valer (quem conecta pela ponte não precisa mudar nada).
pub fn new_token() -> Result<Access, String> {
    let mut a = access()?;
    a.token = random_token()?;
    save(&a)?;
    Ok(a)
}

/// O cabeçalho `Authorization` traz o token? Comparação em tempo constante.
pub fn authorized(header: Option<&str>, token: &str) -> bool {
    let Some(given) = header.and_then(|h| h.strip_prefix("Bearer ")).map(str::trim) else { return false };
    let (a, b) = (given.as_bytes(), token.as_bytes());
    a.len() == b.len() && a.iter().zip(b).fold(0u8, |acc, (x, y)| acc | (x ^ y)) == 0
}

/// `aylis-tool mcp`: a ponte entre o cliente MCP (stdio, uma mensagem JSON por linha) e o
/// servidor HTTP da IDE, com o token. Com a IDE fechada (ou o MCP desligado), cada pedido
/// recebe um erro que diz o que fazer.
pub fn bridge() -> Result<(), String> {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let mut pinned: Option<u32> = None;
    let session = new_session();
    // A IDE não respondia: quando volta a responder, o cliente é avisado para buscar a lista de
    // ferramentas de novo (a de quando ela estava fechada pode ter só as da ponte).
    let mut offline = false;
    // Quem está nesta sessão (o cliente, do aperto de mão, e o título que o agente deu) e a IDE
    // que já sabe disso: a IDE mostra na barra da janela.
    let mut client = String::from("Agente");
    let mut title = String::new();
    let mut told: Option<u32> = None;
    for line in stdin.lock().lines() {
        let line = line.map_err(|e| e.to_string())?;
        if line.trim().is_empty() {
            continue;
        }
        let request = serde_json::from_str::<Value>(&line).unwrap_or(Value::Null);
        if request["method"] == "initialize" {
            client = client_name(request["params"]["clientInfo"]["name"].as_str().unwrap_or_default());
        }
        // `ide_list` e `ide_select` a ponte responde (é ela quem escolhe a IDE).
        if let Some(reply) = own_tool(&request, &mut pinned) {
            writeln!(stdout, "{reply}").and_then(|_| stdout.flush()).map_err(|e| e.to_string())?;
            continue;
        }
        // `session_title`: guarda o título, abre a IDE se preciso e mostra nela.
        if request["method"] == "tools/call" && request["params"]["name"] == "session_title" {
            title = request["params"]["arguments"]["title"].as_str().unwrap_or_default().trim().to_string();
            let (text, opened) = match target_port(&mut pinned, &session, true) {
                Ok(_) => {
                    announce(&mut told, pinned, &session, &client, &title);
                    (format!("Título desta sessão na IDE do Aylis: {title}"), true)
                }
                Err(e) => (e, false),
            };
            let reply = json!({ "jsonrpc": "2.0", "id": request["id"].clone(), "result": { "content": [{ "type": "text", "text": text }] } });
            writeln!(stdout, "{reply}").and_then(|_| stdout.flush()).map_err(|e| e.to_string())?;
            // A lista de ferramentas saiu sem a IDE (só as da ponte, quando não havia uma guardada)
            // e esta chamada abriu a IDE: o cliente precisa pedir a lista de novo, senão fica sem as
            // ferramentas dela (aconteceu numa sessão do Claude Code em 08/11).
            if opened && offline {
                offline = false;
                writeln!(stdout, "{}", json!({ "jsonrpc": "2.0", "method": "notifications/tools/list_changed" })).and_then(|_| stdout.flush()).map_err(|e| e.to_string())?;
            }
            continue;
        }
        // Um projeto já aberto noutra IDE: esta sessão passa para aquela (um projeto, uma IDE;
        // decidido com o autor em 08/11), e as sessões dele trabalham juntas, uma chamada de cada
        // vez. Um projeto que não está aberto, pedido numa IDE que outra sessão também usa: abre
        // numa IDE só desta sessão, sem tirar o projeto da outra.
        if request["method"] == "tools/call" && request["params"]["name"] == "open_project" {
            let path = PathBuf::from(request["params"]["arguments"]["path"].as_str().unwrap_or_default());
            if let Err(e) = route_project(&mut pinned, &session, &path) {
                let reply = json!({ "jsonrpc": "2.0", "id": request["id"].clone(), "result": { "content": [{ "type": "text", "text": e }], "isError": true } });
                writeln!(stdout, "{reply}").and_then(|_| stdout.flush()).map_err(|e| e.to_string())?;
                continue;
            }
        }
        let launch = request["method"] == "tools/call";
        // Sem IDE fixada, o aperto de mão, a lista e os avisos a ponte responde sozinha: fixar uma
        // IDE "livre" já aí pegava a de outra conversa muito antes de esta usar o Aylis (roteiros 92
        // e 95, em 08/11). A IDE é escolhida na primeira chamada de ferramenta.
        if !launch && pinned.is_none() {
            let Some(id) = request.get("id").cloned() else { continue };
            writeln!(stdout, "{}", without_ide(&request, id, "a IDE do Aylis ainda não está ligada a esta conversa: a primeira chamada de ferramenta abre ou escolhe uma")).and_then(|_| stdout.flush()).map_err(|e| e.to_string())?;
            continue;
        }
        let ides_before: Vec<u32> = crate::instances::list().iter().map(|i| i.pid).collect();
        let target = access().and_then(|a| Ok(Access { port: target_port(&mut pinned, &session, launch)?, ..a }));
        // Uma IDE nova para esta sessão: ela fica sabendo quem está nela (e a de antes, que saiu).
        // A sessão que sumiu do registro da IDE (tirada por engano, ou a IDE reiniciou o registro)
        // também se anuncia de novo, senão a IDE parece livre para outra conversa.
        if target.is_ok() && pinned.is_some() && (pinned != told || !registered(pinned, &session)) {
            announce(&mut told, pinned, &session, &client, &title);
        }
        // Esta chamada abriu a IDE: a resposta diz isso (no Claude Desktop, a conversa concluiu "não
        // achei a IDE" porque a chamada respondeu sem avisar que tinha aberto uma).
        let opened = pinned.is_some_and(|p| !ides_before.contains(&p));
        let reply = match target.and_then(|a| post_waiting(a, line.clone(), &request, &mut stdout)) {
            Ok(Some(body)) if opened && request["method"] == "tools/call" => with_note(&body, "(Abri a IDE do Aylis para esta conversa.)"),
            Ok(Some(body)) if request["method"] == "tools/list" => add_own_tools(&body),
            Ok(Some(body)) if request["method"] == "initialize" => {
                remember("initialize", &body);
                body
            }
            Ok(Some(body)) => body,
            Ok(None) => continue,
            Err(e) => {
                offline = true;
                // Só pedidos (com `id`) têm resposta; uma notificação perdida não tem a quem avisar.
                let Some(id) = request.get("id").cloned() else { continue };
                without_ide(&request, id, &e)
            }
        };
        let answered = !reply.contains("\"isError\":true") && !reply.contains("\"error\":");
        writeln!(stdout, "{}", reply.trim_end()).and_then(|_| stdout.flush()).map_err(|e| e.to_string())?;
        if offline && answered && request["method"] != "initialize" && request["method"] != "tools/list" && crate::instances::list().iter().any(|i| i.mcp_port.is_some()) {
            offline = false;
            writeln!(stdout, "{}", json!({ "jsonrpc": "2.0", "method": "notifications/tools/list_changed" })).and_then(|_| stdout.flush()).map_err(|e| e.to_string())?;
        }
    }
    // A sessão acabou (o cliente fechou a ponte): a IDE tira o nome dela da janela.
    if let Some(pid) = told {
        let _ = tell_pid(pid, &session, &client, &title, false);
    }
    Ok(())
}

/// Diz à IDE `pinned` que esta sessão está nela (com o título atual) e, se a sessão estava
/// noutra (`told`), que saiu de lá: aquela fica livre para outra sessão.
fn announce(told: &mut Option<u32>, pinned: Option<u32>, session: &str, client: &str, title: &str) {
    if let Some(old) = told.filter(|t| Some(*t) != pinned) {
        let _ = tell_pid(old, session, client, title, false);
    }
    if let Some(pid) = pinned {
        let _ = tell_pid(pid, session, client, title, true);
    }
    *told = pinned;
}

fn tell_pid(pid: u32, session: &str, client: &str, title: &str, connected: bool) -> Result<(), String> {
    let port = crate::instances::list().into_iter().find(|i| i.pid == pid).and_then(|i| i.mcp_port).ok_or("a IDE fechou")?;
    tell_session(&Access { port, ..access()? }, session, client, title, connected)
}

/// Para onde vai o `open_project` desta sessão (veja o laço da ponte): a IDE que já tem o
/// projeto; senão, a fixada, se só esta sessão a usa; senão, uma livre na tela inicial ou uma
/// nova, só desta sessão.
fn route_project(pinned: &mut Option<u32>, session: &str, path: &Path) -> Result<(), String> {
    let mut ides: Vec<crate::instances::Instance> = crate::instances::list().into_iter().filter(|i| i.mcp_port.is_some()).collect();
    forget_dead_sessions(&mut ides);
    if let Some(i) = ides.iter().find(|i| i.project.as_ref().is_some_and(|p| crate::instances::same_path(p, path))) {
        *pinned = Some(i.pid);
        return Ok(());
    }
    let shared = pinned.and_then(|pid| ides.iter().find(|i| i.pid == pid)).is_some_and(|i| i.sessions.iter().any(|s| s != session));
    if shared {
        // Uma IDE livre na tela inicial serve; senão, uma nova.
        *pinned = Some(match ides.iter().find(|i| i.sessions.is_empty() && i.project.is_none()) {
            Some(i) => i.pid,
            None => open_ide(session)?.pid,
        });
    }
    Ok(())
}

/// Pega a IDE livre `ide` para `session`: anuncia e confere no registro que a primeira sessão
/// dela é esta (duas pontes que acharam a mesma IDE livre ao mesmo tempo: fica a primeira; a
/// outra desiste do anúncio e abre a sua IDE).
fn claim(ide: &crate::instances::Instance, session: &str) -> bool {
    let (Some(port), Ok(a)) = (ide.mcp_port, access()) else { return false };
    let access = Access { port, ..a };
    if tell_session(&access, session, "", "", true).is_err() {
        return false;
    }
    let first = crate::instances::list().into_iter().find(|i| i.pid == ide.pid).and_then(|i| i.sessions.first().cloned());
    if first.as_deref() == Some(session) {
        return true;
    }
    let _ = tell_session(&access, session, "", "", false);
    false
}

/// A sessão está no registro da IDE `pid`.
fn registered(pid: Option<u32>, session: &str) -> bool {
    pid.and_then(|pid| crate::instances::list().into_iter().find(|i| i.pid == pid)).is_some_and(|i| i.sessions.iter().any(|s| s == session))
}

/// Acrescenta uma nota ao texto da resposta de uma ferramenta.
fn with_note(body: &str, note: &str) -> String {
    let Ok(mut v) = serde_json::from_str::<Value>(body) else { return body.to_string() };
    if let Some(content) = v["result"]["content"].as_array_mut() {
        content.insert(0, json!({ "type": "text", "text": note }));
    }
    v.to_string()
}

/// Avisa a IDE de `access` quem está ligado a ela (`aylis/session`).
fn tell_session(access: &Access, session: &str, client: &str, title: &str, connected: bool) -> Result<(), String> {
    let body = json!({ "jsonrpc": "2.0", "id": "session", "method": "aylis/session", "params": { "session": session, "client": client, "title": title, "connected": connected } });
    post(access, &body.to_string()).map(|_| ())
}

/// O nome do cliente para a janela da IDE ("claude-ai" → "Claude Desktop").
fn client_name(raw: &str) -> String {
    let lower = raw.to_lowercase();
    // O modo Claude (agente local) do Claude Desktop se apresenta como "local-agent-mode-<servidor>".
    if lower == "claude-ai" || lower.contains("claude desktop") || lower.starts_with("local-agent-mode") {
        "Claude Desktop".into()
    } else if lower.contains("claude-code") || lower.contains("claude code") {
        "Claude Code".into()
    } else if raw.is_empty() {
        "Agente".into()
    } else {
        raw.to_string()
    }
}

/// A resposta da ponte quando a IDE não responde (fechada ou com o MCP desligado). O cliente
/// (o Claude Desktop) liga a ponte ao abrir, e antes um erro no `initialize` deixava o servidor
/// como "Falhou" até reiniciar o cliente (relatado pelo autor em 08/11): agora o aperto de mão sai
/// da última vez que a IDE respondeu (ou do padrão), a lista de ferramentas é a da IDE
/// ([`ide_tools`]) mais as da ponte, e uma chamada de
/// ferramenta volta como erro da ferramenta, dizendo o que fazer. Quando a IDE abre, as chamadas
/// seguintes já vão para ela.
fn without_ide(request: &Value, id: Value, error: &str) -> String {
    let result = match request["method"].as_str().unwrap_or_default() {
        // A resposta guardada pode ser de uma IDE de antes: as instruções são sempre as desta.
        "initialize" => recalled("initialize").map(|v| v["result"].clone()).filter(|r| r.is_object()).map(|mut r| {
            r["instructions"] = json!(INSTRUCTIONS);
            r
        }).unwrap_or_else(|| {
            let version = request["params"]["protocolVersion"].as_str().unwrap_or("2025-06-18");
            json!({
                "protocolVersion": version,
                "capabilities": { "tools": { "listChanged": true } },
                "serverInfo": { "name": "aylis-ide", "version": env!("CARGO_PKG_VERSION") },
                "instructions": INSTRUCTIONS,
            })
        }),
        "tools/list" => {
            // A lista da IDE é a do mesmo código (não a guardada, que pode ser de uma IDE de antes
            // ou nem existir): o cliente que não pede de novo quando a IDE abre já fica com todas.
            let mut tools = ide_tools();
            tools.extend(own_tools().as_array().cloned().unwrap_or_default());
            json!({ "tools": tools })
        }
        "ping" => json!({}),
        "tools/call" => json!({ "content": [{ "type": "text", "text": error }], "isError": true }),
        _ => return json!({ "jsonrpc": "2.0", "id": id, "error": { "code": -32000, "message": error } }).to_string(),
    };
    json!({ "jsonrpc": "2.0", "id": id, "result": result }).to_string()
}

/// Onde a ponte guarda a última resposta da IDE a `what` (`initialize`, `tools`).
fn memory_file(what: &str) -> Option<PathBuf> {
    Some(crate::dap::tools_dir()?.parent()?.join(format!("mcp-{what}.json")))
}

fn remember(what: &str, body: &str) {
    if let Some(path) = memory_file(what) {
        let _ = std::fs::write(path, body);
    }
}

fn recalled(what: &str) -> Option<Value> {
    serde_json::from_str(&std::fs::read_to_string(memory_file(what)?).ok()?).ok()
}

/// A porta da IDE desta sessão: a fixada (com `ide_select`, ou na primeira chamada).
///
/// A sessão (`session`, uma por ponte, isto é, por conexão do cliente) fica ligada à IDE: a que
/// ela abriu, ou a aberta pelo usuário que ela fixou. Uma IDE aberta por outra sessão não é
/// pega. Sem IDE para ela e com `launch` (uma chamada de ferramenta, não o aperto de mão nem a
/// lista), a ponte abre a IDE com o MCP ligado e a marca da sessão, e espera ela ficar pronta
/// (pedido do autor em 08/11: "caso o Aylis não esteja aberto o MCP deve abri-lo para a sessão
/// do Claude que pediu").
fn target_port(pinned: &mut Option<u32>, session: &str, launch: bool) -> Result<u16, String> {
    let ides: Vec<crate::instances::Instance> = crate::instances::list().into_iter().filter(|i| i.mcp_port.is_some()).collect();
    if let Some(pid) = *pinned {
        if let Some(port) = ides.iter().find(|i| i.pid == pid).and_then(|i| i.mcp_port) {
            return Ok(port);
        }
        // A IDE da sessão fechou: na próxima chamada de ferramenta, abre outra.
        *pinned = None;
        if !launch {
            return Err("a IDE desta sessão fechou; a próxima chamada de ferramenta abre outra".into());
        }
    }
    let mut ides = ides;
    forget_dead_sessions(&mut ides);
    if let Some(i) = ides.iter().find(|i| i.sessions.iter().any(|s| s == session)) {
        *pinned = Some(i.pid);
        return Ok(i.mcp_port.unwrap_or_default());
    }
    // Uma IDE livre (sem sessão): esta sessão a pega e se anuncia na hora; se outra pegou a mesma
    // no mesmo instante, fica com quem chegou primeiro ao registro, e esta abre a sua.
    if launch {
        if let Some(i) = ides.iter().find(|i| i.sessions.is_empty()) {
            if claim(i, session) {
                *pinned = Some(i.pid);
                return Ok(i.mcp_port.unwrap_or_default());
            }
        }
    }
    if !launch {
        return Err("a IDE do Aylis ainda não está aberta: a primeira chamada de ferramenta abre ela".into());
    }
    let ide = open_ide(session)?;
    *pinned = Some(ide.pid);
    Ok(ide.mcp_port.unwrap_or_default())
}

/// Abre a IDE (o `aylis-ide` ao lado deste `aylis-tool`) com o MCP ligado e a marca da sessão, e
/// espera ela aparecer no registro das IDEs com o servidor pronto.
fn open_ide(session: &str) -> Result<crate::instances::Instance, String> {
    let exe = std::env::current_exe().map_err(|e| e.to_string())?.with_file_name(if cfg!(windows) { "aylis-ide.exe" } else { "aylis-ide" });
    if !exe.is_file() {
        return Err(format!("não achei a IDE do Aylis em {} para abrir", exe.display()));
    }
    // A sessão pode já estar numa IDE (dividida com outra sessão): espera a nova, não aquela.
    let before: Vec<u32> = crate::instances::list().iter().map(|i| i.pid).collect();
    let mut command = std::process::Command::new(&exe);
    command
        .env("AYLIS_IDE_MCP", "0")
        .env("AYLIS_IDE_MCP_SESSION", session)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null());
    // A IDE sobrevive à ponte (o cliente pode encerrar a ponte e o grupo de processos dela).
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const DETACHED_PROCESS: u32 = 0x0000_0008;
        const CREATE_BREAKAWAY_FROM_JOB: u32 = 0x0100_0000;
        command.creation_flags(DETACHED_PROCESS | CREATE_BREAKAWAY_FROM_JOB);
        if command.spawn().is_err() {
            command.creation_flags(DETACHED_PROCESS);
            command.spawn().map_err(|e| format!("não consegui abrir a IDE ({}): {e}", exe.display()))?;
            // A IDE ficou no job do cliente: se ele encerra os processos ao fechar, ela morre
            // junto com a ponte, sem aviso. Vai para o log do servidor MCP do cliente.
            if crate::dap::in_kill_on_close_job() {
                eprintln!("aylis-tool mcp: o cliente não deixou a IDE sair do job dele; ela fecha junto quando o cliente encerrar ou reiniciar esta ponte");
            }
        }
    }
    #[cfg(not(windows))]
    command.spawn().map_err(|e| format!("não consegui abrir a IDE ({}): {e}", exe.display()))?;
    let deadline = std::time::Instant::now() + Duration::from_secs(90);
    while std::time::Instant::now() < deadline {
        if let Some(ide) = crate::instances::list().into_iter().find(|i| i.sessions.iter().any(|s| s == session) && !before.contains(&i.pid) && i.mcp_port.is_some()) {
            return Ok(ide);
        }
        std::thread::sleep(Duration::from_millis(250));
    }
    Err("abri a IDE do Aylis, mas ela não ficou pronta em 90 s".into())
}

/// A marca desta sessão (desta ponte): aleatória, uma por processo.
fn new_session() -> String {
    let mut bytes = [0u8; 8];
    let _ = crate::random_bytes(&mut bytes);
    let random: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
    // O pid da ponte na frente: uma ponte encerrada à força (sem avisar a IDE) é reconhecida.
    format!("{}-{random}", std::process::id())
}

/// A ponte da sessão ainda roda (uma marca sem pid conta como viva).
fn session_alive(session: &str) -> bool {
    session.split_once('-').and_then(|(pid, _)| pid.parse::<u32>().ok()).is_none_or(crate::dap::process_alive)
}

/// Tira das IDEs as sessões de pontes que acabaram sem avisar (o cliente encerrou à força): a
/// IDE delas volta a ficar livre e o nome sai da janela.
fn forget_dead_sessions(ides: &mut [crate::instances::Instance]) {
    for ide in ides.iter_mut() {
        let dead: Vec<String> = ide.sessions.iter().filter(|s| !session_alive(s)).cloned().collect();
        for s in &dead {
            if let (Some(port), Ok(a)) = (ide.mcp_port, access()) {
                let _ = tell_session(&Access { port, ..a }, s, "", "", false);
            }
        }
        ide.sessions.retain(|s| !dead.contains(s));
    }
}

/// As ferramentas da própria ponte.
fn own_tools() -> Value {
    json!([
        { "name": "session_title", "description": "Dá um título a esta sessão (um resumo curto do que ela vai fazer), que aparece na barra da janela da IDE do Aylis ligada a ela. Chame no começo da conversa; abre a IDE se ela não estiver aberta.", "inputSchema": { "type": "object", "properties": { "title": { "type": "string", "description": "o título, curto (ex.: Tela de cadastro de clientes)" } }, "required": ["title"] } },
        { "name": "ide_list", "description": "Lista as IDEs do Aylis abertas (cada uma com o projeto dela) e qual está fixada nesta conversa.", "inputSchema": { "type": "object", "properties": {} } },
        { "name": "ide_select", "description": "Fixa nesta conversa a IDE do projeto informado (o caminho do .aylisproj ou parte do nome, como ide_list mostra). As chamadas seguintes vão para ela.", "inputSchema": { "type": "object", "properties": { "project": { "type": "string", "description": "caminho ou parte do nome do projeto; vazio: a IDE sem projeto" } }, "required": ["project"] } }
    ])
}

/// Acrescenta as ferramentas da ponte à lista que a IDE mandou.
fn add_own_tools(body: &str) -> String {
    let Ok(mut v) = serde_json::from_str::<Value>(body) else { return body.to_string() };
    if let (Some(tools), Some(extra)) = (v["result"]["tools"].as_array_mut(), own_tools().as_array()) {
        tools.extend(extra.iter().cloned());
    }
    v.to_string()
}

/// Responde `ide_list` e `ide_select` (as outras chamadas vão para a IDE).
fn own_tool(request: &Value, pinned: &mut Option<u32>) -> Option<String> {
    if request["method"] != "tools/call" {
        return None;
    }
    let name = request["params"]["name"].as_str()?;
    if name != "ide_list" && name != "ide_select" {
        return None;
    }
    let ides: Vec<crate::instances::Instance> = crate::instances::list().into_iter().filter(|i| i.mcp_port.is_some()).collect();
    let describe = |i: &crate::instances::Instance| i.project.as_ref().map_or_else(|| "(sem projeto)".to_string(), |p| p.display().to_string());
    let text = if name == "ide_list" {
        if ides.is_empty() {
            "Nenhuma IDE do Aylis aberta com o MCP ligado: a próxima chamada de ferramenta (project_info, por exemplo) abre uma para esta conversa.".to_string()
        } else {
            ides.iter().map(|i| format!("{}{}", if *pinned == Some(i.pid) { "* " } else { "  " }, describe(i))).collect::<Vec<_>>().join("
")
        }
    } else {
        let wanted = request["params"]["arguments"]["project"].as_str().unwrap_or_default().to_lowercase();
        let found = ides.iter().find(|i| if wanted.is_empty() { i.project.is_none() } else { describe(i).to_lowercase().contains(&wanted) });
        match found {
            Some(i) => {
                *pinned = Some(i.pid);
                format!("Fixada: {}", describe(i))
            }
            None => format!("Nenhuma IDE aberta com o projeto \"{wanted}\" (ide_list mostra as abertas)."),
        }
    };
    Some(json!({ "jsonrpc": "2.0", "id": request["id"].clone(), "result": { "content": [{ "type": "text", "text": text }] } }).to_string())
}

/// O [`post`] numa thread, e enquanto a IDE trabalha (uma compilação pelo `build` leva minutos),
/// o andamento a cada 10 s para o cliente que pediu (`_meta.progressToken`): ele sabe que o Aylis
/// continua e não desiste da chamada.
fn post_waiting(access: Access, body: String, request: &Value, out: &mut impl Write) -> Result<Option<String>, String> {
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let _ = tx.send(post(&access, &body));
    });
    let token = request["params"]["_meta"]["progressToken"].clone();
    let mut waited = 0u64;
    loop {
        match rx.recv_timeout(Duration::from_secs(10)) {
            Ok(result) => return result,
            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                waited += 10;
                if !token.is_null() {
                    let note = json!({ "jsonrpc": "2.0", "method": "notifications/progress", "params": { "progressToken": token, "progress": waited, "message": format!("O Aylis ainda está trabalhando ({waited} s)") } });
                    let _ = writeln!(out, "{note}").and_then(|_| out.flush());
                }
            }
            Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => return Err("a ponte perdeu a resposta da IDE".into()),
        }
    }
}

/// Manda uma mensagem ao servidor da IDE; `None` quando ele aceita sem resposta (202).
fn post(access: &Access, body: &str) -> Result<Option<String>, String> {
    let closed = || "a IDE do Aylis não está aberta com o servidor MCP ligado (Ferramentas → Opções da IDE → MCP)".to_string();
    let mut stream = TcpStream::connect(("127.0.0.1", access.port)).map_err(|_| closed())?;
    // Uma compilação pelo `build` espera até 30 minutos na IDE.
    stream.set_read_timeout(Some(Duration::from_secs(35 * 60))).map_err(|e| e.to_string())?;
    let head = format!(
        "POST /mcp HTTP/1.1\r\nHost: 127.0.0.1:{}\r\nContent-Type: application/json\r\nAccept: application/json\r\nAuthorization: Bearer {}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        access.port,
        access.token,
        body.len()
    );
    stream.write_all(head.as_bytes()).and_then(|_| stream.write_all(body.as_bytes())).map_err(|_| closed())?;
    let mut reader = BufReader::new(stream);
    let mut status = String::new();
    reader.read_line(&mut status).map_err(|_| closed())?;
    let mut length = 0usize;
    loop {
        let mut h = String::new();
        if reader.read_line(&mut h).map_err(|e| e.to_string())? == 0 || h.trim().is_empty() {
            break;
        }
        if let Some((k, v)) = h.split_once(':') {
            if k.trim().eq_ignore_ascii_case("content-length") {
                length = v.trim().parse().unwrap_or(0);
            }
        }
    }
    let code = status.split_whitespace().nth(1).unwrap_or_default();
    match code {
        "200" => {
            let mut text = vec![0u8; length];
            reader.read_exact(&mut text).map_err(|e| e.to_string())?;
            Ok(Some(String::from_utf8_lossy(&text).into_owned()))
        }
        "202" => Ok(None),
        "401" => Err("o token do MCP não confere (gere de novo nas Opções da IDE)".into()),
        other => Err(format!("a IDE respondeu {other}")),
    }
}

/// Onde o Claude Desktop guarda a configuração: no Windows, a pasta normal e, se o Claude foi
/// instalado como pacote (MSIX), a cópia do pacote (é ela que ele lê); no macOS, a de
/// Application Support. No Linux não há Claude Desktop oficial.
pub fn claude_desktop_configs() -> Vec<PathBuf> {
    let mut out = Vec::new();
    if cfg!(windows) {
        if let Some(appdata) = std::env::var_os("APPDATA") {
            out.push(PathBuf::from(appdata).join("Claude").join("claude_desktop_config.json"));
        }
        if let Some(local) = std::env::var_os("LOCALAPPDATA") {
            if let Ok(entries) = std::fs::read_dir(PathBuf::from(local).join("Packages")) {
                for e in entries.flatten() {
                    if e.file_name().to_string_lossy().starts_with("Claude_") {
                        out.push(e.path().join("LocalCache").join("Roaming").join("Claude").join("claude_desktop_config.json"));
                    }
                }
            }
        }
    } else if cfg!(target_os = "macos") {
        if let Some(home) = std::env::var_os("HOME") {
            out.push(Path::new(&home).join("Library/Application Support/Claude/claude_desktop_config.json"));
        }
    }
    out
}

/// Põe o aylis no `mcpServers` da configuração em `path`, mantendo os outros servidores e
/// guardando uma cópia (`.bak`) antes de mudar.
pub fn add_to_claude_desktop(path: &Path, aylis_tool: &Path) -> Result<(), String> {
    let mut config = match std::fs::read_to_string(path) {
        Ok(text) => {
            let _ = std::fs::write(path.with_extension("json.bak"), &text);
            serde_json::from_str::<Value>(&text).map_err(|e| format!("{}: JSON inválido ({e}); não mexi nele", path.display()))?
        }
        Err(_) => json!({}),
    };
    if !config.is_object() {
        return Err(format!("{}: não é um objeto JSON; não mexi nele", path.display()));
    }
    if !config["mcpServers"].is_object() {
        config["mcpServers"] = json!({});
    }
    config["mcpServers"]["aylis"] = json!({ "command": aylis_tool.display().to_string(), "args": ["mcp"] });
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    }
    let text = serde_json::to_string_pretty(&config).map_err(|e| e.to_string())?;
    std::fs::write(path, text + "\n").map_err(|e| format!("{}: {e}", path.display()))
}

/// O comando que registra o aylis no Claude Code (para todos os projetos do usuário).
pub fn claude_code_args(aylis_tool: &Path) -> Vec<String> {
    ["mcp", "add", "--scope", "user", "aylis", "--"].iter().map(|s| s.to_string()).chain([aylis_tool.display().to_string(), "mcp".into()]).collect()
}

/// Uma ferramenta da IDE: nome, descrição e o esquema dos argumentos.
fn tool(name: &str, description: &str, properties: Value, required: &[&str]) -> Value {
    json!({ "name": name, "description": description, "inputSchema": { "type": "object", "properties": properties, "required": required } })
}

/// As ferramentas que a IDE oferece. Ficam aqui, e não na IDE, para a ponte responder a lista
/// inteira mesmo com a IDE fechada: o Claude Code não pede a lista de novo quando a IDE abre
/// (o `notifications/tools/list_changed` é ignorado; visto em 08/11).
pub fn ide_tools() -> Vec<Value> {
    let s = |d: &str| json!({ "type": "string", "description": d });
    vec![
        tool("project_info", "O projeto aberto: nome, linguagem, forms (principal, criados no início, DataModules, variantes, uses) e o form aberto.", json!({}), &[]),
        tool("open_form", "Abre um form (ou DataModule) do projeto no designer; `variant` abre a variante de uma plataforma (android, desktop...).", json!({ "name": s("nome do form"), "variant": s("variante, opcional") }), &["name"]),
        tool("list_components", "A árvore de componentes do form aberto: nome, classe e pai.", json!({}), &[]),
        tool("get_component", "As propriedades de um componente do form aberto (como no layout JSON). Vazio: o form.", json!({ "component": s("nome do componente") }), &[]),
        tool("set_property", "Muda uma propriedade de um componente do form aberto (desfazível). `value` é o valor JSON como no layout: número, texto, booleano, cor \"#rrggbb\", nome de outro componente...", json!({ "component": s("nome do componente (vazio: o form)"), "property": s("nome da propriedade"), "value": { "description": "o valor" } }), &["property", "value"]),
        tool("add_component", "Cria um componente da paleta no form aberto (`class`, pode ser um subtipo como Button.icon), dentro de `parent` (opcional) e com `name` (opcional). Devolve o nome.", json!({ "class": s("classe"), "parent": s("contêiner, opcional"), "name": s("nome, opcional") }), &["class"]),
        tool("delete_component", "Exclui um componente do form aberto.", json!({ "component": s("nome") }), &["component"]),
        tool("get_layout", "O layout JSON inteiro do form aberto.", json!({}), &[]),
        tool("set_layout", "Troca o layout inteiro do form aberto pelo JSON dado (desfazível).", json!({ "layout": s("o documento JSON ({\"aylis\":1,\"root\":{...}})") }), &["layout"]),
        tool("list_classes", "As classes da paleta, por categoria, com os subtipos.", json!({}), &[]),
        tool("save", "Salva o form aberto (ou, com `all`, todos os forms e os fontes alterados nos editores).", json!({ "all": { "type": "boolean" } }), &[]),
        tool("generate_code", "Gera (ou atualiza) o código do projeto: handlers dos forms e Cargo.toml.", json!({}), &[]),
        tool("open_project", "Abre um projeto (.aylisproj), como Arquivo → Abrir projeto.", json!({ "path": s("caminho do .aylisproj") }), &["path"]),
        tool("open_code", "Abre na janela de código (uma aba por arquivo) o código de um form (F12) ou um fonte qualquer (`file`), na linha dada.", json!({ "form": s("nome do form (vazio: o aberto)"), "file": s("caminho de um fonte, opcional"), "line": { "type": "integer" } }), &[]),
        tool("read_code", "O texto de um fonte como está no editor da IDE (com o que ainda não foi salvo), ou do disco se não estiver aberto: o código de um form (`form`) ou um fonte (`file`, caminho ou relativo à pasta do código). `start_line`/`end_line` (a partir de 1) leem só um trecho. Leia por aqui, não pelo disco: o editor pode ter mudanças não salvas.", json!({ "form": s("nome do form (vazio: o aberto)"), "file": s("caminho de um fonte, opcional"), "start_line": { "type": "integer" }, "end_line": { "type": "integer" } }), &[]),
        tool("edit_code", "Edita um fonte pelo editor da IDE (abre se preciso): troca `old` (o texto exato, com linhas em volta para ser único) por `new`, como uma edição do usuário: o Ctrl+Z desfaz, o servidor de linguagem fica sabendo e a aba fica alterada. `all` troca todas as vezes; `save` salva em seguida. Use sempre esta ferramenta para mudar código do projeto, nunca gravando o arquivo direto: a IDE com ele aberto não veria a mudança e a perderia ao salvar. O código gerado (`generated/`) é somente leitura, e os forms (.json) mudam por set_property/set_layout.", json!({ "form": s("nome do form (vazio: o aberto)"), "file": s("caminho de um fonte, opcional"), "old": s("o trecho exato a trocar"), "new": s("o texto novo"), "all": { "type": "boolean" }, "save": { "type": "boolean" } }), &["old", "new"]),
        tool("editor_type","Digita no editor de código ativo (como o teclado: pares que fecham, sugestões do servidor...).", json!({ "text": s("o texto") }), &["text"]),
        tool("editor_key", "Aperta uma tecla no editor de código (\"Ctrl+Space\", \"F12\", \"Alt+Down\", \"Ctrl+/\", \"Enter\", \"Escape\"...); os atalhos dos menus da janela também valem.", json!({ "key": s("a tecla") }), &["key"]),
        tool("editor_hover", "O mouse parado numa linha e coluna (a partir de 1) do editor ativo: pede a dica (o tipo, a documentação, o erro).", json!({ "line": { "type": "integer" }, "column": { "type": "integer" } }), &["line", "column"]),
        tool("editor_state", "O que o editor de código mostra: arquivo, cursor, linha, seleção, a lista de sugestões, a dica, a ajuda dos parâmetros, os erros, o painel de baixo e o servidor de linguagem.", json!({}), &[]),
        tool("dialog_state", "O diálogo aberto por cima (Ir para a linha, Renomear, Abrir fonte, Novo projeto, a paleta de comandos...), só o que aparece: o título, os campos (`fields`), as escolhas (`combos`: o rótulo, o texto e o índice escolhidos e os itens), as listas (`lists`), as caixas (`checks`), os botões (`buttons`, com `enabled`) e os textos.", json!({}), &[]),
        tool(
            "dialog_select",
            "Escolhe um item de uma escolha (ComboBox) ou lista (ListBox) do diálogo aberto, como o usuário (o diálogo reage à troca): `item` é o texto do item ou o índice (a partir de 0); `list`, opcional, é o rótulo do campo (\"Modelo\", \"Linguagem\") ou o número `list` de `dialog_state` (sem ele: a primeira que tiver o item).",
            json!({ "item": { "description": "o texto do item ou o índice (número)" }, "list": { "description": "o rótulo do campo ou o número da lista, opcional" } }),
            &["item"],
        ),
        tool("dialog_type", "Digita no campo com foco do diálogo aberto.", json!({ "text": s("o texto") }), &["text"]),
        tool("dialog_check", "Marca ou desmarca uma caixa (CheckBox) do diálogo aberto, pelo texto dela.", json!({ "text": s("o texto da caixa"), "checked": { "type": "boolean" } }), &["text"]),
        tool("dialog_click", "Clica num botão do diálogo aberto, pelo texto dele (\"OK\", \"Instalar as marcadas\", \"Fechar\"...).", json!({ "text": s("o texto do botão") }), &["text"]),
        tool("menu", "Escolhe um item do menu da IDE (ou da janela de código) pelo caminho, como o usuário: \"Executar/Configurar linguagens…\", \"Arquivo/Salvar\" (o & e as reticências não importam).", json!({ "path": s("os textos separados por /") }), &["path"]),
        tool("dialog_key", "Aperta uma tecla no diálogo aberto (\"Enter\" é o OK, \"Escape\" cancela, \"Down\" desce na lista...).", json!({ "key": s("a tecla") }), &["key"]),
        tool("sources", "Os fontes do projeto na linguagem ativa (os do Ctrl+F12).", json!({}), &[]),
        tool(
            "usages",
            "Onde é usado, no projeto inteiro (forms e código): um componente (`form` e `component`; um Field vira o campo do dataset dele: os controles ligados, as colunas, as expressões e o nome entre aspas no código) ou um handler (`handler`).",
            json!({ "form": s("nome do form do componente"), "component": s("nome do componente"), "handler": s("nome do handler (no lugar de form e component)") }),
            &[],
        ),
        tool("breakpoint", "Liga (`on` true, o padrão) ou desliga o ponto de parada de uma linha (a partir de 1), como o clique na calha ou F5. `file`: caminho do código ou nome do form.", json!({ "file": s("arquivo ou nome do form"), "line": { "type": "integer" }, "on": { "type": "boolean" } }), &["file", "line"]),
        tool("run", "Executar (F9): com ponto de parada, executa no depurador; depurando, continua. A saída aparece em `output`; a depuração, em `debug_state`.", json!({}), &[]),
        tool("debug", "Depurar na IDE (Ctrl+F5): compila e executa no depurador mesmo sem ponto de parada.", json!({}), &[]),
        tool("debug_command", "Comando de depuração: continue (F9), step_over (F8), step_in (F7), step_out (Shift+F8), pause, stop (Ctrl+F2), ou run_to (F4) com `file` e `line`.", json!({ "command": s("continue | step_over | step_in | step_out | pause | stop | run_to"), "file": s("para run_to"), "line": { "type": "integer" } }), &["command"]),
        tool("debug_state", "A depuração agora: situação, se parou e onde (`exec`), a pilha, as variáveis do quadro e os pontos de parada; `evaluate` avalia uma expressão (o \"Avaliar\").", json!({ "evaluate": s("expressão, opcional") }), &[]),
        tool("status", "A linha de situação da IDE.", json!({}), &[]),
        tool("stop", "Para o programa em execução.", json!({}), &[]),
        tool("output", "As últimas linhas da saída (compilação e programa).", json!({ "lines": { "type": "integer", "description": "quantas (padrão 50)" } }), &[]),
        tool("ide_menus", "Todos os itens dos menus da IDE (e da janela de código): o caminho para `menu`, o atalho, se está habilitado e marcado. Os menus de contexto (clique direito) vêm uma vez só, com o lugar na frente (\"Object Inspector › (propriedade) › Ligar a campo…\") e `context` true. O que o usuário faz pelos menus, o agente faz com `menu` e os `dialog_*`.", json!({}), &[]),
        tool("new_project", "Cria um projeto (como Arquivo → Novo projeto) e abre: `dir` é a pasta onde criar (a do projeto é criada dentro), `language` uma das linguagens (Rust, Python, Pascal, C++, Go, C#, TypeScript, Java).", json!({ "dir": s("pasta onde criar"), "name": s("nome do projeto"), "language": s("linguagem (padrão Rust)") }), &["dir", "name"]),
        tool("close_project", "Fecha o projeto (volta à tela inicial).", json!({}), &[]),
        tool("new_form", "Cria um form no projeto aberto e abre no designer (`data_module` true: um DataModule).", json!({ "name": s("nome do form (frmClientes)"), "data_module": { "type": "boolean" } }), &["name"]),
        tool("remove_form", "Tira um form do projeto (o arquivo continua no disco).", json!({ "name": s("nome do form") }), &["name"]),
        tool("set_main_form", "Define o form principal do projeto (o que abre primeiro).", json!({ "name": s("nome do form") }), &["name"]),
        tool("project_settings", "A configuração do projeto aberto (o .aylisproj): nome, linguagem, forms, pasta de saída (`output`), publicação (`publish`), renderizador, temas...", json!({}), &[]),
        tool("set_project_setting", "Muda uma chave da configuração do projeto (como Projeto → Opções) e salva. `key` com pontos para dentro (`output.dir`, `publish.version`); `value` é o JSON.", json!({ "key": s("a chave (output.dir, publish.app_name...)"), "value": { "description": "o valor JSON" } }), &["key", "value"]),
        tool("targets", "Os alvos (desktop, linux, android, web, ios) e o escolhido para Executar e Compilar.", json!({}), &[]),
        tool("set_target", "Escolhe o alvo do Executar (desktop, linux, android, web, ios).", json!({ "target": s("o alvo") }), &["target"]),
        tool("build", "Compilar (Ctrl+F9): compila sem executar (`release` true: otimizado) e espera terminar: a resposta diz se compilou, com os erros e as últimas linhas (pode levar minutos; a primeira compilação de um projeto, mais). `wait` false só começa (acompanhe com `status` e `output`).", json!({ "release": { "type": "boolean" }, "wait": { "type": "boolean" } }), &[]),
        tool("run_without_debug", "Executar sem depurar (Ctrl+Shift+F9); `release` true compila otimizado.", json!({ "release": { "type": "boolean" } }), &[]),
        tool("publish", "Publica para um alvo (desktop, linux, android, web, ios), como Executar → Publicar; `out_dir` opcional.", json!({ "target": s("o alvo (padrão: o escolhido)"), "out_dir": s("pasta de saída, opcional") }), &[]),
        tool("undo", "Desfaz a última mudança no form aberto (Ctrl+Z).", json!({}), &[]),
        tool("redo", "Refaz (Ctrl+Y).", json!({}), &[]),
        tool("select", "Seleciona componentes no designer (o Object Inspector mostra o primeiro). Vazio: nenhum.", json!({ "components": { "type": "array", "items": { "type": "string" } } }), &[]),
        tool("debug_variables", "Os campos de uma variável na depuração: `reference` é o número que `debug_state` dá em cada variável (0: sem campos).", json!({ "reference": { "type": "integer" } }), &["reference"]),
        tool("debug_frame", "Escolhe um quadro da pilha (`index`, de `debug_state`): a linha no código e as variáveis passam a ser as dele.", json!({ "index": { "type": "integer" } }), &["index"]),
        tool("breakpoint_condition", "Ponto de parada com condição (`condition`, uma expressão da linguagem) e/ou contagem de passagens (`hits`); vazias: um ponto comum.", json!({ "file": s("arquivo ou nome do form"), "line": { "type": "integer" }, "condition": s("expressão, opcional"), "hits": s("passagens, opcional") }), &["file", "line"]),
        tool("app_command","Um comando para o app em execução (o protocolo remoto do Aylis): {\"cmd\":\"tree\"}, {\"cmd\":\"props\",\"target\":\"btnSalvar\"}, {\"cmd\":\"set\",\"target\":\"edtNome\",\"prop\":\"text\",\"value\":\"Ana\"} (ou várias: \"props\":{\"text\":\"Ana\"}), {\"cmd\":\"click\",\"target\":\"btnSalvar\"}...", json!({ "command": { "type": "object", "description": "o comando" } }), &["command"]),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_token_is_checked_exactly() {
        let token = "a".repeat(64);
        assert!(authorized(Some(&format!("Bearer {token}")), &token));
        assert!(!authorized(Some(&format!("Bearer {}", "b".repeat(64))), &token));
        assert!(!authorized(Some(&token), &token), "sem o Bearer");
        assert!(!authorized(None, &token));
        assert!(!authorized(Some("Bearer "), &token));
    }

    #[test]
    fn claude_desktop_config_keeps_the_other_servers() {
        let dir = std::env::temp_dir().join(format!("aylis_claude_cfg_{}", std::process::id()));
        let path = dir.join("claude_desktop_config.json");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(&path, r#"{"mcpServers":{"outro":{"command":"x"}},"theme":"dark"}"#).unwrap();
        add_to_claude_desktop(&path, Path::new("C:/aylis/aylis-tool.exe")).unwrap();
        let v: Value = serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(v["mcpServers"]["outro"]["command"], "x");
        assert_eq!(v["mcpServers"]["aylis"]["args"], json!(["mcp"]));
        assert_eq!(v["theme"], "dark");
        assert!(path.with_extension("json.bak").exists(), "a cópia de antes");
        let _ = std::fs::remove_dir_all(dir);
    }

    /// Máquina que nunca teve servidor MCP: sem o arquivo, ou com ele sem a chave `mcpServers`.
    #[test]
    fn claude_desktop_config_is_created_without_servers() {
        let dir = std::env::temp_dir().join(format!("aylis_claude_new_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let path = dir.join("Claude").join("claude_desktop_config.json");
        add_to_claude_desktop(&path, Path::new("C:/aylis/aylis-tool.exe")).unwrap();
        let v: Value = serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(v["mcpServers"]["aylis"]["args"], json!(["mcp"]));

        std::fs::write(&path, r#"{"preferences":{"x":1}}"#).unwrap();
        add_to_claude_desktop(&path, Path::new("C:/aylis/aylis-tool.exe")).unwrap();
        let v: Value = serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(v["mcpServers"]["aylis"]["command"], "C:/aylis/aylis-tool.exe");
        assert_eq!(v["preferences"]["x"], 1);
        let _ = std::fs::remove_dir_all(dir);
    }
}

#[cfg(test)]
mod bridge_tests {
    use super::*;

    /// O modo Claude do Claude Desktop aparece na janela da IDE como "Claude Desktop", não pelo
    /// nome interno; a resposta da chamada que abriu a IDE diz isso.
    #[test]
    fn the_client_names_and_the_opened_note() {
        assert_eq!(client_name("local-agent-mode-aylis"), "Claude Desktop");
        assert_eq!(client_name("claude-ai"), "Claude Desktop");
        assert_eq!(client_name("claude-code"), "Claude Code");
        let body = json!({ "jsonrpc": "2.0", "id": 1, "result": { "content": [{ "type": "text", "text": "projeto: nenhum" }] } }).to_string();
        let noted: Value = serde_json::from_str(&with_note(&body, "(Abri a IDE do Aylis para esta conversa.)")).unwrap();
        assert_eq!(noted["result"]["content"][0]["text"], "(Abri a IDE do Aylis para esta conversa.)");
        assert_eq!(noted["result"]["content"][1]["text"], "projeto: nenhum");
    }

    /// Antes de uma chamada de ferramenta, a ponte não fixa IDE nenhuma: o aperto de mão e a
    /// lista ela responde sozinha (com todas as ferramentas da IDE).
    #[test]
    fn the_handshake_does_not_take_an_ide() {
        let request = json!({ "jsonrpc": "2.0", "id": 1, "method": "initialize", "params": { "protocolVersion": "2025-06-18" } });
        let reply: Value = serde_json::from_str(&without_ide(&request, json!(1), "x")).unwrap();
        assert_eq!(reply["result"]["instructions"], INSTRUCTIONS);
    }

    /// Um processo que existe mas não deixa abrir (acesso negado) conta como vivo; o System
    /// (pid 4) é um deles para um processo comum.
    /// A consulta do job responde sem quebrar (num terminal comum, fora de um job que mata).
    #[test]
    fn the_job_query_answers() {
        let _ = crate::dap::in_kill_on_close_job();
    }

    #[cfg(windows)]
    #[test]
    fn a_process_that_denies_access_is_alive() {
        assert!(crate::dap::process_alive(4));
        assert!(!crate::dap::process_alive(u32::MAX - 3));
    }

    /// A marca da sessão traz o pid da ponte: a desta ponte está viva, a de um processo que não
    /// existe, não (a IDE dela volta a ficar livre).
    #[test]
    fn a_session_of_a_finished_bridge_is_dead() {
        let mine = new_session();
        assert!(mine.starts_with(&format!("{}-", std::process::id())));
        assert!(session_alive(&mine));
        assert!(!session_alive("4294967290-abc"));
        assert!(session_alive("sem-pid"), "marca sem pid conta como viva");
    }

    /// A lista da IDE ganha `session_title`, `ide_list` e `ide_select`; a ponte responde as duas
    /// últimas sem a IDE.
    #[test]
    fn the_bridge_adds_and_answers_its_own_tools() {
        let body = json!({ "jsonrpc": "2.0", "id": 1, "result": { "tools": [{ "name": "get_layout" }] } }).to_string();
        let v: Value = serde_json::from_str(&add_own_tools(&body)).unwrap();
        let names: Vec<&str> = v["result"]["tools"].as_array().unwrap().iter().filter_map(|t| t["name"].as_str()).collect();
        assert_eq!(names, vec!["get_layout", "session_title", "ide_list", "ide_select"]);
        let mut pinned = None;
        let call = json!({ "jsonrpc": "2.0", "id": 7, "method": "tools/call", "params": { "name": "ide_select", "arguments": { "project": "nenhum-projeto-com-este-nome" } } });
        let reply: Value = serde_json::from_str(&own_tool(&call, &mut pinned).unwrap()).unwrap();
        assert_eq!(reply["id"], 7);
        assert!(reply["result"]["content"][0]["text"].as_str().unwrap().contains("Nenhuma IDE"));
        assert!(pinned.is_none());
        let other = json!({ "jsonrpc": "2.0", "id": 8, "method": "tools/call", "params": { "name": "get_layout" } });
        assert!(own_tool(&other, &mut pinned).is_none());
    }

    /// Sem a IDE aberta, a lista já traz as ferramentas dela (o Claude Code não pede a lista de
    /// novo quando a IDE abre).
    #[test]
    fn the_list_without_the_ide_has_the_ide_tools() {
        let request = json!({ "jsonrpc": "2.0", "id": 2, "method": "tools/list" });
        let v: Value = serde_json::from_str(&without_ide(&request, json!(2), "fechada")).unwrap();
        let names: Vec<&str> = v["result"]["tools"].as_array().unwrap().iter().filter_map(|t| t["name"].as_str()).collect();
        for name in ["open_project", "set_property", "edit_code", "build", "app_command", "session_title", "ide_list", "dialog_select"] {
            assert!(names.contains(&name), "falta {name}");
        }
    }
}
