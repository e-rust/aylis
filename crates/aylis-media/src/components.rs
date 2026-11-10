//! Os componentes: [`MediaPlayer`] (áudio, não visual) e [`VideoPlayer`] (vídeo).

use std::cell::RefCell;
use std::path::Path;
use std::sync::{Arc, Mutex};

use aylis_core::{
    Brush, Canvas, Color, Component, ComponentId, ControlBase, EventArgs, ImageFit, PaintCtx, Poster, RectF, TextAlign,
    Ui, properties,
};

use crate::audio::{AudioPlayer, AudioSource, SymphoniaSource};
use crate::output::{Tap, parse_outputs};
use crate::video::{DecodeThread, Frame, FrameSink, Frames, MediaSource};

/// Abre o áudio do aplicativo (pasta do projeto ou pacote) ou, se não houver, do disco, lido
/// aos poucos enquanto toca.
fn open_audio(ui: &Ui, file: &str) -> Result<SymphoniaSource, String> {
    match ui.services().resources.as_ref().and_then(|r| r.read(file)) {
        Some(bytes) => SymphoniaSource::open(bytes, Some(&extension(file))),
        None => SymphoniaSource::open_file(Path::new(file)),
    }
}

fn extension(file: &str) -> String {
    Path::new(file).extension().and_then(|e| e.to_str()).unwrap_or_default().to_lowercase()
}

/// Extensões que são só áudio: o `VideoPlayer` toca sem procurar imagem.
const AUDIO_ONLY: [&str; 9] = ["mp3", "wav", "flac", "ogg", "oga", "opus", "m4a", "aac", "wma"];

fn report(ui: &mut Ui, id: ComponentId, error: &str) {
    ui.fire(id, "on_error", &EventArgs::Text(error.to_string()));
}

// ---------------------------------------------------------------------------

/// Toca áudio (como o `TMediaPlayer`): MP3, AAC/M4A, WAV, FLAC, OGG. Não visual.
///
/// ```no_run
/// # use aylis_core::Ui;
/// # let mut ui = Ui::new(); let player = ui.find("mpSom").unwrap();
/// aylis_media::MediaPlayer::play(&mut ui, player).unwrap();
/// ```
pub struct MediaPlayer {
    /// Arquivo do aplicativo (`assets/audio/aviso.mp3`) ou caminho no disco.
    pub file: String,
    pub volume: f32,
    pub looping: bool,
    /// Começa a tocar quando o aplicativo abre.
    pub autoplay: bool,
    /// As saídas: vazio é a padrão; várias tocam o mesmo som sincronizado, cada uma com volume
    /// e atraso (`Alto-falantes|Fone BT;volume=0.8;delay=180`). Veja [`crate::AudioOutput`].
    pub output: String,
    player: Option<AudioPlayer>,
    loaded: String,
}

impl Default for MediaPlayer {
    fn default() -> Self {
        Self { file: String::new(), volume: 1.0, looping: false, autoplay: false, output: String::new(), player: None, loaded: String::new() }
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

    fn ensure(ui: &mut Ui, id: ComponentId) -> Result<(), String> {
        let (file, ready, output) = match ui.get::<MediaPlayer>(id) {
            Some(m) => (m.file.clone(), m.player.is_some() && m.loaded == m.file, m.output.clone()),
            None => return Err("não é um MediaPlayer".into()),
        };
        if ready {
            return Ok(());
        }
        if file.is_empty() {
            return Err("informe o arquivo (file)".into());
        }
        let source = open_audio(ui, &file)?;
        let poster = ui.poster();
        let on_end: Box<dyn Fn() + Send> = Box::new(move || poster.post(move |ui| {
            ui.fire(id, "on_end", &EventArgs::None);
        }));
        let player = AudioPlayer::open_on(Box::new(move || Ok(Box::new(source) as Box<dyn AudioSource>)), Some(on_end), parse_outputs(&output))?;
        if let Some(m) = ui.get_mut::<MediaPlayer>(id) {
            player.set_volume(m.volume);
            player.set_looping(m.looping);
            m.player = Some(player);
            m.loaded = file;
        }
        Ok(())
    }

    fn with(ui: &Ui, id: ComponentId) -> Option<&AudioPlayer> {
        ui.get::<MediaPlayer>(id)?.player.as_ref()
    }

    /// O que o player toca, para o [`AudioMixer`](crate::AudioMixer) (abre o arquivo, se
    /// preciso).
    pub(crate) fn tap(ui: &mut Ui, id: ComponentId) -> Option<Tap> {
        Self::ensure(ui, id).ok()?;
        Self::with(ui, id).map(AudioPlayer::tap)
    }

    /// Toca (ou continua de onde parou). O erro também vai para o `on_error`.
    pub fn play(ui: &mut Ui, id: ComponentId) -> Result<(), String> {
        if let Err(e) = Self::ensure(ui, id) {
            report(ui, id, &e);
            return Err(e);
        }
        if let Some(p) = Self::with(ui, id) {
            p.play();
        }
        Ok(())
    }

    pub fn pause(ui: &mut Ui, id: ComponentId) {
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
        events: on_end, on_error;
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

/// Mostra vídeo (como o `TMediaPlayer` + `TMediaPlayerControl`): **ele mesmo desenha cada
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
    pub autoplay: bool,
    pub looping: bool,
    pub volume: f32,
    pub muted: bool,
    pub fit: ImageFit,
    pub radius: f32,
    /// A velocidade: 1 é a normal, 2 o dobro, 0.5 a metade. Fora de 1 o som fica mudo
    /// (volta no lugar certo em 1). Só áudio e o navegador tocam sempre em 1.
    pub rate: f32,
    /// As saídas do som (como no [`MediaPlayer`]). No navegador, o `<video>` toca na saída
    /// padrão.
    pub output: String,
    frames: Arc<Mutex<Frames>>,
    decoder: Option<DecodeThread>,
    audio: Option<AudioPlayer>,
    /// Tocando só o som (o arquivo não tem imagem): o tempo vem do áudio.
    audio_only: bool,
    loaded: String,
    clock: RefCell<Clock>,
    poster: Option<Poster>,
    /// O id deste componente (guardado na pintura, para os avisos da thread).
    id_hint: std::cell::Cell<Option<ComponentId>>,
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
            frames: Arc::default(),
            decoder: None,
            audio: None,
            audio_only: false,
            loaded: String::new(),
            clock: RefCell::default(),
            poster: None,
            id_hint: std::cell::Cell::new(None),
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

    fn ensure(ui: &mut Ui, id: ComponentId) -> Result<(), String> {
        let (file, ready) = match ui.get::<VideoPlayer>(id) {
            Some(v) => (v.file.clone(), v.has_source() && v.loaded == v.file),
            None => return Err("não é um VideoPlayer".into()),
        };
        if ready {
            return Ok(());
        }
        if file.is_empty() {
            return Err("informe o arquivo (file)".into());
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
            return Ok(());
        }
        // Só áudio pela extensão, ou um arquivo em que o decodificador não achou imagem.
        if AUDIO_ONLY.contains(&extension(&file).as_str()) {
            return Self::open_audio_only(ui, id, &file);
        }
        // O decodificador abre e vive na thread de vídeo.
        let poster = ui.poster();
        let frames = ui.get::<VideoPlayer>(id).map(|v| v.frames.clone()).unwrap_or_default();
        let (thread, audio_source) = match DecodeThread::open(source, frames, poster.clone()) {
            Ok(opened) => opened,
            Err(video_error) => return Self::open_audio_only(ui, id, &file).map_err(|_| video_error),
        };
        let outputs = ui.get::<VideoPlayer>(id).map(|v| parse_outputs(&v.output)).unwrap_or_default();
        let audio = audio_source.and_then(|s| AudioPlayer::open_on(s, None, outputs).ok());
        if let Some(v) = ui.get_mut::<VideoPlayer>(id) {
            if let Some(a) = &audio {
                a.set_volume(if v.muted { 0.0 } else { v.volume });
            }
            v.decoder = Some(thread);
            v.audio = audio;
            v.loaded = file;
            v.poster = Some(poster);
            *v.clock.borrow_mut() = Clock::at_rate(v.rate);
        }
        Ok(())
    }

    /// Abre o arquivo só com o som (o mesmo motor do [`MediaPlayer`]). O fim do áudio
    /// para o relógio e dispara o `on_end`.
    fn open_audio_only(ui: &mut Ui, id: ComponentId, file: &str) -> Result<(), String> {
        let source = open_audio(ui, file)?;
        let poster = ui.poster();
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
        let outputs = ui.get::<VideoPlayer>(id).map(|v| parse_outputs(&v.output)).unwrap_or_default();
        let audio = AudioPlayer::open_on(Box::new(move || Ok(Box::new(source) as Box<dyn AudioSource>)), Some(on_end), outputs)?;
        if let Some(v) = ui.get_mut::<VideoPlayer>(id) {
            audio.set_volume(if v.muted { 0.0 } else { v.volume });
            audio.set_looping(v.looping);
            v.decoder = None;
            v.audio = Some(audio);
            v.audio_only = true;
            v.loaded = file.to_string();
            v.poster = Some(poster);
            *v.clock.borrow_mut() = Clock::at_rate(v.rate);
        }
        Ok(())
    }

    /// Toca (ou continua de onde parou). O erro também vai para o `on_error`.
    pub fn play(ui: &mut Ui, id: ComponentId) -> Result<(), String> {
        if let Err(e) = Self::ensure(ui, id) {
            report(ui, id, &e);
            return Err(e);
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
        Some(FrameSink { frames: v.frames.clone(), poster: ui.poster() })
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
        output: String => "Behavior";
        events: on_end, on_error;
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
        // O decodificador entrega no tamanho em que aparece (não o 4K inteiro num player pequeno).
        if let Ok(mut f) = self.frames.lock() {
            let scale = canvas.device_scale().max(0.1);
            f.want = ((r.width * scale).round().max(0.0) as u32, (r.height * scale).round().max(0.0) as u32);
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
        match current {
            Some(frame) => {
                if let Some(size) = canvas.image_size(&frame.image) {
                    let (src, dest) = self.fit.place(size, r);
                    canvas.draw_image(&frame.image, src, dest, 1.0);
                }
            }
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
