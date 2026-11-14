//! O componente `Model3D`: um modelo glTF/GLB animado, com câmera orbital, composto com os
//! controles do aylis (os que estiverem por cima aparecem por cima, como qualquer controle).

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use glam::{Mat4, Quat, Vec3};
use aylis_core::{
    Brush, Canvas, Color, Component, ControlBase, CursorShape, EventArgs, EventCtx, FontSpec, GpuDevice, Image, PaintCtx, PointF, PointerEvent,
    RectF, TextAlign, TextStyle, WheelEvent, properties,
};

use crate::model::{Model, Pose, Projection};
use crate::render::{Engine, Frame, GpuModel, MAX_SAMPLES, Output, View};

/// Um dispositivo do 3D: a GPU do canvas, a geração e o motor.
type Engines = (Option<GpuDevice>, u64, Engine);

thread_local! {
    /// Um dispositivo por GPU do canvas (o adaptador do Direct2D, cada contexto OpenGL, ou
    /// nenhum), criado no primeiro `Model3D` desenhado nela.
    ///
    /// Não são destruídos no fim da thread: no OpenGL, o contexto já não existe nessa hora
    /// e destruir o dispositivo travaria. O sistema libera tudo ao fechar o processo.
    static ENGINES: std::mem::ManuallyDrop<RefCell<Vec<Engines>>> =
        const { std::mem::ManuallyDrop::new(RefCell::new(Vec::new())) };
    static GENERATION: Cell<u64> = const { Cell::new(0) };
    /// No navegador: a GPU já está sendo aberta.
    #[cfg(target_arch = "wasm32")]
    static WEB_PENDING: Cell<bool> = const { Cell::new(false) };
}

thread_local! {
    /// O envio do fim do quadro já foi pedido (uma vez por quadro, para todos os `Model3D`).
    static SUBMIT_ASKED: Cell<bool> = const { Cell::new(false) };
}

/// Pede ao canvas que, antes de executar o quadro, o motor de `device` envie os comandos que
/// os `Model3D` gravaram ([`Engine::submit_pending`]).
fn submit_at_frame_end(device: Option<GpuDevice>) {
    if SUBMIT_ASKED.try_with(|asked| asked.replace(true)).unwrap_or(true) {
        return;
    }
    aylis_core::gpu::before_flush(move || {
        let _ = SUBMIT_ASKED.try_with(|asked| asked.set(false));
        with_engine(device, |engine, _| engine.submit_pending());
    });
}

/// Usa o dispositivo do 3D para a GPU `device` do canvas, criando se preciso. Devolve
/// também a geração (cada dispositivo tem a sua: os recursos de um não servem no outro).
fn with_engine<R>(device: Option<GpuDevice>, f: impl FnOnce(&mut Engine, u64) -> R) -> Option<R> {
    // `try_with`: pintar com a thread acabando (o app fechando) só não desenha.
    ENGINES.try_with(|cell| {
        let mut list = cell.borrow_mut();
        let at = match list.iter().position(|(d, _, _)| *d == device) {
            Some(at) => at,
            // No navegador a GPU abre de forma assíncrona: começa agora e desenha quando
            // estiver pronta (o Model3D continua pedindo quadros).
            #[cfg(target_arch = "wasm32")]
            None if device == Some(GpuDevice::Web) => {
                if !WEB_PENDING.with(|p| p.replace(true)) {
                    wasm_bindgen_futures::spawn_local(async move {
                        match Engine::new_web().await {
                            Ok(engine) => {
                                let generation = GENERATION.with(|g| {
                                    g.set(g.get() + 1);
                                    g.get()
                                });
                                ENGINES.with(|c| c.borrow_mut().push((Some(GpuDevice::Web), generation, engine)));
                            }
                            // No console do navegador: sem isso, um 3D que não abre some calado.
                            Err(e) => web_sys::console::warn_1(&format!("aylis-3d: sem GPU para o 3D: {e}").into()),
                        }
                    });
                }
                return None;
            }
            None => {
                let engine = match Engine::new(device) {
                    Ok(e) => e,
                    Err(e) => {
                        crate::render::trace(&format!("sem dispositivo: {e}"));
                        return None;
                    }
                };
                let generation = GENERATION.with(|g| {
                    g.set(g.get() + 1);
                    g.get()
                });
                list.push((device, generation, engine));
                list.len() - 1
            }
        };
        let (_, generation, engine) = &mut list[at];
        Some(f(engine, *generation))
    })
    .ok()
    .flatten()
}

/// O que o componente guarda entre os quadros.
#[derive(Default)]
struct State {
    model: Option<Rc<Model>>,
    loaded: String,
    error: Option<String>,
    gpu: Option<(u64, GpuModel, View)>,
    rest: Option<Pose>,
    /// Clipe atual e o tempo dele (segundos de animação).
    clip: Option<(usize, f32)>,
    /// O anterior, durante a transição, e quando ela começou.
    fading: Option<(usize, f32, f64)>,
    playing_name: String,
    last_time: Option<f64>,
    spin: f32,
    /// Já disse por onde o quadro passou (`AYLIS_3D_DEBUG`).
    reported: bool,
    /// Traçado de raios: a câmera e o tamanho do último quadro e quantas amostras já foram
    /// acumuladas com eles (a cena parada).
    rays_key: Option<[f32; 34]>,
    sample: u32,
}

/// Modelo 3D (glTF 2.0 / GLB) com esqueleto, morph targets e animação.
///
/// - `source`: o arquivo (`.glb`), ou use [`Model3D::set_model`] por código;
/// - `animation`: o clipe tocando (vazio = o primeiro); trocar faz uma transição de
///   `crossfade` ms;
/// - câmera (`orbit`): arraste (ou um dedo) para girar (`yaw`, `pitch`), roda ou pinça para
///   aproximar (`zoom`), dois dedos para deslocar a vista (toque duplo volta ao centro); ou uma
///   câmera do arquivo pelo nome em `camera` (anda com a animação; o mouse não mexe);
/// - luzes: as do arquivo (`KHR_lights_punctual`, até 4) com `file_lights`, senão uma
///   direcional padrão; `shadows` liga a sombra da primeira direcional;
/// - `ray_traced`: traçado de raios, onde a placa traça (ray query; senão, o desenho de
///   sempre): sombra de todas as luzes, reflexo nos materiais lisos e oclusão de ambiente.
///   Com a cena parada, as amostras se acumulam (sombras suaves, reflexos espalhados pela
///   aspereza) até 256;
/// - `background`: sem cor, o fundo é transparente (o que está atrás aparece);
/// - `on_clip_end` (com o nome do clipe): o clipe chegou ao fim. Sem `looping`, uma vez (o
///   modelo fica na última pose; atribuir `animation` de novo, mesmo com o mesmo nome, ou
///   `playing = true` toca do começo); com `looping`, a cada volta. Como a animação anda na pintura, um `Model3D` invisível não avisa.
pub struct Model3D {
    pub base: ControlBase,
    /// O arquivo do modelo (`.glb`, glTF 2.0) no projeto.
    pub source: String,
    /// O nome do clipe de animação tocando; vazio: o primeiro do arquivo.
    pub animation: String,
    /// A animação anda; desligado, o modelo fica na pose atual.
    pub playing: bool,
    /// O clipe recomeça ao chegar ao fim; desligado, toca uma vez e fica na última pose.
    pub looping: bool,
    /// Velocidade da animação (1 = normal, 2 = o dobro).
    pub speed: f32,
    /// Duração da transição ao trocar de clipe, em ms; 0: troca seca.
    pub crossfade: f32,
    /// Giro da câmera em volta do modelo, na horizontal, em graus.
    pub yaw: f32,
    /// Inclinação da câmera, olhando de cima ou de baixo, em graus.
    pub pitch: f32,
    /// Aproximação da câmera (1 = o modelo inteiro na vista; maior aproxima), de 0,2 a 8.
    pub zoom: f32,
    /// Giro do modelo em graus.
    pub model_rotation: f32,
    /// Giro contínuo, em graus por segundo.
    pub auto_rotate: f32,
    /// Arrastar e a roda do mouse mexem na câmera.
    pub orbit: bool,
    /// Câmera do arquivo pelo nome (vazio = a órbita).
    pub camera: String,
    /// Usar as luzes do arquivo, quando houver.
    pub file_lights: bool,
    /// Sombra da primeira luz direcional.
    pub shadows: bool,
    /// Traçado de raios (onde a placa traça).
    pub ray_traced: bool,
    /// A cor do fundo; sem cor, transparente (o que está atrás aparece).
    pub background: Option<Color>,
    /// Mesa giratória: as luzes e as câmeras do arquivo ficam paradas quando o modelo gira
    /// (`model_rotation` e `auto_rotate`), como um objeto girando debaixo da luz e da câmera
    /// do estúdio: o reflexo anda pelo modelo. Só por código (desligado: elas giram junto,
    /// como parte da cena).
    pub turntable: bool,
    /// Brilho (bloom): a luz que os materiais emitem espalha em volta deles, forte junto da
    /// fonte e caindo rápido. O valor é a força (0 = sem brilho). Só por código.
    pub glow: f32,
    /// Em quantos pixels por ponto o 3D é desenhado (a imagem é ampliada para o controle).
    /// 0 = automático: a densidade da tela, mas no celular no máximo [`AUTO_RESOLUTION`]
    /// (as telas de 2,5 a 3,5 px/ponto deixam a GPU do celular abaixo de 60 quadros/s, e num
    /// 3D de luz suave a diferença quase não se vê). Mais alto: mais nítido e mais pesado.
    pub resolution: f32,
    state: RefCell<State>,
    drag: Cell<Option<(PointF, f32, f32)>>,
    /// Sem a órbita, no `turntable`: o mouse arrastando gira o modelo (onde começou e o giro de
    /// então); o dedo continua rolando a tela.
    spin_drag: Cell<Option<(PointF, f32)>>,
    /// O deslocamento da câmera feito com dois dedos (em alturas da vista com zoom 1; o toque
    /// duplo volta ao centro).
    pan: Cell<(f32, f32)>,
}

impl Default for Model3D {
    fn default() -> Self {
        let mut base = ControlBase::new(320.0, 240.0);
        base.set_cursor(CursorShape::Move);
        // O arrasto do dedo gira a câmera (não rola a tela por baixo).
        base.set_pan_on_touch(false);
        Self {
            base,
            source: String::new(),
            animation: String::new(),
            playing: true,
            looping: true,
            speed: 1.0,
            crossfade: 250.0,
            yaw: 30.0,
            pitch: 15.0,
            zoom: 1.0,
            model_rotation: 0.0,
            auto_rotate: 0.0,
            orbit: true,
            camera: String::new(),
            file_lights: true,
            shadows: true,
            ray_traced: false,
            background: None,
            turntable: false,
            glow: 0.0,
            resolution: 0.0,
            state: RefCell::default(),
            drag: Cell::new(None),
            spin_drag: Cell::new(None),
            pan: Cell::new((0.0, 0.0)),
        }
    }
}

/// A resolução automática no celular (pixels por ponto): no ASUS ZenFone 5 (Adreno 512, 2,8
/// px/ponto) o lingote da abertura com brilho fazia 16 quadros/s na densidade da tela, 63 com
/// 1,25 e 84 com 1,0.
#[cfg(any(target_os = "android", target_os = "ios"))]
pub const AUTO_RESOLUTION: f32 = 1.1;
/// No desktop, a densidade da tela (sem limite).
#[cfg(not(any(target_os = "android", target_os = "ios")))]
pub const AUTO_RESOLUTION: f32 = f32::INFINITY;

impl Model3D {
    /// Usa um modelo já lido (o `source` é ignorado).
    pub fn set_model(&mut self, model: Model) {
        let st = self.state.get_mut();
        *st = State { rest: Some(model.rest_pose()), model: Some(Rc::new(model)), loaded: self.source.clone(), ..Default::default() };
    }

    pub fn with_model(mut self, model: Model) -> Self {
        self.set_model(model);
        self
    }

    pub fn model(&self) -> Option<Rc<Model>> {
        self.state.borrow().model.clone()
    }

    /// Os nomes dos clipes do modelo.
    pub fn clips(&self) -> Vec<String> {
        self.model().map(|m| m.clips.iter().map(|c| c.name.clone()).collect()).unwrap_or_default()
    }

    /// Os nomes das câmeras do arquivo (para `camera`).
    pub fn cameras(&self) -> Vec<String> {
        self.model().map(|m| m.cameras.iter().map(|c| c.name.clone()).collect()).unwrap_or_default()
    }

    /// Toca o clipe atual do começo (por código; peça uma repintura depois).
    pub fn restart(&self) {
        if let Some((_, t)) = self.state.borrow_mut().clip.as_mut() {
            *t = 0.0;
        }
    }

    /// O clipe atual, sem `looping`, já chegou ao fim.
    fn finished(&self) -> bool {
        let st = self.state.borrow();
        !self.looping
            && st.playing_name == self.animation
            && st.clip.zip(st.model.as_ref()).is_some_and(|((c, t), m)| m.clips.get(c).is_some_and(|clip| t >= clip.duration))
    }

    /// O clipe atual e o tempo dele, em segundos.
    pub fn clip_time(&self) -> Option<(String, f32)> {
        let st = self.state.borrow();
        let (c, t) = st.clip?;
        Some((st.model.as_ref()?.clips.get(c)?.name.clone(), t))
    }

    /// O erro ao carregar o `source`, se houver.
    pub fn error(&self) -> Option<String> {
        self.state.borrow().error.clone()
    }

    fn load(&self, ctx: &PaintCtx<'_>) {
        let mut st = self.state.borrow_mut();
        if st.loaded == self.source {
            return;
        }
        st.loaded = self.source.clone();
        if self.source.is_empty() {
            return;
        }
        let bytes = ctx.resource(&self.source).or_else(|| std::fs::read(&self.source).ok());
        match bytes.map(|b| Model::from_glb(&b)) {
            Some(Ok(model)) => {
                *st = State { rest: Some(model.rest_pose()), model: Some(Rc::new(model)), loaded: self.source.clone(), ..Default::default() };
            }
            Some(Err(e)) => st.error = Some(e),
            None => st.error = Some(format!("não achei '{}'", self.source)),
        }
    }

    /// Avança a animação até `now` e devolve a pose e, se o clipe atual chegou ao fim neste
    /// passo, o nome dele.
    fn pose(&self, st: &mut State, model: &Model, now: f64) -> (Pose, Option<String>) {
        let dt = st.last_time.map_or(0.0, |t| (now - t).max(0.0) as f32);
        st.last_time = Some(now);
        // Parado enquanto o mouse gira o modelo com a mão.
        if self.auto_rotate != 0.0 && self.spin_drag.get().is_none() {
            st.spin = (st.spin + self.auto_rotate * dt) % 360.0;
        }
        // Troca de clipe: o atual vira o "anterior" da transição.
        let wanted = if self.animation.is_empty() { (!model.clips.is_empty()).then_some(0) } else { model.clip(&self.animation) };
        if st.playing_name != self.animation || st.clip.map(|c| c.0) != wanted {
            st.playing_name = self.animation.clone();
            if let (Some(old), true) = (st.clip, self.crossfade > 0.0) {
                st.fading = Some((old.0, old.1, now));
            }
            st.clip = wanted.map(|c| (c, 0.0));
        }
        let step = if self.playing { dt * self.speed } else { 0.0 };
        let advance = |c: usize, t: f32| {
            let d = model.clips[c].duration.max(1e-6);
            let t = t + step;
            if self.looping { t.rem_euclid(d) } else { t.min(d) }
        };
        let rest = st.rest.clone().unwrap_or_else(|| model.rest_pose());
        let mut pose = rest.clone();
        let mut ended = None;
        if let Some((c, t)) = st.clip {
            let d = model.clips[c].duration;
            // Sem `looping` o tempo para em `d`: avisa só na chegada.
            if step > 0.0 && t + step >= d && (self.looping || t < d) {
                ended = Some(model.clips[c].name.clone());
            }
            let t = advance(c, t);
            st.clip = Some((c, t));
            model.sample(c, t, &mut pose);
        }
        if let Some((c, t, start)) = st.fading {
            let f = ((now - start) as f32 * 1000.0 / self.crossfade.max(1.0)).clamp(0.0, 1.0);
            if f >= 1.0 {
                st.fading = None;
            } else {
                let t = advance(c, t);
                st.fading = Some((c, t, start));
                let mut old = rest;
                model.sample(c, t, &mut old);
                old.blend(&pose, f);
                pose = old;
            }
        }
        (pose, ended)
    }

    fn animating(&self) -> bool {
        // Um clipe sem `looping` que já acabou não pede mais quadros.
        let finished = self.finished();
        let st = self.state.borrow();
        let has_clip = st.model.as_ref().is_some_and(|m| !m.clips.is_empty());
        (self.playing && has_clip && !finished) || st.fading.is_some() || self.auto_rotate != 0.0
    }
}

impl Component for Model3D {
    // O valor só muda a pintura: não refaz o layout da tela.
    fn paint_only_property(&self, name: &str) -> bool {
        matches!(name, "yaw" | "pitch" | "zoom" | "model_rotation" | "auto_rotate" | "playing" | "looping" | "speed" | "animation" | "shadows" | "file_lights" | "ray_traced" | "glow" | "resolution")
    }

    fn class_name(&self) -> &'static str {
        "Model3D"
    }

    fn control(&self) -> Option<&ControlBase> {
        Some(&self.base)
    }

    fn control_mut(&mut self) -> Option<&mut ControlBase> {
        Some(&mut self.base)
    }

    properties! {
        source: String => "Data",
        animation: String => "Animation",
        playing: bool => "Animation",
        looping: bool => "Animation",
        speed: f32 => "Animation",
        crossfade: f32 as Duration => "Animation",
        yaw: f32 as Angle => "Layout",
        pitch: f32 as Angle => "Layout",
        zoom: f32 => "Layout",
        model_rotation: f32 as Angle => "Layout",
        auto_rotate: f32 => "Animation",
        orbit: bool => "Behavior",
        camera: String => "Layout",
        file_lights: bool => "Appearance",
        shadows: bool => "Appearance",
        ray_traced: bool => "Appearance",
        background: Option<Color> => "Appearance",
        resolution: f32 => "Appearance";
        events: on_clip_end
    }

    fn property_changed(&mut self, ctx: &mut EventCtx<'_>, name: &str) {
        // Sem a órbita, o arrasto do dedo volta a rolar a tela por baixo.
        if name == "orbit" {
            self.base.set_pan_on_touch(!self.orbit);
        }
        // O cursor de arrastar só quando arrastar faz algo (girar a câmera ou o modelo).
        if matches!(name, "orbit" | "turntable") {
            self.base.set_cursor(if self.orbit || self.turntable { CursorShape::Move } else { CursorShape::Default });
        }
        // Depois do fim, pedir o clipe (ou o play) de novo toca do começo.
        if (name == "animation" || (name == "playing" && self.playing)) && self.finished() {
            self.restart();
            ctx.repaint();
        }
    }

    fn pointer_down(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if self.orbit {
            self.drag.set(Some((event.position, self.yaw, self.pitch)));
            if event.click_count == 2 && self.pan.get() != (0.0, 0.0) {
                self.pan.set((0.0, 0.0));
                ctx.repaint();
            }
        } else if self.turntable && event.kind != aylis_core::PointerKind::Touch {
            self.spin_drag.set(Some((event.position, self.model_rotation)));
        }
    }

    fn pointer_move(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if let Some((start, rotation)) = self.spin_drag.get() {
            self.model_rotation = rotation + (event.position.x - start.x) * 0.5;
            ctx.repaint();
            return;
        }
        if let Some((start, yaw, pitch)) = self.drag.get() {
            self.yaw = yaw - (event.position.x - start.x) * 0.4;
            self.pitch = (pitch + (event.position.y - start.y) * 0.3).clamp(-85.0, 85.0);
            ctx.repaint();
        }
    }

    fn pointer_up(&mut self, _ctx: &mut EventCtx<'_>, _event: &PointerEvent) {
        self.drag.set(None);
        self.spin_drag.set(None);
    }

    /// Os controles de sempre dos visualizadores 3D: pinça aproxima/afasta e arrastar com os
    /// dois dedos desloca a vista (girar os dedos não faz nada; um dedo gira a câmera).
    fn gesture(&mut self, ctx: &mut EventCtx<'_>, event: &aylis_core::GestureEvent) {
        if self.orbit {
            self.drag.set(None);
            self.zoom = (self.zoom * event.delta_scale).clamp(0.2, 8.0);
            // Em alturas da vista com zoom 1: o deslocamento fica parado na cena quando o zoom
            // muda depois (e o modelo acompanha o dedo no zoom de agora).
            let height = self.base.bounds().height.max(1.0) * self.zoom.max(0.05);
            let (x, y) = self.pan.get();
            self.pan.set((x + event.pan.x / height, y + event.pan.y / height));
            ctx.repaint();
            ctx.set_handled();
        }
    }

    fn wheel(&mut self, ctx: &mut EventCtx<'_>, event: &WheelEvent) {
        // A roda do mouse aproxima; o arrasto do dedo (que chega como roda) é da órbita.
        if self.orbit && !event.touch {
            self.zoom = (self.zoom * 1.1f32.powf(event.delta.y.signum())).clamp(0.2, 8.0);
            ctx.repaint();
            ctx.set_handled();
        }
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        if let Some(bg) = self.background {
            canvas.fill_rect(r, 0.0, &Brush::Solid(bg));
        }
        self.load(ctx);
        let scale = canvas.device_scale().max(0.1);
        // A resolução em que o 3D é desenhado (a imagem é ampliada para o controle).
        let scale = if self.resolution > 0.0 { self.resolution } else { scale.min(AUTO_RESOLUTION) };
        let (w, h) = ((r.width * scale).round() as u32, (r.height * scale).round() as u32);
        let animating = self.animating();
        let mut st = self.state.borrow_mut();
        let Some(model) = st.model.clone() else {
            let (text, color) = match &st.error {
                Some(e) => (e.clone(), ctx.theme.color("danger")),
                None if ctx.design => ("Model3D\nescolha um .glb em source".to_string(), ctx.theme.color("text_muted")),
                None => return,
            };
            if ctx.design {
                let stroke = aylis_core::Stroke::new(ctx.theme.color("border"), 1.0).dashed(vec![5.0, 4.0]);
                canvas.stroke_rect(r.inflate(-0.5), 6.0, &stroke);
            }
            let style = TextStyle { font: FontSpec::new("", 13.0), color, h_align: TextAlign::Center, v_align: TextAlign::Center, wrap: true, ..Default::default() };
            canvas.draw_text(&text, r, &style);
            return;
        };
        if w < 2 || h < 2 {
            return;
        }
        // No designer o modelo fica parado na pose inicial (o designer não
        // roda animações): sem isso, a IDE inteira se redesenharia sem parar.
        let (pose, ended) = self.pose(&mut st, &model, if ctx.design { 0.0 } else { ctx.time });
        if let Some(name) = ended {
            // A pintura não dispara eventos: vai para a fila e roda logo depois.
            let id = ctx.id;
            ctx.ui.poster().post(move |ui| {
                ui.fire(id, "on_clip_end", &EventArgs::Text(name));
            });
        }

        // Câmera em volta da caixa do modelo.
        let (lo, hi) = model.bounds;
        let center = (lo + hi) / 2.0;
        let radius = ((hi - lo).length() / 2.0).max(1e-3);
        let fov = 40f32.to_radians();
        // A esfera cabe na menor abertura: em pé (celular), a horizontal é a mais fechada.
        let half = ((fov / 2.0).tan() * (w as f32 / h as f32).min(1.0)).atan();
        let distance = radius / half.sin() * 1.05 / self.zoom.max(0.05);
        let (yaw, pitch) = (self.yaw.to_radians(), self.pitch.to_radians());
        let back = Vec3::new(yaw.sin() * pitch.cos(), pitch.sin(), yaw.cos() * pitch.cos());
        // O deslocamento dos dois dedos: o modelo acompanha o dedo na tela.
        let (pan_x, pan_y) = self.pan.get();
        let right = Vec3::Y.cross(back).normalize_or_zero();
        let up = back.cross(right);
        let span = 2.0 * distance * self.zoom.max(0.05) * (fov / 2.0).tan();
        let target = center - right * pan_x * span + up * pan_y * span;
        let eye = target + back * distance;
        let spin = Quat::from_rotation_y((self.model_rotation + st.spin).to_radians());
        let root = Mat4::from_translation(center) * Mat4::from_quat(spin) * Mat4::from_translation(-center);
        let mut world = model.world(&pose, root);
        if self.turntable && (!model.lights.is_empty() || !model.cameras.is_empty()) {
            // As luzes e as câmeras não entram no giro: o modelo gira debaixo delas.
            let still = model.world(&pose, Mat4::IDENTITY);
            for node in model.lights.iter().map(|l| l.node).chain(model.cameras.iter().map(|c| c.node)) {
                if let (Some(w), Some(s)) = (world.get_mut(node), still.get(node)) {
                    *w = *s;
                }
            }
        }
        let aspect = w as f32 / h as f32;
        let file_camera = (!self.camera.is_empty()).then(|| model.camera(&self.camera)).flatten();
        let (view, proj, eye) = match file_camera {
            // A câmera do nó (sem a escala dele); a proporção é a do controle.
            Some(cam) => {
                let (_, rotation, position) = world.get(cam.node).copied().unwrap_or(Mat4::IDENTITY).to_scale_rotation_translation();
                let view = Mat4::from_rotation_translation(rotation, position).inverse();
                let proj = match cam.projection {
                    Projection::Perspective { yfov, znear, zfar: Some(zfar), .. } => glam::camera::rh::proj::directx::perspective(yfov, aspect, znear, zfar),
                    Projection::Perspective { yfov, znear, zfar: None, .. } => glam::camera::rh::proj::directx::perspective_infinite(yfov, aspect, znear),
                    Projection::Orthographic { ymag, znear, zfar, .. } => {
                        glam::camera::rh::proj::directx::orthographic(-ymag * aspect, ymag * aspect, -ymag, ymag, znear, zfar)
                    }
                };
                (view, proj, position)
            }
            None => (
                glam::camera::rh::view::look_at_mat4(eye, target, Vec3::Y),
                glam::camera::rh::proj::directx::perspective(fov, aspect, distance * 0.01, distance * 10.0 + radius * 4.0),
                eye,
            ),
        };
        let frame = Frame {
            width: w,
            height: h,
            view,
            proj,
            eye,
            background: [0.0; 4],
            light_dir: Vec3::new(-0.4, -1.0, -0.6),
            file_lights: self.file_lights,
            shadows: self.shadows,
            ray_traced: self.ray_traced,
            sample: 0,
            glow: self.glow,
        };
        // Com a cena parada (a mesma câmera, o mesmo tamanho, nada animando nem sendo
        // arrastado), cada quadro é mais uma amostra na média.
        let mut frame = frame;
        if self.ray_traced {
            let mut key = [0.0f32; 34];
            key[..16].copy_from_slice(&view.to_cols_array());
            key[16..32].copy_from_slice(&proj.to_cols_array());
            key[32] = w as f32;
            key[33] = h as f32;
            let still = !animating && self.drag.get().is_none() && st.rays_key == Some(key);
            // Mexendo, o quadro sem sorteio (sem chuvisco); parada, as amostras sorteadas.
            st.sample = if still { (st.sample + 1).min(MAX_SAMPLES) } else { 0 };
            st.rays_key = Some(key);
            frame.sample = st.sample;
        }

        let device = canvas.gpu_device();
        let share = device.is_some() && crate::diag::diagnostic_setting("AYLIS_3D_COPY").is_none();
        let mut tracing = false;
        let output = with_engine(device, |engine, generation| {
            tracing = engine.ray_tracing();
            if st.gpu.as_ref().is_none_or(|g| g.0 != generation) {
                st.gpu = Some((generation, engine.upload(&model), engine.view()));
            }
            crate::render::trace("render");
            let (_, gpu, view) = st.gpu.as_mut()?;
            engine.render(&model, gpu, view, &world, &pose.weights, &frame, share)
        })
        .flatten();
        drop(st);
        // No OpenGL o 3D desenha no contexto do canvas: o Skia refaz o estado dele, tanto com a
        // textura dividida quanto com a cópia pela CPU (sem isso, a janela inteira some).
        if matches!(device, Some(aylis_core::GpuDevice::OpenGl { .. })) {
            canvas.gpu_state_changed();
        }
        // No OpenGL, os comandos de todos os `Model3D` do quadro vão juntos, antes de o canvas
        // executar os desenhos dele (um envio por controle pesava na CPU do celular).
        if matches!(output, Some(Output::Surface(_))) && matches!(device, Some(aylis_core::GpuDevice::OpenGl { .. })) {
            submit_at_frame_end(device);
        }
        let output_missing = output.is_none();
        let debug = !self.state.borrow().reported && crate::diag::diagnostic_setting("AYLIS_3D_DEBUG").is_some();
        match output {
            Some(Output::Surface(surface)) => {
                let ok = canvas.draw_gpu_surface(r, &surface);
                if !ok {
                    if let Some((_, _, view)) = self.state.borrow_mut().gpu.as_mut() {
                        view.rejected();
                    }
                }
                if debug {
                    crate::diag::diagnostic(&format!("aylis-3d: {w}×{h}, {}", if ok { "quadro pela GPU (sem cópia)" } else { "o canvas recusou a textura" }));
                }
            }
            Some(Output::Pixels { width, height, rgba }) => {
                if debug {
                    crate::diag::diagnostic(&format!("aylis-3d: {w}×{h}, quadro copiado pela CPU"));
                }
                let image = Image::rgba(width, height, rgba);
                canvas.draw_image(&image, RectF::new(0.0, 0.0, width as f32, height as f32), r, 1.0);
            }
            None => {}
        }
        if debug {
            self.state.borrow_mut().reported = true;
        }
        // Com a cena parada, mais quadros enquanto houver amostras a acumular.
        let accumulating = tracing && self.ray_traced && frame.sample < MAX_SAMPLES;
        if (self.animating() && !ctx.design) || output_missing || accumulating {
            ctx.request_frame();
        }
    }
}
