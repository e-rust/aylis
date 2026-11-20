//! O form que entra em outro e sai dele (como as abas do navegador): `host` do form, pelo
//! código, ou arrastando. Embutido, o form fica no contêiner (numa faixa de abas, vira uma aba
//! com o título dele) e continua sendo o mesmo form: os mesmos componentes, dados e handlers.
//! Solto, ele volta a ter janela própria, onde foi solto ou onde estava.
//!
//! Arrastando: a aba de um form puxada para fora da faixa vira a janela dele; a janela de um
//! form com `dockable`, arrastada pela barra de título até uma faixa com `accept_forms`, entra
//! nela como aba (a faixa marca o lugar enquanto a janela passa por cima).

use aylis_core::{Align, ComponentId, EventArgs, PropValue, Ui, WindowPlacement};

use crate::{Form, TabStrip};

/// Embute o form `form` em `host` (numa faixa de abas, como aba no lugar `slot`, ou no fim).
/// Aberto numa janela, ela some (sem fechar o form). O form principal e um modal não entram.
pub fn dock_form(ui: &mut Ui, form: ComponentId, host: ComponentId, slot: Option<usize>) -> Result<(), String> {
    if ui.get::<Form>(form).is_none() {
        return Err("só um Form entra em outro".into());
    }
    if form == host || ui.root_of(host) == form {
        return Err("o form não entra em si mesmo".into());
    }
    if ui.main_form() == Some(form) {
        return Err("o form principal não entra em outro".into());
    }
    if ui.is_modal(form) {
        return Err("um form modal não entra em outro".into());
    }
    // Já embutido noutro lugar: sai de lá primeiro (de uma faixa, a aba sai sem liberar).
    leave_strip(ui, form);
    // O tamanho de janela dele, para voltar igual.
    let size = ui.control(form).map(|cb| (cb.width(), cb.height()));
    if let (Some(f), Some(size)) = (ui.get_mut::<Form>(form), size) {
        if f.undocked_size.is_none() {
            f.undocked_size = Some(size);
        }
    }
    if ui.is_open(form) && !ui.release_window(form) {
        return Err("o form não larga a janela".into());
    }
    if ui.get::<TabStrip>(host).is_some() {
        let caption = ui.get::<Form>(form).map(|f| f.caption.clone()).unwrap_or_default();
        let index = TabStrip::add_page(ui, host, &caption, "", form);
        if let Some(slot) = slot.filter(|&s| s < index) {
            if let Some(s) = ui.get_mut::<TabStrip>(host) {
                s.move_tab(index, slot);
            }
            TabStrip::sync_pages(ui, host);
        }
    } else {
        ui.set_parent(form, Some(host)).map_err(|e| e.to_string())?;
    }
    if let Some(cb) = ui.control_mut(form) {
        cb.set_align(Align::Client);
        cb.set_visible(true);
    }
    if let Some(f) = ui.get_mut::<Form>(form) {
        f.host = Some(host);
        f.docked_in = Some(host);
    }
    ui.layout();
    ui.invalidate();
    ui.fire(form, "on_dock", &EventArgs::Value(PropValue::Component(Some(host))));
    Ok(())
}

/// Solta o form embutido: volta a ter janela própria, com o canto em `at` (pixels físicos da
/// tela; `None`: onde a janela estava antes de entrar) e o tamanho de antes.
pub fn undock_form(ui: &mut Ui, form: ComponentId, at: Option<(i32, i32)>) -> Result<(), String> {
    if ui.get::<Form>(form).is_none() {
        return Err("só um Form sai de outro".into());
    }
    if ui.parent(form).is_none() {
        return Ok(());
    }
    leave_strip(ui, form);
    ui.set_parent(form, None).map_err(|e| e.to_string())?;
    let size = ui.get_mut::<Form>(form).and_then(|f| {
        f.host = None;
        f.docked_in = None;
        f.undocked_size.take()
    });
    if let Some(cb) = ui.control_mut(form) {
        cb.set_align(Align::None);
        cb.set_position(0.0, 0.0);
        if let Some((w, h)) = size {
            cb.set_size(w, h);
        }
        cb.set_visible(true);
    }
    if let Some((x, y)) = at {
        ui.set_window_placement(form, WindowPlacement { x, y, width: 0, height: 0, maximized: false, full_screen: false });
    }
    ui.show(form).map_err(|e| e.to_string())?;
    // Como a aba do navegador que vira janela: independente, não presa por cima de outra.
    ui.set_form_owner(form, None);
    ui.fire(form, "on_undock", &EventArgs::None);
    Ok(())
}

/// O form que é aba de uma faixa sai dela (sem liberar a página).
fn leave_strip(ui: &mut Ui, form: ComponentId) {
    let strip = ui.get::<Form>(form).and_then(|f| f.docked_in).filter(|&h| ui.get::<TabStrip>(h).is_some());
    let Some(strip) = strip else { return };
    let index = ui.get::<TabStrip>(strip).and_then(|s| s.tabs().iter().position(|t| t.page == Some(form)));
    if let Some(index) = index {
        let was = ui.get::<TabStrip>(strip).map(|s| s.tab_index);
        if let Some(s) = ui.get_mut::<TabStrip>(strip) {
            s.remove(index);
        }
        TabStrip::sync_pages(ui, strip);
        if ui.get::<TabStrip>(strip).map(|s| s.tab_index) != was {
            let index = ui.get::<TabStrip>(strip).map_or(0, |s| s.tab_index);
            ui.fire(strip, "on_change", &EventArgs::Value(index.into()));
        }
    }
}
