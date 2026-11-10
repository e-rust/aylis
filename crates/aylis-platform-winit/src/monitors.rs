//! Os monitores pelo winit e, no Windows, o nome do aparelho ("XB273K V5", pela configuração
//! de vídeo do sistema) e a área útil (sem a barra de tarefas). Nas outras plataformas o nome
//! é o do sistema e a área útil é o monitor inteiro.

use aylis_core::monitors::{Monitor, VideoMode};
use winit::event_loop::ActiveEventLoop;
use winit::monitor::MonitorHandle;

/// O nome do sistema de um monitor (o que se guarda para reabrir nele); vazio se o monitor
/// sumiu.
pub(crate) fn device(handle: &MonitorHandle) -> String {
    if !valid(handle) {
        return String::new();
    }
    handle.name().unwrap_or_default()
}

/// O monitor ainda existe? No Windows, o identificador de um monitor desligado ou trocado deixa
/// de valer, e o `name`/`size` do winit 0.30 dão `unwrap` no erro: a IDE caiu assim em 08/11
/// ("Identificador de monitor inválido"), com o refresh dos monitores a cada 2 s. Pergunte
/// antes de chamar os dois.
pub(crate) fn valid(handle: &MonitorHandle) -> bool {
    #[cfg(windows)]
    return windows::work_area(handle).is_some();
    #[cfg(not(windows))]
    {
        let _ = handle;
        true
    }
}

/// O tamanho do monitor (zero se ele sumiu).
pub(crate) fn size(handle: &MonitorHandle) -> winit::dpi::PhysicalSize<u32> {
    if valid(handle) { handle.size() } else { winit::dpi::PhysicalSize::new(0, 0) }
}

/// Os monitores agora. Com `modes`, também os modos de vídeo (centenas de perguntas ao
/// sistema: só quando a lista muda).
pub(crate) fn list(event_loop: &ActiveEventLoop, modes: bool) -> Vec<Monitor> {
    let primary = event_loop.primary_monitor();
    #[cfg(windows)]
    let names = windows::friendly_names();
    event_loop
        .available_monitors()
        .filter(valid)
        .map(|m| {
            let (p, s) = (m.position(), m.size());
            let device = device(&m);
            #[cfg(windows)]
            let name = names.get(&device).cloned().filter(|n| !n.is_empty()).unwrap_or_else(|| device.clone());
            #[cfg(not(windows))]
            let name = device.clone();
            #[cfg(windows)]
            let work = windows::work_area(&m);
            #[cfg(not(windows))]
            let work = None;
            let (work_x, work_y, work_width, work_height) = work.unwrap_or((p.x, p.y, s.width, s.height));
            let mut video: Vec<VideoMode> = if modes {
                m.video_modes()
                    .map(|v| VideoMode { width: v.size().width, height: v.size().height, bit_depth: v.bit_depth(), refresh_hz: v.refresh_rate_millihertz() as f64 / 1000.0 })
                    .collect()
            } else {
                Vec::new()
            };
            video.dedup();
            Monitor {
                name,
                primary: primary.as_ref() == Some(&m),
                device,
                x: p.x,
                y: p.y,
                width: s.width,
                height: s.height,
                work_x,
                work_y,
                work_width,
                work_height,
                scale: m.scale_factor(),
                refresh_hz: m.refresh_rate_millihertz().map_or(0.0, |mhz| mhz as f64 / 1000.0),
                modes: video,
            }
        })
        .collect()
}

/// O índice do monitor na lista (pelo nome do sistema).
pub(crate) fn index_of(monitors: &[Monitor], handle: &MonitorHandle) -> Option<usize> {
    let device = device(handle);
    monitors.iter().position(|m| m.device == device)
}

#[cfg(windows)]
mod windows {
    use std::collections::HashMap;

    use ::windows::Win32::Devices::Display::{
        DISPLAYCONFIG_DEVICE_INFO_GET_SOURCE_NAME, DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_NAME, DISPLAYCONFIG_DEVICE_INFO_HEADER, DISPLAYCONFIG_MODE_INFO,
        DISPLAYCONFIG_PATH_INFO, DISPLAYCONFIG_SOURCE_DEVICE_NAME, DISPLAYCONFIG_TARGET_DEVICE_NAME, DisplayConfigGetDeviceInfo, GetDisplayConfigBufferSizes,
        QDC_ONLY_ACTIVE_PATHS, QueryDisplayConfig,
    };
    use ::windows::Win32::Foundation::ERROR_SUCCESS;
    use ::windows::Win32::Graphics::Gdi::{GetMonitorInfoW, HMONITOR, MONITORINFO};
    use winit::monitor::MonitorHandle;
    use winit::platform::windows::MonitorHandleExtWindows;

    fn text(chars: &[u16]) -> String {
        let end = chars.iter().position(|&c| c == 0).unwrap_or(chars.len());
        String::from_utf16_lossy(&chars[..end])
    }

    /// O nome de cada monitor ligado (`\\.\DISPLAY1` → "XB273K V5").
    pub(super) fn friendly_names() -> HashMap<String, String> {
        let mut names = HashMap::new();
        // SAFETY: os buffers têm o tamanho que o próprio sistema informou; os pacotes de
        // pergunta levam o tamanho e o tipo certos no cabeçalho.
        unsafe {
            let (mut paths, mut modes) = (0u32, 0u32);
            if GetDisplayConfigBufferSizes(QDC_ONLY_ACTIVE_PATHS, &mut paths, &mut modes) != ERROR_SUCCESS {
                return names;
            }
            let mut path_info = vec![DISPLAYCONFIG_PATH_INFO::default(); paths as usize];
            let mut mode_info = vec![DISPLAYCONFIG_MODE_INFO::default(); modes as usize];
            if QueryDisplayConfig(QDC_ONLY_ACTIVE_PATHS, &mut paths, path_info.as_mut_ptr(), &mut modes, mode_info.as_mut_ptr(), None) != ERROR_SUCCESS {
                return names;
            }
            for path in &path_info[..paths as usize] {
                let mut source = DISPLAYCONFIG_SOURCE_DEVICE_NAME::default();
                source.header = DISPLAYCONFIG_DEVICE_INFO_HEADER {
                    r#type: DISPLAYCONFIG_DEVICE_INFO_GET_SOURCE_NAME,
                    size: std::mem::size_of::<DISPLAYCONFIG_SOURCE_DEVICE_NAME>() as u32,
                    adapterId: path.sourceInfo.adapterId,
                    id: path.sourceInfo.id,
                };
                let mut target = DISPLAYCONFIG_TARGET_DEVICE_NAME::default();
                target.header = DISPLAYCONFIG_DEVICE_INFO_HEADER {
                    r#type: DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_NAME,
                    size: std::mem::size_of::<DISPLAYCONFIG_TARGET_DEVICE_NAME>() as u32,
                    adapterId: path.targetInfo.adapterId,
                    id: path.targetInfo.id,
                };
                if DisplayConfigGetDeviceInfo(&mut source.header) == 0 && DisplayConfigGetDeviceInfo(&mut target.header) == 0 {
                    names.insert(text(&source.viewGdiDeviceName), text(&target.monitorFriendlyDeviceName));
                }
            }
        }
        names
    }

    /// A área útil do monitor (sem a barra de tarefas), em pixels físicos.
    pub(super) fn work_area(handle: &MonitorHandle) -> Option<(i32, i32, u32, u32)> {
        let mut info = MONITORINFO { cbSize: std::mem::size_of::<MONITORINFO>() as u32, ..Default::default() };
        // SAFETY: o HMONITOR vem do próprio winit e o `cbSize` diz o tamanho da estrutura.
        let ok = unsafe { GetMonitorInfoW(HMONITOR(handle.hmonitor() as *mut _), &mut info) }.as_bool();
        let r = info.rcWork;
        ok.then(|| (r.left, r.top, (r.right - r.left).max(0) as u32, (r.bottom - r.top).max(0) as u32))
    }
}
