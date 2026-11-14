//! Um form em vários monitores (`span_monitors` do `Form`; veja `aylis_core::span` e
//! `docs/gpu-por-monitor.md`): com o form maior que um monitor, uma janela por monitor, cada
//! uma na placa de vídeo do seu, todas com a principal como dona (a barra de tarefas e o
//! Alt+Tab mostram uma só). O `Ui` continua com um form só: cada janela pinta o seu pedaço
//! com o deslocamento dele e a entrada de cada uma vira um ponto do form.
//!
//! Por enquanto só no Windows (o Direct2D e o Skia); nas outras plataformas a propriedade não
//! tem efeito e o form fica numa janela só.

use std::num::NonZeroU32;

use aylis_core::span::{self, Piece, ScreenRect};
use aylis_core::{ComponentId, PointF, PropValue, RectF, WindowCommand};
use web_time::Instant;
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event_loop::ActiveEventLoop;
use winit::window::Window;

use crate::{App, Backend, Error, FormWindow, SINGLE_SURFACE};

/// Onde vale: o Windows (as janelas com dona, a placa de cada janela). No Linux e no macOS
/// falta conferir as janelas sem moldura em cada monitor e a dona (veja os pendentes do doc).
const SUPPORTED: bool = cfg!(windows) && !SINGLE_SURFACE;

/// Um form em vários monitores.
pub(crate) struct SpanForm {
    /// Onde o form está na tela agora (pixels físicos): o `normal` ou, em tela cheia, todos
    /// os monitores.
    rect: ScreenRect,
    /// Onde ele fica fora da tela cheia.
    normal: ScreenRect,
    /// Pixels por unidade do form (a escala do monitor da janela principal ao virar pedaços;
    /// o zoom vem por cima). Todos os pedaços desenham com ela, para o form não mudar de
    /// tamanho no meio (num monitor com outra escala, ele aparece com a da principal).
    scale: f64,
}

/// O pedaço que uma janela mostra.
#[derive(Clone, Copy, Debug)]
pub(crate) struct SpanPiece {
    pub(crate) piece: Piece,
    /// A janela principal do form (a que existia antes; as outras são extras).
    pub(crate) main: bool,
}

impl App {
    /// A janela `index` é um pedaço extra de um form em vários monitores (não a principal).
    pub(crate) fn is_extra(&self, index: usize) -> bool {
        self.windows.get(index).is_some_and(FormWindow::is_extra)
    }

    /// Pixels por unidade do form na janela `index` (com o zoom): nos pedaços, a do form.
    pub(crate) fn input_scale(&self, index: usize) -> f64 {
        let w = &self.windows[index];
        let base = match self.spans.get(&w.form).filter(|_| w.span.is_some()) {
            Some(s) => s.scale,
            None => w.window.scale_factor(),
        };
        base * self.ui.form_zoom(w.form) as f64
    }

    /// Um ponto da janela `index` (pixels físicos a partir do canto dela) no form.
    pub(crate) fn window_to_form(&self, index: usize, x: f64, y: f64, scale: f64) -> PointF {
        match self.windows[index].span {
            Some(s) => s.piece.to_form(x, y, scale),
            None => PointF::new((x / scale) as f32, (y / scale) as f32),
        }
    }

    /// O tamanho do form em vários monitores, em unidades lógicas (com o zoom).
    pub(crate) fn span_size(&self, form: ComponentId) -> Option<(f64, f64)> {
        let s = self.spans.get(&form)?;
        let scale = s.scale * self.ui.form_zoom(form) as f64;
        Some((s.rect.width as f64 / scale, s.rect.height as f64 / scale))
    }

    fn wants_span(&self, form: ComponentId) -> bool {
        SUPPORTED && matches!(self.ui.get_prop(form, "span_monitors"), Ok(PropValue::Bool(true)))
    }

    /// O form abre em tela cheia em vários monitores: a janela não entra em tela cheia num
    /// monitor só antes de virar os pedaços.
    pub(crate) fn spans_full_screen(&self, form: ComponentId) -> bool {
        self.wants_span(form) && self.monitor_rects().len() >= 2 && self.enum_prop(form, "window_state") == "full_screen"
    }

    /// Os monitores, para dividir o form. Diagnóstico (numa máquina com um monitor só):
    /// `AYLIS_SPAN_SPLIT=N` conta cada monitor como N, lado a lado (N colunas), e o form que
    /// passa da divisa vira as janelas por "monitor" como com monitores de verdade (todas na
    /// mesma placa).
    fn monitor_rects(&self) -> Vec<ScreenRect> {
        let rects = self.ui.monitors().iter().map(|m| ScreenRect::new(m.x, m.y, m.width, m.height));
        match crate::diag::diagnostic_setting("AYLIS_SPAN_SPLIT").and_then(|v| v.trim().parse::<u32>().ok()).filter(|n| (2..=8).contains(n)) {
            Some(n) => rects
                .flat_map(|r| {
                    let w = r.width / n;
                    (0..n).map(move |i| ScreenRect::new(r.x + (i * w) as i32, r.y, if i + 1 == n { r.width - w * (n - 1) } else { w }, r.height))
                })
                .collect(),
            None => rects.collect(),
        }
    }

    /// Os pedidos à janela de um form em vários monitores (ou que pode virar): `true` se já
    /// foram tratados aqui. Mudar o lugar, o tamanho ou a tela cheia refaz os pedaços.
    pub(crate) fn span_command(&mut self, form: ComponentId, command: WindowCommand) -> bool {
        if command == WindowCommand::UpdateSpan {
            self.span_checks.push(form);
            return true;
        }
        if let WindowCommand::SetFullscreen(_) = command
            && (self.spans.contains_key(&form) || self.spans_full_screen(form))
        {
            self.span_checks.push(form);
            return true;
        }
        let zoom = self.ui.form_zoom(form) as f64;
        let size = self.ui.control(form).map(|cb| (cb.width() as f64, cb.height() as f64));
        if !self.spans.contains_key(&form) {
            // Ainda numa janela: ela muda como sempre e, se passou a cruzar monitores, vira os
            // pedaços na conferência desta mesma volta do laço.
            if matches!(command, WindowCommand::ApplySize | WindowCommand::MoveTo(..) | WindowCommand::MoveToMonitor(_)) && self.wants_span(form) {
                self.span_checks.push(form);
            }
            return false;
        }
        let Some(s) = self.spans.get_mut(&form) else { return false };
        match command {
            WindowCommand::ApplySize => {
                if let Some((w, h)) = size {
                    let scale = s.scale * zoom;
                    s.normal.width = (w * scale).round().max(1.0) as u32;
                    s.normal.height = (h * scale).round().max(1.0) as u32;
                }
            }
            WindowCommand::MoveTo(x, y) => (s.normal.x, s.normal.y) = (x, y),
            WindowCommand::MoveToMonitor(i) => {
                let Some(m) = self.ui.monitors().get(i) else { return true };
                let (x, y, w, h) = m.work_area();
                s.normal.x = x + (w as i32 - s.normal.width as i32) / 2;
                s.normal.y = y + (h as i32 - s.normal.height as i32) / 2;
            }
            // Mostrar, esconder e ficar por cima valem para os pedaços também; a principal
            // segue o caminho de sempre.
            WindowCommand::Hide | WindowCommand::Show | WindowCommand::SetTopmost(_) => {
                for w in self.windows.iter().filter(|w| w.form == form && w.is_extra()) {
                    match command {
                        WindowCommand::Hide => w.window.set_visible(false),
                        WindowCommand::Show => w.window.set_visible(true),
                        WindowCommand::SetTopmost(on) => w.window.set_window_level(if on { winit::window::WindowLevel::AlwaysOnTop } else { winit::window::WindowLevel::Normal }),
                        _ => {}
                    }
                }
                return false;
            }
            // Minimizar a principal leva as extras junto (são dela).
            WindowCommand::Minimize | WindowCommand::Restore | WindowCommand::Close | WindowCommand::UpdateGpu => return false,
            // Maximizar, arrastar e redimensionar pela borda não valem para um form em vários
            // monitores (ele já ocupa o lugar dele em cada um).
            _ => return true,
        }
        self.span_checks.push(form);
        true
    }

    /// Os monitores mudaram (ligado, desligado, outra resolução): os forms em vários monitores
    /// refazem os pedaços.
    pub(crate) fn monitors_changed(&mut self) {
        let forms: Vec<ComponentId> = self.windows.iter().filter(|w| !w.is_extra()).map(|w| w.form).filter(|&f| self.wants_span(f) || self.spans.contains_key(&f)).collect();
        self.span_checks.extend(forms);
    }

    /// Confere os forms marcados: viram janelas por monitor, refazem os pedaços ou voltam a
    /// uma janela.
    pub(crate) fn check_spans(&mut self, event_loop: &ActiveEventLoop) {
        let mut forms = std::mem::take(&mut self.span_checks);
        let mut seen = std::collections::HashSet::new();
        forms.retain(|f| seen.insert(*f));
        for form in forms {
            if let Err(e) = self.respan(event_loop, form) {
                aylis_core::stderr_line!("aylis: não deu para abrir o form em vários monitores ({e}); ele fica numa janela");
                self.unspan(form);
            }
        }
    }

    fn main_index(&self, form: ComponentId) -> Option<usize> {
        self.windows.iter().position(|w| w.form == form && !w.is_extra())
    }

    fn respan(&mut self, event_loop: &ActiveEventLoop, form: ComponentId) -> Result<(), Error> {
        let Some(main) = self.main_index(form) else { return Ok(()) };
        if !self.wants_span(form) {
            self.unspan(form);
            return Ok(());
        }
        let monitors = self.monitor_rects();
        let full = self.enum_prop(form, "window_state") == "full_screen";
        // O retângulo do form: o guardado ou, na primeira vez, o da janela (a área cliente,
        // do tamanho do form).
        let (normal, scale) = match self.spans.get(&form) {
            Some(s) => (s.normal, s.scale),
            None => {
                let w = &self.windows[main].window;
                let at = w.inner_position().unwrap_or_default();
                let scale = w.scale_factor();
                let zoom = self.ui.form_zoom(form) as f64;
                let (width, height) = self.ui.control(form).map_or((640.0, 480.0), |cb| (cb.width() as f64, cb.height() as f64));
                let size = PhysicalSize::new((width * scale * zoom).round().max(1.0) as u32, (height * scale * zoom).round().max(1.0) as u32);
                (ScreenRect::new(at.x, at.y, size.width, size.height), scale)
            }
        };
        let rect = if full { ScreenRect::bounding(&monitors).unwrap_or(normal) } else { normal };
        let pieces = span::split(rect, &monitors);
        if pieces.len() < 2 {
            // Num monitor só: uma janela, como sem a propriedade.
            if self.spans.contains_key(&form) {
                self.unspan(form);
            } else if full && self.windows[main].window.fullscreen().is_none() {
                self.windows[main].window.set_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));
            }
            return Ok(());
        }
        let first = !self.spans.contains_key(&form);
        self.spans.insert(form, SpanForm { rect, normal, scale });
        let chosen = span::largest(&pieces).unwrap_or(0);
        {
            let w = &mut self.windows[main];
            if first {
                w.window.set_fullscreen(None);
                w.window.set_maximized(false);
                w.window.set_decorations(false);
                w.window.set_resizable(false);
            }
            place(&w.window, pieces[chosen].screen);
            w.span = Some(SpanPiece { piece: pieces[chosen], main: true });
            w.window.request_redraw();
        }
        // As extras: as que já existem ficam com os pedaços novos, as que faltam abrem e as
        // que sobram fecham.
        let extras: Vec<usize> = (0..self.windows.len()).filter(|&i| self.windows[i].form == form && self.windows[i].is_extra()).collect();
        let others: Vec<Piece> = pieces.iter().enumerate().filter(|(i, _)| *i != chosen).map(|(_, p)| *p).collect();
        for (k, piece) in others.iter().enumerate() {
            match extras.get(k) {
                Some(&i) => {
                    let w = &mut self.windows[i];
                    place(&w.window, piece.screen);
                    w.span = Some(SpanPiece { piece: *piece, main: false });
                    w.window.request_redraw();
                }
                None => self.open_piece(event_loop, form, main, *piece)?,
            }
        }
        for &i in extras.iter().skip(others.len()).rev() {
            self.windows.remove(i);
        }
        let zoom = self.ui.form_zoom(form) as f64;
        self.ui.set_form_pieces(form, pieces.iter().map(|p| p.rect_in_form(scale * zoom)).collect());
        if let Some(main) = self.main_index(form) {
            if first {
                self.follow_pieces(main, true);
            }
            self.sync_form_size(main);
        }
        Ok(())
    }

    /// Volta a uma janela: as extras fecham, a principal fica com a moldura e o lugar do form.
    pub(crate) fn unspan(&mut self, form: ComponentId) {
        self.windows.retain(|w| !(w.form == form && w.is_extra()));
        self.ui.set_form_pieces(form, Vec::new());
        let Some(s) = self.spans.remove(&form) else { return };
        let Some(main) = self.main_index(form) else { return };
        let border = self.enum_prop(form, "border_style");
        let full = self.enum_prop(form, "window_state") == "full_screen";
        {
            let w = &mut self.windows[main];
            w.span = None;
            w.window.set_decorations(!matches!(border.as_str(), "none" | "custom"));
            w.window.set_resizable(border != "single");
            place(&w.window, s.normal);
            if full {
                w.window.set_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));
            }
            w.window.request_redraw();
        }
        self.follow_pieces(main, false);
        self.sync_form_size(main);
    }

    /// Com o form em pedaços, a janela principal segue a placa do monitor dela (as extras já
    /// nascem assim), a não ser que o form peça uma placa; ao voltar, a regra do form.
    fn follow_pieces(&mut self, index: usize, on: bool) {
        #[cfg(any(native_d2d, native_gpu))]
        {
            let form = self.windows[index].form;
            let choice = self.piece_gpu(form, on);
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
        let _ = (index, on);
    }

    /// A placa de um pedaço: a do monitor dele, a menos que o form peça outra.
    #[cfg_attr(web, allow(dead_code))]
    fn piece_gpu(&self, form: ComponentId, span: bool) -> aylis_core::gpu::GpuChoice {
        use aylis_core::gpu::GpuChoice;
        match self.gpu_choice(form) {
            GpuChoice::Auto if span => GpuChoice::FollowMonitor,
            other => other,
        }
    }

    /// Abre a janela de um pedaço extra: sem moldura, fora da barra de tarefas, com a
    /// principal como dona e na placa do monitor dela.
    fn open_piece(&mut self, event_loop: &ActiveEventLoop, form: ComponentId, main: usize, piece: Piece) -> Result<(), Error> {
        let dark = self.ui.theme().dark;
        let mut attributes = Window::default_attributes()
            .with_title(self.title(form))
            .with_decorations(false)
            .with_resizable(false)
            .with_active(false)
            // Como a principal: escondida (o form aberto escondido), as extras também.
            .with_visible(self.windows[main].window.is_visible().unwrap_or(true))
            .with_position(PhysicalPosition::new(piece.screen.x, piece.screen.y))
            .with_inner_size(PhysicalSize::new(piece.screen.width, piece.screen.height))
            .with_theme(Some(if dark { winit::window::Theme::Dark } else { winit::window::Theme::Light }));
        if matches!(self.ui.get_prop(form, "topmost"), Ok(PropValue::Bool(true))) {
            attributes = attributes.with_window_level(winit::window::WindowLevel::AlwaysOnTop);
        }
        #[cfg(not(any(web, mobile)))]
        {
            attributes = attributes.with_window_icon(self.icon.clone());
        }
        #[cfg(windows)]
        {
            use winit::platform::windows::WindowAttributesExtWindows;
            use winit::raw_window_handle::{HasWindowHandle, RawWindowHandle};
            attributes = attributes.with_skip_taskbar(true);
            if let Ok(RawWindowHandle::Win32(h)) = self.windows[main].window.window_handle().map(|h| h.as_raw()) {
                attributes = attributes.with_owner_window(h.hwnd.get());
            }
        }
        #[cfg(not(web))]
        let (window, backend) = crate::create_backend(event_loop, attributes, self.options.renderer, &self.renderer, self.piece_gpu(form, true))?;
        #[cfg(web)]
        let (window, backend) = crate::create_backend(event_loop, attributes, self.options.renderer)?;
        window.request_redraw();
        self.windows.push(FormWindow {
            #[cfg(not(web))]
            access: None,
            backend,
            window,
            form,
            pointer: PointF::ZERO,
            cursor: aylis_core::CursorShape::Default,
            enabled: true,
            edge: None,
            title: self.title(form),
            limits: [0.0; 4],
            #[cfg(windows)]
            trail: Default::default(),
            last_draw: f64::NEG_INFINITY,
            minimized: false,
            dark_title: dark,
            span: Some(SpanPiece { piece, main: false }),
            focused: false,
            hovered: false,
        });
        Ok(())
    }

    /// Os pedaços que perderam o foco: se nenhum pedaço do form ganhou (o foco não só passou
    /// de um monitor para o outro), o form perde.
    pub(crate) fn settle_deactivations(&mut self) {
        for form in std::mem::take(&mut self.deactivations) {
            if !self.windows.iter().any(|w| w.form == form && w.focused) {
                self.ui.pointer_cancel(form);
                self.ui.fire(form, "on_deactivate", &aylis_core::EventArgs::None);
            }
        }
    }

    /// Pinta um form em vários monitores. Com algo mudado no form, todos os pedaços pintam o
    /// que mudou neles (o `take_frame` é um por quadro, para o form inteiro); sem nada mudado
    /// (o sistema pediu: a janela apareceu, mudou de tamanho ou de placa), só esta janela,
    /// inteira.
    pub(crate) fn redraw_span(&mut self, index: usize) -> Result<(), Error> {
        let form = self.windows[index].form;
        let fresh = self.ui.forms_to_paint(&[form]).contains(&form);
        let now = self.started.elapsed().as_secs_f64();
        self.ui.set_time(now);
        self.ui.tick();
        self.ui.update_layout();
        self.ui.run_timers();
        self.ui.update_layout();
        let background = self.ui.theme().color("background");
        let capture = self.snapshot_dir.is_some();
        let scale = (self.spans.get(&form).map_or(1.0, |s| s.scale) * self.ui.form_zoom(form) as f64) as f32;
        let bounds = self.ui.control(form).map(|cb| cb.bounds());
        // O que mudou, nas coordenadas do form (as cópias da rolagem viram área: a origem pode
        // estar em outra janela). `None`: tudo.
        let areas: Option<Vec<RectF>> = match fresh.then(|| self.ui.take_frame(form)) {
            Some(aylis_core::FrameDamage::Parts { moves, areas }) if !capture => {
                let damage = aylis_core::FrameDamage::Parts { moves, areas }.without_moves();
                match damage {
                    aylis_core::FrameDamage::Parts { areas, .. } => Some(areas.into_iter().filter_map(|r| crate::pixel_area(r, scale, bounds)).collect()),
                    aylis_core::FrameDamage::Whole => None,
                }
            }
            _ => None,
        };
        let targets: Vec<usize> = if fresh { (0..self.windows.len()).filter(|&i| self.windows[i].form == form && self.windows[i].span.is_some()).collect() } else { vec![index] };
        let started = Instant::now();
        let mut start = fresh;
        let name = self.ui.name(form).filter(|n| !n.is_empty()).unwrap_or("form").to_string();
        for (k, i) in targets.into_iter().enumerate() {
            let w = &mut self.windows[i];
            let Some(piece) = w.span else { continue };
            let size = crate::surface_size(&w.window);
            let (Some(width), Some(height)) = (NonZeroU32::new(size.width), NonZeroU32::new(size.height)) else { continue };
            let s = scale.max(0.01);
            let rect = RectF::new(piece.piece.origin_x as f32 / s, piece.piece.origin_y as f32 / s, width.get() as f32 / s, height.get() as f32 / s);
            let local: Option<Vec<RectF>> = areas.as_ref().map(|a| a.iter().filter_map(|r| r.intersect(rect)).map(|r| r.offset(-rect.x, -rect.y)).collect());
            if local.as_ref().is_some_and(Vec::is_empty) {
                continue; // nada mudou neste pedaço
            }
            let parts = local.as_deref().map(|a| (&[][..], a));
            let first = std::mem::replace(&mut start, false);
            w.last_draw = now;
            let ui = &mut self.ui;
            let frame: Option<Vec<u8>> = match &mut w.backend {
                #[cfg(native_cpu)]
                Backend::Cpu(cpu) => cpu.draw(&self.renderer.skia, (width, height), scale, background, parts, |c, area| ui.paint_piece(form, c, rect, area, first), capture)?,
                #[cfg(native_gpu)]
                Backend::Gpu(gpu) => {
                    let parts = parts.filter(|_| gpu.keeps_image((width, height)));
                    gpu.draw(&self.renderer.skia, (width, height), scale, background, parts, |c, area| ui.paint_piece(form, c, rect, area, first), capture)?
                }
                #[cfg(native_d2d)]
                Backend::D2d(d2d) => d2d.draw(&self.renderer.d2d, (width, height), scale, background, parts, |c, area| ui.paint_piece(form, c, rect, area, first), capture)?,
                #[cfg(web)]
                Backend::Web(web) => {
                    web.draw((width, height), scale, background, parts, |c, area| ui.paint_piece(form, c, rect, area, first))?;
                    None
                }
            };
            if let (Some(dir), Some(rgba)) = (&self.snapshot_dir, frame) {
                crate::save_snapshot(dir, &format!("{name}.{k}"), &rgba, (width, height));
            }
        }
        let elapsed = started.elapsed().as_secs_f64();
        crate::PAINT_SECS.with(|p| p.set(elapsed));
        self.ui.frame_cost_measured(elapsed);
        self.ui.frame_drawn();
        Ok(())
    }
}

impl FormWindow {
    /// Um pedaço extra de um form em vários monitores (não a janela principal).
    pub(crate) fn is_extra(&self) -> bool {
        self.span.is_some_and(|s| !s.main)
    }
}

/// Põe a janela (sem moldura) no retângulo da tela.
fn place(window: &Window, rect: ScreenRect) {
    window.set_outer_position(PhysicalPosition::new(rect.x, rect.y));
    let _ = window.request_inner_size(PhysicalSize::new(rect.width, rect.height));
}
