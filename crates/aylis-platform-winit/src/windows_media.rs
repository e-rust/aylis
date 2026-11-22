//! Windows: os controles de mídia do sistema (o `SystemMediaTransportControls`, o SMTC) para o
//! `MediaSession`: o quadro do volume, a tela de bloqueio, as teclas de mídia e os botões do
//! fone. Um app desktop sem pacote liga o SMTC à janela do form pelo
//! `ISystemMediaTransportControlsInterop::GetForWindow`.

use aylis_core::ComponentId;
use aylis_core::platform::{MediaCommand, MediaCommandSink, MediaSessionService, NowPlaying};
use windows::Foundation::{TimeSpan, TypedEventHandler};
use windows::Media::{
    MediaPlaybackStatus, MediaPlaybackType, PlaybackPositionChangeRequestedEventArgs, SystemMediaTransportControls, SystemMediaTransportControlsButton,
    SystemMediaTransportControlsButtonPressedEventArgs, SystemMediaTransportControlsTimelineProperties,
};
use windows::Storage::Streams::{DataWriter, InMemoryRandomAccessStream, RandomAccessStreamReference};
use windows::Win32::Foundation::HWND;
use windows::Win32::System::WinRT::ISystemMediaTransportControlsInterop;
use windows::core::HSTRING;

#[derive(Default)]
pub(crate) struct WindowsMediaSession {
    smtc: Option<SystemMediaTransportControls>,
    /// A capa enviada por último (a mesma não é refeita a cada atualização).
    cover: Option<Vec<u8>>,
}

fn err(e: windows::core::Error) -> String {
    format!("SMTC: {e}")
}

/// Segundos para o `TimeSpan` do WinRT (em unidades de 100 ns).
fn span(seconds: f64) -> TimeSpan {
    TimeSpan { Duration: (seconds.max(0.0) * 10_000_000.0) as i64 }
}

impl MediaSessionService for WindowsMediaSession {
    fn start(&mut self, owner: Option<ComponentId>, commands: MediaCommandSink) -> Result<(), String> {
        let hwnd = owner.and_then(crate::win32::hwnd).ok_or("a janela do form ainda não existe")?;
        let interop: ISystemMediaTransportControlsInterop = windows::core::factory::<SystemMediaTransportControls, ISystemMediaTransportControlsInterop>().map_err(err)?;
        // SAFETY: `hwnd` é a janela do form, viva enquanto o form existe (o `win32::hwnd` só
        // devolve janelas abertas).
        let smtc: SystemMediaTransportControls = unsafe { interop.GetForWindow(HWND(hwnd as *mut core::ffi::c_void)) }.map_err(err)?;
        smtc.SetIsEnabled(true).map_err(err)?;
        smtc.SetIsPlayEnabled(true).map_err(err)?;
        smtc.SetIsPauseEnabled(true).map_err(err)?;
        smtc.SetIsStopEnabled(true).map_err(err)?;
        let sink = commands.clone();
        let pressed = TypedEventHandler::<SystemMediaTransportControls, SystemMediaTransportControlsButtonPressedEventArgs>::new(move |_, args| {
            let button = args.as_ref().and_then(|a| a.Button().ok());
            let cmd = match button {
                Some(SystemMediaTransportControlsButton::Play) => Some(MediaCommand::Play),
                Some(SystemMediaTransportControlsButton::Pause) => Some(MediaCommand::Pause),
                Some(SystemMediaTransportControlsButton::Stop) => Some(MediaCommand::Stop),
                Some(SystemMediaTransportControlsButton::Next) => Some(MediaCommand::Next),
                Some(SystemMediaTransportControlsButton::Previous) => Some(MediaCommand::Previous),
                _ => None,
            };
            if let Some(cmd) = cmd {
                sink(cmd);
            }
            Ok(())
        });
        smtc.ButtonPressed(&pressed).map_err(err)?;
        let seek = TypedEventHandler::<SystemMediaTransportControls, PlaybackPositionChangeRequestedEventArgs>::new(move |_, args| {
            if let Some(t) = args.as_ref().and_then(|a| a.RequestedPlaybackPosition().ok()) {
                commands(MediaCommand::Seek(t.Duration as f64 / 10_000_000.0));
            }
            Ok(())
        });
        smtc.PlaybackPositionChangeRequested(&seek).map_err(err)?;
        self.smtc = Some(smtc);
        self.cover = None;
        Ok(())
    }

    fn update(&mut self, now: &NowPlaying) {
        let Some(smtc) = &self.smtc else { return };
        let result = (|| -> windows::core::Result<()> {
            smtc.SetIsNextEnabled(now.can_next)?;
            smtc.SetIsPreviousEnabled(now.can_previous)?;
            smtc.SetPlaybackStatus(if now.playing { MediaPlaybackStatus::Playing } else { MediaPlaybackStatus::Paused })?;
            let display = smtc.DisplayUpdater()?;
            display.SetType(MediaPlaybackType::Music)?;
            let music = display.MusicProperties()?;
            music.SetTitle(&HSTRING::from(&now.title))?;
            music.SetArtist(&HSTRING::from(&now.artist))?;
            music.SetAlbumTitle(&HSTRING::from(&now.album))?;
            if now.cover != self.cover {
                match &now.cover {
                    Some(bytes) => {
                        let stream = InMemoryRandomAccessStream::new()?;
                        let writer = DataWriter::CreateDataWriter(&stream)?;
                        writer.WriteBytes(bytes)?;
                        writer.StoreAsync()?.join()?;
                        writer.DetachStream()?;
                        stream.Seek(0)?;
                        display.SetThumbnail(&RandomAccessStreamReference::CreateFromStream(&stream)?)?;
                    }
                    None => display.SetThumbnail(None)?,
                }
            }
            display.Update()?;
            if now.duration > 0.0 {
                let timeline = SystemMediaTransportControlsTimelineProperties::new()?;
                timeline.SetStartTime(span(0.0))?;
                timeline.SetMinSeekTime(span(0.0))?;
                timeline.SetEndTime(span(now.duration))?;
                timeline.SetMaxSeekTime(span(now.duration))?;
                timeline.SetPosition(span(now.position))?;
                smtc.UpdateTimelineProperties(&timeline)?;
            }
            Ok(())
        })();
        match result {
            Ok(()) => self.cover = now.cover.clone(),
            Err(e) => aylis_core::stderr_line!("aylis: {}", err(e)),
        }
    }

    fn stop(&mut self) {
        if let Some(smtc) = self.smtc.take() {
            let _ = smtc.SetIsEnabled(false);
            if let Ok(d) = smtc.DisplayUpdater() {
                let _ = d.ClearAll();
                let _ = d.Update();
            }
        }
    }
}
