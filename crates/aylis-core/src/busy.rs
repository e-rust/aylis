//! O backend ocupado (itens 3.1 e 3.2 dos ajustes): cada evento que vai ao backend em outro
//! processo leva um número (`seq`), e o cliente responde `done` quando o handler termina.
//! Enquanto algum não teve `done`, o backend está ocupado:
//!
//! - os forms recusam o clique desde o primeiro instante (continuam pintando, animando,
//!   movendo e redimensionando); as teclas esperam numa fila, em ordem, e entram depois;
//! - passado o `delay` (1 s), o cursor de espera; depois de uns 3 s, um indicador discreto do
//!   aylis no canto do form. Com um `BusyWatcher`, vale o que ele declara (`action`,
//!   `busy_form`, `block_input`, `min_visible`, `on_busy_start`/`on_busy_end`);
//! - uma janela aberta pelo próprio pedido (o `MessageDlg` que espera, o `ShowModal`) põe o
//!   pedido em "esperando o usuário": a entrada volta, o relógio para e o `busy_form` some;
//!   recomeça do zero na resposta.
//!
//! Em Rust (sem backend remoto) nada disso acontece: o handler roda na thread da tela.

use crate::component::ComponentId;
use crate::event::EventArgs;
use crate::prop::PropValue;
use crate::ui::Ui;

/// Depois disso sem `done`, o indicador padrão (sem `BusyWatcher`), em segundos.
const INDICATOR_AFTER: f64 = 3.0;

#[derive(Default)]
pub(crate) struct Busy {
    /// Os eventos sem `done`: o número e o form de quem disparou.
    outstanding: Vec<(u64, ComponentId)>,
    /// Desde quando está ocupado (`None`: livre ou esperando o usuário).
    since: Option<f64>,
    /// Janelas abertas pelo pedido que esperam o usuário.
    waiting_user: usize,
    /// O `on_busy_start` já rodou (e o `busy_form` está à mostra, se há um), e quando.
    shown: Option<f64>,
    /// O `busy_form` que o `BusyWatcher` abriu.
    busy_form: Option<ComponentId>,
    /// O que a tela mostra agora (para repintar só na troca).
    stage: Stage,
    /// Quem quer saber dos forms liberados (o protocolo remoto avisa o backend).
    pub(crate) freed_hook: Option<std::rc::Rc<dyn Fn(&mut Ui, &[ComponentId])>>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum Stage {
    #[default]
    Idle,
    Cursor,
    Indicator,
}

/// O que o `BusyWatcher` declara (ou o padrão, sem ele).
struct Watch {
    id: Option<ComponentId>,
    delay: f64,
    action: String,
    busy_form: Option<ComponentId>,
    block_input: bool,
    min_visible: f64,
}

impl Ui {
    fn watch(&self) -> Watch {
        let id = self.components().find(|&c| self.class_name(c) == Some("BusyWatcher"));
        let num = |p: &str, d: f64| match id.map(|i| self.get_prop(i, p)) {
            Some(Ok(PropValue::Int(n))) => n as f64 / 1000.0,
            Some(Ok(PropValue::Float(n))) => n / 1000.0,
            _ => d,
        };
        let action = match id.map(|i| self.get_prop(i, "action")) {
            Some(Ok(PropValue::Enum(a))) => a,
            _ => "default".into(),
        };
        let busy_form = match id.map(|i| self.get_prop(i, "busy_form")) {
            Some(Ok(PropValue::Component(c))) => c,
            _ => None,
        };
        let block_input = !matches!(id.map(|i| self.get_prop(i, "block_input")), Some(Ok(PropValue::Bool(false))));
        Watch { id, delay: num("delay", 1.0), action, busy_form, block_input, min_visible: num("min_visible", 0.3) }
    }

    /// O evento `seq` de `sender` foi ao backend (o `aylis-remote` chama).
    pub fn backend_event_sent(&mut self, seq: u64, sender: ComponentId) {
        let form = self.form_of(sender);
        self.busy.outstanding.push((seq, form));
        if self.busy.since.is_none() && self.busy.waiting_user == 0 {
            self.busy.since = Some(self.time);
        }
    }

    /// O backend terminou o handler do evento `seq` (`done`).
    pub fn backend_done(&mut self, seq: u64) {
        self.busy.outstanding.retain(|(s, _)| *s != seq);
        if !self.busy.outstanding.is_empty() {
            return;
        }
        self.busy.since = None;
        self.end_busy(false);
        // As teclas que esperavam entram agora, em ordem.
        let roots: Vec<ComponentId> = self.input.keys().copied().collect();
        for root in roots {
            self.release_held(root);
        }
        self.request_repaint();
    }

    /// O backend está ocupado (um evento sem `done`, fora de uma janela que espera o usuário).
    pub fn backend_busy(&self) -> bool {
        !self.busy.outstanding.is_empty() && self.busy.waiting_user == 0
    }

    /// A entrada do usuário está recusada porque o backend está ocupado.
    pub(crate) fn busy_blocks_input(&self) -> bool {
        self.backend_busy() && self.watch().block_input
    }

    /// Uma janela aberta pelo pedido (`MessageDlg`, `ShowModal`) espera o usuário (`true`) ou
    /// fechou (`false`): enquanto espera, a entrada volta e o relógio para; depois, recomeça.
    pub fn backend_waiting_user(&mut self, waiting: bool) {
        if waiting {
            self.busy.waiting_user += 1;
            self.busy.since = None;
            self.end_busy(true);
        } else {
            self.busy.waiting_user = self.busy.waiting_user.saturating_sub(1);
            if self.busy.waiting_user == 0 && !self.busy.outstanding.is_empty() {
                self.busy.since = Some(self.time);
            }
        }
        self.request_repaint();
    }

    /// Esconde o que o ocupado mostrou (o `busy_form`, depois do `min_visible`) e avisa o
    /// `on_busy_end`. `now`: sem esperar o `min_visible` (a janela do usuário abriu por cima).
    fn end_busy(&mut self, now: bool) {
        let Some(shown) = self.busy.shown else {
            self.busy.stage = Stage::Idle;
            return;
        };
        let watch = self.watch();
        if !now && self.busy.busy_form.is_some() && self.time - shown < watch.min_visible {
            // Some depois: o `tick` volta aqui quando der o tempo.
            return;
        }
        self.busy.shown = None;
        self.busy.stage = Stage::Idle;
        if let Some(form) = self.busy.busy_form.take() {
            if self.contains(form) {
                self.close(form);
            }
        }
        if let Some(id) = watch.id {
            self.fire(id, "on_busy_end", &EventArgs::None);
        }
    }

    /// A cada quadro: passou do `delay`, mostra o ocupado; o `busy_form` que esperava o
    /// `min_visible` some.
    pub(crate) fn tick_busy(&mut self) {
        if self.busy.since.is_none() {
            if self.busy.shown.is_some() && self.busy.outstanding.is_empty() {
                self.end_busy(false);
            }
            return;
        }
        let Some(since) = self.busy.since else { return };
        let watch = self.watch();
        let elapsed = self.time - since;
        let stage = if elapsed >= watch.delay.max(INDICATOR_AFTER) && watch.action == "default" {
            Stage::Indicator
        } else if elapsed >= watch.delay {
            Stage::Cursor
        } else {
            Stage::Idle
        };
        if stage != Stage::Idle && self.busy.shown.is_none() {
            self.busy.shown = Some(self.time);
            let mut skip_form = false;
            if let Some(id) = watch.id {
                // `on_busy_start` com `set_handled`: sem o `busy_form`.
                skip_form = self.ask(id, "on_busy_start", &EventArgs::None) == crate::question::Asked::Answered(crate::question::Answer::Refused(String::new()));
            }
            if watch.action == "form" && !skip_form {
                if let Some(form) = watch.busy_form.filter(|&f| self.contains(f)) {
                    let _ = self.show(form);
                    self.busy.busy_form = Some(form);
                }
            }
        }
        if stage != self.busy.stage {
            self.busy.stage = stage;
            self.request_repaint();
        }
        // Acorda na próxima troca (o cursor, o indicador).
        let next = if stage == Stage::Idle { since + watch.delay } else { since + watch.delay.max(INDICATOR_AFTER) };
        if next > self.time {
            let at = self.wake_at.get().map_or(next, |t| t.min(next));
            self.wake_at.set(Some(at));
        }
    }

    /// O cursor de espera, enquanto o ocupado está à mostra.
    pub(crate) fn busy_cursor(&self) -> bool {
        self.busy.since.is_some() && self.busy.stage != Stage::Idle && self.watch().action != "none"
    }

    /// O indicador padrão: um aviso pequeno no canto de baixo do form.
    pub(crate) fn paint_busy(&self, root: ComponentId, canvas: &mut dyn crate::canvas::Canvas) {
        if self.busy.stage != Stage::Indicator {
            return;
        }
        let Some(form) = self.control(root).map(|cb| cb.bounds()) else { return };
        let style = self.theme.resolve(&["Hint"], "", crate::component::ControlState::default());
        let ts = crate::canvas::TextStyle { font: style.font.clone(), color: style.text, ..Default::default() };
        let text = crate::messages::t("Aguardando o programa…");
        let pad = style.padding;
        let w = canvas.measure_text(&text, &ts).width + pad.left + pad.right + 1.0;
        let h = (ts.font.size * 1.4).round() + pad.top + pad.bottom;
        let rect = crate::geometry::RectF::new(form.width - w - 12.0, form.height - h - 12.0, w, h);
        if let Some(shadow) = &style.shadow {
            canvas.draw_shadow(rect, style.radius, shadow);
        }
        canvas.fill_rect(rect, style.radius, &crate::canvas::Brush::Solid(style.fill.with_opacity(0.92)));
        canvas.draw_text(&text, crate::geometry::RectF::new(rect.x + pad.left, rect.y + pad.top, w - pad.left - pad.right, h - pad.top - pad.bottom), &ts);
    }

    /// Solta as teclas que esperavam o backend (veja `keyflow.rs`).
    fn release_held(&mut self, root: ComponentId) {
        crate::keyflow::release_after_busy(self, root);
    }
}

impl Ui {
    /// Avisa `f` dos forms liberados (o protocolo remoto conta ao backend, que descarta as
    /// instâncias e os `PostTo` com eles como dono).
    pub fn on_forms_freed(&mut self, f: impl Fn(&mut Ui, &[ComponentId]) + 'static) {
        self.busy.freed_hook = Some(std::rc::Rc::new(f));
    }
}
