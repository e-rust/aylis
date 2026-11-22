//! `MediaSession`: o app nos controles de mídia do sistema. O que toca aparece fora do app (o
//! quadro do volume e a miniatura da barra de tarefas do Windows, o painel do Linux, a Central de
//! Controle do macOS, a tela bloqueada do celular, a aba do navegador), e as teclas de mídia e os
//! botões do fone controlam o player. Decidido em 21/11 (o relatório de um player de música
//! portado para o Aylis: o app original fazia isso com ~450 linhas por sistema).
//!
//! Ligado a um `MediaPlayer` (`player`), opcionalmente a uma `Playlist` (próxima e anterior) e a
//! um `MediaInfo` (título, artista, álbum e capa; sem ele, o nome do arquivo). Os comandos do
//! sistema chegam no `on_command` (e no `on_seek`) e, com `forward` (ligado), já vão para o
//! player e a lista. Cada sistema implementa pelo serviço `media_session` da plataforma; onde
//! não existe, o componente existe e não liga.

use aylis_core::platform::{MediaCommand, NowPlaying};
use aylis_core::{Component, ComponentId, EventArgs, EventCtx, PropValue, Ui, properties};

use crate::{MediaInfo, MediaPlayer, Playlist, VideoPlayer};

/// O player ligado: um `MediaPlayer` (o som) ou um `VideoPlayer` (o som do vídeo).
#[derive(Clone, Copy)]
enum Target {
    Audio(ComponentId),
    Video(ComponentId),
}

impl Target {
    fn of(ui: &Ui, id: ComponentId) -> Option<Target> {
        match ui.class_name(id) {
            Some("MediaPlayer") => Some(Target::Audio(id)),
            Some("VideoPlayer") => Some(Target::Video(id)),
            _ => None,
        }
    }

    fn file(self, ui: &Ui) -> String {
        match self {
            Target::Audio(p) => ui.get::<MediaPlayer>(p).map(|m| m.file.clone()).unwrap_or_default(),
            Target::Video(p) => ui.get::<VideoPlayer>(p).map(|v| v.file.clone()).unwrap_or_default(),
        }
    }

    fn playing(self, ui: &Ui) -> bool {
        match self {
            Target::Audio(p) => MediaPlayer::is_playing(ui, p),
            Target::Video(p) => VideoPlayer::is_playing(ui, p),
        }
    }

    fn position(self, ui: &Ui) -> f64 {
        match self {
            Target::Audio(p) => MediaPlayer::position(ui, p),
            Target::Video(p) => VideoPlayer::position(ui, p),
        }
    }

    fn duration(self, ui: &Ui) -> f64 {
        match self {
            Target::Audio(p) => MediaPlayer::duration(ui, p),
            Target::Video(p) => VideoPlayer::duration(ui, p),
        }
        .unwrap_or_default()
    }

    fn play(self, ui: &mut Ui) -> Result<(), String> {
        match self {
            Target::Audio(p) => MediaPlayer::play(ui, p),
            Target::Video(p) => VideoPlayer::play(ui, p),
        }
    }

    fn pause(self, ui: &mut Ui) {
        match self {
            Target::Audio(p) => MediaPlayer::pause(ui, p),
            Target::Video(p) => VideoPlayer::pause(ui, p),
        }
    }

    fn stop(self, ui: &mut Ui) {
        match self {
            Target::Audio(p) => MediaPlayer::stop(ui, p),
            Target::Video(p) => VideoPlayer::stop(ui, p),
        }
    }

    fn seek(self, ui: &mut Ui, t: f64) {
        match self {
            Target::Audio(p) => MediaPlayer::seek(ui, p, t),
            Target::Video(p) => VideoPlayer::seek(ui, p, t),
        }
    }
}

/// Os controles de mídia do sistema para um player. Não visual.
pub struct MediaSession {
    /// O `MediaPlayer` (ou o `VideoPlayer`) que o sistema controla.
    pub player: Option<ComponentId>,
    /// A `Playlist` do player: a próxima e a anterior do sistema vão para ela.
    pub playlist: Option<ComponentId>,
    /// O `MediaInfo` com o título, o artista, o álbum e a capa (sem ele, o nome do arquivo).
    pub info: Option<ComponentId>,
    /// Aparece nos controles do sistema (desligado: sai deles).
    pub active: bool,
    /// Os comandos do sistema já vão para o player e a lista (desligado: só os eventos).
    pub forward: bool,
    started: bool,
    /// O último enviado ao sistema (para não mandar o mesmo de novo).
    last: Option<NowPlaying>,
}

impl Default for MediaSession {
    fn default() -> Self {
        Self { player: None, playlist: None, info: None, active: true, forward: true, started: false, last: None }
    }
}

impl MediaSession {
    /// O que toca agora, do player, da lista e das tags.
    fn now_playing(ui: &Ui, id: ComponentId) -> Option<NowPlaying> {
        let s = ui.get::<MediaSession>(id)?;
        let player = Target::of(ui, s.player?)?;
        let file = player.file(ui);
        let mut now = NowPlaying { playing: player.playing(ui), position: player.position(ui), duration: player.duration(ui), ..Default::default() };
        if let Some(info) = s.info.and_then(|i| ui.get::<MediaInfo>(i).map(|m| (i, m))) {
            let (i, m) = info;
            now.title = m.title.clone();
            now.artist = m.artist.clone();
            now.album = m.album.clone();
            if now.duration <= 0.0 {
                now.duration = m.duration;
            }
            now.cover = MediaInfo::cover(ui, i);
        }
        if now.title.is_empty() {
            now.title = std::path::Path::new(&file).file_stem().map(|s| s.to_string_lossy().into_owned()).unwrap_or(file);
        }
        if let Some(list) = s.playlist.and_then(|p| ui.get::<Playlist>(p)) {
            let n = list.files().len();
            now.can_next = n > 1 || list.repeat != crate::RepeatMode::Off;
            now.can_previous = n > 0;
        }
        Some(now)
    }

    /// Manda ao sistema o que toca agora (e começa a sessão, na primeira vez).
    pub fn refresh(ui: &mut Ui, id: ComponentId) {
        if ui.design_mode() {
            return;
        }
        let Some(s) = ui.get::<MediaSession>(id) else { return };
        if !s.active || s.player.is_none() {
            return;
        }
        if !s.started {
            let poster = ui.poster();
            let sink: aylis_core::platform::MediaCommandSink = std::sync::Arc::new(move |cmd| {
                poster.post_to(id, move |ui| MediaSession::command(ui, id, cmd));
            });
            let owner = Some(ui.root_of(id));
            let Some(service) = ui.services_mut().media_session.as_mut() else { return };
            if let Err(e) = service.start(owner, sink) {
                aylis_core::stderr_line!("aylis: MediaSession: {e}");
                return;
            }
            if let Some(s) = ui.get_mut::<MediaSession>(id) {
                s.started = true;
            }
        }
        let Some(now) = Self::now_playing(ui, id) else { return };
        if ui.get::<MediaSession>(id).is_some_and(|s| s.last.as_ref() == Some(&now)) {
            return;
        }
        if let Some(service) = ui.services_mut().media_session.as_mut() {
            service.update(&now);
        }
        if let Some(s) = ui.get_mut::<MediaSession>(id) {
            s.last = Some(now);
        }
    }

    /// Sai dos controles do sistema.
    fn end(ui: &mut Ui, id: ComponentId) {
        let started = ui.get_mut::<MediaSession>(id).is_some_and(|s| std::mem::take(&mut s.started));
        if started {
            if let Some(service) = ui.services_mut().media_session.as_mut() {
                service.stop();
            }
        }
        if let Some(s) = ui.get_mut::<MediaSession>(id) {
            s.last = None;
        }
    }

    /// Um comando do sistema: o evento e, com `forward`, o player e a lista.
    pub fn command(ui: &mut Ui, id: ComponentId, cmd: MediaCommand) {
        let Some((player, playlist, forward)) = ui.get::<MediaSession>(id).map(|s| (s.player, s.playlist, s.forward)) else { return };
        ui.fire(id, "on_command", &EventArgs::Text(cmd.name().to_string()));
        if let MediaCommand::Seek(t) = cmd {
            ui.fire(id, "on_seek", &EventArgs::Value(PropValue::Float(t)));
        }
        if forward {
            if let Some(p) = player.and_then(|p| Target::of(ui, p)) {
                let result = match cmd {
                    MediaCommand::Play => p.play(ui),
                    MediaCommand::Pause => {
                        p.pause(ui);
                        Ok(())
                    }
                    MediaCommand::Toggle if p.playing(ui) => {
                        p.pause(ui);
                        Ok(())
                    }
                    MediaCommand::Toggle => p.play(ui),
                    MediaCommand::Stop => {
                        p.stop(ui);
                        Ok(())
                    }
                    MediaCommand::Seek(t) => {
                        p.seek(ui, t);
                        Ok(())
                    }
                    MediaCommand::Next => playlist.map_or(Ok(()), |l| Playlist::next(ui, l)),
                    MediaCommand::Previous => match playlist {
                        Some(l) => Playlist::previous(ui, l),
                        None => {
                            p.seek(ui, 0.0);
                            Ok(())
                        }
                    },
                };
                if let Err(e) = result {
                    aylis_core::stderr_line!("aylis: MediaSession: {e}");
                }
            }
        }
        Self::refresh(ui, id);
    }

    /// O player mudou (tocou, pausou, outro arquivo, as tags chegaram): as sessões ligadas a
    /// ele avisam o sistema.
    pub(crate) fn player_changed(ui: &mut Ui, player: ComponentId) {
        let ids: Vec<ComponentId> = ui.components().filter(|&c| ui.get::<MediaSession>(c).is_some_and(|s| s.player == Some(player))).collect();
        for id in ids {
            Self::refresh(ui, id);
        }
    }
}

impl Component for MediaSession {
    fn class_name(&self) -> &'static str {
        "MediaSession"
    }

    properties! {
        player: Option<ComponentId> => "Behavior",
        playlist: Option<ComponentId> => "Behavior",
        info: Option<ComponentId> => "Behavior",
        active: bool => "Behavior",
        forward: bool => "Behavior";
        events: on_command: String, on_seek: f64
    }

    fn property_changed(&mut self, ctx: &mut EventCtx<'_>, name: &str) {
        let id = ctx.id;
        match name {
            "active" if !self.active => ctx.ui.poster().post_to(id, move |ui| MediaSession::end(ui, id)),
            "active" | "player" | "playlist" | "info" => {
                self.last = None;
                ctx.ui.poster().post_to(id, move |ui| MediaSession::refresh(ui, id));
            }
            _ => {}
        }
    }

    fn notification(&mut self, freed: ComponentId) {
        for link in [&mut self.player, &mut self.playlist, &mut self.info] {
            if *link == Some(freed) {
                *link = None;
            }
        }
    }
}
