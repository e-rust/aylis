//! Celular e navegador: as barras do sistema, a orientação e a área segura.

use super::*;

impl App {
    /// Android: as barras de status e de navegação com o pedido do `SystemBars` ou, sem ele (ou
    /// sem cor), com a cor de fundo do form principal (muda junto com o tema); ícones escuros
    /// sobre fundo claro.
    #[cfg(target_os = "android")]
    pub(crate) fn sync_system_bars(&mut self) {
        let Some(form) = self.ui.main_form() else { return };
        let mut background = self.ui.background_color(form);
        if background.a == 0 {
            background = self.ui.theme().color("background");
        }
        let style = self.ui.system_bars().cloned().unwrap_or_default();
        let status = style.status_color.unwrap_or(background);
        let navigation = style.navigation_color.unwrap_or(status);
        let wanted = (status, navigation, style.dark_icons(status), style.dark_icons(navigation), style.status_visible, style.navigation_visible);
        if self.system_bars == Some(wanted) {
            return;
        }
        let argb = |c: aylis_core::Color| 0xFF00_0000 | (c.r as u32) << 16 | (c.g as u32) << 8 | c.b as u32;
        if android_jni::set_system_bars(argb(status), argb(navigation), wanted.2, wanted.3, wanted.4, wanted.5) {
            self.system_bars = Some(wanted);
        }
    }

    /// iOS: o pedido do `SystemBars` (os ícones e o esconder pelo sistema; a cor é a faixa
    /// que o núcleo pinta por baixo da barra de status).
    #[cfg(target_os = "ios")]
    pub(crate) fn sync_system_bars(&mut self) {
        use winit::platform::ios::{StatusBarStyle, WindowExtIOS};
        let version = self.ui.system_bars_version();
        if version == self.system_bars_applied {
            return;
        }
        self.system_bars_applied = version;
        let style = self.ui.system_bars().cloned().unwrap_or_default();
        let icons = match (style.icons, style.status_color) {
            (aylis_core::BarIcons::Light, _) => StatusBarStyle::LightContent,
            (aylis_core::BarIcons::Dark, _) => StatusBarStyle::DarkContent,
            (aylis_core::BarIcons::Auto, Some(c)) if style.dark_icons(c) => StatusBarStyle::DarkContent,
            (aylis_core::BarIcons::Auto, Some(_)) => StatusBarStyle::LightContent,
            (aylis_core::BarIcons::Auto, None) => StatusBarStyle::Default,
        };
        for w in &self.windows {
            w.window.set_prefers_status_bar_hidden(!style.status_visible);
            w.window.set_preferred_status_bar_style(icons);
        }
    }

    /// Navegador: a cor da barra de status vai para a `theme-color` da página (a barra do
    /// navegador no celular).
    #[cfg(web)]
    pub(crate) fn sync_system_bars(&mut self) {
        let version = self.ui.system_bars_version();
        if version == self.system_bars_applied {
            return;
        }
        self.system_bars_applied = version;
        let Some(color) = self.ui.system_bars().and_then(|s| s.status_color) else { return };
        let Some(document) = web_sys::window().and_then(|w| w.document()) else { return };
        let meta = match document.query_selector("meta[name=theme-color]").ok().flatten() {
            Some(m) => m,
            None => {
                let Ok(m) = document.create_element("meta") else { return };
                let _ = m.set_attribute("name", "theme-color");
                if let Some(head) = document.query_selector("head").ok().flatten() {
                    let _ = head.append_child(&m);
                }
                m
            }
        };
        let _ = meta.set_attribute("content", &color.to_hex());
    }

    /// Celular: a orientação pedida (a do form à frente, a do app ou a do `AYLIS_ORIENTATION`),
    /// só quando muda. Android: `Activity.setRequestedOrientation` (vale com qualquer Activity);
    /// iOS: as orientações válidas da janela (o iOS só gira para as que o `Info.plist` declara).
    #[cfg(mobile)]
    pub(crate) fn sync_orientation(&mut self) {
        let wanted = self.ui.requested_orientation();
        if self.orientation_applied == Some(wanted) {
            return;
        }
        #[cfg(target_os = "android")]
        {
            // As constantes do `ActivityInfo.SCREEN_ORIENTATION_*`.
            use aylis_core::ScreenOrientation as O;
            let code = match wanted {
                O::Any => -1,
                O::Landscape => 0,
                O::Portrait => 1,
                O::SensorLandscape => 6,
                O::SensorPortrait => 7,
            };
            if android_jni::set_requested_orientation(code) {
                self.orientation_applied = Some(wanted);
            }
        }
        #[cfg(target_os = "ios")]
        {
            use winit::platform::ios::{ValidOrientations, WindowExtIOS};
            let valid = if wanted.is_portrait() {
                ValidOrientations::Portrait
            } else if wanted.is_landscape() {
                ValidOrientations::Landscape
            } else {
                ValidOrientations::LandscapeAndPortrait
            };
            for w in &self.windows {
                w.window.set_valid_orientations(valid);
            }
            self.orientation_applied = Some(wanted);
        }
    }

    /// Passa ao form as faixas da tela cobertas pelo sistema (o layout só refaz se mudou).
    pub(crate) fn update_safe_area(&mut self, index: usize) {
        let Some(w) = self.windows.get(index) else { return };
        // O aparelho simulado: a área segura dele, no form principal.
        if let Some((device, landscape)) = simulated_device().filter(|_| self.ui.main_form() == Some(w.form)) {
            self.ui.set_safe_area(w.form, device.safe_area(landscape));
            return;
        }
        if let Some(insets) = system_insets(&w.window) {
            let form = w.form;
            let z = self.ui.form_zoom(form);
            let insets = Bounds::new(insets.left / z, insets.top / z, insets.right / z, insets.bottom / z);
            let changed = self.ui.safe_area(form) != insets;
            self.ui.set_safe_area(form, insets);
            // O teclado abriu (ou mudou de tamanho): o campo com foco fica à vista.
            if changed && self.keyboard {
                if let Some(f) = self.ui.focused(self.ui.root_of(form)) {
                    self.ui.reveal(f);
                }
            }
        }
    }
}
