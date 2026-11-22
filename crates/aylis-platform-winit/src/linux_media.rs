//! Linux: os controles de mídia do sistema pelo MPRIS (o padrão do D-Bus que o GNOME, o KDE, o
//! Cinnamon e o `playerctl` leem) para o `MediaSession`. O app publica o
//! `org.mpris.MediaPlayer2.<app>` na sessão, com o objeto `/org/mpris/MediaPlayer2` e as duas
//! interfaces (`org.mpris.MediaPlayer2` e `.Player`); as teclas de mídia chegam pelos métodos, e
//! cada atualização avisa o painel pelo `PropertiesChanged`.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use aylis_core::ComponentId;
use aylis_core::platform::{MediaCommand, MediaCommandSink, MediaSessionService, NowPlaying};
use zbus::zvariant::{ObjectPath, OwnedValue, Value};

const PATH: &str = "/org/mpris/MediaPlayer2";

/// O estado que o painel lê: o que toca e onde ficou a capa (o MPRIS pede um endereço).
#[derive(Default)]
struct State {
    now: NowPlaying,
    art: String,
}

/// A interface raiz: o nome do app e o `.desktop` dele (o painel mostra o ícone).
struct Root {
    identity: String,
    desktop: String,
}

#[zbus::interface(name = "org.mpris.MediaPlayer2")]
impl Root {
    fn raise(&self) {}

    fn quit(&self) {}

    #[zbus(property)]
    fn can_quit(&self) -> bool {
        false
    }

    #[zbus(property)]
    fn can_raise(&self) -> bool {
        false
    }

    #[zbus(property)]
    fn has_track_list(&self) -> bool {
        false
    }

    #[zbus(property)]
    fn identity(&self) -> String {
        self.identity.clone()
    }

    #[zbus(property)]
    fn desktop_entry(&self) -> String {
        self.desktop.clone()
    }

    #[zbus(property)]
    fn supported_uri_schemes(&self) -> Vec<String> {
        Vec::new()
    }

    #[zbus(property)]
    fn supported_mime_types(&self) -> Vec<String> {
        Vec::new()
    }
}

/// O player: os comandos vão para o app pelo `sink`.
struct Player {
    state: Arc<Mutex<State>>,
    sink: MediaCommandSink,
}

fn owned<'a>(v: impl Into<Value<'a>>) -> Option<OwnedValue> {
    v.into().try_to_owned().ok()
}

impl Player {
    fn now(&self) -> NowPlaying {
        self.state.lock().map(|s| s.now.clone()).unwrap_or_default()
    }
}

#[zbus::interface(name = "org.mpris.MediaPlayer2.Player")]
impl Player {
    fn next(&self) {
        (self.sink)(MediaCommand::Next);
    }

    fn previous(&self) {
        (self.sink)(MediaCommand::Previous);
    }

    fn pause(&self) {
        (self.sink)(MediaCommand::Pause);
    }

    fn play_pause(&self) {
        (self.sink)(MediaCommand::Toggle);
    }

    fn stop(&self) {
        (self.sink)(MediaCommand::Stop);
    }

    fn play(&self) {
        (self.sink)(MediaCommand::Play);
    }

    /// Anda `offset` microssegundos a partir de agora.
    fn seek(&self, offset: i64) {
        let at = self.now().position + offset as f64 / 1_000_000.0;
        (self.sink)(MediaCommand::Seek(at.max(0.0)));
    }

    fn set_position(&self, _track: ObjectPath<'_>, position: i64) {
        (self.sink)(MediaCommand::Seek(position.max(0) as f64 / 1_000_000.0));
    }

    fn open_uri(&self, _uri: &str) {}

    #[zbus(property)]
    fn playback_status(&self) -> String {
        if self.now().playing { "Playing" } else { "Paused" }.into()
    }

    #[zbus(property)]
    fn rate(&self) -> f64 {
        1.0
    }

    #[zbus(property)]
    fn minimum_rate(&self) -> f64 {
        1.0
    }

    #[zbus(property)]
    fn maximum_rate(&self) -> f64 {
        1.0
    }

    #[zbus(property)]
    fn volume(&self) -> f64 {
        1.0
    }

    #[zbus(property)]
    fn position(&self) -> i64 {
        (self.now().position * 1_000_000.0) as i64
    }

    #[zbus(property)]
    fn metadata(&self) -> HashMap<String, OwnedValue> {
        let (now, art) = self.state.lock().map(|s| (s.now.clone(), s.art.clone())).unwrap_or_default();
        let mut m = HashMap::new();
        if let Some(v) = ObjectPath::try_from("/dev/aylis/track/current").ok().and_then(owned) {
            m.insert("mpris:trackid".into(), v);
        }
        let mut put = |key: &str, v: Option<OwnedValue>| {
            if let Some(v) = v {
                m.insert(key.to_string(), v);
            }
        };
        put("xesam:title", owned(now.title.as_str()));
        if !now.artist.is_empty() {
            put("xesam:artist", owned(vec![now.artist.as_str()]));
        }
        if !now.album.is_empty() {
            put("xesam:album", owned(now.album.as_str()));
        }
        if now.duration > 0.0 {
            put("mpris:length", owned((now.duration * 1_000_000.0) as i64));
        }
        if !art.is_empty() {
            put("mpris:artUrl", owned(art.as_str()));
        }
        m
    }

    #[zbus(property)]
    fn can_go_next(&self) -> bool {
        self.now().can_next
    }

    #[zbus(property)]
    fn can_go_previous(&self) -> bool {
        self.now().can_previous
    }

    #[zbus(property)]
    fn can_play(&self) -> bool {
        true
    }

    #[zbus(property)]
    fn can_pause(&self) -> bool {
        true
    }

    #[zbus(property)]
    fn can_seek(&self) -> bool {
        self.now().duration > 0.0
    }

    #[zbus(property)]
    fn can_control(&self) -> bool {
        true
    }
}

/// O MPRIS do app.
#[derive(Default)]
pub(crate) struct Mpris {
    conn: Option<zbus::blocking::Connection>,
    state: Arc<Mutex<State>>,
}

/// O nome do app no barramento (`org.mpris.MediaPlayer2.<executável>`) e o do `.desktop`.
fn names() -> (String, String) {
    let exe = std::env::current_exe().ok().and_then(|p| p.file_stem().map(|s| s.to_string_lossy().into_owned())).unwrap_or_else(|| "app".into());
    let clean: String = exe.chars().map(|c| if c.is_ascii_alphanumeric() { c } else { '_' }).collect();
    let clean = if clean.starts_with(|c: char| c.is_ascii_digit()) { format!("_{clean}") } else { clean };
    (format!("org.mpris.MediaPlayer2.{clean}"), exe.to_lowercase().replace(' ', "-"))
}

/// A capa num arquivo (o MPRIS pede um endereço): na pasta temporária do usuário.
fn write_cover(bytes: &[u8]) -> Option<String> {
    let dir = std::env::var_os("XDG_RUNTIME_DIR").map(std::path::PathBuf::from).unwrap_or_else(std::env::temp_dir);
    let path = dir.join(format!("aylis-mpris-{}.img", std::process::id()));
    std::fs::write(&path, bytes).ok()?;
    Some(format!("file://{}", path.display()))
}

impl MediaSessionService for Mpris {
    fn start(&mut self, _owner: Option<ComponentId>, commands: MediaCommandSink) -> Result<(), String> {
        let (bus, desktop) = names();
        let identity = std::env::current_exe().ok().and_then(|p| p.file_stem().map(|s| s.to_string_lossy().into_owned())).unwrap_or_default();
        let conn = zbus::blocking::connection::Builder::session()
            .and_then(|b| b.name(bus.as_str()))
            .and_then(|b| b.serve_at(PATH, Root { identity, desktop }))
            .and_then(|b| b.serve_at(PATH, Player { state: self.state.clone(), sink: commands }))
            .and_then(|b| b.build())
            .map_err(|e| format!("MPRIS: {e}"))?;
        self.conn = Some(conn);
        Ok(())
    }

    fn update(&mut self, now: &NowPlaying) {
        let Some(conn) = &self.conn else { return };
        if let Ok(mut s) = self.state.lock() {
            if s.now.cover != now.cover {
                s.art = now.cover.as_deref().and_then(write_cover).unwrap_or_default();
            }
            s.now = now.clone();
        }
        // O painel fica sabendo pelo `PropertiesChanged`.
        let Ok(iface) = conn.object_server().interface::<_, Player>(PATH) else { return };
        let emitter = iface.signal_emitter().clone();
        let player = iface.get();
        let result: zbus::Result<()> = zbus::block_on(async {
            player.playback_status_changed(&emitter).await?;
            player.metadata_changed(&emitter).await?;
            player.can_go_next_changed(&emitter).await?;
            player.can_go_previous_changed(&emitter).await?;
            player.can_seek_changed(&emitter).await?;
            Ok(())
        });
        if let Err(e) = result {
            aylis_core::stderr_line!("aylis: MPRIS: {e}");
        }
    }

    fn stop(&mut self) {
        // Soltar a conexão tira o nome do barramento: o app sai do painel.
        self.conn = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Pelo D-Bus de verdade (só com um barramento de sessão; sem ele, pula): o painel lê o
    /// título e o estado, e o `PlayPause` dele chega ao app como `Toggle`.
    #[test]
    fn the_panel_reads_the_track_and_sends_commands() {
        let Ok(probe) = zbus::blocking::Connection::session() else { return };
        drop(probe);
        let got = Arc::new(Mutex::new(Vec::new()));
        let seen = got.clone();
        let mut mpris = Mpris::default();
        mpris.start(None, Arc::new(move |c| seen.lock().unwrap().push(c))).unwrap();
        mpris.update(&NowPlaying { title: "Música".into(), artist: "Banda".into(), duration: 180.0, playing: true, can_next: true, ..Default::default() });
        let (bus, _) = names();
        let client = zbus::blocking::Connection::session().unwrap();
        let player = zbus::blocking::Proxy::new(&client, bus.as_str(), PATH, "org.mpris.MediaPlayer2.Player").unwrap();
        let status: String = player.get_property("PlaybackStatus").unwrap();
        assert_eq!(status, "Playing");
        let meta: HashMap<String, OwnedValue> = player.get_property("Metadata").unwrap();
        let title: String = meta["xesam:title"].try_clone().unwrap().try_into().unwrap();
        assert_eq!(title, "Música");
        let next: bool = player.get_property("CanGoNext").unwrap();
        assert!(next);
        let _: () = player.call("PlayPause", &()).unwrap();
        let _: () = player.call("SetPosition", &(ObjectPath::try_from("/dev/aylis/track/current").unwrap(), 30_000_000i64)).unwrap();
        assert_eq!(*got.lock().unwrap(), vec![MediaCommand::Toggle, MediaCommand::Seek(30.0)]);
        mpris.stop();
    }
}
