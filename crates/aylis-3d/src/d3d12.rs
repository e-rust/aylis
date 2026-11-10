//! Windows: textura e cercas compartilhadas do Direct3D 12, abertas pelo Direct2D (no
//! Direct3D 11) para compor o 3D sem passar pela CPU.
//!
//! A cada quadro, a fila do 3D espera a cerca de liberação (o aylis terminou de ler a
//! textura daquele buffer), desenha e marca a cerca de pronto. O aylis, na GPU, espera a de
//! pronto, compõe e marca a de liberação. A CPU não espera ninguém.

use windows::Win32::Foundation::{CloseHandle, GENERIC_ALL, HANDLE};
use windows::Win32::Graphics::Direct3D12::*;
use windows::Win32::Graphics::Dxgi::Common::*;
use windows::core::PCWSTR;

type Dx12 = wgpu_hal::api::Dx12;

/// O LUID do adaptador (para achar o mesmo do Direct2D).
pub fn adapter_luid(adapter: &wgpu::Adapter) -> Option<i64> {
    let hal = unsafe { adapter.as_hal::<Dx12>()? };
    let desc = unsafe { hal.raw_adapter().GetDesc1().ok()? };
    Some(((desc.AdapterLuid.HighPart as i64) << 32) | desc.AdapterLuid.LowPart as i64)
}

pub fn close(handle: isize) {
    if handle != 0 {
        let _ = unsafe { CloseHandle(HANDLE(handle as *mut _)) };
    }
}

pub struct Interop {
    ready: ID3D12Fence,
    ready_handle: isize,
    release: ID3D12Fence,
    release_handle: isize,
    value: u64,
}

impl Interop {
    pub fn new(device: &wgpu::Device) -> Option<Interop> {
        let hal = unsafe { device.as_hal::<Dx12>()? };
        let raw = hal.raw_device();
        let fence = || -> Option<(ID3D12Fence, isize)> {
            let f: ID3D12Fence = unsafe { raw.CreateFence(0, D3D12_FENCE_FLAG_SHARED).ok()? };
            let h = unsafe { raw.CreateSharedHandle(&f, None, GENERIC_ALL.0, PCWSTR::null()).ok()? };
            Some((f, h.0 as isize))
        };
        let (ready, ready_handle) = fence()?;
        let (release, release_handle) = fence()?;
        Some(Interop { ready, ready_handle, release, release_handle, value: 0 })
    }

    pub fn ready_handle(&self) -> isize {
        self.ready_handle
    }

    pub fn release_handle(&self) -> isize {
        self.release_handle
    }

    /// Textura BGRA compartilhável, já embrulhada como textura do `wgpu`.
    pub fn shared_texture(&self, device: &wgpu::Device, w: u32, h: u32) -> Option<(wgpu::Texture, isize)> {
        let (resource, handle) = {
            let hal = unsafe { device.as_hal::<Dx12>()? };
            let raw = hal.raw_device();
            let heap = D3D12_HEAP_PROPERTIES { Type: D3D12_HEAP_TYPE_DEFAULT, ..Default::default() };
            let desc = D3D12_RESOURCE_DESC {
                Dimension: D3D12_RESOURCE_DIMENSION_TEXTURE2D,
                Alignment: 0,
                Width: w as u64,
                Height: h,
                DepthOrArraySize: 1,
                MipLevels: 1,
                Format: DXGI_FORMAT_B8G8R8A8_UNORM,
                SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
                Layout: D3D12_TEXTURE_LAYOUT_UNKNOWN,
                // Acesso simultâneo: a textura volta sozinha ao estado comum entre os
                // dois dispositivos, sem barreiras.
                Flags: D3D12_RESOURCE_FLAG_ALLOW_RENDER_TARGET | D3D12_RESOURCE_FLAG_ALLOW_SIMULTANEOUS_ACCESS,
            };
            let mut resource: Option<ID3D12Resource> = None;
            unsafe { raw.CreateCommittedResource(&heap, D3D12_HEAP_FLAG_SHARED, &desc, D3D12_RESOURCE_STATE_COMMON, None, &mut resource).ok()? };
            let resource = resource?;
            let handle = unsafe { raw.CreateSharedHandle(&resource, None, GENERIC_ALL.0, PCWSTR::null()).ok()? };
            (resource, handle.0 as isize)
        };
        let size = wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 };
        let hal_texture = unsafe { wgpu_hal::dx12::Device::texture_from_raw(resource, wgpu::TextureFormat::Bgra8Unorm, wgpu::TextureDimension::D2, size, 1, 1) };
        let texture = unsafe {
            device.create_texture_from_hal::<Dx12>(
                hal_texture,
                &wgpu::TextureDescriptor {
                    label: Some("aylis-3d compartilhada"),
                    size,
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    format: wgpu::TextureFormat::Bgra8Unorm,
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
                    view_formats: &[],
                },
                wgpu::TextureUses::UNINITIALIZED,
            )
        };
        Some((texture, handle))
    }

    /// Antes do próximo `submit`: esperar o aylis liberar a textura (`wait`, 0 = nada a
    /// esperar) e marcar o quadro pronto. Devolve (valor de pronto, valor de liberação).
    pub fn submit(&mut self, queue: &wgpu::Queue, wait: u64) -> (u64, u64) {
        self.value += 1;
        // Já liberada: nada a esperar. Entregue há muitos quadros e ainda presa (o aylis não
        // vai mais marcar): também não espera, para a GPU não travar por causa disso.
        let done = unsafe { self.release.GetCompletedValue() };
        let wait = if wait > done && self.value - wait < 8 { wait } else { 0 };
        if let Some(q) = unsafe { queue.as_hal::<Dx12>() } {
            if wait > 0 {
                q.add_wait_fence(self.release.clone(), wait);
            }
            q.add_signal_fence(self.ready.clone(), self.value);
        }
        (self.value, self.value)
    }
}

impl Interop {
    /// Até que valor o aylis já liberou (consulta, sem esperar).
    pub fn released(&self) -> u64 {
        unsafe { self.release.GetCompletedValue() }
    }

    /// Marca a liberação pela CPU (testes, sem um renderizador do aylis do outro lado).
    pub fn release_on_cpu(&self, value: u64) {
        let _ = unsafe { self.release.Signal(value) };
    }
}

impl Drop for Interop {
    fn drop(&mut self) {
        close(self.ready_handle);
        close(self.release_handle);
    }
}

/// O compilador DXC (`dxcompiler.dll`), que o traçado de raios precisa no Direct3D 12 (o FXC
/// não compila ray query). Procura em `AYLIS_DXC`, ao lado do executável, nas ferramentas do
/// aylis, no Vulkan SDK e no Windows SDK (o mais novo). Sem ele, o 3D desenha sem raios.
pub(crate) fn dxc_path() -> Option<std::path::PathBuf> {
    use std::path::PathBuf;
    const FILE: &str = "dxcompiler.dll";
    let env = crate::diag::diagnostic_setting("AYLIS_DXC").map(PathBuf::from);
    let beside = std::env::current_exe().ok().and_then(|e| e.parent().map(|d| d.join(FILE)));
    let tools = std::env::var_os("LOCALAPPDATA").map(|d| PathBuf::from(d).join("aylis/tools/dxc/bin/x64").join(FILE));
    let vulkan = std::env::var_os("VULKAN_SDK").map(|d| PathBuf::from(d).join("Bin").join(FILE));
    let kits = std::env::var_os("ProgramFiles(x86)").and_then(|p| {
        let bin = PathBuf::from(p).join("Windows Kits/10/bin");
        let mut versions: Vec<PathBuf> = std::fs::read_dir(bin).ok()?.flatten().map(|e| e.path().join("x64").join(FILE)).filter(|p| p.is_file()).collect();
        versions.sort();
        versions.pop()
    });
    [env, beside, tools, vulkan, kits].into_iter().flatten().find(|p| p.is_file())
}
