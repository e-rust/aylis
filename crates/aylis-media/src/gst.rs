//! GStreamer (Linux): o decodificador de vídeo do sistema. A biblioteca é carregada na
//! hora (`libgstreamer-1.0.so.0`, `libgstapp-1.0.so.0`): o app compila e abre sem ela, e só
//! o vídeo avisa se o GStreamer não estiver instalado. Decodifica o que os plugins
//! instalados souberem (H.264, HEVC, VP9, AV1...), já convertido para RGBA:
//!
//! ```text
//! filesrc ! decodebin ! videoconvert ! video/x-raw,format=RGBA ! appsink
//! ```
//!
//! A trilha de áudio vai pelo symphonia, como nos outros sistemas.

use std::ffi::{CStr, CString, c_char, c_int, c_void};
use std::path::PathBuf;
use std::sync::{Arc, OnceLock};

use crate::audio::{AudioOpener, AudioSource, SymphoniaSource};
use crate::video::{Frame, MediaSource, VideoDecoder, VideoDecoderFactory, VideoInfo};

type Ptr = *mut c_void;

/// As funções do GStreamer que o aylis usa.
struct Gst {
    init: unsafe extern "C" fn(*mut c_int, *mut c_void),
    parse_launch: unsafe extern "C" fn(*const c_char, *mut Ptr) -> Ptr,
    bin_get_by_name: unsafe extern "C" fn(Ptr, *const c_char) -> Ptr,
    set_state: unsafe extern "C" fn(Ptr, c_int) -> c_int,
    get_state: unsafe extern "C" fn(Ptr, *mut c_int, *mut c_int, u64) -> c_int,
    query_duration: unsafe extern "C" fn(Ptr, c_int, *mut i64) -> c_int,
    seek_simple: unsafe extern "C" fn(Ptr, c_int, c_int, i64) -> c_int,
    object_unref: unsafe extern "C" fn(Ptr),
    mini_object_unref: unsafe extern "C" fn(Ptr),
    sample_get_buffer: unsafe extern "C" fn(Ptr) -> Ptr,
    sample_get_caps: unsafe extern "C" fn(Ptr) -> Ptr,
    caps_get_structure: unsafe extern "C" fn(Ptr, u32) -> Ptr,
    structure_get_int: unsafe extern "C" fn(Ptr, *const c_char, *mut c_int) -> c_int,
    structure_get_fraction: unsafe extern "C" fn(Ptr, *const c_char, *mut c_int, *mut c_int) -> c_int,
    buffer_map: unsafe extern "C" fn(Ptr, *mut MapInfo, c_int) -> c_int,
    buffer_unmap: unsafe extern "C" fn(Ptr, *mut MapInfo),
    try_pull_sample: unsafe extern "C" fn(Ptr, u64) -> Ptr,
    is_eos: unsafe extern "C" fn(Ptr) -> c_int,
    error_free: unsafe extern "C" fn(Ptr),
}

/// `GstMapInfo` (com folga no fim para os campos reservados).
#[repr(C)]
struct MapInfo {
    memory: Ptr,
    flags: c_int,
    data: *mut u8,
    size: usize,
    maxsize: usize,
    reserved: [Ptr; 8],
}

const STATE_NULL: c_int = 1;
const STATE_PAUSED: c_int = 3;
const STATE_PLAYING: c_int = 4;
const STATE_CHANGE_FAILURE: c_int = 0;
const FORMAT_TIME: c_int = 3;
const SEEK_FLUSH_ACCURATE: c_int = 1 | 2;
const MAP_READ: c_int = 1;
const SECOND: f64 = 1_000_000_000.0;

fn load() -> Result<&'static Gst, String> {
    static GST: OnceLock<Result<Gst, String>> = OnceLock::new();
    GST.get_or_init(|| unsafe {
        let open = |name: &str| {
            let c = CString::new(name).expect("sem zero");
            let h = libc::dlopen(c.as_ptr(), libc::RTLD_NOW | libc::RTLD_GLOBAL);
            if h.is_null() { Err(format!("GStreamer não instalado ({name})")) } else { Ok(h) }
        };
        let core = open("libgstreamer-1.0.so.0")?;
        let app = open("libgstapp-1.0.so.0")?;
        let glib = open("libglib-2.0.so.0")?;
        macro_rules! sym {
            ($lib:expr, $name:literal) => {{
                let c = CString::new($name).expect("sem zero");
                let p = libc::dlsym($lib, c.as_ptr());
                if p.is_null() {
                    return Err(format!("GStreamer: falta {}", $name));
                }
                std::mem::transmute::<*mut c_void, _>(p)
            }};
        }
        let gst = Gst {
            init: sym!(core, "gst_init"),
            parse_launch: sym!(core, "gst_parse_launch"),
            bin_get_by_name: sym!(core, "gst_bin_get_by_name"),
            set_state: sym!(core, "gst_element_set_state"),
            get_state: sym!(core, "gst_element_get_state"),
            query_duration: sym!(core, "gst_element_query_duration"),
            seek_simple: sym!(core, "gst_element_seek_simple"),
            object_unref: sym!(core, "gst_object_unref"),
            mini_object_unref: sym!(core, "gst_mini_object_unref"),
            sample_get_buffer: sym!(core, "gst_sample_get_buffer"),
            sample_get_caps: sym!(core, "gst_sample_get_caps"),
            caps_get_structure: sym!(core, "gst_caps_get_structure"),
            structure_get_int: sym!(core, "gst_structure_get_int"),
            structure_get_fraction: sym!(core, "gst_structure_get_fraction"),
            buffer_map: sym!(core, "gst_buffer_map"),
            buffer_unmap: sym!(core, "gst_buffer_unmap"),
            try_pull_sample: sym!(app, "gst_app_sink_try_pull_sample"),
            is_eos: sym!(app, "gst_app_sink_is_eos"),
            error_free: sym!(glib, "g_error_free"),
        };
        (gst.init)(std::ptr::null_mut(), std::ptr::null_mut());
        Ok(gst)
    })
    .as_ref()
    .map_err(Clone::clone)
}

/// Abre a câmera `device` (`/dev/video0`) como fonte ao vivo (sem áudio). Só o quadro
/// mais novo interessa: o appsink descarta os atrasados.
pub(crate) fn open_camera(device: &str, request: &crate::camera::CameraRequest) -> Result<Box<dyn VideoDecoder>, String> {
    let gst = load()?;
    let sizes = crate::camera::sort_sizes(v4l2_sizes(device));
    let source = format!("v4l2src device={}", quoted(std::path::Path::new(device)));
    let sink = "videoconvert ! video/x-raw,format=RGBA ! appsink name=sink sync=false max-buffers=1 drop=true";
    // O tamanho pedido, em vídeo cru ou em MJPEG (muitas webcams só dão os grandes em MJPEG);
    // se a câmera recusar, o tamanho padrão dela.
    let chosen = crate::camera::choose_size(&sizes, request).map(|(w, h)| {
        format!("{source} ! video/x-raw,width={w},height={h};image/jpeg,width={w},height={h} ! decodebin ! {sink}")
    });
    let default = format!("{source} ! {sink}");
    let mut video = match chosen.map(|d| GstVideo::open_pipeline(gst, &d, PathBuf::from(device), None)) {
        Some(Ok(v)) => v,
        _ => GstVideo::open_pipeline(gst, &default, PathBuf::from(device), None)?,
    };
    video.camera_caps = Some(crate::camera::CameraCaps { sizes, has_flash: false });
    Ok(Box::new(video))
}

/// Os tamanhos que a câmera V4L2 (`/dev/videoN`) entrega, em todos os formatos dela (as
/// mesmas chamadas que o `v4l2src` faz): `VIDIOC_ENUM_FMT` e `VIDIOC_ENUM_FRAMESIZES`.
fn v4l2_sizes(device: &str) -> Vec<(u32, u32)> {
    /// `struct v4l2_fmtdesc` (64 bytes).
    #[repr(C)]
    struct FmtDesc {
        index: u32,
        kind: u32,
        flags: u32,
        description: [u8; 32],
        pixel_format: u32,
        mbus_code: u32,
        reserved: [u32; 3],
    }
    /// `struct v4l2_frmsizeenum` (44 bytes): a união é `discrete` (largura, altura) ou
    /// `stepwise` (mínimo, máximo e passo de cada um).
    #[repr(C)]
    struct FrameSizes {
        index: u32,
        pixel_format: u32,
        kind: u32,
        size: [u32; 6],
        reserved: [u32; 2],
    }
    /// `_IOWR('V', 2, struct v4l2_fmtdesc)` e `_IOWR('V', 74, struct v4l2_frmsizeenum)`.
    const VIDIOC_ENUM_FMT: libc::c_ulong = 0xC040_5602;
    const VIDIOC_ENUM_FRAMESIZES: libc::c_ulong = 0xC02C_564A;
    const BUF_TYPE_VIDEO_CAPTURE: u32 = 1;
    const FRMSIZE_TYPE_DISCRETE: u32 = 1;
    let Ok(path) = CString::new(device) else { return Vec::new() };
    let mut sizes = Vec::new();
    // SAFETY: o arquivo é aberto e fechado aqui; as estruturas têm o layout do kernel.
    unsafe {
        let fd = libc::open(path.as_ptr(), libc::O_RDWR | libc::O_NONBLOCK);
        if fd < 0 {
            return sizes;
        }
        for fmt_index in 0..64 {
            let mut fmt: FmtDesc = std::mem::zeroed();
            fmt.index = fmt_index;
            fmt.kind = BUF_TYPE_VIDEO_CAPTURE;
            if libc::ioctl(fd, VIDIOC_ENUM_FMT as _, &mut fmt) != 0 {
                break;
            }
            for size_index in 0..256 {
                let mut fs: FrameSizes = std::mem::zeroed();
                fs.index = size_index;
                fs.pixel_format = fmt.pixel_format;
                if libc::ioctl(fd, VIDIOC_ENUM_FRAMESIZES as _, &mut fs) != 0 {
                    break;
                }
                if fs.kind == FRMSIZE_TYPE_DISCRETE {
                    sizes.push((fs.size[0], fs.size[1]));
                } else {
                    // Faixa contínua: o mínimo e o máximo.
                    sizes.push((fs.size[0], fs.size[3]));
                    sizes.push((fs.size[1], fs.size[4]));
                    break;
                }
            }
        }
        libc::close(fd);
    }
    sizes.retain(|&(w, h)| w > 0 && h > 0);
    sizes
}

/// A fábrica do decodificador do sistema no Linux.
pub struct GStreamer;

impl VideoDecoderFactory for GStreamer {
    fn name(&self) -> &str {
        "GStreamer"
    }

    fn open(&self, source: &MediaSource) -> Result<Box<dyn VideoDecoder>, String> {
        let gst = load()?;
        let path = match source {
            MediaSource::File(p) => p.clone(),
            MediaSource::Bytes { data, extension } => {
                let p = std::env::temp_dir().join(format!("aylis-media-{}-{}.{extension}", std::process::id(), Arc::as_ptr(data) as usize));
                if !p.exists() {
                    std::fs::write(&p, data.as_slice()).map_err(|e| e.to_string())?;
                }
                p
            }
        };
        if !crate::video::is_url(&path) && !path.exists() {
            return Err(aylis_core::messages::tf("arquivo não encontrado: {0}", &[&path.display()]));
        }
        let audio_bytes = match source {
            MediaSource::Bytes { data, .. } => Some((data.clone(), source.extension())),
            MediaSource::File(_) => None,
        };
        GstVideo::open(gst, path, audio_bytes).map(|d| Box::new(d) as Box<dyn VideoDecoder>)
    }
}

struct GstVideo {
    gst: &'static Gst,
    path: PathBuf,
    audio_bytes: Option<(Arc<Vec<u8>>, String)>,
    pipeline: Ptr,
    sink: Ptr,
    info: VideoInfo,
    error: Option<String>,
    /// Câmera: os tamanhos que ela entrega (lidos do V4L2).
    camera_caps: Option<crate::camera::CameraCaps>,
}

/// Escapa o caminho para a descrição do pipeline (entre aspas).
fn quoted(path: &std::path::Path) -> String {
    format!("\"{}\"", path.to_string_lossy().replace('\\', "\\\\").replace('"', "\\\""))
}

impl GstVideo {
    fn open(gst: &'static Gst, path: PathBuf, audio_bytes: Option<(Arc<Vec<u8>>, String)>) -> Result<Self, String> {
        // Endereço de rede: o `uridecodebin` busca e decodifica (W11).
        let source = if crate::video::is_url(&path) { format!("uridecodebin uri={}", quoted(&path)) } else { format!("filesrc location={} ! decodebin", quoted(&path)) };
        let description = format!("{source} ! videoconvert ! video/x-raw,format=RGBA ! appsink name=sink sync=false max-buffers=4");
        Self::open_pipeline(gst, &description, path, audio_bytes)
    }

    fn open_pipeline(gst: &'static Gst, description: &str, path: PathBuf, audio_bytes: Option<(Arc<Vec<u8>>, String)>) -> Result<Self, String> {
        let description = CString::new(description).map_err(|e| e.to_string())?;
        // SAFETY: chamadas do GStreamer com objetos criados aqui; liberados no Drop.
        unsafe {
            let mut err: Ptr = std::ptr::null_mut();
            let pipeline = (gst.parse_launch)(description.as_ptr(), &mut err);
            if !err.is_null() {
                // GError: { domain: u32, code: i32, message: *char }
                let message = *(err as *const u8).add(8).cast::<*const c_char>();
                let text = if message.is_null() { "GStreamer: pipeline".into() } else { CStr::from_ptr(message).to_string_lossy().into_owned() };
                (gst.error_free)(err);
                if pipeline.is_null() {
                    return Err(text);
                }
            }
            if pipeline.is_null() {
                return Err("GStreamer: pipeline".into());
            }
            let name = CString::new("sink").expect("sem zero");
            let sink = (gst.bin_get_by_name)(pipeline, name.as_ptr());
            let mut me = Self { gst, path, audio_bytes, pipeline, sink, info: VideoInfo::default(), error: None, camera_caps: None };
            if sink.is_null() {
                return Err("GStreamer: appsink".into());
            }
            // Pausado, o pipeline abre o arquivo e prepara o primeiro quadro (as medidas
            // vêm daí); tocando, entrega quadros sem relógio (sync=false), no ritmo em que
            // o player pede.
            if (gst.set_state)(pipeline, STATE_PAUSED) == STATE_CHANGE_FAILURE
                || (gst.get_state)(pipeline, std::ptr::null_mut(), std::ptr::null_mut(), 10 * SECOND as u64) == STATE_CHANGE_FAILURE
            {
                return Err(aylis_core::messages::tf("formato de vídeo não suportado pelo sistema ({0})", &[&"GStreamer"]));
            }
            let mut d = 0i64;
            if (gst.query_duration)(pipeline, FORMAT_TIME, &mut d) != 0 && d > 0 {
                me.info.duration = Some(d as f64 / SECOND);
            }
            (gst.set_state)(pipeline, STATE_PLAYING);
            Ok(me)
        }
    }
}

impl VideoDecoder for GstVideo {
    fn camera_caps(&self) -> Option<crate::camera::CameraCaps> {
        self.camera_caps.clone()
    }

    fn info(&self) -> VideoInfo {
        self.info
    }

    fn seek(&mut self, seconds: f64) {
        // SAFETY: pipeline válido.
        unsafe {
            (self.gst.seek_simple)(self.pipeline, FORMAT_TIME, SEEK_FLUSH_ACCURATE, (seconds.max(0.0) * SECOND) as i64);
        }
    }

    fn next_frame(&mut self) -> Option<Frame> {
        let gst = self.gst;
        // SAFETY: a amostra é nossa até o unref; o buffer vale enquanto ela existir.
        unsafe {
            loop {
                let sample = (gst.try_pull_sample)(self.sink, SECOND as u64 / 2);
                if sample.is_null() {
                    if (gst.is_eos)(self.sink) != 0 {
                        return None;
                    }
                    continue;
                }
                let caps = (gst.sample_get_caps)(sample);
                let s = if caps.is_null() { std::ptr::null_mut() } else { (gst.caps_get_structure)(caps, 0) };
                let (mut w, mut h, mut num, mut den) = (0, 0, 0, 1);
                if !s.is_null() {
                    let key = |k: &str| CString::new(k).expect("sem zero");
                    (gst.structure_get_int)(s, key("width").as_ptr(), &mut w);
                    (gst.structure_get_int)(s, key("height").as_ptr(), &mut h);
                    (gst.structure_get_fraction)(s, key("framerate").as_ptr(), &mut num, &mut den);
                }
                let buffer = (gst.sample_get_buffer)(sample);
                if buffer.is_null() || w <= 0 || h <= 0 {
                    (gst.mini_object_unref)(sample);
                    continue;
                }
                // GstBuffer: depois do GstMiniObject (64 bytes) e do pool, o pts em ns.
                let pts = *(buffer as *const u8).add(72).cast::<u64>();
                let mut map = MapInfo { memory: std::ptr::null_mut(), flags: 0, data: std::ptr::null_mut(), size: 0, maxsize: 0, reserved: [std::ptr::null_mut(); 8] };
                let mut pixels = None;
                if (gst.buffer_map)(buffer, &mut map, MAP_READ) != 0 {
                    let (w, h) = (w as usize, h as usize);
                    let data = std::slice::from_raw_parts(map.data, map.size);
                    // Linhas alinhadas em 4 bytes: em RGBA não há enchimento.
                    if data.len() >= w * h * 4 {
                        pixels = Some(data[..w * h * 4].to_vec());
                    }
                    (gst.buffer_unmap)(buffer, &mut map);
                }
                (gst.mini_object_unref)(sample);
                let Some(pixels) = pixels else { continue };
                self.info.width = w as u32;
                self.info.height = h as u32;
                if num > 0 && den > 0 {
                    self.info.fps = Some(num as f64 / den as f64);
                }
                let time = if pts == u64::MAX { 0.0 } else { pts as f64 / SECOND };
                return Some(Frame::rgba(w as u32, h as u32, pixels, time));
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

impl Drop for GstVideo {
    fn drop(&mut self) {
        // SAFETY: liberados uma vez; o pipeline vai para NULL antes (solta as threads dele).
        unsafe {
            (self.gst.set_state)(self.pipeline, STATE_NULL);
            if !self.sink.is_null() {
                (self.gst.object_unref)(self.sink);
            }
            (self.gst.object_unref)(self.pipeline);
        }
    }
}
