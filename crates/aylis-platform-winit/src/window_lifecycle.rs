//! Abrir e fechar as janelas para acompanhar os forms abertos no `Ui`: os atributos, o lugar guardado, a primeira aparição e as opções do `WindowSettings`.

use super::*;

impl App {
    /// Atributos da janela de um form: título, tamanho, borda e posição.
    pub(crate) fn attributes(&self, event_loop: &ActiveEventLoop, form: ComponentId) -> WindowAttributes {
        let (width, height) = self.ui.control(form).map_or((640.0, 480.0), |cb| (cb.width(), cb.height()));
        // Com zoom, a janela cresce junto: o form aparece com o tamanho de projeto.
        let zoom = self.ui.form_zoom(form) as f64;
        // Em pixels físicos (`physical_pixels`), o tamanho do form já é em pixels.
        let physical = self.ui.physical_pixels(form);
        let size: winit::dpi::Size = if physical { PhysicalSize::new(width as f64, height as f64).into() } else { LogicalSize::new(width as f64 * zoom, height as f64 * zoom).into() };
        let mut attributes = Window::default_attributes()
            .with_title(self.title(form))
            .with_inner_size(size)
            // A barra de título do sistema clara ou escura como o tema (no Windows, pelo DWM;
            // o relatório do Tabernáculo, J04).
            .with_theme(Some(if self.ui.theme().dark { winit::window::Theme::Dark } else { winit::window::Theme::Light }));
        // Mostrar sem tomar o foco (`activate_on_show` desligado; J05).
        if matches!(self.ui.get_prop(form, "activate_on_show"), Ok(PropValue::Bool(false))) {
            attributes = attributes.with_active(false);
        }
        match self.enum_prop(form, "border_style").as_str() {
            "single" => {
                attributes = attributes
                    .with_resizable(false)
                    .with_enabled_buttons(WindowButtons::CLOSE | WindowButtons::MINIMIZE);
            }
            "none" => attributes = attributes.with_decorations(false),
            // macOS: a barra do sistema transparente, sem título nem botões, e o conteúdo por
            // baixo dela. A janela continua com moldura, então com os cantos arredondados e a
            // sombra do sistema (sem moldura, sairia reta); `window_corners = square` tira a
            // moldura como nas outras plataformas.
            #[cfg(target_os = "macos")]
            "custom" if self.enum_prop(form, "window_corners") != "square" => {
                use winit::platform::macos::WindowAttributesExtMacOS;
                attributes = attributes
                    .with_resizable(true)
                    .with_titlebar_transparent(true)
                    .with_title_hidden(true)
                    .with_titlebar_buttons_hidden(true)
                    .with_fullsize_content_view(true)
                    .with_movable_by_window_background(false);
            }
            // Barra de título própria (TitleBar): sem a do sistema, redimensionável pelas
            // bordas. No Windows 11 os cantos vêm do DWM (`window_corners`).
            "custom" => attributes = attributes.with_decorations(false).with_resizable(true),
            _ => {}
        }
        // Transparente ou com forma (`transparent`, `window_shape`): o alfa por pixel; com forma,
        // sem a moldura do sistema.
        if self.see_through(form) {
            attributes = attributes.with_transparent(true);
            if self.form_shape(form).is_some() {
                attributes = attributes.with_decorations(false).with_resizable(false);
            }
            // No Direct2D o quadro chega pela composição: sem a superfície do GDI, que apareceria
            // por trás (o fundo branco e a barra de título do sistema) com o desfoque ligado.
            #[cfg(native_d2d)]
            {
                use winit::platform::windows::WindowAttributesExtWindows;
                attributes = attributes.with_no_redirection_bitmap(true);
            }
        }
        if SINGLE_SURFACE {
            return attributes;
        }
        #[cfg(not(any(web, mobile)))]
        {
            attributes = attributes.with_window_icon(self.icon.clone());
        }
        let owner = self.ui.form_owner(form).and_then(|o| self.window_of(o)).map(|w| w.window.clone());
        // Onde a janela estava (o app guardou): vale se ainda cair numa tela ligada.
        let placement = self.ui.window_placement(form);
        let saved = placement.filter(|p| {
            event_loop.available_monitors().any(|m| {
                let (at, size) = (m.position(), monitors::size(&m));
                p.x + 40 >= at.x && p.y >= at.y && p.x + 40 < at.x + size.width as i32 && p.y + 20 < at.y + size.height as i32
            })
        });
        if let Some(p) = saved.filter(|p| p.width > 0 && p.height > 0) {
            attributes = attributes.with_inner_size(PhysicalSize::new(p.width, p.height));
        }
        if let Some(p) = placement {
            attributes = attributes.with_maximized(p.maximized);
        }
        // Com o lugar guardado, a tela cheia é a do monitor dele (o guardado vale mais que o
        // `monitor` do form, como a posição); o `window_state` pedido ou o guardado ligam.
        let full_saved = saved.is_some_and(|p| p.full_screen);
        // O monitor pedido no form (`monitor`, o índice; -1 = o do dono ou o principal).
        let chosen = match self.ui.get_prop(form, "monitor") {
            Ok(PropValue::Int(i)) if i >= 0 => event_loop.available_monitors().nth(i as usize),
            _ => None,
        };
        // O `window_state` do form (o desenhado vale mais que o guardado). A tela cheia vai para
        // o monitor pedido (J07 do Tabernáculo); sem ele, o da janela.
        match self.enum_prop(form, "window_state").as_str() {
            "maximized" => attributes = attributes.with_maximized(true),
            "full_screen" => {
                let monitor = if saved.is_some() { None } else { chosen.clone() };
                let monitor = monitor.or_else(|| event_loop.primary_monitor());
                attributes = attributes.with_fullscreen(Some(self.full_screen(form, monitor)));
            }
            "normal" if full_saved => attributes = attributes.with_fullscreen(Some(winit::window::Fullscreen::Borderless(None))),
            _ => {}
        }
        let position = match self.enum_prop(form, "position").as_str() {
            _ if saved.is_some() => saved.map(|p| PhysicalPosition::new(p.x, p.y)),
            // `left`/`top` a partir do canto do monitor pedido (sem ele, da área de trabalho).
            "designed" => self.ui.control(form).map(|cb| {
                let origin = chosen.as_ref().map_or(PhysicalPosition::new(0, 0), |m| m.position());
                PhysicalPosition::new(origin.x + cb.left() as i32, origin.y + cb.top() as i32)
            }),
            "default" if chosen.is_none() => None,
            // Num canto da área útil (a do monitor pedido ou do principal; J10 do Tabernáculo).
            corner @ ("top_left" | "top_right" | "bottom_left" | "bottom_right") => {
                let monitor = chosen.clone().or_else(|| event_loop.primary_monitor());
                monitor.map(|m| {
                    let (origin, size) = self.work_rect(&m);
                    let scale = if physical { 1.0 } else { m.scale_factor() };
                    let (w, h) = ((width as f64 * scale) as i32, (height as f64 * scale) as i32);
                    let x = if corner.ends_with("left") { origin.x } else { origin.x + size.width as i32 - w };
                    let y = if corner.starts_with("top") { origin.y } else { origin.y + size.height as i32 - h };
                    PhysicalPosition::new(x, y)
                })
            }
            choice => {
                // Centro do dono (se houver, a escolha for "center" e nenhum monitor foi
                // pedido) ou da área útil do monitor (o pedido ou o principal).
                let area = match owner.as_ref().filter(|_| choice == "center" && chosen.is_none()) {
                    // A dona minimizada: o lugar dela quando aberta (senão o diálogo abria em
                    // ≈ -21000, fora da tela).
                    #[cfg(windows)]
                    Some(o) if o.is_minimized() == Some(true) => {
                        win32::normal_rect(o).or_else(|| event_loop.primary_monitor().map(|m| self.work_rect(&m)))
                    }
                    #[cfg(not(windows))]
                    Some(o) if o.is_minimized() == Some(true) => event_loop.primary_monitor().map(|m| self.work_rect(&m)),
                    Some(o) => o.outer_position().ok().map(|p| (p, o.outer_size())),
                    None => chosen.clone().or_else(|| event_loop.primary_monitor()).map(|m| self.work_rect(&m)),
                };
                let scale = if physical { 1.0 } else { chosen.clone().or_else(|| event_loop.primary_monitor()).map_or(1.0, |m| m.scale_factor()) };
                area.map(|(origin, size)| {
                    let w = (width as f64 * scale) as i32;
                    let h = (height as f64 * scale) as i32;
                    PhysicalPosition::new(origin.x + (size.width as i32 - w) / 2, origin.y + (size.height as i32 - h) / 2)
                })
            }
        };
        if let Some(p) = position {
            attributes = attributes.with_position(p);
        }
        attributes = attributes.with_window_level(self.window_level(form));
        // Linux: o identificador do app (o `app_id` do Wayland e o `WM_CLASS` do X11) é o nome do
        // executável, o mesmo do `.desktop` (o da IDE e o do `.deb` dos apps): sem ele, o GNOME
        // mostrava a janela como "Desconhecido", sem o ícone (testado no Fedora em 20/11).
        #[cfg(all(target_os = "linux", not(target_os = "android")))]
        if let Some(id) = std::env::current_exe().ok().and_then(|p| p.file_stem().map(|s| s.to_string_lossy().to_string())) {
            use winit::platform::x11::WindowAttributesExtX11;
            attributes = attributes.with_name(id.clone(), id);
        }
        #[cfg(target_os = "windows")]
        if matches!(self.ui.get_prop(form, "show_in_taskbar"), Ok(PropValue::Bool(false))) {
            use winit::platform::windows::WindowAttributesExtWindows;
            attributes = attributes.with_skip_taskbar(true);
        }
        // Windows: a janela de um form aberto por outro é "possuída" por ele (fica por
        // cima dele e minimiza junto).
        #[cfg(target_os = "windows")]
        if let Some(owner) = &owner {
            use winit::platform::windows::WindowAttributesExtWindows;
            use winit::raw_window_handle::{HasWindowHandle, RawWindowHandle};
            if let Ok(RawWindowHandle::Win32(h)) = owner.window_handle().map(|h| h.as_raw()) {
                attributes = attributes.with_owner_window(h.hwnd.get());
            }
        }
        attributes
    }

    /// `remember_placement`: a posição guardada do form vira a dele, se ainda cabe nos
    /// monitores de agora (senão ele abre como se não houvesse).
    #[cfg(not(any(web, mobile)))]
    pub(crate) fn restore_remembered(&mut self, form: ComponentId) {
        if !matches!(self.ui.get_prop(form, "remember_placement"), Ok(PropValue::Bool(true))) || self.ui.window_placement(form).is_some() {
            return;
        }
        let name = self.ui.name(form).unwrap_or_default().to_string();
        if let Some(p) = self.placements.get(&name).and_then(|s| aylis_core::monitors::restore_placement(s, self.ui.monitors())) {
            self.ui.set_window_placement(form, p);
        }
    }

    pub(crate) fn open_window(&mut self, event_loop: &ActiveEventLoop, form: ComponentId) -> Result<(), Error> {
        #[cfg(not(any(web, mobile)))]
        self.restore_remembered(form);
        let mut attributes = self.attributes(event_loop, form);
        // O form que já nasce em vários monitores (`span_monitors`): a janela principal nasce
        // no pedaço dela, sem moldura e escondida; as outras abrem logo depois dela, e todas
        // aparecem juntas (`span.rs`).
        let spanned = self.plan_span(form, &mut attributes);
        // Windows: a tela cheia vem depois de a janela e o alvo de desenho existirem. Nascida em
        // tela cheia, sem dona e por cima de tudo, a swap chain é recusada (DXGI_ERROR_INVALID_CALL)
        // e a janela não abre (o relatório do Tabernáculo, R01; na 0.1.0 caía no dxgi.dll).
        #[cfg(windows)]
        let fullscreen_later = attributes.fullscreen.take();
        // Com acessibilidade, a janela nasce invisível: o AccessKit se liga antes de ela
        // aparecer (exigência dele). `AYLIS_ACCESSIBILITY=0` desliga.
        #[cfg(not(web))]
        let accessible = setting("AYLIS_ACCESSIBILITY").as_deref() != Some("0");
        #[cfg(not(web))]
        let hidden = self.ui.take_start_hidden(form);
        // A janela nasce escondida e só aparece com o primeiro quadro pintado (sem o branco da
        // abertura): o AccessKit também se liga antes de ela aparecer (exigência dele), e a falta
        // da animação do Windows (`animation` do `WindowSettings`) já vale na primeira aparição.
        #[cfg(not(web))]
        let attributes = if !cfg!(mobile) { attributes.with_visible(false) } else { attributes };
        // Windows: um form modal tem como dono a janela principal e fica sempre na frente
        // dela, mesmo aberto antes de ela aparecer (a pergunta de confiança ao abrir a IDE).
        // Um modal com a janela principal minimizada: ela volta antes (o modal bloqueia a
        // principal, e minimizada o usuário não vê nem um nem outro).
        #[cfg(not(any(web, mobile)))]
        if self.ui.is_modal(form) {
            if let Some(main) = self.windows.first().filter(|w| w.window.is_minimized() == Some(true)) {
                main.window.set_minimized(false);
            }
        }
        #[cfg(windows)]
        use winit::raw_window_handle::HasWindowHandle;
        #[cfg(windows)]
        let modal_owner = self.ui.form_owner(form).and_then(|o| self.window_of(o)).or_else(|| self.windows.first());
        #[cfg(windows)]
        let attributes = match modal_owner.filter(|_| self.ui.is_modal(form)).map(|w| w.window.window_handle().map(|h| h.as_raw())) {
            Some(Ok(winit::raw_window_handle::RawWindowHandle::Win32(owner))) => {
                use winit::platform::windows::WindowAttributesExtWindows;
                attributes.with_owner_window(owner.hwnd.get())
            }
            _ => attributes,
        };
        #[cfg(not(web))]
        let (window, backend) = create_backend(event_loop, attributes, self.options.renderer, &self.renderer, self.gpu_choice(form), self.present_mode(form))?;
        #[cfg(web)]
        let (window, backend) = create_backend(event_loop, attributes, self.options.renderer)?;
        // Windows: as mensagens da janela passam pelos ganchos do app e pela barra de
        // tarefas.
        #[cfg(windows)]
        win32::attach(&window, form, &self.win_proxy);
        // O fundo que a janela mostra antes do primeiro quadro: a cor do form.
        #[cfg(windows)]
        win32::set_background(form, self.erase_color(form));
        #[cfg(windows)]
        win32::set_window_corners(&window, self.corner_preference(form));
        #[cfg(windows)]
        self.shape_window(&window, form);
        // O `WindowSettings` do form (ou do app), com a janela ainda escondida. Sem ele, a janela
        // fica como o sistema a cria.
        #[cfg(not(any(web, mobile)))]
        if self.ui.window_settings_of(form).is_some() {
            apply_window_options(&window, &self.ui.window_options(form));
        }
        #[cfg(windows)]
        if self.form_opacity(form) < 1.0 {
            win32::set_opacity(&window, self.form_opacity(form));
        }
        #[cfg(windows)]
        if let Some(fullscreen) = fullscreen_later.filter(|_| !self.spans_full_screen(form)) {
            window.set_fullscreen(Some(fullscreen));
        }
        if self.enum_prop(form, "window_state") == "minimized" {
            window.set_minimized(true);
        }
        #[cfg(not(web))]
        let access = accessible.then(|| access::Adapter::new(event_loop, &window, self.ui.poster()));
        // A que vai aparecer: com o primeiro quadro (no fim do `open_window`). No Windows ela já
        // fica mostrada, mas encoberta: a swap chain de uma janela escondida descarta o quadro.
        #[cfg(not(web))]
        let reveal = !hidden && !spanned && !cfg!(mobile);
        #[cfg(windows)]
        if reveal {
            win32::set_cloaked(&window, true);
            window.set_visible(true);
        }
        // Nas gravações de uso: qual renderizador cada janela usa (e a placa, no OpenGL e no
        // Direct2D).
        self.ui.set_platform_info("renderizador", backend.name());
        #[cfg(native_d2d)]
        if let Backend::D2d(d2d) = &backend
            && let Some(gpu) = d2d.adapter()
        {
            self.ui.set_platform_info("gpu", &gpu);
            self.ui.set_form_gpu(form, Some(gpu));
        }
        #[cfg(native_d2d)]
        aylis_core::gpu::set_adapter_list(aylis_d2d::adapters);
        #[cfg(native_gpu)]
        if let Some(gpu) = gpu::gl_description() {
            self.ui.set_platform_info("gpu", &gpu);
        }
        // A placa de cada janela no OpenGL (no Wayland, cada uma pode estar numa).
        #[cfg(native_gpu)]
        if let Backend::Gpu(gl) = &backend {
            self.ui.set_form_gpu(form, gl.adapter());
        }
        #[cfg(all(native_gpu, target_os = "linux"))]
        aylis_core::gpu::set_adapter_list(|| gpu_linux::devices().iter().map(gpu_linux::describe).collect());
        let name = self.ui.name(form).unwrap_or_default().to_string();
        self.ui.report(&aylis_core::activity::Activity::Custom { name: "janela_aberta", value: &format!("{name} ({})", backend.name()) });
        self.windows.push(FormWindow {
            #[cfg(not(web))]
            access,
            backend,
            window,
            form,
            pointer: PointF::ZERO,
            cursor: CursorShape::Default,
            enabled: true,
            edge: None,
            title: self.title(form),
            limits: [0.0; 4],
            #[cfg(windows)]
            trail: Default::default(),
            last_draw: f64::NEG_INFINITY,
            minimized: false,
            dark_title: self.ui.theme().dark,
            span: None,
            focused: false,
            hovered: false,
            uncloak_after_draw: false,
            clipped_monitor: false,
        });
        let index = self.windows.len() - 1;
        // As que esperavam por esta janela como dona (veja `reown_windows_of`).
        #[cfg(windows)]
        self.adopt_owned_windows(index);
        if spanned {
            // As outras janelas abrem já, e todas aparecem juntas.
            #[cfg(not(web))]
            let show = !hidden;
            #[cfg(web)]
            let show = true;
            self.open_span(event_loop, form, show);
        } else if matches!(self.ui.get_prop(form, "span_monitors"), Ok(PropValue::Bool(true))) {
            // O form que pode virar pedaços (o lugar dele só se sabe com a janela aberta):
            // confere nesta mesma volta do laço, depois de abrir os forms.
            self.span_checks.push(form);
        }
        self.sync_form_size(index);
        // A janela existe (com o alvo de desenho e, no Windows, o HWND em
        // `aylis::platform::win::hwnd`): o `on_show` vem antes dela, então quem precisa da
        // janela (a barra de título, a posição) usa este (o relatório do Tabernáculo, R03).
        self.ui.fire(form, "on_window_created", &aylis_core::EventArgs::None);
        // O primeiro quadro, e só então a janela aparece (sem o branco da abertura).
        #[cfg(not(web))]
        if reveal {
            if let Err(e) = self.redraw(index) {
                self.ui.report(&aylis_core::activity::Activity::Custom { name: "primeiro_quadro", value: &e.to_string() });
            }
            if let Some(w) = self.windows.get(index) {
                #[cfg(windows)]
                win32::set_cloaked(&w.window, false);
                #[cfg(not(windows))]
                w.window.set_visible(true);
            }
        }
        Ok(())
    }

    pub(crate) fn sync_windows(&mut self, event_loop: &ActiveEventLoop) {
        let open = self.ui.open_forms();
        if SINGLE_SURFACE {
            // Navegador numa tela larga: a tela fica com o principal e os outros viram janelas
            // por cima (`web_windows`).
            let surface = if self.floating_mode() { open.first() } else { open.last() };
            let Some(&top) = surface else { return };
            if self.windows.is_empty() {
                if let Err(e) = self.open_window(event_loop, top) {
                    self.fail(event_loop, e);
                }
            } else if self.windows[0].form != top {
                // Outro form no topo: a tela passa a mostrar ele.
                self.ui.pointer_cancel(self.windows[0].form);
                self.windows[0].form = top;
                self.windows[0].window.set_title(&self.title(top));
                self.sync_form_size(0);
                self.windows[0].window.request_redraw();
            }
        } else {
            // Fechar a janela é soltar o FormWindow.
            let closing: Vec<ComponentId> = self.windows.iter().map(|w| w.form).filter(|f| !open.contains(f)).collect();
            #[cfg(windows)]
            if !closing.is_empty() {
                self.reown_windows_of(&closing);
            }
            // Esconde antes de soltar: o FormWindow solta a superfície de desenho antes da
            // janela, e por um quadro a janela ficava na tela sem nada desenhado (o piscar
            // preto ou branco ao fechar, visto no editor de print do NoOBS).
            for w in self.windows.iter().filter(|w| closing.contains(&w.form)) {
                w.window.set_visible(false);
            }
            // Os pedaços extras antes da principal, que é a dona deles (o Windows destruiria
            // as extras junto com ela, por baixo do winit).
            for form in &closing {
                self.spans.remove(form);
                self.ui.set_form_pieces(*form, Vec::new());
                self.deactivations.retain(|f| f != form);
            }
            self.windows.retain(|w| !(w.is_extra() && closing.contains(&w.form)));
            self.windows.retain(|w| open.contains(&w.form));
            self.update_keep_awake();
            for form in closing {
                self.ui.set_form_gpu(form, None);
                let name = self.ui.name(form).unwrap_or_default().to_string();
                self.ui.report(&aylis_core::activity::Activity::Custom { name: "janela_fechada", value: &name });
            }
            // Os modais por último: a janela do dono já existe quando eles abrem.
            let mut open = open;
            open.sort_by_key(|&f| self.ui.is_modal(f));
            for form in open {
                if self.window_of(form).is_none() {
                    if let Err(e) = self.open_window(event_loop, form) {
                        self.fail(event_loop, e);
                        return;
                    }
                }
            }
            // O caption mudou depois de aberta: o título acompanha.
            let titles: Vec<String> = self.windows.iter().map(|w| self.title(w.form)).collect();
            for (w, title) in self.windows.iter_mut().zip(titles) {
                if w.title != title {
                    w.window.set_title(&title);
                    w.title = title;
                }
            }
            // O mínimo e o máximo do form (ou o zoom) mudaram: a janela acompanha. Só quando
            // mudam (o Windows reaplica o tamanho a cada chamada, e isso vira outro Resized).
            if !SINGLE_SURFACE {
                let limits: Vec<[f64; 4]> = self.windows.iter().map(|w| self.size_limits(w.form)).collect();
                // Os pedaços de um form em vários monitores têm o tamanho do monitor deles.
                for (w, limits) in self.windows.iter_mut().zip(limits).filter(|(w, _)| w.span.is_none()) {
                    if w.limits != limits {
                        let [min_w, min_h, max_w, max_h] = limits;
                        // Em pixels físicos (`physical_pixels`), os limites são pixels.
                        let physical = self.ui.physical_pixels(w.form);
                        let size = |w: f64, h: f64| -> winit::dpi::Size {
                            if physical { PhysicalSize::new(w, h).into() } else { LogicalSize::new(w, h).into() }
                        };
                        w.window.set_min_inner_size((min_w > 0.0 || min_h > 0.0).then(|| size(min_w, min_h)));
                        // Sem máximo num dos lados: grande o bastante para qualquer tela.
                        let open = |v: f64| if v > 0.0 { v } else { 100_000.0 };
                        w.window.set_max_inner_size((max_w > 0.0 || max_h > 0.0).then(|| size(open(max_w), open(max_h))));
                        w.limits = limits;
                    }
                }
            }
            // Form com `auto_size`: a janela fica da altura do conteúdo (L07 do Tabernáculo). Só
            // quando a diferença passa de meio ponto (cada pedido vira outro Resized).
            if !SINGLE_SURFACE {
                for w in self.windows.iter().filter(|w| w.span.is_none()) {
                    let Some(want) = self.ui.auto_form_height(w.form) else { continue };
                    let system = if self.ui.physical_pixels(w.form) { 1.0 } else { w.window.scale_factor() };
                    let zoom = system * self.ui.form_zoom(w.form) as f64;
                    let size = w.window.inner_size().to_logical::<f64>(zoom);
                    if (size.height - want as f64).abs() > 0.5 {
                        let wanted = LogicalSize::new(size.width * self.ui.form_zoom(w.form) as f64, want as f64 * self.ui.form_zoom(w.form) as f64);
                        let _ = w.window.request_inner_size(wanted.to_physical::<u32>(system));
                    }
                }
            }
            // Um diálogo do sistema com dona reabilita a dona ao fechar, mesmo debaixo de um modal
            // do Aylis: a conferência abaixo aplica de novo (o relatório do Tabernáculo, J02).
            #[cfg(windows)]
            let resync = win32::take_dialog_closed();
            #[cfg(not(windows))]
            let resync = false;
            for w in &mut self.windows {
                let enabled = !self.ui.input_blocked(w.form);
                if enabled != w.enabled || (resync && !enabled) {
                    w.enabled = enabled;
                    // Windows: a janela debaixo de um modal fica desabilitada (clicar nela
                    // traz o modal para a frente).
                    #[cfg(target_os = "windows")]
                    {
                        use winit::platform::windows::WindowExtWindows;
                        w.window.set_enable(enabled);
                    }
                }
            }
        }
        if let Some(form) = self.ui.take_activation() {
            if let Some(w) = self.window_of(form) {
                bring_to_front(&self.ui, form, &w.window);
            }
        }
    }

    /// Encerra o laço. O `Poll` antes do `exit()`: o winit chama o `about_to_wait` e logo depois
    /// espera por mensagens sem olhar o `exit()`; com o `Wait` da volta anterior, o processo
    /// dormiria até a próxima mensagem do sistema (o relatório do Tabernáculo, R02).
    pub(crate) fn quit(&mut self, event_loop: &ActiveEventLoop) {
        #[cfg(not(any(web, mobile)))]
        self.placements.flush();
        event_loop.set_control_flow(ControlFlow::Poll);
        event_loop.exit();
    }

    /// A cor com que a janela do form é apagada antes do quadro (a do fundo dele; a do tema se
    /// ele não pinta fundo); `None` na janela transparente.
    #[cfg(windows)]
    pub(crate) fn erase_color(&self, form: ComponentId) -> Option<aylis_core::Color> {
        if self.see_through(form) {
            return None;
        }
        // O `fill` do próprio form vale mais que o do estilo dele.
        let own = match self.ui.get_prop(form, "fill") {
            Ok(PropValue::Color(c)) if c.a == 255 => Some(c),
            _ => None,
        };
        let fill = own.unwrap_or_else(|| self.ui.background_color(form));
        Some(if fill.a == 255 { fill } else { self.ui.theme().color("background") })
    }

    /// A animação do sistema na janela do form (o `animation` do `WindowSettings`).
    #[cfg_attr(any(web, mobile), allow(dead_code))]
    pub(crate) fn window_animation(&self, form: ComponentId) -> bool {
        self.ui.window_options(form).animation
    }

    /// Aplica a todas as janelas abertas as opções do `WindowSettings` delas (um componente
    /// mudou em execução: o de um form ou o do app inteiro).
    #[cfg(not(any(web, mobile)))]
    pub(crate) fn update_window_options(&mut self) {
        for w in &self.windows {
            apply_window_options(&w.window, &self.ui.window_options(w.form));
            #[cfg(windows)]
            self.shape_window(&w.window, w.form);
        }
    }
}
