//! Os monitores: a plataforma informa a lista
//! ([`Ui::set_monitors`]) e em qual deles está a janela de cada form; o app lê com
//! [`Ui::monitors`] e [`Ui::form_monitor`] (nas outras linguagens, pelo componente `Screen`).
//! Coordenadas em pixels físicos da área de trabalho, como o sistema dá (o monitor à esquerda
//! do principal tem `x` negativo).

use crate::component::ComponentId;
use crate::forms::WindowPlacement;
use crate::ui::Ui;

/// Um modo de vídeo que o monitor aceita.
/// O registro de um modo de vídeo no acesso tipado (o `modes` do [`MONITOR_RECORD`]).
pub const VIDEO_MODE_RECORD: crate::prop::RecordInfo = crate::prop::RecordInfo {
    name: "VideoMode",
    fields: &[("width", crate::PropKind::Int), ("height", crate::PropKind::Int), ("bit_depth", crate::PropKind::Int), ("refresh_hz", crate::PropKind::Float)],
};

/// O registro de um monitor no acesso tipado (o resultado do `Screen.monitors`, o JSON de
/// [`Monitor::to_json`]).
pub const MONITOR_RECORD: crate::prop::RecordInfo = crate::prop::RecordInfo {
    name: "Monitor",
    fields: &[
        ("index", crate::PropKind::Int),
        ("name", crate::PropKind::String),
        ("device", crate::PropKind::String),
        ("device_path", crate::PropKind::String),
        ("primary", crate::PropKind::Bool),
        ("x", crate::PropKind::Int),
        ("y", crate::PropKind::Int),
        ("width", crate::PropKind::Int),
        ("height", crate::PropKind::Int),
        ("work_x", crate::PropKind::Int),
        ("work_y", crate::PropKind::Int),
        ("work_width", crate::PropKind::Int),
        ("work_height", crate::PropKind::Int),
        ("scale", crate::PropKind::Float),
        ("refresh_hz", crate::PropKind::Float),
        ("modes", crate::PropKind::RecordList(&VIDEO_MODE_RECORD)),
    ],
};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct VideoMode {
    pub width: u32,
    pub height: u32,
    pub bit_depth: u16,
    pub refresh_hz: f64,
}

/// Um monitor.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Monitor {
    /// O nome do aparelho ("XB273K V5"); sem ele, o do sistema.
    pub name: String,
    /// O nome do sistema (`\\.\DISPLAY1` no Windows): não muda enquanto o monitor fica ligado
    /// na mesma saída, por isso é o que se guarda para reabrir nele.
    pub device: String,
    /// A identidade do aparelho, que sobrevive à troca de cabo de porta e ao monitor desligado e
    /// religado (no Windows, o caminho do aparelho, `\\?\DISPLAY#DELD19B#...`). Texto opaco: só
    /// para comparar com um guardado. Vazio onde o sistema não dá uma identidade (o navegador e
    /// o celular); no macOS e no Linux, o que o sistema dá (o número do monitor; o nome da saída,
    /// `DP-1`), menos estável. O `monitor` do form continua o índice.
    pub device_path: String,
    pub primary: bool,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    /// A área útil (sem a barra de tarefas); igual à do monitor onde o sistema não diz.
    pub work_x: i32,
    pub work_y: i32,
    pub work_width: u32,
    pub work_height: u32,
    /// A escala do sistema (1,5 = 150%).
    pub scale: f64,
    /// A taxa de atualização (0 = o sistema não diz, como no navegador).
    pub refresh_hz: f64,
    pub modes: Vec<VideoMode>,
}

impl Monitor {
    /// A área útil, em pixels físicos: `(x, y, largura, altura)`.
    pub fn work_area(&self) -> (i32, i32, u32, u32) {
        (self.work_x, self.work_y, self.work_width, self.work_height)
    }

    /// Em JSON (o `Screen` das outras linguagens), com os nomes dos campos em inglês.
    pub fn to_json(&self, index: usize) -> String {
        let text = |s: &str| {
            let mut out = String::from("\"");
            for c in s.chars() {
                match c {
                    '"' => out.push_str("\\\""),
                    '\\' => out.push_str("\\\\"),
                    c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
                    c => out.push(c),
                }
            }
            out.push('"');
            out
        };
        let modes: Vec<String> = self
            .modes
            .iter()
            .map(|m| format!("{{\"width\":{},\"height\":{},\"bit_depth\":{},\"refresh_hz\":{}}}", m.width, m.height, m.bit_depth, m.refresh_hz))
            .collect();
        format!(
            "{{\"index\":{index},\"name\":{},\"device\":{},\"device_path\":{},\"primary\":{},\"x\":{},\"y\":{},\"width\":{},\"height\":{},\"work_x\":{},\"work_y\":{},\"work_width\":{},\"work_height\":{},\"scale\":{},\"refresh_hz\":{},\"modes\":[{}]}}",
            text(&self.name),
            text(&self.device),
            text(&self.device_path),
            self.primary,
            self.x,
            self.y,
            self.width,
            self.height,
            self.work_x,
            self.work_y,
            self.work_width,
            self.work_height,
            self.scale,
            self.refresh_hz,
            modes.join(",")
        )
    }
}

/// Onde a janela de um form estava, para reabrir nela (o `remember_placement` do form).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SavedPlacement {
    /// O `device_path` do monitor (ou, sem ele, o `device`).
    pub monitor: String,
    pub placement: WindowPlacement,
}

/// A posição guardada, se ainda serve: o monitor dela continua ligado e a barra de título
/// fica à vista (pelo menos 120 x 40 pixels dentro da área útil de algum monitor). Senão,
/// `None` (o form abre como se não houvesse posição guardada, no monitor principal), como o
/// Windows faz com a janela de um monitor desligado.
pub fn restore_placement(saved: &SavedPlacement, monitors: &[Monitor]) -> Option<WindowPlacement> {
    let p = saved.placement;
    monitors.iter().find(|m| m.device == saved.monitor || (!m.device_path.is_empty() && m.device_path == saved.monitor))?;
    let visible = monitors.iter().any(|m| {
        let (x, y, w, h) = m.work_area();
        let left = p.x.max(x);
        let right = (p.x + p.width.max(120) as i32).min(x + w as i32);
        let top = p.y.max(y);
        let bottom = (p.y + 40).min(y + h as i32);
        right - left >= 120 && bottom - top >= 40
    });
    visible.then_some(p)
}

impl Ui {
    /// Os monitores, na ordem do sistema (vazio antes de a plataforma informar).
    pub fn monitors(&self) -> &[Monitor] {
        &self.monitors
    }

    /// Conta as mudanças da lista (um monitor ligado, desligado ou com outra resolução): o
    /// `Screen` compara para disparar o `on_change`.
    pub fn monitors_version(&self) -> u64 {
        self.monitors_version
    }

    /// A plataforma informa os monitores (só conta como mudança se a lista mudou).
    pub fn set_monitors(&mut self, monitors: Vec<Monitor>) {
        if monitors != self.monitors {
            self.monitors = monitors;
            self.monitors_version += 1;
        }
    }

    /// O monitor (índice em [`Ui::monitors`]) em que está a janela do form.
    pub fn form_monitor(&self, form: ComponentId) -> Option<usize> {
        self.form_monitors.get(&self.root_of(form)).copied()
    }

    /// A janela do form está minimizada (a plataforma informa; o `on_minimize` e o `on_restore`
    /// avisam a mudança).
    pub fn is_minimized(&self, form: ComponentId) -> bool {
        self.minimized_forms.contains(&self.root_of(form))
    }

    /// A plataforma informa que a janela do form minimizou ou voltou.
    pub fn set_form_minimized(&mut self, form: ComponentId, minimized: bool) {
        if minimized {
            self.minimized_forms.insert(form);
        } else {
            self.minimized_forms.remove(&form);
        }
    }

    /// A plataforma informa em que monitor está a janela do form (ao criar a janela, ao mover
    /// e na varredura periódica). Um monitor diferente do anterior dispara o
    /// `on_monitor_change` do form com o índice novo; a primeira informação da janela não (é o
    /// monitor de abrir, que o `on_window_created` já lê), a não ser que ela tenha vindo sem
    /// monitor (no Wayland, ele só se sabe depois do primeiro quadro). Ficar sem monitor não
    /// dispara (o `Screen.on_change` avisa o monitor desligado). O índice é a ordem do sistema:
    /// um monitor ligado ou desligado pode renumerar a lista e disparar o evento sem a janela
    /// sair do lugar.
    pub fn set_form_monitor(&mut self, form: ComponentId, monitor: Option<usize>) {
        let first = self.note_form_monitor_reported(form);
        let old = match monitor {
            Some(m) => self.form_monitors.insert(form, m),
            None => self.form_monitors.remove(&form),
        };
        if let Some(m) = monitor.filter(|&m| old != Some(m) && !first && self.is_open(form)) {
            self.fire(form, "on_monitor_change", &crate::EventArgs::Value(crate::PropValue::Int(m as i64)));
        }
    }
}

/// O mouse na tela ([`Ui::cursor_position`]).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CursorPosition {
    /// Em pixels físicos da área de trabalho (como os monitores).
    pub x: i32,
    pub y: i32,
    /// O monitor sob o mouse (índice em [`Ui::monitors`]), se a lista o tem.
    pub monitor: Option<usize>,
    /// Em dp, a partir do canto do monitor (`(x - monitor.x) / escala`): o ponto para abrir
    /// um popup ou um form ali. Sem monitor, os pixels.
    pub dp_x: f32,
    pub dp_y: f32,
}

/// O registro do mouse na tela no acesso tipado (o `Screen.cursor_position`).
pub const CURSOR_POSITION_RECORD: crate::prop::RecordInfo = crate::prop::RecordInfo {
    name: "CursorPosition",
    fields: &[
        ("x", crate::PropKind::Int),
        ("y", crate::PropKind::Int),
        ("monitor", crate::PropKind::Int),
        ("dp_x", crate::PropKind::Float),
        ("dp_y", crate::PropKind::Float),
    ],
};

impl CursorPosition {
    /// Em JSON (o `Screen.cursor_position`); `monitor` é -1 fora dos monitores conhecidos.
    pub fn to_json(&self) -> String {
        format!(
            "{{\"x\":{},\"y\":{},\"monitor\":{},\"dp_x\":{},\"dp_y\":{}}}",
            self.x,
            self.y,
            self.monitor.map_or(-1, |m| m as i64),
            self.dp_x,
            self.dp_y
        )
    }
}

/// O que o `Ui` guarda da área de trabalho (os "Efeitos de transparência" e a captura).
#[derive(Default)]
struct DesktopState {
    /// O último valor informado pela plataforma (`None`: nunca informou).
    transparency: Option<bool>,
    /// Conta as mudanças da transparência (o `Screen` compara para o `on_transparency_change`).
    transparency_version: u64,
    /// A captura de monitor ligada pelo projeto (`allow_screen_capture`).
    allow_capture: bool,
    /// A sessão do usuário (bloqueada, conectada), a última informada (`None`: nunca informou).
    session: Option<(bool, bool)>,
    /// Conta as mudanças da sessão (o `Screen` compara para o `on_session_change`).
    session_version: u64,
}

/// O que a captura devolve, na thread da tela.
type CaptureDone = Box<dyn FnOnce(&mut Ui, Result<crate::Image, String>)>;

/// As capturas esperando a resposta da plataforma (o `done` fica na thread da tela; a
/// resposta pode vir de outra).
#[derive(Default)]
struct CaptureWaits {
    next: u64,
    waits: Vec<(u64, CaptureDone)>,
}

impl Ui {
    /// O mouse na tela, também fora das janelas do app (sobre a bandeja, sobre outro
    /// programa): em pixels físicos, em dp no monitor, e o monitor. `None` onde o sistema não
    /// diz: Wayland, celular e navegador.
    pub fn cursor_position(&self) -> Option<CursorPosition> {
        let (x, y) = self.services().desktop.as_ref()?.cursor_position()?;
        let monitor = self.monitors.iter().position(|m| x >= m.x && y >= m.y && x < m.x + m.width as i32 && y < m.y + m.height as i32);
        let (dp_x, dp_y) = match monitor.map(|i| &self.monitors[i]) {
            Some(m) if m.scale > 0.0 => (((x - m.x) as f64 / m.scale) as f32, ((y - m.y) as f64 / m.scale) as f32),
            _ => (x as f32, y as f32),
        };
        Some(CursorPosition { x, y, monitor, dp_x, dp_y })
    }

    /// O form do app à vista no ponto da tela (pixels físicos), pela ordem real das janelas (a
    /// de cima, quando duas se cobrem): a mesma busca do soltar entre janelas. `None` fora das
    /// janelas do app, com outro programa por cima, com a janela minimizada, e onde o sistema
    /// não diz (Wayland, celular, navegador).
    pub fn form_at_screen(&self, x: i32, y: i32) -> Option<ComponentId> {
        let form = self.services().desktop.as_ref()?.form_at(x, y)?;
        self.is_open(form).then_some(form)
    }

    /// Os "Efeitos de transparência" do sistema estão ligados (no Windows, Personalização →
    /// Cores): desligados, o vidro (`WindowSettings.blur`, o Acrylic) sai numa cor sólida, e o
    /// app pode trocar por um fundo opaco. Onde o sistema não informa, `true`. A mudança chega
    /// pelo `on_transparency_change` do `Screen`.
    pub fn system_transparency(&self) -> bool {
        self.extension_ref::<DesktopState>()
            .and_then(|d| d.transparency)
            .or_else(|| self.services().desktop.as_ref().and_then(|d| d.transparency()))
            .unwrap_or(true)
    }

    /// A plataforma informa os "Efeitos de transparência" (ao abrir e a cada mudança).
    pub fn set_system_transparency(&mut self, on: bool) {
        let state = self.extension::<DesktopState>();
        if state.transparency != Some(on) {
            if state.transparency.is_some() {
                state.transparency_version += 1;
            }
            state.transparency = Some(on);
        }
    }

    /// Conta as mudanças dos "Efeitos de transparência" (o `Screen` compara para disparar o
    /// `on_transparency_change`).
    pub fn system_transparency_version(&self) -> u64 {
        self.extension_ref::<DesktopState>().map_or(0, |d| d.transparency_version)
    }

    /// A sessão do usuário está bloqueada (no Windows, Win+L ou a proteção de tela com senha):
    /// o app pode pausar o que só faz sentido com alguém olhando. A mudança chega pelo
    /// `on_session_change` do `Screen`. Só o Windows informa; nos outros sistemas, `false`.
    pub fn session_locked(&self) -> bool {
        self.session_state().0
    }

    /// A sessão do usuário está conectada à tela: `false` com a sessão remota desconectada (a
    /// janela do RDP fechada sem sair) ou deixada de lado pela troca rápida de usuário. A
    /// mudança chega pelo `on_session_change` do `Screen`. Só o Windows informa; nos outros
    /// sistemas, `true`.
    pub fn session_connected(&self) -> bool {
        self.session_state().1
    }

    /// Bloqueada e conectada: o último valor da plataforma, ou o que o serviço diz agora.
    fn session_state(&self) -> (bool, bool) {
        self.extension_ref::<DesktopState>()
            .and_then(|d| d.session)
            .or_else(|| self.services().desktop.as_ref().and_then(|d| d.session_state()))
            .unwrap_or((false, true))
    }

    /// A plataforma informa a sessão (ao abrir e a cada mudança): bloqueada e conectada.
    pub fn set_system_session(&mut self, locked: bool, connected: bool) {
        let state = self.extension::<DesktopState>();
        if state.session != Some((locked, connected)) {
            if state.session.is_some() {
                state.session_version += 1;
            }
            state.session = Some((locked, connected));
        }
    }

    /// Conta as mudanças da sessão (o `Screen` compara para disparar o `on_session_change`).
    pub fn system_session_version(&self) -> u64 {
        self.extension_ref::<DesktopState>().map_or(0, |d| d.session_version)
    }

    /// Liga a captura de monitor ([`Ui::capture_monitor`]): o projeto, pela opção
    /// `allow_screen_capture` (desligada por padrão, porque a imagem mostra o que está nos
    /// outros programas).
    pub fn set_allow_screen_capture(&mut self, on: bool) {
        self.extension::<DesktopState>().allow_capture = on;
    }

    /// A captura de monitor está ligada (`allow_screen_capture` do projeto).
    pub fn allow_screen_capture(&self) -> bool {
        self.extension_ref::<DesktopState>().is_some_and(|d| d.allow_capture)
    }

    /// A imagem do monitor `index` (em [`Ui::monitors`]), reduzida para no máximo `max_width`
    /// pixels de largura (0: inteira): a miniatura de cada tela numa configuração. Só com o
    /// `allow_screen_capture` do projeto. Assíncrona: `done` roda na thread da tela, com o erro
    /// se a opção está desligada, o monitor não existe, o sistema recusou (onde ele pergunta ao
    /// usuário: macOS, Wayland, celular, navegador) ou não tem o recurso.
    pub fn capture_monitor(&mut self, index: usize, max_width: u32, done: impl FnOnce(&mut Ui, Result<crate::Image, String>) + 'static) {
        let done: CaptureDone = Box::new(done);
        let refused = if !self.allow_screen_capture() {
            Some(crate::messages::t("A captura da tela está desligada no projeto (allow_screen_capture)."))
        } else if index >= self.monitors.len() {
            Some(crate::messages::tf("O monitor {0} não existe.", &[&index]))
        } else {
            None
        };
        if let Some(message) = refused {
            self.set_timeout(0, move |ui| done(ui, Err(message)));
            return;
        }
        let m = &self.monitors[index];
        let area = (m.x, m.y, m.width, m.height);
        let waits = self.extension::<CaptureWaits>();
        waits.next += 1;
        let token = waits.next;
        waits.waits.push((token, done));
        let poster = self.poster();
        let reply = Box::new(move |result: Result<crate::Image, String>| {
            poster.post(move |ui| {
                let waits = ui.extension::<CaptureWaits>();
                if let Some(i) = waits.waits.iter().position(|(t, _)| *t == token) {
                    let (_, done) = waits.waits.remove(i);
                    done(ui, result);
                }
            });
        });
        match self.services_mut().desktop.as_mut() {
            Some(d) => d.capture_monitor(area, max_width, reply),
            None => reply(Err(crate::messages::t("A captura da tela não existe neste sistema."))),
        }
    }
}
