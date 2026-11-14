//! Os componentes: [`MediaPlayer`] (áudio, não visual) e [`VideoPlayer`] (vídeo).

use std::cell::RefCell;
use std::path::Path;
use std::sync::{Arc, Mutex};

use aylis_core::{
    Brush, Canvas, Color, Component, ComponentId, ControlBase, EventArgs, ImageFit, PaintCtx, Poster, RectF, TextAlign,
    Ui, properties,
};

use crate::audio::{AudioOpener, AudioPlayer, AudioSource, SymphoniaSource, open_audio_location};
use crate::output::{Tap, parse_outputs};
use crate::video::{DecodeThread, Frame, FrameSink, Frames, MediaSource};

/// Abre o áudio do aplicativo (pasta do projeto ou pacote) ou, se não houver, do disco, lido
/// aos poucos enquanto toca. Um endereço (`https://...`) abre na thread do áudio, lido pela
/// rede (W11): o erro chega pelo `on_error`.
fn open_audio(ui: &Ui, file: &str) -> Result<AudioOpener, String> {
    let ready = |s: SymphoniaSource| -> AudioOpener { Box::new(move || Ok(Box::new(s) as Box<dyn AudioSource>)) };
    if let Some(bytes) = ui.services().resources.as_ref().and_then(|r| r.read(file)) {
        return SymphoniaSource::open(bytes, Some(&extension(file))).map(ready);
    }
    if crate::video::is_url(Path::new(file)) {
        let location = file.to_string();
        return Ok(Box::new(move || open_audio_location(&location)));
    }
    SymphoniaSource::open_file(Path::new(file)).map(ready)
}

fn extension(file: &str) -> String {
    crate::video::file_extension(file)
}

/// Extensões que são só áudio: o `VideoPlayer` toca sem procurar imagem.
const AUDIO_ONLY: [&str; 9] = ["mp3", "wav", "flac", "ogg", "oga", "opus", "m4a", "aac", "wma"];

/// O que a abertura em segundo plano do [`VideoPlayer`] entrega.
struct Opened {
    decoder: Option<DecodeThread>,
    audio: Option<AudioPlayer>,
    audio_only: bool,
}

/// Roda `work` numa thread de fundo; no navegador (sem threads), na hora.
fn in_background(name: &str, work: impl FnOnce() + Send + 'static) -> Result<(), String> {
    #[cfg(not(target_arch = "wasm32"))]
    return std::thread::Builder::new().name(name.into()).spawn(work).map(|_| ()).map_err(|e| e.to_string());
    #[cfg(target_arch = "wasm32")]
    {
        let _ = name;
        work();
        Ok(())
    }
}

fn report(ui: &mut Ui, id: ComponentId, error: &str) {
    ui.fire(id, "on_error", &EventArgs::Text(error.to_string()));
}

// ---------------------------------------------------------------------------

/// Toca áudio: MP3, AAC/M4A, WAV, FLAC, OGG. Não visual.
///
/// ```no_run
/// # use aylis_core::Ui;
/// # let mut ui = Ui::new(); let player = ui.find("mpSom").unwrap();
/// aylis_media::MediaPlayer::play(&mut ui, player).unwrap();
/// ```
pub struct MediaPlayer {
    /// Arquivo do aplicativo (`assets/audio/aviso.mp3`) ou caminho no disco.
    pub file: String,
    /// O volume, de 0 (mudo) a 1 (o máximo).
    pub volume: f32,
    /// Volta ao começo ao terminar, sem parar.
    pub looping: bool,
    /// Começa a tocar quando o aplicativo abre.
    pub autoplay: bool,
    /// As saídas: vazio é a padrão; várias tocam o mesmo som sincronizado, cada uma com volume
    /// e atraso (`Alto-falantes|Fone BT;volume=0.8;delay=180`). Veja [`crate::AudioOutput`].
    pub output: String,
    player: Option<AudioPlayer>,
    loaded: String,
    /// O arquivo que está abrindo em segundo plano (a saída de áudio pode levar segundos).
    opening: Option<String>,
    /// O `play` chegou antes de abrir: toca quando ficar pronto.
    play_when_ready: bool,
}

impl Default for MediaPlayer {
    fn default() -> Self {
        Self {
            file: String::new(),
            volume: 1.0,
            looping: false,
            autoplay: false,
            output: String::new(),
            player: None,
            loaded: String::new(),
            opening: None,
            play_when_ready: false,
        }
    }
}

impl MediaPlayer {
    pub fn new(file: impl Into<String>) -> Self {
        Self { file: file.into(), ..Default::default() }
    }

    fn sync(&mut self, name: &str) {
        let Some(p) = &self.player else { return };
        match name {
            "volume" => p.set_volume(self.volume),
            "looping" => p.set_looping(self.looping),
            "output" => p.set_outputs(parse_outputs(&self.output)),
            "file" => self.player = None,
            _ => {}
        }
    }

    /// Abre o arquivo, se preciso, em segundo plano: a saída de áudio pode levar segundos para
    /// responder, e a tela esperava (o relatório do Tabernáculo, W06). `Ok(true)`: pronto;
    /// `Ok(false)`: abrindo (o `on_ready` avisa; o `on_error`, se falhar).
    fn ensure(ui: &mut Ui, id: ComponentId) -> Result<bool, String> {
        let (file, ready, output, opening) = match ui.get::<MediaPlayer>(id) {
            Some(m) => (m.file.clone(), m.player.is_some() && m.loaded == m.file, m.output.clone(), m.opening.clone()),
            None => return Err("não é um MediaPlayer".into()),
        };
        if ready {
            return Ok(true);
        }
        if file.is_empty() {
            return Err("informe o arquivo (file)".into());
        }
        if opening.as_deref() == Some(file.as_str()) {
            return Ok(false);
        }
        let source = open_audio(ui, &file)?;
        let poster = ui.poster();
        if let Some(m) = ui.get_mut::<MediaPlayer>(id) {
            m.opening = Some(file.clone());
        }
        in_background("aylis-audio-open", move || {
            let ended = poster.clone();
            let on_end: Box<dyn Fn() + Send> = Box::new(move || ended.post(move |ui| {
                ui.fire(id, "on_end", &EventArgs::None);
            }));
            let opened = AudioPlayer::open_on(source, Some(on_end), parse_outputs(&output));
            poster.post(move |ui| Self::opened(ui, id, file, opened));
        })?;
        Ok(false)
    }

    /// O arquivo abriu (ou falhou) em segundo plano.
    fn opened(ui: &mut Ui, id: ComponentId, file: String, opened: Result<AudioPlayer, String>) {
        let Some(m) = ui.get_mut::<MediaPlayer>(id) else { return };
        if m.opening.as_deref() != Some(file.as_str()) {
            return; // o arquivo mudou enquanto abria
        }
        m.opening = None;
        let play = std::mem::take(&mut m.play_when_ready);
        match opened {
            Ok(player) => {
                player.set_volume(m.volume);
                player.set_looping(m.looping);
                m.player = Some(player);
                m.loaded = file;
                ui.fire(id, "on_ready", &EventArgs::None);
                if play {
                    let _ = Self::play(ui, id);
                }
            }
            Err(e) => report(ui, id, &e),
        }
    }

    fn with(ui: &Ui, id: ComponentId) -> Option<&AudioPlayer> {
        ui.get::<MediaPlayer>(id)?.player.as_ref()
    }

    /// O que o player toca, para o [`AudioMixer`](crate::AudioMixer) (abre o arquivo, se
    /// preciso).
    pub(crate) fn tap(ui: &mut Ui, id: ComponentId) -> Option<Tap> {
        if !Self::is_ready(ui, id) {
            Self::open_now(ui, id).ok()?;
        }
        Self::with(ui, id).map(AudioPlayer::tap)
    }

    /// Abre o arquivo esperando (o [`AudioMixer`](crate::AudioMixer) precisa do som já aberto
    /// para ligar a fonte).
    fn open_now(ui: &mut Ui, id: ComponentId) -> Result<(), String> {
        let (file, output) = match ui.get::<MediaPlayer>(id) {
            Some(m) => (m.file.clone(), m.output.clone()),
            None => return Err("não é um MediaPlayer".into()),
        };
        if file.is_empty() {
            return Err("informe o arquivo (file)".into());
        }
        let source = open_audio(ui, &file)?;
        let poster = ui.poster();
        let on_end: Box<dyn Fn() + Send> = Box::new(move || poster.post(move |ui| {
            ui.fire(id, "on_end", &EventArgs::None);
        }));
        let player = AudioPlayer::open_on(source, Some(on_end), parse_outputs(&output))?;
        if let Some(m) = ui.get_mut::<MediaPlayer>(id) {
            player.set_volume(m.volume);
            player.set_looping(m.looping);
            m.player = Some(player);
            m.loaded = file;
            m.opening = None;
        }
        Ok(())
    }

    /// Toca (ou continua de onde parou). Abrindo o arquivo, toca quando ele ficar pronto (o
    /// `on_ready` vem antes). O erro também vai para o `on_error`.
    pub fn play(ui: &mut Ui, id: ComponentId) -> Result<(), String> {
        match Self::ensure(ui, id) {
            Ok(true) => {}
            Ok(false) => {
                if let Some(m) = ui.get_mut::<MediaPlayer>(id) {
                    m.play_when_ready = true;
                }
                return Ok(());
            }
            Err(e) => {
                report(ui, id, &e);
                return Err(e);
            }
        }
        if let Some(p) = Self::with(ui, id) {
            p.play();
        }
        Ok(())
    }

    /// O arquivo está aberto (depois do `on_ready`).
    pub fn is_ready(ui: &Ui, id: ComponentId) -> bool {
        Self::with(ui, id).is_some()
    }

    pub fn pause(ui: &mut Ui, id: ComponentId) {
        if let Some(m) = ui.get_mut::<MediaPlayer>(id) {
            m.play_when_ready = false;
        }
        if let Some(p) = Self::with(ui, id) {
            p.pause();
        }
    }

    /// Para e volta ao começo.
    pub fn stop(ui: &mut Ui, id: ComponentId) {
        if let Some(p) = Self::with(ui, id) {
            p.stop();
        }
    }

    pub fn seek(ui: &mut Ui, id: ComponentId, seconds: f64) {
        if let Some(p) = Self::with(ui, id) {
            p.seek(seconds);
        }
    }

    pub fn position(ui: &Ui, id: ComponentId) -> f64 {
        Self::with(ui, id).map_or(0.0, AudioPlayer::position)
    }

    pub fn duration(ui: &Ui, id: ComponentId) -> Option<f64> {
        Self::with(ui, id).and_then(AudioPlayer::duration)
    }

    pub fn is_playing(ui: &Ui, id: ComponentId) -> bool {
        Self::with(ui, id).is_some_and(AudioPlayer::is_playing)
    }
}

impl Component for MediaPlayer {
    fn class_name(&self) -> &'static str {
        "MediaPlayer"
    }

    properties! {
        file: String => "Behavior",
        volume: f32 => "Behavior",
        looping: bool => "Behavior",
        autoplay: bool => "Behavior",
        output: String => "Behavior";
        events: on_ready, on_end, on_error;
        on_set: sync
    }
}

// ---------------------------------------------------------------------------

#[derive(Default)]
struct Clock {
    playing: bool,
    /// Tempo do vídeo quando começou a tocar (ou onde parou).
    base: f64,
    /// Tempo da interface em que começou a tocar.
    started: f64,
    current: Option<Frame>,
    ended: bool,
    /// Erro do decodificador já avisado.
    error: Option<String>,
    quality: PlaybackQuality,
    /// A velocidade (1 = normal). Zero vale 1 (o `Default`).
    rate: f64,
}

/// Quantos quadros o [`VideoPlayer`] mostrou e quantos pulou por chegarem atrasados (como
/// o `getVideoPlaybackQuality` do navegador).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PlaybackQuality {
    pub shown: u64,
    pub dropped: u64,
}

impl Clock {
    fn at_rate(rate: f32) -> Self {
        Self { rate: rate as f64, ..Default::default() }
    }

    fn speed(&self) -> f64 {
        if self.rate > 0.0 { self.rate } else { 1.0 }
    }

    fn position(&self, now: f64) -> f64 {
        if self.playing { self.base + (now - self.started).max(0.0) * self.speed() } else { self.base }
    }

    /// O som acompanha só na velocidade normal; fora dela fica parado.
    fn audio_follows(&self) -> bool {
        (self.speed() - 1.0).abs() < 1e-6
    }
}

/// Mostra vídeo: **ele mesmo desenha cada
/// quadro**, então dá para ter cantos arredondados, controles por cima e animação.
///
/// O arquivo é aberto pelo primeiro decodificador que aceitar (os registrados com
/// [`register_decoder`](crate::register_decoder), depois o do sistema). Para mostrar
/// quadros de outra origem (câmera, stream, codec próprio), use [`VideoPlayer::sink`].
///
/// **Só áudio** (MP3, WAV, FLAC, OGG, M4A... ou um arquivo sem imagem): toca o som como o
/// [`MediaPlayer`], com posição, duração, pausa, busca, `looping` e `on_end` iguais, e
/// mostra uma nota musical com o nome do arquivo no lugar da imagem.
pub struct VideoPlayer {
    pub base: ControlBase,
    /// Arquivo do aplicativo (`assets/video/abertura.mp4`) ou caminho no disco.
    pub file: String,
    /// Começa a tocar assim que o arquivo abre.
    pub autoplay: bool,
    /// Volta ao começo ao terminar, sem parar.
    pub looping: bool,
    /// O volume, de 0 (mudo) a 1 (o máximo).
    pub volume: f32,
    /// Sem som, sem perder o `volume`.
    pub muted: bool,
    /// Como o quadro ocupa o controle: `contain` (inteiro), `cover` (cobre, cortando as
    /// bordas), `stretch`, `center` ou `original`.
    pub fit: ImageFit,
    /// O raio dos cantos arredondados, em pontos (0: retos).
    pub radius: f32,
    /// A velocidade: 1 é a normal, 2 o dobro, 0.5 a metade. Fora de 1 o som fica mudo
    /// (volta no lugar certo em 1). Só áudio e o navegador tocam sempre em 1.
    pub rate: f32,
    /// As saídas do som (como no [`MediaPlayer`]). No navegador, o `<video>` toca na saída
    /// padrão.
    pub output: String,
    /// Pinta o quadro no próprio lugar (o padrão). Desligado, o player só decodifica e segue
    /// o relógio: o app pinta o [`current_frame`](VideoPlayer::current_frame) onde quiser, na
    /// ordem da cena (a letra por cima do vídeo), avisado por `on_frame`. O relatório do
    /// Tabernáculo, W12.
    pub show_video: bool,
    /// Quantos quadros o decodificador prepara à frente do relógio (0: o padrão, 6; W13).
    pub frames_ahead: u32,
    frames: Arc<Mutex<Frames>>,
    decoder: Option<DecodeThread>,
    audio: Option<AudioPlayer>,
    /// Tocando só o som (o arquivo não tem imagem): o tempo vem do áudio.
    audio_only: bool,
    loaded: String,
    clock: RefCell<Clock>,
    poster: Option<Poster>,
    /// O arquivo que está abrindo em segundo plano.
    opening: Option<String>,
    /// O `play` chegou antes de abrir: toca quando ficar pronto.
    play_when_ready: bool,
    /// O id deste componente (guardado na pintura, para os avisos da thread).
    id_hint: std::cell::Cell<Option<ComponentId>>,
    /// O tempo do último quadro avisado em `on_frame`.
    last_announced: std::cell::Cell<Option<f64>>,
    /// Navegador: o `<video>` que toca (no lugar do decodificador e do áudio).
    #[cfg(target_arch = "wasm32")]
    web: Option<crate::web::WebVideo>,
}

impl Default for VideoPlayer {
    fn default() -> Self {
        Self {
            base: ControlBase::new(320.0, 180.0),
            file: String::new(),
            autoplay: false,
            looping: false,
            volume: 1.0,
            muted: false,
            fit: ImageFit::Contain,
            radius: 0.0,
            rate: 1.0,
            output: String::new(),
            show_video: true,
            frames_ahead: 0,
            frames: Arc::default(),
            decoder: None,
            audio: None,
            audio_only: false,
            loaded: String::new(),
            clock: RefCell::default(),
            poster: None,
            opening: None,
            play_when_ready: false,
            id_hint: std::cell::Cell::new(None),
            last_announced: std::cell::Cell::new(None),
            #[cfg(target_arch = "wasm32")]
            web: None,
        }
    }
}

impl VideoPlayer {
    pub fn new(file: impl Into<String>) -> Self {
        Self { file: file.into(), ..Default::default() }
    }

    /// O som do vídeo, para o [`AudioMixer`](crate::AudioMixer) (depois de abrir o arquivo).
    pub(crate) fn tap(ui: &Ui, id: ComponentId) -> Option<Tap> {
        ui.get::<VideoPlayer>(id)?.audio.as_ref().map(AudioPlayer::tap)
    }

    fn sync(&mut self, name: &str) {
        match name {
            "volume" | "muted" => {
                if let Some(a) = &self.audio {
                    a.set_volume(if self.muted { 0.0 } else { self.volume });
                }
                #[cfg(target_arch = "wasm32")]
                if let Some(w) = &self.web {
                    w.set_volume(self.volume, self.muted);
                }
            }
            "output" => {
                if let Some(a) = &self.audio {
                    a.set_outputs(parse_outputs(&self.output));
                }
            }
            "frames_ahead" => {
                if let Ok(mut f) = self.frames.lock() {
                    f.ahead = self.frames_ahead.min(120) as usize;
                }
            }
            "looping" => {
                #[cfg(target_arch = "wasm32")]
                if let Some(w) = &self.web {
                    w.set_looping(self.looping);
                }
                if let Some(a) = self.audio.as_ref().filter(|_| self.audio_only) {
                    a.set_looping(self.looping);
                }
            }
            "file" => {
                #[cfg(target_arch = "wasm32")]
                {
                    self.web = None;
                }
                self.decoder = None;
                self.audio = None;
                self.audio_only = false;
                *self.clock.borrow_mut() = Clock::at_rate(self.rate);
            }
            _ => {}
        }
    }

    /// Abre o arquivo, se preciso. Fora do navegador, em segundo plano: o decodificador e a
    /// saída de áudio podem levar segundos, e a tela esperava (o relatório do Tabernáculo,
    /// W05). `Ok(true)`: pronto; `Ok(false)`: abrindo (o `on_ready` avisa).
    fn ensure(ui: &mut Ui, id: ComponentId) -> Result<bool, String> {
        let (file, ready, opening) = match ui.get::<VideoPlayer>(id) {
            Some(v) => (v.file.clone(), v.has_source() && v.loaded == v.file, v.opening.clone()),
            None => return Err("não é um VideoPlayer".into()),
        };
        if ready {
            return Ok(true);
        }
        if file.is_empty() {
            return Err("informe o arquivo (file)".into());
        }
        if opening.as_deref() == Some(file.as_str()) {
            return Ok(false);
        }
        // Do projeto (bytes) ou do disco (caminho: o decodificador lê aos poucos).
        let source = match ui.services().resources.as_ref().and_then(|r| r.read(&file)) {
            Some(bytes) => MediaSource::Bytes { data: Arc::new(bytes), extension: extension(&file) },
            None => MediaSource::File(file.clone().into()),
        };
        // Navegador: o `<video>` toca (registrar um decodificador continua valendo: ele é
        // tentado antes).
        #[cfg(target_arch = "wasm32")]
        if crate::video::registered_decoders() == 0 {
            let poster = ui.poster();
            let frames = ui.get::<VideoPlayer>(id).map(|v| v.frames.clone()).unwrap_or_default();
            let web = crate::web::WebVideo::open(&source, frames, poster.clone(), id)?;
            if let Some(v) = ui.get_mut::<VideoPlayer>(id) {
                web.set_volume(v.volume, v.muted);
                web.set_looping(v.looping);
                v.web = Some(web);
                v.loaded = file;
                v.poster = Some(poster);
                *v.clock.borrow_mut() = Clock::at_rate(v.rate);
            }
            ui.fire(id, "on_ready", &EventArgs::None);
            return Ok(true);
        }
        // Só áudio pela extensão, ou um arquivo em que o decodificador não achou imagem.
        let audio_only = AUDIO_ONLY.contains(&extension(&file).as_str());
        let poster = ui.poster();
        let (frames, outputs) = ui.get::<VideoPlayer>(id).map(|v| (v.frames.clone(), parse_outputs(&v.output))).unwrap_or_default();
        if let Some(v) = ui.get_mut::<VideoPlayer>(id) {
            v.opening = Some(file.clone());
        }
        in_background("aylis-video-open", move || {
                let opened = if audio_only {
                    Self::open_audio_only(&source, id, &poster, outputs)
                } else {
                    // O decodificador abre e vive na thread de vídeo.
                    match DecodeThread::open(source.clone(), frames, poster.clone()) {
                        Ok((thread, audio_source)) => {
                            let audio = audio_source.and_then(|s| AudioPlayer::open_on(s, None, outputs).ok());
                            Ok(Opened { decoder: Some(thread), audio, audio_only: false })
                        }
                        Err(video_error) => Self::open_audio_only(&source, id, &poster, outputs).map_err(|_| video_error),
                    }
                };
                let back = poster.clone();
                back.post(move |ui| Self::opened(ui, id, file, poster, opened));
        })?;
        Ok(false)
    }

    /// Abre o arquivo só com o som (o mesmo motor do [`MediaPlayer`]), numa thread de fundo.
    /// O fim do áudio para o relógio e dispara o `on_end`.
    fn open_audio_only(source: &MediaSource, id: ComponentId, poster: &Poster, outputs: Vec<crate::output::OutputSpec>) -> Result<Opened, String> {
        let ready = |s: SymphoniaSource| -> AudioOpener { Box::new(move || Ok(Box::new(s) as Box<dyn AudioSource>)) };
        let opener = match source {
            MediaSource::Bytes { data, extension } => SymphoniaSource::open(data.as_ref().clone(), Some(extension)).map(ready)?,
            // Um endereço: lido pela rede, na thread do áudio (W11).
            MediaSource::File(path) if crate::video::is_url(path) => {
                let location = path.to_string_lossy().into_owned();
                Box::new(move || open_audio_location(&location))
            }
            MediaSource::File(path) => SymphoniaSource::open_file(path).map(ready)?,
        };
        let ended = poster.clone();
        let on_end: Box<dyn Fn() + Send> = Box::new(move || {
            ended.post(move |ui| {
                if let Some(v) = ui.get::<VideoPlayer>(id) {
                    let mut clock = v.clock.borrow_mut();
                    clock.playing = false;
                    clock.ended = true;
                }
                ui.fire(id, "on_end", &EventArgs::None);
            })
        });
        let audio = AudioPlayer::open_on(opener, Some(on_end), outputs)?;
        Ok(Opened { decoder: None, audio: Some(audio), audio_only: true })
    }

    /// O arquivo abriu (ou falhou) em segundo plano: o player passa a tocar por ele.
    fn opened(ui: &mut Ui, id: ComponentId, file: String, poster: Poster, opened: Result<Opened, String>) {
        let Some(v) = ui.get_mut::<VideoPlayer>(id) else { return };
        if v.opening.as_deref() != Some(file.as_str()) {
            return; // o arquivo mudou enquanto abria
        }
        v.opening = None;
        let play = std::mem::take(&mut v.play_when_ready);
        match opened {
            Ok(o) => {
                if let Some(a) = &o.audio {
                    a.set_volume(if v.muted { 0.0 } else { v.volume });
                    if o.audio_only {
                        a.set_looping(v.looping);
                    }
                }
                v.decoder = o.decoder;
                v.audio = o.audio;
                v.audio_only = o.audio_only;
                v.loaded = file;
                v.poster = Some(poster);
                *v.clock.borrow_mut() = Clock::at_rate(v.rate);
                ui.fire(id, "on_ready", &EventArgs::None);
                if play {
                    let _ = Self::play(ui, id);
                }
                ui.request_repaint();
            }
            Err(e) => report(ui, id, &e),
        }
    }

    /// O arquivo está aberto (depois do `on_ready`).
    pub fn is_ready(ui: &Ui, id: ComponentId) -> bool {
        ui.get::<VideoPlayer>(id).is_some_and(|v| v.has_source() && v.loaded == v.file)
    }

    /// Toca (ou continua de onde parou). Abrindo o arquivo, toca quando ele ficar pronto (o
    /// `on_ready` vem antes). O erro também vai para o `on_error`.
    pub fn play(ui: &mut Ui, id: ComponentId) -> Result<(), String> {
        match Self::ensure(ui, id) {
            Ok(true) => {}
            Ok(false) => {
                if let Some(v) = ui.get_mut::<VideoPlayer>(id) {
                    v.play_when_ready = true;
                }
                return Ok(());
            }
            Err(e) => {
                report(ui, id, &e);
                return Err(e);
            }
        }
        let now = ui.time();
        #[cfg(target_arch = "wasm32")]
        if let Some(w) = ui.get::<VideoPlayer>(id).and_then(|v| v.web.as_ref()) {
            w.play();
            return Ok(());
        }
        if let Some(v) = ui.get::<VideoPlayer>(id) {
            let mut clock = v.clock.borrow_mut();
            if clock.ended {
                clock.ended = false;
                clock.base = 0.0;
                clock.current = None;
                if let Some(d) = &v.decoder {
                    d.seek(0.0);
                }
                if let Some(a) = &v.audio {
                    a.seek(0.0);
                }
            }
            clock.playing = true;
            clock.started = now;
            if let Some(a) = v.audio.as_ref().filter(|_| v.audio_only || clock.audio_follows()) {
                a.play();
            }
        }
        ui.request_repaint();
        Ok(())
    }

    pub fn pause(ui: &mut Ui, id: ComponentId) {
        if let Some(v) = ui.get_mut::<VideoPlayer>(id) {
            v.play_when_ready = false;
        }
        #[cfg(target_arch = "wasm32")]
        if let Some(w) = ui.get::<VideoPlayer>(id).and_then(|v| v.web.as_ref()) {
            w.pause();
            return;
        }
        let now = ui.time();
        if let Some(v) = ui.get::<VideoPlayer>(id) {
            let mut clock = v.clock.borrow_mut();
            clock.base = clock.position(now);
            clock.playing = false;
            if let Some(a) = &v.audio {
                a.pause();
            }
        }
    }

    /// Para e volta ao começo.
    pub fn stop(ui: &mut Ui, id: ComponentId) {
        Self::pause(ui, id);
        Self::seek(ui, id, 0.0);
    }

    pub fn seek(ui: &mut Ui, id: ComponentId, seconds: f64) {
        #[cfg(target_arch = "wasm32")]
        if let Some(w) = ui.get::<VideoPlayer>(id).and_then(|v| v.web.as_ref()) {
            w.seek(seconds);
            return;
        }
        let now = ui.time();
        if let Some(v) = ui.get::<VideoPlayer>(id) {
            let mut clock = v.clock.borrow_mut();
            clock.base = seconds.max(0.0);
            clock.started = now;
            clock.current = None;
            clock.ended = false;
            if let Some(d) = &v.decoder {
                d.seek(seconds);
            }
            if let Some(a) = &v.audio {
                a.seek(seconds);
            }
        }
        ui.request_repaint();
    }

    pub fn position(ui: &Ui, id: ComponentId) -> f64 {
        #[cfg(target_arch = "wasm32")]
        if let Some(w) = ui.get::<VideoPlayer>(id).and_then(|v| v.web.as_ref()) {
            return w.position();
        }
        match ui.get::<VideoPlayer>(id) {
            // Só áudio: o tempo é o do som.
            Some(v) if v.audio_only => v.audio.as_ref().map_or(0.0, AudioPlayer::position),
            Some(v) => v.clock.borrow().position(ui.time()),
            None => 0.0,
        }
    }

    /// O quadro que o relógio mostra agora, com os pixels (para pintar o vídeo na ordem da
    /// cena, com `show_video` desligado; W12). Pedir os pixels faz o decodificador voltar a
    /// entregá-los na CPU; para pintar sem a ida e volta pela CPU, use
    /// [`current_video_frame`](Self::current_video_frame) e [`Frame::draw`](crate::Frame::draw).
    pub fn current_frame(ui: &Ui, id: ComponentId) -> Option<aylis_core::Image> {
        let v = ui.get::<VideoPlayer>(id)?;
        if let Ok(mut f) = v.frames.lock() {
            f.cpu_only = true;
        }
        v.clock.borrow().current.as_ref().and_then(Frame::to_image)
    }

    /// O quadro que o relógio mostra agora, como está: na placa de vídeo (o Direct2D compõe a
    /// textura direto) ou com os pixels. Pinte com [`Frame::draw`](crate::Frame::draw), que
    /// escolhe o caminho pelo canvas (W15).
    pub fn current_video_frame(ui: &Ui, id: ComponentId) -> Option<Frame> {
        ui.get::<VideoPlayer>(id)?.clock.borrow().current.clone()
    }

    /// O relógio do player (W13): a posição, se está tocando e a velocidade.
    pub fn clock_state(ui: &Ui, id: ComponentId) -> Option<(f64, bool, f64)> {
        let v = ui.get::<VideoPlayer>(id)?;
        let clock = v.clock.borrow();
        Some((Self::position(ui, id), clock.playing, clock.speed()))
    }

    /// Quantos quadros já decodificados esperam a vez (W13).
    pub fn queued_frames(ui: &Ui, id: ComponentId) -> usize {
        ui.get::<VideoPlayer>(id).and_then(|v| v.frames.lock().ok().map(|f| f.queue.len())).unwrap_or(0)
    }

    pub fn duration(ui: &Ui, id: ComponentId) -> Option<f64> {
        let v = ui.get::<VideoPlayer>(id)?;
        if v.audio_only {
            return v.audio.as_ref().and_then(AudioPlayer::duration);
        }
        v.frames.lock().ok().and_then(|f| f.info.duration)
    }

    /// Está tocando um arquivo sem imagem (só o som).
    pub fn is_audio_only(ui: &Ui, id: ComponentId) -> bool {
        ui.get::<VideoPlayer>(id).is_some_and(|v| v.audio_only)
    }

    /// Largura e altura do vídeo (depois que o primeiro quadro chegou).
    pub fn video_size(ui: &Ui, id: ComponentId) -> Option<(u32, u32)> {
        let v = ui.get::<VideoPlayer>(id)?;
        let f = v.frames.lock().ok()?;
        (f.info.width > 0).then_some((f.info.width, f.info.height))
    }

    /// Quadros mostrados e pulados desde que o arquivo abriu.
    pub fn playback_quality(ui: &Ui, id: ComponentId) -> PlaybackQuality {
        ui.get::<VideoPlayer>(id).map_or_else(PlaybackQuality::default, |v| v.clock.borrow().quality)
    }

    pub fn is_playing(ui: &Ui, id: ComponentId) -> bool {
        #[cfg(target_arch = "wasm32")]
        if let Some(w) = ui.get::<VideoPlayer>(id).and_then(|v| v.web.as_ref()) {
            return w.playing();
        }
        ui.get::<VideoPlayer>(id).is_some_and(|v| v.clock.borrow().playing)
    }

    /// Já tem de onde tirar os quadros (decodificador ou, no navegador, o `<video>`).
    fn has_source(&self) -> bool {
        #[cfg(target_arch = "wasm32")]
        if self.web.is_some() {
            return true;
        }
        self.decoder.is_some() || (self.audio_only && self.audio.is_some())
    }

    /// Para entregar quadros prontos de qualquer thread (o player mostra o último).
    pub fn sink(ui: &Ui, id: ComponentId) -> Option<FrameSink> {
        let v = ui.get::<VideoPlayer>(id)?;
        Some(FrameSink { frames: v.frames.clone(), poster: ui.poster(), id })
    }

    /// Escolhe o quadro da hora `now` e devolve quando pedir o próximo (tempo da
    /// interface), se estiver tocando.
    fn advance(&self, now: f64) -> Option<f64> {
        if self.audio_only {
            // Sem quadros: o fim chega pelo áudio (on_end); nada a desenhar a cada quadro.
            return None;
        }
        let mut clock = self.clock.borrow_mut();
        let Ok(mut f) = self.frames.lock() else { return None };
        if f.live {
            clock.current = f.queue.back().cloned();
            return None;
        }
        let pos = clock.position(now);
        let mut taken = 0;
        while let Some(front) = f.queue.front() {
            if front.time <= pos || clock.current.is_none() {
                clock.current = f.queue.pop_front();
                taken += 1;
            } else {
                break;
            }
        }
        if taken > 0 {
            clock.quality.shown += 1;
            clock.quality.dropped += taken - 1;
        }
        if !clock.playing {
            return None;
        }
        // O decodificador vê onde o relógio está (para não converter quadros atrasados).
        f.target = pos;
        if let Some(next) = f.queue.front() {
            return Some(clock.started + (next.time - clock.base) / clock.speed());
        }
        if let Some(e) = f.error.clone().filter(|_| clock.error.is_none()) {
            clock.error = Some(e.clone());
            clock.playing = false;
            if let Some(p) = &self.poster {
                let id = self.id_hint.get();
                p.post(move |ui| {
                    if let Some(id) = id {
                        ui.fire(id, "on_error", &EventArgs::Text(e));
                    }
                });
            }
            return None;
        }
        if f.eof {
            if self.looping {
                drop(f);
                clock.base = 0.0;
                clock.started = now;
                if let Some(d) = &self.decoder {
                    d.seek(0.0);
                }
                if let Some(a) = &self.audio {
                    a.seek(0.0);
                }
            } else {
                // No fim, a posição fica no fim (a duração, ou o último quadro), e não volta ao
                // último `play` (o relatório do Tabernáculo, W04).
                let end = f.info.duration.unwrap_or_else(|| clock.current.as_ref().map_or(pos, |c| c.time));
                clock.base = end.max(clock.current.as_ref().map_or(0.0, |c| c.time));
                clock.playing = false;
                clock.ended = true;
                if let Some(a) = &self.audio {
                    a.pause();
                }
                return None;
            }
        }
        // Esperando o decodificador.
        Some(now + 1.0 / 120.0)
    }
}

impl Component for VideoPlayer {
    fn class_name(&self) -> &'static str {
        "VideoPlayer"
    }

    fn control(&self) -> Option<&ControlBase> {
        Some(&self.base)
    }

    fn control_mut(&mut self) -> Option<&mut ControlBase> {
        Some(&mut self.base)
    }

    properties! {
        file: String => "Behavior",
        autoplay: bool => "Behavior",
        looping: bool => "Behavior",
        volume: f32 => "Behavior",
        muted: bool => "Behavior",
        fit: ImageFit => "Appearance",
        radius: f32 => "Appearance",
        rate: f32 => "Behavior",
        output: String => "Behavior",
        show_video: bool => "Appearance",
        frames_ahead: u32 => "Behavior";
        events: on_ready, on_end, on_error, on_frame;
        on_set: sync
    }

    /// Nova velocidade: o relógio continua de onde está; o som para fora de 1 e volta
    /// no lugar certo em 1.
    fn property_changed(&mut self, ctx: &mut aylis_core::EventCtx<'_>, name: &str) {
        if name != "rate" || self.audio_only {
            return;
        }
        let now = ctx.ui.time();
        let mut clock = self.clock.borrow_mut();
        clock.base = clock.position(now);
        clock.started = now;
        clock.rate = self.rate.max(0.0) as f64;
        if let Some(a) = &self.audio {
            if clock.audio_follows() {
                a.seek(clock.base);
                if clock.playing {
                    a.play();
                }
            } else {
                a.pause();
            }
        }
        drop(clock);
        ctx.repaint();
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        self.id_hint.set(Some(ctx.id));
        // O decodificador entrega no tamanho em que aparece (não o 4K inteiro num player
        // pequeno) e, se o canvas compõe texturas da placa, como textura (W15).
        if let Ok(mut f) = self.frames.lock() {
            let scale = canvas.device_scale().max(0.1);
            f.want = ((r.width * scale).round().max(0.0) as u32, (r.height * scale).round().max(0.0) as u32);
            f.gpu = canvas.gpu_device();
        }
        let was_playing = self.clock.borrow().playing;
        let next = self.advance(ctx.time);
        let ended_now = was_playing && self.clock.borrow().ended;
        if ended_now {
            if let Some(p) = &self.poster {
                let id = ctx.id;
                p.post(move |ui| {
                    ui.fire(id, "on_end", &EventArgs::None);
                });
            }
        }
        if let Some(at) = next {
            ctx.request_frame_at(at);
        }
        canvas.save();
        if self.radius > 0.0 {
            canvas.clip_round_rect(r, self.radius);
        } else {
            canvas.clip_rect(r);
        }
        // As barras em volta do vídeo: pretas, ou a cor própria (`fill`).
        canvas.fill_rect(r, 0.0, &Brush::Solid(ctx.colors.fill.unwrap_or(Color::BLACK)));
        let current = self.clock.borrow().current.clone();
        // Quadro novo: avisa (`on_frame`, com o tempo dele) quem pinta em outro lugar (W12).
        let shown = current.as_ref().map(|f| f.time);
        if shown.is_some() && shown != self.last_announced.get() {
            self.last_announced.set(shown);
            if let (Some(p), Some(t)) = (&self.poster, shown) {
                let id = ctx.id;
                p.post(move |ui| {
                    ui.fire(id, "on_frame", &EventArgs::Value(aylis_core::PropValue::Float(t)));
                });
            }
        }
        let current = current.filter(|_| self.show_video);
        match current {
            Some(frame) => match &frame.gpu {
                Some(gpu) => {
                    let (w, h) = gpu.size();
                    let (src, dest) = self.fit.place(aylis_core::SizeF::new(w as f32, h as f32), r);
                    // O canvas disse que compõe texturas e recusou esta: pixels daqui em diante.
                    if !gpu.draw(canvas, src, dest, 1.0) {
                        if canvas.gpu_device().is_some() {
                            if let Ok(mut f) = self.frames.lock() {
                                f.cpu_only = true;
                            }
                        }
                        if let Some(image) = gpu.to_image() {
                            canvas.draw_image(&image, src, dest, 1.0);
                        }
                    }
                }
                None => {
                    if let Some(size) = canvas.image_size(&frame.image) {
                        let (src, dest) = self.fit.place(size, r);
                        canvas.draw_image(&frame.image, src, dest, 1.0);
                    }
                }
            },
            None if self.clock.borrow().error.is_some() => {
                let mut style = ctx.text_style();
                style.color = ctx.colors.text.unwrap_or(Color::WHITE.with_opacity(0.85));
                style.h_align = TextAlign::Center;
                style.wrap = true;
                let text = format!("⚠ {}", self.clock.borrow().error.clone().unwrap_or_default());
                canvas.draw_text(&text, RectF::new(r.x + 12.0, r.y, r.width - 24.0, r.height), &style);
            }
            // Só áudio: a nota musical e o nome do arquivo no lugar da imagem.
            None if self.audio_only => {
                let muted = ctx.colors.text.unwrap_or(Color::WHITE.with_opacity(0.7));
                let side = (r.width.min(r.height) * 0.28).clamp(24.0, 120.0);
                let icon = RectF::new(r.x + (r.width - side) / 2.0, r.y + (r.height - side) / 2.0 - 14.0, side, side);
                ctx.draw_icon(canvas, "music", icon, muted);
                let mut style = ctx.text_style();
                style.color = muted;
                style.h_align = TextAlign::Center;
                let name = Path::new(&self.file).file_name().and_then(|n| n.to_str()).unwrap_or(&self.file).to_string();
                canvas.draw_text(&name, RectF::new(r.x + 8.0, icon.bottom() + 8.0, r.width - 16.0, 24.0), &style);
            }
            None if ctx.design => {
                // No designer: um quadro escuro com o nome do arquivo.
                let mut style = ctx.text_style();
                style.color = ctx.colors.text.unwrap_or(Color::WHITE.with_opacity(0.8));
                style.h_align = TextAlign::Center;
                let name = if self.file.is_empty() { "VideoPlayer".to_string() } else { format!("▶  {}", self.file) };
                canvas.draw_text(&name, RectF::new(r.x + 8.0, r.y, r.width - 16.0, r.height), &style);
            }
            None => {}
        }
        canvas.restore();
    }
}
