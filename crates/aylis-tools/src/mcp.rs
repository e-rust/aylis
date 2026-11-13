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

Projeto em toda chamada: passe `project` (o caminho do .aylisproj desta conversa) em todas as \
ferramentas, session_title inclusive. É ele que leva a chamada à IDE certa: o cliente pode dividir \
esta conexão entre conversas, cada uma no seu projeto e na sua IDE. Com mais de uma IDE aberta, a \
chamada sem `project` é recusada; ide_list mostra as abertas.

Regra principal: com o projeto aberto na IDE, NÃO grave nem edite os arquivos dele com as suas \
ferramentas de arquivo (forms/*.json, temas, .aylisproj, código). A IDE guarda o que está aberto \
na memória, não vê a mudança no disco e a perde ao salvar. Mude tudo por aqui:
- código: read_code (o texto do editor, com o que não foi salvo) e edit_code (troca um trecho \
exato; o usuário desfaz com Ctrl+Z); open_code abre um fonte;
- forms: open_form, list_components, get_component, set_property, add_component, \
delete_component, get_layout, set_layout e patch_layout (uma parte do form); class_info diz as \
propriedades, os valores e os eventos de uma classe (consulte antes de montar um layout), e o \
set_layout devolve todos os erros de uma vez (dry_run só confere);
- salvar: save (com all, tudo o que mudou);
- generated/ é somente leitura: generate_code refaz.

Compilar e executar: build compila e espera até 45 s; a resposta diz se compilou, com os erros \
e as últimas linhas, ou \"ainda compilando\" (a primeira compilação leva minutos): chame build_wait \
até vir o resultado. Corrija com edit_code e chame build de novo. run_without_debug executa, run \
executa no depurador; os dois esperam a compilação do mesmo jeito e dizem se o app abriu (ou caiu \
logo). output mostra as mensagens (a compilação e o programa, com o nível de cada linha) e status a \
situação; uma chamada que passar de ~45 s responde in_progress com o número da tarefa, e task_status \
traz o fim.Com o app aberto, app_command lê e mexe na tela dele ({\"cmd\": \"tree\"}, \
{\"cmd\": \"click\", \"target\": \"btnSalvar\"}).

Como o código do Aylis é escrito: componentes pelo acesso tipado (self.btnSalvar, \
frmCliente.edtNome), nunca procurando pelo nome em texto (os tipos gerados, como Button e Label, \
têm o nome dos controles do aylis::prelude: no código do form use os do acesso tipado e, se \
precisar dos do prelude, pelo caminho, aylis::prelude::Label); handlers devolvem Result e a recusa é \
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

/// A ponte sai quando o processo que a abriu (o cliente MCP) some: o stdin nem sempre fecha
/// junto, e pontes órfãs ficavam vivas (o relatório do Tabernáculo, M04). Confere a cada 5 s.
/// No navegador (a IDE lite) não há processo pai nem ponte.
#[cfg(target_arch = "wasm32")]
fn exit_with_parent() {}

#[cfg(not(target_arch = "wasm32"))]
fn exit_with_parent() {
    use sysinfo::{Pid, ProcessRefreshKind, ProcessesToUpdate, RefreshKind, System};
    let me = Pid::from_u32(std::process::id());
    let mut system = System::new_with_specifics(RefreshKind::nothing().with_processes(ProcessRefreshKind::nothing()));
    let Some(parent) = system.process(me).and_then(|p| p.parent()) else { return };
    std::thread::spawn(move || {
        loop {
            std::thread::sleep(std::time::Duration::from_secs(5));
            system.refresh_processes_specifics(ProcessesToUpdate::Some(&[parent]), true, ProcessRefreshKind::nothing());
            if system.process(parent).is_none() {
                std::process::exit(0);
            }
        }
    });
}

/// `aylis-tool mcp`: a ponte entre o cliente MCP (stdio, uma mensagem JSON por linha) e o
/// servidor HTTP da IDE, com o token. Com a IDE fechada (ou o MCP desligado), cada pedido
/// recebe um erro que diz o que fazer.
pub fn bridge() -> Result<(), String> {
    exit_with_parent();
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
    // O título de cada IDE (pid) e as IDEs que já sabem desta ponte. O cliente pode dividir uma
    // ponte entre conversas (o Claude Desktop sobe um processo só para todas; visto em 10/11), e
    // cada conversa trabalha numa IDE: nada aqui é "a IDE da sessão", e sim o de cada IDE.
    let mut titles: std::collections::HashMap<u32, String> = std::collections::HashMap::new();
    let mut told: Vec<u32> = Vec::new();
    for line in stdin.lock().lines() {
        let mut line = line.map_err(|e| e.to_string())?;
        if line.trim().is_empty() {
            continue;
        }
        let mut request = serde_json::from_str::<Value>(&line).unwrap_or(Value::Null);
        if request["method"] == "initialize" {
            client = client_name(request["params"]["clientInfo"]["name"].as_str().unwrap_or_default());
        }
        // `ide_list` e `ide_select` a ponte responde (é ela quem escolhe a IDE).
        if let Some(reply) = own_tool(&request, &mut pinned) {
            writeln!(stdout, "{reply}").and_then(|_| stdout.flush()).map_err(|e| e.to_string())?;
            continue;
        }
        // O `project` da chamada escolhe a IDE (e sai dos argumentos: a IDE não o conhece). Sem
        // ele e com mais de uma IDE aberta, a chamada é recusada: numa ponte dividida, a IDE
        // fixada pode ser a de outra conversa.
        if request["method"] == "tools/call" {
            let name = request["params"]["name"].as_str().unwrap_or_default().to_string();
            let project = request["params"]["arguments"].as_object_mut().and_then(|a| a.remove(PROJECT_ARG)).and_then(|v| v.as_str().map(|s| s.trim().to_string())).filter(|s| !s.is_empty());
            if project.is_some() {
                line = request.to_string();
            }
            // Um argumento que a ferramenta não tem é recusado com a lista dos que ela tem (antes
            // era ignorado calado, e o agente achava que tinha valido; M06 do Tabernáculo).
            if let Some(text) = unknown_arguments(&name, &request["params"]["arguments"]) {
                let reply = json!({ "jsonrpc": "2.0", "id": request["id"].clone(), "result": { "content": [{ "type": "text", "text": text }], "isError": true } });
                writeln!(stdout, "{reply}").and_then(|_| stdout.flush()).map_err(|e| e.to_string())?;
                continue;
            }
            let ides: Vec<crate::instances::Instance> = crate::instances::list().into_iter().filter(|i| i.mcp_port.is_some()).collect();
            match route_call(&name, project.as_deref(), &ides) {
                Route::Ide(pid) => pinned = Some(pid),
                Route::Default => {}
                Route::Refuse(text) => {
                    let reply = json!({ "jsonrpc": "2.0", "id": request["id"].clone(), "result": { "content": [{ "type": "text", "text": text }], "isError": true } });
                    writeln!(stdout, "{reply}").and_then(|_| stdout.flush()).map_err(|e| e.to_string())?;
                    continue;
                }
            }
        }
        // `session_title`: guarda o título, abre a IDE se preciso e mostra nela.
        if request["method"] == "tools/call" && request["params"]["name"] == "session_title" {
            let title = request["params"]["arguments"]["title"].as_str().unwrap_or_default().trim().to_string();
            let (text, opened) = match target_port(&mut pinned, &session, true) {
                Ok(_) => {
                    if let Some(pid) = pinned {
                        titles.insert(pid, title.clone());
                    }
                    announce(&mut told, pinned, &session, &client, &titles);
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
        if request["method"] == "tools/call" && (request["params"]["name"] == "open_project" || request["params"]["name"] == "new_project") {
            // O projeto novo ainda não existe: nenhuma IDE o tem, e vale a regra da IDE ocupada.
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
        if target.is_ok() && pinned.is_some() && (!pinned.is_some_and(|p| told.contains(&p)) || !registered(pinned, &session)) {
            announce(&mut told, pinned, &session, &client, &titles);
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
    // A ponte acabou (o cliente a fechou): as IDEs tiram o nome dela da janela.
    for pid in told {
        let _ = tell_pid(pid, &session, &client, titles.get(&pid).map_or("", String::as_str), false);
    }
    Ok(())
}

/// O argumento que toda ferramenta da IDE aceita: o projeto da conversa, que escolhe a IDE.
const PROJECT_ARG: &str = "project";

/// Para onde vai uma chamada de ferramenta.
#[derive(Debug, PartialEq)]
enum Route {
    /// A IDE (pid) do projeto informado.
    Ide(u32),
    /// A de sempre: a fixada, ou a que a ponte abre ou escolhe (`target_port`, `route_project`).
    Default,
    /// Recusada, com o motivo para o agente.
    Refuse(String),
}

/// A IDE de uma chamada (`name`) pelo `project` dela. Abrir e criar projeto escolhem a IDE pelo
/// caminho (`route_project`); sem `project`, só uma IDE aberta serve: com mais de uma, a fixada
/// pode ser a de outra conversa na mesma ponte, e a chamada é recusada dizendo o que passar.
fn route_call(name: &str, project: Option<&str>, ides: &[crate::instances::Instance]) -> Route {
    if matches!(name, "open_project" | "new_project") {
        return Route::Default;
    }
    let describe = |i: &crate::instances::Instance| i.project.as_ref().map_or_else(|| "(sem projeto)".to_string(), |p| p.display().to_string());
    match project {
        Some(wanted) => {
            let found: Vec<&crate::instances::Instance> = ides.iter().filter(|i| project_matches(i, wanted)).collect();
            match found.as_slice() {
                [one] => Route::Ide(one.pid),
                [] => Route::Refuse(format!("o projeto \"{wanted}\" não está aberto em nenhuma IDE do Aylis: abra com open_project (ide_list mostra as abertas)")),
                _ => Route::Refuse(format!("mais de uma IDE aberta tem \"{wanted}\" no projeto: passe o caminho inteiro do .aylisproj")),
            }
        }
        None if ides.len() > 1 => Route::Refuse(format!(
            "há {} IDEs do Aylis abertas: passe `project` (o caminho do .aylisproj desta conversa) em toda chamada. Abertas:\n{}",
            ides.len(),
            ides.iter().map(|i| format!("  {}", describe(i))).collect::<Vec<_>>().join("\n")
        )),
        None => Route::Default,
    }
}

/// A IDE tem o projeto `wanted`: um caminho (o mesmo arquivo) ou parte do nome.
fn project_matches(ide: &crate::instances::Instance, wanted: &str) -> bool {
    let Some(project) = &ide.project else { return false };
    if wanted.ends_with(".aylisproj") || wanted.contains(['/', '\\']) {
        crate::instances::same_path(project, Path::new(wanted))
    } else {
        project.display().to_string().to_lowercase().contains(&wanted.to_lowercase())
    }
}

/// Diz à IDE `pinned` que esta ponte está nela, com o título que a conversa deu a ela. A ponte
/// não sai das outras (pode estar dividida entre conversas, cada uma na sua IDE): sai de todas
/// quando acaba.
fn announce(told: &mut Vec<u32>, pinned: Option<u32>, session: &str, client: &str, titles: &std::collections::HashMap<u32, String>) {
    if let Some(pid) = pinned {
        let _ = tell_pid(pid, session, client, titles.get(&pid).map_or("", String::as_str), true);
        if !told.contains(&pid) {
            told.push(pid);
        }
    }
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
    // Só uma IDE sem projeto e sem outra sessão recebe o projeto: numa ponte que o cliente divide
    // entre conversas, a IDE fixada (ou a "desta sessão") pode ser a da outra conversa, e trocar o
    // projeto dela tiraria o trabalho de lá (visto em 10/11). Um projeto, uma IDE: a fixada, se
    // serve; senão, uma livre na tela inicial; senão, uma nova.
    let usable = |i: &crate::instances::Instance| i.project.is_none() && i.sessions.iter().all(|s| s == session);
    if pinned.and_then(|pid| ides.iter().find(|i| i.pid == pid)).is_some_and(usable) {
        return Ok(());
    }
    *pinned = Some(match ides.iter().find(|i| usable(i)) {
        Some(i) => i.pid,
        None => open_ide(session)?.pid,
    });
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

/// Os argumentos de `arguments` que a ferramenta `name` não declara: a mensagem de recusa, com
/// os que valem. Ferramenta desconhecida (de uma IDE mais nova): sem conferência.
fn unknown_arguments(name: &str, arguments: &Value) -> Option<String> {
    let given = arguments.as_object()?;
    let own = own_tools();
    let def = ide_tools().into_iter().chain(own.as_array().into_iter().flatten().cloned()).find(|t| t["name"] == name)?;
    let known: Vec<String> = def["inputSchema"]["properties"].as_object().map(|p| p.keys().cloned().collect()).unwrap_or_default();
    let unknown: Vec<&String> = given.keys().filter(|k| *k != PROJECT_ARG && !known.contains(k)).collect();
    if unknown.is_empty() {
        return None;
    }
    let names = unknown.iter().map(|k| format!("'{k}'")).collect::<Vec<_>>().join(", ");
    let valid = if known.is_empty() { "nenhum".to_string() } else { known.join(", ") };
    Some(format!("{name}: argumento desconhecido {names} (os desta ferramenta: {valid})"))
}

/// As ferramentas da própria ponte.
fn own_tools() -> Value {
    json!([
        { "name": "session_title", "description": "Dá um título a esta sessão (um resumo curto do que ela vai fazer), que aparece na barra da janela da IDE do Aylis ligada a ela. Chame no começo da conversa; abre a IDE se ela não estiver aberta.", "inputSchema": { "type": "object", "properties": { "title": { "type": "string", "description": "o título, curto (ex.: Tela de cadastro de clientes)" }, "project": project_property() }, "required": ["title"] } },
        { "name": "ide_list", "description": "Lista as IDEs do Aylis abertas (cada uma com o projeto dela) e qual está fixada nesta conversa.", "inputSchema": { "type": "object", "properties": {} } },
        { "name": "ide_select", "description": "Fixa nesta conversa a IDE do projeto informado (o caminho do .aylisproj ou parte do nome, como ide_list mostra), ou a do processo `pid` (duas IDEs com o mesmo projeto). As chamadas seguintes vão para ela.", "inputSchema": { "type": "object", "properties": { "project": { "type": "string", "description": "caminho ou parte do nome do projeto; vazio: a IDE sem projeto" }, "pid": { "type": "integer", "description": "o processo da IDE, como ide_list mostra (opcional)" } } } }
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
    // Com o processo de cada uma: duas IDEs com o mesmo projeto se distinguem (M03).
    let with_pid = |i: &crate::instances::Instance| format!("{}  (pid {})", describe(i), i.pid);
    let text = if name == "ide_list" {
        // As abertas com o MCP desligado também aparecem (sem elas, uma IDE que perdeu o
        // vínculo parecia ter sumido; relatório do RTSPlayer, 10/11).
        let off: Vec<String> = crate::instances::list().into_iter().filter(|i| i.mcp_port.is_none()).map(|i| format!("  {} (MCP desligado: ligue em Ferramentas → Servidor MCP)", describe(&i))).collect();
        if ides.is_empty() && off.is_empty() {
            "Nenhuma IDE do Aylis aberta com o MCP ligado: a próxima chamada de ferramenta (project_info, por exemplo) abre uma para esta conversa.".to_string()
        } else {
            ides.iter().map(|i| format!("{}{}", if *pinned == Some(i.pid) { "* " } else { "  " }, with_pid(i))).chain(off).collect::<Vec<_>>().join("
")
        }
    } else {
        let wanted = request["params"]["arguments"]["project"].as_str().unwrap_or_default().to_lowercase();
        let pid = request["params"]["arguments"]["pid"].as_u64();
        let found = match pid {
            Some(pid) => ides.iter().find(|i| u64::from(i.pid) == pid),
            None => ides.iter().find(|i| if wanted.is_empty() { i.project.is_none() } else { describe(i).to_lowercase().contains(&wanted) }),
        };
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
fn tool(name: &str, description: &str, mut properties: Value, required: &[&str]) -> Value {
    // Toda ferramenta que trabalha num projeto aceita o `project` da conversa (abrir e criar
    // projeto dizem o caminho nos argumentos delas).
    if !matches!(name, "open_project" | "new_project") {
        if let Some(map) = properties.as_object_mut() {
            map.insert(PROJECT_ARG.into(), project_property());
        }
    }
    json!({ "name": name, "description": description, "inputSchema": { "type": "object", "properties": properties, "required": required } })
}

/// O esquema do argumento `project`.
fn project_property() -> Value {
    json!({ "type": "string", "description": "o projeto desta conversa: o caminho do .aylisproj (ou parte do nome, como ide_list mostra). Passe sempre: escolhe a IDE do projeto, e com mais de uma IDE aberta a chamada sem ele é recusada" })
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
        tool("set_properties", "Muda várias propriedades em vários componentes do form aberto numa chamada (cada propriedade, um passo do desfazer). `properties` é {\"propriedade\": valor}, com os valores como no set_property; sem `components`, o form.", json!({ "components": { "type": "array", "items": { "type": "string" }, "description": "nomes dos componentes" }, "properties": { "type": "object", "description": "propriedade → valor" } }), &["properties"]),
        tool("move_component", "Passa um componente para dentro de outro (o pai novo) no form aberto, opcionalmente na posição `index` entre os filhos (desfazível).", json!({ "component": s("nome do componente"), "parent": s("nome do pai novo (vazio: o form)"), "index": { "type": "integer", "description": "posição entre os filhos, opcional" } }), &["component"]),
        tool("set_property", "Muda uma propriedade de um componente do form aberto (desfazível). `value` é o valor JSON como no layout: número, texto, booleano, cor \"#rrggbb\", nome de outro componente...", json!({ "component": s("nome do componente (vazio: o form)"), "property": s("nome da propriedade"), "value": { "description": "o valor" } }), &["property", "value"]),
        tool("add_component", "Cria um componente da paleta no form aberto (`class`, pode ser um subtipo como Button.icon), dentro de `parent` (opcional) e com `name` (opcional). Devolve o nome.", json!({ "class": s("classe"), "parent": s("contêiner, opcional"), "name": s("nome, opcional") }), &["class"]),
        tool("delete_component", "Exclui um componente do form aberto.", json!({ "component": s("nome") }), &["component"]),
        tool("get_layout", "O layout JSON inteiro do form aberto.", json!({}), &[]),
        tool(
            "set_layout",
            "Troca o layout inteiro do form aberto pelo JSON dado (desfazível). Confere tudo antes e devolve todos os erros de uma vez (nada é trocado se houver erro); `dry_run` só confere. Um layout grande pode vir de um arquivo (`layout_file`) em vez do texto. Para mudar uma parte, `patch_layout`.",
            json!({ "layout": s("o documento JSON ({\"aylis\":1,\"root\":{...}})"), "layout_file": s("caminho de um arquivo com o documento, no lugar de `layout`"), "dry_run": { "type": "boolean", "description": "só conferir, sem trocar" } }),
            &[],
        ),
        tool(
            "patch_layout",
            "Troca só um componente do form aberto (e os filhos dele) pelo JSON dado, sem mandar o form inteiro: `component` é o nome dele, `layout` o objeto novo (class, name, propriedades, children). Confere tudo antes, como o set_layout; `dry_run` só confere.",
            json!({ "component": s("nome do componente a trocar"), "layout": { "type": "object", "description": "o componente novo" }, "dry_run": { "type": "boolean" } }),
            &["component", "layout"],
        ),
        tool("list_classes", "As classes da paleta, por categoria, com os subtipos.", json!({}), &[]),
        tool(
            "component_docs",
            "A documentação de um componente (a mesma do site), sob demanda: com `property`, só como aquela propriedade funciona (o que não é óbvio, como o `radius` negativo); sem ela, a página inteira. Consulte quando o tipo do class_info não bastar.",
            json!({ "class": s("a classe (Panel, Button, Chart...)"), "property": s("a propriedade, opcional (radius, anchors...)") }),
            &["class"],
        ),
        tool("search_icons", "Procura ícones pelos nomes nos pacotes (Material, Fluent...): devolve `pacote:nome`, como vai na propriedade `icon`. Várias palavras: todas precisam aparecer.", json!({ "query": s("palavras (ex.: database, arrow left)"), "limit": { "type": "integer", "description": "quantos (padrão 40)" } }), &["query"]),
        tool("designer_snapshot", "A imagem do form aberto no designer (como ficou, sem executar).", json!({}), &[]),
        tool(
            "compare_image",
            "Compara a tela com uma imagem de referência (`reference`, um PNG: a tela antiga, um protótipo): o designer, ou o app em execução com `source: \"app\"`. Devolve quanto difere (%) e a imagem das diferenças em vermelho.",
            json!({ "reference": s("caminho do PNG de referência"), "source": s("designer (padrão) ou app"), "form": s("com source app: outro form aberto, opcional") }),
            &["reference"],
        ),
        tool(
            "class_info",
            "Tudo o que uma classe aceita: cada propriedade com o tipo, a categoria e os valores (de um enum: `trimming` → none, character, word), os eventos (`on_dbl_click`...) e os métodos. Consulte antes de montar um layout.",
            json!({ "class": s("nome da classe (Button, Label, DataRepeater...)") }),
            &["class"],
        ),
        tool("save", "Salva o form aberto (ou, com `all`, todos os forms e os fontes alterados nos editores).", json!({ "all": { "type": "boolean" } }), &[]),
        tool("generate_code", "Gera (ou atualiza) o código do projeto: handlers dos forms e Cargo.toml. Salva antes os forms abertos com alterações (como o Gerar código da IDE); um form novo ainda sem arquivo precisa do save antes.", json!({}), &[]),
        tool("open_project", "Abre um projeto (.aylisproj), como Arquivo → Abrir projeto.", json!({ "path": s("caminho do .aylisproj") }), &["path"]),
        tool("open_code", "Abre na janela de código (uma aba por arquivo) o código de um form (F12) ou um fonte qualquer (`file`), na linha dada.", json!({ "form": s("nome do form (vazio: o aberto)"), "file": s("caminho de um fonte, opcional"), "line": { "type": "integer" } }), &[]),
        tool("read_code", "O texto de um fonte como está no editor da IDE (com o que ainda não foi salvo), ou do disco se não estiver aberto: o código de um form (`form`) ou um fonte (`file`, caminho ou relativo à pasta do código). `start_line`/`end_line` (a partir de 1) leem só um trecho. Leia por aqui, não pelo disco: o editor pode ter mudanças não salvas.", json!({ "form": s("nome do form (vazio: o aberto)"), "file": s("caminho de um fonte, opcional"), "start_line": { "type": "integer" }, "end_line": { "type": "integer" } }), &[]),
        tool("edit_code", "Edita um fonte pelo editor da IDE (abre se preciso): troca `old` (o texto exato, com linhas em volta para ser único) por `new`, como uma edição do usuário: o Ctrl+Z desfaz, o servidor de linguagem fica sabendo e a aba fica alterada. `all` troca todas as vezes; `save` salva em seguida. Com `old` vazio e um `file` que ainda não existe (relativo à pasta do código, como `src/nucleo.rs`), cria o fonte com `new` (em Rust, declare depois o `mod` dele). Use sempre esta ferramenta para mudar código do projeto, nunca gravando o arquivo direto: a IDE com ele aberto não veria a mudança e a perderia ao salvar. O código gerado (`generated/`) é somente leitura, e os forms (.json) mudam por set_property/set_layout.", json!({ "form": s("nome do form (vazio: o aberto)"), "file": s("caminho de um fonte, opcional"), "old": s("o trecho exato a trocar"), "new": s("o texto novo"), "all": { "type": "boolean" }, "save": { "type": "boolean" } }), &["old", "new"]),
        tool("editor_type","Digita no editor de código ativo (como o teclado: pares que fecham, sugestões do servidor...).", json!({ "text": s("o texto") }), &["text"]),
        tool("editor_key", "Aperta uma tecla no editor de código (\"Ctrl+Space\", \"F12\", \"Alt+Down\", \"Ctrl+/\", \"Enter\", \"Escape\"...); os atalhos dos menus da janela também valem.", json!({ "key": s("a tecla") }), &["key"]),
        tool("editor_hover", "O mouse parado numa linha e coluna (a partir de 1) do editor ativo: pede a dica (o tipo, a documentação, o erro).", json!({ "line": { "type": "integer" }, "column": { "type": "integer" } }), &["line", "column"]),
        tool("editor_state", "O que o editor de código mostra: arquivo, cursor, linha, seleção, a lista de sugestões, a dica, a ajuda dos parâmetros, os erros, o painel de baixo e o servidor de linguagem.", json!({}), &[]),
        tool("dialog_state", "O diálogo aberto por cima (Ir para a linha, Renomear, Abrir fonte, Novo projeto, a paleta de comandos...), só o que aparece: o título, os campos (`fields`), as escolhas (`combos`: o rótulo, o texto e o índice escolhidos e os itens), as listas (`lists`), as caixas (`checks`), os botões (`buttons`: `text`, `label` único quando há textos iguais, como \"Corrigir… — Android SDK (Android)\", `id`, `hint` e `enabled`) e os textos.", json!({}), &[]),
        tool(
            "dialog_select",
            "Escolhe um item de uma escolha (ComboBox) ou lista (ListBox) do diálogo aberto, como o usuário (o diálogo reage à troca): `item` é o texto do item ou o índice (a partir de 0); `list`, opcional, é o rótulo do campo (\"Modelo\", \"Linguagem\") ou o número `list` de `dialog_state` (sem ele: a primeira que tiver o item).",
            json!({ "item": { "description": "o texto do item ou o índice (número)" }, "list": { "description": "o rótulo do campo ou o número da lista, opcional" } }),
            &["item"],
        ),
        tool("dialog_type", "Digita no campo com foco do diálogo aberto.", json!({ "text": s("o texto") }), &["text"]),
        tool("dialog_check", "Marca ou desmarca uma caixa (CheckBox) do diálogo aberto, pelo texto dela.", json!({ "text": s("o texto da caixa"), "checked": { "type": "boolean" } }), &["text"]),
        tool("dialog_click", "Clica num botão do diálogo aberto, pelo texto dele (\"OK\", \"Instalar as marcadas\", \"Fechar\"...), pelo `label` ou pelo `id` que `dialog_state` dá. Com dois botões de mesmo texto, recusa e lista os `label`/`id` de cada um.", json!({ "text": s("o texto ou o label do botão"), "id": { "type": "integer", "description": "o id do botão (de dialog_state), no lugar de text" } }), &[]),
        tool("menu", "Escolhe um item de menu da IDE (ou da janela de código), como o usuário: pelo caminho (\"Executar/Configurar linguagens…\", \"Arquivo/Salvar\"; o & e as reticências não importam), por um caminho de menu de contexto como `ide_menus` lista (\"Projeto › Novo form…\"; no Object Inspector, a propriedade no lugar de \"(propriedade)\": \"Object Inspector › caption › Ligar a campo…\") ou pelo `id` de `ide_menus`. Um caminho que vale para mais de um item é recusado com os candidatos (caminho e id).", json!({ "path": s("os textos separados por / (ou por ›, com o lugar na frente)"), "id": { "type": "integer", "description": "o id do item (de ide_menus), no lugar de path" } }), &[]),
        tool("dialog_key", "Aperta uma tecla no diálogo aberto (\"Enter\" é o OK, \"Escape\" cancela, \"Down\" desce na lista...).", json!({ "key": s("a tecla") }), &["key"]),
        tool("sources", "Os fontes do projeto na linguagem ativa (os do Ctrl+F12).", json!({}), &[]),
        tool(
            "usages",
            "Onde é usado, no projeto inteiro (forms e código): um componente (`form` e `component`; um Field vira o campo do dataset dele: os controles ligados, as colunas, as expressões e o nome entre aspas no código) ou um handler (`handler`).",
            json!({ "form": s("nome do form do componente"), "component": s("nome do componente"), "handler": s("nome do handler (no lugar de form e component)") }),
            &[],
        ),
        tool("breakpoint", "Liga (`on` true, o padrão) ou desliga o ponto de parada de uma linha (a partir de 1), como o clique na calha ou F5. `file`: caminho do código ou nome do form.", json!({ "file": s("arquivo ou nome do form"), "line": { "type": "integer" }, "on": { "type": "boolean" } }), &["file", "line"]),
        tool("run", "Executar (F9): com ponto de parada, executa no depurador; depurando, continua. Espera a compilação (até `max_wait` s, 45 por padrão) e responde com os erros dela, ou que o app está rodando (ou que abriu e caiu); compilando ainda, `build_wait` espera o resto. A saída aparece em `output`; a depuração, em `debug_state`.", json!({ "max_wait": { "type": "integer", "description": "segundos de espera nesta chamada (padrão 45)" } }), &[]),
        tool("debug", "Depurar na IDE (Ctrl+F5): compila e executa no depurador mesmo sem ponto de parada. Espera a compilação como o `run`.", json!({ "max_wait": { "type": "integer" } }), &[]),
        tool(
            "build_wait",
            "Espera a compilação em andamento (do `build` ou do `run` que respondeu \"ainda compilando\") até `max_wait` s (padrão 45) e devolve o resultado: se compilou, os erros e as últimas linhas, ou que o app está rodando. Sem nada em andamento, o resultado da última.",
            json!({ "max_wait": { "type": "integer", "description": "segundos de espera nesta chamada (padrão 45)" } }),
            &[],
        ),
        tool("debug_command", "Comando de depuração: continue (F9), step_over (F8), step_in (F7), step_out (Shift+F8), pause, stop (Ctrl+F2), ou run_to (F4) com `file` e `line`.", json!({ "command": s("continue | step_over | step_in | step_out | pause | stop | run_to"), "file": s("para run_to"), "line": { "type": "integer" } }), &["command"]),
        tool("debug_state", "A depuração agora: situação, se parou e onde (`exec`), a pilha, as variáveis do quadro e os pontos de parada; `evaluate` avalia uma expressão (o \"Avaliar\").", json!({ "evaluate": s("expressão, opcional") }), &[]),
        tool("status", "A linha de situação da IDE e as chamadas ainda em andamento (`tasks`, de ferramentas que passaram do prazo).", json!({}), &[]),
        tool(
            "task_status",
            "O fim de uma chamada que respondeu `in_progress` (a IDE levou mais que o prazo de uma chamada, ~45 s, e seguiu trabalhando): espera até `max_wait` s (padrão 30) e devolve o resultado dela, ou que ainda está em andamento. Sem `task`, lista as chamadas em andamento e as terminadas que ninguém leu.",
            json!({ "task": { "type": "integer", "description": "o número da tarefa (o `task` da resposta)" }, "max_wait": { "type": "integer", "description": "segundos de espera nesta chamada (padrão 30)" } }),
            &[],
        ),
        tool("stop", "Para o programa em execução.", json!({}), &[]),
        tool(
            "output",
            "As últimas linhas das mensagens, cada uma com a origem (`source`: build, a compilação e a geração; program, o app e o rastro de eventos) e o nível (`level`: error, warning, success, step, info, note). Por padrão, as da última execução (como o painel); `previous` traz também as de antes, das execuções e dos projetos anteriores (o histórico guarda as últimas 2000 linhas, e trocar de projeto não o limpa).",
            json!({
                "lines": { "type": "integer", "description": "quantas (padrão 50)" },
                "source": { "type": "string", "enum": ["all", "build", "program"], "description": "de onde (padrão all)" },
                "level": { "type": "string", "enum": ["all", "warning", "error"], "description": "warning: avisos e erros; error: só erros (padrão all)" },
                "previous": { "type": "boolean", "description": "também as execuções e os projetos anteriores (cada linha com `run` e `project`)" }
            }),
            &[],
        ),
        tool("ide_menus", "Todos os itens dos menus da IDE (e da janela de código): o caminho e o `id` para `menu`, o atalho, se está habilitado e marcado. Os menus de contexto (clique direito) vêm uma vez só, com o lugar na frente (\"Object Inspector › (propriedade) › Ligar a campo…\") e `context` true. O que o usuário faz pelos menus, o agente faz com `menu` e os `dialog_*`.", json!({}), &[]),
        tool("new_project", "Cria um projeto (como Arquivo → Novo projeto) e abre: `dir` é a pasta onde criar (a do projeto é criada dentro; com `here`, o projeto fica direto em `dir`, mesmo que ela já tenha outras coisas, como o app dentro de um repositório), `language` uma das linguagens (Rust, Python, Pascal, C++, Go, C#, TypeScript, Java). Crie onde o projeto vai ficar: a confiança é pela pasta, e um projeto movido abre em modo restrito.", json!({ "dir": s("pasta onde criar"), "name": s("nome do projeto"), "language": s("linguagem (padrão Rust)"), "here": { "type": "boolean", "description": "o projeto direto em `dir` (sem a pasta com o nome)" } }), &["dir", "name"]),
        tool("close_project", "Fecha o projeto (volta à tela inicial).", json!({}), &[]),
        tool("resolve_conflict", "Resolve um arquivo aberto na IDE que mudou no disco e também tem alterações não salvas (as outras ferramentas recusam com o diff até lá): `keep` \"disk\" recarrega do disco, \"ide\" fica com a da IDE.", json!({ "file": s("o arquivo (como veio no aviso, ou relativo ao projeto)"), "keep": { "type": "string", "enum": ["disk", "ide"] } }), &["file", "keep"]),
        tool("new_form", "Cria um form no projeto aberto e abre no designer (`data_module` true: um DataModule), como Projeto → Novo form. `folder` é a subpasta de forms/ onde ele nasce (cadastros, painel/inicio; sem caminho absoluto nem ..); `file` escolhe o arquivo inteiro (forms/painel/inicio.json); sem eles, forms/ e o nome do arquivo sai do nome do form.", json!({ "name": s("nome do form (frmClientes)"), "data_module": { "type": "boolean" }, "folder": s("subpasta de forms/, opcional (cadastros)"), "file": s("arquivo do form, opcional (forms/painel/inicio.json)") }), &["name"]),
        tool(
            "new_frame",
            "Cria um frame: um form do projeto (como o new_form, com `folder`) com o fundo transparente (mostra o fundo de quem o põe) e, opcionalmente, do tamanho dado. Com `host`, também põe um Frame mostrando ele naquele form (dentro de `parent`, com `frame_name` e `align` opcionais), que fica aberto; sem `host`, o form do frame fica aberto para pôr os componentes.",
            json!({ "name": s("nome do form do frame (fraCartao)"), "folder": s("subpasta de forms/, opcional"), "width": { "type": "number" }, "height": { "type": "number" }, "host": s("form onde pôr o Frame, opcional"), "parent": s("contêiner no host, opcional"), "frame_name": s("nome do Frame no host, opcional"), "align": s("align do Frame no host, opcional (top, client...)") }),
            &["name"],
        ),
        tool("move_form", "Move ou renomeia o arquivo de um form (o JSON, as variantes e o fonte dele em cada linguagem; o módulo de código sai do nome do arquivo). A aba aberta passa a gravar no lugar novo, e o código é gerado de novo.", json!({ "name": s("nome do form"), "file": s("o arquivo novo, dentro de forms/ (forms/painel/inicio.json)") }), &["name", "file"]),
        tool("remove_form", "Tira um form do projeto (o arquivo continua no disco).", json!({ "name": s("nome do form") }), &["name"]),
        tool("set_main_form", "Define o form principal do projeto (o que abre primeiro).", json!({ "name": s("nome do form") }), &["name"]),
        tool("project_settings", "A configuração do projeto aberto (o .aylisproj): nome, linguagem, forms, pasta de saída (`output`), publicação (`publish`), renderizador, temas...", json!({}), &[]),
        tool("set_project_setting", "Muda uma chave da configuração do projeto (como Projeto → Opções) e salva. `key` com pontos para dentro (`output.dir`, `publish.version`); `value` é o JSON.", json!({ "key": s("a chave (output.dir, publish.app_name...)"), "value": { "description": "o valor JSON" } }), &["key", "value"]),
        tool("targets", "Os alvos (desktop, linux, android, web, ios) e o escolhido para Executar e Compilar.", json!({}), &[]),
        tool("set_target", "Escolhe o alvo do Executar (desktop, linux, android, web, ios).", json!({ "target": s("o alvo") }), &["target"]),
        tool("build", "Compilar (Ctrl+F9): compila sem executar (`release` true: otimizado) e espera até `max_wait` s (padrão 45, dentro do prazo de um cliente MCP): a resposta diz se compilou, com os erros e as últimas linhas, ou \"ainda compilando\" (a primeira compilação de um projeto leva minutos): aí chame `build_wait`. `wait` false só começa.", json!({ "release": { "type": "boolean" }, "wait": { "type": "boolean" }, "max_wait": { "type": "integer", "description": "segundos de espera nesta chamada (padrão 45)" } }), &[]),
        tool("run_without_debug", "Executar sem depurar (Ctrl+Shift+F9); `release` true compila otimizado. Espera a compilação como o `run` e devolve os erros dela, ou que o app está rodando (ou que abriu e caiu).", json!({ "release": { "type": "boolean" }, "max_wait": { "type": "integer" } }), &[]),
        tool(
            "publish",
            "Publica para um alvo (desktop, linux, android, web, ios), como Executar → Publicar, e espera como o `build` (até `max_wait` s; depois, `build_wait`; `wait` false só começa). `bundle`: Android em AAB (Play Store) no lugar do APK; `installer`: o instalador do Windows (o tipo das Opções do projeto). Nome, versão e identificador do app ficam em `publish.*` (set_project_setting); o ícone, em set_app_icon. A senha da chave do Android não passa pelo MCP: vem de AYLIS_KEYSTORE_PASSWORD (sem ela, a chave de depuração).",
            json!({ "target": s("o alvo (padrão: o escolhido)"), "out_dir": s("pasta de saída, opcional"), "bundle": { "type": "boolean" }, "installer": { "type": "boolean" }, "wait": { "type": "boolean" }, "max_wait": { "type": "integer", "description": "segundos de espera nesta chamada (padrão 45)" } }),
            &[],
        ),
        tool(
            "get_theme",
            "O tema do projeto como fica (o da biblioteca mais o que o arquivo do projeto muda): `value` é o tema, `own` o que o arquivo muda. Sem `section`, tudo, com só os nomes dos estilos; com `section` (colors, fonts, radius, spacing, durations, shadows, styles, components) e `key`, só aquilo (styles + Button.danger: o estilo).",
            json!({ "section": s("seção, opcional"), "key": s("token ou classe do estilo, opcional") }),
            &[],
        ),
        tool(
            "set_theme",
            "Muda um token (`section` colors e `key` primary, radius e md...) ou um estilo (`section` styles e `key` Button.danger, com o objeto do estilo) no tema do projeto, como no arquivo de tema; `value` null tira a chave (volta a valer a da base). Com um tema da biblioteca, nasce um arquivo em themes/ partindo dele, e o projeto passa a usá-lo. Confere o tema antes de gravar; os forms abertos mostram o tema novo.",
            json!({ "section": s("colors, fonts, radius, spacing, durations, shadows, styles ou components"), "key": s("o token ou a classe do estilo"), "value": { "description": "o valor JSON (\"#RRGGBB\", número, objeto do estilo; null tira)" } }),
            &["section", "key", "value"],
        ),
        tool(
            "translations",
            "As traduções do projeto (como Opções do projeto → Traduções): sem `language`, os idiomas com quantos textos faltam em cada; com ele, os textos (o original, que é a chave, e a tradução; `missing_only`: só os sem tradução). Salva antes os forms alterados.",
            json!({ "language": s("código do idioma (en-US, es...), opcional"), "missing_only": { "type": "boolean" } }),
            &[],
        ),
        tool(
            "set_translations",
            "Grava traduções de um idioma do projeto: `texts` é {\"texto original\": \"tradução\"} (tradução vazia: mostra o original). Um idioma novo nasce assim; os textos novos dos forms entram juntos, vazios.",
            json!({ "language": s("código do idioma (en-US, es...)"), "texts": { "type": "object", "description": "original → tradução" } }),
            &["language", "texts"],
        ),
        tool(
            "set_app_icon",
            "O ícone do app (como Opções do projeto → Ícone do app): `image` é um PNG de 256 px ou mais (ou SVG), relativo ao projeto ou um caminho completo (copiado para assets/images); `background` (#RRGGBB) é o fundo do ícone adaptativo do Android (vazio: a cor da borda da imagem). Gera já o .ico do Windows; as outras plataformas saem ao publicar. Devolve os avisos da imagem.",
            json!({ "image": s("a imagem do ícone"), "background": s("#RRGGBB, opcional") }),
            &["image"],
        ),
        tool(
            "check",
            "A verificação do projeto que o Executar e o Publicar fazem antes de compilar (a do `aylis-tool check`): handlers que faltam, referências entre componentes, assets, fontes, campos e o que muda numa plataforma. Devolve os erros e avisos com o form e o componente. `target` web confere também o que só vale no navegador (padrão: o alvo escolhido). Salva antes os forms alterados.",
            json!({ "target": s("o alvo, opcional (web)") }),
            &[],
        ),
        tool(
            "test",
            "Roda os testes do projeto na ferramenta da linguagem (Rust: cargo test; Python: unittest; Go: go test; TypeScript: bun/node --test; C#: dotnet test) e devolve o resumo: quantos passaram e falharam, cada teste, o que os que falharam mostraram e os erros de compilação. Salva os fontes e gera o código antes. Espera até `max_wait` s (padrão 45); ainda rodando, chame de novo (espera a mesma rodada). No modo restrito, não roda.",
            json!({ "max_wait": { "type": "integer", "description": "segundos de espera nesta chamada (padrão 45)" } }),
            &[],
        ),
        tool(
            "screenshot",
            "A foto de um form do projeto como o designer mostra (`form`; vazio: o aberto), sem abrir aba: o aberto sai com o que não foi salvo, o outro é lido do arquivo; `variant` é a de uma plataforma (android...). Com `source: \"app\"`, a janela do app em execução (`form`: outro form dele).",
            json!({ "form": s("nome do form, opcional"), "variant": s("variante, opcional"), "source": s("designer (padrão) ou app") }),
            &[],
        ),
        tool("undo", "Desfaz a última mudança no form aberto (Ctrl+Z).", json!({}), &[]),
        tool("redo", "Refaz (Ctrl+Y).", json!({}), &[]),
        tool("select", "Seleciona componentes no designer (o Object Inspector mostra o primeiro). Vazio: nenhum.", json!({ "components": { "type": "array", "items": { "type": "string" } } }), &[]),
        tool("debug_variables", "Os campos de uma variável na depuração: `reference` é o número que `debug_state` dá em cada variável (0: sem campos).", json!({ "reference": { "type": "integer" } }), &["reference"]),
        tool("debug_frame", "Escolhe um quadro da pilha (`index`, de `debug_state`): a linha no código e as variáveis passam a ser as dele.", json!({ "index": { "type": "integer" } }), &["index"]),
        tool("breakpoint_condition", "Ponto de parada com condição (`condition`, uma expressão da linguagem) e/ou contagem de passagens (`hits`); vazias: um ponto comum.", json!({ "file": s("arquivo ou nome do form"), "line": { "type": "integer" }, "condition": s("expressão, opcional"), "hits": s("passagens, opcional") }), &["file", "line"]),
        tool(
            "app_command",
            "Um comando para o app em execução, pelo canal de desenvolvimento (o app compilado pela IDE, com a feature `dev`, que o Executar liga; um app publicado não conecta). `command` é o comando do protocolo remoto do Aylis:\n\
             - {\"cmd\":\"snapshot\"}: a imagem da janela, como imagem (com \"target\": só aquele componente; \"form\": outro form);\n\
             - {\"cmd\":\"tree\",\"depth\":2,\"visible_only\":true}: a árvore (sem eles, inteira, que pode ser enorme; \"target\" começa num componente; cada nó traz `rect`, onde ficou depois do layout);\n\
             - {\"cmd\":\"rect\",\"target\":\"pnlX\"}, {\"cmd\":\"props\",\"target\":\"btnSalvar\"}, {\"cmd\":\"get\",\"target\":\"edtNome\",\"prop\":\"text\"}: posição, propriedades, um valor;\n\
             - {\"cmd\":\"set\",\"target\":\"edtNome\",\"prop\":\"text\",\"value\":\"Ana\"} (ou várias: \"props\":{...}), {\"cmd\":\"focus\",...}, {\"cmd\":\"invoke\",\"target\":...,\"method\":...,\"args\":[]}, {\"cmd\":\"highlight\",\"target\":...};\n\
             - {\"cmd\":\"click\",\"target\":\"btnSalvar\"}: o clique do componente; com \"modifiers\" (\"Ctrl+Shift\" ou [\"ctrl\",\"shift\"]; ctrl, shift, alt, meta), \"x\"/\"y\" (dentro do componente; padrão o meio), \"button\" (left, right, middle) ou \"double\":true, o mouse de verdade naquele ponto (Ctrl+clique e Shift+clique na lista, o menu do botão direito);\n\
             - {\"cmd\":\"drag\",\"from\":\"lstOrigem\",\"to\":\"lstDestino\"}: aperta no meio de \"from\", anda em \"steps\" passos (10) e solta no meio de \"to\" (o arrastar e soltar entre componentes; \"from\" igual a \"to\" arrasta dentro de um), com \"from_x\"/\"from_y\"/\"to_x\"/\"to_y\" (dentro de cada um), \"modifiers\" e \"button\" como no click;\n\
             - {\"cmd\":\"key\",\"target\":\"frmX\",\"key\":\"Ctrl+A\"} (\"Enter\", \"Shift+Tab\", \"Alt+S\"; \"modifiers\" soma: {\"key\":\"A\",\"modifiers\":[\"ctrl\"]}): a tecla no controle com foco do form; {\"cmd\":\"type\",\"target\":\"frmX\",\"text\":\"Ana\"}: o texto digitado nele;\n\
             - {\"cmd\":\"trace\",\"on\":true}: o rastro de eventos (cada evento com handler vai para `output`, source program);\n\
             - {\"cmd\":\"profile\",\"on\":true} começa a medir e {\"cmd\":\"profile_take\"} devolve os blocos medidos, as amostras e a memória ({\"cmd\":\"profile\",\"on\":false} para);\n\
             - {\"cmd\":\"reload\",\"kind\":\"form\"|\"theme\"|\"locale\",\"name\":\"frmX\",\"text\":\"...\"}: o hot reload (a IDE já manda ao salvar);\n\
             - {\"cmd\":\"close\"}: fecha o app;\n\
             - {\"cmd\":\"apps\"}: os apps conectados (respondido pela IDE).\n\
             O target é o nome, o caminho (\"frmX.fraY.lblTotal\") ou o id; um nome que está em mais de um lugar é recusado com os caminhos. Com mais de um app conectado, `app` escolhe (o pid ou o nome do projeto, como `apps` lista); sem ele, o último que conectou.",
            json!({ "command": { "type": "object", "description": "o comando" }, "app": { "description": "o app: o pid (número) ou o nome do projeto, como {\"cmd\":\"apps\"} lista; padrão: o último que conectou" } }),
            &["command"],
        ),
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
    fn the_project_of_each_call_picks_the_ide() {
        let ide = |pid: u32, project: Option<&str>| crate::instances::Instance {
            pid,
            project: project.map(PathBuf::from),
            mcp_port: Some(7000 + pid as u16),
            control_port: 1,
            secret: "s".into(),
            focused_at: 0.0,
            sessions: vec!["1-a".into()],
        };
        let ides = [ide(1, Some("C:/git/noobs/NoOBS.aylisproj")), ide(2, Some("C:/git/RTSPlayer/client/RTSPlayer.aylisproj"))];
        // Pelo nome ou pelo caminho, cada chamada vai para a IDE do projeto dela.
        assert_eq!(route_call("open_form", Some("NoOBS"), &ides), Route::Ide(1));
        assert_eq!(route_call("build", Some("C:/git/RTSPlayer/client/RTSPlayer.aylisproj"), &ides), Route::Ide(2));
        assert_eq!(route_call("session_title", Some("rtsplayer"), &ides), Route::Ide(2));
        // Sem projeto, com duas abertas: recusada, dizendo quais são.
        let Route::Refuse(text) = route_call("project_info", None, &ides) else { panic!("devia recusar") };
        assert!(text.contains("NoOBS.aylisproj") && text.contains("RTSPlayer.aylisproj"), "{text}");
        // Um projeto que nenhuma tem; um nome que duas têm.
        assert!(matches!(route_call("open_form", Some("Calculadora"), &ides), Route::Refuse(_)));
        assert!(matches!(route_call("open_form", Some(".aylisproj"), &ides), Route::Refuse(_)));
        // Uma só aberta: como antes, sem precisar do projeto.
        assert_eq!(route_call("project_info", None, &ides[..1]), Route::Default);
        // Abrir e criar projeto escolhem a IDE pelo caminho deles.
        assert_eq!(route_call("open_project", None, &ides), Route::Default);
        assert_eq!(route_call("new_project", None, &ides), Route::Default);
    }

    /// Toda ferramenta da IDE (menos abrir e criar projeto) e o `session_title` aceitam o
    /// `project`; as instruções dizem para passar sempre.
    #[test]
    fn every_tool_takes_the_project() {
        for t in ide_tools().iter().chain(own_tools().as_array().unwrap().iter().filter(|t| t["name"] == "session_title")) {
            let name = t["name"].as_str().unwrap();
            let has = t["inputSchema"]["properties"].get(PROJECT_ARG).is_some();
            assert_eq!(has, !matches!(name, "open_project" | "new_project"), "{name}");
        }
        assert!(INSTRUCTIONS.contains("`project`"));
    }

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

#[cfg(test)]
mod argument_tests {
    use serde_json::json;

    /// M06 do Tabernáculo: o argumento que a ferramenta não tem é recusado, com os que valem; o
    /// `project` vale em todas.
    #[test]
    fn an_unknown_argument_is_refused() {
        let text = super::unknown_arguments("set_property", &json!({ "component": "btn", "propery": "text", "value": 1 })).unwrap();
        assert!(text.contains("'propery'") && text.contains("property"), "{text}");
        assert_eq!(super::unknown_arguments("set_property", &json!({ "component": "btn", "property": "text", "value": 1, "project": "x" })), None);
        assert_eq!(super::unknown_arguments("ferramenta_nova", &json!({ "x": 1 })), None);
    }

    /// M21 do Tabernáculo: o `app_command` diz todos os comandos que o app atende e aceita `app`;
    /// M10/M11/M12/M20: o `menu` e o `dialog_click` aceitam o `id`, o `task_status` existe e o
    /// `output` filtra pela origem e pelo nível.
    #[test]
    fn the_tools_describe_what_they_take() {
        let tools = super::ide_tools();
        let tool = |name: &str| tools.iter().find(|t| t["name"] == name).unwrap_or_else(|| panic!("falta {name}")).clone();
        let app = tool("app_command");
        let text = app["description"].as_str().unwrap();
        for cmd in ["snapshot", "tree", "rect", "props", "set", "click", "drag", "key", "type", "invoke", "trace", "profile", "profile_take", "reload", "close", "apps"] {
            assert!(text.contains(&format!("\"cmd\":\"{cmd}\"")), "falta {cmd}");
        }
        assert!(text.contains("feature `dev`"));
        assert!(text.contains("\"modifiers\"") && text.contains("Ctrl+Shift"), "as teclas seguradas no clique e no arrasto");
        assert_eq!(super::unknown_arguments("app_command", &json!({ "command": { "cmd": "tree" }, "app": 1234 })), None);
        assert_eq!(super::unknown_arguments("menu", &json!({ "id": 12 })), None);
        assert_eq!(super::unknown_arguments("dialog_click", &json!({ "id": 12 })), None);
        assert_eq!(super::unknown_arguments("task_status", &json!({ "task": 1, "max_wait": 5 })), None);
        assert_eq!(super::unknown_arguments("output", &json!({ "lines": 5, "source": "build", "level": "error", "previous": true })), None);
        assert!(tool("menu")["inputSchema"]["required"].as_array().unwrap().is_empty(), "o id basta");
    }
}
