//! As peças para montar fluxos com o push (um app de chamadas, um alarme, um pedido que
//! chega): o `PushTrigger` olha as mensagens de um `PushNotifications` e, quando os dados
//! casam com o filtro, executa uma ação; as ações são o `WakeAppAction` (abre o app por cima
//! da tela bloqueada, tocando) e o `NotifyAction` (uma notificação montada com os dados).
//! Quem liga as peças é o app.
//!
//! Com o app fechado o Rust ainda não roda: onde a plataforma tem o
//! [`ActionService`](aylis_core::platform::ActionService) (o Android), as regras ficam gravadas
//! no aparelho e ela executa a ação sozinha; o que aconteceu (o app abriu, um botão, o tempo
//! acabou) chega aos componentes quando o app roda. Sem ele, os componentes fazem o que dá
//! com o app aberto.

use std::sync::Mutex;

use aylis_core::platform::{ActionEvent, NotificationPriority, SystemNotification, data_matches, json_quote, render_template};
use aylis_core::prop::PropKind;
use aylis_core::{Component, ComponentId, EventArgs, EventCtx, MethodInfo, PropEnum, PropValue, Ui, properties};

/// O toque do `WakeAppAction`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Ringtone {
    /// O toque de chamada do aparelho.
    #[default]
    Default,
    /// O som de notificação.
    Notification,
    /// O som de alarme (toca mesmo no modo silencioso, onde o sistema deixa).
    Alarm,
    /// Sem som.
    None,
}

impl PropEnum for Ringtone {
    const NAMES: &'static [&'static str] = &["default", "notification", "alarm", "none"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Default, Self::Notification, Self::Alarm, Self::None].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Os botões no formato `id:Texto;id2:Texto 2` (sem `:`, o id é o texto).
fn parse_buttons(text: &str) -> Vec<(String, String)> {
    text.split(';')
        .filter_map(|b| {
            let (id, label) = b.split_once(':').unwrap_or((b, b));
            (!id.trim().is_empty()).then(|| (id.trim().to_string(), label.trim().to_string()))
        })
        .collect()
}

/// Os botões em JSON, com os textos ainda como modelos (a plataforma preenche).
fn buttons_json(buttons: &[(String, String)]) -> String {
    let items: Vec<String> = buttons.iter().map(|(id, label)| format!("[{},{}]", json_quote(id), json_quote(label))).collect();
    format!("[{}]", items.join(","))
}

/// Os ids que abrem o app (`*`: todos), em JSON.
fn open_json(open: &str) -> String {
    let items: Vec<String> = open.split(';').map(str::trim).filter(|s| !s.is_empty()).map(json_quote).collect();
    format!("[{}]", items.join(","))
}

/// Um número estável para a notificação de um componente (o mesmo caminho, a mesma).
fn notification_id(path: &str) -> u32 {
    path.bytes().fold(0x4952_4f4eu32, |h, b| h.rotate_left(5) ^ u32::from(b)) & 0x7fff_ffff
}

fn app_name(ui: &Ui) -> String {
    ui.main_form()
        .and_then(|f| match ui.get_prop(f, "caption") {
            Ok(PropValue::String(c)) if !c.is_empty() => Some(c),
            _ => None,
        })
        .unwrap_or_else(|| "aylis".into())
}

fn app_id(name: &str) -> String {
    format!("aylis.{}", name.to_lowercase().chars().filter(|c| c.is_ascii_alphanumeric()).collect::<String>())
}

/// Mostra o form `name` (vazio: o do próprio componente) e o traz para a frente.
fn show_form(ui: &mut Ui, owner: ComponentId, name: &str) {
    let form = if name.trim().is_empty() { Ok(ui.form_of(owner)) } else { ui.create_form(name.trim()) };
    if let Ok(form) = form {
        let _ = ui.show(form);
    }
}

/// A notificação comum (desktop, navegador, iOS): o caminho sem o `ActionService`.
fn system_notification(ui: &mut Ui, id: ComponentId, path: &str, title: String, body: String, buttons: Vec<(String, String)>, priority: NotificationPriority) {
    let name = app_name(ui);
    let n = SystemNotification {
        app_id: app_id(&name),
        app_name: name,
        title,
        body,
        id: notification_id(path),
        channel: "aylis.actions".into(),
        channel_name: aylis_core::messages::t("Avisos"),
        priority,
        actions: buttons,
        component: Some(id),
        ..Default::default()
    };
    if let Some(service) = ui.services_mut().notifications.as_mut() {
        let _ = service.notify(&n);
    }
}

// ---------------------------------------------------------------------------
// Os eventos que voltam da plataforma

/// Eventos que chegaram antes de o componente existir (o app abrindo pela ação): ficam um
/// pouco à espera do form.
static PENDING: Mutex<Vec<(ActionEvent, u32)>> = Mutex::new(Vec::new());

/// Tentativas (de 0,25 s) antes de desistir de um evento sem componente.
const PENDING_TRIES: u32 = 120;

/// Consulta a plataforma e entrega o que voltou das ações a cada componente (todos os
/// componentes daqui chamam no timer; quem chegar primeiro entrega).
pub(crate) fn drain_action_events(ui: &mut Ui) {
    let fresh = ui.services_mut().actions.as_mut().map(|s| s.poll()).unwrap_or_default();
    let mut pending = PENDING.lock().unwrap_or_else(|e| e.into_inner());
    pending.extend(fresh.into_iter().map(|e| (e, 0)));
    if pending.is_empty() {
        return;
    }
    let mut keep = Vec::new();
    for (event, tries) in pending.drain(..) {
        match ui.find_path(&event.component) {
            // Fora do timer de quem recebe: o componente pode ser o próprio que consulta.
            Some(id) => ui.poster().post(move |ui| deliver(ui, id, event)),
            None if tries < PENDING_TRIES => keep.push((event, tries + 1)),
            None => {}
        }
    }
    *pending = keep;
}

fn deliver(ui: &mut Ui, id: ComponentId, event: ActionEvent) {
    let data = event.data.clone();
    if let Some(t) = ui.get_mut::<PushTrigger>(id) {
        t.data = data.clone();
    } else if let Some(w) = ui.get_mut::<WakeAppAction>(id) {
        w.data = data.clone();
        if matches!(event.kind.as_str(), "button" | "timeout") {
            w.running = false;
        }
    } else if let Some(n) = ui.get_mut::<NotifyAction>(id) {
        n.data = data.clone();
    }
    match event.kind.as_str() {
        "trigger" => {
            ui.fire(id, "on_trigger", &EventArgs::Text(data));
        }
        "open" if ui.get::<WakeAppAction>(id).is_some() => {
            let form = ui.get::<WakeAppAction>(id).map(|w| w.form.clone()).unwrap_or_default();
            show_form(ui, id, &form);
            ui.fire(id, "on_open", &EventArgs::Text(data));
        }
        "open" | "click" => {
            ui.fire(id, "on_click", &EventArgs::Text(data));
        }
        "button" => {
            ui.fire(id, "on_button", &EventArgs::Text(event.detail));
        }
        "timeout" => {
            ui.fire(id, "on_timeout", &EventArgs::Text(data));
        }
        _ => {}
    }
}

// ---------------------------------------------------------------------------
// PushTrigger

/// Dispara uma ação quando chega uma mensagem do push que casa com o filtro.
///
/// - `source`: o `PushNotifications` de onde vêm as mensagens.
/// - `filter`: as condições sobre os dados, separadas por `;` (`kind=call`, `kind!=chat`,
///   `caller`: a chave existe); vazio aceita todas.
/// - `action`: o componente executado (qualquer um com o método `execute(dados)`: o
///   `WakeAppAction`, o `NotifyAction` ou um do app).
/// - `replace_default`: a notificação comum da mensagem não aparece (a ação cuida).
///
/// No Android a regra vale com o app fechado (fica gravada no aparelho e o próprio sistema
/// executa a ação); nas outras plataformas, com o app aberto. O `on_trigger` traz os dados
/// quando o app roda.
pub struct PushTrigger {
    pub active: bool,
    pub source: Option<ComponentId>,
    pub filter: String,
    pub action: Option<ComponentId>,
    pub replace_default: bool,
    data: String,
}

impl Default for PushTrigger {
    fn default() -> Self {
        Self { active: true, source: None, filter: String::new(), action: None, replace_default: true, data: "{}".into() }
    }
}

impl PushTrigger {
    pub fn data(&self) -> String {
        self.data.clone()
    }
    fn set_s(&mut self, _: String) {}

    /// Os dados casam com o filtro?
    pub fn matches(&self, data: &str) -> bool {
        self.active && data_matches(&self.filter, data)
    }

    /// A regra em JSON para a plataforma (sem uma ação que ela saiba executar, `None`).
    pub(crate) fn rule(ui: &Ui, id: ComponentId) -> Option<String> {
        let t = ui.get::<PushTrigger>(id)?;
        if !t.active {
            return None;
        }
        let action = t.action?;
        let spec = if let Some(w) = ui.get::<WakeAppAction>(action) {
            w.spec(&ui.path_of(action), &app_name(ui))
        } else if let Some(n) = ui.get::<NotifyAction>(action) {
            n.spec(&ui.path_of(action), "")
        } else {
            return None;
        };
        Some(format!(
            "{{\"trigger\":{},\"filter\":{},\"replace\":{},\"action\":{}}}",
            json_quote(&ui.path_of(id)),
            json_quote(&t.filter),
            t.replace_default,
            spec
        ))
    }
}

impl Component for PushTrigger {
    fn class_name(&self) -> &'static str {
        "PushTrigger"
    }

    properties! {
        active: bool => "Behavior",
        source: Option<ComponentId> => "Behavior",
        filter: String => "Behavior",
        action: Option<ComponentId> => "Behavior",
        replace_default: bool => "Behavior";
        events: on_trigger, on_error;
        computed: data: String => data / set_s
    }

    fn methods(&self) -> &'static [MethodInfo] {
        const METHODS: &[MethodInfo] = &[MethodInfo::new("matches", &[PropKind::String], Some(PropKind::Bool)).params(&["payload"])];
        METHODS
    }

    fn invoke(&mut self, _ctx: &mut EventCtx<'_>, method: &str, args: &[PropValue]) -> Result<PropValue, String> {
        match method {
            "matches" => Ok(PropValue::Bool(self.matches(args.first().and_then(|v| v.to_str().ok()).unwrap_or("{}")))),
            _ => Err(format!("PushTrigger não tem o método '{method}'")),
        }
    }

    fn timer_interval(&self) -> Option<f64> {
        Some(0.25)
    }

    fn timer(&mut self, ctx: &mut EventCtx<'_>) {
        if !ctx.ui.design_mode() {
            drain_action_events(ctx.ui);
        }
    }
}

/// Com o app aberto, onde a plataforma não executa as regras: a mensagem que chegou passa
/// pelos `PushTrigger` ligados ao `source`. Devolve se algum pediu para não mostrar a
/// notificação comum.
pub(crate) fn run_triggers(ui: &mut Ui, source: ComponentId, data: &str) -> bool {
    let triggers: Vec<(ComponentId, Option<ComponentId>, bool)> = ui
        .components()
        .filter_map(|id| ui.get::<PushTrigger>(id).filter(|t| t.source == Some(source) && t.matches(data)).map(|t| (id, t.action, t.replace_default)))
        .collect();
    let mut replace = false;
    for (id, action, replace_default) in triggers {
        replace |= replace_default;
        let data = data.to_string();
        // Depois do evento que está rodando (o `source` está ocupado com ele).
        ui.poster().post(move |ui| {
            if let Some(t) = ui.get_mut::<PushTrigger>(id) {
                t.data = data.clone();
            }
            ui.fire(id, "on_trigger", &EventArgs::Text(data.clone()));
            if let Some(action) = action {
                if let Err(e) = ui.invoke(action, "execute", &[PropValue::String(data)]) {
                    ui.fire(id, "on_error", &EventArgs::Text(e.to_string()));
                }
            }
        });
    }
    replace
}

/// As regras de todos os `PushTrigger` vivos, em JSON (para o [`ActionService`]).
pub(crate) fn all_rules(ui: &Ui) -> String {
    let rules: Vec<String> = ui.components().filter_map(|id| PushTrigger::rule(ui, id)).collect();
    format!("[{}]", rules.join(","))
}

// ---------------------------------------------------------------------------
// WakeAppAction

/// Abre o app por cima da tela bloqueada e acende a tela, tocando, como uma chamada
/// chegando: `execute(dados)` mostra a notificação de tela cheia com `title` e `text` (modelos:
/// `{caller}` vira o valor da chave nos dados) e os botões (`buttons`, `id:Texto;...`); os de
/// `open_buttons` abrem o app (`*`: todos), os outros respondem sem abrir. Com a tela
/// bloqueada ou apagada, o app abre direto no `form`.
///
/// Eventos: `on_open` (o app abriu pela ação, com os dados), `on_button` (o id do botão) e
/// `on_timeout` (passou o `timeout` sem resposta). `stop()` para o toque, tira a notificação
/// e deixa de mostrar o app por cima do bloqueio: chame ao atender ou ao encerrar.
///
/// Android: a notificação de tela cheia (`USE_FULL_SCREEN_INTENT`; no Android 14, a Play
/// Store só libera para apps de chamada e alarme), com o estilo de chamada do Android 12+
/// (`call_style`, com os botões `answer` e `reject`/`decline`) e o toque repetindo até a
/// resposta. Nas outras plataformas, com o app aberto: mostra o form, traz a janela para a
/// frente e avisa por uma notificação comum (sem tocar).
pub struct WakeAppAction {
    pub form: String,
    pub title: String,
    pub text: String,
    pub buttons: String,
    pub open_buttons: String,
    pub ringtone: Ringtone,
    pub vibrate: bool,
    /// Segundos até desistir (0: sem limite).
    pub timeout: f32,
    pub call_style: bool,
    pub turn_screen_on: bool,
    data: String,
    running: bool,
    /// Sem o `ActionService`: quando o tempo acaba (o relógio do `Ui`).
    deadline: Option<f64>,
}

impl Default for WakeAppAction {
    fn default() -> Self {
        Self {
            form: String::new(),
            title: "{title}".into(),
            text: "{body}".into(),
            buttons: String::new(),
            open_buttons: "answer".into(),
            ringtone: Ringtone::Default,
            vibrate: true,
            timeout: 30.0,
            call_style: false,
            turn_screen_on: true,
            data: "{}".into(),
            running: false,
            deadline: None,
        }
    }
}

impl WakeAppAction {
    pub fn data(&self) -> String {
        self.data.clone()
    }
    pub fn running(&self) -> bool {
        self.running
    }
    fn set_s(&mut self, _: String) {}
    fn set_b(&mut self, _: bool) {}

    /// A ação em JSON para a plataforma. Com `data` vazio, os textos vão como modelos (a
    /// plataforma preenche com a mensagem que chegar).
    pub(crate) fn spec(&self, path: &str, app_name: &str) -> String {
        format!(
            "{{\"type\":\"wake\",\"component\":{},\"app_name\":{},\"title\":{},\"text\":{},\"buttons\":{},\"open\":{},\"ringtone\":{},\"vibrate\":{},\"timeout\":{},\"call_style\":{},\"screen_on\":{}}}",
            json_quote(path),
            json_quote(app_name),
            json_quote(&self.title),
            json_quote(&self.text),
            buttons_json(&parse_buttons(&self.buttons)),
            open_json(&self.open_buttons),
            json_quote(self.ringtone.name()),
            self.vibrate,
            self.timeout.max(0.0),
            self.call_style,
            self.turn_screen_on
        )
    }

    fn execute(&mut self, ctx: &mut EventCtx<'_>, data: &str) -> Result<(), String> {
        let path = ctx.ui.path_of(ctx.id);
        self.data = data.to_string();
        self.running = true;
        if let Some(service) = ctx.ui.services_mut().actions.as_mut() {
            return service.execute(&self.spec(&path, ""), data);
        }
        // Sem a plataforma: o app está aberto; mostra o form e avisa.
        show_form(ctx.ui, ctx.id, &self.form);
        let buttons = parse_buttons(&self.buttons).into_iter().map(|(id, label)| (id, render_template(&label, data))).collect();
        system_notification(ctx.ui, ctx.id, &path, render_template(&self.title, data), render_template(&self.text, data), buttons, NotificationPriority::High);
        self.deadline = (self.timeout > 0.0).then(|| ctx.ui.time() + f64::from(self.timeout));
        ctx.fire("on_open", EventArgs::Text(data.to_string()));
        Ok(())
    }

    fn stop(&mut self, ctx: &mut EventCtx<'_>) {
        self.running = false;
        self.deadline = None;
        let path = ctx.ui.path_of(ctx.id);
        if let Some(service) = ctx.ui.services_mut().actions.as_mut() {
            service.stop(&path);
        } else if let Some(service) = ctx.ui.services_mut().notifications.as_mut() {
            let _ = service.cancel(notification_id(&path));
        }
    }
}

impl Component for WakeAppAction {
    fn class_name(&self) -> &'static str {
        "WakeAppAction"
    }

    properties! {
        form: String => "Behavior",
        title: String => "Behavior",
        text: String => "Behavior",
        buttons: String => "Behavior",
        open_buttons: String => "Behavior",
        ringtone: Ringtone => "Behavior",
        vibrate: bool => "Behavior",
        timeout: f32 => "Behavior",
        call_style: bool => "Behavior",
        turn_screen_on: bool => "Behavior";
        events: on_open, on_button, on_timeout;
        computed: data: String => data / set_s, running: bool => running / set_b
    }

    fn methods(&self) -> &'static [MethodInfo] {
        const METHODS: &[MethodInfo] = &[MethodInfo::new("execute", &[PropKind::String], Some(PropKind::Bool)), MethodInfo::new("stop", &[], None)];
        METHODS
    }

    fn invoke(&mut self, ctx: &mut EventCtx<'_>, method: &str, args: &[PropValue]) -> Result<PropValue, String> {
        match method {
            "execute" => {
                let data = args.first().and_then(|v| v.to_str().ok()).filter(|d| !d.trim().is_empty()).unwrap_or("{}").to_string();
                self.execute(ctx, &data)?;
                Ok(PropValue::Bool(true))
            }
            "stop" => {
                self.stop(ctx);
                Ok(PropValue::Null)
            }
            _ => Err(format!("WakeAppAction não tem o método '{method}'")),
        }
    }

    fn timer_interval(&self) -> Option<f64> {
        Some(0.25)
    }

    fn timer(&mut self, ctx: &mut EventCtx<'_>) {
        if ctx.ui.design_mode() {
            return;
        }
        drain_action_events(ctx.ui);
        if self.deadline.is_some_and(|d| ctx.ui.time() >= d) {
            self.stop(ctx);
            ctx.fire("on_timeout", EventArgs::Text(self.data.clone()));
        }
    }
}

// ---------------------------------------------------------------------------
// NotifyAction

/// Uma notificação montada com os dados: `execute(dados)` mostra `title`, `text` e `image`
/// (modelos: `{chave}` vira o valor nos dados; a imagem é um asset, um arquivo ou, no
/// Android, um endereço `https`), no canal e com a prioridade escolhidos, e os botões
/// (`buttons`, `id:Texto;...`; os de `open_buttons` abrem o app, `*`: todos). Eventos:
/// `on_click` (o toque no corpo, com os dados) e `on_button` (o id do botão). `stop()` tira a
/// notificação.
pub struct NotifyAction {
    pub title: String,
    pub text: String,
    pub image: String,
    pub channel: String,
    pub channel_name: String,
    pub priority: NotificationPriority,
    pub buttons: String,
    pub open_buttons: String,
    data: String,
}

impl Default for NotifyAction {
    fn default() -> Self {
        Self {
            title: "{title}".into(),
            text: "{body}".into(),
            image: String::new(),
            channel: "geral".into(),
            channel_name: "Geral".into(),
            priority: NotificationPriority::Default,
            buttons: String::new(),
            open_buttons: "*".into(),
            data: "{}".into(),
        }
    }
}

impl NotifyAction {
    pub fn data(&self) -> String {
        self.data.clone()
    }
    fn set_s(&mut self, _: String) {}

    /// A ação em JSON para a plataforma; `image_png`, a imagem já lida (base64), quando o
    /// app executa.
    pub(crate) fn spec(&self, path: &str, image_png: &str) -> String {
        let priority = match self.priority {
            NotificationPriority::Low => 2,
            NotificationPriority::Default => 3,
            NotificationPriority::High => 4,
        };
        format!(
            "{{\"type\":\"notify\",\"component\":{},\"title\":{},\"text\":{},\"image\":{},\"image_png\":{},\"channel\":{},\"channel_name\":{},\"priority\":{priority},\"buttons\":{},\"open\":{}}}",
            json_quote(path),
            json_quote(&self.title),
            json_quote(&self.text),
            json_quote(&self.image),
            json_quote(image_png),
            json_quote(if self.channel.is_empty() { "geral" } else { &self.channel }),
            json_quote(if self.channel_name.is_empty() { "Geral" } else { &self.channel_name }),
            buttons_json(&parse_buttons(&self.buttons)),
            open_json(&self.open_buttons)
        )
    }

    /// A imagem do asset ou do arquivo (o endereço `https` fica para a plataforma baixar).
    fn image_bytes(&self, ui: &Ui, data: &str) -> Vec<u8> {
        let image = render_template(&self.image, data);
        if image.is_empty() || image.starts_with("http://") || image.starts_with("https://") {
            return Vec::new();
        }
        if std::path::Path::new(&image).is_file() {
            return std::fs::read(&image).unwrap_or_default();
        }
        ui.services().resources.as_ref().and_then(|r| r.read(&image)).unwrap_or_default()
    }

    fn execute(&mut self, ctx: &mut EventCtx<'_>, data: &str) -> Result<(), String> {
        self.data = data.to_string();
        let path = ctx.ui.path_of(ctx.id);
        let png = self.image_bytes(ctx.ui, data);
        if ctx.ui.services().actions.is_some() {
            let spec = self.spec(&path, &base64(&png));
            return ctx.ui.services_mut().actions.as_mut().map_or(Ok(()), |s| s.execute(&spec, data));
        }
        let name = app_name(ctx.ui);
        let n = SystemNotification {
            app_id: app_id(&name),
            app_name: name,
            title: render_template(&self.title, data),
            body: render_template(&self.text, data),
            image_png: png,
            id: notification_id(&path),
            channel: self.channel.clone(),
            channel_name: self.channel_name.clone(),
            priority: self.priority,
            actions: parse_buttons(&self.buttons).into_iter().map(|(id, label)| (id, render_template(&label, data))).collect(),
            component: Some(ctx.id),
            ..Default::default()
        };
        match ctx.ui.services_mut().notifications.as_mut() {
            Some(service) => service.notify(&n),
            None => Err(aylis_core::messages::t("as notificações não existem aqui")),
        }
    }
}

impl Component for NotifyAction {
    fn class_name(&self) -> &'static str {
        "NotifyAction"
    }

    properties! {
        title: String => "Behavior",
        text: String => "Behavior",
        image: String => "Behavior",
        channel: String => "Behavior",
        channel_name: String => "Behavior",
        priority: NotificationPriority => "Behavior",
        buttons: String => "Behavior",
        open_buttons: String => "Behavior";
        events: on_click, on_button;
        computed: data: String => data / set_s
    }

    fn methods(&self) -> &'static [MethodInfo] {
        const METHODS: &[MethodInfo] = &[MethodInfo::new("execute", &[PropKind::String], Some(PropKind::Bool)), MethodInfo::new("stop", &[], None)];
        METHODS
    }

    fn invoke(&mut self, ctx: &mut EventCtx<'_>, method: &str, args: &[PropValue]) -> Result<PropValue, String> {
        match method {
            "execute" => {
                let data = args.first().and_then(|v| v.to_str().ok()).filter(|d| !d.trim().is_empty()).unwrap_or("{}").to_string();
                self.execute(ctx, &data)?;
                Ok(PropValue::Bool(true))
            }
            "stop" => {
                let path = ctx.ui.path_of(ctx.id);
                if let Some(service) = ctx.ui.services_mut().actions.as_mut() {
                    service.stop(&path);
                } else if let Some(service) = ctx.ui.services_mut().notifications.as_mut() {
                    let _ = service.cancel(notification_id(&path));
                }
                Ok(PropValue::Null)
            }
            _ => Err(format!("NotifyAction não tem o método '{method}'")),
        }
    }

    fn timer_interval(&self) -> Option<f64> {
        Some(0.25)
    }

    fn timer(&mut self, ctx: &mut EventCtx<'_>) {
        if !ctx.ui.design_mode() {
            drain_action_events(ctx.ui);
        }
    }
}

/// Base64 padrão (a imagem na ação para o Java).
fn base64(bytes: &[u8]) -> String {
    const T: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let b = [chunk[0], *chunk.get(1).unwrap_or(&0), *chunk.get(2).unwrap_or(&0)];
        let n = (u32::from(b[0]) << 16) | (u32::from(b[1]) << 8) | u32::from(b[2]);
        out.push(T[(n >> 18) as usize & 63] as char);
        out.push(T[(n >> 12) as usize & 63] as char);
        out.push(if chunk.len() > 1 { T[(n >> 6) as usize & 63] as char } else { '=' });
        out.push(if chunk.len() > 2 { T[n as usize & 63] as char } else { '=' });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base64_and_buttons() {
        assert_eq!(base64(b"Man"), "TWFu");
        assert_eq!(base64(b"Ma"), "TWE=");
        assert_eq!(base64(b"M"), "TQ==");
        assert_eq!(parse_buttons("answer:Atender; reject:Recusar;ok"), [("answer".into(), "Atender".into()), ("reject".into(), "Recusar".into()), ("ok".into(), "ok".into())]);
        assert_eq!(open_json("answer; *"), r#"["answer","*"]"#);
    }
}
