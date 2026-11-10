//! MediaCodec (Android): o decodificador de vídeo do sistema, pelo NDK (`AMediaExtractor`
//! separa as trilhas, `AMediaCodec` decodifica, com o hardware do aparelho quando houver).
//! O quadro sai em YUV 4:2:0 e vira RGBA aqui; a trilha de áudio vai pelo symphonia.
//!
//! As chaves do formato são escritas como texto ("width", "color-format"...): as
//! constantes `AMEDIAFORMAT_KEY_*` do NDK nem todas existem no Android 8 (API 26), e um
//! símbolo que falta impede a biblioteca do app de carregar.

use std::ffi::{CStr, CString};
use std::os::fd::AsRawFd;
use std::path::PathBuf;
use std::sync::Arc;

use ndk_sys as sys;

use crate::audio::{AudioOpener, AudioSource, SymphoniaSource};
use crate::video::{Frame, MediaSource, VideoDecoder, VideoDecoderFactory, VideoInfo};

/// A fábrica do decodificador do sistema no Android.
pub struct MediaCodec;

impl VideoDecoderFactory for MediaCodec {
    fn name(&self) -> &str {
        "MediaCodec"
    }

    fn open(&self, source: &MediaSource) -> Result<Box<dyn VideoDecoder>, String> {
        let path = to_path(source)?;
        if !path.exists() {
            return Err(aylis_core::messages::tf("arquivo não encontrado: {0}", &[&path.display()]));
        }
        let mut decoder = McVideo::open(path)?;
        decoder.audio_bytes = match source {
            MediaSource::Bytes { data, .. } => Some((data.clone(), source.extension())),
            MediaSource::File(_) => None,
        };
        Ok(Box::new(decoder))
    }
}

/// A pasta de cache do app (`/data/data/<pacote>/cache`): o nome do pacote é o nome do
/// processo. É onde um vídeo que veio em bytes (de dentro do app) vira arquivo.
fn cache_dir() -> PathBuf {
    let package = std::fs::read("/proc/self/cmdline")
        .ok()
        .map(|b| String::from_utf8_lossy(b.split(|&c| c == 0 || c == b':').next().unwrap_or_default()).to_string())
        .unwrap_or_default();
    let dir = PathBuf::from(format!("/data/data/{package}/cache"));
    if package.is_empty() || std::fs::create_dir_all(&dir).is_err() { std::env::temp_dir() } else { dir }
}

fn to_path(source: &MediaSource) -> Result<PathBuf, String> {
    match source {
        MediaSource::File(p) => Ok(p.clone()),
        MediaSource::Bytes { data, extension } => {
            let path = cache_dir().join(format!("aylis-media-{}.{extension}", Arc::as_ptr(data) as usize));
            if !path.exists() {
                std::fs::write(&path, data.as_slice()).map_err(|e| e.to_string())?;
            }
            Ok(path)
        }
    }
}

fn key(name: &str) -> CString {
    CString::new(name).expect("sem zero")
}

fn get_i32(format: *mut sys::AMediaFormat, name: &str) -> Option<i32> {
    let mut v = 0;
    // SAFETY: formato válido (o chamador garante) e chave terminada em zero.
    unsafe { sys::AMediaFormat_getInt32(format, key(name).as_ptr(), &mut v) }.then_some(v)
}

fn get_i64(format: *mut sys::AMediaFormat, name: &str) -> Option<i64> {
    let mut v = 0;
    // SAFETY: como em get_i32.
    unsafe { sys::AMediaFormat_getInt64(format, key(name).as_ptr(), &mut v) }.then_some(v)
}

fn get_string(format: *mut sys::AMediaFormat, name: &str) -> Option<String> {
    let mut out = std::ptr::null();
    // SAFETY: a string devolvida vale enquanto o formato existir; copiamos na hora.
    unsafe { sys::AMediaFormat_getString(format, key(name).as_ptr(), &mut out) && !out.is_null() }
        .then(|| unsafe { CStr::from_ptr(out) }.to_string_lossy().into_owned())
}

/// Como o quadro decodificado está arrumado na memória.
#[derive(Clone, Copy, Debug, Default)]
struct Layout {
    color: i32,
    stride: usize,
    slice: usize,
    crop: (usize, usize, usize, usize),
}

struct McVideo {
    path: PathBuf,
    /// O vídeo veio em bytes: o áudio decodifica deles (sem ler o arquivo de novo).
    audio_bytes: Option<(Arc<Vec<u8>>, String)>,
    _file: std::fs::File,
    extractor: *mut sys::AMediaExtractor,
    codec: *mut sys::AMediaCodec,
    info: VideoInfo,
    layout: Layout,
    input_done: bool,
    skip_until: Option<f64>,
    error: Option<String>,
}

impl McVideo {
    fn open(path: PathBuf) -> Result<Self, String> {
        let file = std::fs::File::open(&path).map_err(|e| e.to_string())?;
        let len = file.metadata().map_err(|e| e.to_string())?.len() as i64;
        // SAFETY: chamadas do NDK com ponteiros criados aqui; tudo é liberado no Drop (ou
        // aqui mesmo, se der erro).
        unsafe {
            let extractor = sys::AMediaExtractor_new();
            if extractor.is_null() {
                return Err("MediaCodec: extrator".into());
            }
            let mut me = Self {
                path,
                audio_bytes: None,
                _file: file,
                extractor,
                codec: std::ptr::null_mut(),
                info: VideoInfo::default(),
                layout: Layout::default(),
                input_done: false,
                skip_until: None,
                error: None,
            };
            if sys::AMediaExtractor_setDataSourceFd(extractor, me._file.as_raw_fd(), 0, len).0 != 0 {
                return Err("MediaCodec: formato de arquivo não reconhecido".into());
            }
            let tracks = sys::AMediaExtractor_getTrackCount(extractor);
            let mut chosen = None;
            for i in 0..tracks {
                let format = sys::AMediaExtractor_getTrackFormat(extractor, i);
                let mime = get_string(format, "mime").unwrap_or_default();
                if mime.starts_with("video/") && chosen.is_none() {
                    chosen = Some((i, format, mime));
                } else {
                    sys::AMediaFormat_delete(format);
                }
            }
            let Some((track, format, mime)) = chosen else {
                return Err(aylis_core::messages::t("o arquivo não tem vídeo"));
            };
            sys::AMediaExtractor_selectTrack(extractor, track);
            me.info.width = get_i32(format, "width").unwrap_or(0).max(0) as u32;
            me.info.height = get_i32(format, "height").unwrap_or(0).max(0) as u32;
            me.info.duration = get_i64(format, "durationUs").map(|d| d as f64 / 1_000_000.0);
            me.info.fps = get_i32(format, "frame-rate").map(f64::from);
            // Pede YUV 4:2:0 em memória (o padrão sem superfície); o formato de saída diz
            // qual veio.
            sys::AMediaFormat_setInt32(format, key("color-format").as_ptr(), 0x7F42_0888);
            let mime_c = CString::new(mime.clone()).map_err(|e| e.to_string())?;
            let codec = sys::AMediaCodec_createDecoderByType(mime_c.as_ptr());
            if codec.is_null() {
                sys::AMediaFormat_delete(format);
                return Err(aylis_core::messages::tf("formato de vídeo não suportado pelo sistema ({0})", &[&mime]));
            }
            me.codec = codec;
            let configured = sys::AMediaCodec_configure(codec, format, std::ptr::null_mut(), std::ptr::null_mut(), 0).0 == 0;
            sys::AMediaFormat_delete(format);
            if !configured || sys::AMediaCodec_start(codec).0 != 0 {
                return Err(aylis_core::messages::tf("formato de vídeo não suportado pelo sistema ({0})", &[&mime]));
            }
            me.layout = Layout::default();
            Ok(me)
        }
    }

    fn read_output_format(&mut self) {
        // SAFETY: codec válido; o formato devolvido é nosso e é liberado logo.
        unsafe {
            let format = sys::AMediaCodec_getOutputFormat(self.codec);
            if format.is_null() {
                return;
            }
            let width = get_i32(format, "width").unwrap_or(self.info.width as i32).max(0) as usize;
            let height = get_i32(format, "height").unwrap_or(self.info.height as i32).max(0) as usize;
            let stride = get_i32(format, "stride").filter(|&s| s > 0).map_or(width, |s| s as usize);
            let slice = get_i32(format, "slice-height").filter(|&s| s > 0).map_or(height, |s| s as usize);
            let left = get_i32(format, "crop-left").unwrap_or(0).max(0) as usize;
            let top = get_i32(format, "crop-top").unwrap_or(0).max(0) as usize;
            let right = get_i32(format, "crop-right").map_or(width.saturating_sub(1), |v| v.max(0) as usize);
            let bottom = get_i32(format, "crop-bottom").map_or(height.saturating_sub(1), |v| v.max(0) as usize);
            self.layout = Layout { color: get_i32(format, "color-format").unwrap_or(21), stride, slice: slice.max(height), crop: (left, top, right, bottom) };
            // O formato que o aparelho entrega (cada fabricante tem os seus): no logcat, para quem
            // investigar cores erradas.
            crate::android_camera::log(&format!("vídeo: color-format {:#x}, {width}x{height}, stride {stride}, slice {slice}", self.layout.color));
            self.info.width = (right + 1).saturating_sub(left) as u32;
            self.info.height = (bottom + 1).saturating_sub(top) as u32;
            sys::AMediaFormat_delete(format);
        }
    }

    /// Entrega ao codec o próximo pedaço do arquivo, se ele tiver lugar.
    fn feed(&mut self) {
        if self.input_done {
            return;
        }
        // SAFETY: índices e buffers devolvidos pelo próprio codec.
        unsafe {
            let idx = sys::AMediaCodec_dequeueInputBuffer(self.codec, 2_000);
            if idx < 0 {
                return;
            }
            let mut size = 0usize;
            let buf = sys::AMediaCodec_getInputBuffer(self.codec, idx as usize, &mut size);
            let n = if buf.is_null() { -1 } else { sys::AMediaExtractor_readSampleData(self.extractor, buf, size) };
            if n < 0 {
                sys::AMediaCodec_queueInputBuffer(self.codec, idx as usize, 0, 0, 0, sys::AMEDIACODEC_BUFFER_FLAG_END_OF_STREAM);
                self.input_done = true;
            } else {
                let t = sys::AMediaExtractor_getSampleTime(self.extractor).max(0) as u64;
                sys::AMediaCodec_queueInputBuffer(self.codec, idx as usize, 0, n as usize, t, 0);
                sys::AMediaExtractor_advance(self.extractor);
            }
        }
    }

    fn convert(&self, data: &[u8]) -> Option<Vec<u8>> {
        let l = self.layout;
        let (left, top, right, bottom) = l.crop;
        let (w, h) = ((right + 1).saturating_sub(left), (bottom + 1).saturating_sub(top));
        if w == 0 || h == 0 || l.stride == 0 {
            return None;
        }
        let y_size = l.stride * l.slice;
        // 19, 20 e o "flexível" (0x7F420888): I420, U e V em planos separados (o Codec2, do
        // Android 10 em diante, entrega o flexível assim no ByteBuffer). O resto é NV12, U e V
        // intercalados: 21, 39 e os da Qualcomm (0x7FA30C00, 0x7FA30C04 "32m", com a linha e a
        // altura alinhadas, que o `stride` e o `slice-height` já dizem), que o OMX do Android 9
        // entrega num Snapdragon mesmo pedindo o flexível.
        let planar = matches!(l.color, 19 | 20 | 0x7F42_0888);
        let chroma_stride = if planar { l.stride / 2 } else { l.stride };
        let u_plane = y_size;
        let v_plane = y_size + chroma_stride * (l.slice / 2);
        if data.len() < y_size {
            return None;
        }
        let mut out = vec![0u8; w * h * 4];
        for y in 0..h {
            let sy = y + top;
            let crow = sy / 2;
            for x in 0..w {
                let sx = x + left;
                let yy = *data.get(sy * l.stride + sx)? as f32;
                let (u, v) = if planar {
                    (*data.get(u_plane + crow * chroma_stride + sx / 2)?, *data.get(v_plane + crow * chroma_stride + sx / 2)?)
                } else {
                    let i = y_size + crow * l.stride + (sx & !1);
                    (*data.get(i)?, *data.get(i + 1)?)
                };
                // BT.601, faixa limitada (o padrão dos vídeos).
                let c = (yy - 16.0) * 1.164;
                let (d, e) = (u as f32 - 128.0, v as f32 - 128.0);
                let o = (y * w + x) * 4;
                out[o] = (c + 1.596 * e).clamp(0.0, 255.0) as u8;
                out[o + 1] = (c - 0.392 * d - 0.813 * e).clamp(0.0, 255.0) as u8;
                out[o + 2] = (c + 2.017 * d).clamp(0.0, 255.0) as u8;
                out[o + 3] = 255;
            }
        }
        Some(out)
    }
}

impl VideoDecoder for McVideo {
    fn info(&self) -> VideoInfo {
        self.info
    }

    fn seek(&mut self, seconds: f64) {
        // SAFETY: extrator e codec válidos.
        unsafe {
            sys::AMediaExtractor_seekTo(self.extractor, (seconds * 1_000_000.0) as i64, sys::SeekMode::AMEDIAEXTRACTOR_SEEK_PREVIOUS_SYNC);
            sys::AMediaCodec_flush(self.codec);
        }
        self.input_done = false;
        // O extrator volta ao quadro-chave anterior: os quadros antes do tempo pedido são
        // decodificados e descartados (busca precisa).
        self.skip_until = Some(seconds - 0.001);
    }

    fn next_frame(&mut self) -> Option<Frame> {
        if self.error.is_some() {
            return None;
        }
        let mut idle = 0;
        loop {
            self.feed();
            let mut info = sys::AMediaCodecBufferInfo { offset: 0, size: 0, presentationTimeUs: 0, flags: 0 };
            // SAFETY: codec válido; o buffer de saída vale até o release.
            let idx = unsafe { sys::AMediaCodec_dequeueOutputBuffer(self.codec, &mut info, 10_000) };
            if idx >= 0 {
                let idx = idx as usize;
                let eos = info.flags & sys::AMEDIACODEC_BUFFER_FLAG_END_OF_STREAM != 0;
                let time = info.presentationTimeUs as f64 / 1_000_000.0;
                let skip = self.skip_until.is_some_and(|t| time < t);
                let pixels = if info.size > 0 && !skip {
                    let mut size = 0usize;
                    // SAFETY: o buffer devolvido tem `size` bytes; lemos só a parte válida.
                    unsafe {
                        let buf = sys::AMediaCodec_getOutputBuffer(self.codec, idx, &mut size);
                        if buf.is_null() {
                            None
                        } else {
                            let start = (info.offset.max(0) as usize).min(size);
                            let end = (start + info.size.max(0) as usize).min(size);
                            self.convert(std::slice::from_raw_parts(buf.add(start), end - start))
                        }
                    }
                } else {
                    None
                };
                // SAFETY: devolve o buffer ao codec (sem superfície: não renderiza).
                unsafe { sys::AMediaCodec_releaseOutputBuffer(self.codec, idx, false) };
                if let Some(pixels) = pixels {
                    self.skip_until = None;
                    return Some(Frame::rgba(self.info.width, self.info.height, pixels, time));
                }
                if eos {
                    return None;
                }
                if !skip && info.size > 0 {
                    self.error = Some(format!("MediaCodec: formato de cor {:#x} não suportado", self.layout.color));
                    return None;
                }
                continue;
            }
            match idx as i32 {
                sys::AMEDIACODEC_INFO_OUTPUT_FORMAT_CHANGED => self.read_output_format(),
                sys::AMEDIACODEC_INFO_TRY_AGAIN_LATER => {
                    idle += 1;
                    // Sem nada na entrada nem na saída por ~3 s: o codec travou.
                    if self.input_done && idle > 300 {
                        return None;
                    }
                }
                _ => {}
            }
        }
    }

    fn audio(&mut self) -> Option<AudioOpener> {
        // Aberta na thread do áudio; do disco, lida aos poucos enquanto toca.
        let (bytes, path) = (self.audio_bytes.clone(), self.path.clone());
        Some(Box::new(move || {
            let source = match bytes {
                Some((data, ext)) => SymphoniaSource::open(data.as_ref().clone(), Some(&ext))?,
                None => SymphoniaSource::open_file(&path)?,
            };
            Ok(Box::new(source) as Box<dyn AudioSource>)
        }))
    }

    fn error(&self) -> Option<String> {
        self.error.clone()
    }
}

impl Drop for McVideo {
    fn drop(&mut self) {
        // SAFETY: liberados uma vez, na ordem certa (o codec antes do extrator).
        unsafe {
            if !self.codec.is_null() {
                sys::AMediaCodec_stop(self.codec);
                sys::AMediaCodec_delete(self.codec);
            }
            if !self.extractor.is_null() {
                sys::AMediaExtractor_delete(self.extractor);
            }
        }
    }
}
