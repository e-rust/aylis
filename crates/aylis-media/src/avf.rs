//! AVFoundation (macOS e iOS): o decodificador de vídeo do sistema. O `AVAssetReader` lê
//! a trilha de vídeo já em BGRA (com o hardware do aparelho); buscar recria o leitor a
//! partir do tempo pedido. A trilha de áudio vai pelo symphonia, como nos outros sistemas.

use std::ffi::c_void;
use std::path::PathBuf;
use std::sync::Arc;

use objc2::encode::{Encode, Encoding};
use objc2::runtime::AnyObject;
use objc2::{class, msg_send};
use objc2_foundation::{CGSize, NSString};

use crate::audio::{AudioOpener, AudioSource, SymphoniaSource};
use crate::video::{Frame, MediaSource, VideoDecoder, VideoDecoderFactory, VideoInfo};

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
struct CMTime {
    value: i64,
    timescale: i32,
    flags: u32,
    epoch: i64,
}

// SAFETY: o mesmo leiaute do `CMTime` do CoreMedia (struct anônima: "?").
unsafe impl Encode for CMTime {
    const ENCODING: Encoding = Encoding::Struct("?", &[i64::ENCODING, i32::ENCODING, u32::ENCODING, i64::ENCODING]);
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct CMTimeRange {
    start: CMTime,
    duration: CMTime,
}

// SAFETY: o mesmo leiaute do `CMTimeRange`.
unsafe impl Encode for CMTimeRange {
    const ENCODING: Encoding = Encoding::Struct("?", &[CMTime::ENCODING, CMTime::ENCODING]);
}

impl CMTime {
    fn seconds(self) -> Option<f64> {
        // kCMTimeFlags_Valid = 1
        (self.flags & 1 != 0 && self.timescale > 0).then(|| self.value as f64 / self.timescale as f64)
    }

    fn from_seconds(s: f64) -> Self {
        Self { value: (s.max(0.0) * 600.0) as i64, timescale: 600, flags: 1, epoch: 0 }
    }
}

#[link(name = "AVFoundation", kind = "framework")]
unsafe extern "C" {}

#[link(name = "CoreMedia", kind = "framework")]
unsafe extern "C" {
    fn CMSampleBufferGetImageBuffer(buffer: *mut c_void) -> *mut c_void;
    fn CMSampleBufferGetPresentationTimeStamp(buffer: *mut c_void) -> CMTime;
}

#[link(name = "CoreVideo", kind = "framework")]
unsafe extern "C" {
    static kCVPixelBufferPixelFormatTypeKey: *const AnyObject;
    fn CVPixelBufferLockBaseAddress(buffer: *mut c_void, flags: u64) -> i32;
    fn CVPixelBufferUnlockBaseAddress(buffer: *mut c_void, flags: u64) -> i32;
    fn CVPixelBufferGetBaseAddress(buffer: *mut c_void) -> *mut u8;
    fn CVPixelBufferGetBytesPerRow(buffer: *mut c_void) -> usize;
    fn CVPixelBufferGetDataSize(buffer: *mut c_void) -> usize;
    fn CVPixelBufferGetWidth(buffer: *mut c_void) -> usize;
    fn CVPixelBufferGetHeight(buffer: *mut c_void) -> usize;
}

#[link(name = "CoreFoundation", kind = "framework")]
unsafe extern "C" {
    fn CFRelease(obj: *const c_void);
}

/// A fábrica do decodificador do sistema no macOS e no iOS.
pub struct AvFoundation;

impl VideoDecoderFactory for AvFoundation {
    fn name(&self) -> &str {
        "AVFoundation"
    }

    fn open(&self, source: &MediaSource) -> Result<Box<dyn VideoDecoder>, String> {
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
        AvVideo::open(path, audio_bytes).map(|d| Box::new(d) as Box<dyn VideoDecoder>)
    }
}

struct AvVideo {
    path: PathBuf,
    audio_bytes: Option<(Arc<Vec<u8>>, String)>,
    asset: *mut AnyObject,
    track: *mut AnyObject,
    reader: *mut AnyObject,
    output: *mut AnyObject,
    info: VideoInfo,
    error: Option<String>,
}

impl AvVideo {
    fn open(path: PathBuf, audio_bytes: Option<(Arc<Vec<u8>>, String)>) -> Result<Self, String> {
        // SAFETY: mensagens do Objective-C a objetos válidos; os retidos aqui são soltos no
        // Drop.
        unsafe {
            let ns_path = NSString::from_str(&path.to_string_lossy());
            // Endereço de rede (W11): o `NSURL` do texto; arquivo: o do caminho.
            let url: *mut AnyObject = if crate::video::is_url(&path) {
                msg_send![class!(NSURL), URLWithString: &*ns_path]
            } else {
                msg_send![class!(NSURL), fileURLWithPath: &*ns_path]
            };
            let asset: *mut AnyObject = msg_send![class!(AVURLAsset), URLAssetWithURL: url, options: std::ptr::null_mut::<AnyObject>()];
            if asset.is_null() {
                return Err("AVFoundation: não abriu o arquivo".into());
            }
            let _: *mut AnyObject = msg_send![asset, retain];
            let kind = NSString::from_str("vide");
            let tracks: *mut AnyObject = msg_send![asset, tracksWithMediaType: &*kind];
            let count: usize = if tracks.is_null() { 0 } else { msg_send![tracks, count] };
            let mut me = Self {
                path,
                audio_bytes,
                asset,
                track: std::ptr::null_mut(),
                reader: std::ptr::null_mut(),
                output: std::ptr::null_mut(),
                info: VideoInfo::default(),
                error: None,
            };
            if count == 0 {
                return Err(aylis_core::messages::t("o arquivo não tem vídeo"));
            }
            let track: *mut AnyObject = msg_send![tracks, objectAtIndex: 0usize];
            let _: *mut AnyObject = msg_send![track, retain];
            me.track = track;
            let size: CGSize = msg_send![track, naturalSize];
            let fps: f32 = msg_send![track, nominalFrameRate];
            let duration: CMTime = msg_send![asset, duration];
            me.info = VideoInfo { width: size.width as u32, height: size.height as u32, duration: duration.seconds(), fps: (fps > 0.0).then_some(fps as f64) };
            me.start(0.0)?;
            Ok(me)
        }
    }

    /// (Re)cria o leitor a partir de `seconds`.
    fn start(&mut self, seconds: f64) -> Result<(), String> {
        // SAFETY: como em `open`.
        unsafe {
            self.release_reader();
            let mut error: *mut AnyObject = std::ptr::null_mut();
            let reader: *mut AnyObject = msg_send![class!(AVAssetReader), assetReaderWithAsset: self.asset, error: &mut error];
            if reader.is_null() {
                return Err("AVFoundation: leitor".into());
            }
            let _: *mut AnyObject = msg_send![reader, retain];
            self.reader = reader;
            // BGRA ('BGRA' = 0x42475241)
            let format: *mut AnyObject = msg_send![class!(NSNumber), numberWithUnsignedInt: 0x4247_5241u32];
            let settings: *mut AnyObject = msg_send![class!(NSDictionary), dictionaryWithObject: format, forKey: kCVPixelBufferPixelFormatTypeKey];
            let output: *mut AnyObject = msg_send![class!(AVAssetReaderTrackOutput), assetReaderTrackOutputWithTrack: self.track, outputSettings: settings];
            if output.is_null() {
                return Err(aylis_core::messages::tf("formato de vídeo não suportado pelo sistema ({0})", &[&"AVFoundation"]));
            }
            let _: *mut AnyObject = msg_send![output, retain];
            self.output = output;
            let _: () = msg_send![output, setAlwaysCopiesSampleData: false];
            let _: () = msg_send![reader, addOutput: output];
            if seconds > 0.0 {
                let end = self.info.duration.unwrap_or(seconds + 3600.0);
                let range = CMTimeRange { start: CMTime::from_seconds(seconds), duration: CMTime::from_seconds((end - seconds).max(0.0) + 1.0) };
                let _: () = msg_send![reader, setTimeRange: range];
            }
            let ok: bool = msg_send![reader, startReading];
            if !ok {
                return Err(aylis_core::messages::tf("formato de vídeo não suportado pelo sistema ({0})", &[&"AVFoundation"]));
            }
            Ok(())
        }
    }

    fn release_reader(&mut self) {
        // SAFETY: solta o que `start` reteve.
        unsafe {
            if !self.reader.is_null() {
                let _: () = msg_send![self.reader, cancelReading];
                let _: () = msg_send![self.reader, release];
                self.reader = std::ptr::null_mut();
            }
            if !self.output.is_null() {
                let _: () = msg_send![self.output, release];
                self.output = std::ptr::null_mut();
            }
        }
    }
}

impl VideoDecoder for AvVideo {
    fn info(&self) -> VideoInfo {
        self.info
    }

    fn seek(&mut self, seconds: f64) {
        if let Err(e) = self.start(seconds) {
            self.error = Some(e);
        }
    }

    fn next_frame(&mut self) -> Option<Frame> {
        if self.output.is_null() {
            return None;
        }
        // SAFETY: o sample buffer é nosso (copy...) até o CFRelease; o pixel buffer vale
        // enquanto ele existir, travado para leitura.
        unsafe {
            let sample: *mut c_void = msg_send![self.output, copyNextSampleBuffer];
            if sample.is_null() {
                return None;
            }
            let time = CMSampleBufferGetPresentationTimeStamp(sample).seconds().unwrap_or(0.0);
            let image = CMSampleBufferGetImageBuffer(sample);
            let mut frame = None;
            if !image.is_null() && CVPixelBufferLockBaseAddress(image, 1) == 0 {
                let (w, h) = (CVPixelBufferGetWidth(image), CVPixelBufferGetHeight(image));
                let row = CVPixelBufferGetBytesPerRow(image);
                let base = CVPixelBufferGetBaseAddress(image);
                // Cada linha tem pelo menos `w * 4` bytes e todas cabem no buffer (o BGRA
                // pedido garante; outro formato descarta o quadro em vez de ler fora).
                let fits = w.checked_mul(4).is_some_and(|line| {
                    row >= line && row.checked_mul(h.saturating_sub(1)).and_then(|r| r.checked_add(line)).is_some_and(|need| need <= CVPixelBufferGetDataSize(image))
                });
                if !base.is_null() && w > 0 && h > 0 && fits {
                    let mut pixels = Vec::with_capacity(w * h * 4);
                    for y in 0..h {
                        pixels.extend_from_slice(std::slice::from_raw_parts(base.add(y * row), w * 4));
                    }
                    frame = Some(Frame::bgra(w as u32, h as u32, pixels, time));
                    self.info.width = w as u32;
                    self.info.height = h as u32;
                }
                CVPixelBufferUnlockBaseAddress(image, 1);
            }
            CFRelease(sample);
            frame
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

impl Drop for AvVideo {
    fn drop(&mut self) {
        self.release_reader();
        // SAFETY: solta o que `open` reteve.
        unsafe {
            if !self.track.is_null() {
                let _: () = msg_send![self.track, release];
            }
            let _: () = msg_send![self.asset, release];
        }
    }
}
