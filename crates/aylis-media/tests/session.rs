//! O `MediaSession` com um serviço falso no lugar do sistema: começa na primeira atualização, manda
//! o que toca (o nome do arquivo sem `MediaInfo`, a próxima ligada pela `Playlist`), recebe os
//! comandos do sistema no `on_command` e sai dos controles quando `active` desliga.

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use aylis_controls::Form;
use aylis_core::platform::{MediaCommand, MediaCommandSink, MediaSessionService, NowPlaying};
use aylis_core::*;
use aylis_media::{MediaPlayer, MediaSession, Playlist};

#[derive(Default)]
struct Log {
    started: Option<Option<ComponentId>>,
    updates: Vec<NowPlaying>,
    stopped: bool,
    sink: Option<MediaCommandSink>,
}

struct Fake(Arc<Mutex<Log>>);

impl MediaSessionService for Fake {
    fn start(&mut self, owner: Option<ComponentId>, commands: MediaCommandSink) -> Result<(), String> {
        let mut l = self.0.lock().unwrap();
        l.started = Some(owner);
        l.sink = Some(commands);
        Ok(())
    }
    fn update(&mut self, now: &NowPlaying) {
        self.0.lock().unwrap().updates.push(now.clone());
    }
    fn stop(&mut self) {
        self.0.lock().unwrap().stopped = true;
    }
}

#[test]
fn the_session_shows_what_plays_and_takes_the_system_commands() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    aylis_media::register(&mut ui);
    let log = Arc::new(Mutex::new(Log::default()));
    ui.services_mut().media_session = Some(Box::new(Fake(log.clone())));
    let form = ui.add_root(Form::new("frmPlayer", 400.0, 300.0));
    let player = ui.add(MediaPlayer::new("C:/musicas/Minha Canção.mp3"), form);
    let list = ui.add(Playlist::default(), form);
    ui.set_prop(list, "player", PropValue::Component(Some(player))).unwrap();
    ui.set_prop(list, "items", "a.mp3;b.mp3").unwrap();
    let session = ui.add(MediaSession::default(), form);
    ui.set_prop(session, "player", PropValue::Component(Some(player))).unwrap();
    ui.set_prop(session, "playlist", PropValue::Component(Some(list))).unwrap();
    ui.set_prop(session, "forward", false).unwrap();
    let commands = Rc::new(RefCell::new(Vec::new()));
    let seen = commands.clone();
    ui.on(session, "on_command", move |_, _, args| {
        if let EventArgs::Text(t) = args {
            seen.borrow_mut().push(t.clone());
        }
    });
    ui.process_posted();
    {
        let l = log.lock().unwrap();
        assert_eq!(l.started, Some(Some(form)), "começa ligada ao form");
        let now = l.updates.last().expect("mandou o que toca");
        assert_eq!(now.title, "Minha Canção");
        assert!(now.can_next && !now.playing, "{now:?}");
    }
    // Uma tecla de mídia: chega pela função do serviço, de outra thread se for o caso.
    let sink = log.lock().unwrap().sink.clone().unwrap();
    std::thread::spawn(move || sink(MediaCommand::Toggle)).join().unwrap();
    ui.process_posted();
    assert_eq!(*commands.borrow(), vec!["toggle".to_string()]);
    ui.set_prop(session, "active", false).unwrap();
    ui.process_posted();
    assert!(log.lock().unwrap().stopped, "sai dos controles do sistema");
}
