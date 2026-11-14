//! Os sensores do aparelho (categoria "Device", não visuais): [`Location`], [`Battery`],
//! [`Network`], [`MotionSensor`], [`Compass`], [`ProximitySensor`], [`LightSensor`] e
//! [`Vibration`]; e o controle de jogo, [`Gamepad`]. Quem mede é a plataforma (`Services::sensors`); o componente lê a última
//! medida no seu timer (`interval`, em segundos) e dispara `on_change` quando ela muda.
//! `available` é falso onde o aparelho não tem o sensor (os valores ficam em zero).
//!
//! | Plataforma | O que tem |
//! |---|---|
//! | Android | todos (a localização com a permissão `location`) |
//! | Windows | bateria, rede e localização (a do Windows, com a permissão dele) |
//! | Linux | bateria e rede (`/sys`) |
//! | macOS, iOS, navegador | ainda não (`available` falso) |

use aylis_core::platform::Sensor;
use aylis_core::prop::PropKind;
use aylis_core::{Component, EventArgs, EventCtx, MethodInfo, PropValue, Ui, properties};

/// Lê o sensor; guarda e devolve `true` se mudou.
fn poll(ui: &mut Ui, sensor: Sensor, last: &mut Vec<f64>, available: &mut bool) -> bool {
    let Some(service) = ui.services_mut().sensors.as_mut() else {
        *available = false;
        return false;
    };
    *available = service.available(sensor);
    let Some(now) = service.read(sensor) else { return false };
    let changed = now.len() != last.len() || now.iter().zip(last.iter()).any(|(a, b)| (a - b).abs() > 1e-9);
    if changed {
        *last = now;
    }
    changed
}

fn set_active(ui: &mut Ui, sensor: Sensor, on: bool) {
    if let Some(service) = ui.services_mut().sensors.as_mut() {
        service.set_active(sensor, on);
    }
}

/// O timer e o ligar/desligar, iguais em todos os sensores. `$always`: mede sem precisar de
/// `active` (bateria e rede).
macro_rules! sensor_common {
    ($sensor:expr, $always:expr) => {
        fn timer_interval(&self) -> Option<f64> {
            ($always || self.active).then_some(f64::from(self.interval.max(0.05)))
        }

        fn timer(&mut self, ctx: &mut EventCtx<'_>) {
            if ctx.ui.design_mode() {
                return;
            }
            if !self.started {
                self.started = true;
                set_active(ctx.ui, $sensor, true);
            }
            if poll(ctx.ui, $sensor, &mut self.values, &mut self.available) {
                ctx.fire("on_change", EventArgs::None);
            }
        }

        fn property_changed(&mut self, ctx: &mut EventCtx<'_>, name: &str) {
            if name == "active" && !self.active && self.started {
                self.started = false;
                set_active(ctx.ui, $sensor, false);
            }
        }
    };
}

fn at(values: &[f64], i: usize) -> f64 {
    values.get(i).copied().unwrap_or(0.0)
}

/// A localização (GPS, rede): latitude, longitude, precisão (m), altitude e velocidade (m/s).
/// Liga com `active`; a primeira vez pede a permissão (no Android, declare `location` no
/// `Permissions` ou ela é declarada pela publicação ao achar este componente).
pub struct Location {
    /// Mede a localização (a primeira vez pede a permissão); desligado, para.
    pub active: bool,
    /// De quanto em quanto tempo mede, em segundos.
    pub interval: f32,
    values: Vec<f64>,
    available: bool,
    started: bool,
}

impl Default for Location {
    fn default() -> Self {
        Self { active: false, interval: 1.0, values: Vec::new(), available: false, started: false }
    }
}

impl Location {
    pub fn latitude(&self) -> f64 {
        at(&self.values, 0)
    }
    pub fn longitude(&self) -> f64 {
        at(&self.values, 1)
    }
    pub fn accuracy(&self) -> f64 {
        at(&self.values, 2)
    }
    pub fn altitude(&self) -> f64 {
        at(&self.values, 3)
    }
    pub fn speed(&self) -> f64 {
        at(&self.values, 4)
    }
    pub fn available(&self) -> bool {
        self.available
    }
    fn set_f(&mut self, _: f64) {}
    fn set_b(&mut self, _: bool) {}
}

impl Component for Location {
    fn class_name(&self) -> &'static str {
        "Location"
    }

    properties! {
        active: bool => "Behavior",
        interval: f32 => "Behavior";
        events: on_change;
        computed: latitude: f64 => latitude / set_f, longitude: f64 => longitude / set_f, accuracy: f64 => accuracy / set_f,
            altitude: f64 => altitude / set_f, speed: f64 => speed / set_f, available: bool => available / set_b
    }

    sensor_common!(Sensor::Location, false);
}

/// A bateria: `level` (0 a 100) e `charging`. Sempre medindo (a cada `interval`, 5 s).
pub struct Battery {
    /// De quanto em quanto tempo lê a bateria, em segundos.
    pub interval: f32,
    values: Vec<f64>,
    available: bool,
    started: bool,
    active: bool,
}

impl Default for Battery {
    fn default() -> Self {
        Self { interval: 5.0, values: Vec::new(), available: false, started: false, active: true }
    }
}

impl Battery {
    pub fn level(&self) -> f64 {
        (at(&self.values, 0) * 100.0).round()
    }
    pub fn charging(&self) -> bool {
        at(&self.values, 1) > 0.5
    }
    pub fn available(&self) -> bool {
        self.available
    }
    fn set_f(&mut self, _: f64) {}
    fn set_b(&mut self, _: bool) {}
}

impl Component for Battery {
    fn class_name(&self) -> &'static str {
        "Battery"
    }

    properties! {
        interval: f32 => "Behavior";
        events: on_change;
        computed: level: f64 => level / set_f, charging: bool => charging / set_b, available: bool => available / set_b
    }

    sensor_common!(Sensor::Battery, true);
}

/// A rede: `connected` e `kind` (`wifi`, `cellular`, `ethernet`, `other` ou `none`).
pub struct Network {
    /// De quanto em quanto tempo confere a rede, em segundos.
    pub interval: f32,
    values: Vec<f64>,
    available: bool,
    started: bool,
    active: bool,
}

impl Default for Network {
    fn default() -> Self {
        Self { interval: 2.0, values: Vec::new(), available: false, started: false, active: true }
    }
}

impl Network {
    pub fn connected(&self) -> bool {
        at(&self.values, 0) > 0.5
    }
    pub fn kind(&self) -> String {
        match at(&self.values, 1) as i64 {
            1 => "wifi",
            2 => "cellular",
            3 => "ethernet",
            4 => "other",
            _ => "none",
        }
        .into()
    }
    pub fn available(&self) -> bool {
        self.available
    }
    fn set_s(&mut self, _: String) {}
    fn set_b(&mut self, _: bool) {}
}

impl Component for Network {
    fn class_name(&self) -> &'static str {
        "Network"
    }

    properties! {
        interval: f32 => "Behavior";
        events: on_change;
        computed: connected: bool => connected / set_b, kind: String => kind / set_s, available: bool => available / set_b
    }

    sensor_common!(Sensor::Network, true);
}

/// O acelerômetro (com a gravidade), em m/s²: `x`, `y` e `z`.
pub struct MotionSensor {
    /// Lê o acelerômetro; desligado, para.
    pub active: bool,
    /// De quanto em quanto tempo lê, em segundos.
    pub interval: f32,
    values: Vec<f64>,
    available: bool,
    started: bool,
}

impl Default for MotionSensor {
    fn default() -> Self {
        Self { active: false, interval: 0.1, values: Vec::new(), available: false, started: false }
    }
}

impl MotionSensor {
    pub fn x(&self) -> f64 {
        at(&self.values, 0)
    }
    pub fn y(&self) -> f64 {
        at(&self.values, 1)
    }
    pub fn z(&self) -> f64 {
        at(&self.values, 2)
    }
    pub fn available(&self) -> bool {
        self.available
    }
    fn set_f(&mut self, _: f64) {}
    fn set_b(&mut self, _: bool) {}
}

impl Component for MotionSensor {
    fn class_name(&self) -> &'static str {
        "MotionSensor"
    }

    properties! {
        active: bool => "Behavior",
        interval: f32 => "Behavior";
        events: on_change;
        computed: x: f64 => x / set_f, y: f64 => y / set_f, z: f64 => z / set_f, available: bool => available / set_b
    }

    sensor_common!(Sensor::Motion, false);
}

/// A bússola: `heading` em graus (0 = norte, 90 = leste).
pub struct Compass {
    /// Lê a bússola; desligado, para.
    pub active: bool,
    /// De quanto em quanto tempo lê, em segundos.
    pub interval: f32,
    values: Vec<f64>,
    available: bool,
    started: bool,
}

impl Default for Compass {
    fn default() -> Self {
        Self { active: false, interval: 0.2, values: Vec::new(), available: false, started: false }
    }
}

impl Compass {
    pub fn heading(&self) -> f64 {
        at(&self.values, 0)
    }
    pub fn available(&self) -> bool {
        self.available
    }
    fn set_f(&mut self, _: f64) {}
    fn set_b(&mut self, _: bool) {}
}

impl Component for Compass {
    fn class_name(&self) -> &'static str {
        "Compass"
    }

    properties! {
        active: bool => "Behavior",
        interval: f32 => "Behavior";
        events: on_change;
        computed: heading: f64 => heading / set_f, available: bool => available / set_b
    }

    sensor_common!(Sensor::Compass, false);
}

/// O sensor de proximidade (o rosto encostado no telefone): `near` e `distance` (cm).
pub struct ProximitySensor {
    /// Lê o sensor de proximidade; desligado, para.
    pub active: bool,
    /// De quanto em quanto tempo lê, em segundos.
    pub interval: f32,
    values: Vec<f64>,
    available: bool,
    started: bool,
}

impl Default for ProximitySensor {
    fn default() -> Self {
        Self { active: false, interval: 0.2, values: Vec::new(), available: false, started: false }
    }
}

impl ProximitySensor {
    pub fn near(&self) -> bool {
        at(&self.values, 1) > 0.5
    }
    pub fn distance(&self) -> f64 {
        at(&self.values, 0)
    }
    pub fn available(&self) -> bool {
        self.available
    }
    fn set_f(&mut self, _: f64) {}
    fn set_b(&mut self, _: bool) {}
}

impl Component for ProximitySensor {
    fn class_name(&self) -> &'static str {
        "ProximitySensor"
    }

    properties! {
        active: bool => "Behavior",
        interval: f32 => "Behavior";
        events: on_change;
        computed: near: bool => near / set_b, distance: f64 => distance / set_f, available: bool => available / set_b
    }

    sensor_common!(Sensor::Proximity, false);
}

/// O sensor de luz ambiente: `lux`.
pub struct LightSensor {
    /// Lê a luz ambiente; desligado, para.
    pub active: bool,
    /// De quanto em quanto tempo lê, em segundos.
    pub interval: f32,
    values: Vec<f64>,
    available: bool,
    started: bool,
}

impl Default for LightSensor {
    fn default() -> Self {
        Self { active: false, interval: 0.5, values: Vec::new(), available: false, started: false }
    }
}

impl LightSensor {
    pub fn lux(&self) -> f64 {
        at(&self.values, 0)
    }
    pub fn available(&self) -> bool {
        self.available
    }
    fn set_f(&mut self, _: f64) {}
    fn set_b(&mut self, _: bool) {}
}

impl Component for LightSensor {
    fn class_name(&self) -> &'static str {
        "LightSensor"
    }

    properties! {
        active: bool => "Behavior",
        interval: f32 => "Behavior";
        events: on_change;
        computed: lux: f64 => lux / set_f, available: bool => available / set_b
    }

    sensor_common!(Sensor::Light, false);
}

/// Vibra o aparelho: `vibrate(ms)` (no Android, a publicação declara a permissão `VIBRATE`).
#[derive(Default)]
pub struct Vibration;

impl Vibration {
    /// Vibra por `ms` milissegundos; `false` onde não há vibração.
    pub fn vibrate(ui: &mut Ui, ms: u32) -> bool {
        ui.services_mut().sensors.as_mut().is_some_and(|s| s.vibrate(ms))
    }
}

// Sem propriedades, só o método: o `get_prop` gerado não tem casos.
#[allow(unreachable_code)]
impl Component for Vibration {
    fn class_name(&self) -> &'static str {
        "Vibration"
    }

    properties! {
        ;
        events: on_error
    }

    fn methods(&self) -> &'static [MethodInfo] {
        const METHODS: &[MethodInfo] = &[MethodInfo::new("vibrate", &[PropKind::Int], Some(PropKind::Bool)).params(&["ms"])];
        METHODS
    }

    fn invoke(&mut self, ctx: &mut EventCtx<'_>, method: &str, args: &[PropValue]) -> Result<PropValue, String> {
        match method {
            "vibrate" => {
                let ms = args.first().and_then(|v| v.to_i64().ok()).unwrap_or(200).clamp(1, 10_000) as u32;
                Ok(PropValue::Bool(Vibration::vibrate(ctx.ui, ms)))
            }
            _ => Err(format!("Vibration não tem o método '{method}'")),
        }
    }
}

/// O serviço em segundo plano: a lógica sem tela do projeto (`src/background.rs`, com
/// `pub fn run(service: aylis::background::Service)`) roda numa thread que continua com a tela
/// fechada; no Android, num serviço em primeiro plano com a notificação fixa (`title`,
/// `text`) do tipo `kind` (o que o Android 14+ exige: `dataSync`, `location`,
/// `mediaPlayback`, `connectedDevice`...). Com `keep_alive`, o sistema reinicia o serviço (e no
/// boot) e o app pede para sair da otimização de bateria; alguns fabricantes têm listas
/// próprias (veja `docs/segundo-plano.md`). `send` manda texto à lógica; o que ela manda chega
/// em `on_message` (guardado enquanto a tela estava fechada). No iOS não há serviço assim: a
/// lógica roda com o app aberto.
pub struct BackgroundService {
    /// Liga o serviço e a lógica do projeto; desligado, para os dois.
    pub active: bool,
    /// O título da notificação fixa do serviço; vazio: "Rodando em segundo plano".
    pub title: String,
    /// O texto da notificação fixa do serviço.
    pub text: String,
    /// O tipo do serviço no Android 14+ (`dataSync`, `location`, `mediaPlayback`,
    /// `connectedDevice`...).
    pub kind: String,
    /// O sistema reinicia o serviço (e no boot) e o app pede para sair da otimização de bateria.
    pub keep_alive: bool,
    started: bool,
}

impl Default for BackgroundService {
    fn default() -> Self {
        Self { active: false, title: String::new(), text: String::new(), kind: "dataSync".into(), keep_alive: false, started: false }
    }
}

impl BackgroundService {
    /// Liga: o serviço do sistema (onde houver) e a lógica do projeto.
    fn begin(&mut self, ctx: &mut EventCtx<'_>) {
        self.started = true;
        let title = if self.title.is_empty() { aylis_core::messages::t("Rodando em segundo plano") } else { self.title.clone() };
        let (text, kind, keep) = (self.text.clone(), self.kind.clone(), self.keep_alive);
        if let Some(host) = ctx.ui.services_mut().background.as_mut() {
            if let Err(e) = host.start(&title, &text, &kind, keep) {
                ctx.fire("on_error", EventArgs::Text(e));
            }
        }
        if !aylis_core::background::start(None) {
            ctx.fire("on_error", EventArgs::Text(aylis_core::messages::t("o projeto não tem a unidade background (src/background.rs)")));
        }
    }
}

impl Component for BackgroundService {
    fn class_name(&self) -> &'static str {
        "BackgroundService"
    }

    properties! {
        active: bool => "Behavior",
        title: String => "Behavior",
        text: String => "Behavior",
        kind: String => "Behavior",
        keep_alive: bool => "Behavior";
        events: on_message, on_error
    }

    fn methods(&self) -> &'static [MethodInfo] {
        const METHODS: &[MethodInfo] = &[MethodInfo::new("send", &[PropKind::String], None), MethodInfo::new("running", &[], Some(PropKind::Bool))];
        METHODS
    }

    fn invoke(&mut self, _ctx: &mut EventCtx<'_>, method: &str, args: &[PropValue]) -> Result<PropValue, String> {
        match method {
            "send" => {
                aylis_core::background::send_to_service(args.first().and_then(|v| v.to_str().ok()).unwrap_or_default());
                Ok(PropValue::Null)
            }
            "running" => Ok(PropValue::Bool(aylis_core::background::running())),
            _ => Err(format!("BackgroundService não tem o método '{method}'")),
        }
    }

    fn timer_interval(&self) -> Option<f64> {
        Some(0.25)
    }

    fn timer(&mut self, ctx: &mut EventCtx<'_>) {
        if ctx.ui.design_mode() {
            return;
        }
        if self.active && !self.started {
            self.begin(ctx);
        }
        for message in aylis_core::background::take_messages() {
            ctx.fire("on_message", EventArgs::Text(message));
        }
    }

    fn property_changed(&mut self, ctx: &mut EventCtx<'_>, name: &str) {
        if name == "active" && !self.active && self.started {
            self.started = false;
            aylis_core::background::stop();
            if let Some(host) = ctx.ui.services_mut().background.as_mut() {
                host.stop();
            }
        }
    }
}

/// O telefone: `dial` (abre o discador com o número), `call` (liga direto; com
/// `direct_call`, a publicação declara `CALL_PHONE`) e o estado das chamadas (`state`:
/// `idle`, `ringing`, `dialing`, `active`, `holding`; `number`), com `watch_state` (declara
/// `READ_PHONE_STATE` e `READ_CALL_LOG`, para o número). `on_call_state` a cada mudança.
///
/// `dialer_app`: o modo "app de telefone" (o app vira o discador padrão com
/// `request_dialer`, recebe as chamadas e atende, recusa e desliga; `history` lista as
/// últimas). A Play Store só aceita essas permissões em apps de telefone. Hoje, o Android.
#[derive(Default)]
pub struct Phone {
    /// Acompanha o estado das chamadas (`state`, `number` e o `on_call_state`); a publicação
    /// declara `READ_PHONE_STATE` e `READ_CALL_LOG`.
    pub watch_state: bool,
    /// O `call` liga direto, sem passar pelo discador; a publicação declara `CALL_PHONE`.
    pub direct_call: bool,
    /// O modo "app de telefone": o app pode virar o discador padrão, receber as chamadas e
    /// atender, recusar e desligar.
    pub dialer_app: bool,
    state: String,
    number: String,
    watching: bool,
}

impl Phone {
    pub fn state(&self) -> String {
        if self.state.is_empty() { "idle".into() } else { self.state.clone() }
    }
    pub fn number(&self) -> String {
        self.number.clone()
    }
    fn set_s(&mut self, _: String) {}

    fn service(ui: &mut Ui) -> Result<&mut Box<dyn aylis_core::platform::PhoneService>, String> {
        ui.services_mut().phone.as_mut().ok_or_else(|| aylis_core::messages::t("este aparelho não faz ligações"))
    }
}

impl Component for Phone {
    fn class_name(&self) -> &'static str {
        "Phone"
    }

    properties! {
        watch_state: bool => "Behavior",
        direct_call: bool => "Behavior",
        dialer_app: bool => "Behavior";
        events: on_call_state, on_error;
        computed: state: String => state / set_s, number: String => number / set_s
    }

    fn methods(&self) -> &'static [MethodInfo] {
        use PropKind::{Bool, Int, String};
        const METHODS: &[MethodInfo] = &[
            MethodInfo::new("dial", &[String], Some(Bool)).params(&["number"]),
            MethodInfo::new("call", &[String], Some(Bool)).params(&["number"]),
            MethodInfo::new("answer", &[], Some(Bool)),
            MethodInfo::new("reject", &[], Some(Bool)),
            MethodInfo::new("hang_up", &[], Some(Bool)),
            MethodInfo::new("request_dialer", &[], Some(Bool)),
            MethodInfo::new("history", &[Int], Some(String)).params(&["limit"]),
        ];
        METHODS
    }

    fn invoke(&mut self, ctx: &mut EventCtx<'_>, method: &str, args: &[PropValue]) -> Result<PropValue, String> {
        let text = args.first().and_then(|v| v.to_str().ok()).unwrap_or_default().to_string();
        let s = Self::service(ctx.ui)?;
        let ok = |r: Result<(), String>| r.map(|_| PropValue::Bool(true));
        match method {
            "dial" => ok(s.dial(&text)),
            "call" => ok(s.call(&text)),
            "answer" => Ok(PropValue::Bool(s.answer())),
            "reject" => Ok(PropValue::Bool(s.reject())),
            "hang_up" => Ok(PropValue::Bool(s.hang_up())),
            "request_dialer" => ok(s.request_dialer()),
            "history" => s.history(args.first().and_then(|v| v.to_i64().ok()).unwrap_or(20).clamp(1, 500) as u32).map(PropValue::String),
            _ => Err(format!("Phone não tem o método '{method}'")),
        }
    }

    fn timer_interval(&self) -> Option<f64> {
        (self.watch_state || self.dialer_app).then_some(0.25)
    }

    fn timer(&mut self, ctx: &mut EventCtx<'_>) {
        if ctx.ui.design_mode() {
            return;
        }
        let Ok(s) = Self::service(ctx.ui) else { return };
        if !self.watching {
            self.watching = true;
            s.watch(true);
        }
        let (state, number) = s.state();
        if state != self.state() || (!number.is_empty() && number != self.number) {
            self.state = state.clone();
            if !number.is_empty() {
                self.number = number;
            }
            ctx.fire("on_call_state", EventArgs::Text(state));
        }
    }
}

/// O form sobre a tela de bloqueio (`show_when_locked`), acendendo a tela ao abrir
/// (`turn_screen_on`), e `dismiss` para pedir o desbloqueio. Com uma `Notification` de
/// `full_screen`, o app abre por cima do bloqueio (uma ligação, um alarme). Hoje, o Android.
#[derive(Default)]
pub struct LockScreen {
    /// O form aparece por cima da tela de bloqueio.
    pub show_when_locked: bool,
    /// Acende a tela quando o form abre.
    pub turn_screen_on: bool,
    applied: Option<(bool, bool)>,
}

impl Component for LockScreen {
    fn class_name(&self) -> &'static str {
        "LockScreen"
    }

    properties! {
        show_when_locked: bool => "Behavior",
        turn_screen_on: bool => "Behavior";
        events: on_error
    }

    fn methods(&self) -> &'static [MethodInfo] {
        const METHODS: &[MethodInfo] = &[MethodInfo::new("dismiss", &[], None)];
        METHODS
    }

    fn invoke(&mut self, ctx: &mut EventCtx<'_>, method: &str, _args: &[PropValue]) -> Result<PropValue, String> {
        match method {
            "dismiss" => {
                if let Some(s) = ctx.ui.services_mut().phone.as_mut() {
                    s.dismiss_keyguard();
                }
                Ok(PropValue::Null)
            }
            _ => Err(format!("LockScreen não tem o método '{method}'")),
        }
    }

    // Aplica no primeiro timer e quando as propriedades mudam.
    fn timer_interval(&self) -> Option<f64> {
        Some(0.5)
    }

    fn timer(&mut self, ctx: &mut EventCtx<'_>) {
        let want = (self.show_when_locked, self.turn_screen_on);
        if ctx.ui.design_mode() || self.applied == Some(want) {
            return;
        }
        self.applied = Some(want);
        if let Some(s) = ctx.ui.services_mut().phone.as_mut() {
            s.lock_screen(want.0, want.1);
        }
    }
}

/// Um controle de jogo (Xbox, PlayStation, genérico): o `index` (0 = o primeiro conectado),
/// os analógicos (`left_x`, `left_y`, `right_x`, `right_y`, de -1 a 1, y positivo para baixo)
/// e os gatilhos (`left_trigger`, `right_trigger`, de 0 a 1), lidos a cada quadro com a
/// `dead_zone` aplicada. `on_button_down`/`on_button_up` trazem o nome do botão (`a`, `b`,
/// `x`, `y`, `lb`, `rb`, `lt`, `rt`, `back`, `start`, `l3`, `r3`, `up`, `down`, `left`,
/// `right`, `guide`; no PlayStation, `a` = ✕); `on_change`, os analógicos mexeram;
/// `on_connect`/`on_disconnect` (`device_name`, o nome do controle). Métodos:
/// `pressed(nome)` e `rumble(força, ms)`.
///
/// | Plataforma | O que tem |
/// |---|---|
/// | Windows, Linux, macOS, navegador | tudo (pelo `gilrs`; vibração no Windows e no Linux) |
/// | Android | botões, direcional e o analógico esquerdo (o winit descarta o direito e os gatilhos analógicos) |
/// | iOS | ainda não (`connected` falso) |
pub struct Gamepad {
    /// Qual controle conectado ler (0 = o primeiro).
    pub index: u32,
    /// Lê o controle a cada quadro e dispara os eventos; desligado, para.
    pub active: bool,
    /// A zona morta dos analógicos e gatilhos, de 0 a 1: abaixo dela é 0 (a folga do
    /// controle parado).
    pub dead_zone: f32,
    state: Option<aylis_core::platform::GamepadState>,
    axes: [f32; 6],
    count: u32,
}

impl Default for Gamepad {
    fn default() -> Self {
        Self { index: 0, active: true, dead_zone: 0.15, state: None, axes: [0.0; 6], count: 0 }
    }
}

/// Zona morta: abaixo dela é 0 e o resto vai de 0 a 1 (nos analógicos, pela distância do centro).
fn dead(x: f32, y: f32, zone: f32) -> (f32, f32) {
    let len = (x * x + y * y).sqrt();
    if len <= zone || len == 0.0 {
        return (0.0, 0.0);
    }
    let scale = ((len - zone) / (1.0 - zone).max(1e-3)).min(1.0) / len;
    (x * scale, y * scale)
}

impl Gamepad {
    pub fn connected(&self) -> bool {
        self.state.is_some()
    }
    /// O nome do controle no sistema ("Xbox Wireless Controller"...).
    pub fn device_name(&self) -> String {
        self.state.as_ref().map(|s| s.name.clone()).unwrap_or_default()
    }
    /// Quantos controles estão conectados.
    pub fn count(&self) -> u32 {
        self.count
    }
    pub fn left_x(&self) -> f64 {
        f64::from(self.axes[0])
    }
    pub fn left_y(&self) -> f64 {
        f64::from(self.axes[1])
    }
    pub fn right_x(&self) -> f64 {
        f64::from(self.axes[2])
    }
    pub fn right_y(&self) -> f64 {
        f64::from(self.axes[3])
    }
    pub fn left_trigger(&self) -> f64 {
        f64::from(self.axes[4])
    }
    pub fn right_trigger(&self) -> f64 {
        f64::from(self.axes[5])
    }
    /// O botão está apertado agora (pelo nome, como em `on_button_down`).
    pub fn pressed(&self, button: &str) -> bool {
        self.state.as_ref().is_some_and(|s| s.pressed(button))
    }
    /// Vibra o controle (força de 0 a 1); `false` se ele não vibra.
    pub fn rumble(&self, ui: &mut Ui, strength: f32, ms: u32) -> bool {
        let index = self.index as usize;
        ui.services_mut().gamepads.as_mut().is_some_and(|g| g.rumble(index, strength, ms))
    }
    fn set_s(&mut self, _: String) {}
    fn set_u(&mut self, _: u32) {}
    fn set_f(&mut self, _: f64) {}
    fn set_b(&mut self, _: bool) {}

    /// Lê o controle e dispara os eventos do que mudou.
    fn poll(&mut self, ctx: &mut EventCtx<'_>) {
        let pads = ctx.ui.services_mut().gamepads.as_mut().map(|g| g.pads()).unwrap_or_default();
        self.count = pads.len() as u32;
        let now = pads.into_iter().nth(self.index as usize);
        let before = std::mem::replace(&mut self.state, now);
        match (&before, &self.state) {
            (None, Some(_)) => ctx.fire("on_connect", EventArgs::None),
            (Some(_), None) => ctx.fire("on_disconnect", EventArgs::None),
            _ => {}
        }
        let (old, new) = (before.as_ref().map_or(0, |s| s.buttons), self.state.as_ref().map_or(0, |s| s.buttons));
        for (i, name) in aylis_core::platform::GAMEPAD_BUTTONS.iter().enumerate() {
            let bit = 1u32 << i;
            if new & bit != 0 && old & bit == 0 {
                ctx.fire("on_button_down", EventArgs::Value((*name).into()));
            } else if old & bit != 0 && new & bit == 0 {
                ctx.fire("on_button_up", EventArgs::Value((*name).into()));
            }
        }
        let raw = self.state.as_ref().map_or([0.0; 6], |s| s.axes);
        let zone = self.dead_zone.clamp(0.0, 0.95);
        let (lx, ly) = dead(raw[0], raw[1], zone);
        let (rx, ry) = dead(raw[2], raw[3], zone);
        let trigger = |t: f32| if t <= zone / 2.0 { 0.0 } else { t.min(1.0) };
        let axes = [lx, ly, rx, ry, trigger(raw[4]), trigger(raw[5])];
        if axes.iter().zip(self.axes.iter()).any(|(a, b)| (a - b).abs() > 1e-3) {
            self.axes = axes;
            ctx.fire("on_change", EventArgs::None);
        }
    }
}

impl Component for Gamepad {
    fn class_name(&self) -> &'static str {
        "Gamepad"
    }

    properties! {
        index: u32 => "Behavior",
        active: bool => "Behavior",
        dead_zone: f32 => "Behavior";
        events: on_button_down, on_button_up, on_change, on_connect, on_disconnect;
        computed: connected: bool => connected / set_b, device_name: String => device_name / set_s, count: u32 => count / set_u,
            left_x: f64 => left_x / set_f, left_y: f64 => left_y / set_f, right_x: f64 => right_x / set_f,
            right_y: f64 => right_y / set_f, left_trigger: f64 => left_trigger / set_f, right_trigger: f64 => right_trigger / set_f
    }

    // A cada quadro (60 por segundo) enquanto `active`.
    fn timer_interval(&self) -> Option<f64> {
        self.active.then_some(1.0 / 60.0)
    }

    fn timer(&mut self, ctx: &mut EventCtx<'_>) {
        if !ctx.ui.design_mode() {
            self.poll(ctx);
        }
    }

    fn methods(&self) -> &'static [MethodInfo] {
        const METHODS: &[MethodInfo] = &[
            MethodInfo::new("pressed", &[PropKind::String], Some(PropKind::Bool)).params(&["button"]),
            MethodInfo::new("rumble", &[PropKind::Float, PropKind::Int], Some(PropKind::Bool)).params(&["strength", "ms"]),
        ];
        METHODS
    }

    fn invoke(&mut self, ctx: &mut EventCtx<'_>, method: &str, args: &[PropValue]) -> Result<PropValue, String> {
        match method {
            "pressed" => {
                let button = args.first().and_then(|v| v.to_str().ok()).unwrap_or_default();
                Ok(PropValue::Bool(self.pressed(button)))
            }
            "rumble" => {
                let strength = args.first().and_then(|v| v.to_f64().ok()).unwrap_or(1.0) as f32;
                let ms = args.get(1).and_then(|v| v.to_i64().ok()).unwrap_or(200).clamp(1, 10_000) as u32;
                Ok(PropValue::Bool(self.rumble(ctx.ui, strength, ms)))
            }
            _ => Err(format!("Gamepad não tem o método '{method}'")),
        }
    }
}

/// Os monitores, para todas as linguagens: `count()`, `primary()`
/// (o índice do principal), `monitors()` (a lista em JSON: nome do aparelho, nome do sistema,
/// principal, posição e tamanho, área útil sem a barra de tarefas, escala, taxa de
/// atualização e modos de vídeo), `monitor(i)`, `refresh_rate(i)` e `form_monitor(form)` (em
/// qual está a janela do form); `on_change` quando um monitor entra, sai ou muda de
/// resolução. Em Rust, também direto: `ui.monitors()` e `ui.form_monitor(form)`. Para abrir
/// num monitor, o `monitor` do form; para mudar, o `move_to_monitor` dele.
#[derive(Default)]
pub struct Screen {
    version: Option<u64>,
}

// Sem propriedades, só os métodos: o `get_prop` gerado não tem casos.
#[allow(unreachable_code)]
impl Component for Screen {
    fn class_name(&self) -> &'static str {
        "Screen"
    }

    properties! {
        ;
        events: on_change
    }

    fn timer_interval(&self) -> Option<f64> {
        Some(1.0)
    }

    fn timer(&mut self, ctx: &mut EventCtx<'_>) {
        let now = ctx.ui.monitors_version();
        if self.version.is_some_and(|v| v != now) {
            ctx.fire("on_change", EventArgs::None);
        }
        self.version = Some(now);
    }

    fn methods(&self) -> &'static [MethodInfo] {
        const METHODS: &[MethodInfo] = &[
            MethodInfo::new("count", &[], Some(PropKind::Int)),
            MethodInfo::new("primary", &[], Some(PropKind::Int)),
            MethodInfo::new("monitors", &[], Some(PropKind::String)),
            MethodInfo::new("monitor", &[PropKind::Int], Some(PropKind::String)).params(&["index"]),
            MethodInfo::new("refresh_rate", &[PropKind::Int], Some(PropKind::Float)).params(&["index"]),
            MethodInfo::new("form_monitor", &[PropKind::String], Some(PropKind::Int)).params(&["form"]),
        ];
        METHODS
    }

    fn invoke(&mut self, ctx: &mut EventCtx<'_>, method: &str, args: &[PropValue]) -> Result<PropValue, String> {
        let monitors = ctx.ui.monitors();
        let index = || args.first().and_then(|v| v.to_i64().ok()).filter(|i| *i >= 0).map(|i| i as usize);
        Ok(match method {
            "count" => PropValue::Int(monitors.len() as i64),
            "primary" => PropValue::Int(monitors.iter().position(|m| m.primary).map_or(-1, |i| i as i64)),
            "monitors" => PropValue::String(format!("[{}]", monitors.iter().enumerate().map(|(i, m)| m.to_json(i)).collect::<Vec<_>>().join(","))),
            "monitor" => PropValue::String(index().and_then(|i| monitors.get(i).map(|m| m.to_json(i))).unwrap_or_else(|| "null".into())),
            "refresh_rate" => PropValue::Float(index().and_then(|i| monitors.get(i)).map_or(0.0, |m| m.refresh_hz)),
            "form_monitor" => {
                let name = args.first().and_then(|v| v.to_str().ok()).unwrap_or_default();
                let form = ctx.ui.find_path(name);
                PropValue::Int(form.and_then(|f| ctx.ui.form_monitor(f)).map_or(-1, |i| i as i64))
            }
            _ => return Err(format!("Screen não tem o método '{method}'")),
        })
    }
}
