//! Percurso de pintura da árvore e o contexto entregue a cada controle.

use std::cell::Cell;

use crate::anim::Easing;
use crate::canvas::{Canvas, TextStyle};
use crate::component::{ComponentId, ControlState};
use crate::geometry::{PointF, RectF};
use crate::theme::{ResolvedStyle, Theme};
use crate::ui::Ui;

/// O que um controle recebe para se pintar.
pub struct PaintCtx<'a> {
    /// O `Ui` (só leitura): para ler os filhos ou propriedades de outros componentes.
    pub ui: &'a Ui,
    /// O controle sendo pintado.
    pub id: ComponentId,
    /// Área do controle, na origem (o canvas já foi posicionado).
    pub rect: RectF,
    pub state: ControlState,
    /// Estilo do tema já resolvido para a classe, a variante e o estado atuais (no meio
    /// de uma transição, já vem interpolado).
    pub style: &'a ResolvedStyle,
    pub theme: &'a Theme,
    /// Tempo em segundos, para animações.
    pub time: f64,
    /// Pintando no designer da IDE: controles invisíveis no app (uma imagem vazia, um
    /// contêiner sem borda) podem se mostrar para serem encontrados.
    pub design: bool,
    /// As cores próprias do controle (já postas no `style` e no `theme`): o controle
    /// sabe, por exemplo, que o fundo é dele e não o desenho da plataforma.
    pub colors: crate::ColorOverride,
    frame_request: &'a Cell<bool>,
    wake_at: &'a Cell<Option<f64>>,
    resources: Option<&'a dyn crate::platform::ResourceService>,
}

impl PaintCtx<'_> {
    /// O mesmo contexto com outro estilo: um controle com cores próprias (o `fill` de um
    /// botão) troca as do tema e continua usando o desenho de sempre.
    pub fn with_style<'b>(&'b self, style: &'b ResolvedStyle) -> PaintCtx<'b> {
        PaintCtx { style, ..*self }
    }

    /// Lê um arquivo do aplicativo (`assets/images/logo.png`) pelo serviço de recursos.
    pub fn resource(&self, path: &str) -> Option<Vec<u8>> {
        self.resources?.read(path)
    }

    /// Desenha um ícone pelo nome (os do app e os do aylis), centralizado em `rect`.
    /// Nome desconhecido não desenha nada.
    pub fn draw_icon(&self, canvas: &mut dyn crate::Canvas, name: &str, rect: RectF, color: crate::Color) {
        if let Some(icon) = self.ui.icon(name) {
            icon.draw(canvas, rect, color);
        }
    }

    /// A forma dos controles no tema deste controle ([`Look`](crate::theme::Look)).
    pub fn look(&self) -> crate::theme::Look {
        self.theme.look
    }

    /// A variante (`style`) do controle sendo pintado ("" = a padrão).
    pub fn variant(&self) -> &str {
        self.ui.control(self.id).map_or("", |c| c.style())
    }

    /// Estilo de texto com a fonte e a cor do tema.
    pub fn text_style(&self) -> TextStyle {
        TextStyle { font: self.style.font.clone(), color: self.style.text, ..Default::default() }
    }

    /// A fonte do controle (família, peso, itálico...) no tamanho que o desenho pede; o
    /// `font_size` próprio, se houver, vale mais.
    pub fn font(&self, size: f32) -> crate::FontSpec {
        let cb = self.ui.control(self.id);
        let own = cb.and_then(|c| c.font().size);
        // A escala de fonte da vista ativa do form (o `style.font` já vem escalado).
        let scale = cb.map_or(1.0, |c| c.text_scale());
        crate::FontSpec { size: own.unwrap_or(size) * scale, ..self.style.font.clone() }
    }

    /// Como [`PaintCtx::font`], em monoespaçada, se ninguém (o controle ou um pai) escolheu a
    /// família.
    pub fn monospace_font(&self, size: f32) -> crate::FontSpec {
        let mut font = self.font(size);
        let mut current = Some(self.id);
        while let Some(id) = current {
            if self.ui.control(id).is_some_and(|c| !c.font().family.is_empty()) {
                return font;
            }
            current = self.ui.parent(id);
        }
        font.family = "monospace".into();
        font
    }

    /// Pede um frame no horário `time` (em segundos, na escala de [`PaintCtx::time`]). Bom
    /// para mudanças pontuais, como o cursor de texto piscando: a plataforma dorme até lá
    /// em vez de repintar sem parar.
    pub fn request_frame_at(&self, time: f64) {
        let next = self.wake_at.get().map_or(time, |t| t.min(time));
        self.wake_at.set(Some(next));
        // Na hora, só este controle é repintado (veja `Ui::take_due_frames`).
        let mut timed = self.ui.timed_frames.borrow_mut();
        let at = timed.get(&self.id).map_or(time, |t| t.min(time));
        timed.insert(self.id, at);
    }

    /// Pede mais um frame logo em seguida. Use em pinturas que mudam com `time` (efeitos
    /// animados): enquanto alguém pedir, a plataforma continua repintando.
    pub fn request_frame(&self) {
        self.frame_request.set(true);
        // No próximo quadro, só a área deste controle (veja `Ui::damage`).
        self.ui.frame_ids.borrow_mut().push(self.id);
    }

    /// Desenha por cima de tudo do form, depois que todos os controles foram pintados
    /// (como a dica): um balão que sai da área do controle e não pode ficar atrás do
    /// painel vizinho. `draw` desenha nas coordenadas deste controle (as de `rect`).
    pub fn overlay(&self, draw: impl FnOnce(&mut dyn Canvas)) {
        let mut rec = crate::pages::RecordingCanvas::new(None);
        draw(&mut rec);
        let at = self.ui.absolute_rect(self.id).map_or(PointF::default(), |r| r.origin());
        self.ui.overlays.borrow_mut().push((at, crate::pages::Page { ops: rec.ops }));
        self.ui.overlay_users.borrow_mut().insert(self.id);
    }
}

/// Estilo mostrado no último frame e transição em andamento de um componente.
#[derive(Clone, Debug, Default)]
pub(crate) struct Visual {
    state: Option<ControlState>,
    pub(crate) shown: Option<ResolvedStyle>,
    from: Option<ResolvedStyle>,
    start: f64,
    /// Até onde a pintura passou do controle na última vez (a sombra mais larga dos estados
    /// dele); `None` antes da primeira pintura. A repintura por parte usa ([`Ui::damage`]).
    pub(crate) reach: Option<f32>,
    /// Onde o próprio desenho dele foi parar na última pintura (sem os filhos), nas
    /// coordenadas do form ([`crate::ink`]).
    pub(crate) ink: Option<RectF>,
    /// O resumo do desenho próprio dele na última pintura ([`crate::trace`]): igual no
    /// ensaio, a mudança de estado não repinta. `None`: não resumido.
    pub(crate) drawing: Option<u64>,
    /// O deslocamento (`translate`) dele na última pintura: mudou, o lugar antigo dele e dos
    /// filhos entra na área, e o novo é o antigo andado da diferença ([`Ui::take_damage`]).
    pub(crate) shift: PointF,
    /// Muda quando o desenho de dentro dele (o próprio ou o de um descendente) muda: a chave
    /// da camada guardada (`cache_layer`).
    pub(crate) version: u64,
    /// Na última pintura, o controle cobriu o próprio retângulo com cor opaca (ele, pelo
    /// [`Component::opaque`](crate::Component::opaque), ou um filho do tamanho dele que cobre):
    /// o que fica inteiro atrás dele não é pintado nem pede quadros.
    pub(crate) covers: bool,
    /// A chave da camada guardada dele na última pintura (`begin_cached`): quando ela muda ou
    /// a camada deixa de valer, a imagem antiga é solta do renderizador (`forget_cached`), em
    /// vez de ocupar memória até sair pelo limite.
    pub(crate) layer_key: Option<u64>,
}

/// O que a pintura de um controle usa ([`Ui::look`]).
struct Look<'t> {
    enabled: bool,
    state: ControlState,
    /// O tema que passa aos filhos (o próprio do controle, se ele tem).
    inherited: &'t Theme,
    mixed: &'t Theme,
    recolored: Option<std::rc::Rc<Theme>>,
    colors: crate::component::ColorOverride,
    style: ResolvedStyle,
    transitioning: bool,
    paint_fill: bool,
    font: crate::component::FontOverride,
    reach: f32,
}

impl Look<'_> {
    /// O tema que o controle vê (com as cores próprias por cima).
    fn theme(&self) -> &Theme {
        self.recolored.as_deref().unwrap_or(self.mixed)
    }
}

impl Visual {
    /// Estilo a pintar agora: o do tema, ou um intermediário se o estado mudou há pouco e o
    /// tema define uma duração de transição. Devolve também se a transição continua.
    fn update(&mut self, state: ControlState, target: ResolvedStyle, now: f64) -> (ResolvedStyle, bool) {
        let duration = target.transition_ms as f64 / 1000.0;
        // Só há transição se o estilo do novo estado for outro (o fundo de um form, sem
        // estilo de hover, não fica repintando à toa).
        if self.state.is_some_and(|s| s != state) && duration > 0.0 && self.shown.as_ref().is_some_and(|s| *s != target) {
            if let Some(shown) = self.shown.take() {
                self.from = Some(shown);
                self.start = now;
            }
        }
        self.state = Some(state);
        let (style, running) = match &self.from {
            Some(from) if duration > 0.0 => {
                let t = ((now - self.start) / duration) as f32;
                if t >= 1.0 {
                    (target, false)
                } else {
                    (from.lerp(&target, Easing::OutQuad.apply(t)), true)
                }
            }
            _ => (target, false),
        };
        if !running {
            self.from = None;
        }
        self.shown = Some(style.clone());
        (style, running)
    }
}

impl Ui {
    /// Pinta o form `root` e todos os descendentes visíveis. Recalcula o layout antes,
    /// se for preciso.
    ///
    /// Para cada controle: posiciona o canvas na origem dele, aplica a opacidade (a do
    /// controle vezes a do tema), chama `paint` (ou o `on_paint` da instância), aplica o
    /// clip se `clip_children`, pinta os filhos e por fim chama `paint_after`.
    /// Desenha o componente (um form, um painel) numa imagem PNG de `width` x `height`
    /// (unidades lógicas, vezes `scale` em pixels), fora da tela, com o renderizador da
    /// plataforma. O componente é medido nesse tamanho antes. `None` sem o serviço.
    pub fn render_png(&mut self, id: ComponentId, width: f32, height: f32, scale: f32) -> Option<Vec<u8>> {
        let service = self.services_mut().offscreen.take()?;
        if let Some(cb) = self.control_mut(id) {
            cb.set_position(0.0, 0.0).set_size(width, height);
        }
        self.layout();
        // Como na tela: o que os `on_resize` mudaram entra num layout novo antes de pintar (a
        // foto saía da primeira passada, diferente da tela; o relatório do Tabernáculo, B02).
        for _ in 0..4 {
            if !self.needs_layout() {
                break;
            }
            self.layout();
        }
        let background = self.theme().color("background");
        let png = service.render_png(width.ceil() as u32, height.ceil() as u32, scale, background, &mut |canvas| self.paint(id, canvas));
        self.services_mut().offscreen = Some(service);
        png
    }

    /// O controle `id` (com os filhos) como está na tela, numa imagem do tamanho dele (vezes
    /// `scale` em pixels), com o fundo transparente: o desenho de um cartão, de uma linha, de
    /// um ícone para acompanhar o cursor num arrasto ([`DragVisual`](crate::DragVisual)). Não
    /// mexe no controle (nem posição, nem layout). `None` sem o serviço de desenho fora da tela.
    pub fn snapshot(&self, id: ComponentId, scale: f32) -> Option<crate::Image> {
        let service = self.services().offscreen.as_ref()?;
        let cb = self.cb(id)?;
        let (place, size) = {
            let b = cb.bounds();
            let t = cb.translate();
            (PointF::new(b.x + t.x, b.y + t.y), (b.width.ceil().max(1.0) as u32, b.height.ceil().max(1.0) as u32))
        };
        let enabled = self.ancestors_enabled(id);
        let png = service.render_png(size.0, size.1, scale, crate::Color::TRANSPARENT, &mut |canvas| {
            // `paint_node` põe o controle onde ele fica no pai: o canvas volta essa distância.
            canvas.translate(-place.x, -place.y);
            let mut classes = Vec::new();
            self.paint_node(id, canvas, enabled, &mut classes, &self.theme, &Default::default(), None);
        })?;
        Some(crate::Image::encoded(png))
    }

    /// Os pais de `id` estão todos habilitados?
    fn ancestors_enabled(&self, id: ComponentId) -> bool {
        let mut cur = self.parent(id);
        while let Some(c) = cur {
            if self.cb(c).is_some_and(|cb| !cb.enabled()) {
                return false;
            }
            cur = self.parent(c);
        }
        true
    }

    pub fn paint(&mut self, root: ComponentId, canvas: &mut dyn Canvas) {
        self.paint_region(root, canvas, None);
    }

    /// Como [`Ui::paint`], só na área `area` do form (a de [`Ui::take_damage`]): só os
    /// controles que a tocam são pintados. Quem chama recorta o canvas nela e guarda o resto
    /// da imagem do quadro anterior.
    pub fn paint_region(&mut self, root: ComponentId, canvas: &mut dyn Canvas, area: Option<RectF>) {
        match area {
            Some(a) => self.paint_regions(root, canvas, &[a]),
            None => self.paint_regions(root, canvas, &[]),
        }
    }

    /// Como [`Ui::paint_region`], com várias áreas pintadas de uma vez (as de
    /// [`Ui::take_frame`]; vazio: o form inteiro): só os controles que tocam alguma delas são
    /// pintados (60 barras espalhadas numa tela gigante não pintam tudo o que fica entre elas).
    /// Quem chama recorta o canvas na união delas.
    pub fn paint_regions(&mut self, root: ComponentId, canvas: &mut dyn Canvas, areas: &[RectF]) {
        let area = areas.iter().copied().reduce(|a, b| a.union(b));
        let _profile = crate::profile::scope(crate::profile::Lane::Paint, || format!("pintura {}", self.path_of(root)));
        self.update_layout();
        self.style_animating.set(false);
        self.wake_at.set(None);
        let mut classes = Vec::new();
        self.overlays.borrow_mut().clear();
        if area.is_none() {
            self.overlay_users.borrow_mut().clear();
            // Pintura inteira: não se sabe o que mudou desde a última, as camadas são refeitas.
            self.layer_epoch.set(self.layer_epoch.get() + 1);
        }
        // As camadas que deixaram de valer fora da pintura (a transição acabou, um controle
        // foi liberado) saem do renderizador.
        for key in self.dead_layers.borrow_mut().drain(..) {
            canvas.forget_cached(key);
        }
        // As camadas automáticas neste form: a propriedade `layer_cache` dele, ou o padrão do
        // projeto (um form sem a propriedade, como o do designer, fica com o do projeto).
        let caching = match self.get_prop(root, "layer_cache") {
            Ok(crate::PropValue::Enum(name)) => <crate::LayerCaching as crate::prop::PropEnum>::from_name(&name).unwrap_or_default(),
            _ => crate::LayerCaching::Project,
        };
        self.auto_layers.set(caching.enabled());
        // Só o que cabe no form aparece (o resto, numa caixa rolada, nem é pintado).
        let visible: Option<Vec<RectF>> = self.control(root).map(|cb| match area {
            Some(_) => areas.iter().map(|a| intersection(cb.bounds(), *a)).filter(|r| r.width > 0.0 && r.height > 0.0).collect(),
            None => vec![cb.bounds()],
        });
        let visible = visible.as_deref();
        self.spilled.set(false);
        self.paint_area.set(area);
        self.paint_count.set(0);
        let mut ink = crate::ink::InkCanvas::new(&mut *canvas, &self.ink);
        // O modo de texto do app (`layout`/`core`: o núcleo mede e desenha o texto, veja `textcore.rs`).
        if self.text_mode == crate::textcore::TextMode::Native {
            self.paint_node(root, &mut ink, true, &mut classes, &self.theme, &Default::default(), visible);
        } else {
            let mut text = crate::textcore::TextCanvas { inner: &mut ink, mode: self.text_mode };
            self.paint_node(root, &mut text, true, &mut classes, &self.theme, &Default::default(), visible);
        }
        self.flush_paint_failures();
        // Por cima de tudo (os irmãos pintados depois não cobrem): ver `PaintCtx::overlay`.
        let origin = self.absolute_rect(root).map_or(PointF::default(), |r| r.origin());
        for (at, page) in self.overlays.borrow_mut().drain(..) {
            canvas.save();
            canvas.translate(at.x - origin.x, at.y - origin.y);
            page.replay(canvas);
            canvas.restore();
        }
        self.paint_drag(root, canvas);
        self.paint_hint(root, canvas);
        self.paint_busy(root, canvas);
        self.frame_requests.borrow_mut().insert(root, (self.style_animating.get(), self.wake_at.get()));
        self.repaint_requested = false;
        // Um controle desenhou além de onde desenhava antes e fora da área: o que passou foi
        // recortado, e o próximo quadro repinta tudo.
        if area.is_some() && self.spilled.get() {
            self.request_repaint();
        }
    }

    /// Quantos controles a última pintura ([`Ui::paint_regions`]) desenhou: os que passaram do
    /// recorte das áreas. Não depende da máquina: os testes de desempenho conferem com ele.
    pub fn last_paint_count(&self) -> usize {
        self.paint_count.get()
    }

    /// Pinta só os filhos do form, sem o recorte dele (nas coordenadas do form): o designer
    /// mostra, apagado, o que foi arrastado para fora da área do form.
    pub fn paint_overflow(&mut self, root: ComponentId, canvas: &mut dyn Canvas) {
        self.update_layout();
        let Some(cb) = self.cb(root) else { return };
        let inherited = match cb.theme_name() {
            "" => &self.theme,
            name => self.theme_named(name).unwrap_or(&self.theme),
        };
        let enabled = cb.enabled();
        let size = self.form_area(root);
        let mut classes = Vec::new();
        // Só quem passa da borda: o resto o recorte apagaria de qualquer jeito (e pintar um
        // Model3D ou um vídeo duas vezes por quadro custa caro).
        let children: Vec<ComponentId> = self.children(root).iter().copied().filter(|&c| self.sticks_out(c, size)).collect();
        for child in children {
            self.paint_node(child, canvas, enabled, &mut classes, inherited, &cb.font, None);
        }
    }

    /// Algum filho do form passa da borda dele (o designer mostra, apagado, o que ficou
    /// para fora). Sem nenhum, o designer nem abre a camada.
    pub fn overflows(&self, root: ComponentId) -> bool {
        let area = self.form_area(root);
        self.children(root).iter().any(|&c| self.sticks_out(c, area))
    }

    /// A área do form nas coordenadas absolutas dos controles.
    fn form_area(&self, root: ComponentId) -> RectF {
        let origin = self.absolute_rect(root).unwrap_or_default();
        RectF::new(origin.x, origin.y, self.cb(root).map_or(0.0, |cb| cb.width()), self.cb(root).map_or(0.0, |cb| cb.height()))
    }

    /// O controle (ou algo dentro dele, se ele não recorta os filhos) sai da área do form.
    fn sticks_out(&self, id: ComponentId, area: RectF) -> bool {
        let Some(cb) = self.cb(id) else { return false };
        if !cb.visible() {
            return false;
        }
        let Some(r) = self.absolute_rect(id) else { return false };
        // A sombra e o giro também podem passar um pouco: uma folga de 1 px não conta.
        let outside = r.x < area.x - 1.0 || r.y < area.y - 1.0 || r.right() > area.right() + 1.0 || r.bottom() > area.bottom() + 1.0 || cb.rotation() != 0.0;
        outside || (!cb.clip_children() && self.children(id).iter().any(|&c| self.sticks_out(c, area)))
    }

    /// O arrasto em andamento: o contorno do destino que aceita e o rótulo junto do mouse
    /// (estilo `Hint` do tema).
    fn paint_drag(&self, root: ComponentId, canvas: &mut dyn Canvas) {
        let Some((label, at, target)) = self.drag_view(root) else { return };
        let origin = self.absolute_rect(root).map_or(PointF::default(), |r| r.origin());
        // Só o destino desta janela (o de outra é pintado lá) e que não se destaca sozinho
        // (quem recebe pelos ganchos `drop_*` mostra onde entra).
        let data = self.drag_data(root).unwrap_or_default();
        let target = target.filter(|&t| self.root_of(t) == root && !self.component(t).is_some_and(|c| c.drop_accepts(self, t, data)));
        if let Some(r) = target.and_then(|t| self.absolute_rect(t)) {
            let r = RectF::new(r.x - origin.x, r.y - origin.y, r.width, r.height);
            canvas.stroke_rect(r.inflate(1.0), 6.0, &crate::canvas::Stroke::new(self.theme.color("primary"), 2.0));
        }
        // A imagem junto do cursor, quando a plataforma não a mostra numa janela própria.
        let visual = self.drag_visual(root).map(|(v, _)| v);
        let mut label_at = PointF::new(at.x + 14.0, at.y + 14.0);
        if let Some(v) = visual {
            let r = RectF::new(at.x - v.hotspot.x, at.y - v.hotspot.y, v.size.width, v.size.height);
            if !self.native_drag_image {
                if let Some(size) = canvas.image_size(&v.image) {
                    canvas.save();
                    canvas.clip_round_rect(r, 6.0);
                    let (src, dest) = crate::ImageFit::Cover.place(size, r);
                    canvas.draw_image(&v.image, src, dest, v.opacity);
                    canvas.restore();
                }
            }
            // O rótulo vai embaixo da imagem, não por cima dela.
            label_at = PointF::new(r.x, r.bottom() + 4.0);
        }
        if label.is_empty() {
            return;
        }
        let style = self.theme.resolve(&["Hint"], "", ControlState::default());
        let mut ts = crate::canvas::TextStyle { font: style.font.clone(), color: style.text, ..Default::default() };
        ts.trimming = crate::text::Trimming::Character;
        let pad = style.padding;
        let w = canvas.measure_text(&label, &ts).width.min(320.0) + pad.left + pad.right + 1.0;
        let h = (ts.font.size * 1.4).round() + pad.top + pad.bottom;
        let rect = RectF::new(label_at.x, label_at.y, w, h);
        if let Some(shadow) = &style.shadow {
            canvas.draw_shadow(rect, style.radius, shadow);
        }
        canvas.fill_rect(rect, style.radius, &crate::canvas::Brush::Solid(style.fill.with_opacity(0.92)));
        canvas.draw_text(&label, RectF::new(rect.x + pad.left, rect.y + pad.top, w - pad.left - pad.right, h - pad.top - pad.bottom), &ts);
    }

    /// O balão da dica aberta, por cima de tudo, perto do mouse (estilo `Hint` do tema).
    fn paint_hint(&self, root: ComponentId, canvas: &mut dyn Canvas) {
        let Some((text, at)) = self.visible_hint(root) else { return };
        let Some(form) = self.control(root).map(|cb| cb.bounds()) else { return };
        let style = self.theme.resolve(&["Hint"], "", ControlState::default());
        let mut ts = crate::canvas::TextStyle { font: style.font.clone(), color: style.text, ..Default::default() };
        ts.v_align = crate::canvas::TextAlign::Leading;
        let max = 280.0f32.min(form.width - 16.0).max(40.0);
        let mut measure = |s: &str| canvas.measure_text(s, &ts).width;
        let lines = crate::text::layout_lines(&text, max, 12, true, crate::text::Trimming::Word, &mut measure);
        let width = lines.iter().map(|l| measure(l)).fold(0.0f32, f32::max);
        let lh = (ts.font.size * 1.4).round();
        let pad = style.padding;
        let w = width + pad.left + pad.right + 1.0;
        let h = lines.len() as f32 * lh + pad.top + pad.bottom;
        let mut x = at.x + 4.0;
        let mut y = at.y + 22.0;
        if x + w > form.width - 4.0 {
            x = (form.width - 4.0 - w).max(4.0);
        }
        if y + h > form.height - 4.0 {
            y = (at.y - h - 8.0).max(4.0);
        }
        let rect = RectF::new(x, y, w, h);
        if let Some(shadow) = &style.shadow {
            canvas.draw_shadow(rect, style.radius, shadow);
        }
        canvas.fill_rect(rect, style.radius, &crate::canvas::Brush::Solid(style.fill));
        // A borda do estilo `Hint` do tema (C19 do Tabernáculo); o padrão continua sem.
        if style.border_width > 0.0 && !style.border_color.is_transparent() {
            canvas.stroke_rect(rect, style.radius, &style.border_stroke(style.border_color, style.border_width));
        }
        let mut ly = y + pad.top;
        for line in &lines {
            canvas.draw_text(line, RectF::new(x + pad.left, ly, width + 1.0, lh), &ts);
            ly += lh;
        }
    }

    /// O que vale para a pintura de `id`: estado, tema e estilo (já com a transição, as cores
    /// e a fonte próprias). `peek`: só olha, sem andar a transição guardada (o ensaio de
    /// [`Ui::drawing_unchanged`]).
    #[allow(clippy::too_many_arguments)]
    fn look<'t>(
        &'t self,
        id: ComponentId,
        component: &dyn crate::Component,
        cb: &crate::ControlBase,
        visual: &std::cell::RefCell<Visual>,
        parent_enabled: bool,
        classes: &mut Vec<&'static str>,
        inherited: &'t Theme,
        parent_font: &crate::component::FontOverride,
        peek: bool,
    ) -> Look<'t> {
        let enabled = parent_enabled && cb.enabled();
        let mut state = cb.state();
        state.disabled = !enabled;
        if let Some(checked) = component.checked_state() {
            state.checked = checked;
        }
        if self.still_picture {
            state.hover = false;
            state.pressed = false;
            state.focused = false;
        }
        // O estado do primeiro ancestral que não herda (a linha da lista) vale para ele.
        if cb.inherit_state() {
            let mut up = self.parent(id);
            while let Some(p) = up {
                let Some(pcb) = self.cb(p) else { break };
                if !pcb.inherit_state() {
                    let ps = pcb.state();
                    state.hover |= ps.hover;
                    state.pressed |= ps.pressed;
                    state.checked |= self.component(p).and_then(|c| c.checked_state()).unwrap_or(ps.checked);
                    break;
                }
                up = self.parent(p);
            }
        }
        // Focado por clique ou toque: sem o anel de foco, a não ser num campo de texto (como o
        // `:focus-visible` dos navegadores); pelo teclado, o anel aparece.
        if state.focused && !component.accepts_text() && self.focus_from_pointer(self.root_of(id)) {
            state.focused = false;
        }

        classes.clear();
        component.class_chain(classes);
        // Tema próprio (vale para os filhos) e mistura por classe (só para este controle).
        let inherited = match cb.theme_name() {
            "" => inherited,
            name => self.theme_named(name).unwrap_or(inherited),
        };
        let mixed = self.mixed_theme(inherited, classes);
        // Cores próprias: o tema que o controle vê com elas por cima (guardado, para não
        // copiar o tema a cada quadro).
        let colors = cb.colors;
        let recolored = colors.recolors_theme().then(|| {
            let key = (mixed.name.clone(), colors);
            self.recolored.borrow_mut().entry(key).or_insert_with(|| std::rc::Rc::new(colors.apply_theme(mixed))).clone()
        });
        let (target, reach) = {
            let theme: &Theme = recolored.as_deref().unwrap_or(mixed);
            (theme.resolve(classes, cb.style(), state), theme.reach(classes, cb.style()).max(cb.shadow().map_or(0.0, |s| s.reach())))
        };
        // No designer o relógio não anda: sem transição, o estado final (desabilitado...) direto.
        let (mut style, transitioning) = if self.design_mode {
            (target, false)
        } else if peek {
            visual.borrow().clone().update(state, target, self.time)
        } else {
            visual.borrow_mut().update(state, target, self.time)
        };
        // A fonte do controle por cima da herdada (o `ParentFont`): vale para ele e os filhos.
        let font = if cb.font.is_empty() { parent_font.clone() } else { cb.font.over(parent_font) };
        if !font.is_empty() {
            font.apply(&mut style.font);
        }
        // A escala de fonte da vista ativa do form.
        if cb.text_scale() != 1.0 {
            style.font.size *= cb.text_scale();
        }
        // Uma fonte do projeto pelo nome (no componente ou no estilo) vira a família dela.
        if !self.font_names().is_empty() {
            style.font.family = self.resolve_font(&style.font.family);
        }
        // Nos controles sem fundo (Label, CheckBox, Gauge...), o `fill` próprio vira o fundo do
        // controle inteiro, pintado aqui (como o `Color` do Delphi).
        let paint_fill = colors.fill.is_some() && component.transparent_background();
        if let Some(fill) = colors.fill {
            // Num controle de clicar, o fundo próprio ainda reage ao mouse: clareia (fundo
            // escuro) ou escurece (fundo claro), e mais ao apertar.
            let toward = if crate::component::luminance(fill) < 0.5 { crate::Color::WHITE } else { crate::Color::BLACK };
            let amount = if !cb.can_focus() { 0.0 } else if state.pressed { 0.2 } else if state.hover || state.checked { 0.1 } else { 0.0 };
            style.fill = fill.lerp(toward.with_opacity(fill.a as f32 / 255.0), amount);
        }
        if let Some(border) = colors.border {
            style.border_color = border;
            style.border_width = style.border_width.max(1.0);
        }
        if let Some(text) = colors.text {
            style.text = text;
        }
        Look { enabled, state, inherited, mixed, recolored, colors, style, transitioning, paint_fill, font, reach }
    }

    /// O desenho próprio de `id` (sem os filhos) daria o mesmo que na última pintura: a
    /// mudança de estado dele não precisa de repintura. Só ensaia quem pinta sem efeito
    /// colateral ([`crate::Component::paint_is_pure`]) e já foi pintado; girado, espelhado
    /// ou com a pintura em erro, não arrisca.
    pub(crate) fn drawing_unchanged(&self, id: ComponentId) -> bool {
        let Some(entry) = self.entry(id) else { return false };
        let Some(component) = entry.component.as_deref() else { return false };
        let Some(cb) = component.control() else { return false };
        let Some(before) = entry.visual.borrow().drawing else { return false };
        if !component.paint_is_pure() || entry.paint_hook.is_some() || self.paint_failed(id) || self.flips_content(id) || cb.rotation() != 0.0 {
            return false;
        }
        // O que os ancestrais passam: habilitado, tema e fonte.
        let mut chain = Vec::new();
        let mut current = self.parent(id);
        while let Some(p) = current {
            chain.push(p);
            current = self.parent(p);
        }
        let (mut enabled, mut inherited, mut font) = (true, &*self.theme, crate::component::FontOverride::default());
        for &a in chain.iter().rev() {
            let Some(acb) = self.control(a) else { continue };
            enabled &= acb.enabled();
            if !acb.theme_name().is_empty() {
                inherited = self.theme_named(acb.theme_name()).unwrap_or(inherited);
            }
            if !acb.font.is_empty() {
                font = acb.font.over(&font);
            }
        }
        let mut classes = Vec::new();
        let look = self.look(id, component, cb, &entry.visual, enabled, &mut classes, inherited, &font, true);
        // Uma transição de verdade (o estilo do novo estado é outro) muda o desenho.
        if look.transitioning {
            return false;
        }
        let (requested, wake) = (Cell::new(false), Cell::new(None));
        let ctx = PaintCtx {
            ui: self,
            id,
            rect: RectF::from_size(cb.bounds().size()),
            state: look.state,
            style: &look.style,
            theme: look.theme(),
            time: self.time,
            design: self.design_mode,
            colors: look.colors,
            frame_request: &requested,
            wake_at: &wake,
            resources: self.services().resources.as_deref(),
        };
        let ghost = if self.design_mode && cb.design_hidden { 0.35 } else { 1.0 };
        let opacity = cb.opacity() * look.style.opacity * ghost;
        if opacity <= 0.0 {
            return false;
        }
        let mut hasher = std::hash::DefaultHasher::new();
        std::hash::Hash::hash(&opacity.to_bits(), &mut hasher);
        std::hash::Hash::hash(&cb.blend_mode(), &mut hasher);
        let text = self.services().text.as_deref();
        let mut trace = crate::trace::TraceCanvas::new(None, text, &mut hasher);
        let painted = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.paint_own(&mut trace, component, &ctx, look.paint_fill, cb.shadow().as_ref());
            component.paint_after(&mut trace, &ctx);
        }));
        let volatile = trace.volatile;
        painted.is_ok() && !volatile && !requested.get() && std::hash::Hasher::finish(&hasher) == before
    }

    /// O desenho do próprio controle antes dos filhos: sombra, fundo próprio, vidro e o
    /// `paint` do componente (a pintura e o ensaio passam por aqui).
    fn paint_own(&self, canvas: &mut dyn Canvas, component: &dyn crate::Component, ctx: &PaintCtx<'_>, paint_fill: bool, shadow: Option<&crate::Shadow>) {
        let style = ctx.style;
        if let Some(shadow) = shadow.or(style.shadow.as_ref()) {
            canvas.draw_shadow(ctx.rect, style.radius, shadow);
        }
        if let Some(fill) = ctx.colors.fill.filter(|_| paint_fill) {
            canvas.fill_rect(ctx.rect, style.radius, &crate::Brush::Solid(fill));
        }
        // Vidro: o que está atrás aparece desfocado (o controle pinta o fundo translúcido).
        if style.material != crate::theme::Material::None {
            canvas.backdrop_blur(ctx.rect, style.radius, style.material.blur());
        }
        component.paint(canvas, ctx);
        if style.material == crate::theme::Material::Glass {
            paint_glass_light(canvas, ctx.rect, style.radius, ctx.theme.dark);
        }
    }

    /// Como [`Ui::paint_own`], para quem não é resumido: com o gancho de pintura, espelhado
    /// ou com a pintura em erro (o retângulo de erro no lugar).
    #[allow(clippy::too_many_arguments)]
    fn paint_own_with_hook(
        &self,
        canvas: &mut dyn Canvas,
        id: ComponentId,
        entry: &crate::ui::Entry,
        component: &dyn crate::Component,
        ctx: &PaintCtx<'_>,
        paint_fill: bool,
        shadow: Option<&crate::Shadow>,
        flip: bool,
    ) {
        let style = ctx.style;
        if let Some(shadow) = shadow.or(style.shadow.as_ref()) {
            canvas.draw_shadow(ctx.rect, style.radius, shadow);
        }
        if let Some(fill) = ctx.colors.fill.filter(|_| paint_fill) {
            canvas.fill_rect(ctx.rect, style.radius, &crate::Brush::Solid(fill));
        }
        // Vidro: o que está atrás aparece desfocado (o controle pinta o fundo translúcido).
        if style.material != crate::theme::Material::None {
            canvas.backdrop_blur(ctx.rect, style.radius, style.material.blur());
        }
        if flip {
            canvas.save();
            canvas.translate(ctx.rect.width, 0.0);
        }
        {
            let mut mirror;
            let target: &mut dyn Canvas = if flip {
                mirror = Mirror { inner: &mut *canvas };
                &mut mirror
            } else {
                &mut *canvas
            };
            // Um pânico ao pintar marca o componente: daí em diante, o retângulo de erro no
            // lugar dele (uma caixa por quadro seria pior que o erro).
            if self.paint_failed(id) {
                paint_failure(target, ctx.rect);
            } else {
                let painted = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| match &entry.paint_hook {
                    Some(hook) => hook(target, ctx, &|c: &mut dyn Canvas, x: &PaintCtx<'_>| component.paint(c, x)),
                    None => component.paint(target, ctx),
                }));
                if let Err(payload) = painted {
                    self.note_paint_failure(id, payload.as_ref());
                }
            }
        }
        if flip {
            canvas.restore();
        }
        if style.material == crate::theme::Material::Glass {
            paint_glass_light(canvas, ctx.rect, style.radius, ctx.theme.dark);
        }
    }

    fn paint_node(
        &self,
        id: ComponentId,
        canvas: &mut dyn Canvas,
        parent_enabled: bool,
        classes: &mut Vec<&'static str>,
        inherited: &Theme,
        parent_font: &crate::component::FontOverride,
        visible: Option<&[RectF]>,
    ) {
        let Some(entry) = self.entry(id) else { return };
        let Some(component) = entry.component.as_deref() else { return };
        let Some(cb) = component.control() else { return };
        if !cb.visible() || (cb.design_only && !self.design_mode) {
            return;
        }
        // Fora da área visível (nas coordenadas do pai; com folga para a sombra e o anel de
        // foco): nem ele nem os filhos são pintados. Numa caixa rolada com centenas de filhos,
        // só os que aparecem custam; e um animado fora da vista não pede quadros. Quem não
        // recorta os filhos segue: um filho dele pode passar da borda e aparecer na área.
        let place = {
            let b = cb.bounds();
            let t = cb.translate();
            RectF::new(b.x + t.x, b.y + t.y, b.width, b.height)
        };
        let contained = cb.clip_children() || entry.children.is_empty();
        if contained && cb.rotation() == 0.0 && visible.is_some_and(|v| !v.iter().any(|r| place.inflate(CULL_MARGIN).intersects(*r))) {
            // Ainda assim pinta se o desenho dele, da última vez, passou da borda até a área
            // (a extensão está nas coordenadas do form; `visible`, nas do pai).
            let ink = entry.visual.borrow().ink;
            let reaches = ink.zip(self.absolute_rect(id)).zip(visible).is_some_and(|((ink, abs), v)| v.iter().any(|r| ink.offset(place.x - abs.x, place.y - abs.y).intersects(*r)));
            if !reaches {
                return;
            }
        }
        self.paint_count.set(self.paint_count.get() + 1);
        let look = self.look(id, component, cb, &entry.visual, parent_enabled, classes, inherited, parent_font, false);
        entry.visual.borrow_mut().reach = Some(look.reach);
        let (enabled, state, inherited, colors, paint_fill) = (look.enabled, look.state, look.inherited, look.colors, look.paint_fill);
        let (theme, style, font) = (look.theme(), &look.style, &look.font);
        if look.transitioning {
            self.style_animating.set(true);
            self.frame_ids.borrow_mut().push(id);
        }
        // Escondido pela propriedade no designer: aparece translúcido.
        let ghost = if self.design_mode && cb.design_hidden { 0.35 } else { 1.0 };
        let opacity = cb.opacity() * style.opacity * ghost;
        if opacity <= 0.0 {
            entry.visual.borrow_mut().drawing = None;
            return;
        }
        // Daqui em diante, o que for desenhado é deste controle (os filhos guardam e devolvem
        // a conta do pai).
        let outer_ink = self.ink.borrow_mut().take();
        let bounds = cb.bounds();
        // O quadro que ele pediu numa hora (`request_frame_at`) vale até ele se pintar de novo:
        // a pintura de agora pede o próximo, se precisar.
        {
            let mut timed = self.timed_frames.borrow_mut();
            if !timed.is_empty() {
                timed.remove(&id);
            }
        }
        // Com `scale`, o controle se desenha no espaço de dentro (o retângulo dividido pela
        // escala), que a pintura amplia.
        let scale = cb.scale();
        let ctx = PaintCtx {
            ui: self,
            id,
            rect: RectF::from_size(cb.inner_size()),
            state,
            style: &style,
            theme,
            time: self.time,
            design: self.design_mode,
            colors,
            frame_request: &self.style_animating,
            wake_at: &self.wake_at,
            resources: self.services().resources.as_deref(),
        };

        // Interface espelhada: o controle se desenha virado (a origem na borda direita).
        let flip = self.flips_content(id);
        // Camada guardada (`cache_layer`, ou o lugar ou a transparência animando num contêiner
        // que recorta os filhos): enquanto nada muda por dentro, a imagem guardada é recomposta
        // no lugar e na transparência de agora, sem pintar ele nem os filhos. Numa transição
        // (as páginas de um `TabControl`, os forms empilhados), também sem recortar os filhos:
        // por 250 ms, o que passa da borda da página fica de fora. As automáticas (sem o
        // `cache_layer` no controle) seguem o `layer_cache` do form ou o padrão do projeto.
        let automatic = self.auto_layers.get()
            && !entry.children.is_empty()
            && ((cb.clip_children() && self.animates_placement(id)) || self.in_transition(id));
        // Com mistura, o desenho depende do que está por baixo: não vira imagem guardada.
        let blend = cb.blend_mode();
        let cached = !flip && blend == crate::BlendMode::Normal && (cb.cache_layer() || automatic);

        canvas.save();
        let shift = cb.translate();
        // A camada guardada anda em pixels inteiros: a imagem é feita com a fração de pixel do
        // quadro em que foi montada e reposta em posição arredondada, e um deslocamento com
        // fração (a curva da animação) a deixaria meio pixel fora do que a pintura inteira faz.
        let drawn = if cached && shift != PointF::ZERO {
            let s = canvas.device_scale().max(f32::EPSILON);
            PointF::new((shift.x * s).round() / s, (shift.y * s).round() / s)
        } else {
            shift
        };
        canvas.translate(bounds.x + drawn.x, bounds.y + drawn.y);
        let rotation = cb.rotation();
        if rotation != 0.0 {
            canvas.translate(bounds.width / 2.0, bounds.height / 2.0);
            canvas.rotate(rotation);
            canvas.translate(-bounds.width / 2.0, -bounds.height / 2.0);
        }
        if scale != 1.0 {
            canvas.scale(scale, scale);
        }
        let layer = opacity < 1.0 || blend != crate::BlendMode::Normal;
        if blend != crate::BlendMode::Normal {
            canvas.save_layer_blend(opacity, blend, ctx.rect.inflate(look.reach + 1.0));
        } else if layer {
            canvas.save_layer(opacity);
        }

        // O desenho próprio resumido num hash, para o ensaio de `drawing_unchanged` (só de
        // quem pinta sem efeito colateral).
        let traced = component.paint_is_pure() && entry.paint_hook.is_none() && !flip && rotation == 0.0 && !self.paint_failed(id);
        let mut hasher = std::hash::DefaultHasher::new();
        std::hash::Hash::hash(&opacity.to_bits(), &mut hasher);
        std::hash::Hash::hash(&blend, &mut hasher);
        let mut volatile = false;
        let layer_key = cached.then(|| {
            let mut key = std::hash::DefaultHasher::new();
            let version = entry.visual.borrow().version;
            std::hash::Hash::hash(&(id, version, self.layer_epoch.get(), self.design_mode), &mut key);
            std::hash::Hasher::finish(&key)
        });
        // A camada de antes deixou de valer (o desenho de dentro mudou, a pintura foi inteira,
        // a transição acabou): sai do renderizador agora, e não quando passar do limite.
        let old_key = std::mem::replace(&mut entry.visual.borrow_mut().layer_key, layer_key);
        if let Some(old) = old_key.filter(|old| Some(*old) != layer_key) {
            canvas.forget_cached(old);
        }
        let cache_hit = layer_key.is_some_and(|key| canvas.begin_cached(key, ctx.rect.inflate(look.reach + 1.0)));
        if !cache_hit {
            if traced {
                let mut trace = crate::trace::TraceCanvas::new(Some(&mut *canvas), None, &mut hasher);
                let painted = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| self.paint_own(&mut trace, component, &ctx, paint_fill, cb.shadow().as_ref())));
                volatile |= trace.volatile;
                if let Err(payload) = painted {
                    self.note_paint_failure(id, payload.as_ref());
                }
            } else {
                self.paint_own_with_hook(canvas, id, entry, component, &ctx, paint_fill, cb.shadow().as_ref(), flip);
            }

            // Um form rolado (o teclado aberto) não desenha por baixo das barras do sistema.
            let scrolled_form = cb.scroll_offset().y > 0.0 && self.parent(id).is_none();
            let clip = cb.clip_children() || scrolled_form;
            if clip {
                canvas.save();
                let r = if scrolled_form {
                    let s = cb.safe_area;
                    crate::RectF::new(ctx.rect.x, ctx.rect.y + s.top, ctx.rect.width, (ctx.rect.height - s.top - s.bottom).max(0.0))
                } else {
                    ctx.rect
                };
                canvas.clip_rect(r);
            }
            // A área visível nas coordenadas deste controle (recortada por ele, se ele recorta).
            // Girado, os filhos não são descartados (a conta seria a do retângulo girado).
            // Montando a camada, os filhos todos (a imagem vale para as próximas áreas).
            let inner: Option<Vec<RectF>> = match visible.filter(|_| rotation == 0.0 && !cached) {
                Some(v) => Some(
                    v.iter()
                        .map(|r| r.offset(-place.x, -place.y))
                        // No espaço de dentro (`scale`).
                        .map(|r| if scale == 1.0 { r } else { RectF::new(r.x / scale, r.y / scale, r.width / scale, r.height / scale) })
                        .map(|local| if clip { intersection(local, ctx.rect) } else { local })
                        .filter(|r| r.width > 0.0 && r.height > 0.0)
                        .collect(),
                ),
                None if clip && rotation == 0.0 => Some(vec![ctx.rect]),
                None => None,
            };
            let hidden = self.covered_children(&entry.children, inner.as_deref());
            for (k, &child) in entry.children.iter().enumerate() {
                if hidden.get(k).copied().unwrap_or(false) {
                    continue;
                }
                self.paint_node(child, canvas, enabled, classes, inherited, &font, inner.as_deref());
            }
            if clip {
                canvas.restore();
            }
            if self.paint_failed(id) {
                // Nada por cima: o retângulo de erro já está lá.
            } else if flip {
                canvas.save();
                canvas.translate(ctx.rect.width, 0.0);
                let after = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| component.paint_after(&mut Mirror { inner: &mut *canvas }, &ctx)));
                canvas.restore();
                if let Err(payload) = after {
                    self.note_paint_failure(id, payload.as_ref());
                }
            } else if traced {
                let mut trace = crate::trace::TraceCanvas::new(Some(&mut *canvas), None, &mut hasher);
                let after = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| component.paint_after(&mut trace, &ctx)));
                volatile |= trace.volatile;
                if let Err(payload) = after {
                    self.note_paint_failure(id, payload.as_ref());
                }
            } else if let Err(payload) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| component.paint_after(canvas, &ctx))) {
                self.note_paint_failure(id, payload.as_ref());
            }
            if cached {
                canvas.end_cached();
            }
        }

        if layer {
            canvas.restore();
        }
        canvas.restore();
        let ink = std::mem::replace(&mut *self.ink.borrow_mut(), outer_ink);
        let mut visual = entry.visual.borrow_mut();
        if let (Some(new), Some(area)) = (ink, self.paint_area.get()) {
            // Desenhou fora da área e além de onde desenhava antes: o que passou ficou
            // recortado na tela.
            if !area.contains_rect(new) && !visual.ink.is_some_and(|old| old.contains_rect(new)) {
                self.spilled.set(true);
            }
        }
        visual.ink = ink;
        visual.shift = shift;
        visual.drawing = (traced && !cache_hit && !volatile && !self.paint_failed(id)).then(|| std::hash::Hasher::finish(&hasher));
        // Cobre o próprio retângulo: opaco ele mesmo, ou um filho do tamanho dele que cobre. Só
        // inteiro (sem transparência nem giro): por baixo, nada aparece.
        let whole = opacity >= 1.0 && rotation == 0.0 && !flip && !self.paint_failed(id);
        drop(visual);
        let own = component.opaque(&style) || paint_fill && colors.fill.is_some_and(|f| f.a == 255) && style.radius <= 0.0;
        let by_child = || {
            entry.children.iter().any(|&c| {
                self.entry(c).is_some_and(|e| {
                    e.visual.borrow().covers
                        && e.component.as_deref().and_then(|c| c.control()).is_some_and(|ccb| ccb.visible() && place_of(ccb).contains_rect(ctx.rect))
                })
            })
        };
        let covers = whole && (own || by_child());
        let was = std::mem::replace(&mut entry.visual.borrow_mut().covers, covers);
        if was && !covers {
            // Deixou de cobrir (ficou transparente, sumiu um filho): os irmãos de trás foram
            // pulados neste quadro pela conta antiga; o próximo repinta a área com eles.
            self.style_animating.set(true);
            self.frame_ids.borrow_mut().push(id);
        }
    }

    /// Os filhos que não precisam ser pintados: inteiros atrás de um irmão pintado depois que
    /// cobre (opaco, [`Visual::covers`]). Um medidor animando embaixo de uma tela que o cobre
    /// não pinta nem pede quadros. Quem não recorta os filhos só sai se a área inteira pintada
    /// aqui (`inner`) fica embaixo da cobertura (um filho dele pode passar da borda).
    fn covered_children(&self, children: &[ComponentId], inner: Option<&[RectF]>) -> Vec<bool> {
        let mut hidden = Vec::new();
        // As coberturas dos irmãos de cima (no máximo umas poucas, as maiores: o custo fica
        // proporcional aos filhos).
        let mut covers: Vec<RectF> = Vec::new();
        for (k, &child) in children.iter().enumerate().rev() {
            let Some(entry) = self.entry(child) else { continue };
            let Some(cb) = entry.component.as_deref().and_then(|c| c.control()) else { continue };
            if !cb.visible() {
                continue;
            }
            let place = place_of(cb);
            if !covers.is_empty() {
                let reach = entry.visual.borrow().reach.unwrap_or(CULL_MARGIN);
                let own = place.inflate(reach);
                let contained = !entry.children.is_empty() && !cb.clip_children();
                let gone = covers.iter().any(|c| {
                    c.contains_rect(own) && (!contained || inner.is_some_and(|areas| !areas.is_empty() && areas.iter().all(|a| c.contains_rect(*a))))
                });
                if gone {
                    if hidden.is_empty() {
                        hidden = vec![false; children.len()];
                    }
                    hidden[k] = true;
                    continue;
                }
            }
            if entry.visual.borrow().covers {
                covers.push(place);
                if covers.len() > 4 {
                    covers.sort_by(|a, b| (b.width * b.height).total_cmp(&(a.width * a.height)));
                    covers.truncate(4);
                }
            }
        }
        hidden
    }
}

/// O retângulo de um controle no pai, com o deslocamento (`translate`).
fn place_of(cb: &crate::component::ControlBase) -> RectF {
    let b = cb.bounds();
    let t = cb.translate();
    RectF::new(b.x + t.x, b.y + t.y, b.width, b.height)
}

/// A luz do Liquid Glass sobre o controle: brilho na metade de cima, borda clara em cima e
/// mais escura embaixo (a "espessura" do vidro).
/// O lugar de um componente que entrou em pânico ao pintar: vermelho translúcido, com um X.
fn paint_failure(canvas: &mut dyn Canvas, rect: RectF) {
    let red = crate::color::Color::rgb(220, 38, 38);
    canvas.fill_rect(rect, 0.0, &crate::canvas::Brush::Solid(red.with_opacity(0.12)));
    let stroke = crate::canvas::Stroke::new(red, 2.0);
    canvas.stroke_rect(rect.inflate(-1.0), 0.0, &stroke);
    canvas.draw_line(rect.origin(), PointF::new(rect.right(), rect.bottom()), &stroke);
    canvas.draw_line(PointF::new(rect.right(), rect.y), PointF::new(rect.x, rect.bottom()), &stroke);
}

pub fn paint_glass_light(canvas: &mut dyn Canvas, rect: crate::RectF, radius: f32, dark: bool) {
    use crate::canvas::{Brush, GradientStop, Stroke};
    use crate::{Color, PointF};
    if rect.width <= 2.0 || rect.height <= 2.0 {
        return;
    }
    let shine = if dark { 0.16 } else { 0.42 };
    canvas.fill_rect(
        rect,
        radius,
        &Brush::Linear {
            start: PointF::new(0.0, rect.y),
            end: PointF::new(0.0, rect.y + rect.height * 0.6),
            stops: vec![GradientStop::new(0.0, Color::WHITE.with_opacity(shine)), GradientStop::new(1.0, Color::WHITE.with_opacity(0.0))],
        },
    );
    let edge = Brush::Linear {
        start: PointF::new(rect.x, rect.y),
        end: PointF::new(rect.x + rect.width * 0.35, rect.bottom()),
        stops: vec![
            GradientStop::new(0.0, Color::WHITE.with_opacity(if dark { 0.45 } else { 0.95 })),
            GradientStop::new(0.55, Color::WHITE.with_opacity(if dark { 0.10 } else { 0.35 })),
            GradientStop::new(1.0, if dark { Color::WHITE.with_opacity(0.18) } else { Color::BLACK.with_opacity(0.10) }),
        ],
    };
    let r = (radius - 0.5).max(0.0);
    canvas.stroke_rect(rect.inflate(-0.5), r, &Stroke::new(edge, 1.0));
}

/// O desenho de um controle espelhado (interface da direita para a esquerda): a origem vai
/// para a borda direita e todo `x` vira `-x`. Textos e imagens não viram (ficam legíveis),
/// só trocam de lugar, e o alinhamento do começo vira o do fim.
pub(crate) struct Mirror<'a> {
    pub(crate) inner: &'a mut dyn Canvas,
}

impl Mirror<'_> {
    fn x(x: f32) -> f32 {
        -x
    }

    fn point(p: crate::PointF) -> crate::PointF {
        crate::PointF::new(-p.x, p.y)
    }

    fn rect(r: crate::RectF) -> crate::RectF {
        crate::RectF::new(-r.right(), r.y, r.width, r.height)
    }

    fn path(p: &crate::Path) -> crate::Path {
        p.map_points(Self::point)
    }

    fn brush(b: &crate::Brush) -> crate::Brush {
        use crate::Brush;
        match b {
            Brush::Solid(c) => Brush::Solid(*c),
            Brush::Linear { start, end, stops } => Brush::Linear { start: Self::point(*start), end: Self::point(*end), stops: stops.clone() },
            Brush::Radial { center, radius, stops } => Brush::Radial { center: Self::point(*center), radius: *radius, stops: stops.clone() },
            // O giro inverte: o ângulo vira o do espelho e as paradas, ao contrário.
            Brush::Sweep { center, start_angle, stops } => Brush::Sweep {
                center: Self::point(*center),
                start_angle: 180.0 - start_angle,
                stops: stops.iter().rev().map(|s| crate::GradientStop::new(1.0 - s.offset, s.color)).collect(),
            },
        }
    }

    fn stroke(s: &crate::Stroke) -> crate::Stroke {
        crate::Stroke { brush: Self::brush(&s.brush), ..s.clone() }
    }
}

impl Canvas for Mirror<'_> {
    fn save(&mut self) {
        self.inner.save();
    }
    fn restore(&mut self) {
        self.inner.restore();
    }
    fn translate(&mut self, dx: f32, dy: f32) {
        self.inner.translate(Self::x(dx), dy);
    }
    fn scale(&mut self, sx: f32, sy: f32) {
        self.inner.scale(sx, sy);
    }
    fn rotate(&mut self, degrees: f32) {
        self.inner.rotate(-degrees);
    }
    fn clip_rect(&mut self, rect: crate::RectF) {
        self.inner.clip_rect(Self::rect(rect));
    }
    fn save_layer(&mut self, opacity: f32) {
        self.inner.save_layer(opacity);
    }
    fn save_layer_blend(&mut self, opacity: f32, mode: crate::BlendMode, bounds: crate::RectF) {
        self.inner.save_layer_blend(opacity, mode, Self::rect(bounds));
    }
    fn fill_rect(&mut self, rect: crate::RectF, radius: f32, brush: &crate::Brush) {
        self.inner.fill_rect(Self::rect(rect), radius, &Self::brush(brush));
    }
    fn stroke_rect(&mut self, rect: crate::RectF, radius: f32, stroke: &crate::Stroke) {
        self.inner.stroke_rect(Self::rect(rect), radius, &Self::stroke(stroke));
    }
    fn fill_ellipse(&mut self, rect: crate::RectF, brush: &crate::Brush) {
        self.inner.fill_ellipse(Self::rect(rect), &Self::brush(brush));
    }
    fn stroke_ellipse(&mut self, rect: crate::RectF, stroke: &crate::Stroke) {
        self.inner.stroke_ellipse(Self::rect(rect), &Self::stroke(stroke));
    }
    fn draw_line(&mut self, from: crate::PointF, to: crate::PointF, stroke: &crate::Stroke) {
        self.inner.draw_line(Self::point(from), Self::point(to), &Self::stroke(stroke));
    }
    fn draw_text(&mut self, text: &str, rect: crate::RectF, style: &TextStyle) {
        use crate::TextAlign;
        let mut style = style.clone();
        style.h_align = match style.h_align {
            TextAlign::Leading => TextAlign::Trailing,
            TextAlign::Trailing => TextAlign::Leading,
            other => other,
        };
        self.inner.draw_text(text, Self::rect(rect), &style);
    }
    fn measure_text(&mut self, text: &str, style: &TextStyle) -> crate::SizeF {
        self.inner.measure_text(text, style)
    }
    fn draw_image(&mut self, image: &crate::Image, src: crate::RectF, dest: crate::RectF, opacity: f32) {
        self.inner.draw_image(image, src, Self::rect(dest), opacity);
    }
    fn image_size(&mut self, image: &crate::Image) -> Option<crate::SizeF> {
        self.inner.image_size(image)
    }
    fn clip_round_rect(&mut self, rect: crate::RectF, radius: f32) {
        self.inner.clip_round_rect(Self::rect(rect), radius);
    }
    fn fill_path(&mut self, path: &crate::Path, brush: &crate::Brush) {
        self.inner.fill_path(&Self::path(path), &Self::brush(brush));
    }
    fn clip_path(&mut self, path: &crate::Path, inverse: bool) {
        self.inner.clip_path(&Self::path(path), inverse);
    }
    fn backdrop_blur(&mut self, rect: crate::RectF, radius: f32, blur: f32) -> bool {
        self.inner.backdrop_blur(Self::rect(rect), radius, blur)
    }
    fn begin_masked(&mut self, inverse: bool) -> bool {
        self.inner.begin_masked(inverse)
    }
    fn begin_mask(&mut self) {
        self.inner.begin_mask();
    }
    fn end_mask(&mut self) {
        self.inner.end_mask();
    }
    fn stroke_path(&mut self, path: &crate::Path, stroke: &crate::Stroke) {
        self.inner.stroke_path(&Self::path(path), &Self::stroke(stroke));
    }
    fn draw_shadow(&mut self, rect: crate::RectF, radius: f32, shadow: &crate::Shadow) {
        let mut shadow = shadow.clone();
        shadow.offset.x = -shadow.offset.x;
        self.inner.draw_shadow(Self::rect(rect), radius, &shadow);
    }
    fn device_scale(&self) -> f32 {
        self.inner.device_scale()
    }
    fn gpu_device(&self) -> Option<crate::GpuDevice> {
        self.inner.gpu_device()
    }
    fn draw_gpu_surface(&mut self, dest: crate::RectF, surface: &crate::GpuSurface) -> bool {
        self.inner.draw_gpu_surface(Self::rect(dest), surface)
    }
}

/// A folga do descarte na pintura: o que fica a menos disso da área visível ainda é pintado
/// (a sombra, o anel de foco e os selos passam um pouco da borda do controle).
const CULL_MARGIN: f32 = 48.0;

/// A parte comum de dois retângulos (vazia se não se tocam).
fn intersection(a: RectF, b: RectF) -> RectF {
    let x = a.x.max(b.x);
    let y = a.y.max(b.y);
    RectF::new(x, y, (a.right().min(b.right()) - x).max(0.0), (a.bottom().min(b.bottom()) - y).max(0.0))
}
