//! Linux: notificações push pelo UnifiedPush (o padrão aberto do Linux, pelo D-Bus da
//! sessão). Um distribuidor instalado (ntfy, KUnifiedPush, NextPush...) mantém a conexão com
//! o servidor de push e entrega ao app: o aylis publica o `org.unifiedpush.Connector1` com o
//! nome do app no barramento e pede o registro ao `org.unifiedpush.Distributor1`. O token é o
//! endpoint (um endereço HTTP para onde o servidor do app manda um POST com a mensagem, em
//! [`push_message_from_payload`]).

use std::sync::{Arc, Mutex};

use aylis_core::platform::{PushConfig, PushEvent, PushMessage, PushService, push_event, push_message_from_payload};

const DISTRIBUTOR_PREFIX: &str = "org.unifiedpush.Distributor.";
const DISTRIBUTOR_PATH: &str = "/org/unifiedpush/Distributor";
const DISTRIBUTOR_INTERFACE: &str = "org.unifiedpush.Distributor1";

/// O lado do app: o distribuidor chama estes métodos.
struct Connector {
    token: String,
}

#[zbus::interface(name = "org.unifiedpush.Connector1")]
impl Connector {
    fn message(&self, token: &str, message: Vec<u8>, id: &str) {
        if token == self.token {
            let m = push_message_from_payload(&String::from_utf8_lossy(&message));
            push_event(PushEvent::Message(PushMessage { id: if m.id.is_empty() { id.to_string() } else { m.id.clone() }, ..m }));
        }
    }

    fn new_endpoint(&self, token: &str, endpoint: &str) {
        if token == self.token {
            push_event(PushEvent::Token(endpoint.to_string()));
        }
    }

    fn unregistered(&self, token: &str) {
        if token == self.token {
            push_event(PushEvent::Token(String::new()));
        }
    }
}

/// O nome do app no barramento (`dev.aylis.<executável>`): o distribuidor chama por ele.
fn service_name() -> String {
    let exe = std::env::current_exe().ok().and_then(|p| p.file_stem().map(|s| s.to_string_lossy().into_owned())).unwrap_or_else(|| "app".into());
    let clean: String = exe.chars().map(|c| if c.is_ascii_alphanumeric() { c } else { '_' }).collect();
    let clean = if clean.starts_with(|c: char| c.is_ascii_digit()) { format!("_{clean}") } else { clean };
    format!("dev.aylis.{clean}")
}

/// O token da instalação (o mesmo a cada abertura: o distribuidor guarda o registro por ele).
fn instance_token(service: &str) -> String {
    let dir = std::env::var_os("XDG_CONFIG_HOME")
        .map(std::path::PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| std::path::Path::new(&h).join(".config")))
        .unwrap_or_else(std::env::temp_dir)
        .join(service);
    let file = dir.join("unifiedpush-token");
    if let Ok(t) = std::fs::read_to_string(&file) {
        if !t.trim().is_empty() {
            return t.trim().to_string();
        }
    }
    let seed = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_nanos()).unwrap_or_default() ^ (u128::from(std::process::id()) << 64);
    let token = format!("{seed:032x}");
    let _ = std::fs::create_dir_all(&dir);
    let _ = std::fs::write(&file, &token);
    token
}

#[derive(Default)]
pub(crate) struct UnifiedPush {
    state: Arc<Mutex<Option<(zbus::blocking::Connection, String, String)>>>,
}

/// Os distribuidores no barramento (os rodando e os que o D-Bus sabe abrir).
fn distributors(conn: &zbus::blocking::Connection) -> zbus::Result<Vec<String>> {
    let dbus = zbus::blocking::fdo::DBusProxy::new(conn)?;
    let mut names: Vec<String> = dbus.list_names()?.into_iter().map(|n| n.to_string()).collect();
    names.extend(dbus.list_activatable_names()?.into_iter().map(|n| n.to_string()));
    names.retain(|n| n.starts_with(DISTRIBUTOR_PREFIX));
    names.sort();
    names.dedup();
    Ok(names)
}

fn register(state: &Mutex<Option<(zbus::blocking::Connection, String, String)>>) -> Result<(), String> {
    let service = service_name();
    let token = instance_token(&service);
    let conn = zbus::blocking::connection::Builder::session()
        .and_then(|b| b.name(service.as_str()))
        .and_then(|b| b.serve_at("/org/unifiedpush/Connector", Connector { token: token.clone() }))
        .and_then(|b| b.build())
        .map_err(|e| format!("UnifiedPush: {e}"))?;
    let list = distributors(&conn).map_err(|e| format!("UnifiedPush: {e}"))?;
    let Some(distributor) = list.first().cloned() else {
        return Err(aylis_core::messages::t("nenhum distribuidor do UnifiedPush instalado (ntfy, KUnifiedPush...)"));
    };
    let description = std::env::current_exe().ok().and_then(|p| p.file_stem().map(|s| s.to_string_lossy().into_owned())).unwrap_or_default();
    let (result, reason): (String, String) = zbus::blocking::Proxy::new(&conn, distributor.clone(), DISTRIBUTOR_PATH, DISTRIBUTOR_INTERFACE)
        .and_then(|proxy| proxy.call("Register", &(service.as_str(), token.as_str(), description.as_str())))
        .map_err(|e| format!("UnifiedPush: {e}"))?;
    if result != "REGISTRATION_SUCCEEDED" {
        return Err(format!("UnifiedPush: {result} {reason}").trim().to_string());
    }
    // O endpoint chega depois, no NewEndpoint. A conexão fica viva para receber.
    if let Ok(mut s) = state.lock() {
        *s = Some((conn, distributor, token));
    }
    Ok(())
}

impl PushService for UnifiedPush {
    fn available(&self) -> bool {
        zbus::blocking::Connection::session().ok().and_then(|c| distributors(&c).ok()).is_some_and(|l| !l.is_empty())
    }

    fn provider(&self) -> &'static str {
        "unifiedpush"
    }

    fn register(&mut self, _config: &PushConfig) {
        if self.state.lock().is_ok_and(|s| s.is_some()) {
            return;
        }
        let state = self.state.clone();
        std::thread::spawn(move || {
            if let Err(e) = register(&state) {
                push_event(PushEvent::Error(e));
            }
        });
    }

    fn unregister(&mut self) {
        if let Some((conn, distributor, token)) = self.state.lock().ok().and_then(|mut s| s.take()) {
            if let Ok(proxy) = zbus::blocking::Proxy::new(&conn, distributor.as_str(), DISTRIBUTOR_PATH, DISTRIBUTOR_INTERFACE) {
                let _: zbus::Result<()> = proxy.call("Unregister", &(token.as_str(),));
            }
        }
    }

    // O distribuidor só entrega: quem mostra é o componente.
    fn shows_notifications(&self) -> bool {
        false
    }
}
