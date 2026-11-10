//! Taskbar e mensagens da janela (Windows): progresso e indicador no botão da barra de
//! tarefas, botões na miniatura (passe o mouse no botão), tarefas no clique direito, e um
//! gancho que impede maximizar pelo menu da janela.
//!
//! ```text
//! cargo run -p aylis-platform-winit --example taskbar --features cpu
//! ```
//!
//! `AYLIS_AUTO_CLOSE=1` fecha em 3 s (teste); `AYLIS_TASKBAR_DEBUG=1` mostra cada chamada ao
//! shell.

use std::cell::Cell;
use std::rc::Rc;

use aylis_controls::*;
use aylis_core::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("aylis — barra de tarefas", 480.0, 220.0));
    let status = ui.add(Label::new("Passe o mouse no botão do app na barra de tarefas."), form);
    ui.control_mut(status).unwrap().set_align(Align::Client).set_margins([16.0, 16.0, 16.0, 16.0]);
    let data = concat!(env!("CARGO_MANIFEST_DIR"), "/examples/data/");
    let mut bar = Taskbar::default();
    bar.progress = TaskbarProgress::Normal;
    bar.value = 40.0;
    bar.badge = "5".into();
    bar.buttons = format!("{data}previous.png|Anterior;{data}play.png|Tocar;{data}next.png|Próxima");
    bar.tasks = "Nova mensagem|--nova;Abrir caixa de entrada|--entrada".into();
    let bar = ui.add(bar, form);
    ui.on(bar, "on_button", move |ui, _, args| {
        let _ = ui.set_prop(status, "text", format!("Botão da miniatura: {args:?}"));
    });

    // Ganchos nas mensagens do Windows: conta todas e impede maximizar pelo menu.
    let count = Rc::new(Cell::new(0u32));
    {
        use aylis_platform_winit::win::{MessageReply, msg, on_message};
        let count = count.clone();
        on_message(form, move |m| {
            count.set(count.get() + 1);
            if m.message == msg::WM_SYSCOMMAND && m.wparam & 0xFFF0 == msg::SC_MAXIMIZE {
                return MessageReply::Handled(0);
            }
            MessageReply::Default
        });
    }
    if std::env::var("AYLIS_AUTO_CLOSE").is_ok() {
        let timer = ui.add(Timer::new(3000), form);
        ui.on(timer, "on_timer", move |ui, _, _| {
            println!("mensagens vistas pelo gancho: {}", count.get());
            ui.request_close();
        });
    }
    aylis_platform_winit::run(ui, form)
}
