//! Windows: as mensagens da janela (o app pode interceptar qualquer uma) e o botão na
//! barra de tarefas (progresso, indicador, botões na miniatura, tarefas do clique direito).
//!
//! Cada janela do aylis ganha uma "subclass" (`SetWindowSubclass`): as mensagens passam
//! antes por aqui, onde os ganchos do app ([`on_message`]) podem deixar seguir, impedir
//! (com um resultado) ou trocar a mensagem; e onde chegam os cliques nos botões da
//! miniatura.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Mutex;

use aylis_core::{ComponentId, TaskbarEvent, TaskbarProgress, TaskbarSpec, Ui};
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::System::Com::{CLSCTX_INPROC_SERVER, CoCreateInstance};
use windows::Win32::UI::Shell::{
    DefSubclassProc, ICustomDestinationList, IShellLinkW, ITaskbarList3, SetWindowSubclass, THB_FLAGS, THB_ICON,
    THB_TOOLTIP, THBF_ENABLED, THBN_CLICKED, THUMBBUTTON, TBPF_ERROR, TBPF_INDETERMINATE, TBPF_NOPROGRESS, TBPF_NORMAL, TBPF_PAUSED,
};
use windows::Win32::UI::Shell::Common::{IObjectArray, IObjectCollection};
use windows::Win32::UI::WindowsAndMessaging::{CreateIconFromResourceEx, DestroyIcon, HICON, LR_DEFAULTCOLOR, RegisterWindowMessageW, WM_COMMAND};
use windows::core::{GUID, HSTRING, Interface, PCWSTR};

/// Uma mensagem do Windows para a janela de um form.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WindowMessage {
    /// O form dono da janela.
    pub form: ComponentId,
    /// `WM_*` (os mais comuns estão em [`msg`]).
    pub message: u32,
    pub wparam: usize,
    pub lparam: isize,
}

/// O que fazer com a mensagem.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MessageReply {
    /// Segue o caminho normal (o próximo gancho, depois o aylis e o Windows).
    Default,
    /// Para aqui: a mensagem não é processada, e este é o resultado dela.
    Handled(isize),
    /// Troca a mensagem por outra (e segue).
    Replace { message: u32, wparam: usize, lparam: isize },
}

/// Mensagens e comandos comuns, para não precisar procurar os números.
pub mod msg {
    pub const WM_CLOSE: u32 = 0x0010;
    pub const WM_SIZE: u32 = 0x0005;
    pub const WM_MOVE: u32 = 0x0003;
    pub const WM_ACTIVATE: u32 = 0x0006;
    pub const WM_GETMINMAXINFO: u32 = 0x0024;
    pub const WM_SYSCOMMAND: u32 = 0x0112;
    pub const WM_POWERBROADCAST: u32 = 0x0218;
    pub const WM_DISPLAYCHANGE: u32 = 0x007E;
    pub const WM_SETTINGCHANGE: u32 = 0x001A;
    pub const WM_QUERYENDSESSION: u32 = 0x0011;
    /// No `WM_SYSCOMMAND`, o comando está em `wparam & 0xFFF0`.
    pub const SC_MINIMIZE: usize = 0xF020;
    pub const SC_MAXIMIZE: usize = 0xF030;
    pub const SC_RESTORE: usize = 0xF120;
    pub const SC_CLOSE: usize = 0xF060;
    pub const SC_MOVE: usize = 0xF010;
    pub const SC_SIZE: usize = 0xF000;
}

type Hook = Rc<dyn Fn(&WindowMessage) -> MessageReply>;

thread_local! {
    static HOOKS: RefCell<HashMap<ComponentId, Vec<Hook>>> = RefCell::new(HashMap::new());
    /// O `HWND` da janela de cada form aberto.
    static HWNDS: RefCell<HashMap<ComponentId, isize>> = RefCell::new(HashMap::new());
}

/// O `HWND` da janela do form (na thread da interface), para chamar a API do Windows
/// direto (o tema da barra de título, o selo na barra de tarefas). `None` antes de a janela
/// existir.
pub fn hwnd(form: ComponentId) -> Option<isize> {
    HWNDS.with(|h| h.borrow().get(&form).copied())
}

/// Cliques nos botões da miniatura: (form, botão), e a janela cujo botão na barra foi
/// (re)criado.
static THUMB_CLICKS: Mutex<Vec<(u64, usize)>> = Mutex::new(Vec::new());
static BUTTON_CREATED: Mutex<Vec<u64>> = Mutex::new(Vec::new());
static WAKE: Mutex<Option<winit::event_loop::EventLoopProxy<()>>> = Mutex::new(None);

/// Intercepta as mensagens do Windows da janela do form `form` (na thread da interface,
/// antes do processamento normal). Vários ganchos: o primeiro que não devolver `Default`
/// decide. Para efeitos na interface, use o `Poster` (o gancho não recebe o `Ui`).
///
/// ```no_run
/// # use aylis_platform_winit::win::{on_message, msg, MessageReply};
/// # let form = aylis_core::ComponentId::from_u64(1);
/// // Impede maximizar.
/// on_message(form, |m| {
///     if m.message == msg::WM_SYSCOMMAND && m.wparam & 0xFFF0 == msg::SC_MAXIMIZE {
///         MessageReply::Handled(0)
///     } else {
///         MessageReply::Default
///     }
/// });
/// ```
pub fn on_message(form: ComponentId, hook: impl Fn(&WindowMessage) -> MessageReply + 'static) {
    HOOKS.with(|h| h.borrow_mut().entry(form).or_default().push(Rc::new(hook)));
}

/// Tira os ganchos do form.
pub fn clear_message_hooks(form: ComponentId) {
    HOOKS.with(|h| h.borrow_mut().remove(&form));
}

fn taskbar_created() -> u32 {
    static ID: std::sync::OnceLock<u32> = std::sync::OnceLock::new();
    // SAFETY: registra (ou obtém) o número da mensagem pelo nome.
    *ID.get_or_init(|| unsafe { RegisterWindowMessageW(&HSTRING::from("TaskbarButtonCreated")) })
}

fn wake() {
    if let Ok(w) = WAKE.lock() {
        if let Some(p) = w.as_ref() {
            let _ = p.send_event(());
        }
    }
}

unsafe extern "system" fn subclass(hwnd: HWND, message: u32, wparam: WPARAM, lparam: LPARAM, _id: usize, form: usize) -> LRESULT {
    let form_id = ComponentId::from_u64(form as u64);
    if message == taskbar_created() {
        if let Ok(mut q) = BUTTON_CREATED.lock() {
            q.push(form as u64);
        }
        wake();
    } else if message == WM_COMMAND && (wparam.0 >> 16) as u32 == THBN_CLICKED {
        if let Ok(mut q) = THUMB_CLICKS.lock() {
            q.push((form as u64, wparam.0 & 0xFFFF));
        }
        wake();
    }
    let hooks: Vec<Hook> = HOOKS.with(|h| h.borrow().get(&form_id).cloned().unwrap_or_default());
    let mut m = WindowMessage { form: form_id, message, wparam: wparam.0, lparam: lparam.0 };
    for hook in hooks {
        match hook(&m) {
            MessageReply::Default => {}
            MessageReply::Handled(result) => return LRESULT(result),
            MessageReply::Replace { message, wparam, lparam } => m = WindowMessage { message, wparam, lparam, ..m },
        }
    }
    // SAFETY: segue para o processamento normal (o do winit).
    unsafe { DefSubclassProc(hwnd, m.message, WPARAM(m.wparam), LPARAM(m.lparam)) }
}

/// A janela de um form: põe a subclass (uma vez).
pub(crate) fn attach(window: &winit::window::Window, form: ComponentId, proxy: &winit::event_loop::EventLoopProxy<()>) {
    if let Ok(mut w) = WAKE.lock() {
        if w.is_none() {
            *w = Some(proxy.clone());
        }
    }
    taskbar_created();
    if let Some(hwnd) = hwnd_of(window) {
        HWNDS.with(|h| h.borrow_mut().insert(form, hwnd.0 as isize));
        // SAFETY: a janela existe; a subclass sai junto com ela.
        unsafe {
            let _ = SetWindowSubclass(hwnd, Some(subclass), 1, form.to_u64() as usize);
        }
    }
}

/// Mostra ou esconde pela API do Windows (além do winit, que guarda o estado dele e não
/// age quando acha que nada mudou).
pub(crate) fn set_shown(window: &winit::window::Window, shown: bool) {
    use windows::Win32::UI::WindowsAndMessaging::{IsWindowVisible, SW_HIDE, SW_SHOW, ShowWindow};
    let Some(hwnd) = hwnd_of(window) else { return };
    // SAFETY: a janela existe (é a do form).
    unsafe {
        if IsWindowVisible(hwnd).as_bool() != shown {
            let _ = ShowWindow(hwnd, if shown { SW_SHOW } else { SW_HIDE });
        }
    }
}

/// Avisa o winit que a janela está à vista quando o app a mostrou pela API do Windows (o
/// NoOBS, ao voltar da bandeja). Sem isso, o winit acha que ela está escondida e, ao trocar o
/// estilo (a tela cheia), regrava sem o `WS_VISIBLE`: a janela sumia, até da barra de tarefas.
pub(crate) fn sync_shown(window: &winit::window::Window) {
    use windows::Win32::UI::WindowsAndMessaging::IsWindowVisible;
    let Some(hwnd) = hwnd_of(window) else { return };
    // SAFETY: a janela existe (é a do form).
    if unsafe { IsWindowVisible(hwnd) }.as_bool() {
        window.set_visible(true);
    }
}

/// Os pontos por onde o mouse passou entre dois eventos de movimento. O Windows junta os
/// movimentos e entrega uns 60 por segundo (ou menos, com o quadro lento); o histórico do
/// mouse (`GetMouseMovePointsEx`, os últimos 64 pontos na taxa do mouse) devolve os do
/// meio, para um traço rápido de desenho sair fiel em vez de em retas.
#[derive(Default)]
pub(crate) struct MouseTrail {
    /// O último ponto entregue: x e y na tela (pixels) e o horário do Windows.
    last: Option<(i32, i32, u32)>,
}

impl MouseTrail {
    /// Os pontos entre o evento anterior e este (em `x`, `y`, pixels da área da janela),
    /// em ordem e sem o próprio ponto do evento, na mesma escala da posição recebida.
    pub(crate) fn points_before(&mut self, window: &winit::window::Window, x: f64, y: f64) -> Vec<(f64, f64)> {
        use windows::Win32::Foundation::POINT;
        use windows::Win32::Graphics::Gdi::ClientToScreen;
        use windows::Win32::UI::Input::KeyboardAndMouse::{GMMP_USE_DISPLAY_POINTS, GetMouseMovePointsEx, MOUSEMOVEPOINT};
        let Some(hwnd) = hwnd_of(window) else { return Vec::new() };
        let mut origin = POINT { x: 0, y: 0 };
        if !unsafe { ClientToScreen(hwnd, &mut origin) }.as_bool() {
            return Vec::new();
        }
        let (sx, sy) = (x.round() as i32 + origin.x, y.round() as i32 + origin.y);
        // As coordenadas vão em 16 bits: negativas (monitor à esquerda) voltam com 65536 a mais.
        let input = MOUSEMOVEPOINT { x: sx & 0xFFFF, y: sy & 0xFFFF, time: 0, dwExtraInfo: 0 };
        let mut history = [MOUSEMOVEPOINT::default(); 64];
        let n = unsafe { GetMouseMovePointsEx(std::mem::size_of::<MOUSEMOVEPOINT>() as u32, &input, &mut history, GMMP_USE_DISPLAY_POINTS) };
        if n <= 0 {
            self.last = None;
            return Vec::new();
        }
        let fix = |v: i32| if v > 32767 { v - 65536 } else { v };
        // Do mais novo (o deste evento) para o mais velho.
        let points: Vec<(i32, i32, u32)> = history[..n as usize].iter().map(|p| (fix(p.x), fix(p.y), p.time)).collect();
        let mut between = Vec::new();
        if let Some(last) = self.last {
            for &p in &points[1..] {
                if p == last || p.2 < last.2 {
                    break;
                }
                between.push(p);
            }
        }
        self.last = Some(points[0]);
        between.reverse();
        between.dedup_by_key(|p| (p.0, p.1));
        between.into_iter().filter(|p| (p.0, p.1) != (sx, sy)).map(|p| ((p.0 - origin.x) as f64, (p.1 - origin.y) as f64)).collect()
    }

    /// O mouse saiu da janela: o próximo evento começa sem pontos do meio.
    pub(crate) fn reset(&mut self) {
        self.last = None;
    }
}

pub(crate) fn hwnd_of(window: &winit::window::Window) -> Option<HWND> {
    use winit::raw_window_handle::{HasWindowHandle, RawWindowHandle};
    match window.window_handle().ok()?.as_raw() {
        RawWindowHandle::Win32(h) => Some(HWND(h.hwnd.get() as *mut _)),
        _ => None,
    }
}

/// Troca a janela dona de `window` (`None`: sem dona). O Windows destrói as janelas possuídas
/// junto com a dona: antes de fechar um form, as janelas que ele possui passam para outra
/// (senão a do form que continua aberto morre por baixo do winit, e o próximo `inner_size`
/// dela derruba o app: o ERP, ao fechar a segunda de duas telas, em 08/11).
pub(crate) fn set_owner(window: &winit::window::Window, owner: Option<&winit::window::Window>) {
    use windows::Win32::UI::WindowsAndMessaging::GWLP_HWNDPARENT;
    let Some(hwnd) = hwnd_of(window) else { return };
    let owner = owner.and_then(hwnd_of).map_or(0, |h| h.0 as isize);
    // SAFETY: as duas são janelas vivas deste processo (as do winit, guardadas pelo app), e o
    // GWLP_HWNDPARENT de uma janela de topo é a dona dela (o uso documentado da chamada).
    unsafe {
        #[cfg(target_pointer_width = "64")]
        windows::Win32::UI::WindowsAndMessaging::SetWindowLongPtrW(hwnd, GWLP_HWNDPARENT, owner);
        #[cfg(target_pointer_width = "32")]
        windows::Win32::UI::WindowsAndMessaging::SetWindowLongW(hwnd, GWLP_HWNDPARENT, owner as i32);
    }
}

/// Onde a janela fica quando não está minimizada (o retângulo "normal" do Windows, com a
/// borda): minimizada, a posição dela é ≈ -32000 e um diálogo centrado sobre ela abria fora
/// da tela, com a IDE travada por um modal invisível (conferência de 08/11).
pub(crate) fn normal_rect(window: &winit::window::Window) -> Option<(winit::dpi::PhysicalPosition<i32>, winit::dpi::PhysicalSize<u32>)> {
    use windows::Win32::UI::WindowsAndMessaging::{GetWindowPlacement, WINDOWPLACEMENT};
    let hwnd = hwnd_of(window)?;
    let mut placement = WINDOWPLACEMENT { length: std::mem::size_of::<WINDOWPLACEMENT>() as u32, ..Default::default() };
    // SAFETY: a janela é deste processo e está viva (o winit a guarda), e a estrutura tem o
    // `length` que a chamada exige.
    unsafe { GetWindowPlacement(hwnd, &mut placement) }.ok()?;
    // O retângulo normal vem em coordenadas da área de trabalho; com a barra de tarefas em cima
    // ou à esquerda há um deslocamento, que para centralizar um diálogo não pesa.
    let r = placement.rcNormalPosition;
    (r.right > r.left && r.bottom > r.top).then(|| (winit::dpi::PhysicalPosition::new(r.left, r.top), winit::dpi::PhysicalSize::new((r.right - r.left) as u32, (r.bottom - r.top) as u32)))
}

/// Um ícone do Windows a partir de RGBA (um recurso de ícone montado na memória: cabeçalho
/// BITMAPINFO, pixels BGRA de baixo para cima e a máscara).
pub(crate) fn hicon_from_rgba(rgba: &[u8], w: u32, h: u32) -> Option<HICON> {
    let mut res = Vec::with_capacity(40 + rgba.len() + (h * w.div_ceil(32) * 4) as usize);
    let push32 = |v: &mut Vec<u8>, x: u32| v.extend_from_slice(&x.to_le_bytes());
    push32(&mut res, 40);
    push32(&mut res, w);
    push32(&mut res, h * 2);
    res.extend_from_slice(&1u16.to_le_bytes());
    res.extend_from_slice(&32u16.to_le_bytes());
    for _ in 0..6 {
        push32(&mut res, 0);
    }
    for y in (0..h).rev() {
        for x in 0..w {
            let i = ((y * w + x) * 4) as usize;
            res.extend_from_slice(&[rgba[i + 2], rgba[i + 1], rgba[i], rgba[i + 3]]);
        }
    }
    res.extend(std::iter::repeat_n(0u8, (h * w.div_ceil(32) * 4) as usize));
    // SAFETY: o recurso montado acima tem o formato que a função espera.
    unsafe { CreateIconFromResourceEx(&res, true, 0x0003_0000, w as i32, h as i32, LR_DEFAULTCOLOR).ok() }
}

#[derive(Default)]
struct Applied {
    progress: Option<(TaskbarProgress, u32)>,
    badge: Option<(String, aylis_core::Color)>,
    buttons: Option<Vec<(usize, String)>>,
    added: bool,
    tasks: Option<Vec<aylis_core::JumpTask>>,
}

#[derive(Default)]
pub(crate) struct Taskbars {
    list: Option<ITaskbarList3>,
    applied: HashMap<ComponentId, Applied>,
    /// Janelas cujo botão na barra já existe (o Windows avisa).
    ready: std::collections::HashSet<u64>,
}

impl Taskbars {
    fn list(&mut self) -> Option<ITaskbarList3> {
        if self.list.is_none() {
            // SAFETY: COM já iniciado na thread da janela (o winit inicia o OLE).
            self.list = unsafe { CoCreateInstance::<_, ITaskbarList3>(&CLSID_TASKBAR_LIST, None, CLSCTX_INPROC_SERVER) }.ok().filter(|l| unsafe { l.HrInit() }.is_ok());
        }
        self.list.clone()
    }

    /// Aplica os `Taskbar` do app nas janelas dos forms e entrega os cliques nos botões.
    pub(crate) fn sync(&mut self, ui: &mut Ui, windows: &[(ComponentId, &winit::window::Window)]) {
        if let Ok(mut q) = BUTTON_CREATED.lock() {
            for form in q.drain(..) {
                // O botão (re)nasceu (o Explorer reiniciou, a janela apareceu): reaplica tudo.
                self.ready.insert(form);
                self.applied.retain(|_, _| false);
            }
        }
        let specs = ui.taskbars();
        for (comp, form, spec) in &specs {
            let Some(window) = windows.iter().find(|(f, _)| f == form).map(|(_, w)| *w) else { continue };
            if !self.ready.contains(&form.to_u64()) {
                continue;
            }
            let (Some(hwnd), Some(list)) = (hwnd_of(window), self.list()) else { continue };
            let applied = self.applied.entry(*comp).or_default();
            apply(&list, hwnd, spec, applied);
        }
        self.applied.retain(|id, _| specs.iter().any(|(c, _, _)| c == id));
        let clicks = THUMB_CLICKS.lock().map(|mut q| std::mem::take(&mut *q)).unwrap_or_default();
        for (form, button) in clicks {
            let form = ComponentId::from_u64(form);
            if let Some((comp, _, _)) = specs.iter().find(|(_, f, _)| *f == form) {
                ui.send_message(*comp, &TaskbarEvent { button });
            }
        }
    }
}

const CLSID_TASKBAR_LIST: GUID = GUID::from_u128(0x56fdf344_fd6d_11d0_958a_006097c9a090);
const CLSID_DESTINATION_LIST: GUID = GUID::from_u128(0x77f10cf0_3db5_4966_b520_b7c54fd35ed6);
const CLSID_COLLECTION: GUID = GUID::from_u128(0x2d3468c1_36a7_43b6_ac24_d3f02fd9607a);
const CLSID_SHELL_LINK: GUID = GUID::from_u128(0x00021401_0000_0000_c000_000000000046);

/// `AYLIS_TASKBAR_DEBUG=1`: escreve o resultado de cada chamada ao shell (diagnóstico).
fn debug(what: &str, result: &windows::core::Result<()>) {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    if *ON.get_or_init(|| crate::diag::diagnostic_setting("AYLIS_TASKBAR_DEBUG").is_some()) {
        aylis_core::stderr_line!("taskbar: {what}: {}", result.as_ref().map_or_else(|e| e.message(), |_| "ok".into()));
    }
}

fn apply(list: &ITaskbarList3, hwnd: HWND, spec: &TaskbarSpec, applied: &mut Applied) {
    // SAFETY: chamadas COM com a janela viva e objetos criados aqui.
    unsafe {
        let progress = (spec.progress, (spec.value * 10.0) as u32);
        if applied.progress != Some(progress) {
            let flag = match spec.progress {
                TaskbarProgress::None => TBPF_NOPROGRESS,
                TaskbarProgress::Normal => TBPF_NORMAL,
                TaskbarProgress::Paused => TBPF_PAUSED,
                TaskbarProgress::Error => TBPF_ERROR,
                TaskbarProgress::Indeterminate => TBPF_INDETERMINATE,
            };
            debug("progresso", &list.SetProgressState(hwnd, flag));
            if matches!(spec.progress, TaskbarProgress::Normal | TaskbarProgress::Paused | TaskbarProgress::Error) {
                debug("valor", &list.SetProgressValue(hwnd, progress.1 as u64, 1000));
            }
            applied.progress = Some(progress);
        }
        let badge = (spec.badge.clone(), spec.badge_color);
        if applied.badge.as_ref() != Some(&badge) {
            if spec.badge.is_empty() {
                let _ = list.SetOverlayIcon(hwnd, HICON::default(), PCWSTR::null());
            } else {
                // O indicador: só o círculo (com o número) num ícone de 32 pixels.
                let mut px = vec![0u8; 32 * 32 * 4];
                crate::badge::draw(&mut px, 32, 32, &spec.badge, spec.badge_color, 1.0);
                if let Some(icon) = hicon_from_rgba(&px, 32, 32) {
                    debug("indicador", &list.SetOverlayIcon(hwnd, icon, &HSTRING::from(spec.badge.as_str())));
                    let _ = DestroyIcon(icon);
                }
            }
            applied.badge = Some(badge);
        }
        let buttons: Vec<(usize, String)> = spec.buttons.iter().map(|b| (b.icon.as_ref().map_or(0, |i| i.len()), b.tooltip.clone())).collect();
        if !spec.buttons.is_empty() && applied.buttons.as_ref() != Some(&buttons) {
            let mut icons = Vec::new();
            let thumbs: Vec<THUMBBUTTON> = spec
                .buttons
                .iter()
                .enumerate()
                .map(|(i, b)| {
                    let mut t = THUMBBUTTON { dwMask: THB_ICON | THB_TOOLTIP | THB_FLAGS, iId: i as u32, dwFlags: THBF_ENABLED, ..Default::default() };
                    if let Some((px, w, h)) = b.icon.as_ref().and_then(|bytes| crate::decode_rgba(bytes)) {
                        if let Some(icon) = hicon_from_rgba(&px, w, h) {
                            t.hIcon = icon;
                            icons.push(icon);
                        }
                    }
                    for (k, c) in b.tooltip.encode_utf16().take(259).enumerate() {
                        t.szTip[k] = c;
                    }
                    t
                })
                .collect();
            // Os botões só podem ser postos uma vez; depois, só atualizados.
            let ok = if applied.added { list.ThumbBarUpdateButtons(hwnd, &thumbs) } else { list.ThumbBarAddButtons(hwnd, &thumbs) };
            debug("botões da miniatura", &ok);
            if ok.is_ok() {
                applied.added = true;
                applied.buttons = Some(buttons);
            }
            for icon in icons {
                let _ = DestroyIcon(icon);
            }
        }
        if applied.tasks.as_ref() != Some(&spec.tasks) {
            debug("tarefas", &set_tasks(&spec.tasks));
            applied.tasks = Some(spec.tasks.clone());
        }
    }
}

/// As tarefas do clique direito no botão (a "jump list"): abrem o próprio app com os
/// argumentos de cada uma.
fn set_tasks(tasks: &[aylis_core::JumpTask]) -> windows::core::Result<()> {
    use windows::Win32::Storage::EnhancedStorage::PKEY_Title;
    use windows::Win32::UI::Shell::PropertiesSystem::IPropertyStore;
    // SAFETY: objetos COM criados e usados aqui mesmo.
    unsafe {
        let list: ICustomDestinationList = CoCreateInstance(&CLSID_DESTINATION_LIST, None, CLSCTX_INPROC_SERVER)?;
        if tasks.is_empty() {
            return list.DeleteList(PCWSTR::null());
        }
        let mut slots = 0u32;
        let _removed: IObjectArray = list.BeginList(&mut slots)?;
        let collection: IObjectCollection = CoCreateInstance(&CLSID_COLLECTION, None, CLSCTX_INPROC_SERVER)?;
        let exe = std::env::current_exe().map(|p| p.display().to_string()).unwrap_or_default();
        for task in tasks {
            let link: IShellLinkW = CoCreateInstance(&CLSID_SHELL_LINK, None, CLSCTX_INPROC_SERVER)?;
            link.SetPath(&HSTRING::from(exe.as_str()))?;
            link.SetArguments(&HSTRING::from(task.arguments.as_str()))?;
            link.SetDescription(&HSTRING::from(task.title.as_str()))?;
            let store: IPropertyStore = link.cast()?;
            store.SetValue(&PKEY_Title, &windows::Win32::System::Com::StructuredStorage::PROPVARIANT::from(task.title.as_str()))?;
            store.Commit()?;
            collection.AddObject(&link)?;
        }
        let array: IObjectArray = collection.cast()?;
        list.AddUserTasks(&array)?;
        list.CommitList()
    }
}
