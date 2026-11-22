//! O `LoftyTagsProvider`: as tags de mídia pelo lofty, que lê mais formatos de tag que o
//! Symphonia (ID3v1 e v2, APE, Vorbis, MP4, RIFF INFO) e as capas de todos. Para o
//! `MediaInfo` (o leitor padrão, sem provedor, é o Symphonia do `aylis-media`). Atrás da
//! feature `tags`: só o app que liga este provedor leva o lofty.

use aylis_core::providers::{MediaInput, MediaTags, MediaTagsProvider, Provided, TagReader};
use aylis_core::Component;
use lofty::prelude::*;

/// O provedor de tags pelo lofty. Não visual, sem propriedades: ligue no `provider` do
/// `MediaInfo`.
#[derive(Default)]
pub struct LoftyTagsProvider;

/// Lê as tags pelo lofty (o arquivo do disco ou os bytes do projeto).
pub fn read_tags(input: &MediaInput) -> Result<MediaTags, String> {
    let tagged = match input {
        MediaInput::Path(path) => lofty::read_from_path(path).map_err(|e| format!("{path}: {e}"))?,
        MediaInput::Bytes(bytes, _) => lofty::probe::Probe::new(std::io::Cursor::new(bytes.as_slice()))
            .guess_file_type()
            .map_err(|e| e.to_string())?
            .read()
            .map_err(|e| e.to_string())?,
    };
    let mut tags = MediaTags { duration: tagged.properties().duration().as_secs_f64(), ..Default::default() };
    if let Some(tag) = tagged.primary_tag().or_else(|| tagged.first_tag()) {
        tags.title = tag.title().map(|s| s.to_string()).unwrap_or_default();
        tags.artist = tag.artist().map(|s| s.to_string()).unwrap_or_default();
        tags.album = tag.album().map(|s| s.to_string()).unwrap_or_default();
        tags.track = tag.track().unwrap_or(0);
        tags.year = tag.date().map_or(0, |d| u32::from(d.year));
        tags.cover = tag.pictures().first().map(|p| p.data().to_vec());
    }
    Ok(tags)
}

impl Component for LoftyTagsProvider {
    fn class_name(&self) -> &'static str {
        "LoftyTagsProvider"
    }

    fn provided(&self) -> Option<Provided<'_>> {
        Some(Provided::MediaTags(self))
    }
}

impl MediaTagsProvider for LoftyTagsProvider {
    fn tag_reader(&self) -> TagReader {
        std::sync::Arc::new(read_tags)
    }
}
