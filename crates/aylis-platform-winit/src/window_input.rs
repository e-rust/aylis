//! A entrada de cada janela (mouse, teclado, roda, toque) para o form, e o arrasto entre janelas do app.

use super::*;

impl App {
    /// Trata um evento de entrada da janela `index`. Devolve `false` se não for de entrada.
    pub(crate) fn handle_input(&mut self, index: usize, event: &WindowEvent) -> bool {
        let form = self.windows[index].form;
        // Nos pedaços de um form em vários monitores, a escala do form (`span.rs`).
        let scale = self.input_scale(index);
        self.ui.set_time(self.started.elapsed().as_secs_f64());
        // Navegador com janelas: o ponteiro vai ao form sob ele; o teclado, ao de cima.
        #[cfg(web)]
        if self.floating_mode() {
            if self.floating_input(index, event, scale) {
                // O cursor também segue o ponteiro aqui (sem isso ficava no último valor,
                // a mãozinha depois de um clique).
                self.after_input();
                return true;
            }
            if let Some(&top) = self.ui.open_forms().last() {
                return self.handle_input_for(index, event, top, scale);
            }
        }
        self.handle_input_for(index, event, form, scale)
    }

    /// Durante um arrasto, a janela do app que está por cima no ponto (pela ordem real das
    /// janelas no Windows), quando é a de outro form: o destino está nela, mesmo com o mouse
    /// ainda dentro da janela onde o arrasto começou (uma janela pequena por cima de uma
    /// grande). `None`: a de cima é a do próprio form, de outro programa, ou não dá para saber
    /// (fora do Windows).
    #[cfg(not(any(web, mobile)))]
    pub(crate) fn drag_cover(&self, index: usize, position: winit::dpi::PhysicalPosition<f64>) -> Option<usize> {
        #[cfg(windows)]
        {
            let w = &self.windows[index];
            let origin = w.window.inner_position().ok()?;
            let top = crate::win32::top_level_at((origin.x as f64 + position.x) as i32, (origin.y as f64 + position.y) as i32)?;
            let i = self.windows.iter().position(|o| crate::win32::hwnd_of(&o.window).is_some_and(|h| h.0 as isize == top))?;
            (self.windows[i].form != w.form && self.windows[i].window.is_minimized() != Some(true)).then_some(i)
        }
        #[cfg(not(windows))]
        {
            let _ = (index, position);
            None
        }
    }

    /// Durante um arrasto que começou em `form`, o mouse fora da janela dele (ou dentro, mas
    /// com a janela de outro form por cima: `cover`): acha a outra janela do app sob ele (pela
    /// posição na tela) e passa ao núcleo o form e o ponto nele.
    #[cfg(not(any(web, mobile)))]
    pub(crate) fn drag_between_windows(&mut self, index: usize, form: ComponentId, position: winit::dpi::PhysicalPosition<f64>, cover: Option<usize>) {
        let w = &self.windows[index];
        let size = w.window.inner_size();
        let inside = match w.span {
            // Um form em vários monitores: dentro de qualquer pedaço dele é dentro (o
            // `pointer_move` já levou o ponto ao form).
            Some(_) => {
                let p = self.window_to_form(index, position.x, position.y, self.input_scale(index));
                self.ui.control(form).is_some_and(|cb| p.x >= 0.0 && p.y >= 0.0 && p.x < cb.width() && p.y < cb.height())
            }
            None => position.x >= 0.0 && position.y >= 0.0 && position.x < size.width as f64 && position.y < size.height as f64,
        };
        let Ok(origin) = w.window.inner_position() else { return };
        let screen = (origin.x as f64 + position.x, origin.y as f64 + position.y);
        // Quem solta fora das janelas (a aba que vira janela) precisa saber onde.
        self.ui.set_screen_pointer(Some(aylis_core::ScreenPointer { x: screen.0 as i32, y: screen.1 as i32, scale: w.window.scale_factor() as f32 }));
        let mut over = cover.and_then(|i| {
            let at = self.windows[i].window.inner_position().ok()?;
            let local = (screen.0 - at.x as f64, screen.1 - at.y as f64);
            Some((self.windows[i].form, self.window_to_form(i, local.0, local.1, self.input_scale(i))))
        });
        if !inside && over.is_none() {
            for (i, other) in self.windows.iter().enumerate() {
                if i == index || other.form == form || other.window.is_minimized() == Some(true) {
                    continue;
                }
                let (Ok(at), size) = (other.window.inner_position(), other.window.inner_size()) else { continue };
                let local = (screen.0 - at.x as f64, screen.1 - at.y as f64);
                if local.0 >= 0.0 && local.1 >= 0.0 && local.0 < size.width as f64 && local.1 < size.height as f64 {
                    over = Some((other.form, self.window_to_form(i, local.0, local.1, self.input_scale(i))));
                    break;
                }
            }
        }
        match over {
            Some((other, at)) => self.ui.drag_over_form(form, Some(other), at),
            // Dentro da própria janela o `pointer_move` já cuidou; fora de todas, sem destino.
            None if !inside => self.ui.drag_over_form(form, None, PointF::default()),
            None => {}
        }
    }

    /// O resto da entrada, para o form `form`.
    pub(crate) fn handle_input_for(&mut self, index: usize, event: &WindowEvent, form: ComponentId, scale: f64) -> bool {
        let modifiers = self.modifiers;
        let pointer_event = |position: PointF, button, kind| PointerEvent { position, button, kind, modifiers, ..Default::default() };
        match event {
            WindowEvent::ModifiersChanged(m) => self.modifiers = input::modifiers(m.state()),
            WindowEvent::CursorMoved { position, .. } => {
                // O ponto no form (num pedaço de um form em vários monitores, mais a origem dele).
                let p = self.window_to_form(index, position.x, position.y, scale);
                self.windows[index].pointer = p;
                self.windows[index].hovered = true;
                // Janela sem a barra do sistema (`border_style = custom`): as bordas
                // redimensionam.
                let edge = self.resize_edge(index, p);
                if edge != self.windows[index].edge {
                    self.windows[index].edge = edge;
                    let w = &self.windows[index];
                    w.window.set_cursor(match edge {
                        Some(d) => winit::window::CursorIcon::from(d),
                        None => input::cursor(w.cursor),
                    });
                }
                // Arrastando com a janela de outro form por cima deste ponto: o destino é ela, e
                // o form de baixo não recebe o movimento (senão o soltar caía nele).
                #[cfg(not(any(web, mobile)))]
                let cover = if edge.is_none() && self.ui.dragging(form) { self.drag_cover(index, *position) } else { None };
                #[cfg(any(web, mobile))]
                let cover: Option<usize> = None;
                if edge.is_none() && cover.is_some() {
                    #[cfg(not(any(web, mobile)))]
                    self.drag_between_windows(index, form, *position, cover);
                } else if edge.is_none() {
                    // Os pontos por onde o mouse passou desde o último evento, antes deste.
                    #[cfg(windows)]
                    {
                        let w = &mut self.windows[index];
                        let points = w.trail.points_before(&w.window, position.x, position.y);
                        for (x, y) in points {
                            let q = self.window_to_form(index, x, y, scale);
                            self.ui.pointer_move(form, pointer_event(q, Default::default(), PointerKind::Mouse));
                        }
                    }
                    self.ui.pointer_move(form, pointer_event(p, Default::default(), PointerKind::Mouse));
                    // O arrasto saiu desta janela: o destino pode estar em outra janela do app.
                    #[cfg(not(any(web, mobile)))]
                    if self.ui.dragging(form) {
                        self.drag_between_windows(index, form, *position, None);
                    }
                }
            }
            WindowEvent::CursorLeft { .. } => {
                #[cfg(windows)]
                self.windows[index].trail.reset();
                self.windows[index].hovered = false;
                // Num form em vários monitores, o mouse que passou para o pedaço vizinho
                // continua no form.
                if !self.windows.iter().any(|w| w.form == form && w.hovered) {
                    self.ui.pointer_leave(form)
                }
            }
            // Arquivos arrastados do sistema (um evento por arquivo).
            WindowEvent::DroppedFile(path) => {
                let p = self.windows[index].pointer;
                self.hovered_files.clear();
                self.ui.files_hover(form, p, None);
                self.ui.drop_files(form, p, vec![path.to_string_lossy().to_string()]);
            }
            // Arquivos do sistema passando por cima (um evento por arquivo): a zona sob o mouse
            // se destaca (F03 do Tabernáculo).
            WindowEvent::HoveredFile(path) => {
                self.hovered_files.push(path.to_string_lossy().to_string());
                let p = self.windows[index].pointer;
                self.ui.files_hover(form, p, Some(self.hovered_files.clone()));
            }
            WindowEvent::HoveredFileCancelled => {
                self.hovered_files.clear();
                let p = self.windows[index].pointer;
                self.ui.files_hover(form, p, None);
            }
            WindowEvent::MouseInput { state: ElementState::Pressed, button: winit::event::MouseButton::Left, .. }
                if self.windows[index].edge.is_some() =>
            {
                if let Some(d) = self.windows[index].edge {
                    let _ = self.windows[index].window.drag_resize_window(d);
                }
            }
            WindowEvent::MouseInput { state, button, .. } => {
                let Some(button) = input::button(*button) else { return true };
                let e = pointer_event(self.windows[index].pointer, button, PointerKind::Mouse);
                match state {
                    ElementState::Pressed => self.ui.pointer_down(form, e),
                    ElementState::Released => self.ui.pointer_up(form, e),
                }
            }
            WindowEvent::MouseWheel { delta: delta_kind, .. } if self.ui.zoom_keys() && modifiers.ctrl => {
                // Ctrl + roda: o zoom do app, de 10 em 10%.
                let up = match delta_kind {
                    MouseScrollDelta::LineDelta(_, y) => *y > 0.0,
                    MouseScrollDelta::PixelDelta(p) => p.y > 0.0,
                };
                let zoom = self.ui.zoom();
                self.ui.set_zoom(if up { zoom + 0.1 } else { zoom - 0.1 });
            }
            WindowEvent::MouseWheel { delta: delta_kind, .. } => {
                // Positivo = rolar para cima / para a esquerda, como no winit.
                let (delta, lines) = self.wheel_delta(delta_kind, scale);
                // No navegador, a posição do próprio evento (sem movimento antes, seria o canto).
                #[cfg(web)]
                if let Some(p) = web::wheel_position() {
                    self.windows[index].pointer = p;
                }
                let e = WheelEvent { position: self.windows[index].pointer, delta, modifiers, lines, ..Default::default() };
                self.ui.wheel(form, e);
            }
            // Android: o analógico e os botões de um controle de jogo chegam como toque e
            // tecla; viram o estado do `Gamepad` e não chegam à tela.
            #[cfg(target_os = "android")]
            WindowEvent::Touch(touch) if android_gamepad::touch(&touch.device_id, touch.phase, touch.location.x, touch.location.y) => {}
            #[cfg(target_os = "android")]
            WindowEvent::KeyboardInput { device_id, event, .. } if android_gamepad::key(device_id, event) => {}
            WindowEvent::Touch(touch) => {
                let p = self.window_to_form(index, touch.location.x, touch.location.y, scale);
                self.windows[index].pointer = p;
                let e = PointerEvent { pointer_id: touch.id, ..pointer_event(p, Default::default(), PointerKind::Touch) };
                match touch.phase {
                    TouchPhase::Started => self.ui.pointer_down(form, e),
                    TouchPhase::Moved => self.ui.pointer_move(form, e),
                    TouchPhase::Ended => {
                        self.ui.pointer_up(form, e);
                        self.ui.pointer_leave(form);
                        self.tapped = true;
                    }
                    TouchPhase::Cancelled => self.ui.pointer_cancel(form),
                }
            }
            // Celular: o "voltar" fecha o form de cima (um modal, uma tela aberta com show);
            // no principal, encerra (o Android volta para a tela anterior).
            WindowEvent::KeyboardInput { event, .. }
                if cfg!(mobile)
                    && input::is_pressed(event)
                    && matches!(event.logical_key, winit::keyboard::Key::Named(winit::keyboard::NamedKey::BrowserBack | winit::keyboard::NamedKey::GoBack)) =>
            {
                // Antes, o `on_back` do form do topo: o app que troca de tela por dentro (frames)
                // trata a tecla e nada mais acontece.
                self.ui.back_pressed(|ui| {
                    let top = ui.open_forms().last().copied();
                    match top {
                        Some(f) if ui.is_modal(f) => ui.set_modal_result(f, aylis_core::ModalResult::Cancel),
                        Some(f) if Some(f) != ui.main_form() => {
                            ui.close(f);
                        }
                        // Android: o app vai para segundo plano e volta como estava (como os
                        // apps do Android 12 em diante); no iOS não há "voltar" do sistema.
                        #[cfg(target_os = "android")]
                        _ if android_jni::move_task_to_back() => {}
                        _ => ui.request_close(),
                    }
                });
            }
            WindowEvent::KeyboardInput { event, .. }
                if self.ui.zoom_keys()
                    && (modifiers.ctrl || modifiers.meta)
                    && input::is_pressed(event)
                    && matches!(event.logical_key.to_text(), Some("+" | "=" | "-" | "0")) =>
            {
                let zoom = self.ui.zoom();
                match event.logical_key.to_text() {
                    Some("+" | "=") => self.ui.set_zoom(zoom + 0.1),
                    Some("-") => self.ui.set_zoom(zoom - 0.1),
                    _ => self.ui.set_zoom(1.0),
                }
            }
            // Ctrl+Shift+M: o painel de memória (só no app em depuração, que o liga).
            WindowEvent::KeyboardInput { event, .. }
                if modifiers.ctrl
                    && modifiers.shift
                    && input::is_pressed(event)
                    && event.logical_key.to_text().is_some_and(|t| t.eq_ignore_ascii_case("m"))
                    && self.ui.toggle_memory_panel() => {}
            WindowEvent::KeyboardInput { event, .. } => {
                let key = input::key_event(event, modifiers);
                if input::is_pressed(event) {
                    self.ui.key_down(form, key);
                    // Ctrl+A, Alt+F... são atalhos, não texto (o winit manda o "a" junto). Com
                    // AltGr (Ctrl+Alt no Windows) a tecla produz texto, como o @.
                    let shortcut = (modifiers.ctrl != modifiers.alt) || modifiers.meta;
                    if let Some(text) = event.text.as_ref().filter(|t| !shortcut && t.chars().all(|c| !c.is_control())) {
                        self.ui.text_input(form, text);
                    }
                } else {
                    self.ui.key_up(form, key);
                }
            }
            // Pinça e giro do touchpad (macOS): o mesmo gesto dos dois dedos na tela.
            WindowEvent::PinchGesture { delta, phase, .. } => {
                let delta = if delta.is_finite() { *delta as f32 } else { 0.0 };
                let e = aylis_core::GestureEvent { phase: gesture_phase(*phase), center: self.windows[index].pointer, scale: 1.0 + delta, delta_scale: 1.0 + delta, ..Default::default() };
                self.ui.gesture(form, e);
            }
            WindowEvent::RotationGesture { delta, phase, .. } => {
                let r = -(*delta).to_radians();
                let e = aylis_core::GestureEvent { phase: gesture_phase(*phase), center: self.windows[index].pointer, scale: 1.0, delta_scale: 1.0, rotation: r, delta_rotation: r, ..Default::default() };
                self.ui.gesture(form, e);
            }
            // Texto composto pelo IME (teclado do iOS, acentos e idiomas asiáticos).
            WindowEvent::Ime(Ime::Commit(text)) => self.ui.text_input(form, text),
            // Num form em vários monitores, o foco que passa de um pedaço para outro não sai do
            // form: quem perdeu espera o fim da volta do laço (`App::settle_deactivations`).
            WindowEvent::Focused(focused) if self.windows[index].span.is_some() => {
                self.windows[index].focused = *focused;
                let moved = self.deactivations.contains(&form);
                if *focused {
                    self.deactivations.retain(|&f| f != form);
                    if !moved {
                        self.ui.fire(form, "on_activate", &aylis_core::EventArgs::None);
                    }
                } else if !moved {
                    self.deactivations.push(form);
                }
            }
            WindowEvent::Focused(focused) => {
                self.windows[index].focused = *focused;
                // O mouse preso volta a ficar preso ao voltar para a janela (o Windows solta ao
                // trocar de janela); ao sair dela, solta.
                #[cfg(not(any(web, mobile)))]
                self.apply_cursor_confine(index);
                if !focused {
                    self.ui.pointer_cancel(form);
                }
                // `on_activate`/`on_deactivate` do form (um menu fecha ao perder o foco).
                self.ui.fire(form, if *focused { "on_activate" } else { "on_deactivate" }, &aylis_core::EventArgs::None);
            }
            _ => return false,
        }
        self.after_input();
        true
    }

    /// Android: aplica no controle com foco o que o teclado do sistema mandou (texto,
    /// composição, correção, emoji, apagar, Enter).
    #[cfg(target_os = "android")]
    pub(crate) fn apply_keyboard_edits(&mut self) {
        let Some(form) = self.windows.first().map(|w| w.form) else { return };
        use aylis_core::{Key, KeyEvent};
        let press = |ui: &mut Ui, key: Key| {
            let e = KeyEvent { key, ..Default::default() };
            ui.key_down(form, e);
            ui.key_up(form, e);
        };
        for edit in android_jni::take() {
            match edit {
                android_jni::Edit::Text { before, after, text } => {
                    for _ in 0..before {
                        press(&mut self.ui, Key::Backspace);
                    }
                    for _ in 0..after {
                        press(&mut self.ui, Key::Delete);
                    }
                    if !text.is_empty() {
                        self.ui.text_input(form, &text);
                    }
                }
                android_jni::Edit::Key(code) => {
                    let key = match code {
                        66 => Key::Enter,
                        67 => Key::Backspace,
                        112 => Key::Delete,
                        61 => Key::Tab,
                        19 => Key::Up,
                        20 => Key::Down,
                        21 => Key::Left,
                        22 => Key::Right,
                        122 => Key::Home,
                        123 => Key::End,
                        _ => continue,
                    };
                    press(&mut self.ui, key);
                }
            }
        }
        self.after_input();
    }

    /// O tipo de teclado do controle com foco: 0 texto, 1 senha, 2 número, 3 e-mail,
    /// 4 texto sem sugestões nem correção (`keyboard_suggestions` desligado).
    #[cfg(target_os = "android")]
    pub(crate) fn keyboard_kind(&self, form: ComponentId) -> i32 {
        let Some(f) = self.ui.focused(form) else { return 0 };
        if matches!(self.ui.get_prop(f, "password"), Ok(PropValue::Bool(true))) {
            return 1;
        }
        let rules = match self.ui.get_prop(f, "rules") {
            Ok(PropValue::String(r)) => r,
            _ => String::new(),
        };
        if rules.contains("number") || self.ui.component(f).map(|c| c.class_name()) == Some("NumberBox") {
            2
        } else if rules.contains("email") {
            3
        } else if matches!(self.ui.get_prop(f, "keyboard_suggestions"), Ok(PropValue::Bool(false))) {
            4
        } else {
            0
        }
    }

    /// Atualiza cursores (e, no celular, o teclado virtual) e pede pintura se algo mudou.
    pub(crate) fn after_input(&mut self) {
        // A imagem do arrasto em andamento acompanha o cursor (ou some, se acabou).
        #[cfg(windows)]
        {
            let visual = self.windows.iter().find_map(|w| self.ui.drag_visual(w.form).map(|(v, _)| (v, w.window.scale_factor() as f32)));
            self.drag_window.sync(visual.as_ref().map(|(v, scale)| (v, *scale)));
        }
        let floating_mode = self.floating_mode();
        for w in &mut self.windows {
            // Com janelas no navegador, o cursor é o do form sob o ponteiro.
            #[cfg(web)]
            let cursor = if floating_mode { self.floating.cursor(&self.ui) } else { self.ui.cursor(w.form) };
            #[cfg(not(web))]
            let cursor = {
                let _ = floating_mode;
                self.ui.cursor(w.form)
            };
            if cursor != w.cursor {
                w.cursor = cursor;
                w.window.set_cursor(input::cursor(cursor));
                w.window.set_cursor_visible(cursor != CursorShape::None);
            }
        }
        // No desktop o teclado é físico; ligar o IME lá mudaria a entrega das teclas.
        if cfg!(mobile) {
            if let Some(w) = self.windows.first() {
                let wanted = self.ui.wants_text_input(w.form);
                let tapped = std::mem::take(&mut self.tapped);
                if wanted != self.keyboard || (wanted && tapped) {
                    self.keyboard = wanted;
                    #[cfg(target_os = "android")]
                    let full = android_jni::show_keyboard(wanted, self.keyboard_kind(w.form));
                    #[cfg(not(target_os = "android"))]
                    let full = false;
                    if !full {
                        w.window.set_ime_allowed(wanted);
                    }
                    self.watch_insets_until = Some(Instant::now() + Duration::from_millis(1500));
                }
            }
        }
        if self.ui.needs_repaint() && self.frame_due().is_none() {
            // Os pedaços extras de um form em vários monitores pintam junto com a principal.
            for w in self.windows.iter().filter(|w| !w.is_extra()) {
                w.window.request_redraw();
            }
        }
    }

    /// Abre e fecha janelas para acompanhar os forms abertos no `Ui`.
    /// O deslocamento de um evento da roda (positivo = para cima / para a esquerda) e se veio
    /// em dentes.
    pub(crate) fn wheel_delta(&self, delta_kind: &MouseScrollDelta, scale: f64) -> (PointF, bool) {
        let step = self.ui.scroll_options().wheel_step;
        match delta_kind {
            // Cada dente vale o passo do app (`Ui::scroll_options`).
            MouseScrollDelta::LineDelta(x, y) => (PointF::new(x * step, y * step), true),
            MouseScrollDelta::PixelDelta(p) => {
                let p = p.to_logical::<f32>(scale);
                match input::wheel_notches(p.x, p.y) {
                    // A roda do mouse no navegador (em pixels, cerca de 100 por dente): vira
                    // dentes, com o passo e o deslize do app.
                    Some((x, y)) => (PointF::new(x * step, y * step), true),
                    // Touchpad: os pixels contínuos, direto.
                    None => (PointF::new(p.x, p.y), false),
                }
            }
        }
    }

    /// Navegador numa tela larga: os forms abertos por cima do principal viram janelas dentro
    /// do canvas (`web_windows`); no celular e numa tela estreita, cada um ocupa a tela.
    pub(crate) fn floating_mode(&self) -> bool {
        #[cfg(web)]
        if let Some(w) = self.windows.first() {
            let logical = surface_size(&w.window).to_logical::<f32>(draw_scale(&w.window));
            return logical.width >= web_windows::MIN_WIDTH;
        }
        false
    }

    /// O ponteiro com janelas no navegador: o `web_windows` entrega ao form sob ele (ou arrasta
    /// e fecha pela barra). `false`: não era do ponteiro.
    #[cfg(web)]
    pub(crate) fn floating_input(&mut self, index: usize, event: &WindowEvent, scale: f64) -> bool {
        use web_windows::Pointer;
        let base = self.windows[index].form;
        let canvas = self.ui.control(base).map_or(aylis_core::SizeF::default(), |cb| aylis_core::SizeF::new(cb.width(), cb.height()));
        let modifiers = self.modifiers;
        let at = |position: PointF, button, kind| PointerEvent { position, button, kind, modifiers, ..Default::default() };
        let mut leave_after = false;
        let pointer = match event {
            WindowEvent::CursorMoved { position, .. } => {
                let p = position.to_logical::<f32>(scale);
                let p = PointF::new(p.x, p.y);
                self.windows[index].pointer = p;
                Pointer::Move(at(p, Default::default(), PointerKind::Mouse))
            }
            WindowEvent::CursorLeft { .. } => Pointer::Leave,
            WindowEvent::MouseInput { state, button, .. } => {
                let Some(button) = input::button(*button) else { return true };
                let e = at(self.windows[index].pointer, button, PointerKind::Mouse);
                if *state == ElementState::Pressed { Pointer::Down(e) } else { Pointer::Up(e) }
            }
            // Ctrl + roda: o zoom do app (o caminho de sempre).
            WindowEvent::MouseWheel { .. } if self.ui.zoom_keys() && modifiers.ctrl => return false,
            WindowEvent::MouseWheel { delta: delta_kind, .. } => {
                let (delta, lines) = self.wheel_delta(delta_kind, scale);
                if let Some(p) = web::wheel_position() {
                    self.windows[index].pointer = p;
                }
                Pointer::Wheel(WheelEvent { position: self.windows[index].pointer, delta, modifiers, lines, ..Default::default() })
            }
            WindowEvent::Touch(touch) => {
                let p = touch.location.to_logical::<f32>(scale);
                let p = PointF::new(p.x, p.y);
                self.windows[index].pointer = p;
                let e = PointerEvent { pointer_id: touch.id, ..at(p, Default::default(), PointerKind::Touch) };
                match touch.phase {
                    TouchPhase::Started => Pointer::Down(e),
                    TouchPhase::Moved => Pointer::Move(e),
                    TouchPhase::Ended => {
                        leave_after = true;
                        self.tapped = true;
                        Pointer::Up(e)
                    }
                    TouchPhase::Cancelled => {
                        let top = self.ui.open_forms().last().copied().unwrap_or(base);
                        self.ui.pointer_cancel(top);
                        return true;
                    }
                }
            }
            _ => return false,
        };
        let mut repaint = self.floating.pointer(&mut self.ui, base, canvas, pointer);
        if leave_after {
            repaint |= self.floating.pointer(&mut self.ui, base, canvas, Pointer::Leave);
        }
        if repaint {
            self.windows[index].window.request_redraw();
        }
        true
    }

    /// Durante o arrasto da janela `index` pela barra de título (só a de um form `dockable`):
    /// diz ao núcleo sobre que janela do app (e que ponto dela) o mouse está, para o destino
    /// acender; fora das janelas do app, ou atrás de outro programa, nenhum.
    #[cfg(windows)]
    pub(crate) fn window_drag_over(&mut self, index: usize) {
        let Some(w) = self.windows.get(index) else { return };
        let form = w.form;
        if !win32::in_size_move(form) || !matches!(self.ui.get_prop(form, "dockable"), Ok(PropValue::Bool(true))) {
            return;
        }
        let Some(hwnd) = win32::hwnd_of(&w.window) else { return };
        let Some((x, y)) = win32::cursor_pos() else { return };
        let over = win32::window_below(hwnd.0 as isize, x, y).and_then(|below| {
            let i = self.windows.iter().position(|o| win32::hwnd_of(&o.window).is_some_and(|h| h.0 as isize == below))?;
            let at = self.windows[i].window.inner_position().ok()?;
            let local = (x as f64 - at.x as f64, y as f64 - at.y as f64);
            Some((self.windows[i].form, self.window_to_form(i, local.0, local.1, self.input_scale(i))))
        });
        self.ui.window_drag_over(form, over);
    }
}
