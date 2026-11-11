//! O aviso de versão nova do Aylis, como no NoOBS: uma consulta por dia às releases do GitHub
//! (depois das 6 h locais; veja `aylis_tools::updates`) e, havendo versão nova, a faixa
//! "Nova versão disponível" embaixo da barra de ferramentas, com "Abrir página" e o ✕ que
//! dispensa até a IDE abrir de novo.
//!
//! Parado não gasta: um só `set_timeout` até a próxima consulta, sem tique. A IDE compilada do
//! repositório (o executável dentro de um `target/`) não consulta sozinha: só pelo menu.

use std::cell::{Cell, RefCell};

use aylis_core::{Align, ComponentId, Ui};
use aylis_controls::Panel;
use serde_json::Value;

use crate::widgets::{label, tool};

/// A opção (liga/desliga) e o instante da última consulta respondida, no config da IDE.
const OPTION: &str = "check_updates";
const LAST: &str = "last_update_check";
/// Sem resposta do GitHub (sem rede), tenta de novo depois disto.
const RETRY_MS: u64 = 30 * 60 * 1000;

#[derive(Clone, Copy)]
struct Banner {
    panel: ComponentId,
    text: ComponentId,
}

#[derive(Default)]
struct Session {
    checking: bool,
    timer: Option<u64>,
    /// A página da versão nova (o "Abrir página").
    url: String,
    /// A versão que o ✕ dispensou nesta sessão (não volta até abrir a IDE de novo).
    dismissed: String,
    /// O item "Avisar de novas versões" do menu Ajuda.
    menu_item: Option<ComponentId>,
}

thread_local! {
    static BANNER: Cell<Option<Banner>> = const { Cell::new(None) };
    static SESSION: RefCell<Session> = RefCell::new(Session::default());
}

/// A versão desta IDE.
pub fn current() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// A consulta automática está ligada (o padrão).
pub fn enabled() -> bool {
    crate::actions::config_flag(OPTION, true)
}

/// A IDE compilada do repositório (dentro de um `target/`) ou com `AYLIS_IDE_NO_UPDATE`: sem
/// consulta automática (como o "build de desenvolvimento" do NoOBS).
fn dev_build() -> bool {
    std::env::var_os("AYLIS_IDE_NO_UPDATE").is_some()
        || std::env::current_exe().is_ok_and(|exe| exe.components().any(|c| c.as_os_str().eq_ignore_ascii_case("target")))
}

/// Monta a faixa do aviso (escondida), logo abaixo da barra de ferramentas.
pub fn build_banner(ui: &mut Ui, form: ComponentId) -> ComponentId {
    let panel = ui.add(Panel::new(0.0, 38.0), form);
    ui.control_mut(panel).unwrap().set_align(Align::Top).set_padding([12.0, 4.0, 8.0, 4.0]).set_visible(false);
    let close = tool(ui, panel, "✕", 32.0, "flat");
    ui.control_mut(close).unwrap().set_align(Align::Right).set_margins([4.0, 0.0, 0.0, 0.0]);
    let _ = ui.set_prop(close, "hint", "Dispensar o aviso (volta quando a IDE abrir de novo)");
    let open = tool(ui, panel, "Abrir página", 110.0, "");
    ui.control_mut(open).unwrap().set_align(Align::Right).set_margins([8.0, 0.0, 0.0, 0.0]);
    let text = label(ui, panel, "", Align::Client, 0.0);
    let _ = ui.set_prop(text, "style", "warning");
    ui.on_click(open, |ui, _| {
        let url = SESSION.with(|s| s.borrow().url.clone());
        let url = if url.starts_with("https://") { url } else { aylis_tools::updates::releases_url() };
        if let Err(e) = aylis_tools::shell::open(&url) {
            aylis_controls::toast(ui, &format!("⚠ {e}"));
        }
        hide(ui);
    });
    ui.on_click(close, |ui, _| hide(ui));
    BANNER.with(|b| b.set(Some(Banner { panel, text })));
    panel
}

/// O item "Avisar de novas versões" do menu (marcado conforme a opção).
pub fn set_menu_item(ui: &mut Ui, item: ComponentId) {
    let _ = ui.set_prop(item, "checked", enabled());
    SESSION.with(|s| s.borrow_mut().menu_item = Some(item));
}

/// Liga ou desliga a consulta automática (o item do menu Ajuda).
pub fn toggle(ui: &mut Ui) {
    let on = !enabled();
    crate::actions::update_config(OPTION, Value::from(on));
    if let Some(item) = SESSION.with(|s| s.borrow().menu_item) {
        let _ = ui.set_prop(item, "checked", on);
    }
    if on {
        start(ui);
    } else if let Some(timer) = SESSION.with(|s| s.borrow_mut().timer.take()) {
        ui.clear_timeout(timer);
    }
}

/// A IDE abriu: agenda a consulta do dia (agora, se já passou da hora).
pub fn start(ui: &mut Ui) {
    if enabled() && !dev_build() && crate::lite::can(crate::lite::Capability::Disk) {
        schedule(ui, None);
    }
}

fn now() -> i64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs() as i64)
}

/// Marca a próxima consulta: daqui a `ms`, ou no próximo horário do dia.
fn schedule(ui: &mut Ui, ms: Option<u64>) {
    let ms = ms.unwrap_or_else(|| {
        let (last, now) = (crate::actions::config_value(LAST).as_i64().unwrap_or(0), now());
        aylis_tools::updates::seconds_until_due(last, now, aylis_core::platform::local_offset(now)) as u64 * 1000
    });
    if let Some(old) = SESSION.with(|s| s.borrow_mut().timer.take()) {
        ui.clear_timeout(old);
    }
    let timer = ui.set_timeout(ms, |ui| {
        SESSION.with(|s| s.borrow_mut().timer = None);
        check(ui, false);
    });
    SESSION.with(|s| s.borrow_mut().timer = Some(timer));
}

/// Consulta agora (`manual`: pelo menu, e o resultado aparece mesmo sem versão nova).
pub fn check(ui: &mut Ui, manual: bool) {
    if SESSION.with(|s| std::mem::replace(&mut s.borrow_mut().checking, true)) {
        return;
    }
    if manual {
        aylis_controls::toast(ui, &ui.tr("Verificando se há versão nova do Aylis…"));
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let poster = ui.poster();
        std::thread::spawn(move || {
            let fetched = aylis_tools::updates::fetch();
            poster.post(move |ui| finish(ui, fetched, manual));
        });
    }
    #[cfg(target_arch = "wasm32")]
    finish(ui, aylis_tools::updates::fetch(), manual);
}

/// O resultado da consulta, de volta na thread da interface.
pub fn finish(ui: &mut Ui, fetched: aylis_tools::updates::Fetched, manual: bool) {
    SESSION.with(|s| s.borrow_mut().checking = false);
    // Só uma resposta do GitHub conta como a consulta do dia: sem rede, tenta de novo.
    if fetched.status != 0 {
        crate::actions::update_config(LAST, Value::from(now()));
    }
    let update = aylis_tools::updates::pick_update(current(), &fetched.releases).cloned();
    match &update {
        Some(release) => show(ui, &release.tag, &release.url, manual),
        None if manual && fetched.status == 200 => {
            let text = ui.tr(&format!("O Aylis está atualizado (versão {}).", current()));
            aylis_controls::toast(ui, &text);
        }
        None if manual => {
            let text = ui.tr("Não foi possível verificar agora. Confira a conexão e tente de novo.");
            aylis_controls::toast(ui, &text);
        }
        None => {}
    }
    if enabled() && !dev_build() {
        schedule(ui, (fetched.status == 0).then_some(RETRY_MS));
    }
}

/// Mostra a faixa da versão nova (a que o ✕ dispensou só volta pelo menu).
fn show(ui: &mut Ui, tag: &str, url: &str, manual: bool) {
    let Some(banner) = BANNER.with(Cell::get) else { return };
    let skip = SESSION.with(|s| {
        let mut s = s.borrow_mut();
        s.url = url.to_string();
        !manual && s.dismissed == tag
    });
    if skip {
        return;
    }
    let version = tag.trim_start_matches(['v', 'V']);
    let text = format!("Nova versão do Aylis disponível: a {} já pode ser baixada (esta é a {}).", version, current());
    let _ = ui.set_prop(banner.text, "text", text);
    if let Some(c) = ui.control_mut(banner.panel) {
        c.set_visible(true);
    }
    SESSION.with(|s| s.borrow_mut().dismissed = tag.to_string());
    crate::lang::translate(ui);
}

/// Esconde a faixa.
pub fn hide(ui: &mut Ui) {
    if let Some(c) = BANNER.with(Cell::get).and_then(|b| ui.control_mut(b.panel)) {
        c.set_visible(false);
    }
}

/// A faixa está à vista (para os testes).
pub fn banner_visible(ui: &Ui) -> bool {
    BANNER.with(Cell::get).and_then(|b| ui.control(b.panel)).is_some_and(|c| c.visible())
}
