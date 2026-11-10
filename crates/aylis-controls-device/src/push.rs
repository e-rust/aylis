//! O componente `PushNotifications`: registrar o aparelho no serviço de push do sistema e
//! receber as mensagens que o servidor do app manda. A plataforma faz o trabalho
//! ([`aylis_core::platform::PushService`]); o componente consulta num timer.

use aylis_core::platform::{NotificationPriority, PushConfig, PushEvent, PushMessage, PushService, SystemNotification};
use aylis_core::prop::PropKind;
use aylis_core::{Component, EventArgs, EventCtx, MethodInfo, PropValue, Ui, properties};

/// Notificações push: com `active`, registra quando o form abre (pede a permissão de
/// notificar onde é preciso) e o token deste aparelho chega no `on_token` (fica em `token`,
/// com o serviço em `provider`): o app manda os dois ao servidor dele, que envia por ali.
/// Uma mensagem com o app aberto chega no `on_message`; o toque na notificação (com o app
/// aberto ou fechado), no `on_opened`. Os dois deixam `title`, `body`, `data` (os dados, um
/// objeto JSON; `data_value("chave")` pega um) e `message_id`.
///
/// | Plataforma | Serviço (`provider`) | O que o projeto precisa |
/// |---|---|---|
/// | Android | `fcm` (Firebase Cloud Messaging, pelo Google Play Services, sem o SDK) | `sender_id`, `project_id`, `app_id` e `api_key` do `google-services.json` |
/// | iOS, macOS | `apns` (o token em hexadecimal) | a capacidade Push Notifications no certificado (a publicação põe o `aps-environment`) |
/// | Navegador | `webpush` (o token é a assinatura em JSON) | `vapid_key`, e a página em https (o service worker da publicação recebe) |
/// | Windows | `wns` (o token é o endereço do canal) | o app empacotado (MSIX): sem a identidade de pacote o Windows recusa |
/// | Linux | `unifiedpush` (o token é o endpoint) | um distribuidor do UnifiedPush instalado (ntfy, KUnifiedPush...) |
///
/// Com o app fechado ou em segundo plano, a mensagem com título aparece como notificação do
/// sistema; com ele aberto, só se `show_in_foreground`. Onde não houver serviço, `available`
/// é falso e o `register` responde no `on_error`.
pub struct PushNotifications {
    /// Registrar quando o form abrir.
    pub active: bool,
    /// Com o app aberto, a mensagem com título também vira notificação do sistema.
    pub show_in_foreground: bool,
    pub sender_id: String,
    pub project_id: String,
    pub app_id: String,
    pub api_key: String,
    pub vapid_key: String,
    token: String,
    provider: String,
    available: bool,
    registered: bool,
    /// Já pediu o registro (um erro não repete sozinho: o `register` tenta de novo).
    requested: bool,
    last: PushMessage,
    /// As regras mandadas à plataforma por último, e quando conferir de novo.
    rules: String,
    next_rules: f64,
}

impl Default for PushNotifications {
    fn default() -> Self {
        Self {
            active: true,
            show_in_foreground: true,
            sender_id: String::new(),
            project_id: String::new(),
            app_id: String::new(),
            api_key: String::new(),
            vapid_key: String::new(),
            token: String::new(),
            provider: String::new(),
            available: false,
            registered: false,
            requested: false,
            last: PushMessage { data: "{}".into(), ..Default::default() },
            rules: String::new(),
            next_rules: 0.0,
        }
    }
}

impl PushNotifications {
    pub fn token(&self) -> String {
        self.token.clone()
    }
    pub fn provider(&self) -> String {
        self.provider.clone()
    }
    pub fn available(&self) -> bool {
        self.available
    }
    pub fn registered(&self) -> bool {
        self.registered
    }
    pub fn title(&self) -> String {
        self.last.title.clone()
    }
    pub fn body(&self) -> String {
        self.last.body.clone()
    }
    pub fn data(&self) -> String {
        self.last.data.clone()
    }
    pub fn message_id(&self) -> String {
        self.last.id.clone()
    }
    fn set_s(&mut self, _: String) {}
    fn set_b(&mut self, _: bool) {}

    /// A última mensagem (do `on_message` ou do `on_opened`).
    pub fn message(&self) -> &PushMessage {
        &self.last
    }

    /// Um valor dos dados da última mensagem (o texto; vazio se não houver).
    pub fn data_value(&self, key: &str) -> String {
        aylis_core::platform::json_field(&self.last.data, key).unwrap_or_default()
    }

    fn config(&self) -> PushConfig {
        PushConfig {
            sender_id: self.sender_id.trim().to_string(),
            project_id: self.project_id.trim().to_string(),
            app_id: self.app_id.trim().to_string(),
            api_key: self.api_key.trim().to_string(),
            vapid_key: self.vapid_key.trim().to_string(),
            show_in_foreground: self.show_in_foreground,
        }
    }

    fn service(ui: &mut Ui) -> Result<&mut Box<dyn PushService>, String> {
        ui.services_mut().push.as_mut().ok_or_else(|| aylis_core::messages::t("as notificações push não existem aqui"))
    }

    fn register_now(&mut self, ui: &mut Ui) -> Result<(), String> {
        self.requested = true;
        let config = self.config();
        // Indisponível, o próprio serviço diz o motivo (sem o Play Services, sem pacote MSIX,
        // sem distribuidor...) no `on_error`.
        let service = Self::service(ui)?;
        self.provider = service.provider().to_string();
        service.register(&config);
        Ok(())
    }

    /// A mensagem com título vira notificação do sistema, onde a plataforma não mostra
    /// sozinha (o app desktop está sempre "aberto": vale o `show_in_foreground`).
    fn show(&self, ui: &mut Ui, m: &PushMessage) {
        let shows = ui.services().push.as_ref().is_none_or(|s| s.shows_notifications());
        if shows || !self.show_in_foreground || (m.title.is_empty() && m.body.is_empty()) {
            return;
        }
        let app_name = ui
            .main_form()
            .and_then(|f| match ui.get_prop(f, "caption") {
                Ok(PropValue::String(c)) => Some(c),
                _ => None,
            })
            .unwrap_or_else(|| "aylis".into());
        let notification = SystemNotification {
            app_id: format!("aylis.{}", app_name.to_lowercase().chars().filter(|c| c.is_ascii_alphanumeric()).collect::<String>()),
            app_name,
            title: m.title.clone(),
            body: m.body.clone(),
            // O mesmo id da mensagem atualiza a notificação em vez de criar outra.
            id: m.id.bytes().fold(0x5055_5348u32, |h, b| h.rotate_left(5) ^ u32::from(b)),
            channel: "aylis.push".into(),
            channel_name: "Push".into(),
            priority: NotificationPriority::High,
            ..Default::default()
        };
        if let Some(service) = ui.services_mut().notifications.as_mut() {
            let _ = service.notify(&notification);
        }
    }

    /// Manda à plataforma as regras dos `PushTrigger` (quando mudam), para valerem com o app
    /// fechado.
    fn sync_rules(&mut self, ui: &mut Ui) {
        if ui.services().actions.is_none() {
            return;
        }
        let rules = crate::actions::all_rules(ui);
        if rules != self.rules {
            if let Some(service) = ui.services_mut().actions.as_mut() {
                service.set_push_rules(&rules);
            }
            self.rules = rules;
        }
    }

    fn handle(&mut self, ctx: &mut EventCtx<'_>, event: PushEvent) {
        match event {
            PushEvent::Token(token) => {
                self.registered = !token.is_empty();
                self.token = token.clone();
                ctx.fire("on_token", EventArgs::Text(token));
            }
            PushEvent::Message(m) => {
                // Os `PushTrigger` ligados aqui (onde a plataforma não executou as regras ela
                // mesma, como o Android faz).
                let replaced = ctx.ui.services().actions.is_none() && crate::actions::run_triggers(ctx.ui, ctx.id, &message_data(&m));
                if !replaced {
                    self.show(ctx.ui, &m);
                }
                self.last = m;
                ctx.fire("on_message", EventArgs::Text(self.last.data.clone()));
            }
            PushEvent::Opened(m) => {
                self.last = m;
                ctx.fire("on_opened", EventArgs::Text(self.last.data.clone()));
            }
            PushEvent::Error(e) => ctx.fire("on_error", EventArgs::Text(e)),
        }
    }
}

/// Os dados da mensagem com o título, o texto e o id juntos (`title`, `body`,
/// `message_id`), para os filtros e os modelos dos `PushTrigger`. Uma chave dos dados com o
/// mesmo nome vale mais.
pub(crate) fn message_data(m: &PushMessage) -> String {
    let extra = format!(
        "\"title\":{},\"body\":{},\"message_id\":{}",
        aylis_core::platform::json_quote(&m.title),
        aylis_core::platform::json_quote(&m.body),
        aylis_core::platform::json_quote(&m.id)
    );
    let data = m.data.trim();
    match data.strip_suffix('}') {
        Some(head) if data.starts_with('{') => {
            let head = head.trim_end();
            if head == "{" { format!("{{{extra}}}") } else { format!("{head},{extra}}}") }
        }
        _ => format!("{{{extra}}}"),
    }
}

impl Component for PushNotifications {
    fn class_name(&self) -> &'static str {
        "PushNotifications"
    }

    properties! {
        active: bool => "Behavior",
        show_in_foreground: bool => "Behavior",
        sender_id: String => "Behavior",
        project_id: String => "Behavior",
        app_id: String => "Behavior",
        api_key: String => "Behavior",
        vapid_key: String => "Behavior";
        events: on_token, on_message, on_opened, on_error;
        computed: token: String => token / set_s, provider: String => provider / set_s, available: bool => available / set_b,
            registered: bool => registered / set_b, title: String => title / set_s, body: String => body / set_s,
            data: String => data / set_s, message_id: String => message_id / set_s
    }

    fn methods(&self) -> &'static [MethodInfo] {
        use PropKind::{Bool, String};
        const METHODS: &[MethodInfo] = &[
            MethodInfo::new("register", &[], Some(Bool)),
            MethodInfo::new("unregister", &[], None),
            MethodInfo::new("data_value", &[String], Some(String)).params(&["key"]),
        ];
        METHODS
    }

    fn invoke(&mut self, ctx: &mut EventCtx<'_>, method: &str, args: &[PropValue]) -> Result<PropValue, String> {
        match method {
            "register" => {
                self.register_now(ctx.ui)?;
                Ok(PropValue::Bool(true))
            }
            "unregister" => {
                if let Some(s) = ctx.ui.services_mut().push.as_mut() {
                    s.unregister();
                }
                self.registered = false;
                self.token.clear();
                Ok(PropValue::Null)
            }
            "data_value" => {
                let key = args.first().and_then(|v| v.to_str().ok()).unwrap_or_default();
                Ok(PropValue::String(self.data_value(key)))
            }
            _ => Err(format!("PushNotifications não tem o método '{method}'")),
        }
    }

    // O toque que abriu o app chega antes de o form existir: fica na fila até aqui.
    fn timer_interval(&self) -> Option<f64> {
        Some(0.25)
    }

    fn timer(&mut self, ctx: &mut EventCtx<'_>) {
        if ctx.ui.design_mode() {
            return;
        }
        self.available = ctx.ui.services().push.as_ref().is_some_and(|s| s.available());
        // As regras antes do registro (a primeira mensagem pode chegar logo); depois, a cada
        // 2 s, para valer o que o app mudou.
        if ctx.ui.time() >= self.next_rules {
            self.next_rules = ctx.ui.time() + 2.0;
            self.sync_rules(ctx.ui);
        }
        crate::actions::drain_action_events(ctx.ui);
        if self.active && !self.requested {
            if let Err(e) = self.register_now(ctx.ui) {
                ctx.fire("on_error", EventArgs::Text(e));
            }
        }
        let mut events = ctx.ui.services_mut().push.as_mut().map(|s| s.poll()).unwrap_or_default();
        events.extend(aylis_core::platform::take_push_events());
        for event in events {
            self.handle(ctx, event);
        }
    }
}
