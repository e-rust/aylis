//! Os pedidos à janela (`Ui::window_command`): estado, nível, forma, cantos, opacidade, tela cheia, mouse preso e manter acordado.

use super::*;

impl App {
    /// A janela do form mostrada encoberta aparece com o próximo quadro pintado.
    #[cfg_attr(not(windows), allow(unused_variables))]
    fn uncloak_after_draw(&mut self, form: ComponentId) {
        #[cfg(windows)]
        {
            self.ui.request_repaint();
            if let Some(i) = self.windows.iter().position(|w| w.form == form && !w.is_extra()) {
                self.windows[i].uncloak_after_draw = true;
            }
        }
    }

    /// A borda de redimensionar sob `p`, num form `custom` não maximizado.
    pub(crate) fn resize_edge(&self, index: usize, p: PointF) -> Option<winit::window::ResizeDirection> {
        use winit::window::ResizeDirection as D;
        let w = self.windows.get(index)?;
        // Maximizada ou em tela cheia, não redimensiona pelas bordas (em tela cheia, a borda de
        // 6 px mostrava a seta de redimensionar e puxar tirava da tela cheia; relatado em 20/11).
        if SINGLE_SURFACE || w.span.is_some() || w.window.is_maximized() || w.window.fullscreen().is_some() || self.enum_prop(w.form, "border_style") != "custom" {
            return None;
        }
        // Com o botão apertado (a aba arrastada para fora pela borda) ou com o mouse fora da
        // janela (a captura do arrasto manda os pontos de fora), a borda não toma o mouse: o
        // movimento é do app.
        if self.pointer_down || self.ui.dragging(w.form) {
            return None;
        }
        let size = w.window.inner_size().to_logical::<f32>(self.system_scale(index) * self.ui.form_zoom(w.form) as f64);
        if p.x < 0.0 || p.y < 0.0 || p.x >= size.width || p.y >= size.height {
            return None;
        }
        const EDGE: f32 = 6.0;
        let (left, right) = (p.x < EDGE, p.x > size.width - EDGE);
        let (top, bottom) = (p.y < EDGE, p.y > size.height - EDGE);
        Some(match (left, right, top, bottom) {
            (true, _, true, _) => D::NorthWest,
            (_, true, true, _) => D::NorthEast,
            (true, _, _, true) => D::SouthWest,
            (_, true, _, true) => D::SouthEast,
            (true, ..) => D::West,
            (_, true, ..) => D::East,
            (_, _, true, _) => D::North,
            (_, _, _, true) => D::South,
            _ => return None,
        })
    }

    /// A janela `index` mudou de tamanho (o `Resized`, o `ScaleFactorChanged` ou o tamanho que o
    /// winit aplicou na hora, [`request_size`]): o form, o lugar guardado, a forma e a placa
    /// acompanham.
    pub(crate) fn window_resized(&mut self, index: usize) {
        // Minimizou ou voltou: o form fica sabendo (o relatório do Tabernáculo, P06).
        let minimized = self.windows[index].window.is_minimized() == Some(true);
        if minimized != self.windows[index].minimized {
            self.windows[index].minimized = minimized;
            let form = self.windows[index].form;
            self.ui.set_form_minimized(form, minimized);
            self.ui.fire(form, if minimized { "on_minimize" } else { "on_restore" }, &aylis_core::EventArgs::None);
        }
        self.sync_form_size(index);
        self.store_placement(index);
        #[cfg(windows)]
        self.apply_shape(index);
        // Maximizar ou encaixar (Win+Shift+seta) também leva a outro monitor.
        self.follow_monitor(index, false);
        self.windows[index].window.request_redraw();
    }

    /// As janelas dos `forms` cujo tamanho o winit aplicou na hora ([`request_size`] deu
    /// `true`): o que o `Resized` faria.
    pub(crate) fn sizes_applied(&mut self, forms: &[ComponentId]) {
        for &form in forms {
            if let Some(index) = self.windows.iter().position(|w| w.form == form && !w.is_extra()) {
                // O `on_resize_end` também conta este tamanho.
                let size = self.windows[index].window.inner_size();
                self.report_window_change(index, &WindowEvent::Resized(size));
                self.window_resized(index);
            }
        }
    }

    /// Executa os pedidos à janela (`Ui::window_command`): a barra de título própria.
    pub(crate) fn apply_window_commands(&mut self) {
        use aylis_core::{ResizeEdge, WindowCommand};
        use winit::window::ResizeDirection as D;
        // As janelas que o winit redimensionou na hora (Wayland, macOS), sem `Resized` depois.
        let mut applied: Vec<ComponentId> = Vec::new();
        for (form, command) in self.ui.take_window_commands() {
            // A propriedade `gpu` do form mudou: a janela confere a placa (precisa do backend
            // mutável, então fica fora do `match` abaixo).
            if command == WindowCommand::UpdateGpu {
                self.update_gpu(form);
                continue;
            }
            // Um `WindowSettings` do form mudou: as janelas que usam as opções dele (o do app vale
            // para todas; o de um form, só para as dele).
            if command == WindowCommand::UpdateOptions {
                #[cfg(not(any(web, mobile)))]
                self.update_window_options(form);
                continue;
            }
            // O form em vários monitores (ou que pode virar): o lugar, o tamanho e a tela cheia
            // refazem os pedaços (`span.rs`).
            if self.span_command(form, command) {
                continue;
            }
            // `physical_pixels` mudou (numa janela só; os pedaços se refazem no `span_command`):
            // a janela fica onde está e o form passa a ter o tamanho dela na escala nova.
            if command == WindowCommand::UpdateScale {
                let indices: Vec<usize> = (0..self.windows.len()).filter(|&i| self.windows[i].form == form).collect();
                for index in indices {
                    self.sync_form_size(index);
                    self.windows[index].limits = [-1.0; 4];
                    self.windows[index].window.request_redraw();
                }
                continue;
            }
            // Vale para o app (o maior pedido dos forms abertos), com ou sem a janela deste.
            if command == WindowCommand::UpdateKeepAwake {
                self.update_keep_awake();
                continue;
            }
            #[cfg(not(any(web, mobile)))]
            if command == WindowCommand::UpdateCursorConfine {
                let indices: Vec<usize> = (0..self.windows.len()).filter(|&i| self.windows[i].form == form && !self.windows[i].is_extra()).collect();
                for index in indices {
                    self.apply_cursor_confine(index);
                }
                continue;
            }
            // Pega o token de ativação do `Ui` (precisa dele mutável).
            if command == WindowCommand::BringToFront {
                self.bring_window_to_front(form);
                continue;
            }
            let Some(w) = self.window_of(form) else { continue };
            let window = &w.window;
            match command {
                WindowCommand::Minimize => window.set_minimized(true),
                WindowCommand::Maximize => window.set_maximized(true),
                WindowCommand::Restore => {
                    window.set_minimized(false);
                    window.set_maximized(false);
                }
                WindowCommand::ToggleMaximize => window.set_maximized(!window.is_maximized()),
                WindowCommand::StartDrag => {
                    let _ = window.drag_window();
                }
                WindowCommand::StartResize(edge) => {
                    let _ = window.drag_resize_window(match edge {
                        ResizeEdge::Left => D::West,
                        ResizeEdge::Right => D::East,
                        ResizeEdge::Top => D::North,
                        ResizeEdge::Bottom => D::South,
                        ResizeEdge::TopLeft => D::NorthWest,
                        ResizeEdge::TopRight => D::NorthEast,
                        ResizeEdge::BottomLeft => D::SouthWest,
                        ResizeEdge::BottomRight => D::SouthEast,
                    });
                }
                WindowCommand::SetTopmost(on) => {
                    window.set_window_level(if on { winit::window::WindowLevel::AlwaysOnTop } else { winit::window::WindowLevel::Normal });
                    #[cfg(windows)]
                    win32::watch_topmost(form, on);
                }
                WindowCommand::UpdateLevel => {
                    let level = self.window_level(form);
                    window.set_window_level(level);
                    // O winit só chama o sistema quando o nível muda: outra janela sempre por cima
                    // que passou à frente não perdia o lugar. Reafirmado sempre, sem ativar.
                    #[cfg(windows)]
                    if level == winit::window::WindowLevel::AlwaysOnTop {
                        win32::reassert_topmost(window);
                    }
                    // De volta à frente: o próximo "perdeu o topo" avisa de novo.
                    #[cfg(windows)]
                    {
                        win32::watch_topmost(form, level == winit::window::WindowLevel::AlwaysOnTop);
                        win32::check_topmost();
                    }
                }
                WindowCommand::RequestAttention(critical) => window.request_user_attention(Some(if critical {
                    winit::window::UserAttentionType::Critical
                } else {
                    winit::window::UserAttentionType::Informational
                })),
                WindowCommand::UpdateOpacity => {
                    #[cfg(windows)]
                    win32::set_opacity(window, self.form_opacity(form));
                }
                WindowCommand::Hide => {
                    window.set_visible(false);
                    // O app pode ter mostrado a janela pela API do Windows: o winit acha
                    // que ela já está escondida e não faz nada. Escondida de verdade.
                    #[cfg(windows)]
                    win32::set_shown(window, false);
                }
                // O form que não toma o foco (`no_activate`, `activate_on_show` desligado) volta
                // sem ativar: o painel sempre por cima que some e reaparece não tira o teclado de
                // quem digita em outro programa.
                WindowCommand::Show => {
                    let activate = self.takes_focus(form);
                    if show_window(&self.ui, form, window, activate) {
                        self.uncloak_after_draw(form);
                    }
                }
                WindowCommand::SetFullscreen(on) => {
                    #[cfg(windows)]
                    win32::sync_shown(window);
                    let monitor = match self.ui.get_prop(form, "monitor") {
                        Ok(PropValue::Int(i)) if i >= 0 => window.available_monitors().nth(i as usize),
                        _ => None,
                    };
                    let monitor = monitor.or_else(|| window.current_monitor());
                    window.set_fullscreen(on.then(|| self.full_screen(form, monitor)))
                }
                // No meio da área útil do monitor; maximizada, volta, muda e maximiza de novo.
                WindowCommand::MoveToMonitor(index) => {
                    let Some(target) = window.available_monitors().nth(index) else { continue };
                    // Em tela cheia: sai dela, a janela comum vai para o outro monitor e entra em
                    // tela cheia lá. Trocando direto, o lugar guardado para a volta ao normal era o
                    // da tela cheia, e a janela voltava do tamanho do monitor (testado em 20/11).
                    let full = window.fullscreen().is_some();
                    if full {
                        window.set_fullscreen(None);
                    }
                    let maximized = window.is_maximized();
                    if maximized {
                        window.set_maximized(false);
                    }
                    let (origin, area) = self.work_rect(&target);
                    let size = window.outer_size();
                    let x = origin.x + (area.width as i32 - size.width as i32).max(0) / 2;
                    let y = origin.y + (area.height as i32 - size.height as i32).max(0) / 2;
                    window.set_outer_position(PhysicalPosition::new(x, y));
                    if maximized {
                        window.set_maximized(true);
                    }
                    if full {
                        window.set_fullscreen(Some(self.full_screen(form, Some(target))));
                    }
                }
                WindowCommand::MoveTo(x, y) => {
                    if window.is_maximized() {
                        window.set_maximized(false);
                    }
                    window.set_outer_position(PhysicalPosition::new(x, y));
                    // O form que não toma o foco (`no_activate`, `activate_on_show` desligado)
                    // continua sem ele depois de mudar de lugar.
                    if self.takes_focus(form) {
                        window.focus_window();
                    }
                }
                // Lugar e tamanho em pixels físicos, a partir do canto do monitor:
                // sem a tela cheia nem o maximizar, e sem tomar o foco, salvo com `activate`.
                WindowCommand::Place { monitor, x, y, width, height, activate } => {
                    let origin = window.available_monitors().nth(monitor).filter(monitors::valid).map_or(PhysicalPosition::new(0, 0), |m| m.position());
                    if window.fullscreen().is_some() {
                        window.set_fullscreen(None);
                    }
                    if window.is_maximized() {
                        window.set_maximized(false);
                    }
                    let (x, y) = (origin.x.saturating_add(x), origin.y.saturating_add(y));
                    #[cfg(windows)]
                    win32::place(window, x, y, width.max(1), height.max(1), activate);
                    #[cfg(not(windows))]
                    {
                        window.set_outer_position(PhysicalPosition::new(x, y));
                        // O winit pede o tamanho de dentro: tira a moldura que a janela tem agora.
                        let (outer, inner) = (window.outer_size(), window.inner_size());
                        let frame = (outer.width.saturating_sub(inner.width), outer.height.saturating_sub(inner.height));
                        if request_size(window, winit::dpi::PhysicalSize::new(width.saturating_sub(frame.0).max(1), height.saturating_sub(frame.1).max(1))) {
                            applied.push(form);
                        }
                        if activate {
                            window.focus_window();
                        }
                    }
                    // Escondida pelo `Hide`: aparece no lugar novo, ativando só com `activate` (o
                    // `SetWindowPos` sem ativar não mostra a janela escondida).
                    if window.is_visible() == Some(false) {
                        if show_window(&self.ui, form, window, activate) {
                            self.uncloak_after_draw(form);
                        }
                    } else {
                        #[cfg(windows)]
                        if activate {
                            bring_to_front(&self.ui, form, window);
                        }
                    }
                }
                // A área cliente do tamanho do form (J11): o tamanho pedido é o de dentro, e o
                // `Resized` que vem depois confirma (ou o sistema limita, e o form fica com o que
                // coube). No Wayland não vem `Resized`: o winit aplica na hora (`request_size`), e
                // a superfície e o form acompanham no fim desta volta. No celular e no navegador
                // o form ocupa a tela: nada a fazer.
                WindowCommand::ApplySize => {
                    if let Some(cb) = self.ui.control(form).filter(|_| !SINGLE_SURFACE) {
                        let zoom = self.ui.form_zoom(form) as f64;
                        let want = LogicalSize::new(cb.width() as f64 * zoom, cb.height() as f64 * zoom);
                        // Em pixels físicos (`physical_pixels`), a unidade é o pixel.
                        let scale = if self.ui.physical_pixels(form) { 1.0 } else { window.scale_factor() };
                        let now = window.inner_size().to_logical::<f64>(scale);
                        if (now.width - want.width).abs() > 0.5 || (now.height - want.height).abs() > 0.5 {
                            if request_size(window, want.to_physical::<u32>(scale)) {
                                applied.push(form);
                            }
                        }
                    }
                }
                // Os cantos (`window_corners`, ou a `border_style` que muda o padrão deles).
                // A forma e a transparência também (`window_shape`, `shape_radius`, `transparent`).
                #[cfg(windows)]
                WindowCommand::UpdateCorners => {
                    win32::set_window_corners(window, self.corner_preference(form));
                    window.set_transparent(self.see_through(form));
                    if self.form_shape(form).is_some() {
                        window.set_decorations(false);
                    }
                    if let Some(index) = self.windows.iter().position(|w| w.form == form) {
                        self.apply_shape(index);
                    }
                    self.update_gpu(form);
                }
                #[cfg(not(windows))]
                WindowCommand::UpdateCorners => {}
                WindowCommand::Close | WindowCommand::BringToFront | WindowCommand::UpdateGpu | WindowCommand::UpdateSpan | WindowCommand::UpdateScale | WindowCommand::UpdateOptions | WindowCommand::UpdateKeepAwake | WindowCommand::UpdateCursorConfine => {}
            }
        }
        self.sizes_applied(&applied);
        // As opções que mudaram com a janela escondida, agora que ela apareceu (pelo `Show`, pelo
        // `Place` ou por outro caminho).
        #[cfg(not(any(web, mobile)))]
        for index in 0..self.windows.len() {
            if self.windows[index].options_pending && self.window_shown(index) {
                self.apply_pending_options(index);
            }
        }
    }

    /// O botão maximizar das barras próprias (Windows): o mouse que chegou pela moldura vai
    /// para o `Ui` (a barra realça e clica), e a área de cada um, em pixels, vai para a
    /// subclass, que a diz ao Windows (no 11, o menu de layouts com o mouse parado).
    #[cfg(windows)]
    pub(crate) fn caption_buttons(&mut self) {
        use aylis_core::PointerButton;
        for (form, kind, x, y) in win32::take_caption_input() {
            let Some(w) = self.window_of(form) else { continue };
            let scale = if self.ui.physical_pixels(form) { 1.0 } else { w.window.scale_factor() };
            let p = winit::dpi::PhysicalPosition::new(f64::from(x), f64::from(y)).to_logical::<f32>(scale);
            let at = |button| PointerEvent { position: PointF::new(p.x, p.y), button, kind: PointerKind::Mouse, modifiers: self.modifiers, ..Default::default() };
            match kind {
                win32::CaptionInput::Move => self.ui.pointer_move(form, at(Default::default())),
                win32::CaptionInput::Down => {
                    self.ui.pointer_move(form, at(Default::default()));
                    self.ui.pointer_down(form, at(PointerButton::Primary));
                }
                win32::CaptionInput::Up => self.ui.pointer_up(form, at(PointerButton::Primary)),
                win32::CaptionInput::Leave => self.ui.pointer_leave(form),
            }
        }
        // Um form em vários monitores não maximiza (e a janela principal não começa no canto dele).
        for w in self.windows.iter().filter(|w| !w.is_extra()) {
            let system = if self.ui.physical_pixels(w.form) { 1.0 } else { w.window.scale_factor() };
            let scale = system as f32 * self.ui.form_zoom(w.form);
            let rect = self.ui.maximize_button(w.form).filter(|_| w.span.is_none()).map(|r| {
                ((r.x * scale).round() as i32, (r.y * scale).round() as i32, (r.right() * scale).round() as i32, (r.bottom() * scale).round() as i32)
            });
            win32::set_maximize_button(w.form, rect);
        }
    }

    /// Os cantos que o form pede ao DWM (o valor do `DWM_WINDOW_CORNER_PREFERENCE`): o padrão
    /// com a barra própria (`custom`) é o de uma janela comum do Windows 11, arredondado; com
    /// `none` (a tela de abertura, a projeção), o do sistema, que deixa retos.
    #[cfg(windows)]
    pub(crate) fn corner_preference(&self, form: ComponentId) -> i32 {
        // Com forma própria, o DWM não arredonda por cima dela.
        if self.form_shape(form).is_some() {
            return 1;
        }
        crate::corner_preference(&self.enum_prop(form, "window_corners"), &self.enum_prop(form, "border_style"))
    }

    /// A forma da janela do form (`window_shape`; `None`: o retângulo, ou a janela que não fica
    /// transparente: [`Self::transparency_supported`]).
    pub(crate) fn form_shape(&self, form: ComponentId) -> Option<aylis_core::ClipShape> {
        if SINGLE_SURFACE || !self.transparency_supported() {
            return None;
        }
        self.ui.component(form).and_then(|c| c.clip_shape())
    }

    /// A janela pode ter alfa por pixel: no celular e no navegador o form é a tela; no Windows,
    /// só com o Direct2D (o Skia apresenta pelo OpenGL ou pelo GDI, sem o alfa da composição:
    /// a janela fica opaca e retangular, e o form pinta a forma sobre a cor de fundo).
    pub(crate) fn transparency_supported(&self) -> bool {
        if SINGLE_SURFACE {
            return false;
        }
        #[cfg(native_d2d)]
        return self.options.renderer == Renderer::D2d;
        #[cfg(all(windows, not(native_d2d)))]
        return false;
        #[cfg(not(windows))]
        return true;
    }

    /// A janela do form deixa ver o que está atrás: `transparent` ou uma forma própria.
    pub(crate) fn see_through(&self, form: ComponentId) -> bool {
        self.transparency_supported() && (matches!(self.ui.get_prop(form, "transparent"), Ok(PropValue::Bool(true))) || self.form_shape(form).is_some())
    }

    /// A região da janela `index` na forma do form (Windows: os cliques fora dela vão para trás)
    /// e o desfoque do `WindowSettings.blur`, recortado pela forma.
    #[cfg(windows)]
    pub(crate) fn apply_shape(&self, index: usize) {
        if let Some(w) = self.windows.get(index) {
            self.shape_window(&w.window, w.form);
        }
    }

    #[cfg(windows)]
    pub(crate) fn shape_window(&self, window: &Window, form: ComponentId) {
        let scale = if self.ui.physical_pixels(form) { 1.0 } else { window.scale_factor() } * self.ui.form_zoom(form) as f64;
        let shape = self.form_shape(form);
        win32::set_window_shape(window, shape.clone(), scale);
        // O winit não desfoca no Windows: pela composição (o vidro fosco do form transparente).
        let blur = self.ui.window_settings_of(form).is_some() && self.ui.window_options(form).blur;
        win32::set_window_blur(window, blur, shape, scale);
    }

    /// O nível da janela do form: por cima de todas (`topmost`), por baixo (`bottommost`) ou
    /// normal.
    pub(crate) fn window_level(&self, form: ComponentId) -> winit::window::WindowLevel {
        use winit::window::WindowLevel;
        let on = |name| matches!(self.ui.get_prop(form, name), Ok(PropValue::Bool(true)));
        if on("topmost") {
            WindowLevel::AlwaysOnTop
        } else if on("bottommost") {
            WindowLevel::AlwaysOnBottom
        } else {
            WindowLevel::Normal
        }
    }

    /// O `window_opacity` do form (1 sem a propriedade).
    #[cfg_attr(not(windows), allow(dead_code))]
    pub(crate) fn form_opacity(&self, form: ComponentId) -> f32 {
        match self.ui.get_prop(form, "window_opacity") {
            Ok(PropValue::Float(v)) => (v as f32).clamp(0.05, 1.0),
            _ => 1.0,
        }
    }

    /// A tela cheia do form no `monitor`: sem bordas ou, com `full_screen_mode = exclusive`, o
    /// modo de vídeo do monitor mais perto do pedido (`video_width`, `video_height`,
    /// `video_refresh`; zero: o atual). Sem modos (o monitor não diz), sem bordas.
    pub(crate) fn full_screen(&self, form: ComponentId, monitor: Option<winit::monitor::MonitorHandle>) -> winit::window::Fullscreen {
        use winit::window::Fullscreen;
        if self.enum_prop(form, "full_screen_mode") != "exclusive" {
            return Fullscreen::Borderless(monitor);
        }
        let Some(m) = monitor else { return Fullscreen::Borderless(None) };
        let int = |name| match self.ui.get_prop(form, name) {
            Ok(PropValue::Int(v)) if v > 0 => Some(v as u32),
            _ => None,
        };
        let size = monitors::size(&m);
        let width = int("video_width").unwrap_or(size.width);
        let height = int("video_height").unwrap_or(size.height);
        let refresh = match self.ui.get_prop(form, "video_refresh") {
            Ok(PropValue::Float(v)) if v > 0.0 => v,
            _ => m.refresh_rate_millihertz().map_or(60.0, |mhz| mhz as f64 / 1000.0),
        };
        let distance = |v: &winit::monitor::VideoModeHandle| {
            let s = v.size();
            let hz = v.refresh_rate_millihertz() as f64 / 1000.0;
            let pixels = (s.width as i64 - width as i64).abs() + (s.height as i64 - height as i64).abs();
            pixels as f64 * 1000.0 + (hz - refresh).abs() * 10.0 + (32 - v.bit_depth().min(32)) as f64
        };
        match m.video_modes().min_by(|a, b| distance(a).total_cmp(&distance(b))) {
            Some(mode) => Fullscreen::Exclusive(mode),
            None => Fullscreen::Borderless(Some(m)),
        }
    }

    /// Junta o `keep_awake` dos forms abertos e pede ao sistema (só quando muda).
    pub(crate) fn update_keep_awake(&mut self) {
        let level = self
            .windows
            .iter()
            .map(|w| match self.enum_prop(w.form, "keep_awake").as_str() {
                "display" => 2,
                "system" => 1,
                _ => 0,
            })
            .max()
            .unwrap_or(0);
        if level != self.keep_awake {
            self.keep_awake = level;
            #[cfg(windows)]
            win32::keep_awake(level);
        }
    }

    /// O `confine_cursor` da janela `index`, ativa ou não: prende na janela (o winit) ou no
    /// monitor dela (no Windows, o `ClipCursor`; nas outras, como na janela), ou solta.
    #[cfg(not(any(web, mobile)))]
    pub(crate) fn apply_cursor_confine(&mut self, index: usize) {
        use winit::window::CursorGrabMode;
        let Some(w) = self.windows.get(index) else { return };
        let mode = self.enum_prop(w.form, "confine_cursor");
        let active = w.focused;
        #[cfg(windows)]
        {
            if mode == "monitor" && active {
                let _ = w.window.set_cursor_grab(CursorGrabMode::None);
                if let Some(m) = w.window.current_monitor() {
                    let (at, size) = (m.position(), monitors::size(&m));
                    win32::clip_cursor(Some((at.x, at.y, at.x + size.width as i32, at.y + size.height as i32)));
                }
                self.windows[index].clipped_monitor = true;
                return;
            }
            if w.clipped_monitor {
                win32::clip_cursor(None);
            }
        }
        let grab = match mode.as_str() {
            "window" | "monitor" if active => CursorGrabMode::Confined,
            _ => CursorGrabMode::None,
        };
        let _ = w.window.set_cursor_grab(grab);
        #[cfg(windows)]
        {
            self.windows[index].clipped_monitor = false;
        }
    }
}

/// Mostra a janela do form (escondida, minimizada ou atrás), ativando só com `activate`.
/// Devolve se ela estava escondida e foi mostrada encoberta (no Windows, aparece com o
/// próximo quadro, sem o quadro de antes).
///
/// Sem ativar, no Windows, a janela é mostrada com `SW_SHOWNOACTIVATE` antes de o winit saber:
/// o `set_visible` dele faz `SW_SHOW` (que ativa) quando a janela nasceu ativável, e sobre a
/// janela já visível não faz nada.
fn show_window(ui: &Ui, form: ComponentId, window: &Window, activate: bool) -> bool {
    #[cfg(windows)]
    let hidden_before = window.is_visible() == Some(false);
    #[cfg(windows)]
    if hidden_before {
        win32::set_cloaked(window, true);
    }
    #[cfg(windows)]
    if !activate {
        win32::show_no_activate(window);
    }
    window.set_visible(true);
    // Sem saber (o X11 e o Wayland às vezes não dizem), desminimiza do mesmo jeito.
    if window.is_minimized() != Some(false) {
        window.set_minimized(false);
    }
    #[cfg(windows)]
    win32::set_shown(window, true);
    if activate {
        bring_to_front(ui, form, window);
    }
    #[cfg(windows)]
    if hidden_before {
        window.request_redraw();
    }
    #[cfg(windows)]
    return hidden_before;
    #[cfg(not(windows))]
    false
}

/// Pede o tamanho de dentro da janela. `true` quando o winit o aplicou na hora: no Wayland (o
/// tamanho é do cliente, `xdg_surface.set_window_geometry`) e no macOS ele devolve o tamanho novo
/// e não manda `Resized` (a doc do `request_inner_size`: "resize event in such case may not be
/// generated"), então quem chamou faz o que o `Resized` faria ([`App::sizes_applied`]). Sem isso,
/// no GNOME a superfície ficava do tamanho de antes e a janela não mudava (conferido em 24/11).
/// Nos outros, o pedido vai ao sistema e o `Resized` chega depois (`false`).
pub(crate) fn request_size(window: &Window, size: winit::dpi::PhysicalSize<u32>) -> bool {
    let before = window.inner_size();
    window.request_inner_size(size).is_some_and(|now| now != before)
}
