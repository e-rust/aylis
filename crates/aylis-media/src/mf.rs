//! Media Foundation (Windows): o decodificador de vídeo padrão (MP4/H.264, e o que mais o
//! sistema souber: HEVC, VP9, WMV... conforme as extensões instaladas), a trilha de áudio
//! do vídeo e a gravação de MP4 H.264 ([`write_mp4`]).
//!
//! Os objetos do Media Foundation são criados na thread que vai usá-los (a de vídeo ou a
//! de áudio do player), na primeira chamada.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use aylis_core::{GpuDevice, GpuSurface, Image};
use windows::Win32::Foundation::{CloseHandle, GENERIC_ALL, HANDLE, HMODULE, LUID, RECT};
use windows::Win32::Graphics::Direct3D::{D3D_DRIVER_TYPE_HARDWARE, D3D_DRIVER_TYPE_UNKNOWN};
use windows::Win32::Graphics::Direct3D11::*;
use windows::Win32::Graphics::Dxgi::Common::{DXGI_FORMAT_B8G8R8A8_UNORM, DXGI_RATIONAL, DXGI_SAMPLE_DESC};
use windows::Win32::Graphics::Dxgi::{
    CreateDXGIFactory1, DXGI_SHARED_RESOURCE_READ, DXGI_SHARED_RESOURCE_WRITE, IDXGIAdapter, IDXGIDevice, IDXGIFactory4, IDXGIResource1,
};
use windows::Win32::Media::MediaFoundation::*;
use windows::Win32::System::Com::StructuredStorage::PROPVARIANT;
use windows::Win32::System::Com::{COINIT_APARTMENTTHREADED, CoInitializeEx};
use windows::core::{GUID, HSTRING, Interface, PCWSTR};

use crate::audio::{AudioOpener, AudioSource};
use crate::video::{Frame, GpuFrame, GpuFrameSource, MediaSource, VideoDecoder, VideoDecoderFactory, VideoInfo};

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

// ---------------------------------------------------------------------------
// A placa de vídeo: decodificação e quadros como textura (W15)

/// A placa em que os vídeos decodificam: um Direct3D 11 por placa, o mesmo para todos os
/// vídeos dela, porque o quadro decodificado é convertido numa textura do mesmo dispositivo.
///
/// As cercas compartilhadas ([`FencePair`]) ficam abertas enquanto o processo vive e passam
/// de um vídeo para o seguinte: o Direct2D guarda as que abriu pelo número do handle, e um
/// handle fechado pode voltar com o mesmo número para outra cerca (que ele nunca veria
/// marcada).
struct VideoGpu {
    luid: i64,
    /// Criado sem placa pedida (a padrão do sistema).
    default: bool,
    device: ID3D11Device,
    context: ID3D11DeviceContext,
    /// O aylis usa o contexto imediato em sequências (converter e marcar a cerca, copiar
    /// para a CPU): uma de cada vez. O Media Foundation usa o mesmo contexto, protegido pelo
    /// `SetMultithreadProtected`.
    lock: Mutex<()>,
    fences: Mutex<Vec<FencePair>>,
}

static GPUS: Mutex<Vec<Arc<VideoGpu>>> = Mutex::new(Vec::new());

/// A placa em que os vídeos decodificam por padrão (o LUID do adaptador, como no
/// [`GpuDevice::Direct3D11`]). `None` sem placa com decodificação.
pub fn video_adapter() -> Option<i64> {
    VideoGpu::get(None).map(|g| g.luid)
}

fn luid_value(luid: LUID) -> i64 {
    ((luid.HighPart as i64) << 32) | luid.LowPart as i64
}

impl VideoGpu {
    /// A placa `luid` (`None`: a padrão), criada na primeira vez. `None` sem placa com
    /// decodificação (máquina virtual, driver básico).
    fn get(luid: Option<i64>) -> Option<Arc<VideoGpu>> {
        let mut list = GPUS.lock().ok()?;
        if let Some(g) = list.iter().find(|g| luid.map_or(g.default, |l| g.luid == l)) {
            return Some(g.clone());
        }
        let g = Arc::new(Self::create(luid)?);
        list.push(g.clone());
        Some(g)
    }

    fn create(luid: Option<i64>) -> Option<Self> {
        // SAFETY: COM do DXGI e do Direct3D 11, com objetos criados aqui.
        unsafe {
            let adapter: Option<IDXGIAdapter> = match luid {
                Some(l) => Some(CreateDXGIFactory1::<IDXGIFactory4>().ok()?.EnumAdapterByLuid(LUID { LowPart: l as u32, HighPart: (l >> 32) as i32 }).ok()?),
                None => None,
            };
            let mut device = None;
            let mut context = None;
            D3D11CreateDevice(
                adapter.as_ref(),
                if adapter.is_some() { D3D_DRIVER_TYPE_UNKNOWN } else { D3D_DRIVER_TYPE_HARDWARE },
                HMODULE::default(),
                D3D11_CREATE_DEVICE_VIDEO_SUPPORT | D3D11_CREATE_DEVICE_BGRA_SUPPORT,
                None,
                D3D11_SDK_VERSION,
                Some(&mut device),
                None,
                Some(&mut context),
            )
            .ok()?;
            let (device, context) = (device?, context?);
            // O decodificador e o conversor do Media Foundation usam o dispositivo de outras threads.
            if let Ok(mt) = device.cast::<ID3D11Multithread>() {
                let _ = mt.SetMultithreadProtected(true);
            }
            let luid = luid_value(device.cast::<IDXGIDevice>().ok()?.GetAdapter().ok()?.GetDesc().ok()?.AdapterLuid);
            Some(Self { luid, default: adapter.is_none(), device, context, lock: Mutex::new(()), fences: Mutex::new(Vec::new()) })
        }
    }

    /// O gerente de dispositivo do Media Foundation para um leitor (decodifica nesta placa).
    fn manager(&self) -> Option<IMFDXGIDeviceManager> {
        // SAFETY: COM do Media Foundation, com o dispositivo vivo.
        unsafe {
            let mut token = 0u32;
            let mut manager = None;
            MFCreateDXGIDeviceManager(&mut token, &mut manager).ok()?;
            let manager = manager?;
            manager.ResetDevice(&self.device, token).ok()?;
            Some(manager)
        }
    }

    /// Um par de cercas (o quadro pronto, o quadro lido), reaproveitado se houver.
    fn fence_pair(&self) -> Option<FencePair> {
        if let Some(p) = self.fences.lock().ok()?.pop() {
            return Some(p);
        }
        let device: ID3D11Device5 = self.device.cast().ok()?;
        let fence = || -> Option<(ID3D11Fence, isize)> {
            // SAFETY: cerca criada aqui; o handle fica aberto enquanto o processo vive.
            unsafe {
                let mut f: Option<ID3D11Fence> = None;
                device.CreateFence(0, D3D11_FENCE_FLAG_SHARED, &mut f).ok()?;
                let f = f?;
                let handle = f.CreateSharedHandle(None, GENERIC_ALL.0, PCWSTR::null()).ok()?;
                Some((f, handle.0 as isize))
            }
        };
        let (ready, ready_handle) = fence()?;
        let (release, release_handle) = fence()?;
        Some(FencePair { ready, ready_handle, release, release_handle, value: 0 })
    }
}

/// As duas cercas compartilhadas de um vídeo: `ready` (o aylis marca quando o quadro está
/// na textura) e `release` (o canvas marca quando terminou de ler). Os dois usam o mesmo
/// número por quadro, que só cresce.
struct FencePair {
    ready: ID3D11Fence,
    ready_handle: isize,
    release: ID3D11Fence,
    release_handle: isize,
    /// O último número usado.
    value: u64,
}

/// As cercas de um vídeo, devolvidas à placa quando a última textura dele morre.
struct Fences {
    gpu: Arc<VideoGpu>,
    pair: Option<FencePair>,
    value: AtomicU64,
}

impl Fences {
    fn pair(&self) -> &FencePair {
        self.pair.as_ref().expect("cercas vivas até o Drop")
    }
}

impl Drop for Fences {
    fn drop(&mut self) {
        if let (Some(mut pair), Ok(mut free)) = (self.pair.take(), self.gpu.fences.lock()) {
            pair.value = self.value.load(Ordering::Acquire);
            free.push(pair);
        }
    }
}

/// Uma textura BGRA compartilhada (handle NT) que recebe quadros convertidos.
struct Slot {
    /// Único no processo: o Direct2D guarda o bitmap aberto por ele.
    id: u64,
    texture: ID3D11Texture2D,
    handle: isize,
    view: ID3D11VideoProcessorOutputView,
    size: (u32, u32),
    /// O número do quadro que está nela (o mesmo das cercas).
    value: AtomicU64,
    /// O canvas compôs este quadro: antes de reescrever, espera a cerca de liberação.
    drawn: AtomicBool,
    fences: Arc<Fences>,
}

impl Drop for Slot {
    fn drop(&mut self) {
        // SAFETY: handle criado por `CreateSharedHandle` e fechado só aqui.
        unsafe {
            let _ = CloseHandle(HANDLE(self.handle as *mut _));
        }
    }
}

static NEXT_SLOT: AtomicU64 = AtomicU64::new(1);

/// O quadro entregue ao player: segura a textura até o último clone sumir.
struct TextureFrame {
    slot: Arc<Slot>,
}

impl GpuFrameSource for TextureFrame {
    fn drawn(&self) {
        self.slot.drawn.store(true, Ordering::Release);
    }

    fn download(&self) -> Option<Image> {
        let gpu = &self.slot.fences.gpu;
        let (w, h) = self.slot.size;
        // SAFETY: textura de cópia criada aqui; o mapa é lido dentro dos limites que o
        // próprio Direct3D informa (`RowPitch` por linha) e desfeito em seguida.
        unsafe {
            let desc = D3D11_TEXTURE2D_DESC {
                Width: w,
                Height: h,
                MipLevels: 1,
                ArraySize: 1,
                Format: DXGI_FORMAT_B8G8R8A8_UNORM,
                SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
                Usage: D3D11_USAGE_STAGING,
                BindFlags: 0,
                CPUAccessFlags: D3D11_CPU_ACCESS_READ.0 as u32,
                MiscFlags: 0,
            };
            let mut staging = None;
            gpu.device.CreateTexture2D(&desc, None, Some(&mut staging)).ok()?;
            let staging = staging?;
            let _lock = gpu.lock.lock().ok()?;
            gpu.context.CopyResource(&staging, &self.slot.texture);
            let mut mapped = D3D11_MAPPED_SUBRESOURCE::default();
            gpu.context.Map(&staging, 0, D3D11_MAP_READ, 0, Some(&mut mapped)).ok()?;
            let row = w as usize * 4;
            let mut pixels = vec![0u8; row * h as usize];
            for y in 0..h as usize {
                let src = (mapped.pData as *const u8).add(y * mapped.RowPitch as usize);
                std::ptr::copy_nonoverlapping(src, pixels.as_mut_ptr().add(y * row), row);
            }
            gpu.context.Unmap(&staging, 0);
            Some(Image::bgra(w, h, pixels))
        }
    }
}

/// O conversor da placa (YUV para BGRA, no tamanho de saída) para um formato de entrada.
struct Processor {
    enumerator: ID3D11VideoProcessorEnumerator,
    processor: ID3D11VideoProcessor,
    /// Entrada (com o enchimento), área visível, saída e espaço de cor.
    key: ((u32, u32), RECT, (u32, u32), u32),
}

/// Quantas texturas um vídeo usa (a fila, o quadro na tela e o que o canvas ainda lê).
const POOL: usize = 12;
/// Quantos quadros esperam na fila com texturas.
const QUEUED: usize = 8;

/// Os quadros como textura: o decodificador entrega NV12 na placa, e o conversor de vídeo
/// do Direct3D 11 escreve BGRA numa textura compartilhada, que o Direct2D compõe direto.
struct TexturePath {
    gpu: Arc<VideoGpu>,
    video_device: ID3D11VideoDevice,
    video_context: ID3D11VideoContext,
    context4: ID3D11DeviceContext4,
    fences: Arc<Fences>,
    processor: Option<Processor>,
    pool: Vec<Arc<Slot>>,
}

impl TexturePath {
    fn new(gpu: Arc<VideoGpu>) -> Option<Self> {
        let video_device: ID3D11VideoDevice = gpu.device.cast().ok()?;
        let video_context: ID3D11VideoContext = gpu.context.cast().ok()?;
        let context4: ID3D11DeviceContext4 = gpu.context.cast().ok()?;
        let pair = gpu.fence_pair()?;
        let value = AtomicU64::new(pair.value);
        let fences = Arc::new(Fences { gpu: gpu.clone(), pair: Some(pair), value });
        Some(Self { gpu, video_device, video_context, context4, fences, processor: None, pool: Vec::new() })
    }

    /// O conversor para esta entrada e saída (refeito, com as texturas, quando mudam).
    fn processor(&mut self, key: ((u32, u32), RECT, (u32, u32), u32), fps: f64) -> Result<(), String> {
        if self.processor.as_ref().is_some_and(|p| p.key == key) {
            return Ok(());
        }
        let ((iw, ih), source, (ow, oh), color) = key;
        let rate = DXGI_RATIONAL { Numerator: (fps.max(1.0) * 1000.0).round() as u32, Denominator: 1000 };
        let desc = D3D11_VIDEO_PROCESSOR_CONTENT_DESC {
            InputFrameFormat: D3D11_VIDEO_FRAME_FORMAT_PROGRESSIVE,
            InputFrameRate: rate,
            InputWidth: iw,
            InputHeight: ih,
            OutputFrameRate: rate,
            OutputWidth: ow,
            OutputHeight: oh,
            Usage: D3D11_VIDEO_USAGE_PLAYBACK_NORMAL,
        };
        // SAFETY: COM do vídeo do Direct3D 11; as estruturas vivem durante as chamadas.
        unsafe {
            let enumerator = self.video_device.CreateVideoProcessorEnumerator(&desc).map_err(err)?;
            let processor = self.video_device.CreateVideoProcessor(&enumerator, 0).map_err(err)?;
            let vc = &self.video_context;
            let target = RECT { left: 0, top: 0, right: ow as i32, bottom: oh as i32 };
            vc.VideoProcessorSetStreamFrameFormat(&processor, 0, D3D11_VIDEO_FRAME_FORMAT_PROGRESSIVE);
            vc.VideoProcessorSetStreamSourceRect(&processor, 0, true, Some(&source));
            vc.VideoProcessorSetStreamDestRect(&processor, 0, true, Some(&target));
            vc.VideoProcessorSetOutputTargetRect(&processor, true, Some(&target));
            vc.VideoProcessorSetStreamColorSpace(&processor, 0, &D3D11_VIDEO_PROCESSOR_COLOR_SPACE { _bitfield: color });
            // Saída RGB de 0 a 255 (a mesma do caminho pela CPU).
            vc.VideoProcessorSetOutputColorSpace(&processor, &D3D11_VIDEO_PROCESSOR_COLOR_SPACE { _bitfield: 0 });
            // Sem "melhorias" automáticas do driver: o quadro sai igual ao do caminho pela CPU.
            vc.VideoProcessorSetStreamAutoProcessingMode(&processor, 0, false);
            self.processor = Some(Processor { enumerator, processor, key });
        }
        // As texturas antigas morrem quando os quadros que as usam saírem da tela.
        self.pool.clear();
        Ok(())
    }

    /// Uma textura livre: ninguém segura o quadro dela e, se o canvas o compôs, a placa dele
    /// já terminou de ler. Sem nenhuma, cria até [`POOL`]; passando disso, espera um pouco
    /// (o canvas pode não marcar a cerca: a janela escondida) e depois cria outra.
    fn slot(&mut self, size: (u32, u32)) -> Result<Arc<Slot>, String> {
        let deadline = Instant::now() + Duration::from_millis(100);
        loop {
            // SAFETY: leitura do valor da cerca.
            let done = unsafe { self.fences.pair().release.GetCompletedValue() };
            let free = |s: &&Arc<Slot>| Arc::strong_count(s) == 1;
            if let Some(s) = self.pool.iter().filter(free).find(|s| !s.drawn.load(Ordering::Acquire) || done >= s.value.load(Ordering::Acquire)) {
                return Ok(s.clone());
            }
            let late = Instant::now() >= deadline;
            if late {
                if let Some(s) = self.pool.iter().find(free) {
                    return Ok(s.clone());
                }
            }
            if self.pool.len() < POOL || late {
                let s = Arc::new(self.new_slot(size)?);
                self.pool.push(s.clone());
                return Ok(s);
            }
            std::thread::sleep(Duration::from_millis(1));
        }
    }

    fn new_slot(&self, (w, h): (u32, u32)) -> Result<Slot, String> {
        let enumerator = &self.processor.as_ref().ok_or("sem conversor")?.enumerator;
        let desc = D3D11_TEXTURE2D_DESC {
            Width: w,
            Height: h,
            MipLevels: 1,
            ArraySize: 1,
            Format: DXGI_FORMAT_B8G8R8A8_UNORM,
            SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
            Usage: D3D11_USAGE_DEFAULT,
            BindFlags: (D3D11_BIND_RENDER_TARGET.0 | D3D11_BIND_SHADER_RESOURCE.0) as u32,
            CPUAccessFlags: 0,
            MiscFlags: (D3D11_RESOURCE_MISC_SHARED.0 | D3D11_RESOURCE_MISC_SHARED_NTHANDLE.0) as u32,
        };
        // SAFETY: textura e vista criadas aqui; o handle é fechado no `Drop` do `Slot`.
        unsafe {
            let mut texture = None;
            self.gpu.device.CreateTexture2D(&desc, None, Some(&mut texture)).map_err(err)?;
            let texture = texture.ok_or("Direct3D 11: textura")?;
            let view_desc = D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC {
                ViewDimension: D3D11_VPOV_DIMENSION_TEXTURE2D,
                Anonymous: D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC_0 { Texture2D: D3D11_TEX2D_VPOV { MipSlice: 0 } },
            };
            let mut view = None;
            self.video_device.CreateVideoProcessorOutputView(&texture, enumerator, &view_desc, Some(&mut view)).map_err(err)?;
            let handle = texture
                .cast::<IDXGIResource1>()
                .map_err(err)?
                .CreateSharedHandle(None, DXGI_SHARED_RESOURCE_READ.0 | DXGI_SHARED_RESOURCE_WRITE.0, PCWSTR::null())
                .map_err(err)?;
            Ok(Slot {
                id: NEXT_SLOT.fetch_add(1, Ordering::Relaxed),
                texture,
                handle: handle.0 as isize,
                view: view.ok_or("Direct3D 11: vista")?,
                size: (w, h),
                value: AtomicU64::new(0),
                drawn: AtomicBool::new(false),
                fences: self.fences.clone(),
            })
        }
    }

    /// Converte o quadro decodificado (na placa) numa textura compartilhada.
    fn convert(&mut self, sample: &IMFSample, key: ((u32, u32), RECT, (u32, u32), u32), fps: f64, time: f64) -> Result<Frame, String> {
        // SAFETY: COM do Media Foundation e do Direct3D 11; o `pInputSurface` da estrutura
        // é solto à mão depois da conversão (ManuallyDrop).
        unsafe {
            let buffer = sample.GetBufferByIndex(0).map_err(err)?;
            let dxgi: IMFDXGIBuffer = buffer.cast().map_err(|_| "o quadro não está na placa de vídeo".to_string())?;
            let mut raw = std::ptr::null_mut();
            dxgi.GetResource(&ID3D11Texture2D::IID, &mut raw).map_err(err)?;
            let decoded = ID3D11Texture2D::from_raw(raw);
            let index = dxgi.GetSubresourceIndex().map_err(err)?;
            self.processor(key, fps)?;
            let size = key.2;
            let slot = self.slot(size)?;
            let p = self.processor.as_ref().ok_or("sem conversor")?;
            let in_desc = D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC {
                FourCC: 0,
                ViewDimension: D3D11_VPIV_DIMENSION_TEXTURE2D,
                Anonymous: D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC_0 { Texture2D: D3D11_TEX2D_VPIV { MipSlice: 0, ArraySlice: index } },
            };
            let mut input = None;
            self.video_device.CreateVideoProcessorInputView(&decoded, &p.enumerator, &in_desc, Some(&mut input)).map_err(err)?;
            let mut stream = D3D11_VIDEO_PROCESSOR_STREAM { Enable: true.into(), pInputSurface: std::mem::ManuallyDrop::new(input), ..Default::default() };
            let value = {
                let _lock = self.gpu.lock.lock().map_err(|e| e.to_string())?;
                let blt = self.video_context.VideoProcessorBlt(&p.processor, &slot.view, 0, std::slice::from_ref(&stream));
                std::mem::ManuallyDrop::drop(&mut stream.pInputSurface);
                blt.map_err(err)?;
                let value = self.fences.value.fetch_add(1, Ordering::AcqRel) + 1;
                self.context4.Signal(&self.fences.pair().ready, value).map_err(err)?;
                self.gpu.context.Flush();
                value
            };
            slot.value.store(value, Ordering::Release);
            slot.drawn.store(false, Ordering::Release);
            let pair = self.fences.pair();
            let surface = GpuSurface::D3D12 {
                id: slot.id,
                texture: slot.handle,
                width: size.0,
                height: size.1,
                ready: pair.ready_handle,
                ready_value: value,
                release: pair.release_handle,
                release_value: value,
            };
            Ok(Frame::texture(GpuFrame::new(surface, TextureFrame { slot }), time))
        }
    }
}

/// O espaço de cor do vídeo para o conversor da placa (os bits do
/// `D3D11_VIDEO_PROCESSOR_COLOR_SPACE`): a matriz BT.709 ou BT.601 e a faixa (16 a 235 ou
/// 0 a 255). Sem a informação no arquivo, o que os players fazem: BT.709 de 720 linhas para
/// cima, faixa de 16 a 235.
fn color_bits(matrix: Option<u32>, range: Option<u32>, height: u32) -> u32 {
    let bt709 = match matrix {
        Some(m) if m == MFVideoTransferMatrix_BT709.0 as u32 => true,
        Some(m) if m == MFVideoTransferMatrix_BT601.0 as u32 => false,
        _ => height >= 720,
    };
    let full = range == Some(MFNominalRange_0_255.0 as u32);
    // bit 2: matriz 709; bits 4 e 5: faixa nominal (1 = 16 a 235, 2 = 0 a 255).
    (if bt709 { 1 << 2 } else { 0 }) | (if full { 2 } else { 1 }) << 4
}

/// O tamanho de saída para mostrar `native` em `want` pixels: nunca maior que o vídeo, com
/// lados pares.
fn scaled(native: (u32, u32), want: (u32, u32)) -> (u32, u32) {
    let (nw, nh) = native;
    if want.0 == 0 || want.1 == 0 {
        return native;
    }
    let scale = (want.0 as f64 / nw as f64).max(want.1 as f64 / nh as f64).min(1.0);
    let even = |v: f64| ((v / 2.0).round() as u32 * 2).max(2);
    (even(nw as f64 * scale), even(nh as f64 * scale))
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
    Box::new(MfVideo::new(PathBuf::new(), Some(id.to_string()), request))
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
        Ok(Box::new(MfVideo::new(path, None, Default::default())))
    }
}

struct MfVideo {
    path: PathBuf,
    /// Câmera (o "symbolic link" do dispositivo) no lugar do arquivo.
    camera: Option<String>,
    reader: Option<IMFSourceReader>,
    /// O gerente do Media Foundation da placa em que o leitor decodifica.
    gpu: Option<IMFDXGIDeviceManager>,
    /// A placa (o mesmo dispositivo do gerente).
    device: Option<Arc<VideoGpu>>,
    /// A placa pedida pelo canvas (`None`: a padrão do sistema).
    luid: Option<i64>,
    /// Entregando os quadros como textura (W15).
    texture: Option<TexturePath>,
    /// O tamanho do vídeo (a saída pode ser menor: [`VideoDecoder::set_output_size`]).
    native: Option<(u32, u32)>,
    /// O tamanho em que o vídeo aparece (0: o do vídeo).
    want: (u32, u32),
    info: VideoInfo,
    stride: i32,
    /// Canto da área visível dentro do quadro decodificado. O H.264 codifica em blocos de
    /// 16: um vídeo de 360 linhas sai com 368, e a área visível diz quais mostrar.
    crop: (usize, usize),
    /// Linhas do quadro decodificado (com o enchimento).
    rows: usize,
    /// Largura e altura do quadro decodificado (com o enchimento).
    coded: (u32, u32),
    /// O espaço de cor para o conversor da placa ([`color_bits`]).
    color: u32,
    /// O tempo do último quadro entregue (para reabrir no mesmo lugar em outra placa).
    last_time: f64,
    error: Option<String>,
    /// Depois de uma busca: o leitor volta ao quadro-chave anterior; os quadros antes do
    /// tempo pedido são decodificados e descartados (busca precisa).
    skip_until: Option<f64>,
    /// Câmera: a qualidade pedida e o que ela oferece (lido ao abrir).
    request: crate::camera::CameraRequest,
    caps: Option<crate::camera::CameraCaps>,
}

impl MfVideo {
    fn new(path: PathBuf, camera: Option<String>, request: crate::camera::CameraRequest) -> Self {
        Self {
            path,
            camera,
            reader: None,
            gpu: None,
            device: None,
            luid: None,
            texture: None,
            native: None,
            want: (0, 0),
            info: VideoInfo::default(),
            stride: 0,
            crop: (0, 0),
            rows: 0,
            coded: (0, 0),
            color: 0,
            last_time: 0.0,
            error: None,
            skip_until: None,
            request,
            caps: None,
        }
    }

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
            if let Some((gpu, manager)) = VideoGpu::get(self.luid).and_then(|g| g.manager().map(|m| (g, m))) {
                if self.open_with(Some(&manager)).is_ok() {
                    self.gpu = Some(manager);
                    self.device = Some(gpu);
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
            self.coded = (full_w, full_h);
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
            self.color = color_bits(current.GetUINT32(&MF_MT_YUV_MATRIX).ok(), current.GetUINT32(&MF_MT_VIDEO_NOMINAL_RANGE).ok(), self.info.height);
        }
        Ok(())
    }

    /// Pede ao leitor quadros `subtype` no tamanho `size` (`None`: o do vídeo).
    fn set_type(&self, subtype: &GUID, size: Option<(u32, u32)>) -> windows::core::Result<()> {
        let Some(reader) = &self.reader else { return Err(windows::core::Error::empty()) };
        // SAFETY: leitor válido; o tipo de mídia é criado aqui.
        unsafe {
            let mt = MFCreateMediaType()?;
            mt.SetGUID(&MF_MT_MAJOR_TYPE, &MFMediaType_Video)?;
            mt.SetGUID(&MF_MT_SUBTYPE, subtype)?;
            if let Some((w, h)) = size {
                mt.SetUINT64(&MF_MT_FRAME_SIZE, ((w as u64) << 32) | h as u64)?;
            }
            reader.SetCurrentMediaType(VIDEO, None, &mt)
        }
    }

    /// Passa a entregar texturas: o leitor dá o quadro decodificado (NV12, ou P010 nos
    /// vídeos de 10 bits) e o conversor da placa faz o resto.
    fn start_texture(&mut self) {
        let Some(gpu) = self.device.clone() else { return };
        if self.gpu.is_none() {
            return;
        }
        let Some(path) = TexturePath::new(gpu) else { return };
        if ![MFVideoFormat_NV12, MFVideoFormat_P010].iter().any(|s| self.set_type(s, None).is_ok()) {
            self.back_to_pixels();
            return;
        }
        if self.read_format().is_err() {
            self.back_to_pixels();
            return;
        }
        self.texture = Some(path);
    }

    /// Volta aos pixels (RGB32 no tamanho em que o vídeo aparece).
    fn back_to_pixels(&mut self) {
        self.texture = None;
        let size = self.native.map(|n| scaled(n, self.want)).filter(|s| Some(*s) != self.native);
        if self.set_type(&MFVideoFormat_RGB32, size).is_err() {
            let _ = self.set_type(&MFVideoFormat_RGB32, None);
        }
        let _ = self.read_format();
    }

    /// Reabre o arquivo em outra placa (a do canvas), no mesmo lugar.
    fn reopen_on(&mut self, luid: i64) {
        let at = self.last_time;
        self.reader = None;
        self.gpu = None;
        self.device = None;
        self.texture = None;
        self.luid = Some(luid);
        if self.ensure() {
            self.seek(at);
            if self.want != (0, 0) {
                let want = self.want;
                self.set_output_size(want.0, want.1);
            }
        }
    }

    fn read(&mut self) -> Result<Option<Frame>, String> {
        loop {
            let reader = self.reader.clone().ok_or("sem leitor")?;
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
            self.last_time = time;
            if let Some(path) = &mut self.texture {
                let (cx, cy) = (self.crop.0 as i32, self.crop.1 as i32);
                let source = RECT { left: cx, top: cy, right: cx + self.info.width as i32, bottom: cy + self.info.height as i32 };
                let out = scaled((self.info.width, self.info.height), self.want);
                let key = (self.coded, source, out, self.color);
                match path.convert(&sample, key, self.info.fps.unwrap_or(30.0), time) {
                    Ok(frame) => return Ok(Some(frame)),
                    // O quadro não veio na placa (o decodificador do processador) ou a
                    // conversão falhou: volta aos pixels a partir do próximo.
                    Err(_) => {
                        self.back_to_pixels();
                        continue;
                    }
                }
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
        self.want = (width, height);
        // Com texturas, o conversor da placa já entrega no tamanho pedido (a cada quadro).
        if self.texture.is_some() {
            return;
        }
        // Só o conversor da placa redimensiona; e nunca maior que o vídeo.
        let (Some(_), Some(_), Some(native)) = (&self.reader, &self.gpu, self.native) else { return };
        let out = scaled(native, (width, height));
        if out == (self.info.width, self.info.height) {
            return;
        }
        if self.set_type(&MFVideoFormat_RGB32, Some(out)).is_ok() {
            let _ = self.read_format();
        }
    }

    fn set_gpu_device(&mut self, device: Option<GpuDevice>) {
        if self.camera.is_some() || !self.ensure() {
            return;
        }
        match device {
            Some(GpuDevice::Direct3D11 { adapter_luid }) => {
                // O canvas está em outra placa: a textura tem de nascer na dele.
                if self.device.as_ref().is_some_and(|g| g.luid != adapter_luid) {
                    self.reopen_on(adapter_luid);
                }
                if self.texture.is_none() && self.device.as_ref().is_some_and(|g| g.luid == adapter_luid) {
                    self.start_texture();
                }
            }
            _ => {
                if self.texture.is_some() {
                    self.back_to_pixels();
                }
            }
        }
    }

    fn max_queued(&self) -> Option<usize> {
        self.texture.as_ref().map(|_| QUEUED)
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
        Some(Box::new(move || Ok(Box::new(MfAudio::new(path, duration)) as Box<dyn AudioSource>)))
    }
}

/// O som de um arquivo ou de um endereço (`http://`, `https://`: o Media Foundation lê pela
/// rede, com `Range`), decodificado pelo Media Foundation em `f32`. Abre aqui, na thread que
/// chama (a do áudio, onde a fonte vai viver); o erro diz por que não abriu.
pub fn open_audio(location: &str) -> Result<Box<dyn AudioSource>, String> {
    let mut audio = MfAudio::new(PathBuf::from(location), None);
    if !audio.ensure() {
        return Err(audio.error.take().unwrap_or_else(|| "Media Foundation: o arquivo não tem som".into()));
    }
    Ok(Box::new(audio))
}

/// A trilha de áudio de um vídeo, decodificada pelo Media Foundation em `f32`.
struct MfAudio {
    path: PathBuf,
    reader: Option<IMFSourceReader>,
    rate: u32,
    channels: u16,
    failed: bool,
    error: Option<String>,
    duration: Option<f64>,
}

impl MfAudio {
    fn new(path: PathBuf, duration: Option<f64>) -> Self {
        Self { path, reader: None, rate: 48_000, channels: 2, failed: false, error: None, duration }
    }

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
                if self.duration.is_none() {
                    self.duration = reader
                        .GetPresentationAttribute(MF_SOURCE_READER_MEDIASOURCE.0 as u32, &MF_PD_DURATION)
                        .ok()
                        .and_then(|pv| u64::try_from(&pv).ok())
                        .map(|d| d as f64 / 10_000_000.0);
                }
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
        if let Err(e) = result {
            self.failed = true; // sem trilha de áudio: silêncio
            self.error = Some(err(e));
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
