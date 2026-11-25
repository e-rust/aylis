//! Os serviços da plataforma que o `Ui` usa (o texto, o desenho fora da tela, os diálogos de
//! arquivo; no navegador, os dele), postos antes da janela ou pelo `run`.

use super::*;

/// Entrega ao `Ui` os serviços desta plataforma que ele ainda não tiver. A medição de
/// texto é a do renderizador em uso (as larguras batem com o desenho).
/// Põe já o serviço de texto do renderizador `renderer` (o que o `run` usaria), para que o
/// `measure_text` valha na montagem dos forms, antes da janela: sem ele, a largura do texto
/// era estimada e um rótulo com `auto_size` nascia cortado (o relatório do Tabernáculo, T01).
/// O `run` mantém o que já estiver posto.
#[cfg(not(web))]
pub fn install_text_service(ui: &mut Ui, renderer: Renderer) {
    if ui.services().text.is_some() {
        return;
    }
    let chosen = service_renderer(renderer);
    let Ok(painters) = Painters::new() else { return };
    let services = ui.services_mut();
    let _ = (&painters, chosen);
    #[cfg(native_d2d)]
    if chosen == Renderer::D2d {
        services.text = Some(Box::new(painters.d2d.text_service()));
    }
    #[cfg(native_skia)]
    if services.text.is_none() {
        services.text = Some(Box::new(painters.skia.text_service()));
    }
}

/// No navegador o serviço de texto nasce com o `run` (precisa do documento).
#[cfg(web)]
pub fn install_text_service(_ui: &mut Ui, _renderer: Renderer) {}

/// O renderizador dos serviços postos antes da janela (o texto e o desenho fora da tela): o
/// pedido, ou, no automático, o do diagnóstico `AYLIS_RENDERER`.
#[cfg(not(web))]
pub(crate) fn service_renderer(renderer: Renderer) -> Renderer {
    Painters::resolve(match setting("AYLIS_RENDERER").as_deref() {
        Some("cpu") if renderer == Renderer::Auto => Renderer::Cpu,
        Some("gpu") if renderer == Renderer::Auto => Renderer::Gpu,
        Some("d2d") if renderer == Renderer::Auto => Renderer::D2d,
        _ => renderer,
    })
}

/// Põe já o desenho fora da tela do renderizador `renderer` (o mesmo que o
/// [`install_text_service`] escolhe: Direct2D no Windows com a feature `d2d`, Skia nas
/// outras), para o `Ui::render_png` e o `Ui::snapshot` valerem sem janela: o modo sem
/// plataforma dos testes e das fotos (`aylis::headless`; o relatório do Tabernáculo, B04).
/// O `run` mantém o que já estiver posto.
#[cfg(not(web))]
pub fn install_offscreen_service(ui: &mut Ui, renderer: Renderer) {
    let services = ui.services_mut();
    if services.offscreen.is_some() {
        return;
    }
    let chosen = service_renderer(renderer);
    let _ = chosen;
    #[cfg(native_d2d)]
    if chosen == Renderer::D2d {
        services.offscreen = Some(Box::new(D2dOffscreen));
    }
    #[cfg(native_skia)]
    if services.offscreen.is_none() {
        services.offscreen = Some(Box::new(SkiaOffscreen));
    }
}

/// No navegador o desenho fora da tela nasce com o `run` (`aylis_web::WebOffscreen`, precisa do
/// documento).
#[cfg(web)]
pub fn install_offscreen_service(_ui: &mut Ui, _renderer: Renderer) {}

#[cfg(not(web))]
pub(crate) fn install_services(ui: &mut Ui, renderer: &Painters, chosen: Renderer) {
    // O efeito de fim da rolagem do sistema (o `overscroll: auto`): o Android até o 11 com o
    // brilho, do 12 em diante esticando; o iOS com a mola; o desktop e o navegador sem nada.
    #[cfg(target_os = "android")]
    ui.set_overscroll_style(if android_device::sdk() >= 31 { aylis_core::Overscroll::Stretch } else { aylis_core::Overscroll::Glow });
    #[cfg(target_os = "ios")]
    ui.set_overscroll_style(aylis_core::Overscroll::Bounce);
    // As barras do sistema (o `SystemBars`): o Android, o iOS (com a faixa da cor pintada pelo
    // núcleo) e a `theme-color` do navegador.
    #[cfg(any(target_os = "android", target_os = "ios", web))]
    aylis_core::system_bars::set_available(true);
    #[cfg(target_os = "ios")]
    ui.set_paints_status_strip(true);
    // A captura das teclas do sistema (o `KeyboardCapture`): o gancho do Windows, o
    // `XGrabKeyboard` do X11 e o inibidor de atalhos do Wayland.
    #[cfg(any(windows, all(target_os = "linux", not(target_os = "android"))))]
    aylis_core::system_keys::set_capture_available(true);
    // A orientação para testar no desenvolvimento (`AYLIS_ORIENTATION`, ou `adb shell setprop
    // debug.aylis.orientation landscape`), por cima do projeto e dos forms; só em debug.
    #[cfg(mobile)]
    if let Some(value) = diag::diagnostic_setting("AYLIS_ORIENTATION") {
        match aylis_core::ScreenOrientation::parse(&value) {
            Some(o) => ui.set_orientation_override(Some(o)),
            None => diag::diagnostic(&format!("aylis: AYLIS_ORIENTATION '{value}' não existe (any, portrait, landscape, sensor_portrait, sensor_landscape)")),
        }
    }
    let poster = ui.poster();
    let services = ui.services_mut();
    #[cfg(native_skia)]
    if services.offscreen.is_none() {
        services.offscreen = Some(Box::new(SkiaOffscreen));
    }
    // Só com o Direct2D (o padrão no Windows): a captura também sai por ele.
    #[cfg(all(native_d2d, not(native_skia)))]
    if services.offscreen.is_none() {
        services.offscreen = Some(Box::new(D2dOffscreen));
    }
    if services.text.is_none() {
        let _ = (renderer, chosen);
        #[cfg(native_d2d)]
        if chosen == Renderer::D2d {
            services.text = Some(Box::new(renderer.d2d.text_service()));
        }
        #[cfg(native_skia)]
        if services.text.is_none() {
            services.text = Some(Box::new(renderer.skia.text_service()));
        }
    }
    // A foto da janela com as vistas nativas (W10).
    #[cfg(target_os = "windows")]
    if services.window_capture.is_none() {
        services.window_capture = Some(Box::new(win32::WindowCapture));
    }
    // O motor de texto nativo (correção e previsão), se o app não pôs outro.
    #[cfg(target_os = "windows")]
    if services.text_assist.is_none() {
        services.text_assist = Some(Box::new(textassist_win::WindowsTextAssist::new()));
    }
    #[cfg(target_os = "windows")]
    if services.biometrics.is_none() {
        services.biometrics = Some(Box::new(hello::WindowsHello));
    }
    #[cfg(all(target_os = "linux", not(target_os = "android")))]
    if services.text_assist.is_none() {
        services.text_assist = Some(Box::new(textassist_linux::LinuxTextAssist::new()));
    }
    #[cfg(not(mobile))]
    if services.file_dialogs.is_none() {
        services.file_dialogs = Some(Box::new(SystemFileDialogs));
    }
    #[cfg(not(any(web, mobile)))]
    if services.printer.is_none() {
        services.printer = Some(Box::new(print::SystemPrinter));
    }
    #[cfg(native_clipboard)]
    if services.clipboard.is_none() {
        if let Some(clipboard) = clipboard::SystemClipboard::new() {
            services.clipboard = Some(Box::new(clipboard));
        }
    }
    #[cfg(target_os = "android")]
    if services.clipboard.is_none() {
        services.clipboard = Some(Box::new(android_jni::Clipboard));
    }
    if services.http.is_none() {
        services.http = Some(Box::new(http::NativeHttp::default()));
    }
    // Abrir endereços e apps (o `AppLink`); no desktop, o link que abriu o app chega pela
    // linha de comando.
    #[cfg(not(any(web, mobile)))]
    if services.links.is_none() {
        services.links = Some(Box::new(links::DesktopLinks));
        links::command_line_links();
    }
    // NFC por leitor USB e Bluetooth BLE no desktop (as features `nfc` e `bluetooth`).
    #[cfg(all(feature = "nfc", not(any(web, mobile))))]
    if services.nfc.is_none() {
        services.nfc = Some(Box::new(desktop_nfc::DesktopNfc::new()));
    }
    #[cfg(all(feature = "bluetooth", not(any(web, mobile))))]
    if services.bluetooth.is_none() {
        services.bluetooth = Some(Box::new(desktop_bluetooth::DesktopBluetooth::new()));
    }
    // A automação OLE (o `OleObject`): a thread do COM só nasce no primeiro uso.
    #[cfg(windows)]
    if services.ole.is_none() {
        services.ole = Some(Box::new(win_ole::WinOle::new()));
    }
    // O WMI (o `WmiQuery`): a thread das consultas só nasce na primeira.
    #[cfg(windows)]
    if services.wmi.is_none() {
        services.wmi = Some(Box::new(win_wmi::WinWmi::new()));
    }
    #[cfg(any(windows, all(target_os = "linux", not(target_os = "android"))))]
    if services.sensors.is_none() {
        services.sensors = Some(Box::new(sensors::DesktopSensors::new()));
    }
    #[cfg(not(mobile))]
    if services.gamepads.is_none() {
        services.gamepads = gamepads::GilrsPads::new().map(|p| Box::new(p) as Box<dyn aylis_core::platform::GamepadService>);
    }
    #[cfg(target_os = "android")]
    if services.gamepads.is_none() {
        services.gamepads = Some(Box::new(android_gamepad::AndroidGamepads));
    }
    #[cfg(windows)]
    if services.notifications.is_none() {
        services.notifications = Some(Box::new(notify::WindowsNotifications::new(poster.clone())));
    }
    // Notificações push: o WNS (Windows), o APNs (iOS e macOS), o UnifiedPush (Linux); o FCM
    // do Android vem com a AylisActivity, abaixo.
    #[cfg(windows)]
    if services.push.is_none() {
        services.push = Some(Box::new(windows_push::WnsPush::default()));
    }
    // Os controles de mídia do sistema (o `MediaSession`).
    #[cfg(windows)]
    if services.media_session.is_none() {
        services.media_session = Some(Box::new(windows_media::WindowsMediaSession::default()));
    }
    #[cfg(all(target_os = "linux", not(target_os = "android")))]
    if services.media_session.is_none() {
        services.media_session = Some(Box::new(linux_media::Mpris::default()));
    }
    #[cfg(any(target_os = "ios", target_os = "macos"))]
    if services.push.is_none() {
        services.push = Some(Box::new(apple_push::ApplePush::new()));
    }
    #[cfg(all(target_os = "linux", not(target_os = "android")))]
    if services.push.is_none() {
        services.push = Some(Box::new(linux_push::UnifiedPush::default()));
    }
    #[cfg(target_os = "android")]
    if services.notifications.is_none() && android_jni::has_webview() {
        services.notifications = Some(Box::new(android_jni::Notifications::new(poster.clone())));
    }
    // Arquivos, permissões e biometria do iOS.
    #[cfg(target_os = "ios")]
    {
        if services.file_dialogs.is_none() {
            services.file_dialogs = Some(Box::new(ios_device::IosFileDialogs));
        }
        if services.permissions.is_none() {
            services.permissions = Some(Box::new(ios_device::IosPermissions::new()));
        }
        if services.biometrics.is_none() {
            services.biometrics = Some(Box::new(ios_device::IosBiometrics));
        }
    }
    // Arquivos, permissões e biometria pela AylisActivity.
    #[cfg(target_os = "android")]
    if android_jni::has_webview() {
        if services.links.is_none() {
            services.links = Some(Box::new(android_device::AndroidLinks));
        }
        if services.sensors.is_none() {
            services.sensors = Some(Box::new(android_device::AndroidSensors));
        }
        if services.background.is_none() {
            services.background = Some(Box::new(android_device::AndroidBackground));
        }
        if services.phone.is_none() {
            services.phone = Some(Box::new(android_device::AndroidPhone));
        }
        if services.nfc.is_none() {
            services.nfc = Some(Box::new(android_radio::AndroidNfc));
        }
        if services.bluetooth.is_none() {
            services.bluetooth = Some(Box::new(android_radio::AndroidBluetooth));
        }
        if services.push.is_none() {
            services.push = Some(Box::new(android_push::AndroidPush));
        }
        if services.actions.is_none() {
            services.actions = Some(Box::new(android_push::AndroidActions));
        }
        if services.file_dialogs.is_none() {
            services.file_dialogs = Some(Box::new(android_device::AndroidFileDialogs));
        }
        if services.permissions.is_none() {
            services.permissions = Some(Box::new(android_device::AndroidPermissions));
        }
        if services.biometrics.is_none() {
            services.biometrics = Some(Box::new(android_device::AndroidBiometrics));
        }
        android_device::install_files();
    }
    let _ = poster;
}

/// Desenho fora da tela com o Skia na CPU (a imagem de um form na notificação).
#[cfg(native_skia)]
pub(crate) struct SkiaOffscreen;

#[cfg(native_skia)]
impl aylis_core::platform::OffscreenService for SkiaOffscreen {
    fn render_png(&self, width: u32, height: u32, scale: f32, background: aylis_core::Color, paint: &mut dyn FnMut(&mut dyn aylis_core::Canvas)) -> Option<Vec<u8>> {
        let (w, h) = ((width as f32 * scale).ceil() as u32, (height as f32 * scale).ceil() as u32);
        aylis_skia::SkiaRenderer::new().render_png(w, h, scale, background, |canvas| paint(canvas))
    }

    fn render_rgba(&self, width: u32, height: u32, scale: f32, background: aylis_core::Color, paint: &mut dyn FnMut(&mut dyn aylis_core::Canvas)) -> Option<Vec<u8>> {
        aylis_skia::SkiaRenderer::new().render_rgba(width, height, scale, background, |canvas| paint(canvas))
    }

    fn render_area_rgba(
        &self,
        width: u32,
        height: u32,
        background: aylis_core::Color,
        over: Option<(&[u8], aylis_core::RectF)>,
        paint: &mut dyn FnMut(&mut dyn aylis_core::Canvas),
    ) -> Option<Vec<u8>> {
        // O Skia pinta em `0xAARRGGBB` (BGRA lido como u32).
        let mut pixels: Vec<u32> = match over {
            Some((previous, _)) if previous.len() == (width * height * 4) as usize => {
                previous.chunks_exact(4).map(|p| 0xFF00_0000 | (p[0] as u32) << 16 | (p[1] as u32) << 8 | p[2] as u32).collect()
            }
            _ => vec![0; (width * height) as usize],
        };
        let area = over.map(|(_, a)| a);
        if !aylis_skia::SkiaRenderer::new().render_area_to_pixels(&mut pixels, width, height, 1.0, background, area, |canvas| paint(canvas)) {
            return None;
        }
        Some(pixels.iter().flat_map(|&x| [(x >> 16) as u8, (x >> 8) as u8, x as u8, 255]).collect())
    }
}

#[cfg(native_d2d)]
pub(crate) struct D2dOffscreen;

#[cfg(native_d2d)]
impl aylis_core::platform::OffscreenService for D2dOffscreen {
    fn render_png(&self, width: u32, height: u32, scale: f32, background: aylis_core::Color, paint: &mut dyn FnMut(&mut dyn aylis_core::Canvas)) -> Option<Vec<u8>> {
        let (w, h) = ((width as f32 * scale).ceil() as u32, (height as f32 * scale).ceil() as u32);
        let rgba = aylis_d2d::D2dRenderer::new().ok()?.render_rgba(w, h, scale, background, |canvas| paint(canvas))?;
        aylis_d2d::encode_png(&rgba, w, h)
    }

    fn render_rgba(&self, width: u32, height: u32, scale: f32, background: aylis_core::Color, paint: &mut dyn FnMut(&mut dyn aylis_core::Canvas)) -> Option<Vec<u8>> {
        aylis_d2d::D2dRenderer::new().ok()?.render_rgba(width, height, scale, background, |canvas| paint(canvas))
    }

    fn render_area_rgba(
        &self,
        width: u32,
        height: u32,
        background: aylis_core::Color,
        over: Option<(&[u8], aylis_core::RectF)>,
        paint: &mut dyn FnMut(&mut dyn aylis_core::Canvas),
    ) -> Option<Vec<u8>> {
        aylis_d2d::D2dRenderer::new().ok()?.render_area_rgba(width, height, 1.0, background, over, |canvas| paint(canvas))
    }
}


#[cfg(web)]
pub(crate) fn install_web_services(ui: &mut Ui) {
    // Imagens decodificadas pelo navegador chegam depois: repinta quando carregam.
    let poster = ui.poster();
    let paste = poster.clone();
    aylis_web::on_image_loaded(move || poster.post(|ui| ui.request_repaint()));
    let services = ui.services_mut();
    // Copiar e colar pela área de transferência do sistema.
    if services.clipboard.is_none() {
        services.clipboard = Some(Box::new(web::WebClipboard));
        web::watch_paste(paste);
        web::keep_mouse_navigation();
    }
    if services.text.is_none() {
        if let Ok(text) = aylis_web::WebTextService::new() {
            services.text = Some(Box::new(text));
        }
    }
    // O desenho fora da tela por um `<canvas>` solto (o texto do `PaintCanvas` que vira pixel).
    if services.offscreen.is_none() {
        services.offscreen = Some(Box::new(aylis_web::WebOffscreen));
    }
    if services.http.is_none() {
        services.http = Some(Box::new(http::WebHttp));
    }
    if services.links.is_none() {
        services.links = Some(Box::new(links::WebLinks));
    }
    // Controles de jogo pela Gamepad API.
    if services.gamepads.is_none() {
        services.gamepads = gamepads::GilrsPads::new().map(|p| Box::new(p) as Box<dyn aylis_core::platform::GamepadService>);
    }
    // Arquivos, permissões e biometria do navegador.
    if services.file_dialogs.is_none() {
        services.file_dialogs = Some(Box::new(web_device::WebFileDialogs));
    }
    if services.permissions.is_none() {
        services.permissions = Some(Box::new(web_device::WebPermissions::new()));
    }
    if services.biometrics.is_none() {
        services.biometrics = Some(Box::new(web_device::WebBiometrics::new()));
    }
    // Os controles de mídia do sistema (o `MediaSession`), pela Media Session API.
    if services.media_session.is_none() {
        services.media_session = Some(Box::new(web_media::WebMediaSession::default()));
    }
    // Web NFC e Web Bluetooth (só no Chrome; onde não houver, `available` é falso).
    if services.nfc.is_none() {
        services.nfc = Some(Box::new(web_radio::WebNfc::default()));
    }
    if services.bluetooth.is_none() {
        services.bluetooth = Some(Box::new(web_radio::WebBluetooth::default()));
    }
    // Web Push pelo service worker da publicação.
    if services.push.is_none() {
        services.push = Some(Box::new(web_push::WebPush::new()));
    }
    web_device::install_files();
}

/// Diálogos de arquivo do sistema (rfd), numa thread própria: a interface não trava.
#[cfg(not(any(web, mobile)))]
pub(crate) struct SystemFileDialogs;

#[cfg(not(any(web, mobile)))]
impl aylis_core::platform::FileDialogService for SystemFileDialogs {
    fn show(&mut self, options: aylis_core::platform::FileDialogOptions, done: Box<dyn FnOnce(Vec<String>) + Send>) {
        // A janela do form dono (no Windows, pelo HWND): o diálogo fica por cima dela.
        #[cfg(windows)]
        let parent = options.owner.and_then(win32::hwnd);
        std::thread::spawn(move || {
            let mut dialog = rfd::FileDialog::new();
            #[cfg(windows)]
            if let Some(hwnd) = parent {
                dialog = dialog.set_parent(&win32::ParentWindow(hwnd));
            }
            if !options.title.is_empty() {
                dialog = dialog.set_title(&options.title);
            }
            for (name, exts) in &options.filters {
                // No macOS o painel não entende o "*": sem esse filtro, ele já mostra todos.
                if cfg!(target_os = "macos") && exts.iter().any(|e| e == "*") {
                    continue;
                }
                dialog = dialog.add_filter(name, exts);
            }
            if !options.directory.is_empty() {
                dialog = dialog.set_directory(&options.directory);
            }
            if !options.file_name.is_empty() {
                dialog = dialog.set_file_name(&options.file_name);
            }
            let to_string = |p: std::path::PathBuf| p.display().to_string();
            let paths = if options.folder && options.multiple {
                dialog.pick_folders().unwrap_or_default().into_iter().map(to_string).collect()
            } else if options.folder {
                dialog.pick_folder().map(to_string).into_iter().collect()
            } else if options.save {
                dialog.save_file().map(to_string).into_iter().collect()
            } else if options.multiple {
                dialog.pick_files().unwrap_or_default().into_iter().map(to_string).collect()
            } else {
                dialog.pick_file().map(to_string).into_iter().collect()
            };
            #[cfg(windows)]
            if parent.is_some() {
                win32::dialog_closed();
            }
            done(paths);
        });
    }
}
