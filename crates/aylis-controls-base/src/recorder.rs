//! Gravação de uso e métricas: o `EventRecorder` grava tudo o que o usuário faz (cliques,
//! teclas, texto, eventos, forms, erros) num arquivo `.aylisrec` que a IDE abre para ver o
//! comportamento; o `Analytics` soma métricas de uso entre as sessões (`analytics.json`).
//! Os dois ouvem a atividade do `Ui` ([`aylis_core::activity`]) e não precisam de código:
//! no form com `active`, já começam. Tudo fica no aparelho; nada vai para a rede.
//!
//! O `.aylisrec` é JSON Lines: a primeira linha é o cabeçalho (`"type": "start"`), cada
//! linha seguinte um acontecimento com `t` (segundos desde o início) e `type`.

use std::cell::RefCell;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Mutex;

use aylis_core::activity::{Activity, ObserverId};
use aylis_core::{Component, ComponentId, EventCtx, Key, Modifiers, PointerButton, PropValue, Ui, properties};
use serde_json::{Map, Value, json};
use web_time::{Instant, SystemTime, UNIX_EPOCH};

/// Extensão das gravações.
pub const RECORDING_EXTENSION: &str = "aylisrec";

// ---------------------------------------------------------------------------
// pasta e arquivos

/// A pasta de dados do app no aparelho: `%LOCALAPPDATA%\<app>` no Windows,
/// `~/.local/share/<app>` no Linux e no macOS, a pasta temporária onde não houver.
pub fn app_data_dir(app: &str) -> PathBuf {
    let app = sanitize(if app.is_empty() { "aylis-app" } else { app });
    let base = if cfg!(windows) {
        std::env::var_os("LOCALAPPDATA").map(PathBuf::from)
    } else {
        std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".local").join("share"))
    };
    // No navegador não há pasta temporária (nem disco): um caminho só de nome.
    base.unwrap_or_else(|| if cfg!(target_arch = "wasm32") { PathBuf::from("/aylis") } else { std::env::temp_dir() }).join(app)
}

fn sanitize(name: &str) -> String {
    name.chars().map(|c| if c.is_alphanumeric() || c == '-' || c == '_' || c == '.' { c } else { '-' }).collect()
}

fn unix_ms() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_millis() as u64)
}

/// `2026-10-26_18-30-05`, em UTC (sem depender do fuso do sistema).
fn stamp(ms: u64) -> String {
    let secs = ms / 1000;
    let (days, rest) = (secs / 86_400, secs % 86_400);
    let (y, m, d) = civil_from_days(days as i64);
    format!("{y:04}-{m:02}-{d:02}_{:02}-{:02}-{:02}", rest / 3600, rest % 3600 / 60, rest % 60)
}

/// `26/10/2026 18:30 UTC`.
pub fn utc_text(ms: u64) -> String {
    let secs = ms / 1000;
    let (y, m, d) = civil_from_days((secs / 86_400) as i64);
    let rest = secs % 86_400;
    format!("{d:02}/{m:02}/{y} {:02}:{:02} UTC", rest / 3600, rest % 3600 / 60)
}

/// Dias desde 1970-01-01 → (ano, mês, dia) (algoritmo de Howard Hinnant).
fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (if m <= 2 { yoe + era * 400 + 1 } else { yoe + era * 400 }, m, d)
}

/// Deixa só as `keep` gravações mais novas da pasta.
fn prune(dir: &Path, keep: usize) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    let mut files: Vec<(std::time::SystemTime, PathBuf)> = entries
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|e| e == RECORDING_EXTENSION))
        .filter_map(|p| Some((p.metadata().ok()?.modified().ok()?, p)))
        .collect();
    files.sort();
    let extra = files.len().saturating_sub(keep);
    for (_, p) in files.into_iter().take(extra) {
        let _ = std::fs::remove_file(p);
    }
}

// ---------------------------------------------------------------------------
// descrição dos acontecimentos (comum às gravações e às métricas)

fn mods_text(m: Modifiers) -> String {
    let mut parts = Vec::new();
    if m.ctrl {
        parts.push("Ctrl");
    }
    if m.shift {
        parts.push("Shift");
    }
    if m.alt {
        parts.push("Alt");
    }
    if m.meta {
        parts.push("Meta");
    }
    parts.join("+")
}

fn key_text(key: Key) -> String {
    match key {
        Key::Char(c) => c.to_uppercase().to_string(),
        other => format!("{other:?}"),
    }
}

/// `Ctrl+Shift+N`.
pub fn chord_text(key: Key, m: Modifiers) -> String {
    let mods = mods_text(m);
    if mods.is_empty() { key_text(key) } else { format!("{mods}+{}", key_text(key)) }
}

fn button_text(b: PointerButton) -> &'static str {
    match b {
        PointerButton::Primary => "left",
        PointerButton::Secondary => "right",
        PointerButton::Middle => "middle",
        PointerButton::Back => "back",
        PointerButton::Forward => "forward",
    }
}

/// O caminho de um componente (`frmIde.btnSalvar`). Sem nome: o antepassado com nome
/// mais próximo e, dali para baixo, `Classe[posição]` (`frmIde/Panel[3]/Button[0]`).
fn path(ui: &Ui, id: ComponentId) -> String {
    if ui.name(id).is_some_and(|n| !n.is_empty()) {
        return ui.path_of(id);
    }
    let mut steps = Vec::new();
    let mut current = id;
    while ui.name(current).is_none_or(str::is_empty) {
        let Some(parent) = ui.parent(current) else { break };
        let index = ui.children(parent).iter().position(|&c| c == current).unwrap_or(0);
        steps.push(format!("{}[{index}]", ui.class_name(current).unwrap_or("?")));
        current = parent;
    }
    steps.reverse();
    let base = if ui.name(current).is_some_and(|n| !n.is_empty()) { ui.path_of(current) } else { ui.class_name(current).unwrap_or("?").to_string() };
    std::iter::once(base).chain(steps).collect::<Vec<_>>().join("/")
}

/// O texto visível de um componente (para saber em que o usuário clicou).
fn caption(ui: &Ui, id: ComponentId) -> Option<String> {
    ["text", "caption"].into_iter().find_map(|p| match ui.get_prop(id, p) {
        Ok(PropValue::String(s)) if !s.is_empty() => Some(s.chars().take(60).collect()),
        _ => None,
    })
}

/// O componente, a classe e o texto dele (o de um campo secreto não: só `_masked`).
fn describe(ui: &Ui, id: ComponentId, out: &mut Map<String, Value>, prefix: &str, opts: Options) {
    out.insert(format!("{prefix}"), Value::String(path(ui, id)));
    out.insert(format!("{prefix}_class"), Value::String(ui.class_name(id).unwrap_or("?").to_string()));
    if is_secret(ui, Some(id), opts) {
        out.insert(format!("{prefix}_masked"), Value::Bool(true));
    } else if let Some(t) = caption(ui, id) {
        out.insert(format!("{prefix}_text"), Value::String(t));
    }
}

fn flag(ui: &Ui, id: ComponentId, prop: &str) -> bool {
    matches!(ui.get_prop(id, prop), Ok(PropValue::Bool(true)))
}

/// Dado pessoal (LGPD): o `sensitive` do controle ou do campo ligado a ele.
fn is_sensitive(ui: &Ui, id: ComponentId) -> bool {
    flag(ui, id, "sensitive") || flag(ui, id, "field_sensitive")
}

/// O que se digita em `id` não vai para a gravação: senha sempre; dado pessoal sem o
/// `record_sensitive`; qualquer caixa de texto com o `mask_text`.
fn is_secret(ui: &Ui, id: Option<ComponentId>, opts: Options) -> bool {
    let Some(id) = id else { return opts.mask_text };
    let text_box = ui.get_prop(id, "sensitive").is_ok();
    flag(ui, id, "password") || (is_sensitive(ui, id) && !record_sensitive(ui, opts)) || (opts.mask_text && text_box)
}

/// O `record_sensitive` do gravador agora (liga e desliga durante a gravação).
fn record_sensitive(ui: &Ui, opts: Options) -> bool {
    ui.get::<EventRecorder>(opts.recorder).is_some_and(|r| r.record_sensitive)
}

/// Opções de como gravar.
#[derive(Clone, Copy)]
struct Options {
    pointer_moves: bool,
    mask_text: bool,
    timers: bool,
    /// O próprio gravador (o `record_sensitive` é lido na hora).
    recorder: ComponentId,
}

/// O que a gravação lembra entre um acontecimento e outro.
#[derive(Default)]
struct Track {
    /// Botão pressionado: os movimentos são um arrasto.
    dragging: bool,
    /// Quando entrou o último movimento (no máximo um a cada 0,1 s).
    last_move: f64,
    /// O `record_sensitive` da última linha (a mudança vai para a gravação).
    record_sensitive: Option<bool>,
    /// O `sensitive` de cada campo já visto (idem).
    sensitive: std::collections::HashMap<ComponentId, bool>,
}

/// As mudanças de política (LGPD) que valem para este acontecimento, antes dele: ligou ou
/// desligou o `record_sensitive`, ou o `sensitive` do campo em foco.
fn policy_changes(ui: &Ui, a: &Activity<'_>, opts: Options, track: &mut Track) -> Vec<Map<String, Value>> {
    let mut out = Vec::new();
    let now = record_sensitive(ui, opts);
    if track.record_sensitive != Some(now) {
        if track.record_sensitive.is_some() {
            let mut o = Map::new();
            o.insert("type".into(), json!("record_sensitive"));
            o.insert("on".into(), json!(now));
            out.push(o);
        }
        track.record_sensitive = Some(now);
    }
    let form = match a {
        Activity::KeyDown { form, .. } | Activity::Text { form, .. } => *form,
        _ => return out,
    };
    if let Some(f) = ui.focused(form) {
        let s = is_sensitive(ui, f);
        let before = track.sensitive.insert(f, s);
        // Só a mudança (e a primeira vez que um campo sensível aparece).
        if before.map_or(s, |b| b != s) {
            let mut o = Map::new();
            o.insert("type".into(), json!("sensitive"));
            o.insert("component".into(), json!(path(ui, f)));
            o.insert("on".into(), json!(s));
            out.push(o);
        }
    }
    out
}

/// Um acontecimento como objeto JSON (sem o `t`; `now` é ele). `None`: não vale gravar.
fn entry(ui: &Ui, a: &Activity<'_>, opts: Options, track: &mut Track, now: f64) -> Option<Map<String, Value>> {
    let mut o = Map::new();
    // Form sem nome (um diálogo montado em código): a classe e o título.
    let form_name = |f: ComponentId| match ui.name(f).filter(|n| !n.is_empty()) {
        Some(n) => n.to_string(),
        None => match caption(ui, f) {
            Some(t) => format!("{} \"{t}\"", ui.class_name(f).unwrap_or("Form")),
            None => path(ui, f),
        },
    };
    let pointer = |o: &mut Map<String, Value>, f: ComponentId, x: f32, y: f32| {
        o.insert("form".into(), json!(form_name(f)));
        o.insert("x".into(), json!(x.round()));
        o.insert("y".into(), json!(y.round()));
        if let Some(t) = ui.hit_test(f, aylis_core::PointF::new(x, y)) {
            describe(ui, t, o, "target", opts);
        }
        if ui.input_blocked(f) {
            o.insert("blocked".into(), json!(true));
        }
    };
    match a {
        Activity::PointerDown { form, event } | Activity::PointerUp { form, event } => {
            let down = matches!(a, Activity::PointerDown { .. });
            track.dragging = down;
            o.insert("type".into(), json!(if down { "pointer_down" } else { "pointer_up" }));
            pointer(&mut o, *form, event.position.x, event.position.y);
            o.insert("button".into(), json!(button_text(event.button)));
            if event.click_count > 1 {
                o.insert("clicks".into(), json!(event.click_count));
            }
            let m = mods_text(event.modifiers);
            if !m.is_empty() {
                o.insert("mods".into(), json!(m));
            }
        }
        Activity::PointerMove { form, event } => {
            if (!opts.pointer_moves && !track.dragging) || now - track.last_move < 0.1 {
                return None;
            }
            track.last_move = now;
            o.insert("type".into(), json!(if track.dragging { "drag" } else { "pointer_move" }));
            o.insert("form".into(), json!(form_name(*form)));
            o.insert("x".into(), json!(event.position.x.round()));
            o.insert("y".into(), json!(event.position.y.round()));
        }
        Activity::Wheel { form, event } => {
            o.insert("type".into(), json!("wheel"));
            pointer(&mut o, *form, event.position.x, event.position.y);
            o.insert("dx".into(), json!(event.delta.x.round()));
            o.insert("dy".into(), json!(event.delta.y.round()));
        }
        Activity::KeyDown { form, event } => {
            if event.repeat {
                return None;
            }
            let focus = ui.focused(*form);
            let secret = is_secret(ui, focus, opts);
            let printable = matches!(event.key, Key::Char(_)) && !event.modifiers.ctrl && !event.modifiers.alt && !event.modifiers.meta;
            o.insert("type".into(), json!("key"));
            o.insert("form".into(), json!(form_name(*form)));
            o.insert("key".into(), json!(if secret && printable { "*".to_string() } else { chord_text(event.key, event.modifiers) }));
            if let Some(f) = focus {
                describe(ui, f, &mut o, "focus", opts);
            }
            if ui.input_blocked(*form) {
                o.insert("blocked".into(), json!(true));
            }
        }
        Activity::KeyUp { .. } => return None,
        Activity::Text { form, text } => {
            let focus = ui.focused(*form);
            let secret = is_secret(ui, focus, opts);
            o.insert("type".into(), json!("text"));
            o.insert("form".into(), json!(form_name(*form)));
            o.insert("text".into(), json!(if secret { "*".repeat(text.chars().count()) } else { text.to_string() }));
            if let Some(f) = focus {
                describe(ui, f, &mut o, "focus", opts);
            }
        }
        Activity::Event { component, event, handler } => {
            if !opts.timers && *event == "on_timer" {
                return None;
            }
            o.insert("type".into(), json!("event"));
            o.insert("event".into(), json!(event));
            describe(ui, *component, &mut o, "component", opts);
            if !handler.is_empty() {
                o.insert("handler".into(), json!(handler));
            }
        }
        Activity::FormShown { form, modal } => {
            o.insert("type".into(), json!("form_shown"));
            o.insert("form".into(), json!(form_name(*form)));
            if let Some(t) = caption(ui, *form) {
                o.insert("title".into(), json!(t));
            }
            if *modal {
                o.insert("modal".into(), json!(true));
            }
        }
        Activity::FormClosed { form } => {
            o.insert("type".into(), json!("form_closed"));
            o.insert("form".into(), json!(form_name(*form)));
        }
        Activity::Error { message } => {
            o.insert("type".into(), json!("error"));
            o.insert("message".into(), json!(message));
        }
        Activity::Custom { name, value } => {
            o.insert("type".into(), json!("metric"));
            o.insert("name".into(), json!(name));
            o.insert("value".into(), json!(value));
        }
    }
    Some(o)
}

// ---------------------------------------------------------------------------
// pânicos

/// Arquivos abertos para gravar: um pânico (erro fatal) vira a última linha de cada um.
static PANIC_TARGETS: Mutex<Vec<(PathBuf, u64)>> = Mutex::new(Vec::new());

fn install_panic_hook() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        let previous = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            let message = info.payload().downcast_ref::<&str>().map(|s| s.to_string()).or_else(|| info.payload().downcast_ref::<String>().cloned()).unwrap_or_default();
            let place = info.location().map(|l| format!("{}:{}", l.file(), l.line())).unwrap_or_default();
            if let Ok(targets) = PANIC_TARGETS.lock() {
                for (file, started) in targets.iter() {
                    let t = unix_ms().saturating_sub(*started) as f64 / 1000.0;
                    let line = json!({ "t": t, "type": "error", "fatal": true, "message": message, "location": place });
                    append_line(file, &line.to_string());
                }
            }
            previous(info);
        }));
    });
}

fn append_line(file: &Path, line: &str) {
    use std::io::Write;
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(file) {
        let _ = writeln!(f, "{line}");
    }
}

// ---------------------------------------------------------------------------
// EventRecorder

/// Grava a sessão inteira num `.aylisrec` (como uma caixa-preta): cliques com o componente
/// atingido, teclas e atalhos, texto digitado, eventos com handler, forms abertos e
/// fechados, erros (inclusive pânicos). A IDE abre o arquivo (Arquivo → Abrir gravação…)
/// e mostra a linha do tempo.
///
/// Com `active` ele começa sozinho; `EventRecorder::start`/`stop` também. Campos com
/// `password` sempre gravam `*`; os com `sensitive` (dado pessoal, LGPD; no controle ou no
/// Field ligado a ele) também, a não ser com `record_sensitive`; `mask_text` mascara toda
/// digitação. Ligar e desligar o `record_sensitive` ou o `sensitive` de um campo durante a
/// gravação fica registrado nela.
pub struct EventRecorder {
    pub active: bool,
    /// Pasta das gravações; vazio: `<pasta do app>/recordings` ([`app_data_dir`]).
    pub folder: String,
    /// O nome do app (pasta e cabeçalho); vazio: o título do form principal.
    pub app_name: String,
    /// Grava o movimento do ponteiro (muitas linhas); os arrastos sempre entram.
    pub pointer_moves: bool,
    /// Troca toda digitação por `*`.
    pub mask_text: bool,
    /// Grava também o que se digita nos campos `sensitive` (dado pessoal). Desligado, eles
    /// gravam `*`. Liga e desliga em tempo de execução (a mudança fica na gravação).
    pub record_sensitive: bool,
    /// Grava também os `on_timer` (um por tique).
    pub timers: bool,
    /// Quantas gravações manter na pasta (as mais antigas saem).
    pub keep: u32,
    /// O arquivo da gravação em andamento (só leitura).
    pub file_name: String,
    session: Rc<RefCell<Option<Session>>>,
}

struct Session {
    observer: ObserverId,
    file: PathBuf,
}

impl Default for EventRecorder {
    fn default() -> Self {
        Self {
            active: true,
            folder: String::new(),
            app_name: String::new(),
            pointer_moves: false,
            mask_text: false,
            record_sensitive: false,
            timers: false,
            keep: 20,
            file_name: String::new(),
            session: Rc::default(),
        }
    }
}

impl EventRecorder {
    /// Começa a gravar (se já gravava, continua no mesmo arquivo). Devolve o arquivo.
    pub fn start(ui: &mut Ui, id: ComponentId) -> Option<PathBuf> {
        let r = ui.get::<EventRecorder>(id)?;
        if let Some(s) = r.session.borrow().as_ref() {
            return Some(s.file.clone());
        }
        let session = r.session.clone();
        let opts = Options { pointer_moves: r.pointer_moves, mask_text: r.mask_text, timers: r.timers, recorder: id };
        let recording_sensitive = r.record_sensitive;
        let app = if r.app_name.is_empty() {
            ui.main_form().and_then(|f| caption(ui, f)).unwrap_or_else(|| "aylis-app".into())
        } else {
            r.app_name.clone()
        };
        let dir = if r.folder.is_empty() { app_data_dir(&app).join("recordings") } else { PathBuf::from(&r.folder) };
        let keep = r.keep.max(1) as usize;
        std::fs::create_dir_all(&dir).ok()?;
        let started_ms = unix_ms();
        let file = dir.join(format!("{}_{}.{RECORDING_EXTENSION}", sanitize(&app), stamp(started_ms)));
        let header = json!({
            "type": "start",
            "format": 1,
            "app": app,
            "start_ms": started_ms,
            "os": std::env::consts::OS,
            "arch": std::env::consts::ARCH,
            "aylis": env!("CARGO_PKG_VERSION"),
            "record_sensitive": recording_sensitive,
            "open_forms": ui.open_forms().into_iter().map(|f| ui.name(f).unwrap_or_default().to_string()).collect::<Vec<_>>(),
            "platform": ui.platform_info().iter().map(|(k, v)| (k.clone(), json!(v))).collect::<Map<String, Value>>(),
        });
        append_line(&file, &header.to_string());
        prune(&dir, keep);
        install_panic_hook();
        if let Ok(mut t) = PANIC_TARGETS.lock() {
            t.push((file.clone(), started_ms));
        }
        let start = Instant::now();
        let target = file.clone();
        let mut track = Track::default();
        let observer = ui.add_observer(move |ui, a| {
            let now = start.elapsed().as_secs_f64();
            let t = json!((now * 1000.0).round() / 1000.0);
            for mut change in policy_changes(ui, a, opts, &mut track) {
                let mut line = Map::new();
                line.insert("t".into(), t.clone());
                line.append(&mut change);
                append_line(&target, &Value::Object(line).to_string());
            }
            if let Some(mut o) = entry(ui, a, opts, &mut track, now) {
                let mut line = Map::new();
                line.insert("t".into(), json!((now * 1000.0).round() / 1000.0));
                line.append(&mut o);
                append_line(&target, &Value::Object(line).to_string());
            }
        });
        *session.borrow_mut() = Some(Session { observer, file: file.clone() });
        if let Some(r) = ui.get_mut::<EventRecorder>(id) {
            r.file_name = file.display().to_string();
        }
        Some(file)
    }

    /// Para de gravar (o arquivo fica).
    pub fn stop(ui: &mut Ui, id: ComponentId) {
        let Some(session) = ui.get::<EventRecorder>(id).and_then(|r| r.session.borrow_mut().take()) else { return };
        ui.remove_observer(session.observer);
        if let Ok(mut t) = PANIC_TARGETS.lock() {
            t.retain(|(f, _)| *f != session.file);
        }
    }

    /// Está gravando?
    pub fn recording(ui: &Ui, id: ComponentId) -> bool {
        ui.get::<EventRecorder>(id).is_some_and(|r| r.session.borrow().is_some())
    }

    fn started(&self) -> bool {
        self.session.borrow().is_some()
    }
}

impl Component for EventRecorder {
    fn class_name(&self) -> &'static str {
        "EventRecorder"
    }

    properties! {
        active: bool => "Behavior",
        folder: String => "Behavior",
        app_name: String => "Behavior",
        pointer_moves: bool => "Behavior",
        mask_text: bool => "Behavior",
        record_sensitive: bool => "Behavior",
        timers: bool => "Behavior",
        keep: u32 => "Behavior",
        file_name: String => "Data"
    }

    // Começa no primeiro tique depois de entrar no Ui (ninguém precisa chamar `start`).
    fn timer_interval(&self) -> Option<f64> {
        (self.active && !self.started()).then_some(0.001)
    }

    fn timer(&mut self, ctx: &mut EventCtx<'_>) {
        if ctx.ui.design_mode() {
            return;
        }
        // O componente está fora do Ui durante o método: começa depois, pelo correio.
        let id = ctx.id;
        // (Até o correio entregar pode vir outro tique: `start` não começa duas vezes.)
        ctx.ui.poster().post(move |ui| {
            EventRecorder::start(ui, id);
            // Começou: o tique de partida sai da agenda.
            ui.sync_timer(id);
        });
    }

    fn property_changed(&mut self, ctx: &mut EventCtx<'_>, name: &str) {
        if name == "active" && !self.active && self.started() {
            let id = ctx.id;
            ctx.ui.poster().post(move |ui| {
                EventRecorder::stop(ui, id);
                ui.sync_timer(id);
            });
        }
    }
}

// ---------------------------------------------------------------------------
// leitura (a IDE)

/// Uma gravação lida do disco.
#[derive(Clone, Debug, Default)]
pub struct Recording {
    /// A linha `"type": "start"`.
    pub header: Value,
    pub entries: Vec<Value>,
}

/// As chaves (e os valores de `type` e `button`) das gravações feitas antes do formato em
/// inglês: lidas como as de hoje.
const LEGACY_KEYS: [(&str, &str); 23] = [
    ("tipo", "type"),
    ("formato", "format"),
    ("inicio_ms", "start_ms"),
    ("so", "os"),
    ("arquitetura", "arch"),
    ("forms_abertos", "open_forms"),
    ("plataforma", "platform"),
    ("alvo", "target"),
    ("alvo_classe", "target_class"),
    ("alvo_texto", "target_text"),
    ("bloqueado", "blocked"),
    ("botao", "button"),
    ("cliques", "clicks"),
    ("tecla", "key"),
    ("foco", "focus"),
    ("foco_classe", "focus_class"),
    ("foco_texto", "focus_text"),
    ("texto", "text"),
    ("evento", "event"),
    ("componente", "component"),
    ("componente_classe", "component_class"),
    ("componente_texto", "component_text"),
    ("titulo", "title"),
];
const LEGACY_MORE_KEYS: [(&str, &str); 4] = [("mensagem", "message"), ("local", "location"), ("nome", "name"), ("valor", "value")];
const LEGACY_VALUES: [(&str, &str); 18] = [
    ("inicio", "start"),
    ("ponteiro_baixo", "pointer_down"),
    ("ponteiro_cima", "pointer_up"),
    ("ponteiro_move", "pointer_move"),
    ("arrasto", "drag"),
    ("roda", "wheel"),
    ("tecla", "key"),
    ("texto", "text"),
    ("evento", "event"),
    ("form_aberto", "form_shown"),
    ("form_fechado", "form_closed"),
    ("erro", "error"),
    ("metrica", "metric"),
    ("esquerdo", "left"),
    ("direito", "right"),
    ("meio", "middle"),
    ("voltar", "back"),
    ("avancar", "forward"),
];

/// Uma linha de gravação antiga (chaves em português) no formato de hoje.
fn upgrade_legacy(line: Value) -> Value {
    let Value::Object(map) = line else { return line };
    if !map.contains_key("tipo") {
        return Value::Object(map);
    }
    let rename = |k: &str| LEGACY_KEYS.iter().chain(&LEGACY_MORE_KEYS).find(|(old, _)| *old == k).map_or(k, |(_, new)| *new).to_string();
    let value = |v: Value| match v.as_str().and_then(|s| LEGACY_VALUES.iter().find(|(old, _)| *old == s)) {
        Some((_, new)) => Value::String((*new).into()),
        None => v,
    };
    Value::Object(
        map.into_iter()
            .map(|(k, v)| {
                let v = if k == "tipo" || k == "botao" { value(v) } else { v };
                (rename(&k), v)
            })
            .collect(),
    )
}

impl Recording {
    pub fn parse(text: &str) -> Result<Recording, String> {
        let mut lines = text.lines().filter(|l| !l.trim().is_empty());
        let header: Value = serde_json::from_str(lines.next().ok_or("gravação vazia")?).map_err(|e| format!("cabeçalho inválido: {e}"))?;
        let header = upgrade_legacy(header);
        if header["type"] != "start" {
            return Err("não é uma gravação do Aylis (falta o cabeçalho)".into());
        }
        // Uma linha cortada (o app caiu no meio dela) não perde o resto.
        let entries = lines.filter_map(|l| serde_json::from_str(l).ok()).map(upgrade_legacy).collect();
        Ok(Recording { header, entries })
    }

    pub fn open(path: &Path) -> Result<Recording, String> {
        Self::parse(&std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?)
    }

    /// Duração em segundos (o `t` do último acontecimento).
    pub fn duration(&self) -> f64 {
        self.entries.last().and_then(|e| e["t"].as_f64()).unwrap_or(0.0)
    }

    pub fn errors(&self) -> usize {
        self.entries.iter().filter(|e| e["type"] == "error").count()
    }

    /// Uma linha legível do acontecimento (para listas e relatórios).
    pub fn describe(entry: &Value) -> String {
        let s = |k: &str| entry[k].as_str().unwrap_or_default().to_string();
        let target = |prefix: &str| {
            let text = s(&format!("{prefix}_text"));
            let class = s(&format!("{prefix}_class"));
            let p = s(prefix);
            match (text.is_empty(), p.is_empty()) {
                (false, _) => format!("{class} \"{text}\" ({p})"),
                (true, false) => format!("{class} {p}"),
                _ => String::new(),
            }
        };
        let blocked = if entry["blocked"] == true { " [bloqueado por um modal]" } else { "" };
        match entry["type"].as_str().unwrap_or_default() {
            "pointer_down" => format!("clique {} em {}{blocked}", s("button"), target("target")),
            "pointer_up" => format!("soltou em {}{blocked}", target("target")),
            "drag" | "pointer_move" => format!("{} em {}, {}", s("type"), entry["x"], entry["y"]),
            "wheel" => format!("roda {} em {}{blocked}", entry["dy"], target("target")),
            "key" => format!("tecla {} (foco: {}){blocked}", s("key"), target("focus")),
            "text" => format!("digitou \"{}\" em {}", s("text"), target("focus")),
            "event" => {
                let h = s("handler");
                format!("{} de {}{}", s("event"), target("component"), if h.is_empty() { String::new() } else { format!(" → {h}") })
            }
            "form_shown" => format!("abriu {}{}", s("form"), if entry["modal"] == true { " (modal)" } else { "" }),
            "form_closed" => format!("fechou {}", s("form")),
            "error" => format!("ERRO: {} {}", s("message"), s("location")),
            "metric" => format!("métrica {} = {}", s("name"), s("value")),
            other => other.to_string(),
        }
    }
}

// ---------------------------------------------------------------------------
// Analytics

/// Métricas de uso do app, somadas entre as sessões num JSON no aparelho: sessões, tempo
/// de uso, tempo e aberturas por form, cliques por componente, eventos por handler,
/// atalhos, erros (os últimos com a mensagem) e métricas do próprio app
/// ([`Analytics::track`]). Grava a cada 30 s com novidades, num erro e ao fechar o form
/// principal. [`Analytics::summary`] devolve o JSON.
pub struct Analytics {
    pub active: bool,
    /// O arquivo; vazio: `<pasta do app>/analytics.json`.
    pub file: String,
    pub app_name: String,
    state: Rc<RefCell<Option<Metrics>>>,
}

impl Default for Analytics {
    fn default() -> Self {
        Self { active: true, file: String::new(), app_name: String::new(), state: Rc::default() }
    }
}

/// As chaves de primeiro nível do `analytics.json` antes do formato em inglês.
const ANALYTICS_LEGACY_KEYS: [(&str, &str); 10] = [
    ("sessoes", "sessions"),
    ("primeira_ms", "first_ms"),
    ("ultima_ms", "last_ms"),
    ("tempo_total_s", "total_time_s"),
    ("cliques", "clicks"),
    ("eventos", "events"),
    ("atalhos", "shortcuts"),
    ("metricas", "metrics"),
    ("ultimos_erros", "last_errors"),
    ("erros", "errors"),
];

struct Metrics {
    observer: ObserverId,
    file: PathBuf,
    data: Value,
    /// Desde quando cada form está aberto.
    open: HashMap<ComponentId, (String, Instant)>,
    session_start: Instant,
    last_save: Instant,
    dirty: bool,
}

impl Metrics {
    fn bump(&mut self, section: &str, key: &str) {
        let v = &mut self.data[section][key];
        *v = json!(v.as_u64().unwrap_or(0) + 1);
        self.dirty = true;
    }

    fn save(&mut self) {
        let elapsed = self.session_start.elapsed().as_secs_f64();
        let mut data = self.data.clone();
        let total = data["total_time_s"].as_f64().unwrap_or(0.0) + elapsed;
        data["total_time_s"] = json!(total.round());
        // O tempo dos forms ainda abertos também conta.
        for (name, since) in self.open.values() {
            let t = &mut data["forms"][name]["time_s"];
            *t = json!((t.as_f64().unwrap_or(0.0) + since.elapsed().as_secs_f64()).round());
        }
        data["last_ms"] = json!(unix_ms());
        if let Some(dir) = self.file.parent() {
            let _ = std::fs::create_dir_all(dir);
        }
        let _ = std::fs::write(&self.file, serde_json::to_string_pretty(&data).unwrap_or_default());
        self.last_save = Instant::now();
        self.dirty = false;
    }
}

impl Analytics {
    /// Começa a medir (lendo o que já havia no arquivo).
    pub fn start(ui: &mut Ui, id: ComponentId) {
        let Some(a) = ui.get::<Analytics>(id) else { return };
        if a.state.borrow().is_some() {
            return;
        }
        let state = a.state.clone();
        let app = if a.app_name.is_empty() { ui.main_form().and_then(|f| caption(ui, f)).unwrap_or_else(|| "aylis-app".into()) } else { a.app_name.clone() };
        let file = if a.file.is_empty() { app_data_dir(&app).join("analytics.json") } else { PathBuf::from(&a.file) };
        let mut data: Value = std::fs::read_to_string(&file).ok().and_then(|t| serde_json::from_str(&t).ok()).filter(Value::is_object).unwrap_or_else(|| json!({}));
        // Um arquivo do formato antigo (chaves em português): as somas continuam.
        if let Some(map) = data.as_object_mut() {
            for (old, new) in ANALYTICS_LEGACY_KEYS {
                if let Some(v) = map.remove(old) {
                    map.entry(new).or_insert(v);
                }
            }
        }
        data["app"] = json!(app);
        data["sessions"] = json!(data["sessions"].as_u64().unwrap_or(0) + 1);
        if data["first_ms"].is_null() {
            data["first_ms"] = json!(unix_ms());
        }
        for section in ["forms", "clicks", "events", "shortcuts", "metrics"] {
            if !data[section].is_object() {
                data[section] = json!({});
            }
        }
        if !data["last_errors"].is_array() {
            data["last_errors"] = json!([]);
        }
        let weak = Rc::downgrade(&state);
        let main = ui.main_form();
        let observer = ui.add_observer(move |ui, a| {
            let Some(state) = weak.upgrade() else { return };
            let mut guard = state.borrow_mut();
            let Some(m) = guard.as_mut() else { return };
            match a {
                Activity::PointerDown { form, event } => {
                    if let Some(t) = ui.hit_test(*form, event.position) {
                        m.bump("clicks", &path(ui, t));
                    }
                }
                Activity::KeyDown { event, .. } if !event.repeat && (event.modifiers.ctrl || event.modifiers.alt || matches!(event.key, Key::F(_))) => {
                    m.bump("shortcuts", &chord_text(event.key, event.modifiers));
                }
                Activity::Event { component, event, .. } if *event != "on_timer" => {
                    m.bump("events", &format!("{}.{event}", path(ui, *component)));
                }
                Activity::FormShown { form, .. } => {
                    let name = ui.name(*form).unwrap_or_default().to_string();
                    let n = &mut m.data["forms"][&name]["opens"];
                    *n = json!(n.as_u64().unwrap_or(0) + 1);
                    m.open.insert(*form, (name, Instant::now()));
                    m.dirty = true;
                }
                Activity::FormClosed { form } => {
                    if let Some((name, since)) = m.open.remove(form) {
                        let t = &mut m.data["forms"][&name]["time_s"];
                        *t = json!((t.as_f64().unwrap_or(0.0) + since.elapsed().as_secs_f64()).round());
                    }
                    m.dirty = true;
                    if Some(*form) == main {
                        m.save();
                    }
                }
                Activity::Error { message } => {
                    m.data["errors"] = json!(m.data["errors"].as_u64().unwrap_or(0) + 1);
                    let list = m.data["last_errors"].as_array_mut().expect("criado no início");
                    list.push(json!({ "ms": unix_ms(), "message": message }));
                    let extra = list.len().saturating_sub(10);
                    list.drain(..extra);
                    m.save();
                }
                Activity::Custom { name, value } => {
                    let entry = &mut m.data["metrics"][*name];
                    entry["n"] = json!(entry["n"].as_u64().unwrap_or(0) + 1);
                    entry["last"] = json!(value);
                    if let Ok(v) = value.parse::<f64>() {
                        entry["sum"] = json!(entry["sum"].as_f64().unwrap_or(0.0) + v);
                    }
                    m.dirty = true;
                }
                _ => {}
            }
            if m.dirty && m.last_save.elapsed().as_secs() >= 30 {
                m.save();
            }
        });
        let open: HashMap<ComponentId, (String, Instant)> = ui.open_forms().into_iter().map(|f| (f, (ui.name(f).unwrap_or_default().to_string(), Instant::now()))).collect();
        let mut m = Metrics { observer, file, data, open, session_start: Instant::now(), last_save: Instant::now(), dirty: true };
        m.save();
        *state.borrow_mut() = Some(m);
    }

    pub fn stop(ui: &mut Ui, id: ComponentId) {
        let Some(m) = ui.get::<Analytics>(id).and_then(|a| a.state.borrow_mut().take()) else { return };
        ui.remove_observer(m.observer);
        let mut m = m;
        m.save();
    }

    /// Uma métrica do app (`"order_placed"`, `"1250.00"`): conta, guarda o último
    /// valor e, se for número, soma. Vai também para as gravações.
    pub fn track(ui: &mut Ui, name: &str, value: &str) {
        ui.report(&Activity::Custom { name, value });
    }

    /// As métricas como estão agora (o mesmo JSON do arquivo).
    pub fn summary(ui: &Ui, id: ComponentId) -> Option<Value> {
        let a = ui.get::<Analytics>(id)?;
        let state = a.state.borrow();
        state.as_ref().map(|m| m.data.clone())
    }

    /// Grava agora.
    pub fn flush(ui: &Ui, id: ComponentId) {
        if let Some(a) = ui.get::<Analytics>(id) {
            if let Some(m) = a.state.borrow_mut().as_mut() {
                m.save();
            }
        }
    }

    fn started(&self) -> bool {
        self.state.borrow().is_some()
    }
}

impl Component for Analytics {
    fn class_name(&self) -> &'static str {
        "Analytics"
    }

    properties! {
        active: bool => "Behavior",
        file: String => "Behavior",
        app_name: String => "Behavior"
    }

    fn timer_interval(&self) -> Option<f64> {
        (self.active && !self.started()).then_some(0.001)
    }

    fn timer(&mut self, ctx: &mut EventCtx<'_>) {
        if ctx.ui.design_mode() {
            return;
        }
        let id = ctx.id;
        ctx.ui.poster().post(move |ui| {
            Analytics::start(ui, id);
            ui.sync_timer(id);
        });
    }

    fn property_changed(&mut self, ctx: &mut EventCtx<'_>, name: &str) {
        if name == "active" && !self.active && self.started() {
            let id = ctx.id;
            ctx.ui.poster().post(move |ui| {
                Analytics::stop(ui, id);
                ui.sync_timer(id);
            });
        }
    }
}
