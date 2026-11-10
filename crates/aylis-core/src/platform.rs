//! Serviços de plataforma (como as interfaces `IFMX*Service` do FMX).
//!
//! O core só conhece estes traits. Cada plataforma (Windows, Linux, web, Android, iOS)
//! fornece as implementações num crate próprio e as entrega ao [`Ui`](crate::Ui) em
//! [`Services`]. Sem serviços (por exemplo em testes), o `Ui` funciona normalmente,
//! só não abre janelas.

use crate::canvas::TextService;
use crate::geometry::{RectF, SizeF};

/// Identificador de uma janela nativa.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct WindowId(pub u64);

/// Identificador de um timer.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TimerId(pub u64);

/// Janelas nativas (como `IFMXWindowService`).
pub trait WindowService {
    fn create_window(&mut self, title: &str, size: SizeF) -> WindowId;
    fn close_window(&mut self, window: WindowId);
    fn set_title(&mut self, window: WindowId, title: &str);
    fn set_size(&mut self, window: WindowId, size: SizeF);
    /// Pede que a janela seja repintada no próximo frame.
    fn request_redraw(&mut self, window: WindowId);
    /// Escala de DPI da janela (1.0 = 96 dpi).
    fn scale_factor(&self, window: WindowId) -> f32;
}

/// Timers e frames (como `IFMXTimerService`).
pub trait TimerService {
    fn start_timer(&mut self, interval_ms: u32, repeat: bool) -> TimerId;
    fn stop_timer(&mut self, timer: TimerId);
    /// Pede um frame (usado pelas animações; só enquanto houver animação ativa).
    fn request_frame(&mut self);
    /// Relógio monotônico em segundos.
    fn now(&self) -> f64;
}

/// Área de transferência (como `IFMXClipboardService`).
pub trait ClipboardService {
    fn get_text(&mut self) -> Option<String>;
    fn set_text(&mut self, text: &str);
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CursorShape {
    #[default]
    Default,
    Pointer,
    Text,
    Wait,
    Crosshair,
    Move,
    NotAllowed,
    ResizeHorizontal,
    ResizeVertical,
    ResizeNeSw,
    ResizeNwSe,
    /// Sem cursor (o vídeo tocando com o mouse parado).
    None,
}

impl CursorShape {
    pub const ALL: [CursorShape; 12] = [
        CursorShape::Default,
        CursorShape::Pointer,
        CursorShape::Text,
        CursorShape::Wait,
        CursorShape::Crosshair,
        CursorShape::Move,
        CursorShape::NotAllowed,
        CursorShape::ResizeHorizontal,
        CursorShape::ResizeVertical,
        CursorShape::ResizeNeSw,
        CursorShape::ResizeNwSe,
        CursorShape::None,
    ];
}

impl crate::prop::PropEnum for CursorShape {
    const NAMES: &'static [&'static str] = &[
        "default",
        "pointer",
        "text",
        "wait",
        "crosshair",
        "move",
        "not_allowed",
        "resize_horizontal",
        "resize_vertical",
        "resize_ne_sw",
        "resize_nw_se",
        "none",
    ];

    fn from_index(index: usize) -> Option<Self> {
        CursorShape::ALL.get(index).copied()
    }

    fn index(self) -> usize {
        self as usize
    }
}

/// Cursor do mouse (como `IFMXCursorService`). Plataformas sem mouse podem ignorar.
pub trait CursorService {
    fn set_cursor(&mut self, shape: CursorShape);
}

/// Aparência preferida pelo sistema.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Appearance {
    #[default]
    Light,
    Dark,
}

/// Tela (como `IFMXScreenService`).
pub trait ScreenService {
    fn screen_size(&self) -> SizeF;
    fn scale_factor(&self) -> f32;
    /// Tema claro/escuro do sistema operacional.
    fn appearance(&self) -> Appearance;
}

/// Tipo de teclado virtual a mostrar no mobile.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum KeyboardKind {
    #[default]
    Text,
    Number,
    Decimal,
    Phone,
    Email,
    Url,
}

/// Teclado virtual e IME (como `IFMXVirtualKeyboardService`).
pub trait TextInputService {
    fn show_keyboard(&mut self, kind: KeyboardKind);
    fn hide_keyboard(&mut self);
    /// Onde está o cursor de texto (para a janela de composição do IME).
    fn set_ime_rect(&mut self, rect: RectF);
}

/// Formatos regionais (como o `FormatSettings`).
#[derive(Clone, Debug, PartialEq)]
pub struct LocaleInfo {
    pub decimal_separator: char,
    pub thousand_separator: char,
    pub short_date_format: String,
    pub long_time_format: String,
    pub currency_symbol: String,
    /// Nomes dos meses (janeiro a dezembro) e dos dias da semana abreviados (domingo
    /// primeiro), para calendários.
    pub month_names: [&'static str; 12],
    pub short_day_names: [&'static str; 7],
    /// Primeiro dia da semana nos calendários (0 = domingo).
    pub first_day_of_week: u8,
}

const MONTHS_EN: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
const DAYS_EN: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];

impl Default for LocaleInfo {
    fn default() -> Self {
        Self {
            decimal_separator: '.',
            thousand_separator: ',',
            short_date_format: "yyyy-mm-dd".into(),
            long_time_format: "hh:nn:ss".into(),
            currency_symbol: "$".into(),
            month_names: MONTHS_EN,
            short_day_names: DAYS_EN,
            first_day_of_week: 0,
        }
    }
}

impl LocaleInfo {
    /// Português do Brasil: `1.234,56`, `dd/mm/yyyy`, `R$`.
    pub fn pt_br() -> Self {
        Self {
            decimal_separator: ',',
            thousand_separator: '.',
            short_date_format: "dd/mm/yyyy".into(),
            long_time_format: "hh:nn:ss".into(),
            currency_symbol: "R$".into(),
            month_names: ["janeiro", "fevereiro", "março", "abril", "maio", "junho", "julho", "agosto", "setembro", "outubro", "novembro", "dezembro"],
            short_day_names: ["dom", "seg", "ter", "qua", "qui", "sex", "sáb"],
            first_day_of_week: 0,
        }
    }

    /// Espanhol: `1.234,56`, `dd/mm/yyyy`, `€`, semana começando na segunda.
    pub fn es() -> Self {
        Self {
            decimal_separator: ',',
            thousand_separator: '.',
            short_date_format: "dd/mm/yyyy".into(),
            long_time_format: "hh:nn:ss".into(),
            currency_symbol: "€".into(),
            month_names: ["enero", "febrero", "marzo", "abril", "mayo", "junio", "julio", "agosto", "septiembre", "octubre", "noviembre", "diciembre"],
            short_day_names: ["dom", "lun", "mar", "mié", "jue", "vie", "sáb"],
            first_day_of_week: 1,
        }
    }

    /// Inglês dos EUA: `1,234.56`, `mm/dd/yyyy`, `$`.
    pub fn en_us() -> Self {
        Self {
            decimal_separator: '.',
            thousand_separator: ',',
            short_date_format: "mm/dd/yyyy".into(),
            long_time_format: "hh:nn:ss".into(),
            currency_symbol: "$".into(),
            month_names: MONTHS_EN,
            short_day_names: DAYS_EN,
            first_day_of_week: 0,
        }
    }
}

pub trait LocaleService {
    fn locale(&self) -> LocaleInfo;
}

/// Um `LocaleInfo` fixo também serve como serviço (útil para escolher o idioma do app).
impl LocaleService for LocaleInfo {
    fn locale(&self) -> LocaleInfo {
        self.clone()
    }
}

/// Eventos de ciclo de vida do aplicativo (importante no mobile).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LifecycleEvent {
    Resumed,
    Suspended,
    LowMemory,
    /// A superfície de desenho foi perdida (GPU); é preciso recriar.
    SurfaceLost,
}

/// Arquivos do aplicativo (imagens, áudio, vídeo, fontes) por caminho relativo à pasta
/// do projeto, como `assets/images/logo.png`. No desktop vêm do disco; no Android podem vir
/// de dentro do APK. O `aylis-project` instala um que lê da pasta do projeto.
pub trait ResourceService {
    fn read(&self, path: &str) -> Option<Vec<u8>>;
}

/// Cria forms pelo nome (como o `Application.CreateForm`). O `aylis-project` instala um que
/// carrega os forms do projeto (`forms/*.json`).
pub trait FormFactory {
    /// Cria o form `name` (sem abrir) e devolve o id.
    fn create_form(&mut self, ui: &mut crate::Ui, name: &str) -> Result<crate::ComponentId, String>;
    /// Nomes dos forms que esta fábrica sabe criar.
    fn form_names(&self) -> Vec<String>;
}

/// Opções de um diálogo de arquivo (Abrir/Salvar).
#[derive(Clone, Debug, Default)]
pub struct FileDialogOptions {
    pub title: String,
    /// (nome, extensões sem ponto): `("Imagens", ["png", "jpg"])`.
    pub filters: Vec<(String, Vec<String>)>,
    pub directory: String,
    pub file_name: String,
    /// Salvar em vez de abrir.
    pub save: bool,
    /// Abrir vários.
    pub multiple: bool,
}

/// Diálogos de arquivo do sistema. Assíncronos: `done` recebe os caminhos escolhidos
/// (vazio = cancelado), de qualquer thread.
pub trait FileDialogService {
    fn show(&mut self, options: FileDialogOptions, done: Box<dyn FnOnce(Vec<String>) + Send>);
}

/// Uma permissão que o sistema pergunta ao usuário (celular e navegador; no desktop quase
/// tudo já vem liberado).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Permission {
    Camera,
    Microphone,
    /// A localização com o app aberto.
    Location,
    Notifications,
    /// Fotos e vídeos da galeria.
    Photos,
    Contacts,
    Calendar,
    Bluetooth,
}

impl Permission {
    pub const ALL: [Permission; 8] = [
        Permission::Camera,
        Permission::Microphone,
        Permission::Location,
        Permission::Notifications,
        Permission::Photos,
        Permission::Contacts,
        Permission::Calendar,
        Permission::Bluetooth,
    ];

    /// O nome usado nos forms e no código (`camera`, `microphone`...).
    pub fn name(self) -> &'static str {
        match self {
            Permission::Camera => "camera",
            Permission::Microphone => "microphone",
            Permission::Location => "location",
            Permission::Notifications => "notifications",
            Permission::Photos => "photos",
            Permission::Contacts => "contacts",
            Permission::Calendar => "calendar",
            Permission::Bluetooth => "bluetooth",
        }
    }

    pub fn parse(name: &str) -> Option<Permission> {
        let name = name.trim();
        Permission::ALL.into_iter().find(|p| p.name().eq_ignore_ascii_case(name))
    }

    /// Uma lista separada por `;` (ou `,`); nomes desconhecidos ficam de fora.
    pub fn parse_list(list: &str) -> Vec<Permission> {
        let mut out = Vec::new();
        for p in list.split([';', ',']).filter_map(Permission::parse) {
            if !out.contains(&p) {
                out.push(p);
            }
        }
        out
    }
}

/// A situação de uma permissão.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PermissionState {
    /// Liberada.
    Granted,
    /// Negada (no celular, às vezes só nas configurações do aparelho dá para liberar).
    Denied,
    /// Ainda não perguntada: pedir mostra a pergunta do sistema.
    Prompt,
    /// O aparelho ou o navegador não tem isso.
    Unavailable,
}

impl PermissionState {
    pub fn name(self) -> &'static str {
        match self {
            PermissionState::Granted => "granted",
            PermissionState::Denied => "denied",
            PermissionState::Prompt => "prompt",
            PermissionState::Unavailable => "unavailable",
        }
    }
}

/// Consultar e pedir permissões ao sistema. `done` recebe cada permissão pedida com a
/// resposta, de qualquer thread.
pub trait PermissionService {
    fn state(&self, permission: Permission) -> PermissionState;
    fn request(&mut self, permissions: Vec<Permission>, done: Box<dyn FnOnce(Vec<(Permission, PermissionState)>) + Send>);
}

/// O tipo de biometria do aparelho.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Biometry {
    /// Sem biometria (ou nenhuma cadastrada).
    None,
    Fingerprint,
    Face,
    Iris,
    /// Tem, mas o sistema não diz qual (Windows Hello, a chave do navegador, vários tipos).
    Any,
}

impl Biometry {
    pub fn name(self) -> &'static str {
        match self {
            Biometry::None => "none",
            Biometry::Fingerprint => "fingerprint",
            Biometry::Face => "face",
            Biometry::Iris => "iris",
            Biometry::Any => "any",
        }
    }
}

/// Como terminou a verificação biométrica.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BiometricResult {
    Success,
    /// O usuário desistiu (ou o app foi para o fundo).
    Cancelled,
    /// Não reconheceu, bloqueou por tentativas, sem biometria: o motivo.
    Failed(String),
}

/// A biometria do aparelho (digital, rosto; o Windows Hello). `done` recebe o resultado,
/// de qualquer thread.
pub trait BiometricService {
    fn availability(&self) -> Biometry;
    /// Pede a verificação mostrando `reason`. Com `allow_device_credential`, o PIN ou o
    /// padrão do aparelho também valem.
    fn authenticate(&mut self, reason: &str, allow_device_credential: bool, done: Box<dyn FnOnce(BiometricResult) + Send>);
}

/// Imprimir na impressora do sistema: mostra o diálogo de impressão (impressora, cópias,
/// páginas) e imprime as páginas gravadas. `done` recebe `Ok(true)` (impresso), `Ok(false)`
/// (cancelado) ou o erro, de qualquer thread.
pub trait PrintService {
    fn print(&mut self, title: &str, pages: crate::pages::PageSet, done: Box<dyn FnOnce(Result<bool, String>) + Send>);
}

/// Baixar da internet (HTTP GET) sem travar a interface (os blocos do mapa, imagens de
/// um endereço). `done` recebe o corpo da resposta, ou o erro, de qualquer thread: use o
/// [`Poster`](crate::Poster) para voltar à interface.
pub trait HttpService {
    fn get(&mut self, url: &str, done: Box<dyn FnOnce(Result<Vec<u8>, String>) + Send>);
}

/// Uma notificação do sistema (o toast do Windows, a notificação do Android).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SystemNotification {
    /// Quem notifica (o AppUserModelID no Windows: `empresa.app`).
    pub app_id: String,
    /// O nome que aparece na notificação.
    pub app_name: String,
    pub title: String,
    pub body: String,
    /// Imagem (caminho de um PNG), opcional.
    pub image: String,
    /// A imagem em PNG já lida (dos assets do app, ou um form desenhado); vence `image`.
    pub image_png: Vec<u8>,
    /// Mostrar a imagem grande (embaixo do texto), em vez de miniatura ao lado.
    pub big_image: bool,
    /// Identifica a notificação: mostrar de novo com o mesmo número atualiza a anterior.
    pub id: u32,
    /// Canal (Android 8+): o usuário liga, desliga e escolhe o som por canal.
    pub channel: String,
    pub channel_name: String,
    pub priority: NotificationPriority,
    /// Botões da notificação: (identificador, texto).
    pub actions: Vec<(String, String)>,
    /// Fixa: o usuário não descarta (uma tarefa em andamento).
    pub ongoing: bool,
    /// Barra de progresso (0 a 1); `None`: sem barra.
    pub progress: Option<f32>,
    /// Tela cheia (Android): abre o app por cima de tudo, até da tela de bloqueio (uma
    /// ligação, um alarme). Precisa da permissão `USE_FULL_SCREEN_INTENT`.
    pub full_screen: bool,
    /// O componente que recebe `on_click` e `on_action` quando o usuário toca.
    pub component: Option<crate::ComponentId>,
}

/// Importância da notificação (no Android, a do canal: som, vibração, aparecer por cima).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum NotificationPriority {
    /// Silenciosa, só na lista.
    Low,
    #[default]
    Default,
    /// Aparece por cima do app aberto (heads-up), com som.
    High,
}

impl crate::PropEnum for NotificationPriority {
    const NAMES: &'static [&'static str] = &["low", "default", "high"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Low, Self::Default, Self::High].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Notificações do sistema.
pub trait NotificationService {
    fn notify(&mut self, notification: &SystemNotification) -> Result<(), String>;
    /// Tira a notificação `id` da lista.
    fn cancel(&mut self, id: u32) -> Result<(), String> {
        let _ = id;
        Ok(())
    }
}

/// Abrir endereços e outros apps, compartilhar e pedir algo a outro app (o componente
/// `AppLink`). O que a plataforma não tem responde com o motivo.
pub trait LinkService {
    /// Abre o endereço no app que cuida dele (o navegador, o e-mail, o mapa, o discador).
    fn open_url(&mut self, url: &str) -> Result<(), String>;
    /// Abre outro app: o pacote no Android, o executável ou o nome registrado no desktop.
    fn open_app(&mut self, app: &str) -> Result<(), String> {
        Err(format!("abrir outro app não existe aqui ({app})"))
    }
    /// Compartilha texto e arquivos pela folha do sistema.
    fn share(&mut self, text: &str, files: &[String]) -> Result<(), String> {
        let _ = (text, files);
        Err("compartilhar não existe aqui".into())
    }
    /// Pede `action` a outro app (uma `Intent` do Android com `data`): `done` recebe o que ele
    /// devolveu, de qualquer thread.
    fn request(&mut self, action: &str, data: &str, done: Box<dyn FnOnce(Result<String, String>) + Send>) {
        let _ = data;
        done(Err(format!("pedir a outro app não existe aqui ({action})")));
    }
}

/// Os sensores do aparelho (os componentes da categoria "Dispositivo").
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Sensor {
    /// `[latitude, longitude, precisão em metros, altitude, velocidade em m/s]`.
    Location,
    /// `[carga de 0 a 1, carregando 0/1]`.
    Battery,
    /// `[conectado 0/1, tipo: 0 nenhum, 1 Wi-Fi, 2 celular, 3 cabo, 4 outro]`.
    Network,
    /// A aceleração com a gravidade, em m/s²: `[x, y, z]`.
    Motion,
    /// `[rumo em graus, 0 = norte]`.
    Compass,
    /// `[distância em cm, perto 0/1]`.
    Proximity,
    /// `[lux]`.
    Light,
}

/// Lê os sensores do aparelho. A plataforma guarda a última leitura de cada um (os
/// componentes consultam num timer) e só mede os ligados (bateria e rede valem sempre).
pub trait SensorService {
    /// O aparelho tem esse sensor.
    fn available(&self, sensor: Sensor) -> bool;
    /// Começa ou para de medir (a localização pede a permissão na primeira vez).
    fn set_active(&mut self, sensor: Sensor, on: bool);
    /// A última leitura (`None`: ainda não mediu).
    fn read(&mut self, sensor: Sensor) -> Option<Vec<f64>>;
    /// Vibra por `ms` milissegundos; `false` se o aparelho não vibra.
    fn vibrate(&mut self, ms: u32) -> bool {
        let _ = ms;
        false
    }
}

/// Os botões de um controle de jogo (o componente `Gamepad`), com os nomes do Xbox; no
/// PlayStation, `a` = ✕, `b` = ○, `x` = □ e `y` = △. O bit `i` de [`GamepadState::buttons`]
/// é o botão `GAMEPAD_BUTTONS[i]`.
pub const GAMEPAD_BUTTONS: [&str; 17] =
    ["a", "b", "x", "y", "lb", "rb", "lt", "rt", "back", "start", "l3", "r3", "up", "down", "left", "right", "guide"];

/// Um controle de jogo conectado.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GamepadState {
    pub name: String,
    /// Os botões apertados (um bit por nome de [`GAMEPAD_BUTTONS`]).
    pub buttons: u32,
    /// `[esquerdo x, esquerdo y, direito x, direito y, gatilho esquerdo, gatilho direito]`:
    /// os analógicos de -1 a 1 (y positivo para baixo, como na tela), os gatilhos de 0 a 1.
    pub axes: [f32; 6],
}

impl GamepadState {
    /// Marca o botão pelo nome (os da plataforma usam ao montar o estado).
    pub fn set_button(&mut self, name: &str, down: bool) {
        if let Some(i) = GAMEPAD_BUTTONS.iter().position(|b| *b == name) {
            if down {
                self.buttons |= 1 << i;
            } else {
                self.buttons &= !(1 << i);
            }
        }
    }

    pub fn pressed(&self, name: &str) -> bool {
        GAMEPAD_BUTTONS.iter().position(|b| *b == name).is_some_and(|i| self.buttons & (1 << i) != 0)
    }
}

/// Os controles de jogo (o componente `Gamepad`). O componente consulta a cada quadro.
pub trait GamepadService {
    /// Os controles conectados agora, na ordem em que entraram.
    fn pads(&mut self) -> Vec<GamepadState>;
    /// Vibra o controle `index` com a força (0 a 1) por `ms`; `false` se ele não vibra.
    fn rumble(&mut self, index: usize, strength: f32, ms: u32) -> bool {
        let _ = (index, strength, ms);
        false
    }
}

/// Ligações e a tela de bloqueio (os componentes `Phone` e `LockScreen`; hoje, o Android).
pub trait PhoneService {
    /// Abre o discador com o número (sem permissão).
    fn dial(&mut self, number: &str) -> Result<(), String>;
    /// Liga direto (a permissão `CALL_PHONE`, pedida na primeira vez).
    fn call(&mut self, number: &str) -> Result<(), String>;
    /// Acompanha o estado das chamadas (`READ_PHONE_STATE`).
    fn watch(&mut self, on: bool);
    /// O estado agora (`idle`, `ringing`, `dialing`, `active`, `holding`) e o número (quando o
    /// sistema informa).
    fn state(&mut self) -> (String, String);
    /// O modo "app de telefone": pede para ser o discador padrão (o `InCallService`).
    fn request_dialer(&mut self) -> Result<(), String> {
        Err("não existe aqui".into())
    }
    /// No modo "app de telefone": atender, recusar, desligar.
    fn answer(&mut self) -> bool {
        false
    }
    fn reject(&mut self) -> bool {
        false
    }
    fn hang_up(&mut self) -> bool {
        false
    }
    /// As últimas `max` chamadas (`READ_CALL_LOG`): `número|tipo|quando|segundos` por linha.
    fn history(&mut self, max: u32) -> Result<String, String> {
        let _ = max;
        Err("não existe aqui".into())
    }
    /// O form sobre a tela de bloqueio e acender a tela ao abrir.
    fn lock_screen(&mut self, show_when_locked: bool, turn_screen_on: bool);
    /// Pede ao sistema para destravar (o usuário confirma).
    fn dismiss_keyguard(&mut self);
}

/// O serviço em primeiro plano do sistema (o Android: a notificação fixa que mantém o
/// processo vivo com a tela fechada). `kind`: o tipo do Android 14+ (`dataSync`, `location`,
/// `mediaPlayback`, `connectedDevice`...); `keep_alive`: o sistema reinicia o serviço (e no
/// boot) e o app pede a isenção da otimização de bateria.
pub trait BackgroundHost {
    fn start(&mut self, title: &str, text: &str, kind: &str, keep_alive: bool) -> Result<(), String>;
    fn stop(&mut self);
}

/// O que chega de fora para o app: um link que abriu o app (`on_link`) ou algo que outro
/// app compartilhou com ele (`on_shared`: o texto, ou os arquivos separados por `;`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Incoming {
    Link(String),
    Shared(String),
}

static INCOMING: std::sync::Mutex<Vec<Incoming>> = std::sync::Mutex::new(Vec::new());

/// A plataforma entrega o que chegou de fora (de qualquer thread). Fica guardado até um
/// `AppLink` pegar: o app aberto pelo próprio link ainda nem criou o form.
pub fn push_incoming(item: Incoming) {
    if let Ok(mut list) = INCOMING.lock() {
        list.push(item);
    }
}

/// Tira da fila o que chegou de fora (o `AppLink` chama).
pub fn take_incoming() -> Vec<Incoming> {
    INCOMING.lock().map(|mut list| std::mem::take(&mut *list)).unwrap_or_default()
}

/// Desenho fora da tela: o `Ui` pinta um componente numa imagem (a notificação com "um
/// form inteiro", capturas). A plataforma fornece com o renderizador dela.
pub trait OffscreenService {
    /// `width` x `height` em unidades lógicas, multiplicadas por `scale` em pixels.
    fn render_png(&self, width: u32, height: u32, scale: f32, background: crate::Color, paint: &mut dyn FnMut(&mut dyn crate::Canvas)) -> Option<Vec<u8>>;
}

/// Conjunto de serviços fornecidos pela plataforma. Todos são opcionais.
#[derive(Default)]
pub struct Services {
    pub offscreen: Option<Box<dyn OffscreenService>>,
    pub window: Option<Box<dyn WindowService>>,
    pub timer: Option<Box<dyn TimerService>>,
    pub clipboard: Option<Box<dyn ClipboardService>>,
    pub cursor: Option<Box<dyn CursorService>>,
    pub screen: Option<Box<dyn ScreenService>>,
    pub text_input: Option<Box<dyn TextInputService>>,
    pub locale: Option<Box<dyn LocaleService>>,
    pub text: Option<Box<dyn TextService>>,
    pub resources: Option<Box<dyn ResourceService>>,
    pub forms: Option<Box<dyn FormFactory>>,
    pub file_dialogs: Option<Box<dyn FileDialogService>>,
    /// Consultar e pedir permissões (câmera, localização...): o componente `Permissions`.
    pub permissions: Option<Box<dyn PermissionService>>,
    /// Digital, rosto, Windows Hello: o componente `Biometrics`.
    pub biometrics: Option<Box<dyn BiometricService>>,
    pub http: Option<Box<dyn HttpService>>,
    pub notifications: Option<Box<dyn NotificationService>>,
    /// Abrir endereços e apps, compartilhar: o componente `AppLink`.
    pub links: Option<Box<dyn LinkService>>,
    /// Localização, bateria, rede, movimento, bússola, proximidade, luz e vibração.
    pub sensors: Option<Box<dyn SensorService>>,
    /// O serviço em primeiro plano (o `BackgroundService`); sem ele, a lógica roda enquanto o
    /// app roda.
    pub background: Option<Box<dyn BackgroundHost>>,
    /// Ligações e tela de bloqueio (`Phone`, `LockScreen`).
    pub phone: Option<Box<dyn PhoneService>>,
    /// Controles de jogo (`Gamepad`).
    pub gamepads: Option<Box<dyn GamepadService>>,
    pub printer: Option<Box<dyn PrintService>>,
    /// Correção e sugestão de texto dos campos (`spellcheck`, `autocomplete`): o motor
    /// nativo da plataforma, ou outro que o app ponha no lugar ([`crate::textassist`]).
    pub text_assist: Option<Box<dyn crate::textassist::TextAssist>>,
    /// Tags NFC (`Nfc`).
    pub nfc: Option<Box<dyn NfcService>>,
    /// Bluetooth BLE e clássico (`Bluetooth`).
    pub bluetooth: Option<Box<dyn BluetoothService>>,
    /// Notificações push (`PushNotifications`): FCM, APNs, Web Push, WNS, UnifiedPush.
    pub push: Option<Box<dyn PushService>>,
    /// As ações do aparelho (`WakeAppAction`, `NotifyAction`) e as regras do `PushTrigger`
    /// executadas pela plataforma, também com o app fechado. Sem ele, os componentes fazem o
    /// que dá com o app aberto (mostrar o form, a notificação comum).
    pub actions: Option<Box<dyn ActionService>>,
}

/// Uma tag NFC encostada.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct NfcTag {
    /// O id em hexadecimal com dois pontos (`04:A2:1B:...`; [`crate::ndef::id_text`]).
    pub id: String,
    /// A tecnologia (`NfcA`, `MifareUltralight`, `IsoDep`...), como a plataforma informa.
    pub tech: String,
    /// A mensagem NDEF (vazia numa tag sem NDEF).
    pub records: Vec<crate::ndef::NdefRecord>,
    /// Dá para gravar nela.
    pub writable: bool,
}

/// O que o [`NfcService`] tem para contar.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NfcEvent {
    Tag(NfcTag),
    /// A gravação pedida terminou.
    Written,
    Error(String),
}

/// Ler e gravar tags NFC (o componente `Nfc`). O componente consulta [`NfcService::poll`]
/// num timer; ler só funciona com o app na frente.
pub trait NfcService {
    /// Há NFC aqui (o aparelho tem, ou há um leitor USB ligado).
    fn available(&self) -> bool;
    /// O NFC está ligado (no Android dá para desligar nas configurações).
    fn enabled(&self) -> bool {
        self.available()
    }
    /// Começa ou para de ler as tags que encostarem.
    fn set_reading(&mut self, on: bool);
    /// Grava a mensagem NDEF ([`crate::ndef::encode`]) na próxima tag que encostar; o
    /// resultado chega como [`NfcEvent::Written`] ou [`NfcEvent::Error`].
    fn write(&mut self, message: Vec<u8>);
    /// Desiste da gravação pendente.
    fn cancel_write(&mut self) {}
    /// O que aconteceu desde a última consulta.
    fn poll(&mut self) -> Vec<NfcEvent>;
}

/// Um aparelho Bluetooth achado.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct BleDevice {
    /// O endereço (Android, Windows, Linux) ou o identificador que o sistema dá (macOS, iOS,
    /// navegador).
    pub id: String,
    pub name: String,
    /// A força do sinal em dBm (0: o sistema não informou).
    pub rssi: i32,
}

/// Uma característica de um aparelho conectado.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct BleCharacteristic {
    pub service: String,
    pub characteristic: String,
    /// `read`, `write`, `write_without_response`, `notify`, `indicate`.
    pub properties: Vec<String>,
}

/// O que o [`BluetoothService`] tem para contar.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BluetoothEvent {
    Found(BleDevice),
    /// A procura parou (pedido, erro ou o sistema parou).
    ScanStopped,
    /// Conectado (BLE, com as características; o clássico, sem).
    Connected { device: String, services: Vec<BleCharacteristic> },
    Disconnected(String),
    /// Uma leitura pedida ou um aviso (`notify`/`indicate`).
    Value { device: String, characteristic: String, data: Vec<u8> },
    Written { device: String, characteristic: String },
    /// Bytes que chegaram pelo canal serial (clássico).
    Received { device: String, data: Vec<u8> },
    Error(String),
}

/// Bluetooth (o componente `Bluetooth`): BLE e o canal serial do clássico (SPP/RFCOMM). As
/// operações respondem depois, pelo [`BluetoothService::poll`]; os uuids chegam inteiros e
/// em minúsculas ([`ble_uuid`]).
pub trait BluetoothService {
    fn available(&self) -> bool;
    /// O Bluetooth está ligado.
    fn enabled(&self) -> bool {
        self.available()
    }
    /// Começa (com os serviços que o aparelho anuncia e parte do nome; vazios, todos) ou
    /// para de procurar.
    fn scan(&mut self, on: bool, services: Vec<String>, name: String);
    fn connect(&mut self, device: &str);
    fn disconnect(&mut self, device: &str);
    fn read(&mut self, device: &str, service: &str, characteristic: &str);
    fn write(&mut self, device: &str, service: &str, characteristic: &str, data: Vec<u8>, with_response: bool);
    /// Liga ou desliga os avisos de mudança (`notify` ou `indicate`).
    fn subscribe(&mut self, device: &str, service: &str, characteristic: &str, on: bool);
    /// Os aparelhos pareados (clássico): id e nome.
    fn paired(&mut self) -> Vec<(String, String)> {
        Vec::new()
    }
    /// Abre o canal serial (SPP) com um pareado; conecta depois ([`BluetoothEvent::Connected`]).
    fn open_serial(&mut self, device: &str) -> Result<(), String> {
        let _ = device;
        Err(crate::messages::t("o Bluetooth clássico não está disponível aqui"))
    }
    fn send_serial(&mut self, device: &str, data: Vec<u8>) -> Result<(), String> {
        let _ = (device, data);
        Err(crate::messages::t("o Bluetooth clássico não está disponível aqui"))
    }
    fn close_serial(&mut self, device: &str) {
        let _ = device;
    }
    /// O que aconteceu desde a última consulta.
    fn poll(&mut self) -> Vec<BluetoothEvent>;
}

/// O uuid inteiro e em minúsculas: a forma curta do Bluetooth SIG (`180D`, `0x2A37`) vira
/// `0000180d-0000-1000-8000-00805f9b34fb`.
pub fn ble_uuid(text: &str) -> String {
    let t = text.trim().trim_start_matches("0x").trim_start_matches("0X").to_ascii_lowercase();
    if (t.len() == 4 || t.len() == 8) && t.chars().all(|c| c.is_ascii_hexdigit()) {
        format!("{t:0>8}-0000-1000-8000-00805f9b34fb")
    } else {
        t
    }
}

/// Uma notificação push que chegou.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PushMessage {
    /// O id que o serviço deu (vazio se ele não deu).
    pub id: String,
    /// O título e o texto da parte "notification" (vazios numa mensagem só de dados).
    pub title: String,
    pub body: String,
    /// Os dados do app, como um objeto JSON (`{}` sem dados).
    pub data: String,
}

/// O que chega das notificações push.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PushEvent {
    /// O endereço deste aparelho no serviço (o token do FCM, o do APNs em hexadecimal, a
    /// assinatura do Web Push em JSON, o canal do WNS, o endpoint do UnifiedPush): o app
    /// manda ao servidor dele, que usa para enviar.
    Token(String),
    /// Uma mensagem chegou com o app aberto.
    Message(PushMessage),
    /// O usuário tocou na notificação (o app abriu ou voltou para a frente por ela).
    Opened(PushMessage),
    Error(String),
}

/// A configuração do registro. Cada serviço usa a parte dele: o FCM (Android), as quatro
/// primeiras, que estão no `google-services.json`; o Web Push, a chave VAPID pública.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PushConfig {
    /// O número do projeto do Firebase (`project_number`, o "sender id").
    pub sender_id: String,
    /// O id do projeto (`project_id`).
    pub project_id: String,
    /// O id do app no Firebase (`mobilesdk_app_id`: `1:123:android:abc`).
    pub app_id: String,
    /// A chave da API (`current_key`).
    pub api_key: String,
    /// A chave VAPID pública (base64url) do servidor que envia o Web Push.
    pub vapid_key: String,
    /// Com o app aberto, a mensagem com título também aparece como notificação do sistema
    /// (sem isso, só o `on_message`).
    pub show_in_foreground: bool,
}

/// Notificações push (o componente `PushNotifications`). O registro responde depois: o
/// token, as mensagens e os toques chegam por [`push_event`] (de qualquer thread, quando o
/// sistema chama) ou pelo [`PushService::poll`].
pub trait PushService {
    /// Há um serviço de push aqui (no Android, o Google Play Services; no navegador, a Push
    /// API e o service worker).
    fn available(&self) -> bool;
    /// O serviço: `fcm`, `apns`, `webpush`, `wns` ou `unifiedpush` (o servidor precisa saber
    /// por onde enviar).
    fn provider(&self) -> &'static str;
    /// Pede a permissão de notificar (onde é preciso) e registra; o token chega como
    /// [`PushEvent::Token`].
    fn register(&mut self, config: &PushConfig);
    /// Deixa de receber (o token deixa de valer).
    fn unregister(&mut self) {}
    /// A própria plataforma mostra a notificação da mensagem (Android, iOS, macOS,
    /// navegador). Sem isso (o WNS raw, o UnifiedPush), o componente mostra pelo
    /// [`NotificationService`] com o `show_in_foreground`.
    fn shows_notifications(&self) -> bool {
        true
    }
    /// O que aconteceu desde a última consulta (as plataformas que guardam numa fila).
    fn poll(&mut self) -> Vec<PushEvent> {
        Vec::new()
    }
}

static PUSH_EVENTS: std::sync::Mutex<Vec<PushEvent>> = std::sync::Mutex::new(Vec::new());

/// A plataforma entrega um evento de push (de qualquer thread). Fica guardado até o
/// `PushNotifications` pegar: o toque que abriu o app chega antes de o form existir.
pub fn push_event(event: PushEvent) {
    if let Ok(mut list) = PUSH_EVENTS.lock() {
        list.push(event);
    }
}

/// Tira da fila os eventos de push (o `PushNotifications` chama).
pub fn take_push_events() -> Vec<PushEvent> {
    PUSH_EVENTS.lock().map(|mut list| std::mem::take(&mut *list)).unwrap_or_default()
}

/// Uma linha de evento de push no formato que as plataformas com código fora do Rust usam
/// (o Java do Android, o service worker): campos separados por tab, `token	<token>`,
/// `message	<id>	<título>	<texto>	<dados JSON>`, `opened	...` e `error	<mensagem>`.
pub fn parse_push_line(line: &str) -> Option<PushEvent> {
    let mut f = line.split('\t');
    let kind = f.next()?;
    let mut next = || f.next().unwrap_or_default().to_string();
    match kind {
        "token" => Some(PushEvent::Token(next())),
        "error" => Some(PushEvent::Error(next())),
        "message" | "opened" => {
            let mut m = PushMessage { id: next(), title: next(), body: next(), data: next() };
            if m.data.trim().is_empty() {
                m.data = "{}".into();
            }
            Some(if kind == "message" { PushEvent::Message(m) } else { PushEvent::Opened(m) })
        }
        _ => None,
    }
}

/// O valor de `key` num objeto JSON de um nível (os dados do push são texto → texto): o
/// texto sem as aspas, ou o valor como está (número, `true`, objeto).
pub fn json_field(json: &str, key: &str) -> Option<String> {
    let chars: Vec<char> = json.chars().collect();
    let mut i = 0;
    let skip_ws = |i: &mut usize| {
        while *i < chars.len() && chars[*i].is_whitespace() {
            *i += 1;
        }
    };
    // Lê um texto entre aspas a partir de `i` (em cima da aspa).
    let read_string = |i: &mut usize| -> Option<String> {
        if chars.get(*i) != Some(&'"') {
            return None;
        }
        *i += 1;
        let mut out = String::new();
        while *i < chars.len() {
            let c = chars[*i];
            *i += 1;
            match c {
                '"' => return Some(out),
                '\\' => {
                    let e = *chars.get(*i)?;
                    *i += 1;
                    match e {
                        'n' => out.push('\n'),
                        't' => out.push('\t'),
                        'r' => out.push('\r'),
                        'b' => out.push('\u{8}'),
                        'f' => out.push('\u{c}'),
                        'u' => {
                            let hex: String = chars.get(*i..*i + 4)?.iter().collect();
                            *i += 4;
                            let mut code = u32::from_str_radix(&hex, 16).ok()?;
                            // Par substituto (emoji): 😀.
                            if (0xD800..0xDC00).contains(&code) && chars.get(*i) == Some(&'\\') && chars.get(*i + 1) == Some(&'u') {
                                let low: String = chars.get(*i + 2..*i + 6)?.iter().collect();
                                if let Ok(low) = u32::from_str_radix(&low, 16) {
                                    *i += 6;
                                    code = 0x10000 + ((code - 0xD800) << 10) + (low.wrapping_sub(0xDC00) & 0x3FF);
                                }
                            }
                            out.push(char::from_u32(code).unwrap_or('\u{FFFD}'));
                        }
                        other => out.push(other),
                    }
                }
                c => out.push(c),
            }
        }
        None
    };
    skip_ws(&mut i);
    if chars.get(i) != Some(&'{') {
        return None;
    }
    i += 1;
    loop {
        skip_ws(&mut i);
        if chars.get(i) == Some(&'}') || i >= chars.len() {
            return None;
        }
        let name = read_string(&mut i)?;
        skip_ws(&mut i);
        if chars.get(i) != Some(&':') {
            return None;
        }
        i += 1;
        skip_ws(&mut i);
        let value = if chars.get(i) == Some(&'"') {
            read_string(&mut i)?
        } else {
            // Número, literal, objeto ou lista: até a vírgula ou o fecha do mesmo nível.
            let start = i;
            let (mut depth, mut quoted) = (0i32, false);
            while i < chars.len() {
                let c = chars[i];
                if quoted {
                    if c == '\\' {
                        i += 1;
                    } else if c == '"' {
                        quoted = false;
                    }
                } else {
                    match c {
                        '"' => quoted = true,
                        '{' | '[' => depth += 1,
                        '}' | ']' if depth == 0 => break,
                        '}' | ']' => depth -= 1,
                        ',' if depth == 0 => break,
                        _ => {}
                    }
                }
                i += 1;
            }
            chars[start..i.min(chars.len())].iter().collect::<String>().trim().to_string()
        };
        if name == key {
            return Some(value);
        }
        skip_ws(&mut i);
        if chars.get(i) == Some(&',') {
            i += 1;
        }
    }
}

/// A mensagem de um push que chega cru (o WNS do tipo raw, o UnifiedPush, o Web Push): um
/// objeto JSON com `title`, `body`, `id` e `data` (objeto), ou no formato do FCM
/// (`{"notification": {"title", "body"}, "data": {...}}`). O que não for JSON vira o texto.
pub fn push_message_from_payload(text: &str) -> PushMessage {
    let text = text.trim();
    if !text.starts_with('{') {
        return PushMessage { body: text.to_string(), data: "{}".into(), ..Default::default() };
    }
    let notification = json_field(text, "notification").unwrap_or_default();
    let pick = |key: &str| json_field(text, key).or_else(|| json_field(&notification, key)).unwrap_or_default();
    let data = json_field(text, "data").filter(|d| d.trim_start().starts_with('{')).unwrap_or_else(|| "{}".into());
    PushMessage { id: pick("id"), title: pick("title"), body: pick("body"), data }
}

/// Um texto de modelo com os dados de uma mensagem: `{chave}` vira o valor da chave no
/// objeto JSON `data` (vazio se não houver); `{{` e `}}` são as chaves literais.
pub fn render_template(template: &str, data: &str) -> String {
    let mut out = String::new();
    let mut rest = template;
    while let Some(i) = rest.find(['{', '}']) {
        out.push_str(&rest[..i]);
        let tail = &rest[i..];
        if tail.starts_with("{{") || tail.starts_with("}}") {
            out.push_str(&tail[..1]);
            rest = &tail[2..];
        } else if let (true, Some(end)) = (tail.starts_with('{'), tail.find('}')) {
            out.push_str(&json_field(data, tail[1..end].trim()).unwrap_or_default());
            rest = &tail[end + 1..];
        } else {
            out.push_str(&tail[..1]);
            rest = &tail[1..];
        }
    }
    out.push_str(rest);
    out
}

/// Um filtro de dados (`kind=call;priority=high`): todas as condições valem (`chave=valor`,
/// `chave!=valor`, `chave` sozinha: existe e não é vazia). Vazio: aceita tudo.
pub fn data_matches(filter: &str, data: &str) -> bool {
    filter.split(';').map(str::trim).filter(|c| !c.is_empty()).all(|c| {
        if let Some((k, v)) = c.split_once("!=") {
            json_field(data, k.trim()).unwrap_or_default() != v.trim()
        } else if let Some((k, v)) = c.split_once('=') {
            json_field(data, k.trim()).unwrap_or_default() == v.trim()
        } else {
            json_field(data, c).is_some_and(|v| !v.is_empty())
        }
    })
}

/// O texto em JSON (com as aspas), para as plataformas que montam JSON à mão.
pub fn json_quote(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + 2);
    out.push('"');
    for c in text.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// O que volta de uma ação executada pela plataforma.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActionEvent {
    /// `open` (o app abriu pela ação), `button` (um botão da notificação), `click` (o corpo
    /// da notificação), `timeout` (passou o tempo sem resposta) ou `trigger` (uma regra do
    /// push disparou).
    pub kind: String,
    /// O caminho do componente (`frmPrincipal.wake1`; o `PushTrigger` no `trigger`).
    pub component: String,
    /// O id do botão (no `button`).
    pub detail: String,
    /// Os dados da mensagem (JSON).
    pub data: String,
}

/// Uma linha de evento de ação (`tipo	componente	detalhe	dados`), como o Java manda.
pub fn parse_action_line(line: &str) -> Option<ActionEvent> {
    let mut f = line.split('\t');
    let kind = f.next().filter(|k| !k.is_empty())?.to_string();
    let component = f.next()?.to_string();
    let detail = f.next().unwrap_or_default().to_string();
    let data = f.next().filter(|d| !d.trim().is_empty()).unwrap_or("{}").to_string();
    Some(ActionEvent { kind, component, detail, data })
}

/// As ações do aparelho executadas pela própria plataforma (o Android, com o app fechado e a
/// tela bloqueada). A ação vai como JSON (o `spec` que o componente monta: `type` `wake` ou
/// `notify`, os textos de modelo, os botões...); os dados, como o objeto JSON da mensagem.
pub trait ActionService {
    fn execute(&mut self, spec: &str, data: &str) -> Result<(), String>;
    /// Para a ação do componente (o toque, a notificação, o app por cima do bloqueio).
    fn stop(&mut self, component: &str);
    /// Grava as regras do `PushTrigger` (uma lista JSON) para valerem com o app fechado: a
    /// mensagem que casa executa a ação ali mesmo, antes de o app abrir.
    fn set_push_rules(&mut self, rules: &str);
    /// O que aconteceu desde a última consulta (também o que aconteceu com o app fechado).
    fn poll(&mut self) -> Vec<ActionEvent>;
}


/// Fuso horário: a plataforma liga uma função que devolve, para um instante UTC (segundos
/// desde 1970), quantos segundos o horário local está à frente (com horário de verão). Sem
/// ela (testes, servidor), o horário local é o UTC.
static LOCAL_OFFSET: std::sync::RwLock<Option<fn(i64) -> i64>> = std::sync::RwLock::new(None);

pub fn set_local_offset_fn(f: fn(i64) -> i64) {
    if let Ok(mut slot) = LOCAL_OFFSET.write() {
        *slot = Some(f);
    }
}

/// O fuso fixado pelo projeto (`time_zone` no `.aylisproj`): vale no lugar do fuso do
/// aparelho, para todos verem a hora da empresa.
static PROJECT_OFFSET: std::sync::RwLock<Option<fn(i64) -> i64>> = std::sync::RwLock::new(None);

/// O projeto fixa o fuso (veja [`PROJECT_OFFSET`]); `None` volta ao do aparelho.
pub fn set_project_offset_fn(f: Option<fn(i64) -> i64>) {
    if let Ok(mut slot) = PROJECT_OFFSET.write() {
        *slot = f;
    }
}

/// Segundos que o horário local está à frente do UTC no instante `utc`: o fuso do projeto,
/// se houver, senão o do aparelho.
pub fn local_offset(utc: i64) -> i64 {
    let project = PROJECT_OFFSET.read().ok().and_then(|f| *f);
    project.or_else(|| LOCAL_OFFSET.read().ok().and_then(|f| *f)).map_or(0, |f| f(utc))
}
