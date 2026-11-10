//! Esta IDE no registro das IDEs abertas ([`aylis_tools::instances`]): anota o projeto aberto,
//! a porta do MCP e o foco, e atende o canal de controle (outra IDE aberta por um duplo clique
//! manda o arquivo para cá, ou pede para esta vir para a frente).

use std::cell::RefCell;

use aylis_core::{ComponentId, Ui, WindowCommand};
use aylis_tools::instances::{Command, Instance};

use crate::app::Ids;
use crate::state::Shared;

thread_local! {
    static ME: RefCell<Option<Instance>> = const { RefCell::new(None) };
    /// O estado e o form da IDE, para os comandos que chegam pelo canal (a thread dele manda
    /// pela fila da interface, que roda aqui).
    static TARGET: RefCell<Option<(Shared, Ids)>> = const { RefCell::new(None) };
}

/// Anota esta IDE e liga o canal de controle.
pub fn start(ui: &mut Ui, state: &Shared, ids: &Ids) {
    TARGET.with(|t| *t.borrow_mut() = Some((state.clone(), *ids)));
    let secret = aylis_tools::instances::new_secret();
    let poster = ui.poster();
    let port = aylis_tools::instances::listen(secret.clone(), move |command| {
        poster.post(move |ui| handle(ui, command));
    });
    let Ok(control_port) = port else { return };
    // O MCP pode ter ligado antes (ao montar a IDE).
    let mcp_port = state.borrow().mcp.as_ref().map(|s| s.port());
    // Aberta pela ponte do MCP para uma sessão de agente: a marca dela (a ponte espera esta IDE).
    let sessions = std::env::var("AYLIS_IDE_MCP_SESSION").ok().filter(|s| !s.is_empty()).into_iter().collect();
    let mut me = Instance { pid: std::process::id(), project: None, mcp_port, control_port, secret, focused_at: 0.0, sessions };
    me.touch();
    ME.with(|m| *m.borrow_mut() = Some(me));
    // Ganhar o foco conta para o MCP escolher esta IDE.
    ui.on(ids.form, "on_activate", |_, _, _| focused());
}

fn handle(ui: &mut Ui, command: Command) {
    let Some((state, ids)) = TARGET.with(|t| t.borrow().clone()) else { return };
    if let Command::Open(path) = &command {
        crate::actions::open_recent(ui, &state, &ids, path);
        crate::app::refresh(ui, &state, &ids);
    }
    bring_to_front(ui, ids.form);
}

fn bring_to_front(ui: &mut Ui, form: ComponentId) {
    ui.window_command(form, WindowCommand::Show);
}

/// O projeto aberto mudou (ou fechou): atualiza o registro (o refresh chama a cada volta; só
/// grava quando muda).
pub fn sync(state: &Shared) {
    let project = state.borrow().project.as_ref().map(|p| p.path.clone());
    ME.with(|m| {
        if let Some(me) = m.borrow_mut().as_mut()
            && me.project != project
        {
            me.project = project;
            let _ = me.save();
        }
    });
}

/// A porta em que o servidor MCP desta IDE ficou (`None`: desligado).
pub fn set_mcp_port(port: Option<u16>) {
    ME.with(|m| {
        if let Some(me) = m.borrow_mut().as_mut() {
            me.mcp_port = port;
            let _ = me.save();
        }
    });
}

/// Uma sessão de agente entrou (`connected`) ou saiu desta IDE: o registro diz à ponte das
/// outras sessões que esta IDE está ocupada (ou livre de novo).
pub fn set_session(session: &str, connected: bool) {
    ME.with(|m| {
        if let Some(me) = m.borrow_mut().as_mut() {
            let had = me.sessions.iter().any(|s| s == session);
            if connected && !had {
                me.sessions.push(session.to_string());
            } else if !connected && had {
                me.sessions.retain(|s| s != session);
            } else {
                return;
            }
            let _ = me.save();
        }
    });
}

fn focused() {
    ME.with(|m| {
        if let Some(me) = m.borrow_mut().as_mut() {
            me.touch();
        }
    });
}

/// Tira esta IDE do registro (ao fechar).
pub fn stop() {
    aylis_tools::instances::unregister(std::process::id());
}
