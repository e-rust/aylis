//! Media Foundation (Windows): o decodificador de vídeo padrão (MP4/H.264, e o que mais o
//! sistema souber: HEVC, VP9, WMV... conforme as extensões instaladas), a trilha de áudio
//! do vídeo e a gravação de MP4 H.264 ([`write_mp4`]).
//!
//! Os objetos do Media Foundation são criados na thread que vai usá-los (a de vídeo ou a
//! de áudio do player), na primeira chamada.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use windows::Win32::Foundation::HMODULE;
use windows::Win32::Graphics::Direct3D::D3D_DRIVER_TYPE_HARDWARE;
use windows::Win32::Graphics::Direct3D11::{
    D3D11_CREATE_DEVICE_BGRA_SUPPORT, D3D11_CREATE_DEVICE_VIDEO_SUPPORT, D3D11_SDK_VERSION, D3D11CreateDevice, ID3D11Multithread,
};
use windows::Win32::Media::MediaFoundation::*;
use windows::Win32::System::Com::StructuredStorage::PROPVARIANT;
use windows::Win32::System::Com::{COINIT_APARTMENTTHREADED, CoInitializeEx};
use windows::core::{GUID, HSTRING, Interface};

use crate::audio::{AudioOpener, AudioSource};
use crate::video::{Frame, MediaSource, VideoDecoder, VideoDecoderFactory, VideoInfo};

const VIDEO: u32 = MF_SOURCE_READER_FIRST_VIDEO_STREAM.0 as u32;
const AUDIO: u32 = MF_SOURCE_READER_FIRST_AUDIO_STREAM.0 as u32;
const ALL: u32 = MF_SOURCE_READER_ALL_STREAMS.0 as u32;

/// COM e Media Foundation nesta thread (pode ser chamada várias vezes).
fn init() {
    // SAFETY: inicialização de COM/MF sem parâmetros de saída; repetir é permitido.
    unsafe {
        // STA: compatível com a thread da janela (o winit chama OleInitialize, que exige
        // STA). Os objetos do Media Foundation são livres de thread, então tanto faz nas
        // threads de decodificação. Se a thread já for MTA, o erro é ignorado.
        let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
        let _ = MFStartup(MF_VERSION, MFSTARTUP_FULL);
    }
}

/// A placa de vídeo para o Media Foundation decodificar (e converter as cores) nela.
/// `None` sem placa com decodificação (máquina virtual, driver básico).
fn gpu_manager() -> Option<IMFDXGIDeviceManager> {
    // SAFETY: COM do Direct3D 11 e do Media Foundation, com objetos criados aqui.
    unsafe {
        let mut device = None;
        D3D11CreateDevice(
            None,
            D3D_DRIVER_TYPE_HARDWARE,
            HMODULE::default(),
            D3D11_CREATE_DEVICE_VIDEO_SUPPORT | D3D11_CREATE_DEVICE_BGRA_SUPPORT,
            None,
            D3D11_SDK_VERSION,
            Some(&mut device),
            None,
            None,
        )
        .ok()?;
        let device = device?;
        // O decodificador e o conversor do Media Foundation usam o dispositivo de outras threads.
        if let Ok(mt) = device.cast::<ID3D11Multithread>() {
            let _ = mt.SetMultithreadProtected(true);
        }
        let mut token = 0u32;
        let mut manager = None;
        MFCreateDXGIDeviceManager(&mut token, &mut manager).ok()?;
        let manager = manager?;
        manager.ResetDevice(&device, token).ok()?;
        Some(manager)
    }
}

fn err(e: windows::core::Error) -> String {
    format!("Media Foundation: {}", e.message())
}

/// Bytes viram um arquivo temporário (o leitor do Media Foundation abre por caminho).
fn to_path(source: &MediaSource) -> Result<PathBuf, String> {
    match source {
        MediaSource::File(p) => Ok(p.clone()),
        MediaSource::Bytes { data, extension } => {
            let path = std::env::temp_dir().join(format!("aylis-media-{}-{}.{extension}", std::process::id(), Arc::as_ptr(data) as usize));
            if !path.exists() {
                std::fs::write(&path, data.as_slice()).map_err(|e| e.to_string())?;
            }
            Ok(path)
        }
    }
}

/// As câmeras do sistema: (nome, identificador).
pub fn cameras() -> Vec<(String, String)> {
    init();
    let mut out = Vec::new();
    // SAFETY: a lista devolvida pelo sistema é lida e liberada aqui.
    unsafe {
        let mut attrs = None;
        if MFCreateAttributes(&mut attrs, 1).is_err() {
            return out;
        }
        let Some(attrs) = attrs else { return out };
        if attrs.SetGUID(&MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE, &MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_VIDCAP_GUID).is_err() {
            return out;
        }
        let mut list: *mut Option<IMFActivate> = std::ptr::null_mut();
        let mut count = 0u32;
        if MFEnumDeviceSources(&attrs, &mut list, &mut count).is_err() || list.is_null() {
            return out;
        }
        for i in 0..count as usize {
            if let Some(device) = (*list.add(i)).take() {
                let text = |key: &GUID| {
                    let mut ptr = windows::core::PWSTR::null();
                    let mut len = 0u32;
                    device.GetAllocatedString(key, &mut ptr, &mut len).ok().map(|_| {
                        let s = ptr.to_string().unwrap_or_default();
                        windows::Win32::System::Com::CoTaskMemFree(Some(ptr.0 as *const _));
                        s
                    })
                };
                if let (Some(name), Some(id)) = (text(&MF_DEVSOURCE_ATTRIBUTE_FRIENDLY_NAME), text(&MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_VIDCAP_SYMBOLIC_LINK)) {
                    out.push((name, id));
                }
            }
        }
        windows::Win32::System::Com::CoTaskMemFree(Some(list as *const _));
    }
    out
}

/// Abre a câmera `id` (de [`cameras`]) como um decodificador ao vivo.
pub fn open_camera(id: &str, request: crate::camera::CameraRequest) -> Box<dyn VideoDecoder> {
    Box::new(MfVideo {
        request,
        caps: None,
        path: PathBuf::new(),
        camera: Some(id.to_string()),
        reader: None,
        gpu: None,
        native: None,
        info: VideoInfo::default(),
        stride: 0,
        crop: (0, 0),
        rows: 0,
        error: None,
        skip_until: None,
    })
}

/// A fábrica do decodificador do sistema.
pub struct MediaFoundation;

impl VideoDecoderFactory for MediaFoundation {
    fn name(&self) -> &str {
        "Media Foundation"
    }

    fn open(&self, source: &MediaSource) -> Result<Box<dyn VideoDecoder>, String> {
        let path = to_path(source)?;
        // Um endereço vai direto ao `MFCreateSourceReaderFromURL` (que lê pela rede, com `Range`).
        if !crate::video::is_url(&path) && !path.exists() {
            return Err(aylis_core::messages::tf("arquivo não encontrado: {0}", &[&path.display()]));
        }
        Ok(Box::new(MfVideo {
            path,
            camera: None,
            reader: None,
            gpu: None,
            native: None,
            info: VideoInfo::default(),
            stride: 0,
            crop: (0, 0),
            rows: 0,
            error: None,
            skip_until: None,
            request: Default::default(),
            caps: None,
        }))
    }
}

struct MfVideo {
    path: PathBuf,
    /// Câmera (o "symbolic link" do dispositivo) no lugar do arquivo.
    camera: Option<String>,
    reader: Option<IMFSourceReader>,
    /// A placa em que o leitor decodifica (enquanto ele existir).
    gpu: Option<IMFDXGIDeviceManager>,
    /// O tamanho do vídeo (a saída pode ser menor: [`VideoDecoder::set_output_size`]).
    native: Option<(u32, u32)>,
    info: VideoInfo,
    stride: i32,
    /// Canto da área visível dentro do quadro decodificado. O H.264 codifica em blocos de
    /// 16: um vídeo de 360 linhas sai com 368, e a área visível diz quais mostrar.
    crop: (usize, usize),
    /// Linhas do quadro decodificado (com o enchimento).
    rows: usize,
    error: Option<String>,
    /// Depois de uma busca: o leitor volta ao quadro-chave anterior; os quadros antes do
    /// tempo pedido são decodificados e descartados (busca precisa).
    skip_until: Option<f64>,
    /// Câmera: a qualidade pedida e o que ela oferece (lido ao abrir).
    request: crate::camera::CameraRequest,
    caps: Option<crate::camera::CameraCaps>,
}

impl MfVideo {
    fn ensure(&mut self) -> bool {
        if self.reader.is_some() {
            return true;
        }
        if self.error.is_some() {
            return false;
        }
        match self.open() {
            Ok(()) => true,
            Err(e) => {
                self.error = Some(e);
                false
            }
        }
    }

    fn open(&mut self) -> Result<(), String> {
        init();
        // Arquivos: decodifica e converte as cores na placa (um 4K a 60 quadros por segundo
        // não cabe no processador); se a placa recusar o formato, pelo processador.
        if self.camera.is_none() {
            if let Some(manager) = gpu_manager() {
                if self.open_with(Some(&manager)).is_ok() {
                    self.gpu = Some(manager);
                    return Ok(());
                }
                self.reader = None;
            }
        }
        self.open_with(None)
    }

    fn open_with(&mut self, gpu: Option<&IMFDXGIDeviceManager>) -> Result<(), String> {
        // SAFETY: chamadas de COM com objetos válidos criados aqui mesmo.
        unsafe {
            let mut attrs = None;
            MFCreateAttributes(&mut attrs, 4).map_err(err)?;
            let attrs = attrs.ok_or("Media Foundation: atributos")?;
            match gpu {
                Some(manager) => {
                    attrs.SetUnknown(&MF_SOURCE_READER_D3D_MANAGER, manager).map_err(err)?;
                    attrs.SetUINT32(&MF_READWRITE_ENABLE_HARDWARE_TRANSFORMS, 1).map_err(err)?;
                    attrs.SetUINT32(&MF_SOURCE_READER_ENABLE_ADVANCED_VIDEO_PROCESSING, 1).map_err(err)?;
                }
                // Converte para RGB32 (o leitor faz a conversão de cor).
                None => attrs.SetUINT32(&MF_SOURCE_READER_ENABLE_VIDEO_PROCESSING, 1).map_err(err)?,
            }
            let reader = match &self.camera {
                Some(link) => {
                    let mut dev = None;
                    MFCreateAttributes(&mut dev, 2).map_err(err)?;
                    let dev = dev.ok_or("Media Foundation: atributos")?;
                    dev.SetGUID(&MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE, &MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_VIDCAP_GUID).map_err(err)?;
                    dev.SetString(&MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_VIDCAP_SYMBOLIC_LINK, &HSTRING::from(link.as_str())).map_err(err)?;
                    let source = MFCreateDeviceSource(&dev).map_err(err)?;
                    MFCreateSourceReaderFromMediaSource(&source, &attrs).map_err(err)?
                }
                None => MFCreateSourceReaderFromURL(&HSTRING::from(self.path.as_os_str()), &attrs).map_err(err)?,
            };
            reader.SetStreamSelection(ALL, false).map_err(err)?;
            reader.SetStreamSelection(VIDEO, true).map_err(|_| aylis_core::messages::t("o arquivo não tem vídeo"))?;
            if self.camera.is_some() {
                self.choose_camera_format(&reader);
            }
            let mt = MFCreateMediaType().map_err(err)?;
            mt.SetGUID(&MF_MT_MAJOR_TYPE, &MFMediaType_Video).map_err(err)?;
            mt.SetGUID(&MF_MT_SUBTYPE, &MFVideoFormat_RGB32).map_err(err)?;
            reader.SetCurrentMediaType(VIDEO, None, &mt).map_err(|e| aylis_core::messages::tf("formato de vídeo não suportado pelo sistema ({0})", &[&e.message()]))?;
            self.reader = Some(reader);
            self.read_format()?;
            self.native = Some((self.info.width, self.info.height));
            let duration = self
                .reader
                .as_ref()
                .and_then(|r| r.GetPresentationAttribute(MF_SOURCE_READER_MEDIASOURCE.0 as u32, &MF_PD_DURATION).ok())
                .and_then(|pv| u64::try_from(&pv).ok())
                .map(|d| d as f64 / 10_000_000.0);
            self.info.duration = duration;
        }
        Ok(())
    }

    /// Câmera: lê os formatos que ela oferece (os tamanhos vão para `caps`) e escolhe o do
    /// tamanho pedido, com a maior taxa de quadros dele; a conversão para RGB32 vem depois,
    /// no mesmo tamanho. Se a câmera recusar, fica o formato padrão dela.
    fn choose_camera_format(&mut self, reader: &IMFSourceReader) {
        // SAFETY: leitor válido; os tipos de mídia são só lidos.
        unsafe {
            let mut natives = Vec::new();
            let mut index = 0;
            while let Ok(t) = reader.GetNativeMediaType(VIDEO, index) {
                if let Ok(size) = t.GetUINT64(&MF_MT_FRAME_SIZE) {
                    let rate = t.GetUINT64(&MF_MT_FRAME_RATE).map(|r| (r >> 32) as f64 / (r as u32).max(1) as f64).unwrap_or(0.0);
                    natives.push(((size >> 32) as u32, size as u32, rate, t));
                }
                index += 1;
            }
            let sizes = crate::camera::sort_sizes(natives.iter().map(|n| (n.0, n.1)).collect());
            let chosen = crate::camera::choose_size(&sizes, &self.request);
            self.caps = Some(crate::camera::CameraCaps { sizes, has_flash: false });
            let best = chosen.and_then(|(w, h)| natives.iter().filter(|n| n.0 == w && n.1 == h).max_by(|a, b| a.2.total_cmp(&b.2)));
            if let Some((_, _, _, t)) = best {
                let _ = reader.SetCurrentMediaType(VIDEO, None, t);
            }
        }
    }

    /// Tamanho, passo das linhas e quadros por segundo do formato atual.
    fn read_format(&mut self) -> Result<(), String> {
        let reader = self.reader.as_ref().ok_or("sem leitor")?;
        // SAFETY: leitor válido.
        unsafe {
            let current = reader.GetCurrentMediaType(VIDEO).map_err(err)?;
            let size = current.GetUINT64(&MF_MT_FRAME_SIZE).map_err(err)?;
            let (full_w, full_h) = ((size >> 32) as u32, size as u32);
            self.rows = full_h as usize;
            self.stride = current.GetUINT32(&MF_MT_DEFAULT_STRIDE).map(|s| s as i32).unwrap_or(full_w as i32 * 4);
            // área visível (sem as linhas e colunas de enchimento do codificador)
            let mut area = MFVideoArea::default();
            // SAFETY: MFVideoArea é uma struct C simples; o slice cobre exatamente seus bytes.
            let bytes = std::slice::from_raw_parts_mut(&mut area as *mut MFVideoArea as *mut u8, std::mem::size_of::<MFVideoArea>());
            let visible = [MF_MT_MINIMUM_DISPLAY_APERTURE, MF_MT_GEOMETRIC_APERTURE]
                .iter()
                .any(|key| current.GetBlob(key, bytes, None).is_ok() && area.Area.cx > 0 && area.Area.cy > 0);
            let (x, y) = (area.OffsetX.value.max(0) as u32, area.OffsetY.value.max(0) as u32);
            if visible && x + area.Area.cx as u32 <= full_w && y + area.Area.cy as u32 <= full_h {
                self.info.width = area.Area.cx as u32;
                self.info.height = area.Area.cy as u32;
                self.crop = (x as usize, y as usize);
            } else {
                self.info.width = full_w;
                self.info.height = full_h;
                self.crop = (0, 0);
            }
            self.info.fps = current.GetUINT64(&MF_MT_FRAME_RATE).ok().map(|r| (r >> 32) as f64 / (r as u32).max(1) as f64);
        }
        Ok(())
    }

    fn read(&mut self) -> Result<Option<Frame>, String> {
        loop {
            let reader = self.reader.as_ref().ok_or("sem leitor")?;
            let mut flags = 0u32;
            let mut timestamp = 0i64;
            let mut sample = None;
            // SAFETY: os ponteiros apontam para variáveis locais vivas durante a chamada.
            unsafe { reader.ReadSample(VIDEO, 0, None, Some(&mut flags), Some(&mut timestamp), Some(&mut sample)) }.map_err(err)?;
            if flags & MF_SOURCE_READERF_ENDOFSTREAM.0 as u32 != 0 {
                return Ok(None);
            }
            if flags & MF_SOURCE_READERF_CURRENTMEDIATYPECHANGED.0 as u32 != 0 {
                self.read_format()?;
            }
            let Some(sample) = sample else { continue }; // marcação de tempo sem quadro
            let time = timestamp as f64 / 10_000_000.0;
            if let Some(target) = self.skip_until {
                let half_frame = 0.5 / self.info.fps.unwrap_or(30.0).max(1.0);
                if time + half_frame < target {
                    continue;
                }
                self.skip_until = None;
            }
            let (w, h) = (self.info.width as usize, self.info.height as usize);
            let mut pixels = vec![0u8; w * h * 4];
            // SAFETY: o buffer fica travado enquanto copiamos; a cópia respeita o passo das
            // linhas informado pelo próprio buffer (negativo = de baixo para cima).
            unsafe {
                let buffer = sample.ConvertToContiguousBuffer().map_err(err)?;
                let mut ptr: *mut u8 = std::ptr::null_mut();
                let mut pitch = 0i32;
                let (top, pitch, locked_2d) = match buffer.cast::<IMF2DBuffer>() {
                    Ok(b2) if b2.Lock2D(&mut ptr, &mut pitch).is_ok() => (ptr, pitch as isize, Some(b2)),
                    _ => {
                        let mut len = 0u32;
                        buffer.Lock(&mut ptr, None, Some(&mut len)).map_err(err)?;
                        let stride = self.stride as isize;
                        let rows = self.rows as isize;
                        let top = if stride < 0 { ptr.offset(-stride * (rows - 1)) } else { ptr };
                        (top, stride, None)
                    }
                };
                for y in 0..h {
                    let src = top.offset((y + self.crop.1) as isize * pitch).add(self.crop.0 * 4);
                    std::ptr::copy_nonoverlapping(src, pixels.as_mut_ptr().add(y * w * 4), w * 4);
                }
                match locked_2d {
                    Some(b2) => b2.Unlock2D().ok(),
                    None => buffer.Unlock().ok(),
                };
            }
            // RGB32 não tem alfa: o quarto byte pode vir zerado (o conversor da placa já o
            // deixa em 255, e aí não precisa passar pelos pixels).
            if pixels.chunks_exact(4).take(64).any(|px| px[3] != 255) {
                for px in pixels.chunks_exact_mut(4) {
                    px[3] = 255;
                }
            }
            return Ok(Some(Frame::bgra(w as u32, h as u32, pixels, time)));
        }
    }
}

impl VideoDecoder for MfVideo {
    fn camera_caps(&self) -> Option<crate::camera::CameraCaps> {
        self.caps.clone()
    }

    fn info(&self) -> VideoInfo {
        // O tamanho do vídeo, não o da saída reduzida.
        match self.native {
            Some((width, height)) => VideoInfo { width, height, ..self.info },
            None => self.info,
        }
    }

    fn set_output_size(&mut self, width: u32, height: u32) {
        // Só o conversor da placa redimensiona; e nunca maior que o vídeo.
        let (Some(reader), Some(_), Some((nw, nh))) = (&self.reader, &self.gpu, self.native) else { return };
        let scale = (width as f64 / nw as f64).max(height as f64 / nh as f64).min(1.0);
        let even = |v: f64| ((v / 2.0).round() as u32 * 2).max(2);
        let (ow, oh) = (even(nw as f64 * scale), even(nh as f64 * scale));
        if (ow, oh) == (self.info.width, self.info.height) {
            return;
        }
        // SAFETY: leitor válido; o tipo de mídia é criado aqui.
        let ok = unsafe {
            (|| -> windows::core::Result<()> {
                let mt = MFCreateMediaType()?;
                mt.SetGUID(&MF_MT_MAJOR_TYPE, &MFMediaType_Video)?;
                mt.SetGUID(&MF_MT_SUBTYPE, &MFVideoFormat_RGB32)?;
                mt.SetUINT64(&MF_MT_FRAME_SIZE, ((ow as u64) << 32) | oh as u64)?;
                reader.SetCurrentMediaType(VIDEO, None, &mt)
            })()
        };
        if ok.is_ok() {
            let _ = self.read_format();
        }
    }

    fn seek(&mut self, seconds: f64) {
        if !self.ensure() {
            return;
        }
        if let Some(reader) = &self.reader {
            let position = PROPVARIANT::from((seconds * 10_000_000.0) as i64);
            // SAFETY: GUID nulo = tempo em 100 ns; o PROPVARIANT vive durante a chamada.
            if unsafe { reader.SetCurrentPosition(&GUID::zeroed(), &position) }.is_ok() {
                self.skip_until = (seconds > 0.0).then_some(seconds);
            }
        }
    }

    fn seek_keyframe(&mut self, seconds: f64) {
        self.seek(seconds);
        self.skip_until = None;
    }

    fn skip_until(&mut self, seconds: f64) {
        self.skip_until = Some(seconds);
    }

    fn next_frame(&mut self) -> Option<Frame> {
        if !self.ensure() {
            return None;
        }
        match self.read() {
            Ok(frame) => frame,
            Err(e) => {
                self.error = Some(e);
                None
            }
        }
    }

    fn error(&self) -> Option<String> {
        self.error.clone()
    }

    fn audio(&mut self) -> Option<AudioOpener> {
        // O leitor de áudio nasce na thread do áudio, onde vai ser usado.
        let (path, duration) = (self.path.clone(), self.info.duration);
        Some(Box::new(move || Ok(Box::new(MfAudio { path, reader: None, rate: 48_000, channels: 2, failed: false, duration }) as Box<dyn AudioSource>)))
    }
}

/// A trilha de áudio de um vídeo, decodificada pelo Media Foundation em `f32`.
struct MfAudio {
    path: PathBuf,
    reader: Option<IMFSourceReader>,
    rate: u32,
    channels: u16,
    failed: bool,
    duration: Option<f64>,
}

impl MfAudio {
    fn ensure(&mut self) -> bool {
        if self.reader.is_some() {
            return true;
        }
        if self.failed {
            return false;
        }
        init();
        // SAFETY: chamadas de COM com objetos criados aqui.
        let result: Result<(), windows::core::Error> = unsafe {
            (|| {
                let reader = MFCreateSourceReaderFromURL(&HSTRING::from(self.path.as_os_str()), None)?;
                reader.SetStreamSelection(ALL, false)?;
                reader.SetStreamSelection(AUDIO, true)?;
                let mt = MFCreateMediaType()?;
                mt.SetGUID(&MF_MT_MAJOR_TYPE, &MFMediaType_Audio)?;
                mt.SetGUID(&MF_MT_SUBTYPE, &MFAudioFormat_Float)?;
                reader.SetCurrentMediaType(AUDIO, None, &mt)?;
                let current = reader.GetCurrentMediaType(AUDIO)?;
                self.channels = current.GetUINT32(&MF_MT_AUDIO_NUM_CHANNELS)? as u16;
                self.rate = current.GetUINT32(&MF_MT_AUDIO_SAMPLES_PER_SECOND)?;
                self.reader = Some(reader);
                Ok(())
            })()
        };
        if result.is_err() {
            self.failed = true; // sem trilha de áudio: silêncio
        }
        self.reader.is_some()
    }
}

impl AudioSource for MfAudio {
    fn sample_rate(&self) -> u32 {
        self.rate
    }

    fn channels(&self) -> u16 {
        self.channels
    }

    fn read(&mut self, out: &mut Vec<f32>) -> bool {
        if !self.ensure() {
            return false;
        }
        let Some(reader) = &self.reader else { return false };
        loop {
            let mut flags = 0u32;
            let mut sample = None;
            // SAFETY: ponteiros para variáveis locais.
            if unsafe { reader.ReadSample(AUDIO, 0, None, Some(&mut flags), None, Some(&mut sample)) }.is_err() {
                return false;
            }
            if flags & MF_SOURCE_READERF_ENDOFSTREAM.0 as u32 != 0 {
                return false;
            }
            let Some(sample) = sample else { continue };
            // SAFETY: o buffer fica travado durante a cópia e tem `len` bytes; lidos como bytes
            // (sem supor o alinhamento de f32, que o Media Foundation não garante).
            unsafe {
                let Ok(buffer) = sample.ConvertToContiguousBuffer() else { return false };
                let mut ptr: *mut u8 = std::ptr::null_mut();
                let mut len = 0u32;
                if buffer.Lock(&mut ptr, None, Some(&mut len)).is_err() {
                    return false;
                }
                if !ptr.is_null() {
                    let bytes = std::slice::from_raw_parts(ptr, len as usize);
                    out.extend(bytes.chunks_exact(4).map(|b| f32::from_ne_bytes([b[0], b[1], b[2], b[3]])));
                }
                let _ = buffer.Unlock();
            }
            return true;
        }
    }

    fn seek(&mut self, seconds: f64) {
        if self.ensure() {
            if let Some(reader) = &self.reader {
                let position = PROPVARIANT::from((seconds * 10_000_000.0) as i64);
                // SAFETY: como no vídeo.
                let _ = unsafe { reader.SetCurrentPosition(&GUID::zeroed(), &position) };
            }
        }
    }

    fn duration(&self) -> Option<f64> {
        self.duration
    }
}

/// Grava um MP4 H.264 com quadros BGRA (`width * height * 4` bytes cada), com o
/// codificador do sistema. Largura e altura precisam ser pares.
pub fn write_mp4(path: &Path, width: u32, height: u32, fps: u32, frames: impl IntoIterator<Item = Vec<u8>>) -> Result<(), String> {
    if width % 2 != 0 || height % 2 != 0 {
        return Err("largura e altura precisam ser pares (H.264)".into());
    }
    init();
    let size = ((width as u64) << 32) | height as u64;
    let rate = ((fps as u64) << 32) | 1;
    let duration = 10_000_000 / fps as i64;
    // SAFETY: chamadas de COM com objetos criados aqui; os buffers são travados durante a cópia.
    unsafe {
        let writer = MFCreateSinkWriterFromURL(&HSTRING::from(path.as_os_str()), None, None).map_err(err)?;
        let out = MFCreateMediaType().map_err(err)?;
        out.SetGUID(&MF_MT_MAJOR_TYPE, &MFMediaType_Video).map_err(err)?;
        out.SetGUID(&MF_MT_SUBTYPE, &MFVideoFormat_H264).map_err(err)?;
        out.SetUINT32(&MF_MT_AVG_BITRATE, 2_000_000).map_err(err)?;
        out.SetUINT32(&MF_MT_INTERLACE_MODE, MFVideoInterlace_Progressive.0 as u32).map_err(err)?;
        out.SetUINT64(&MF_MT_FRAME_SIZE, size).map_err(err)?;
        out.SetUINT64(&MF_MT_FRAME_RATE, rate).map_err(err)?;
        out.SetUINT64(&MF_MT_PIXEL_ASPECT_RATIO, (1 << 32) | 1).map_err(err)?;
        let stream = writer.AddStream(&out).map_err(err)?;
        let input = MFCreateMediaType().map_err(err)?;
        input.SetGUID(&MF_MT_MAJOR_TYPE, &MFMediaType_Video).map_err(err)?;
        input.SetGUID(&MF_MT_SUBTYPE, &MFVideoFormat_RGB32).map_err(err)?;
        input.SetUINT32(&MF_MT_INTERLACE_MODE, MFVideoInterlace_Progressive.0 as u32).map_err(err)?;
        input.SetUINT64(&MF_MT_FRAME_SIZE, size).map_err(err)?;
        input.SetUINT64(&MF_MT_FRAME_RATE, rate).map_err(err)?;
        input.SetUINT64(&MF_MT_PIXEL_ASPECT_RATIO, (1 << 32) | 1).map_err(err)?;
        // Linhas de cima para baixo.
        input.SetUINT32(&MF_MT_DEFAULT_STRIDE, width * 4).map_err(err)?;
        writer.SetInputMediaType(stream, &input, None).map_err(err)?;
        writer.BeginWriting().map_err(err)?;
        for (i, frame) in frames.into_iter().enumerate() {
            let len = width * height * 4;
            if frame.len() != len as usize {
                return Err(format!("quadro {i} com {} bytes (esperado {len})", frame.len()));
            }
            let buffer = MFCreateMemoryBuffer(len).map_err(err)?;
            let mut ptr: *mut u8 = std::ptr::null_mut();
            buffer.Lock(&mut ptr, None, None).map_err(err)?;
            std::ptr::copy_nonoverlapping(frame.as_ptr(), ptr, frame.len());
            buffer.Unlock().map_err(err)?;
            buffer.SetCurrentLength(len).map_err(err)?;
            let sample = MFCreateSample().map_err(err)?;
            sample.AddBuffer(&buffer).map_err(err)?;
            sample.SetSampleTime(i as i64 * duration).map_err(err)?;
            sample.SetSampleDuration(duration).map_err(err)?;
            writer.WriteSample(stream, &sample).map_err(err)?;
        }
        writer.Finalize().map_err(err)?;
    }
    Ok(())
}
