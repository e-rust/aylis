//! Os monitores e as placas de vídeo de cada janela: a lista, em que monitor cada form está, a troca de placa e o lugar da janela.

use super::*;

impl App {
    /// A área útil de um monitor (sem a barra de tarefas), em pixels físicos.
    pub(crate) fn work_rect(&self, handle: &winit::monitor::MonitorHandle) -> (PhysicalPosition<i32>, PhysicalSize<u32>) {
        match monitors::index_of(self.ui.monitors(), handle).map(|i| self.ui.monitors()[i].work_area()) {
            Some((x, y, w, h)) if w > 0 && h > 0 => (PhysicalPosition::new(x, y), PhysicalSize::new(w, h)),
            _ => (handle.position(), monitors::size(handle)),
        }
    }

    /// Os monitores mudaram? Pergunta ao sistema no máximo a cada 2 s (os modos de vídeo, só
    /// quando a lista muda) e diz a cada form em que monitor está a janela dele.
    pub(crate) fn refresh_monitors(&mut self, event_loop: &ActiveEventLoop, force: bool) {
        if !force && self.monitors_checked.is_some_and(|t| t.elapsed() < Duration::from_secs(2)) {
            // No Wayland o monitor da janela só se sabe pelo `wl_surface.enter`, que o compositor
            // manda depois do primeiro quadro: ao criar a janela, o `current_monitor` ainda é
            // `None`. Enquanto alguma não tem monitor, confere a cada volta (barato: o winit
            // guarda as saídas da superfície), em vez de esperar a varredura de 2 s.
            if self.windows.iter().any(|w| !w.is_extra() && self.ui.form_monitor(w.form).is_none()) {
                self.sync_form_monitors();
            }
            return;
        }
        self.monitors_checked = Some(Instant::now());
        let light = monitors::list(event_loop, false);
        let known: Vec<aylis_core::monitors::Monitor> = self.ui.monitors().iter().map(|m| aylis_core::monitors::Monitor { modes: Vec::new(), ..m.clone() }).collect();
        if light != known {
            self.ui.set_monitors(monitors::list(event_loop, true));
            self.monitors_changed();
        }
        self.sync_form_monitors();
    }

    /// Em que monitor está a janela de cada form.
    pub(crate) fn sync_form_monitors(&mut self) {
        let found: Vec<(ComponentId, Option<usize>)> = self
            .windows
            .iter()
            .filter(|w| !w.is_extra())
            .map(|w| (w.form, w.window.current_monitor().and_then(|m| monitors::index_of(self.ui.monitors(), &m))))
            .collect();
        for (form, index) in found {
            self.ui.set_form_monitor(form, index);
        }
    }

    /// O modo de apresentar do form: a propriedade `present` dele ou, vazia, o do projeto.
    #[cfg_attr(web, allow(dead_code))]
    pub(crate) fn present_mode(&self, form: ComponentId) -> aylis_core::gpu::PresentMode {
        if self.see_through(form) {
            return aylis_core::gpu::PresentMode::Transparent;
        }
        match self.ui.get_prop(form, "present") {
            Ok(PropValue::String(text) | PropValue::Enum(text)) => aylis_core::gpu::present_for(&text),
            _ => aylis_core::gpu::present(),
        }
    }

    /// A regra de placa de vídeo do form: a propriedade `gpu` dele ou, vazia, a do projeto.
    #[cfg_attr(web, allow(dead_code))]
    pub(crate) fn gpu_choice(&self, form: ComponentId) -> aylis_core::gpu::GpuChoice {
        match self.ui.get_prop(form, "gpu") {
            Ok(PropValue::String(text)) => aylis_core::gpu::choice_for(&text),
            _ => aylis_core::gpu::choice(),
        }
    }

    /// Com a placa que segue o monitor (Direct2D): se a janela `index` passou para um monitor
    /// de outra placa, o alvo é recriado nela e o form recebe o `on_gpu_change`. Sem a cópia de
    /// cada quadro entre as placas, a janela não trava no monitor da outra.
    ///
    /// No OpenGL quem avisa a troca é o compositor do Wayland (`App::follow_feedback`).
    ///
    /// Arrastando a janela (o laço de mover do Windows), espera soltar: o `WM_EXITSIZEMOVE`
    /// confere de novo (`win32::take_gpu_checks`), e a janela não troca de placa no meio do
    /// arraste nem várias vezes sobre a divisa. `force`: os monitores mudaram (confere mesmo
    /// no monitor de antes).
    pub(crate) fn follow_monitor(&mut self, index: usize, force: bool) {
        #[cfg(native_d2d)]
        {
            let Some(w) = self.windows.get_mut(index) else { return };
            if !force && win32::in_size_move(w.form) {
                return;
            }
            let changed = match &mut w.backend {
                Backend::D2d(d2d) => d2d.follow_monitor(&self.renderer.d2d, force),
                #[allow(unreachable_patterns)]
                _ => return,
            };
            self.after_gpu_change(index, changed);
        }
        #[cfg(not(native_d2d))]
        let _ = (index, force);
    }

    /// Linux (Wayland), `gpu = monitor`: o compositor diz a placa preferida de cada janela
    /// (o feedback do `linux-dmabuf`); a que mudou de placa troca. Barato quando nada mudou.
    #[cfg(all(native_gpu, target_os = "linux"))]
    pub(crate) fn follow_feedback(&mut self) {
        for index in 0..self.windows.len() {
            let w = &mut self.windows[index];
            let Backend::Gpu(gl) = &mut w.backend else { continue };
            let changed = gl.follow_monitor(&w.window);
            if !matches!(changed, Ok(false)) {
                self.after_gpu_change(index, changed);
            }
        }
    }

    /// A propriedade `gpu` (ou a `present`) do form mudou com o app rodando.
    pub(crate) fn update_gpu(&mut self, form: ComponentId) {
        #[cfg(native_d2d)]
        {
            let present = self.present_mode(form);
            for index in 0..self.windows.len() {
                if self.windows[index].form != form {
                    continue;
                }
                let changed = match &mut self.windows[index].backend {
                    Backend::D2d(d2d) => d2d.set_present(&self.renderer.d2d, present),
                    #[allow(unreachable_patterns)]
                    _ => continue,
                };
                self.after_gpu_change(index, changed);
            }
        }
        #[cfg(any(native_d2d, native_gpu))]
        {
            let choice = self.gpu_choice(form);
            let Some(index) = self.windows.iter().position(|w| w.form == form) else { return };
            let w = &mut self.windows[index];
            let changed = match &mut w.backend {
                #[cfg(native_d2d)]
                Backend::D2d(d2d) => d2d.set_choice(&self.renderer.d2d, choice),
                #[cfg(native_gpu)]
                Backend::Gpu(gl) => gl.set_choice(&w.window, choice),
                #[allow(unreachable_patterns)]
                _ => return,
            };
            self.after_gpu_change(index, changed);
        }
        #[cfg(not(any(native_d2d, native_gpu)))]
        let _ = form;
    }

    /// Depois de conferir a placa da janela `index`: trocou, então o quadro sai inteiro na
    /// placa nova e o form fica sabendo.
    #[cfg(any(native_d2d, native_gpu))]
    pub(crate) fn after_gpu_change(&mut self, index: usize, changed: Result<bool, Error>) {
        match changed {
            Ok(true) => {
                let w = &self.windows[index];
                let (form, adapter) = (w.form, match &w.backend {
                    #[cfg(native_d2d)]
                    Backend::D2d(d2d) => d2d.adapter(),
                    #[cfg(native_gpu)]
                    Backend::Gpu(gl) => gl.adapter(),
                    #[allow(unreachable_patterns)]
                    _ => None,
                });
                w.window.request_redraw();
                // Os pedaços extras de um form em vários monitores podem estar em outra placa:
                // o form fica com a da janela principal.
                if w.is_extra() {
                    return;
                }
                if let Some(gpu) = &adapter {
                    self.ui.set_platform_info("gpu", gpu);
                }
                self.ui.set_form_gpu(form, adapter);
            }
            Ok(false) => {}
            Err(e) => aylis_core::stderr_line!("aylis: não deu para trocar de placa ({e}); a janela fica na de antes"),
        }
    }

    /// Ajusta o form ao tamanho da janela, em unidades lógicas.
    /// Guarda onde a janela está ([`aylis_core::WindowPlacement`]); maximizada ou em tela
    /// cheia, só marca (a posição e o tamanho de antes ficam, para voltar a eles). Antes, a tela
    /// cheia gravava o retângulo dela como o lugar normal, e o `remember_placement` reabria a
    /// janela comum do tamanho do monitor (testado em 20/11).
    pub(crate) fn store_placement(&mut self, index: usize) {
        let Some(w) = self.windows.get(index) else { return };
        // Num form em vários monitores, o lugar é o dos pedaços (`span.rs`), não o da janela.
        if SINGLE_SURFACE || w.span.is_some() || w.window.is_minimized() == Some(true) {
            return;
        }
        let form = w.form;
        let mut p = self.ui.window_placement(form).unwrap_or_default();
        p.maximized = w.window.is_maximized();
        p.full_screen = w.window.fullscreen().is_some();
        if p.full_screen {
            // A posição de antes fica (se ainda não há, o canto do monitor: reabre nele).
            let monitor = w.window.current_monitor();
            let at = monitor.as_ref().map(|m| m.position());
            if let Some(at) = at.filter(|_| p.width == 0 || !monitor.as_ref().is_some_and(|m| contains(m, p.x, p.y))) {
                (p.x, p.y) = (at.x, at.y);
            }
        } else if !p.maximized {
            if let Ok(at) = w.window.outer_position() {
                (p.x, p.y) = (at.x, at.y);
            }
            let size = w.window.inner_size();
            if size.width > 0 && size.height > 0 {
                (p.width, p.height) = (size.width, size.height);
            }
        }
        self.ui.set_window_placement(form, p);
        self.recorded_placements.insert(form, p);
        // O monitor dela agora e, com `remember_placement`, a posição para reabrir igual.
        let monitor = w.window.current_monitor();
        self.ui.set_form_monitor(form, monitor.as_ref().and_then(|m| monitors::index_of(self.ui.monitors(), m)));
        #[cfg(not(any(web, mobile)))]
        if let Some(m) = monitor.filter(|_| matches!(self.ui.get_prop(form, "remember_placement"), Ok(PropValue::Bool(true)))) {
            let name = self.ui.name(form).unwrap_or_default().to_string();
            // A identidade do aparelho, quando o sistema dá (sobrevive à troca de porta); senão, o
            // nome do sistema.
            let known = monitors::index_of(self.ui.monitors(), &m).map(|i| self.ui.monitors()[i].device_path.clone()).filter(|p| !p.is_empty());
            let monitor = known.unwrap_or_else(|| monitors::device(&m));
            self.placements.set(&name, aylis_core::monitors::SavedPlacement { monitor, placement: p });
        }
    }

    /// A escala do sistema na janela `index` (pixels por ponto, sem o zoom): a do monitor ou,
    /// num form em pixels físicos (`physical_pixels`), 1.
    pub(crate) fn system_scale(&self, index: usize) -> f64 {
        let w = &self.windows[index];
        if self.ui.physical_pixels(w.form) { 1.0 } else { draw_scale(&w.window) }
    }

    pub(crate) fn sync_form_size(&mut self, index: usize) {
        let Some(w) = self.windows.get(index) else { return };
        #[cfg(native_gpu)]
        #[allow(irrefutable_let_patterns)] // quando só a feature `gpu` está ligada
        if let Backend::Gpu(gpu) = &w.backend {
            gpu.resize(&w.window);
        }
        let form = w.form;
        // Num form em vários monitores, o tamanho é o do form inteiro (todos os pedaços).
        let logical = match self.span_size(form).filter(|_| w.span.is_some()) {
            Some((width, height)) => LogicalSize::new(width, height),
            None => surface_size(&w.window).to_logical::<f64>(self.system_scale(index) * self.ui.form_zoom(form) as f64),
        };
        if let Some(cb) = self.ui.control_mut(form) {
            cb.set_size(logical.width as f32, logical.height as f32);
        }
        self.update_safe_area(index);
    }
}
