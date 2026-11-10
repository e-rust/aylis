//! # aylis-3d
//!
//! Modelos 3D no aylis: glTF 2.0 / GLB (malhas, materiais PBR, esqueleto, morph targets e
//! clipes de animação), desenhados com `wgpu` e compostos com os controles na própria GPU.

#[cfg(windows)]
mod d3d12;
pub(crate) mod diag;
#[cfg(feature = "fbx")]
pub mod fbx;
pub mod glb;
pub mod ingot;
pub mod model;
mod model3d;
pub mod render;
pub mod sample;

pub use model::{Camera, Clip, Light, LightKind, Model, Pose, Projection};

/// O que uma placa oferece de traçado de raios (para decidir o caminho; ver `docs/3d.md`).
#[derive(Clone, Debug)]
pub struct RayTracingSupport {
    pub adapter: String,
    pub backend: String,
    /// Consulta de raio dentro de qualquer shader (sombras e reflexos traçados).
    pub ray_query: bool,
    /// Pipeline de traçado completo (raygen/hit/miss), o do path tracing em tempo real.
    pub pipelines: bool,
}

/// As placas de vídeo do sistema pelo nome (sem as que desenham na CPU), para escolher uma
/// (`aylis_core::gpu`).
#[cfg(not(target_arch = "wasm32"))]
pub fn adapters() -> Vec<String> {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor { backends: wgpu::Backends::PRIMARY, ..wgpu::InstanceDescriptor::new_without_display_handle() });
    let mut names: Vec<String> = Vec::new();
    for a in pollster::block_on(instance.enumerate_adapters(wgpu::Backends::PRIMARY)) {
        let info = a.get_info();
        if info.device_type != wgpu::DeviceType::Cpu && !names.contains(&info.name) {
            names.push(info.name);
        }
    }
    names
}

/// As placas do sistema e o suporte de cada uma a traçado de raios pelo `wgpu`.
#[cfg(not(target_arch = "wasm32"))]
pub fn ray_tracing_support() -> Vec<RayTracingSupport> {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor { backends: wgpu::Backends::PRIMARY, ..wgpu::InstanceDescriptor::new_without_display_handle() });
    pollster::block_on(instance.enumerate_adapters(wgpu::Backends::PRIMARY))
        .into_iter()
        .map(|a| {
            let info = a.get_info();
            let features = a.features();
            RayTracingSupport {
                adapter: info.name,
                backend: format!("{:?}", info.backend),
                ray_query: features.contains(wgpu::Features::EXPERIMENTAL_RAY_QUERY),
                pipelines: features.contains(wgpu::Features::EXPERIMENTAL_RAY_TRACING_PIPELINES),
            }
        })
        .collect()
}
pub use model3d::{AUTO_RESOLUTION, Model3D};
pub use render::{Engine, Frame, GpuModel, Output, View};

/// Registra o `Model3D` na paleta (categoria "3D").
pub fn register(ui: &mut aylis_core::Ui) {
    ui.register_class::<Model3D>("3D");
}
