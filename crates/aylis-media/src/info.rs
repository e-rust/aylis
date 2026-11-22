//! `MediaInfo`: as tags de um arquivo de áudio ou vídeo (título, artista, álbum, faixa, ano), a
//! duração e a capa embutida, sem tocar. Decidido em 21/11 (o relatório de um player de música
//! portado para o Aylis): um componente próprio ligado a um provedor ("não denominar ao player
//! algo que pode ter vários provedores"), e o `MediaPlayer` só toca.
//!
//! Lê o `file` (um arquivo do projeto ou um caminho do disco), ou segue o `source` (um
//! `MediaPlayer`: a cada arquivo que ele abre, as tags dele). O `probe(caminho)` lê outro
//! arquivo em segundo plano, com o resultado no `on_probe` (uma playlist inteira, sem abrir um
//! player por música). Sem `provider`, o leitor padrão do Aylis (o Symphonia, que já toca o som);
//! o `LoftyTagsProvider` do `aylis-providers` lê mais formatos de tag.

use std::cell::RefCell;
use std::io::Cursor;
use std::path::Path;

use aylis_core::providers::{MediaInput, MediaTags, Provided, TagReader};
use aylis_core::{Component, ComponentId, EventArgs, EventCtx, MethodInfo, PropKind, PropValue, Ui, properties};
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::{MetadataOptions, MetadataRevision, StandardTagKey};
use symphonia::core::probe::Hint;

/// O registro das tags no acesso tipado (o que o `on_probe` leva).
pub const MEDIA_TAGS_RECORD: aylis_core::RecordInfo = aylis_core::RecordInfo {
    name: "MediaTagsInfo",
    fields: &[
        ("file", PropKind::String),
        ("title", PropKind::String),
        ("artist", PropKind::String),
        ("album", PropKind::String),
        ("track", PropKind::Int),
        ("year", PropKind::Int),
        ("duration", PropKind::Float),
        ("error", PropKind::String),
    ],
};

/// O resultado de um `probe`: o arquivo, as tags (sem a capa: o `cover_of` lê) e o erro, se
/// não deu para ler.
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct MediaTagsInfo {
    pub file: String,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub track: i64,
    pub year: i64,
    pub duration: f64,
    pub error: String,
}

impl aylis_core::EventValue for MediaTagsInfo {
    const ARG: aylis_core::EventArg = aylis_core::EventArg::Value(PropKind::Record(&MEDIA_TAGS_RECORD));
    fn from_args(args: &EventArgs) -> Option<Self> {
        let text = match args {
            EventArgs::Text(t) | EventArgs::Value(PropValue::String(t)) => t.as_str(),
            _ => return None,
        };
        serde_json::from_str(text).ok()
    }
}

/// O leitor padrão: o Symphonia (o mesmo que toca), pelas tags do contêiner e as de antes
/// dele (o ID3v2 do MP3), a primeira imagem embutida como capa e a duração da faixa de áudio.
pub fn read_tags(input: &MediaInput) -> Result<MediaTags, String> {
    let (stream, extension) = match input {
        MediaInput::Path(path) => {
            let file = std::fs::File::open(path).map_err(|e| format!("{path}: {e}"))?;
            let ext = Path::new(path).extension().map(|e| e.to_string_lossy().to_lowercase());
            (MediaSourceStream::new(Box::new(file), Default::default()), ext)
        }
        MediaInput::Bytes(bytes, ext) => (MediaSourceStream::new(Box::new(Cursor::new(bytes.clone())), Default::default()), Some(ext.to_lowercase())),
    };
    let mut hint = Hint::new();
    if let Some(ext) = extension.as_deref().filter(|e| !e.is_empty()) {
        hint.with_extension(ext);
    }
    let mut probed = symphonia::default::get_probe()
        .format(&hint, stream, &FormatOptions::default(), &MetadataOptions::default())
        .map_err(|e| format!("formato não reconhecido: {e}"))?;
    let mut tags = MediaTags::default();
    // As de antes do contêiner (o ID3v2) e depois as do contêiner: a primeira que tiver vale.
    if let Some(meta) = probed.metadata.get() {
        if let Some(rev) = meta.current() {
            fill(&mut tags, rev);
        }
    }
    if let Some(rev) = probed.format.metadata().current() {
        fill(&mut tags, rev);
    }
    if let Some(track) = probed.format.default_track() {
        let p = &track.codec_params;
        if let (Some(n), Some(rate)) = (p.n_frames, p.sample_rate) {
            tags.duration = n as f64 / rate.max(1) as f64;
        }
    }
    Ok(tags)
}

fn fill(tags: &mut MediaTags, rev: &MetadataRevision) {
    for tag in rev.tags() {
        let value = tag.value.to_string();
        let value = value.trim();
        if value.is_empty() {
            continue;
        }
        let number = || value.split(['/', '-', ' ']).next().and_then(|n| n.trim().parse::<u32>().ok()).unwrap_or(0);
        match tag.std_key {
            Some(StandardTagKey::TrackTitle) if tags.title.is_empty() => tags.title = value.to_string(),
            Some(StandardTagKey::Artist) if tags.artist.is_empty() => tags.artist = value.to_string(),
            Some(StandardTagKey::AlbumArtist) if tags.artist.is_empty() => tags.artist = value.to_string(),
            Some(StandardTagKey::Album) if tags.album.is_empty() => tags.album = value.to_string(),
            Some(StandardTagKey::TrackNumber) if tags.track == 0 => tags.track = number(),
            Some(StandardTagKey::Date | StandardTagKey::ReleaseDate | StandardTagKey::OriginalDate) if tags.year == 0 => tags.year = number(),
            _ => {}
        }
    }
    if tags.cover.is_none() {
        tags.cover = rev.visuals().first().map(|v| v.data.to_vec());
    }
}

/// O leitor padrão como [`TagReader`].
pub fn default_reader() -> TagReader {
    std::sync::Arc::new(read_tags)
}

/// As tags de um arquivo ou de um player, com provedor. Não visual.
#[derive(Default)]
pub struct MediaInfo {
    /// O arquivo: do projeto (`assets/audio/musica.mp3`) ou um caminho do disco.
    pub file: String,
    /// Um `MediaPlayer` a acompanhar: a cada arquivo que ele abre, as tags dele (no lugar de
    /// `file`).
    pub source: Option<ComponentId>,
    /// O provedor das tags (vazio: o Symphonia).
    pub provider: Option<ComponentId>,
    /// O título da música (ou do vídeo), das tags.
    pub title: String,
    /// O artista (ou o artista do álbum, sem o da faixa).
    pub artist: String,
    /// O álbum.
    pub album: String,
    /// O número da faixa no álbum (0: sem).
    pub track: i64,
    /// O ano (0: sem).
    pub year: i64,
    /// Em segundos.
    pub duration: f64,
    cover: RefCell<Option<Vec<u8>>>,
    /// O arquivo das tags de agora (para não ler duas vezes o mesmo).
    loaded: String,
}

impl MediaInfo {
    /// A capa embutida do arquivo lido (PNG ou JPEG, para o `Image::set_image_bytes`).
    pub fn cover(ui: &Ui, id: ComponentId) -> Option<Vec<u8>> {
        ui.get::<MediaInfo>(id)?.cover.borrow().clone()
    }

    /// Lê `path` em segundo plano; o resultado vem no `on_probe` (com o `file` dele).
    pub fn probe(ui: &mut Ui, id: ComponentId, path: &str) {
        let Some(reader) = reader(ui, id) else { return };
        let input = input_of(ui, path);
        let (poster, file) = (ui.poster(), path.to_string());
        run_in_background(move || {
            let info = match reader(&input) {
                Ok(t) => MediaTagsInfo { file, title: t.title, artist: t.artist, album: t.album, track: t.track as i64, year: t.year as i64, duration: t.duration, error: String::new() },
                Err(error) => MediaTagsInfo { file, error, ..Default::default() },
            };
            poster.post(move |ui| {
                ui.fire(id, "on_probe", &EventArgs::Text(serde_json::to_string(&info).unwrap_or_default()));
            });
        });
    }

    /// Lê o arquivo do `file` (ou o do player) e atualiza as propriedades, avisando no
    /// `on_loaded` (ou no `on_error`).
    fn load(ui: &mut Ui, id: ComponentId, file: String) {
        let Some(m) = ui.get_mut::<MediaInfo>(id) else { return };
        if file.is_empty() || m.loaded == file {
            return;
        }
        m.loaded = file.clone();
        let Some(reader) = reader(ui, id) else { return };
        let input = input_of(ui, &file);
        let poster = ui.poster();
        run_in_background(move || {
            let result = reader(&input);
            poster.post(move |ui| {
                let Some(m) = ui.get_mut::<MediaInfo>(id) else { return };
                if m.loaded != file {
                    return; // outro arquivo chegou enquanto lia
                }
                match result {
                    Ok(t) => {
                        (m.title, m.artist, m.album, m.track, m.year, m.duration) = (t.title, t.artist, t.album, t.track as i64, t.year as i64, t.duration);
                        *m.cover.borrow_mut() = t.cover;
                        let source = m.source;
                        ui.fire(id, "on_loaded", &EventArgs::None);
                        // As tags chegaram: os controles do sistema mostram o título e a capa.
                        if let Some(player) = source {
                            crate::session::MediaSession::player_changed(ui, player);
                        }
                    }
                    Err(e) => {
                        (m.title, m.artist, m.album, m.track, m.year, m.duration) = Default::default();
                        *m.cover.borrow_mut() = None;
                        ui.fire(id, "on_error", &EventArgs::Text(e));
                    }
                }
                ui.request_repaint();
            });
        });
    }

    /// Um `MediaPlayer` abriu `file`: os `MediaInfo` ligados a ele leem as tags.
    pub(crate) fn player_opened(ui: &mut Ui, player: ComponentId, file: &str) {
        let mut followers = Vec::new();
        let mut stack: Vec<ComponentId> = ui.roots().to_vec();
        while let Some(c) = stack.pop() {
            if ui.get::<MediaInfo>(c).is_some_and(|m| m.source == Some(player)) {
                followers.push(c);
            }
            stack.extend(ui.children(c).iter().copied());
        }
        for id in followers {
            Self::load(ui, id, file.to_string());
        }
    }
}

/// O leitor do provedor ligado, ou o padrão; no designer, nenhum (não lê arquivo nenhum).
fn reader(ui: &Ui, id: ComponentId) -> Option<TagReader> {
    if ui.design_mode() {
        return None;
    }
    let provider = ui.get::<MediaInfo>(id)?.provider;
    Some(match provider.and_then(|p| ui.provided(p)) {
        Some(Provided::MediaTags(p)) => p.tag_reader(),
        _ => default_reader(),
    })
}

/// Um arquivo do projeto vira bytes (o pacote embutido); o resto, o caminho do disco.
fn input_of(ui: &Ui, file: &str) -> MediaInput {
    match ui.services().resources.as_ref().and_then(|r| r.read(file)) {
        Some(bytes) => MediaInput::Bytes(bytes, crate::video::file_extension(file)),
        None => MediaInput::Path(file.to_string()),
    }
}

/// Em outra thread fora do navegador (lá, `std::thread` não roda: na hora).
fn run_in_background(work: impl FnOnce() + Send + 'static) {
    if cfg!(target_arch = "wasm32") {
        work();
    } else if std::thread::Builder::new().name("aylis-media-info".into()).spawn(work).is_err() {
        aylis_core::stderr_line!("aylis: MediaInfo: não consegui abrir a thread de leitura");
    }
}

impl Component for MediaInfo {
    fn class_name(&self) -> &'static str {
        "MediaInfo"
    }

    properties! {
        file: String => "Behavior",
        source: Option<ComponentId> => "Behavior",
        provider: Option<ComponentId> => "Behavior",
        title: String => "Data",
        artist: String => "Data",
        album: String => "Data",
        track: i64 => "Data",
        year: i64 => "Data",
        duration: f64 => "Data";
        events: on_loaded: (), on_probe: MediaTagsInfo, on_error: String
    }

    fn methods(&self) -> &'static [MethodInfo] {
        const METHODS: &[MethodInfo] = &[MethodInfo::new("probe", &[PropKind::String], None).params(&["path"]), MethodInfo::new("cover", &[], Some(PropKind::String))];
        METHODS
    }

    /// Pelo protocolo: o `cover` em base64 (o protocolo só leva texto).
    fn invoke(&mut self, ctx: &mut EventCtx<'_>, method: &str, args: &[PropValue]) -> Result<PropValue, String> {
        match method {
            "probe" => {
                let path = args.first().and_then(|v| v.to_str().ok()).ok_or("probe: informe o caminho")?.to_string();
                let id = ctx.id;
                ctx.ui.poster().post_to(id, move |ui| MediaInfo::probe(ui, id, &path));
                Ok(PropValue::Null)
            }
            "cover" => Ok(PropValue::String(self.cover.borrow().as_deref().map(aylis_core::image::encode_base64).unwrap_or_default())),
            _ => Err(format!("MediaInfo não tem o método '{method}'")),
        }
    }

    fn property_changed(&mut self, ctx: &mut EventCtx<'_>, name: &str) {
        if name == "file" && self.source.is_none() {
            let (id, file) = (ctx.id, self.file.clone());
            ctx.ui.poster().post_to(id, move |ui| MediaInfo::load(ui, id, file));
        }
    }

    fn notification(&mut self, freed: ComponentId) {
        if self.source == Some(freed) {
            self.source = None;
        }
        if self.provider == Some(freed) {
            self.provider = None;
        }
    }
}

/// O provedor padrão como componente (o Symphonia): o mesmo que vale sem provedor, para quem
/// quer deixar explícito no form.
#[derive(Default)]
pub struct SymphoniaTagsProvider;

impl Component for SymphoniaTagsProvider {
    fn class_name(&self) -> &'static str {
        "SymphoniaTagsProvider"
    }

    fn provided(&self) -> Option<Provided<'_>> {
        Some(Provided::MediaTags(self))
    }
}

impl aylis_core::providers::MediaTagsProvider for SymphoniaTagsProvider {
    fn tag_reader(&self) -> TagReader {
        default_reader()
    }
}
