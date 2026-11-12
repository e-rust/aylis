//! O renderizador: dispositivo `wgpu`, modelos enviados à GPU e as vistas (cada `Model3D`
//! tem a sua). O quadro sai numa textura compartilhada com o renderizador do aylis (sem
//! passar pela CPU) ou, como reserva, em pixels.

use std::collections::HashMap;

use glam::{Mat4, Vec3};
use aylis_core::{GpuDevice, GpuSurface};
use wgpu::util::DeviceExt;

use crate::model::{LightKind, Material, Model, Vertex};

const SAMPLES: u32 = 4;
const COLOR: wgpu::TextureFormat = wgpu::TextureFormat::Bgra8Unorm;
const DEPTH: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;
/// Lado do mapa de sombra, em texels.
const SHADOW_SIZE: u32 = 2048;
/// Luzes por quadro (as do arquivo além disso ficam de fora).
pub const MAX_LIGHTS: usize = 4;
/// Ossos por esqueleto (o resto é ignorado).
pub const MAX_JOINTS: usize = 128;
/// Tamanho de um bloco de `Draw` no buffer (alinhamento dos offsets dinâmicos).
const DRAW_STRIDE: u64 = 256;

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraUniform {
    view_proj: [[f32; 4]; 4],
    eye: [f32; 4],
    sky: [f32; 4],
    ground: [f32; 4],
    /// Luzes, sombra ligada, texel da sombra em UV e no mundo.
    counts: [f32; 4],
    shadow_view_proj: [[f32; 4]; 4],
    lights: [LightUniform; MAX_LIGHTS],
    /// Traçado de raios: ligado, amostra acumulada, alcance da oclusão, afastamento.
    rays: [f32; 4],
    /// Brilho: a força com que a emissão vai para a textura do brilho.
    glow: [f32; 4],
}

/// O Direct3D 12 foi aberto com o compilador DXC (o único que compila ray query).
static DXC: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Amostras acumuladas com a cena parada (depois disso o quadro não muda mais).
pub const MAX_SAMPLES: u32 = 256;
/// Formato da média das amostras (16 bits: 1/256 de cada amostra não some).
const ACCUM: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
/// Com a textura do brilho em 8 bits, ela guarda 1/4 da luz (a brasa passa de 1).
const GLOW_RANGE_8BIT: f32 = 4.0;
/// Níveis do brilho, cada um com metade do tamanho (o último, 1/32 do quadro).
const GLOW_LEVELS: usize = 5;
/// O menor nível do brilho, em pixels (o lado menor).
const GLOW_MIN: u32 = 8;
/// Quanto cada nível mais fundo (mais espalhado) pesa ao subir: o brilho fica forte junto
/// da fonte e cai rápido.
const GLOW_FALLOFF: f64 = 0.55;

/// Os passes do brilho.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Bloom {
    Down,
    Up,
    Composite,
}

#[repr(C)]
#[derive(Clone, Copy, Default, bytemuck::Pod, bytemuck::Zeroable)]
struct LightUniform {
    /// Posição e tipo (0 direcional, 1 pontual, 2 cone).
    position: [f32; 4],
    /// Para onde vai e o alcance (0 = infinito).
    direction: [f32; 4],
    color: [f32; 4],
    /// Escala e deslocamento do cosseno do cone.
    cone: [f32; 4],
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct DrawUniform {
    world: [[f32; 4]; 4],
    normal: [[f32; 4]; 4],
    base_color: [f32; 4],
    params: [f32; 4],
    emissive: [f32; 4],
    /// Força do mapa de normais, da oclusão, tem metal/aspereza, tem emissão.
    maps: [f32; 4],
    /// Verniz: força e aspereza.
    coat: [f32; 4],
}

/// O que o shader de um desenho precisa: o resto fica fora do pipeline compilado.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
struct Variant {
    /// Algum mapa no material (normal, oclusão, metal/aspereza, emissão).
    maps: bool,
    shadow: bool,
    /// Mais de uma luz.
    lights: bool,
    /// Com o traçado de raios (o shader com ray query e o grupo 4).
    rays: bool,
    /// Com o brilho: a emissão vai também para a textura do brilho (segunda saída).
    glow: bool,
}

/// O que um quadro produziu.
pub enum Output {
    /// Textura na GPU, para o canvas compor sem cópia.
    Surface(GpuSurface),
    /// Pixels RGBA (alfa não pré-multiplicado), quando não há como compartilhar.
    Pixels { width: u32, height: u32, rgba: Vec<u8> },
}

/// Câmera e cena de um quadro.
#[derive(Clone, Copy, Debug)]
pub struct Frame {
    pub width: u32,
    pub height: u32,
    pub view: Mat4,
    pub proj: Mat4,
    pub eye: Vec3,
    /// Fundo RGBA 0..1 (alfa 0 = transparente: o que está atrás aparece).
    pub background: [f32; 4],
    /// Direção da luz padrão (de onde ela vem para onde vai).
    pub light_dir: Vec3,
    /// Usar as luzes do arquivo, quando houver (no lugar da padrão).
    pub file_lights: bool,
    /// Sombra da primeira luz direcional.
    pub shadows: bool,
    /// Traçado de raios (quando a placa traça; senão, o desenho de sempre): sombra de
    /// todas as luzes, reflexo nos materiais lisos e oclusão de ambiente.
    pub ray_traced: bool,
    /// Com `ray_traced`: a amostra acumulada (1, 2, 3… com a cena parada; a imagem é a
    /// média delas, até [`MAX_SAMPLES`]). 0 = não acumular.
    pub sample: u32,
    /// Brilho (bloom): a luz que os materiais emitem espalha em volta deles, forte junto da
    /// fonte e caindo rápido; o valor é a força (0 = sem brilho, o desenho de sempre).
    pub glow: f32,
}

/// O dispositivo e o que é comum a todas as vistas.
pub struct Engine {
    pub info: wgpu::AdapterInfo,
    /// LUID do adaptador (Windows), para conferir com o do renderizador do aylis.
    pub luid: Option<i64>,
    camera_layout: wgpu::BindGroupLayout,
    draw_layout: wgpu::BindGroupLayout,
    material_layout: wgpu::BindGroupLayout,
    joints_layout: wgpu::BindGroupLayout,
    pipeline_layout: wgpu::PipelineLayout,
    shader: wgpu::ShaderModule,
    pipelines: HashMap<(bool, bool, Variant, wgpu::TextureFormat), wgpu::RenderPipeline>,
    /// O traçado de raios, quando a placa traça (ray query).
    rays: Option<Rays>,
    /// Acumular as amostras: a mistura na média e a cópia para a saída (por formato).
    accum_layout: wgpu::BindGroupLayout,
    accum_shader: wgpu::ShaderModule,
    accum_pipelines: HashMap<(bool, wgpu::TextureFormat), wgpu::RenderPipeline>,
    /// O brilho: o formato da textura dele (16 bits onde a placa desenha e mistura nele;
    /// senão 8 bits guardando 1/[`GLOW_RANGE_8BIT`] da luz), os passes e o amostrador.
    glow_format: wgpu::TextureFormat,
    bloom_layout: wgpu::BindGroupLayout,
    bloom_shader: wgpu::ShaderModule,
    bloom_pipelines: HashMap<(Bloom, wgpu::TextureFormat), wgpu::RenderPipeline>,
    bloom_sampler: wgpu::Sampler,
    /// O passe da sombra: só profundidade, vista da luz (a câmera dele não tem o mapa).
    shadow_camera_layout: wgpu::BindGroupLayout,
    shadow_pipeline: Option<wgpu::RenderPipeline>,
    sampler: wgpu::Sampler,
    shadow_sampler: wgpu::Sampler,
    white: wgpu::TextureView,
    white_linear: wgpu::TextureView,
    flat_normal: wgpu::TextureView,
    /// Mapa de sombra vazio, para vistas sem sombra.
    no_shadow: wgpu::TextureView,
    no_joints: wgpu::BindGroup,
    #[cfg(windows)]
    interop: Option<crate::d3d12::Interop>,
    /// Criado no contexto OpenGL do canvas: as texturas já são dele.
    gl: bool,
    /// No navegador: a instância e o adaptador (para os `<canvas>` das vistas).
    #[cfg(target_arch = "wasm32")]
    web: Option<(wgpu::Instance, wgpu::Adapter)>,
    #[cfg(target_arch = "wasm32")]
    web_format: Option<wgpu::TextureFormat>,
    /// No WebGL: o canvas de cujo contexto o dispositivo nasceu, até a primeira vista pegar.
    #[cfg(target_arch = "wasm32")]
    web_gl: Option<(web_sys::HtmlCanvasElement, wgpu::Surface<'static>)>,
    /// O dispositivo é do WebGL (um canvas só: uma vista 3D por página).
    #[cfg(target_arch = "wasm32")]
    web_is_gl: bool,
    #[cfg_attr(any(not(feature = "gl"), target_arch = "wasm32", target_vendor = "apple"), allow(dead_code))]
    gl_loader: usize,
    /// O estado do OpenGL que o wgpu e o Skia dividem (veja [`GlState`]).
    #[cfg(all(feature = "gl", not(target_arch = "wasm32"), not(target_vendor = "apple")))]
    gl_state: Option<GlState>,
    /// No OpenGL: os comandos dos `Model3D` pintados neste quadro, enviados de uma vez antes do
    /// flush do canvas ([`Engine::submit_pending`]). Um envio por controle custava ~2,5 ms de
    /// CPU cada no ASUS ZenFone 5 (o wgpu refaz o estado do OpenGL e as cercas a cada envio).
    pending: Vec<wgpu::CommandBuffer>,
    next_id: u64,
    // Por último: os campos saem na ordem da declaração, e o dispositivo do Direct3D 12
    // não pode ser destruído antes dos recursos dele (o processo cai ao fechar).
    pub queue: wgpu::Queue,
    pub device: wgpu::Device,
}

impl Engine {
    /// Abre a GPU de acordo com a do canvas: no Direct2D, o mesmo adaptador no Direct3D 12
    /// (texturas compartilhadas); no Skia com OpenGL, o próprio contexto dele. Sem nada
    /// (canvas na CPU), a GPU padrão, entregando pixels.
    pub fn new(device: Option<GpuDevice>) -> Result<Engine, String> {
        match device {
            Some(GpuDevice::Direct3D11 { adapter_luid }) => Self::new_d3d(Some(adapter_luid)),
            #[cfg(all(feature = "gl", not(target_arch = "wasm32"), not(target_vendor = "apple")))]
            Some(GpuDevice::OpenGl { loader, .. }) => Self::new_gl(loader),
            _ => Self::new_d3d(None),
        }
    }

    /// No contexto OpenGL atual (o do Skia), achando as funções com `loader`.
    ///
    #[cfg(all(feature = "gl", not(target_arch = "wasm32"), not(target_vendor = "apple")))]
    pub fn new_gl(loader: usize) -> Result<Engine, String> {
        type Loader = unsafe extern "C" fn(*const std::ffi::c_char) -> *const std::ffi::c_void;
        // SAFETY: a plataforma entrega o endereço de uma função com esta assinatura.
        let load: Loader = unsafe { std::mem::transmute::<usize, Loader>(loader) };
        clear_gl_errors(load);
        if crate::diag::diagnostic_setting("AYLIS_3D_TRACE").is_some() {
            let f = unsafe { load(c"glGetString".as_ptr()) };
            if !f.is_null() {
                let get: extern "system" fn(u32) -> *const std::ffi::c_char = unsafe { std::mem::transmute(f) };
                for id in [0x1F02u32, 0x8B8C] {
                    let p = get(id);
                    if !p.is_null() {
                        trace(&format!("GL {}", unsafe { std::ffi::CStr::from_ptr(p) }.to_string_lossy()));
                    }
                }
            }
        }
        let mut exposed = unsafe {
            wgpu_hal::gles::Adapter::new_external(
                |name| std::ffi::CString::new(name).map_or(std::ptr::null(), |n| load(n.as_ptr())),
                wgpu::GlBackendOptions::default(),
            )
        }
        .ok_or("OpenGL: o contexto não serve para o wgpu (precisa de GL 3.3 ou GLES 3.0)")?;
        trace("adaptador GL criado");
        // O tradutor de OpenGL ES do emulador do Android responde 0 para
        // GL_MAX_VERTEX_ATTRIB_BINDINGS (impossível: nada desenharia). O mínimo do GLES 3.1 é 16.
        if exposed.capabilities.limits.max_vertex_buffers == 0 {
            trace("OpenGL informou 0 vertex buffers; usando 8");
            exposed.capabilities.limits.max_vertex_buffers = 8;
        }
        // O 3D não usa computação nem desenho indireto. Sem isso, o wgpu tenta montar um
        // pipeline de computação ao abrir o dispositivo, e drivers que dizem GLES 3.1 mas só
        // compilam GLSL ES 3.00 (o emulador do Android) perdem o dispositivo.
        exposed.capabilities.downlevel.flags.remove(wgpu::DownlevelFlags::COMPUTE_SHADERS | wgpu::DownlevelFlags::INDIRECT_EXECUTION);
        // Sem backend nenhum: com o do OpenGL, o wgpu montaria um contexto próprio e, no
        // Windows, desligaria o do Skia ao terminar. O adaptador de fora não precisa dele.
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor { backends: wgpu::Backends::empty(), ..wgpu::InstanceDescriptor::new_without_display_handle() });
        let adapter = unsafe { instance.create_adapter_from_hal(exposed) };
        let mut engine = Self::with_adapter(adapter)?;
        trace("dispositivo GL criado");
        engine.gl = true;
        engine.gl_loader = loader;
        // Ao abrir, o wgpu deixou o VAO dele ligado: é o que ele espera achar em todo quadro.
        engine.gl_state = GlState::load(load);
        if let Some(st) = &mut engine.gl_state {
            st.vao = st.current_vao();
            st.hand_to_skia();
        }
        Ok(engine)
    }

    fn new_d3d(luid: Option<i64>) -> Result<Engine, String> {
        let backends = if cfg!(windows) && luid.is_some() { wgpu::Backends::DX12 } else { wgpu::Backends::PRIMARY };
        #[cfg_attr(not(windows), allow(unused_mut))]
        let mut descriptor = wgpu::InstanceDescriptor { backends, ..wgpu::InstanceDescriptor::new_without_display_handle() };
        // O traçado de raios no Direct3D 12 precisa do compilador DXC.
        #[cfg(windows)]
        if let Some(dxc) = crate::d3d12::dxc_path() {
            descriptor.backend_options.dx12.shader_compiler = wgpu::Dx12Compiler::DynamicDxc { dxc_path: dxc.display().to_string() };
            DXC.store(true, std::sync::atomic::Ordering::Relaxed);
        }
        let instance = wgpu::Instance::new(descriptor);
        #[cfg(windows)]
        let chosen = luid.and_then(|want| {
            let adapters = pollster::block_on(instance.enumerate_adapters(backends));
            adapters.into_iter().find(|a| crate::d3d12::adapter_luid(a) == Some(want))
        });
        #[cfg(not(windows))]
        let chosen: Option<wgpu::Adapter> = None;
        // Sem canvas na GPU para seguir: a placa escolhida no projeto (ou AYLIS_GPU).
        use aylis_core::gpu::GpuChoice;
        let choice = aylis_core::gpu::choice();
        let chosen = chosen.or_else(|| match &choice {
            GpuChoice::Named(_) => pollster::block_on(instance.enumerate_adapters(backends)).into_iter().find(|a| choice.matches_name(&a.get_info().name)),
            _ => None,
        });
        let power_preference = match choice {
            GpuChoice::HighPerformance => wgpu::PowerPreference::HighPerformance,
            GpuChoice::LowPower => wgpu::PowerPreference::LowPower,
            _ => wgpu::PowerPreference::default(),
        };
        let adapter = match chosen {
            Some(a) => a,
            None => pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions { power_preference, ..Default::default() })).map_err(|e| format!("sem GPU: {e}"))?,
        };
        Self::with_adapter(adapter)
    }

    pub fn with_adapter(adapter: wgpu::Adapter) -> Result<Engine, String> {
        pollster::block_on(Self::from_adapter(adapter))
    }

    /// No navegador: WebGPU, ou WebGL 2 onde não há (iPhone antes do iOS 26, muitos Android, ou
    /// um `navigator.gpu` que não dá adaptador). Abrir a GPU lá é assíncrono.
    ///
    /// No WebGL o adaptador só existe a partir do contexto de um canvas (pedir sem
    /// `compatible_surface` não acha nenhum): o canvas nasce aqui e vai para a primeira vista.
    #[cfg(target_arch = "wasm32")]
    pub async fn new_web() -> Result<Engine, String> {
        let webgpu = wgpu::Instance::new(wgpu::InstanceDescriptor { backends: wgpu::Backends::BROWSER_WEBGPU, ..wgpu::InstanceDescriptor::new_without_display_handle() });
        let webgpu_error = match webgpu.request_adapter(&wgpu::RequestAdapterOptions::default()).await {
            Ok(adapter) => match Self::from_adapter(adapter.clone()).await {
                Ok(mut engine) => {
                    engine.web = Some((webgpu, adapter));
                    return Ok(engine);
                }
                Err(e) => e,
            },
            Err(e) => e.to_string(),
        };
        trace(&format!("sem WebGPU ({webgpu_error}); tentando o WebGL 2"));
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor { backends: wgpu::Backends::GL, ..wgpu::InstanceDescriptor::new_without_display_handle() });
        let canvas = hidden_canvas().ok_or("sem documento para o canvas do 3D")?;
        let surface = instance.create_surface(wgpu::SurfaceTarget::Canvas(canvas.clone())).map_err(|e| format!("sem WebGL 2: {e}"))?;
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions { compatible_surface: Some(&surface), ..Default::default() })
            .await
            .map_err(|e| format!("sem GPU no navegador (WebGPU: {webgpu_error}; WebGL 2: {e})"))?;
        let mut engine = Self::from_adapter(adapter.clone()).await?;
        engine.web = Some((instance, adapter));
        engine.web_gl = Some((canvas, surface));
        engine.web_is_gl = true;
        Ok(engine)
    }

    pub async fn from_adapter(adapter: wgpu::Adapter) -> Result<Engine, String> {
        let info = adapter.get_info();
        // Traçado de raios: só pedido onde a placa tem (AYLIS_3D_NO_RAYS desliga, para testes).
        let compiler = info.backend != wgpu::Backend::Dx12 || DXC.load(std::sync::atomic::Ordering::Relaxed);
        let rays = compiler && adapter.features().contains(wgpu::Features::EXPERIMENTAL_RAY_QUERY) && crate::diag::diagnostic_setting("AYLIS_3D_NO_RAYS").is_none();
        let mut limits = wgpu::Limits::downlevel_webgl2_defaults().using_resolution(adapter.limits());
        if rays {
            limits = limits.using_minimum_supported_acceleration_structure_values();
            limits.max_bind_groups = limits.max_bind_groups.max(5);
            limits.max_storage_buffers_per_shader_stage = limits.max_storage_buffers_per_shader_stage.max(1);
            limits.max_storage_buffer_binding_size = limits.max_storage_buffer_binding_size.max(1 << 20);
        }
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("aylis-3d"),
                required_features: if rays { wgpu::Features::EXPERIMENTAL_RAY_QUERY } else { wgpu::Features::empty() },
                required_limits: limits,
                // SAFETY: o ray query do wgpu ainda é experimental; só é pedido onde a placa
                // diz ter, e o resto do desenho não depende dele.
                experimental_features: if rays { unsafe { wgpu::ExperimentalFeatures::enabled() } } else { wgpu::ExperimentalFeatures::disabled() },
                ..Default::default()
            })
            .await
        .map_err(|e| {
            trace(&format!("request_device falhou: {e}"));
            format!("GPU: {e}")
        })?;
        #[cfg(windows)]
        let luid = crate::d3d12::adapter_luid(&adapter);
        #[cfg(not(windows))]
        let luid = None;

        let uniform = |binding: u32, vis: wgpu::ShaderStages, dynamic: bool| wgpu::BindGroupLayoutEntry {
            binding,
            visibility: vis,
            ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: dynamic, min_binding_size: None },
            count: None,
        };
        let both = wgpu::ShaderStages::VERTEX_FRAGMENT;
        let texture = |binding: u32, sample_type: wgpu::TextureSampleType| wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Texture { sample_type, view_dimension: wgpu::TextureViewDimension::D2, multisampled: false },
            count: None,
        };
        let sampler_entry = |binding: u32, kind: wgpu::SamplerBindingType| wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Sampler(kind),
            count: None,
        };
        let color = wgpu::TextureSampleType::Float { filterable: true };
        let camera_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("câmera"),
            entries: &[uniform(0, both, false), texture(1, wgpu::TextureSampleType::Depth), sampler_entry(2, wgpu::SamplerBindingType::Comparison)],
        });
        let shadow_camera_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: Some("câmera da sombra"), entries: &[uniform(0, both, false)] });
        let draw_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: Some("desenho"), entries: &[uniform(0, both, true)] });
        // Cor, amostrador, normal, metal/aspereza, oclusão, emissão.
        let material_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("material"),
            entries: &[
                texture(0, color),
                sampler_entry(1, wgpu::SamplerBindingType::Filtering),
                texture(2, color),
                texture(3, color),
                texture(4, color),
                texture(5, color),
            ],
        });
        let joints_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: Some("ossos"), entries: &[uniform(0, wgpu::ShaderStages::VERTEX, false)] });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("aylis-3d"),
            bind_group_layouts: &[Some(&camera_layout), Some(&draw_layout), Some(&material_layout), Some(&joints_layout)],
            immediate_size: 0,
        });
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("aylis-3d"), source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()) });
        let rays = if rays { Rays::new(&device, [&camera_layout, &draw_layout, &material_layout, &joints_layout]) } else { None };
        let accum_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("accumulate"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture { sample_type: wgpu::TextureSampleType::Float { filterable: false }, view_dimension: wgpu::TextureViewDimension::D2, multisampled: false },
                count: None,
            }],
        });
        let accum_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("accumulate"), source: wgpu::ShaderSource::Wgsl(include_str!("accumulate.wgsl").into()) });
        // O brilho em 16 bits só onde a placa desenha, mistura, filtra e faz MSAA nele.
        let half = adapter.get_texture_format_features(wgpu::TextureFormat::Rgba16Float);
        let needs = wgpu::TextureFormatFeatureFlags::FILTERABLE
            | wgpu::TextureFormatFeatureFlags::BLENDABLE
            | wgpu::TextureFormatFeatureFlags::MULTISAMPLE_X4
            | wgpu::TextureFormatFeatureFlags::MULTISAMPLE_RESOLVE;
        let glow_format = if half.allowed_usages.contains(wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING) && half.flags.contains(needs) {
            wgpu::TextureFormat::Rgba16Float
        } else {
            wgpu::TextureFormat::Rgba8Unorm
        };
        let bloom_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("brilho"),
            entries: &[texture(0, color), sampler_entry(1, wgpu::SamplerBindingType::Filtering)],
        });
        let bloom_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("brilho"), source: wgpu::ShaderSource::Wgsl(include_str!("bloom.wgsl").into()) });
        let bloom_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("brilho"),
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::MipmapFilterMode::Linear,
            ..Default::default()
        });
        let shadow_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("sombra"),
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            compare: Some(wgpu::CompareFunction::LessEqual),
            ..Default::default()
        });
        let white = upload_texture(&device, &queue, 1, 1, &[255, 255, 255, 255], true);
        let white_linear = upload_texture(&device, &queue, 1, 1, &[255, 255, 255, 255], false);
        let flat_normal = upload_texture(&device, &queue, 1, 1, &[128, 128, 255, 255], false);
        let no_shadow = device
            .create_texture(&wgpu::TextureDescriptor {
                label: Some("sem sombra"),
                size: wgpu::Extent3d { width: 1, height: 1, depth_or_array_layers: 1 },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: DEPTH,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[],
            })
            .create_view(&Default::default());
        // Limpa uma vez: uma textura nunca escrita o wgpu zera antes de cada uso.
        {
            let mut encoder = device.create_command_encoder(&Default::default());
            encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("sem sombra"),
                color_attachments: &[],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &no_shadow,
                    depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(1.0), store: wgpu::StoreOp::Store }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            queue.submit([encoder.finish()]);
        }
        let joints = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("sem ossos"),
            contents: bytemuck::cast_slice(&[Mat4::IDENTITY.to_cols_array(); MAX_JOINTS]),
            usage: wgpu::BufferUsages::UNIFORM,
        });
        let no_joints = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &joints_layout,
            entries: &[wgpu::BindGroupEntry { binding: 0, resource: joints.as_entire_binding() }],
        });
        #[cfg(windows)]
        let interop = crate::d3d12::Interop::new(&device);
        let mut engine = Engine {
            device,
            queue,
            info,
            luid,
            camera_layout,
            draw_layout,
            material_layout,
            joints_layout,
            pipeline_layout,
            shader,
            pipelines: HashMap::new(),
            rays,
            accum_layout,
            accum_shader,
            accum_pipelines: HashMap::new(),
            glow_format,
            bloom_layout,
            bloom_shader,
            bloom_pipelines: HashMap::new(),
            bloom_sampler,
            shadow_camera_layout,
            shadow_pipeline: None,
            sampler,
            shadow_sampler,
            white,
            white_linear,
            flat_normal,
            no_shadow,
            no_joints,
            #[cfg(windows)]
            interop,
            gl: false,
            #[cfg(all(feature = "gl", not(target_arch = "wasm32"), not(target_vendor = "apple")))]
            gl_state: None,
            #[cfg(target_arch = "wasm32")]
            web: None,
            #[cfg(target_arch = "wasm32")]
            web_format: None,
            #[cfg(target_arch = "wasm32")]
            web_gl: None,
            #[cfg(target_arch = "wasm32")]
            web_is_gl: false,
            gl_loader: 0,
            pending: Vec::new(),
            next_id: 1,
        };
        engine.check_rays();
        Ok(engine)
    }

    /// A placa traça raios: o `Frame::ray_traced` vale.
    pub fn ray_tracing(&self) -> bool {
        self.rays.is_some()
    }

    /// No Direct3D 12 o shader só compila ao montar o pipeline: monta um com raios e, se
    /// falhar (compilador DXC antigo, driver), desliga o traçado.
    fn check_rays(&mut self) {
        if self.rays.is_none() {
            return;
        }
        let internal = self.device.push_error_scope(wgpu::ErrorFilter::Internal);
        let validation = self.device.push_error_scope(wgpu::ErrorFilter::Validation);
        let _ = self.pipeline(false, false, Variant { rays: true, ..Default::default() }, COLOR);
        let errors = [pollster::block_on(validation.pop()), pollster::block_on(internal.pop())];
        if let Some(e) = errors.into_iter().flatten().next() {
            trace(&format!("sem traçado de raios: {e}"));
            self.rays = None;
            self.pipelines.clear();
        }
    }

    /// Marca a cerca de liberação pela CPU, como se o aylis tivesse lido a textura (só
    /// para testes sem um renderizador do outro lado).
    #[doc(hidden)]
    pub fn release_on_cpu(&self, value: u64) {
        #[cfg(windows)]
        if let Some(i) = &self.interop {
            i.release_on_cpu(value);
        }
        let _ = value;
    }

    /// Envia de uma vez os comandos gravados neste quadro (no OpenGL, com a textura dividida;
    /// o `Model3D` pede isto para o fim do quadro, antes do flush do canvas). `true` se havia o
    /// que enviar.
    pub fn submit_pending(&mut self) -> bool {
        if self.pending.is_empty() {
            return false;
        }
        #[cfg(all(feature = "gl", not(target_arch = "wasm32"), not(target_vendor = "apple")))]
        if self.gl {
            clear_gl_errors(unsafe { std::mem::transmute::<usize, GlLoader>(self.gl_loader) });
            if let Some(st) = &self.gl_state {
                st.take_from_skia();
            }
        }
        self.queue.submit(self.pending.drain(..));
        #[cfg(all(feature = "gl", not(target_arch = "wasm32"), not(target_vendor = "apple")))]
        if let Some(st) = &self.gl_state {
            st.hand_to_skia();
        }
        true
    }

    /// O que fica do alvo com multiamostragem depois de resolvido. Descartar é o mais rápido,
    /// mas no OpenGL ES o wgpu resolve com um blit no fim da passada e há drivers (a Adreno 512
    /// do ASUS ZenFone 5, Android 9) que descartam antes: a imagem saía vazia, sem erro.
    fn msaa_store(&self) -> wgpu::StoreOp {
        // Pelo backend (e não só pelo contexto dividido): vale também para o WebGL do navegador.
        if self.info.backend == wgpu::Backend::Gl { wgpu::StoreOp::Store } else { wgpu::StoreOp::Discard }
    }

    /// Formato das texturas de saída: RGBA no OpenGL (o que o Skia lê), BGRA nos outros.
    fn color_format(&self) -> wgpu::TextureFormat {
        #[cfg(target_arch = "wasm32")]
        if let Some(f) = self.web_format {
            return f;
        }
        if self.gl { wgpu::TextureFormat::Rgba8Unorm } else { COLOR }
    }

    /// Há como entregar o quadro sem cópia ao renderizador do aylis.
    pub fn can_share(&self) -> bool {
        #[cfg(windows)]
        if self.interop.is_some() {
            return true;
        }
        #[cfg(target_arch = "wasm32")]
        if self.web.is_some() {
            return true;
        }
        self.gl
    }

    fn pipeline(&mut self, blend: bool, double_sided: bool, variant: Variant, format: wgpu::TextureFormat) -> wgpu::RenderPipeline {
        if let Some(p) = self.pipelines.get(&(blend, double_sided, variant, format)) {
            return p.clone();
        }
        let flag = |on: bool| if on { 1.0 } else { 0.0 };
        let constants = [("MAPS", flag(variant.maps)), ("SHADOW", flag(variant.shadow)), ("LIGHTS", flag(variant.lights))];
        let attrs = wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x2, 3 => Uint32x4, 4 => Float32x4, 5 => Float32x4];
        let (layout, shader) = match (&self.rays, variant.rays) {
            (Some(r), true) => (&r.pipeline_layout, &r.shader),
            _ => (&self.pipeline_layout, &self.shader),
        };
        let p = self.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("aylis-3d"),
            layout: Some(layout),
            vertex: wgpu::VertexState {
                module: shader,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &[Some(wgpu::VertexBufferLayout { array_stride: std::mem::size_of::<Vertex>() as u64, step_mode: wgpu::VertexStepMode::Vertex, attributes: &attrs })],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: if double_sided { None } else { Some(wgpu::Face::Back) },
                ..Default::default()
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: DEPTH,
                depth_write_enabled: Some(!blend),
                depth_compare: Some(wgpu::CompareFunction::Less),
                stencil: Default::default(),
                bias: Default::default(),
            }),
            multisample: wgpu::MultisampleState { count: SAMPLES, ..Default::default() },
            fragment: Some(wgpu::FragmentState {
                module: shader,
                entry_point: Some(if variant.glow { "fs_glow" } else { "fs_main" }),
                compilation_options: wgpu::PipelineCompilationOptions { constants: &constants, ..Default::default() },
                targets: &[
                    Some(wgpu::ColorTargetState { format, blend: blend.then_some(wgpu::BlendState::PREMULTIPLIED_ALPHA_BLENDING), write_mask: wgpu::ColorWrites::ALL }),
                    // A emissão, para o brilho.
                    variant.glow.then(|| wgpu::ColorTargetState {
                        format: self.glow_format,
                        blend: blend.then_some(wgpu::BlendState::PREMULTIPLIED_ALPHA_BLENDING),
                        write_mask: wgpu::ColorWrites::ALL,
                    }),
                ][..if variant.glow { 2 } else { 1 }],
            }),
            multiview_mask: None,
            cache: None,
        });
        self.pipelines.insert((blend, double_sided, variant, format), p.clone());
        p
    }

    /// O pipeline do passe da sombra: só o vértice (o mesmo do desenho), sem cor e sem
    /// descartar faces (peças finas também fazem sombra).
    fn shadow_pipeline(&mut self) -> wgpu::RenderPipeline {
        if let Some(p) = &self.shadow_pipeline {
            return p.clone();
        }
        let layout = self.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("sombra"),
            bind_group_layouts: &[Some(&self.shadow_camera_layout), Some(&self.draw_layout), None, Some(&self.joints_layout)],
            immediate_size: 0,
        });
        let attrs = wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x2, 3 => Uint32x4, 4 => Float32x4, 5 => Float32x4];
        let p = self.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("sombra"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &self.shader,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &[Some(wgpu::VertexBufferLayout { array_stride: std::mem::size_of::<Vertex>() as u64, step_mode: wgpu::VertexStepMode::Vertex, attributes: &attrs })],
            },
            primitive: wgpu::PrimitiveState { topology: wgpu::PrimitiveTopology::TriangleList, cull_mode: None, ..Default::default() },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: DEPTH,
                depth_write_enabled: Some(true),
                depth_compare: Some(wgpu::CompareFunction::Less),
                stencil: Default::default(),
                bias: Default::default(),
            }),
            multisample: Default::default(),
            fragment: None,
            multiview_mask: None,
            cache: None,
        });
        self.shadow_pipeline = Some(p.clone());
        p
    }

    /// Envia o modelo à GPU (uma vez por modelo).
    pub fn upload(&self, model: &Model) -> GpuModel {
        // A mesma imagem pode ir como cor (sRGB) e como dado (linear): uma cópia de cada.
        let mut uploaded: HashMap<(usize, bool), wgpu::TextureView> = HashMap::new();
        let mut texture = |index: Option<usize>, srgb: bool| -> Option<wgpu::TextureView> {
            let t = model.textures.get(index?)?;
            Some(uploaded.entry((index?, srgb)).or_insert_with(|| upload_texture(&self.device, &self.queue, t.width, t.height, &t.rgba, srgb)).clone())
        };
        let materials = model
            .materials
            .iter()
            .map(|m| {
                let views = [
                    texture(m.texture, true).unwrap_or_else(|| self.white.clone()),
                    texture(m.normal_texture, false).unwrap_or_else(|| self.flat_normal.clone()),
                    texture(m.metallic_roughness_texture, false).unwrap_or_else(|| self.white_linear.clone()),
                    texture(m.occlusion_texture, false).unwrap_or_else(|| self.white_linear.clone()),
                    texture(m.emissive_texture, true).unwrap_or_else(|| self.white.clone()),
                ];
                self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("material"),
                    layout: &self.material_layout,
                    entries: &[
                        wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&views[0]) },
                        wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(&self.sampler) },
                        wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::TextureView(&views[1]) },
                        wgpu::BindGroupEntry { binding: 3, resource: wgpu::BindingResource::TextureView(&views[2]) },
                        wgpu::BindGroupEntry { binding: 4, resource: wgpu::BindingResource::TextureView(&views[3]) },
                        wgpu::BindGroupEntry { binding: 5, resource: wgpu::BindingResource::TextureView(&views[4]) },
                    ],
                })
            })
            .collect();
        let meshes = model
            .meshes
            .iter()
            .map(|mesh| {
                mesh.primitives
                    .iter()
                    .map(|p| {
                        let morph = !p.targets.is_empty();
                        // Com o traçado de raios, os mesmos buffers montam a cena.
                        let rays = if self.rays.is_some() { wgpu::BufferUsages::BLAS_INPUT } else { wgpu::BufferUsages::empty() };
                        let vertex = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                            label: Some("vértices"),
                            contents: bytemuck::cast_slice(&p.vertices),
                            usage: wgpu::BufferUsages::VERTEX | rays | if morph { wgpu::BufferUsages::COPY_DST } else { wgpu::BufferUsages::empty() },
                        });
                        let index = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                            label: Some("índices"),
                            contents: bytemuck::cast_slice(&p.indices),
                            usage: wgpu::BufferUsages::INDEX | rays,
                        });
                        GpuPrimitive {
                            vertex,
                            index,
                            count: p.indices.len() as u32,
                            vertices: p.vertices.len() as u32,
                            material: p.material,
                            weights: Vec::new(),
                            scratch: Vec::new(),
                            blas: None,
                            positions: None,
                            shape_changed: false,
                        }
                    })
                    .collect()
            })
            .collect();
        GpuModel { meshes, materials }
    }

    /// Desenha um quadro de `model` na pose dada (`world`: matriz de cada nó; `weights`:
    /// pesos dos morph targets de cada nó). `share`: entregar como textura na GPU.
    #[allow(clippy::too_many_arguments)]
    pub fn render(&mut self, model: &Model, gpu: &mut GpuModel, view: &mut View, world: &[Mat4], weights: &[Vec<f32>], frame: &Frame, share: bool) -> Option<Output> {
        let (w, h) = (frame.width.max(1), frame.height.max(1));
        let share = share && self.can_share() && !view.share_failed;
        // Um erro antigo do OpenGL (de quem desenhou antes) pareceria um erro do wgpu; e o
        // VAO dele precisa estar ligado. Com a textura dividida o envio é no fim do quadro, e
        // isso fica para lá ([`Engine::submit_pending`]).
        #[cfg(all(feature = "gl", not(target_arch = "wasm32"), not(target_vendor = "apple")))]
        if self.gl && !share {
            clear_gl_errors(unsafe { std::mem::transmute::<usize, GlLoader>(self.gl_loader) });
            if let Some(st) = &self.gl_state {
                st.take_from_skia();
            }
        }
        #[cfg(target_arch = "wasm32")]
        if share && self.web.is_some() {
            return self.render_web(model, gpu, view, world, weights, frame);
        }
        if view.size != (w, h) || view.shared != share || view.targets.is_empty() {
            self.resize_view(view, w, h, share)?;
        }
        let target = self.free_target(view, w, h);
        let resolve = view.targets[target].view.clone();
        trace("encode");
        let encoder = self.encode(model, gpu, view, world, weights, frame, &resolve);
        trace("finish");
        let out = self.finish(view, target, encoder, w, h, share);
        trace("pronto");
        out
    }

    /// Desenha direto numa textura (a de uma janela, por exemplo), no formato BGRA.
    #[allow(clippy::too_many_arguments)]
    pub fn render_to(&mut self, model: &Model, gpu: &mut GpuModel, view: &mut View, world: &[Mat4], weights: &[Vec<f32>], frame: &Frame, target: &wgpu::TextureView) {
        let (w, h) = (frame.width.max(1), frame.height.max(1));
        if view.size != (w, h) || view.targets.is_empty() {
            let _ = self.resize_view(view, w, h, false);
        }
        let encoder = self.encode(model, gpu, view, world, weights, frame, target);
        self.queue.submit([encoder.finish()]);
    }

    /// Prepara os buffers do quadro e grava o passe (MSAA resolvido em `resolve`).
    #[allow(clippy::too_many_arguments)]
    fn encode(&mut self, model: &Model, gpu: &mut GpuModel, view: &mut View, world: &[Mat4], weights: &[Vec<f32>], frame: &Frame, resolve: &wgpu::TextureView) -> wgpu::CommandEncoder {
        // Luzes: as do arquivo (presas aos nós) ou a padrão. A primeira, se for direcional,
        // faz a sombra.
        let (lights, count) = scene_lights(model, world, frame);
        // Com o traçado de raios, a sombra vem dos raios (de todas as luzes), não do mapa.
        let traced = frame.ray_traced && self.rays.is_some();
        let shadow_dir = (frame.shadows && !traced && count > 0 && lights[0].position[3] == 0.0).then(|| Vec3::from_slice(&lights[0].direction[..3]));
        let opaque = model.materials.iter().any(|m| !m.blend);
        let shadow = shadow_dir.filter(|_| opaque).map(|dir| shadow_matrix(model, dir));
        let radius = ((model.bounds.1 - model.bounds.0).length() / 2.0).max(1e-3);
        if shadow.is_some() && view.shadow_map.is_none() {
            self.add_shadow_map(view);
        }

        // Câmera.
        let (shadow_view_proj, texel_world) = shadow.unwrap_or((Mat4::IDENTITY, 0.0));
        let mut camera = CameraUniform {
            view_proj: (frame.proj * frame.view).to_cols_array_2d(),
            eye: frame.eye.extend(1.0).into(),
            sky: [0.62, 0.66, 0.75, 1.0],
            ground: [0.30, 0.27, 0.25, 1.0],
            counts: [count as f32, if shadow.is_some() { 1.0 } else { 0.0 }, 1.0 / SHADOW_SIZE as f32, texel_world],
            shadow_view_proj: shadow_view_proj.to_cols_array_2d(),
            lights,
            rays: [if traced { 1.0 } else { 0.0 }, frame.sample as f32, radius * 0.35, radius * 0.002],
            glow: [frame.glow / self.glow_range(), 0.0, 0.0, 0.0],
        };
        self.queue.write_buffer(&view.camera, 0, bytemuck::bytes_of(&camera));
        if shadow.is_some() {
            camera.view_proj = shadow_view_proj.to_cols_array_2d();
            self.queue.write_buffer(&view.shadow_camera, 0, bytemuck::bytes_of(&camera));
        }

        // Morph targets: mistura na CPU quando os pesos mudam.
        for (i, node) in model.nodes.iter().enumerate() {
            let Some(m) = node.mesh else { continue };
            let w = weights.get(i).map(Vec::as_slice).unwrap_or(&[]);
            for (p, gp) in model.meshes[m].primitives.iter().zip(gpu.meshes[m].iter_mut()) {
                if p.targets.is_empty() || gp.weights == w {
                    continue;
                }
                gp.weights = w.to_vec();
                gp.scratch.clear();
                gp.scratch.extend_from_slice(&p.vertices);
                for (t, &wt) in p.targets.iter().zip(w) {
                    if wt == 0.0 {
                        continue;
                    }
                    for (v, (dp, dn)) in gp.scratch.iter_mut().zip(t.positions.iter().zip(&t.normals)) {
                        for k in 0..3 {
                            v.position[k] += dp[k] * wt;
                            v.normal[k] += dn[k] * wt;
                        }
                    }
                }
                self.queue.write_buffer(&gp.vertex, 0, bytemuck::cast_slice(&gp.scratch));
                gp.shape_changed = true;
            }
        }

        // Ossos de cada esqueleto.
        while view.joints.len() < model.skins.len() {
            let buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("ossos"),
                size: (MAX_JOINTS * 64) as u64,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
            let group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &self.joints_layout,
                entries: &[wgpu::BindGroupEntry { binding: 0, resource: buffer.as_entire_binding() }],
            });
            view.joints.push((buffer, group));
        }
        for (s, (buffer, _)) in model.skins.iter().enumerate().map(|(s, _)| s).zip(&view.joints) {
            let m: Vec<[f32; 16]> = model.joint_matrices(s, world).iter().take(MAX_JOINTS).map(|m| m.to_cols_array()).collect();
            self.queue.write_buffer(buffer, 0, bytemuck::cast_slice(&m));
        }

        // Lista de desenhos: opacos primeiro, transparentes de trás para a frente.
        struct Item {
            mesh: usize,
            prim: usize,
            skin: Option<usize>,
            material: Material,
            depth: f32,
            uniform: DrawUniform,
        }
        let mut items = Vec::new();
        for (i, node) in model.nodes.iter().enumerate() {
            let Some(m) = node.mesh else { continue };
            for (pi, p) in model.meshes[m].primitives.iter().enumerate() {
                let mat = model.materials.get(p.material).cloned().unwrap_or_default();
                let skin = node.skin.filter(|&s| s < model.skins.len());
                let world_m = if skin.is_some() { Mat4::IDENTITY } else { world[i] };
                let has = |t: Option<usize>| t.is_some_and(|t| t < model.textures.len());
                let uniform = DrawUniform {
                    world: world_m.to_cols_array_2d(),
                    normal: world_m.inverse().transpose().to_cols_array_2d(),
                    base_color: mat.base_color,
                    params: [mat.metallic, mat.roughness, if skin.is_some() { 1.0 } else { 0.0 }, if has(mat.texture) { 1.0 } else { 0.0 }],
                    emissive: [mat.emissive[0], mat.emissive[1], mat.emissive[2], if mat.blend { 1.0 } else { 0.0 }],
                    maps: [
                        if has(mat.normal_texture) { mat.normal_scale } else { 0.0 },
                        if has(mat.occlusion_texture) { mat.occlusion_strength } else { 0.0 },
                        if has(mat.metallic_roughness_texture) { 1.0 } else { 0.0 },
                        if has(mat.emissive_texture) { 1.0 } else { 0.0 },
                    ],
                    coat: [mat.clearcoat.clamp(0.0, 1.0), mat.clearcoat_roughness.clamp(0.0, 1.0), 0.0, 0.0],
                };
                let center = world[i].transform_point3(Vec3::ZERO);
                items.push(Item { mesh: m, prim: pi, skin, material: mat, depth: (frame.view.transform_point3(center)).z, uniform });
            }
        }
        items.sort_by(|a, b| a.material.blend.cmp(&b.material.blend).then(if a.material.blend { a.depth.total_cmp(&b.depth) } else { std::cmp::Ordering::Equal }));
        let needed = (items.len().max(1) as u64) * DRAW_STRIDE;
        if view.draw_capacity < needed {
            let (buffer, group) = self.draw_buffer(needed.next_power_of_two());
            view.draws = buffer;
            view.draw_group = group;
            view.draw_capacity = needed.next_power_of_two();
        }
        let mut bytes = vec![0u8; needed as usize];
        for (k, it) in items.iter().enumerate() {
            let at = k * DRAW_STRIDE as usize;
            bytes[at..at + std::mem::size_of::<DrawUniform>()].copy_from_slice(bytemuck::bytes_of(&it.uniform));
        }
        self.queue.write_buffer(&view.draws, 0, &bytes);

        // O passe: MSAA, resolvido na textura de saída.
        // No navegador, o do canvas da vista (BGRA no WebGPU, RGBA no WebGL), que é o do alvo
        // com multiamostragem (`render_web`).
        #[cfg(target_arch = "wasm32")]
        let web = self.web.is_some();
        #[cfg(not(target_arch = "wasm32"))]
        let web = false;
        let format = if web || (self.gl && view.size != (0, 0) && resolve_is_view_target(view, resolve)) { self.color_format() } else { COLOR };
        let mut encoder = self.device.create_command_encoder(&Default::default());

        // Traçado de raios: a cena do quadro, uma instância por desenho (o índice dele é o da
        // cor que o reflexo encontra).
        let scene: Vec<SceneItem> = items
            .iter()
            .map(|it| SceneItem {
                mesh: it.mesh,
                prim: it.prim,
                skin: it.skin,
                transform: Mat4::from_cols_array_2d(&it.uniform.world),
                colors: [
                    [it.material.base_color[0], it.material.base_color[1], it.material.base_color[2], it.material.metallic],
                    [it.material.emissive[0], it.material.emissive[1], it.material.emissive[2], it.material.roughness],
                ],
                blend: it.material.blend,
            })
            .collect();
        let rays_group = if traced { self.build_scene(model, gpu, view, world, &scene, &mut encoder) } else { None };
        let traced = rays_group.is_some();
        // Acumulando (a cena parada): o quadro sai numa textura à parte e entra na média.
        let accumulate = traced && frame.sample > 0;
        if accumulate {
            self.accum_targets(view);
        }
        let fresh = if accumulate { view.fresh.clone() } else { None };
        // Brilho: a emissão sai também numa textura à parte, que o bloom espalha no fim.
        let glow = frame.glow > 0.0;
        if glow {
            self.glow_targets(view);
        }
        let glow_out = if glow { view.glow_msaa.clone().zip(view.glow_levels.first().cloned()) } else { None };

        // Antes, a sombra: a profundidade dos opacos vista da luz.
        if let (Some(_), Some(map)) = (shadow, view.shadow_map.clone()) {
            let pipeline = self.shadow_pipeline();
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("sombra"),
                color_attachments: &[],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &map,
                    depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(1.0), store: wgpu::StoreOp::Store }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            pass.set_pipeline(&pipeline);
            pass.set_bind_group(0, &view.shadow_camera_group, &[]);
            for (k, it) in items.iter().enumerate() {
                if it.material.blend {
                    continue;
                }
                let gp = &gpu.meshes[it.mesh][it.prim];
                pass.set_bind_group(1, &view.draw_group, &[(k as u64 * DRAW_STRIDE) as u32]);
                match it.skin {
                    Some(s) => pass.set_bind_group(3, &view.joints[s].1, &[]),
                    None => pass.set_bind_group(3, &self.no_joints, &[]),
                }
                pass.set_vertex_buffer(0, gp.vertex.slice(..));
                pass.set_index_buffer(gp.index.slice(..), wgpu::IndexFormat::Uint32);
                pass.draw_indexed(0..gp.count, 0, 0..1);
            }
        }
        {
            let bg = frame.background;
            let msaa_store = self.msaa_store();
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("aylis-3d"),
                color_attachments: &[
                    Some(wgpu::RenderPassColorAttachment {
                        view: &view.msaa,
                        depth_slice: None,
                        resolve_target: Some(fresh.as_ref().unwrap_or(resolve)),
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color { r: (bg[0] * bg[3]) as f64, g: (bg[1] * bg[3]) as f64, b: (bg[2] * bg[3]) as f64, a: bg[3] as f64 }),
                            store: msaa_store,
                        },
                    }),
                    glow_out.as_ref().map(|(msaa, level)| wgpu::RenderPassColorAttachment {
                        view: msaa,
                        depth_slice: None,
                        resolve_target: Some(level),
                        ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT), store: msaa_store },
                    }),
                ][..if glow_out.is_some() { 2 } else { 1 }],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &view.depth,
                    depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(1.0), store: wgpu::StoreOp::Discard }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            pass.set_bind_group(0, &view.camera_group, &[]);
            if let Some(group) = &rays_group {
                pass.set_bind_group(4, group, &[]);
            }
            let mut current: Option<(bool, bool, Variant)> = None;
            for (k, it) in items.iter().enumerate() {
                let m = &it.material;
                let has = |t: Option<usize>| t.is_some_and(|t| t < model.textures.len());
                let variant = Variant {
                    maps: has(m.normal_texture) || has(m.occlusion_texture) || has(m.metallic_roughness_texture) || has(m.emissive_texture),
                    shadow: shadow.is_some(),
                    lights: count > 1,
                    rays: traced,
                    glow,
                };
                let key = (m.blend, m.double_sided, variant);
                if current != Some(key) {
                    let p = self.pipeline(key.0, key.1, key.2, format);
                    pass.set_pipeline(&p);
                    current = Some(key);
                }
                let gp = &gpu.meshes[it.mesh][it.prim];
                pass.set_bind_group(1, &view.draw_group, &[(k as u64 * DRAW_STRIDE) as u32]);
                pass.set_bind_group(2, gpu.materials.get(gp.material).unwrap_or(&gpu.materials[0]), &[]);
                match it.skin {
                    Some(s) => pass.set_bind_group(3, &view.joints[s].1, &[]),
                    None => pass.set_bind_group(3, &self.no_joints, &[]),
                }
                pass.set_vertex_buffer(0, gp.vertex.slice(..));
                pass.set_index_buffer(gp.index.slice(..), wgpu::IndexFormat::Uint32);
                pass.draw_indexed(0..gp.count, 0, 0..1);
            }
        }
        if let (Some(fresh), Some(accum)) = (fresh, view.accum.clone()) {
            // A amostra nova entra na média com peso 1/n; a média vai para a saída.
            let weight = 1.0 / frame.sample.max(1) as f64;
            self.copy_pass(&mut encoder, &fresh, &accum, ACCUM, Some(weight));
            self.copy_pass(&mut encoder, &accum, resolve, format, None);
        }
        if glow_out.is_some() {
            self.bloom(&mut encoder, view, resolve, format);
        }
        encoder
    }

    /// Quanto a textura do brilho guarda a menos (8 bits: 1/4 da luz, para não estourar).
    fn glow_range(&self) -> f32 {
        if self.glow_format == wgpu::TextureFormat::Rgba16Float { 1.0 } else { GLOW_RANGE_8BIT }
    }

    /// As texturas do brilho (uma vez por tamanho): a emissão com MSAA e os níveis, do
    /// tamanho do quadro até 1/32 (ou até ficar pequeno demais).
    fn glow_targets(&self, view: &mut View) {
        if view.glow_msaa.is_some() && !view.glow_levels.is_empty() {
            return;
        }
        let make = |w: u32, h: u32, samples: u32, usage: wgpu::TextureUsages| {
            self.device
                .create_texture(&wgpu::TextureDescriptor {
                    label: Some("brilho"),
                    size: wgpu::Extent3d { width: w.max(1), height: h.max(1), depth_or_array_layers: 1 },
                    mip_level_count: 1,
                    sample_count: samples,
                    dimension: wgpu::TextureDimension::D2,
                    format: self.glow_format,
                    usage,
                    view_formats: &[],
                })
                .create_view(&Default::default())
        };
        let (w, h) = view.size;
        view.glow_msaa = Some(make(w, h, SAMPLES, wgpu::TextureUsages::RENDER_ATTACHMENT));
        let usage = wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING;
        // Até `GLOW_MIN` pixels: num 3D pequeno os níveis de 2 a 4 pixels quase não mudam a imagem e
        // cada um são duas passadas (no celular, 36 lingotes pequenos enviavam em 47 ms; um
        // grande, com todos os níveis acima disso, fica igual).
        view.glow_levels = (0..=GLOW_LEVELS).map(|k| (w >> k, h >> k)).take_while(|&(lw, lh)| (lw >= GLOW_MIN && lh >= GLOW_MIN) || (lw == w && lh == h)).map(|(lw, lh)| make(lw, lh, 1, usage)).collect();
        view.glow_groups = view
            .glow_levels
            .iter()
            .map(|source| {
                self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("brilho"),
                    layout: &self.bloom_layout,
                    entries: &[
                        wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(source) },
                        wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(&self.bloom_sampler) },
                    ],
                })
            })
            .collect();
    }

    /// O brilho: desce a emissão nível a nível, sobe somando (os níveis mais fundos pesam
    /// menos) e soma a luz espalhada por cima do quadro em `target`.
    fn bloom(&mut self, encoder: &mut wgpu::CommandEncoder, view: &View, target: &wgpu::TextureView, format: wgpu::TextureFormat) {
        let levels = &view.glow_levels;
        if levels.len() < 2 {
            return;
        }
        for k in 1..levels.len() {
            self.bloom_pass(encoder, Bloom::Down, &view.glow_groups[k - 1], &levels[k], self.glow_format, None);
        }
        for k in (1..levels.len() - 1).rev() {
            self.bloom_pass(encoder, Bloom::Up, &view.glow_groups[k + 1], &levels[k], self.glow_format, Some(GLOW_FALLOFF));
        }
        self.bloom_pass(encoder, Bloom::Composite, &view.glow_groups[1], target, format, Some(1.0));
    }

    /// Um passe do brilho de `source` para `target`; com `weight`, soma por cima do que está
    /// lá (com esse peso), senão substitui.
    fn bloom_pass(&mut self, encoder: &mut wgpu::CommandEncoder, kind: Bloom, group: &wgpu::BindGroup, target: &wgpu::TextureView, format: wgpu::TextureFormat, weight: Option<f64>) {
        let pipeline = self.bloom_pipeline(kind, format);
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("brilho"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: target,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations { load: if weight.is_some() { wgpu::LoadOp::Load } else { wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT) }, store: wgpu::StoreOp::Store },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, group, &[]);
        if let Some(w) = weight {
            pass.set_blend_constant(wgpu::Color { r: w, g: w, b: w, a: w });
        }
        pass.draw(0..3, 0..1);
    }

    fn bloom_pipeline(&mut self, kind: Bloom, format: wgpu::TextureFormat) -> wgpu::RenderPipeline {
        if let Some(p) = self.bloom_pipelines.get(&(kind, format)) {
            return p.clone();
        }
        let layout = self.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("brilho"),
            bind_group_layouts: &[Some(&self.bloom_layout)],
            immediate_size: 0,
        });
        // Descer substitui; subir e compor somam por cima (com o peso da constante).
        let add = wgpu::BlendComponent { src_factor: wgpu::BlendFactor::Constant, dst_factor: wgpu::BlendFactor::One, operation: wgpu::BlendOperation::Add };
        let (entry, blend) = match kind {
            Bloom::Down => ("fs_down", None),
            Bloom::Up => ("fs_up", Some(wgpu::BlendState { color: add, alpha: add })),
            Bloom::Composite => ("fs_composite", Some(wgpu::BlendState { color: add, alpha: add })),
        };
        let range = [("RANGE", self.glow_range() as f64)];
        let p = self.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("brilho"),
            layout: Some(&layout),
            vertex: wgpu::VertexState { module: &self.bloom_shader, entry_point: Some("vs_main"), compilation_options: Default::default(), buffers: &[] },
            primitive: Default::default(),
            depth_stencil: None,
            multisample: Default::default(),
            fragment: Some(wgpu::FragmentState {
                module: &self.bloom_shader,
                entry_point: Some(entry),
                compilation_options: wgpu::PipelineCompilationOptions { constants: &range, ..Default::default() },
                targets: &[Some(wgpu::ColorTargetState { format, blend, write_mask: wgpu::ColorWrites::ALL })],
            }),
            multiview_mask: None,
            cache: None,
        });
        self.bloom_pipelines.insert((kind, format), p.clone());
        p
    }

    /// Traçado de raios: monta a forma de cada primitivo (uma vez; de novo quando ela muda,
    /// com esqueleto ou morph targets), a cena com uma instância por desenho opaco e as cores
    /// delas, e devolve o grupo 4 do shader. `None` se não houver o que traçar.
    fn build_scene(&mut self, model: &Model, gpu: &mut GpuModel, view: &mut View, world: &[Mat4], scene: &[SceneItem], encoder: &mut wgpu::CommandEncoder) -> Option<wgpu::BindGroup> {
        let rays = self.rays.as_ref()?;
        let size_of = |gp: &GpuPrimitive| wgpu::BlasTriangleGeometrySizeDescriptor {
            vertex_format: wgpu::VertexFormat::Float32x3,
            vertex_count: gp.vertices,
            index_format: Some(wgpu::IndexFormat::Uint32),
            index_count: Some(gp.count),
            flags: wgpu::AccelerationStructureGeometryFlags::OPAQUE,
        };
        // O esqueleto anda na CPU para a cena: as posições já no mundo.
        let mut joints: HashMap<usize, Vec<Mat4>> = HashMap::new();
        for it in scene.iter().filter(|it| !it.blend) {
            let gp = &mut gpu.meshes[it.mesh][it.prim];
            if gp.count == 0 || gp.vertices == 0 {
                continue;
            }
            if let Some(s) = it.skin {
                let m = joints.entry(s).or_insert_with(|| model.joint_matrices(s, world));
                let source = if gp.scratch.is_empty() { &model.meshes[it.mesh].primitives[it.prim].vertices } else { &gp.scratch };
                let positions: Vec<[f32; 3]> = source
                    .iter()
                    .map(|v| {
                        let p = glam::Vec4::new(v.position[0], v.position[1], v.position[2], 1.0);
                        let mut out = glam::Vec4::ZERO;
                        for k in 0..4 {
                            if v.weights[k] != 0.0 {
                                out += m.get(v.joints[k] as usize).copied().unwrap_or(Mat4::IDENTITY) * p * v.weights[k];
                            }
                        }
                        [out.x, out.y, out.z]
                    })
                    .collect();
                let buffer = gp.positions.get_or_insert_with(|| {
                    self.device.create_buffer(&wgpu::BufferDescriptor {
                        label: Some("posições da cena"),
                        size: (positions.len() * 12) as u64,
                        usage: wgpu::BufferUsages::BLAS_INPUT | wgpu::BufferUsages::COPY_DST,
                        mapped_at_creation: false,
                    })
                });
                self.queue.write_buffer(buffer, 0, bytemuck::cast_slice(&positions));
                gp.shape_changed = true;
            }
            if gp.blas.is_none() {
                gp.blas = Some(self.device.create_blas(
                    &wgpu::CreateBlasDescriptor {
                        label: Some("forma"),
                        flags: wgpu::AccelerationStructureFlags::PREFER_FAST_TRACE,
                        update_mode: wgpu::AccelerationStructureUpdateMode::Build,
                    },
                    wgpu::BlasGeometrySizeDescriptors::Triangles { descriptors: vec![size_of(gp)] },
                ));
                gp.shape_changed = true;
            }
        }
        // As formas novas ou mudadas, montadas de novo.
        let mut sizes = Vec::new();
        let mut pending = Vec::new();
        for (mi, mesh) in gpu.meshes.iter_mut().enumerate() {
            for (pi, gp) in mesh.iter_mut().enumerate() {
                if gp.shape_changed && gp.blas.is_some() {
                    gp.shape_changed = false;
                    sizes.push(size_of(gp));
                    pending.push((mi, pi));
                }
            }
        }
        let entries: Vec<wgpu::BlasBuildEntry> = pending
            .iter()
            .zip(&sizes)
            .map(|(&(mi, pi), size)| {
                let gp = &gpu.meshes[mi][pi];
                let (buffer, stride) = match &gp.positions {
                    Some(p) => (p, 12),
                    None => (&gp.vertex, std::mem::size_of::<Vertex>() as u64),
                };
                wgpu::BlasBuildEntry {
                    blas: gp.blas.as_ref().expect("forma criada acima"),
                    geometry: wgpu::BlasGeometries::TriangleGeometries(vec![wgpu::BlasTriangleGeometry {
                        size,
                        vertex_buffer: buffer,
                        first_vertex: 0,
                        vertex_stride: stride,
                        index_buffer: Some(&gp.index),
                        first_index: Some(0),
                        transform_buffer: None,
                        transform_buffer_offset: None,
                    }]),
                }
            })
            .collect();
        // A cena: uma instância por desenho opaco (o número dela é o índice da cor).
        let count = scene.len().max(1) as u32;
        if view.tlas.as_ref().is_none_or(|t| t.1 < count) {
            let capacity = count.next_power_of_two();
            let tlas = self.device.create_tlas(&wgpu::CreateTlasDescriptor {
                label: Some("cena"),
                max_instances: capacity,
                flags: wgpu::AccelerationStructureFlags::PREFER_FAST_TRACE,
                update_mode: wgpu::AccelerationStructureUpdateMode::Build,
            });
            view.tlas = Some((tlas, capacity));
        }
        let (tlas, capacity) = view.tlas.as_mut()?;
        let mut any = false;
        for k in 0..*capacity as usize {
            tlas[k] = scene.get(k).filter(|it| !it.blend).and_then(|it| {
                let gp = &gpu.meshes[it.mesh][it.prim];
                let blas = gp.blas.as_ref()?;
                // Com esqueleto, as posições já estão no mundo.
                let transform = if gp.positions.is_some() { Mat4::IDENTITY } else { it.transform };
                Some(wgpu::TlasInstance::new(blas, rows_3x4(transform), k as u32, 0xff))
            });
            any |= tlas[k].is_some();
        }
        if !any {
            return None;
        }
        encoder.build_acceleration_structures(entries.iter(), std::iter::once(&*tlas));
        // As cores: duas por desenho.
        let colors: Vec<[f32; 4]> = scene.iter().flat_map(|it| it.colors).collect();
        let bytes = (colors.len().max(2) * 16) as u64;
        if view.hit_colors.as_ref().is_none_or(|b| b.size() < bytes) {
            view.hit_colors = Some(self.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("cores da cena"),
                size: bytes.next_power_of_two(),
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }));
        }
        let hit_colors = view.hit_colors.as_ref()?;
        if !colors.is_empty() {
            self.queue.write_buffer(hit_colors, 0, bytemuck::cast_slice(&colors));
        }
        Some(self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("raios"),
            layout: &rays.layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: tlas.as_binding() },
                wgpu::BindGroupEntry { binding: 1, resource: hit_colors.as_entire_binding() },
            ],
        }))
    }

    /// As texturas da acumulação no tamanho da vista: a do quadro novo e a da média.
    fn accum_targets(&self, view: &mut View) {
        if view.fresh.is_some() && view.accum.is_some() {
            return;
        }
        let (w, h) = view.size;
        let make = |format, label| {
            self.device
                .create_texture(&wgpu::TextureDescriptor {
                    label: Some(label),
                    size: wgpu::Extent3d { width: w.max(1), height: h.max(1), depth_or_array_layers: 1 },
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    format,
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
                    view_formats: &[],
                })
                .create_view(&Default::default())
        };
        view.fresh = Some(make(COLOR, "amostra"));
        view.accum = Some(make(ACCUM, "média das amostras"));
    }

    /// Copia `source` para `target` desenhando (formatos diferentes); com `weight`, mistura
    /// por cima do que está lá (peso da nova).
    fn copy_pass(&mut self, encoder: &mut wgpu::CommandEncoder, source: &wgpu::TextureView, target: &wgpu::TextureView, format: wgpu::TextureFormat, weight: Option<f64>) {
        let pipeline = self.accum_pipeline(weight.is_some(), format);
        let group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &self.accum_layout,
            entries: &[wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(source) }],
        });
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("accumulate"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: target,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations { load: if weight.is_some() { wgpu::LoadOp::Load } else { wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT) }, store: wgpu::StoreOp::Store },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &group, &[]);
        if let Some(w) = weight {
            pass.set_blend_constant(wgpu::Color { r: w, g: w, b: w, a: w });
        }
        pass.draw(0..3, 0..1);
    }

    fn accum_pipeline(&mut self, blend: bool, format: wgpu::TextureFormat) -> wgpu::RenderPipeline {
        if let Some(p) = self.accum_pipelines.get(&(blend, format)) {
            return p.clone();
        }
        let layout = self.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("accumulate"),
            bind_group_layouts: &[Some(&self.accum_layout)],
            immediate_size: 0,
        });
        let mix = wgpu::BlendComponent { src_factor: wgpu::BlendFactor::Constant, dst_factor: wgpu::BlendFactor::OneMinusConstant, operation: wgpu::BlendOperation::Add };
        let p = self.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("accumulate"),
            layout: Some(&layout),
            vertex: wgpu::VertexState { module: &self.accum_shader, entry_point: Some("vs_main"), compilation_options: Default::default(), buffers: &[] },
            primitive: Default::default(),
            depth_stencil: None,
            multisample: Default::default(),
            fragment: Some(wgpu::FragmentState {
                module: &self.accum_shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState { format, blend: blend.then_some(wgpu::BlendState { color: mix, alpha: mix }), write_mask: wgpu::ColorWrites::ALL })],
            }),
            multiview_mask: None,
            cache: None,
        });
        self.accum_pipelines.insert((blend, format), p.clone());
        p
    }

    /// Entrega o quadro: textura compartilhada ou pixels.
    fn finish(&mut self, view: &mut View, target: usize, mut encoder: wgpu::CommandEncoder, w: u32, h: u32, share: bool) -> Option<Output> {

        // OpenGL: a textura já é do contexto do canvas, e os comandos estão em ordem. O envio fica
        // para o fim do quadro, junto com o dos outros controles ([`Engine::submit_pending`]).
        #[cfg(all(feature = "gl", not(target_arch = "wasm32"), not(target_vendor = "apple")))]
        if share && self.gl {
            self.pending.push(encoder.finish());
            let t = &view.targets[target];
            let hal = unsafe { t.texture.as_hal::<wgpu_hal::api::Gles>()? };
            let wgpu_hal::gles::TextureInner::Texture { raw, .. } = &hal.inner else { return None };
            return Some(Output::Surface(GpuSurface::GlTexture { id: t.id, texture: raw.0.get(), width: w, height: h }));
        }

        #[cfg(windows)]
        if share {
            let interop = self.interop.as_mut()?;
            let t = &mut view.targets[target];
            let shared = t.shared.as_ref()?;
            let (ready, release) = interop.submit(&self.queue, t.release_wait);
            self.queue.submit([encoder.finish()]);
            t.release_wait = release;
            return Some(Output::Surface(GpuSurface::D3D12 {
                id: shared.id,
                texture: shared.handle,
                width: w,
                height: h,
                ready: interop.ready_handle(),
                ready_value: ready,
                release: interop.release_handle(),
                release_value: release,
            }));
        }

        #[cfg(not(windows))]
        let _ = share;
        // Reserva: copia para a CPU (espera a GPU terminar).
        // No OpenGL, o VAO e os parâmetros de leitura voltam ao padrão do Skia depois da cópia.
        #[cfg(all(feature = "gl", not(target_arch = "wasm32"), not(target_vendor = "apple")))]
        let hand_back = self.gl;
        let row = (w * 4).div_ceil(256) * 256;
        let size = (row * h) as u64;
        if view.readback.as_ref().is_none_or(|b| b.size() < size) {
            view.readback = Some(self.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("cópia"),
                size,
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
                mapped_at_creation: false,
            }));
        }
        let readback = view.readback.as_ref()?;
        encoder.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo { texture: &view.targets[target].texture, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
            wgpu::TexelCopyBufferInfo { buffer: readback, layout: wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(row), rows_per_image: Some(h) } },
            wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
        );
        self.queue.submit([encoder.finish()]);
        let slice = readback.slice(..size);
        slice.map_async(wgpu::MapMode::Read, |_| {});
        self.device.poll(wgpu::PollType::wait_indefinitely()).ok()?;
        let rgba_order = self.color_format() == wgpu::TextureFormat::Rgba8Unorm;
        let mut rgba = Vec::with_capacity((w * h * 4) as usize);
        {
            let data = slice.get_mapped_range().ok()?;
            for y in 0..h as usize {
                for px in data[y * row as usize..y * row as usize + w as usize * 4].chunks_exact(4) {
                    // BGRA (ou RGBA, no OpenGL) pré-multiplicado → RGBA comum.
                    let a = px[3];
                    let un = |c: u8| if a == 0 { 0 } else { ((c as u32 * 255 + a as u32 / 2) / a as u32).min(255) as u8 };
                    let (r, b) = if rgba_order { (px[0], px[2]) } else { (px[2], px[0]) };
                    rgba.extend_from_slice(&[un(r), un(px[1]), un(b), a]);
                }
            }
        }
        readback.unmap();
        #[cfg(all(feature = "gl", not(target_arch = "wasm32"), not(target_vendor = "apple")))]
        if hand_back {
            if let Some(st) = &self.gl_state {
                st.hand_to_skia();
            }
        }
        Some(Output::Pixels { width: w, height: h, rgba })
    }

    fn draw_buffer(&self, size: u64) -> (wgpu::Buffer, wgpu::BindGroup) {
        let buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("desenhos"),
            size,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &self.draw_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &buffer,
                    offset: 0,
                    size: std::num::NonZeroU64::new(std::mem::size_of::<DrawUniform>() as u64),
                }),
            }],
        });
        (buffer, group)
    }

    /// Uma vista nova (cada `Model3D` tem a sua).
    pub fn view(&self) -> View {
        let camera = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("câmera"),
            size: std::mem::size_of::<CameraUniform>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let camera_group = self.camera_group(&camera, &self.no_shadow);
        let shadow_camera = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("câmera da sombra"),
            size: std::mem::size_of::<CameraUniform>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let shadow_camera_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &self.shadow_camera_layout,
            entries: &[wgpu::BindGroupEntry { binding: 0, resource: shadow_camera.as_entire_binding() }],
        });
        let (draws, draw_group) = self.draw_buffer(DRAW_STRIDE * 16);
        let blank = |format, samples, usage| {
            self.device
                .create_texture(&wgpu::TextureDescriptor {
                    label: None,
                    size: wgpu::Extent3d { width: 1, height: 1, depth_or_array_layers: 1 },
                    mip_level_count: 1,
                    sample_count: samples,
                    dimension: wgpu::TextureDimension::D2,
                    format,
                    usage,
                    view_formats: &[],
                })
                .create_view(&Default::default())
        };
        View {
            size: (0, 0),
            shared: false,
            share_failed: false,
            msaa: blank(COLOR, SAMPLES, wgpu::TextureUsages::RENDER_ATTACHMENT),
            depth: blank(DEPTH, SAMPLES, wgpu::TextureUsages::RENDER_ATTACHMENT),
            targets: Vec::new(),
            next: 0,
            camera,
            camera_group,
            shadow_camera,
            shadow_camera_group,
            shadow_map: None,
            draws,
            draw_group,
            draw_capacity: DRAW_STRIDE * 16,
            joints: Vec::new(),
            readback: None,
            tlas: None,
            hit_colors: None,
            fresh: None,
            accum: None,
            glow_msaa: None,
            glow_levels: Vec::new(),
            glow_groups: Vec::new(),
            #[cfg(target_arch = "wasm32")]
            web: None,
        }
    }

    fn camera_group(&self, camera: &wgpu::Buffer, shadow_map: &wgpu::TextureView) -> wgpu::BindGroup {
        self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &self.camera_layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: camera.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::TextureView(shadow_map) },
                wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::Sampler(&self.shadow_sampler) },
            ],
        })
    }

    /// O mapa de sombra da vista (criado no primeiro quadro com sombra).
    fn add_shadow_map(&self, view: &mut View) {
        let map = self
            .device
            .create_texture(&wgpu::TextureDescriptor {
                label: Some("sombra"),
                size: wgpu::Extent3d { width: SHADOW_SIZE, height: SHADOW_SIZE, depth_or_array_layers: 1 },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: DEPTH,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            })
            .create_view(&Default::default());
        view.camera_group = self.camera_group(&view.camera, &map);
        view.shadow_map = Some(map);
    }

    fn resize_view(&mut self, view: &mut View, w: u32, h: u32, share: bool) -> Option<()> {
        let make = |format, samples, usage| {
            self.device
                .create_texture(&wgpu::TextureDescriptor {
                    label: Some("aylis-3d"),
                    size: wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
                    mip_level_count: 1,
                    sample_count: samples,
                    dimension: wgpu::TextureDimension::D2,
                    format,
                    usage,
                    view_formats: &[],
                })
                .create_view(&Default::default())
        };
        view.msaa = make(self.color_format(), SAMPLES, wgpu::TextureUsages::RENDER_ATTACHMENT);
        view.depth = make(DEPTH, SAMPLES, wgpu::TextureUsages::RENDER_ATTACHMENT);
        view.targets.clear();
        view.fresh = None;
        view.accum = None;
        view.glow_msaa = None;
        view.glow_levels.clear();
        // Duas texturas de saída: o 3D desenha numa enquanto o aylis compõe a outra.
        view.size = (w, h);
        view.shared = share;
        for _ in 0..2 {
            self.add_target(view, w, h)?;
        }
        Some(())
    }

    /// No navegador: desenha no `<canvas>` da vista (WebGPU ou WebGL 2), que o canvas 2D do
    /// aylis copia.
    #[cfg(target_arch = "wasm32")]
    fn render_web(&mut self, model: &Model, gpu: &mut GpuModel, view: &mut View, world: &[Mat4], weights: &[Vec<f32>], frame: &Frame) -> Option<Output> {
        let (w, h) = (frame.width.max(1), frame.height.max(1));
        if view.web.is_none() {
            let element = self.next_id as u32;
            self.next_id += 1;
            let (canvas, surface) = match self.web_gl.take() {
                // WebGL: o dispositivo nasceu do contexto do canvas dele, e só esse canvas serve.
                Some(gl) => gl,
                // WebGL com o canvas já de outra vista: ela fica sem desenho.
                None if self.web_is_gl => return None,
                None => {
                    let canvas = hidden_canvas()?;
                    let (instance, _) = self.web.as_ref()?;
                    let surface = instance.create_surface(wgpu::SurfaceTarget::Canvas(canvas.clone())).ok()?;
                    (canvas, surface)
                }
            };
            canvas.set_id(&format!("aylis-3d-{element}"));
            let (_, adapter) = self.web.as_ref()?;
            let caps = surface.get_capabilities(adapter);
            if self.web_format.is_none() {
                let pick = [wgpu::TextureFormat::Bgra8Unorm, wgpu::TextureFormat::Rgba8Unorm].into_iter().find(|f| caps.formats.contains(f));
                self.web_format = Some(pick.or(caps.formats.first().copied())?);
            }
            // Pré-multiplicado: o fundo transparente do 3D deixa ver o que o aylis desenhou
            // atrás. O WebGPU sempre aceita, mesmo quando o wgpu não lista. O WebGL só lista
            // `Opaque`, mas o contexto dele já nasce com alfa pré-multiplicado (o padrão do
            // navegador), e o fundo transparente continua transparente.
            let alpha = if self.web_is_gl { caps.alpha_modes.first().copied().unwrap_or(wgpu::CompositeAlphaMode::Opaque) } else { wgpu::CompositeAlphaMode::PreMultiplied };
            view.web = Some(WebTarget { canvas, surface, element, alpha, size: (0, 0) });
        }
        let format = self.color_format();
        if view.size != (w, h) {
            let make = |format, usage| {
                self.device
                    .create_texture(&wgpu::TextureDescriptor {
                        label: Some("aylis-3d"),
                        size: wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
                        mip_level_count: 1,
                        sample_count: SAMPLES,
                        dimension: wgpu::TextureDimension::D2,
                        format,
                        usage,
                        view_formats: &[],
                    })
                    .create_view(&Default::default())
            };
            view.msaa = make(format, wgpu::TextureUsages::RENDER_ATTACHMENT);
            view.depth = make(DEPTH, wgpu::TextureUsages::RENDER_ATTACHMENT);
            // As do brilho também (como no `resize_view`): ficavam do primeiro tamanho, o WebGPU
            // recusava todo quadro depois de mudar de tamanho e o 3D sumia (o lingote num
            // `<iframe>`, que nasce de um tamanho e é ajustado em seguida).
            view.glow_msaa = None;
            view.glow_levels.clear();
            view.size = (w, h);
        }
        let web = view.web.as_mut()?;
        if web.size != (w, h) {
            web.canvas.set_width(w);
            web.canvas.set_height(h);
            web.surface.configure(&self.device, &wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format,
                width: w,
                height: h,
                present_mode: wgpu::PresentMode::Fifo,
                desired_maximum_frame_latency: 2,
                alpha_mode: web.alpha,
                view_formats: vec![],
                color_space: Default::default(),
            });
            web.size = (w, h);
        }
        let texture = match web.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(t) | wgpu::CurrentSurfaceTexture::Suboptimal(t) => t,
            _ => return None,
        };
        let element = web.element;
        let target = texture.texture.create_view(&Default::default());
        let encoder = self.encode(model, gpu, view, world, weights, frame, &target);
        self.queue.submit([encoder.finish()]);
        self.queue.present(texture);
        Some(Output::Surface(GpuSurface::WebCanvas { element, width: w, height: h }))
    }

    /// Uma textura de saída a mais na vista.
    fn add_target(&mut self, view: &mut View, w: u32, h: u32) -> Option<()> {
        {
            #[cfg(windows)]
            if view.shared && !self.gl {
                let interop = self.interop.as_ref()?;
                let (texture, handle) = interop.shared_texture(&self.device, w, h)?;
                let id = self.next_id;
                self.next_id += 1;
                let view_ = texture.create_view(&Default::default());
                view.targets.push(Target { texture, view: view_, shared: Some(Shared { handle, id }), release_wait: 0, id });
                return Some(());
            }
            let texture = self.device.create_texture(&wgpu::TextureDescriptor {
                label: Some("aylis-3d saída"),
                size: wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                // No OpenGL o formato é RGBA: é o que o Skia aceita como textura.
                format: self.color_format(),
                // Amostrável: no OpenGL vira textura (e não renderbuffer), que o Skia lê.
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            });
            let view_ = texture.create_view(&Default::default());
            let id = self.next_id;
            self.next_id += 1;
            view.targets.push(Target { texture, view: view_, shared: None, release_wait: 0, id });
        }
        Some(())
    }

    /// A próxima textura de saída que o aylis já terminou de ler (a CPU só consulta a cerca,
    /// não espera). Se todas estão ocupadas, cria mais uma (até 4); só então a GPU espera
    /// pela mais antiga.
    fn free_target(&mut self, view: &mut View, w: u32, h: u32) -> usize {
        #[cfg(windows)]
        if view.shared && !self.gl {
            let done = self.interop.as_ref().map_or(u64::MAX, |i| i.released());
            let n = view.targets.len();
            for k in 0..n {
                let i = (view.next + k) % n;
                if view.targets[i].release_wait <= done {
                    view.targets[i].release_wait = 0;
                    view.next = i + 1;
                    return i;
                }
            }
            if n < 4 && self.add_target(view, w, h).is_some() {
                view.next = n + 1;
                return n;
            }
        }
        let _ = (w, h);
        let i = view.next % view.targets.len();
        view.next = i + 1;
        i
    }
}

/// O traçado de raios (ray query): o grupo 4 do shader (a cena e as cores) e o shader com
/// o bloco dos raios.
struct Rays {
    layout: wgpu::BindGroupLayout,
    pipeline_layout: wgpu::PipelineLayout,
    shader: wgpu::ShaderModule,
}

impl Rays {
    /// `None` se o shader com ray query não compilar (driver ou compilador sem suporte).
    fn new(device: &wgpu::Device, groups: [&wgpu::BindGroupLayout; 4]) -> Option<Rays> {
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("raios"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::AccelerationStructure { vertex_return: false },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Storage { read_only: true }, has_dynamic_offset: false, min_binding_size: None },
                    count: None,
                },
            ],
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("aylis-3d com raios"),
            bind_group_layouts: &[Some(groups[0]), Some(groups[1]), Some(groups[2]), Some(groups[3]), Some(&layout)],
            immediate_size: 0,
        });
        // O shader de sempre com o bloco dos raios trocado pelo de verdade.
        let base = include_str!("shader.wgsl");
        let end_marker = "// @rays-end";
        let (start, end) = (base.find("// @rays-begin")?, base.find(end_marker)? + end_marker.len());
        let source = format!("enable wgpu_ray_query;\n{}{}{}", &base[..start], include_str!("rays.wgsl"), &base[end..]);
        let scope = device.push_error_scope(wgpu::ErrorFilter::Validation);
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("aylis-3d com raios"), source: wgpu::ShaderSource::Wgsl(source.into()) });
        if let Some(e) = pollster::block_on(scope.pop()) {
            trace(&format!("sem traçado de raios: {e}"));
            return None;
        }
        Some(Rays { layout, pipeline_layout, shader })
    }
}

/// Um desenho na cena do traçado de raios.
struct SceneItem {
    mesh: usize,
    prim: usize,
    skin: Option<usize>,
    transform: Mat4,
    /// (cor, metal) e (emissão, aspereza): o que o reflexo encontra.
    colors: [[f32; 4]; 2],
    blend: bool,
}

/// Uma linha de [`Mat4`] por vez: a matriz 3×4 (linhas) das instâncias da cena.
fn rows_3x4(m: Mat4) -> [f32; 12] {
    let c = m.to_cols_array_2d();
    [c[0][0], c[1][0], c[2][0], c[3][0], c[0][1], c[1][1], c[2][1], c[3][1], c[0][2], c[1][2], c[2][2], c[3][2]]
}

/// O modelo na GPU.
pub struct GpuModel {
    meshes: Vec<Vec<GpuPrimitive>>,
    materials: Vec<wgpu::BindGroup>,
}

struct GpuPrimitive {
    vertex: wgpu::Buffer,
    index: wgpu::Buffer,
    count: u32,
    vertices: u32,
    material: usize,
    /// Pesos dos morph targets já aplicados (e o rascunho da mistura).
    weights: Vec<f32>,
    scratch: Vec<Vertex>,
    /// Traçado de raios: a forma na cena (BLAS), as posições já no mundo de quem tem
    /// esqueleto (o esqueleto anda na CPU para a cena) e se a forma mudou desde a montagem.
    blas: Option<wgpu::Blas>,
    positions: Option<wgpu::Buffer>,
    shape_changed: bool,
}

/// Textura compartilhada (Direct3D 12; nas outras plataformas ainda não há).
#[cfg_attr(not(windows), allow(dead_code))]
pub(crate) struct Shared {
    pub(crate) handle: isize,
    pub(crate) id: u64,
}

impl Drop for Shared {
    fn drop(&mut self) {
        #[cfg(windows)]
        crate::d3d12::close(self.handle);
    }
}

#[cfg_attr(not(windows), allow(dead_code))]
struct Target {
    texture: wgpu::Texture,
    view: wgpu::TextureView,
    shared: Option<Shared>,
    /// Valor da cerca de liberação que o aylis marca quando terminar de ler esta textura.
    release_wait: u64,
    #[cfg_attr(not(feature = "gl"), allow(dead_code))]
    id: u64,
}

/// As texturas e buffers de um `Model3D`.
pub struct View {
    size: (u32, u32),
    shared: bool,
    /// O canvas recusou a textura: daqui em diante, cópia pela CPU.
    share_failed: bool,
    msaa: wgpu::TextureView,
    depth: wgpu::TextureView,
    targets: Vec<Target>,
    next: usize,
    camera: wgpu::Buffer,
    camera_group: wgpu::BindGroup,
    shadow_camera: wgpu::Buffer,
    shadow_camera_group: wgpu::BindGroup,
    shadow_map: Option<wgpu::TextureView>,
    draws: wgpu::Buffer,
    draw_group: wgpu::BindGroup,
    draw_capacity: u64,
    joints: Vec<(wgpu::Buffer, wgpu::BindGroup)>,
    readback: Option<wgpu::Buffer>,
    /// Traçado de raios: a cena (com a capacidade) e as cores das instâncias.
    tlas: Option<(wgpu::Tlas, u32)>,
    hit_colors: Option<wgpu::Buffer>,
    /// Acumulação: o quadro novo e a média das amostras.
    fresh: Option<wgpu::TextureView>,
    accum: Option<wgpu::TextureView>,
    /// Brilho: a emissão desenhada (MSAA) e os níveis dela, do tamanho do quadro até 1/32.
    glow_msaa: Option<wgpu::TextureView>,
    glow_levels: Vec<wgpu::TextureView>,
    /// Um grupo por nível do brilho, com ele como fonte (criados junto com os níveis: as
    /// passadas do brilho não criam nada a cada quadro).
    glow_groups: Vec<wgpu::BindGroup>,
    /// No navegador: o `<canvas>` (WebGPU) da vista.
    #[cfg(target_arch = "wasm32")]
    web: Option<WebTarget>,
}

/// Um `<canvas>` escondido no corpo da página, onde o 3D desenha (o canvas 2D do aylis copia).
#[cfg(target_arch = "wasm32")]
fn hidden_canvas() -> Option<web_sys::HtmlCanvasElement> {
    use wasm_bindgen::JsCast;
    let document = web_sys::window()?.document()?;
    let canvas: web_sys::HtmlCanvasElement = document.create_element("canvas").ok()?.dyn_into().ok()?;
    let _ = canvas.style().set_property("display", "none");
    document.body()?.append_child(&canvas).ok()?;
    Some(canvas)
}

#[cfg(target_arch = "wasm32")]
struct WebTarget {
    canvas: web_sys::HtmlCanvasElement,
    surface: wgpu::Surface<'static>,
    element: u32,
    alpha: wgpu::CompositeAlphaMode,
    size: (u32, u32),
}

#[cfg(target_arch = "wasm32")]
impl Drop for WebTarget {
    fn drop(&mut self) {
        self.canvas.remove();
    }
}

impl View {
    /// O canvas recusou a última textura entregue: ela não será liberada, então não se
    /// espera por ela, e a vista passa a entregar pixels.
    pub fn rejected(&mut self) {
        self.share_failed = true;
        for t in &mut self.targets {
            t.release_wait = 0;
        }
    }
}

/// Textura RGBA sRGB com mipmaps (feitos na CPU, por média de 2×2).
/// As luzes do quadro para o shader e quantas são.
fn scene_lights(model: &Model, world: &[Mat4], frame: &Frame) -> ([LightUniform; MAX_LIGHTS], usize) {
    let mut out = [LightUniform::default(); MAX_LIGHTS];
    if !frame.file_lights || model.lights.is_empty() {
        let dir = frame.light_dir.normalize_or(Vec3::NEG_Y);
        out[0] = LightUniform { position: [0.0; 4], direction: dir.extend(0.0).into(), color: [2.6, 2.5, 2.4, 0.0], cone: [0.0; 4] };
        return (out, 1);
    }
    // A primeira direcional vai na frente: é a da sombra.
    let mut order: Vec<&crate::model::Light> = model.lights.iter().collect();
    if let Some(i) = order.iter().position(|l| l.kind == LightKind::Directional) {
        let first = order.remove(i);
        order.insert(0, first);
    }
    let mut n = 0;
    for light in order.into_iter().take(MAX_LIGHTS) {
        let m = world.get(light.node).copied().unwrap_or(Mat4::IDENTITY);
        let position = m.transform_point3(Vec3::ZERO);
        let direction = m.transform_vector3(Vec3::NEG_Z).normalize_or(Vec3::NEG_Z);
        let (kind, cone) = match light.kind {
            LightKind::Directional => (0.0, [0.0; 4]),
            LightKind::Point => (1.0, [0.0; 4]),
            LightKind::Spot { inner, outer } => {
                // Como no glTF: escala e deslocamento sobre o cosseno do ângulo.
                let scale = 1.0 / (inner.cos() - outer.cos()).max(0.001);
                (2.0, [scale, -outer.cos() * scale, 0.0, 0.0])
            }
        };
        let c = light.color.map(|c| c * light.intensity);
        out[n] = LightUniform {
            position: position.extend(kind).into(),
            direction: direction.extend(light.range.unwrap_or(0.0)).into(),
            color: [c[0], c[1], c[2], 0.0],
            cone,
        };
        n += 1;
    }
    (out, n)
}

/// A projeção da sombra (luz direcional `dir`) cobrindo o modelo, e o tamanho de um texel
/// no mundo. A esfera em volta da caixa do modelo ganha folga para a animação.
fn shadow_matrix(model: &Model, dir: Vec3) -> (Mat4, f32) {
    let (lo, hi) = model.bounds;
    let center = (lo + hi) / 2.0;
    let r = ((hi - lo).length() / 2.0).max(1e-3) * 1.3;
    let dir = dir.normalize_or(Vec3::NEG_Y);
    let up = if dir.y.abs() > 0.99 { Vec3::Z } else { Vec3::Y };
    let view = glam::camera::rh::view::look_at_mat4(center - dir * r * 2.0, center, up);
    let proj = glam::camera::rh::proj::directx::orthographic(-r, r, -r, r, 0.0, r * 4.0);
    (proj * view, 2.0 * r / SHADOW_SIZE as f32)
}

fn upload_texture(device: &wgpu::Device, queue: &wgpu::Queue, w: u32, h: u32, rgba: &[u8], srgb: bool) -> wgpu::TextureView {
    let levels = 32 - w.max(h).max(1).leading_zeros();
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("textura"),
        size: wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
        mip_level_count: levels,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: if srgb { wgpu::TextureFormat::Rgba8UnormSrgb } else { wgpu::TextureFormat::Rgba8Unorm },
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });
    let (mut cw, mut ch, mut level) = (w, h, rgba.to_vec());
    for mip in 0..levels {
        queue.write_texture(
            wgpu::TexelCopyTextureInfo { texture: &texture, mip_level: mip, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
            &level,
            wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(cw * 4), rows_per_image: Some(ch) },
            wgpu::Extent3d { width: cw, height: ch, depth_or_array_layers: 1 },
        );
        let (nw, nh) = ((cw / 2).max(1), (ch / 2).max(1));
        let mut next = vec![0u8; (nw * nh * 4) as usize];
        for y in 0..nh {
            for x in 0..nw {
                for c in 0..4 {
                    let at = |xx: u32, yy: u32| level[((yy.min(ch - 1) * cw + xx.min(cw - 1)) * 4 + c) as usize] as u32;
                    next[((y * nw + x) * 4 + c) as usize] = ((at(x * 2, y * 2) + at(x * 2 + 1, y * 2) + at(x * 2, y * 2 + 1) + at(x * 2 + 1, y * 2 + 1)) / 4) as u8;
                }
            }
        }
        (cw, ch, level) = (nw, nh, next);
    }
    texture.create_view(&Default::default())
}

/// `resolve` é uma das texturas de saída da vista (e não a de uma janela, em `render_to`).
fn resolve_is_view_target(view: &View, resolve: &wgpu::TextureView) -> bool {
    view.targets.iter().any(|t| &t.view == resolve)
}

pub(crate) fn trace(what: &str) {
    if crate::diag::diagnostic_setting("AYLIS_3D_TRACE").is_some() {
        crate::diag::diagnostic(&format!("aylis-3d: {what}"));
    }
}

#[cfg(all(feature = "gl", not(target_arch = "wasm32"), not(target_vendor = "apple")))]
type GlLoader = unsafe extern "C" fn(*const std::ffi::c_char) -> *const std::ffi::c_void;

/// Esvazia a fila de erros do OpenGL.
#[cfg(all(feature = "gl", not(target_arch = "wasm32"), not(target_vendor = "apple")))]
fn clear_gl_errors(load: GlLoader) {
    let f = unsafe { load(c"glGetError".as_ptr()) };
    if f.is_null() {
        return;
    }
    // SAFETY: glGetError é `GLenum glGetError(void)`.
    let get_error: extern "system" fn() -> u32 = unsafe { std::mem::transmute(f) };
    for _ in 0..32 {
        if get_error() == 0 {
            break;
        }
    }
}

/// O estado do OpenGL que o wgpu e o Skia dividem no mesmo contexto. O wgpu espera achar o
/// VAO dele ligado; o Skia (no OpenGL ES) desenha no VAO 0 e espera os parâmetros de upload
/// padrão. Sem isso, um escreve formatos de vértice no VAO do outro (o modelo sai deformado)
/// e os textos do Skia sobem com o alinhamento do wgpu (letras quebradas).
#[cfg(all(feature = "gl", not(target_arch = "wasm32"), not(target_vendor = "apple")))]
struct GlState {
    vao: u32,
    bind_vao: extern "system" fn(u32),
    bind_buffer: extern "system" fn(u32, u32),
    pixel_store: extern "system" fn(u32, i32),
    get_integer: extern "system" fn(u32, *mut i32),
}

#[cfg(all(feature = "gl", not(target_arch = "wasm32"), not(target_vendor = "apple")))]
impl GlState {
    fn load(load: GlLoader) -> Option<GlState> {
        let f = |name: &std::ffi::CStr| {
            let p = unsafe { load(name.as_ptr()) };
            (!p.is_null()).then_some(p)
        };
        // SAFETY: as assinaturas são as do OpenGL para cada nome.
        unsafe {
            Some(GlState {
                vao: 0,
                bind_vao: std::mem::transmute::<*const std::ffi::c_void, extern "system" fn(u32)>(f(c"glBindVertexArray")?),
                bind_buffer: std::mem::transmute::<*const std::ffi::c_void, extern "system" fn(u32, u32)>(f(c"glBindBuffer")?),
                pixel_store: std::mem::transmute::<*const std::ffi::c_void, extern "system" fn(u32, i32)>(f(c"glPixelStorei")?),
                get_integer: std::mem::transmute::<*const std::ffi::c_void, extern "system" fn(u32, *mut i32)>(f(c"glGetIntegerv")?),
            })
        }
    }

    fn current_vao(&self) -> u32 {
        let mut v = 0i32;
        (self.get_integer)(0x85B5, &mut v); // GL_VERTEX_ARRAY_BINDING
        v as u32
    }

    /// Antes do wgpu desenhar: o VAO dele.
    fn take_from_skia(&self) {
        (self.bind_vao)(self.vao);
    }

    /// Depois do wgpu: o VAO 0 e os parâmetros de upload e leitura padrão, para o Skia.
    fn hand_to_skia(&self) {
        (self.bind_vao)(0);
        (self.bind_buffer)(0x88EC, 0); // GL_PIXEL_UNPACK_BUFFER
        (self.bind_buffer)(0x88EB, 0); // GL_PIXEL_PACK_BUFFER
        for (name, value) in [
            (0x0CF5, 4), // GL_UNPACK_ALIGNMENT
            (0x0D05, 4), // GL_PACK_ALIGNMENT
            (0x0CF2, 0), // GL_UNPACK_ROW_LENGTH
            (0x0D02, 0), // GL_PACK_ROW_LENGTH
            (0x0CF3, 0), // GL_UNPACK_SKIP_ROWS
            (0x0CF4, 0), // GL_UNPACK_SKIP_PIXELS
            (0x806E, 0), // GL_UNPACK_IMAGE_HEIGHT
            (0x806D, 0), // GL_UNPACK_SKIP_IMAGES
        ] {
            (self.pixel_store)(name, value);
        }
    }
}
