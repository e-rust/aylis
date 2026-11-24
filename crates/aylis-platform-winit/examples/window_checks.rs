//! A conferência dos defeitos de janela e de entrada: um cenário por vez, que escreve no console
//! o que aconteceu (para conferir na tela ou com o mouse e o teclado simulados).
//!
//! ```text
//! cargo run -p aylis-platform-winit --example window_checks -- <cenário> [--close N]
//! ```
//!
//! - `monitor` (item 2): escreve o `form_monitor` no `on_window_created` (antes vinha `None`);
//! - `activate` (item 3): o form maximizado escreve `ativado` ao ganhar o foco na abertura;
//! - `alt_tab` (item 4): `alt_tab = false`; 1 s depois vai para a tela cheia, 2 s depois volta,
//!   3 s depois maximiza (a janela não pode voltar à barra de tarefas em nenhum passo); 4 s
//!   depois religa o `alt_tab` pela propriedade (a janela volta à barra sem `UpdateOptions`);
//! - `races` (item 5): maximizada; 1 s depois `Restore` + `SetFullscreen(true)` na mesma
//!   passada, 2,5 s depois `SetFullscreen(false)` + `Maximize`; escreve o lugar em cada passo;
//! - `topmost` (item 6): `topmost` com `no_activate`; a cada 2 s (até 10 s) pede `UpdateLevel` e, aos 3 s,
//!   `MoveTo` (o foco não pode vir para ela);
//! - `hide_show`: `topmost` com `no_activate` (sem mexer no `activate_on_show`); 1,5 s depois
//!   `Hide`, 3 s depois `Show`, 4,5 s depois `Hide` de novo e 6 s depois `Place` sem ativar (a
//!   janela tem de voltar nos dois casos sem tirar o foco de quem está na frente);
//! - `enter` (item 12): o botão com foco abre uma pergunta no Enter; escreve a resposta;
//! - `text` (o `VK_PACKET`): um `Edit` com foco escreve cada texto que recebe;
//! - `center` (item 33): um segundo form `position = center` abre, é movido, fecha e abre de
//!   novo; escreve onde abriu cada vez (no centro; com `--remember`, onde estava);
//! - `edge` (item 30): a barra própria (`border_style = custom`) com um item para arrastar
//!   perto da borda de cima: com o botão apertado, a borda não vira a de redimensionar;
//! - `grab` (item 38): o form inteiro com o cursor `grab` (e `grabbing` com o botão apertado);
//! - `blur`: um form de vidro (`WindowSettings.blur`) que desliga e religa o desfoque a cada 2 s
//!   (até 8 s; item 20; o logon e os "Efeitos de transparência" se conferem à mão).

use aylis_core::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let scenario = args.get(1).cloned().unwrap_or_else(|| "monitor".into());
    let value = |name: &str| args.iter().position(|a| a == name).and_then(|i| args.get(i + 1)).cloned();
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.create("Form", None)?;
    ui.set_prop(form, "caption", format!("window_checks {scenario}"))?;
    ui.set_prop(form, "width", 640.0)?;
    ui.set_prop(form, "height", 400.0)?;
    let label = ui.create("Label", Some(form))?;
    ui.set_prop(label, "align", "top")?;
    ui.set_prop(label, "height", 40.0)?;
    ui.set_prop(label, "text", format!("cenário: {scenario}"))?;
    let say = |text: String| {
        println!("{text}");
        let _ = std::io::Write::flush(&mut std::io::stdout());
    };
    ui.on(form, "on_window_created", move |ui, f, _| say(format!("on_window_created: monitor {:?}", ui.form_monitor(f))));
    ui.on(form, "on_activate", move |_, _, _| say("ativado".into()));
    ui.on(form, "on_deactivate", move |_, _, _| say("desativado".into()));
    let place = move |ui: &Ui, f: ComponentId, step: &str| {
        let p = ui.window_placement(f).unwrap_or_default();
        say(format!("{step}: x {} y {} {}x{} maximizada {} tela cheia {}", p.x, p.y, p.width, p.height, p.maximized, p.full_screen));
    };
    match scenario.as_str() {
        "activate" => ui.set_prop(form, "window_state", "maximized")?,
        "alt_tab" => {
            let settings = ui.create("WindowSettings", Some(form))?;
            ui.set_prop(settings, "alt_tab", false)?;
            let steps: [(u64, &'static str); 4] = [(1000, "tela cheia"), (2000, "normal"), (3000, "maximizada"), (4000, "alt_tab ligado")];
            for (at, step) in steps {
                ui.set_timeout(at, move |ui| {
                    match step {
                        "tela cheia" => ui.window_command(form, WindowCommand::SetFullscreen(true)),
                        "normal" => ui.window_command(form, WindowCommand::SetFullscreen(false)),
                        "maximizada" => ui.window_command(form, WindowCommand::Maximize),
                        _ => {
                            let _ = ui.set_prop(settings, "alt_tab", true);
                        }
                    }
                    say(format!("passo: {step}"));
                });
            }
        }
        "races" => {
            ui.set_prop(form, "window_state", "maximized")?;
            ui.set_timeout(1000, move |ui| {
                place(ui, form, "antes");
                ui.window_command(form, WindowCommand::Restore);
                ui.window_command(form, WindowCommand::SetFullscreen(true));
            });
            ui.set_timeout(2000, move |ui| place(ui, form, "tela cheia"));
            ui.set_timeout(2500, move |ui| {
                ui.window_command(form, WindowCommand::SetFullscreen(false));
                ui.window_command(form, WindowCommand::Maximize);
            });
            ui.set_timeout(3500, move |ui| place(ui, form, "maximizada"));
        }
        "topmost" => {
            ui.set_prop(form, "topmost", true)?;
            ui.set_prop(form, "activate_on_show", false)?;
            let settings = ui.create("WindowSettings", Some(form))?;
            ui.set_prop(settings, "no_activate", true)?;
            for at in [2000, 4000, 6000, 8000, 10000] {
                ui.set_timeout(at, move |ui| ui.window_command(form, WindowCommand::UpdateLevel));
            }
            ui.set_timeout(3000, move |ui| {
                ui.window_command(form, WindowCommand::MoveTo(100, 100));
                say("passo: MoveTo".into());
            });
        }
        "hide_show" => {
            ui.set_prop(form, "topmost", true)?;
            let settings = ui.create("WindowSettings", Some(form))?;
            ui.set_prop(settings, "no_activate", true)?;
            let steps: [(u64, &'static str); 4] = [(1500, "Hide"), (3000, "Show"), (4500, "Hide"), (6000, "Place")];
            for (at, step) in steps {
                ui.set_timeout(at, move |ui| {
                    ui.window_command(
                        form,
                        match step {
                            "Hide" => WindowCommand::Hide,
                            "Show" => WindowCommand::Show,
                            _ => WindowCommand::Place { monitor: 0, x: 200, y: 200, width: 500, height: 300, activate: false },
                        },
                    );
                    say(format!("passo: {step}"));
                });
            }
        }
        "enter" => {
            let button = ui.create("Button", Some(form))?;
            ui.set_prop(button, "text", "Enter abre a pergunta")?;
            ui.set_prop(button, "left", 20.0)?;
            ui.set_prop(button, "top", 60.0)?;
            ui.set_prop(button, "width", 240.0)?;
            ui.on(button, "on_key_down", move |ui, _, args| {
                if matches!(args, EventArgs::Key(k) if k.key == Key::Enter) {
                    say("pergunta aberta".into());
                    aylis_controls::confirm(ui, "Remover?", "Remover este item?", "Remover", true, move |_, yes| say(format!("respondeu: {yes}")));
                }
            });
            ui.set_focus(button);
        }
        "text" => {
            let edit = ui.create("Edit", Some(form))?;
            ui.set_prop(edit, "left", 20.0)?;
            ui.set_prop(edit, "top", 60.0)?;
            ui.set_prop(edit, "width", 400.0)?;
            ui.on(edit, "on_change", move |ui, e, _| say(format!("texto: {:?}", ui.get_prop(e, "text").map(|v| format!("{v:?}")).unwrap_or_default())));
            ui.set_focus(edit);
        }
        "center" => {
            let other = ui.create("Form", None)?;
            ui.set_prop(other, "name", "frmWindowChecksCenter")?;
            ui.set_prop(other, "caption", "window_checks centro")?;
            ui.set_prop(other, "width", 300.0)?;
            ui.set_prop(other, "height", 200.0)?;
            ui.set_prop(other, "position", "center")?;
            // `--remember`: com o `remember_placement`, a segunda abertura é onde ele estava.
            if args.iter().any(|a| a == "--remember") {
                ui.set_prop(other, "remember_placement", true)?;
            }
            ui.set_timeout(800, move |ui| {
                let _ = ui.show(other);
            });
            ui.set_timeout(1600, move |ui| {
                place(ui, other, "primeira abertura");
                ui.window_command(other, WindowCommand::MoveTo(40, 40));
            });
            ui.set_timeout(2400, move |ui| {
                place(ui, other, "movida");
                ui.close(other);
            });
            ui.set_timeout(3200, move |ui| {
                let _ = ui.show(other);
            });
            ui.set_timeout(4000, move |ui| place(ui, other, "segunda abertura"));
        }
        "edge" => {
            ui.set_prop(form, "border_style", "custom")?;
            let item = ui.create("Label", Some(form))?;
            ui.set_prop(item, "left", 20.0)?;
            ui.set_prop(item, "top", 60.0)?;
            ui.set_prop(item, "width", 160.0)?;
            ui.set_prop(item, "height", 40.0)?;
            ui.set_prop(item, "text", "Arraste pela borda")?;
            ui.set_prop(item, "draggable", true)?;
            ui.set_prop(item, "drag_data", "item")?;
            ui.on(item, "on_drag_end", move |_, _, args| say(format!("fim do arrasto: {args:?}")));
        }
        "grab" => {
            let panel = ui.create("Panel", Some(form))?;
            ui.set_prop(panel, "align", "client")?;
            ui.set_prop(panel, "cursor", "grab")?;
            ui.on(panel, "on_pointer_down", move |ui, p, _| {
                let _ = ui.set_prop(p, "cursor", "grabbing");
            });
            ui.on(panel, "on_pointer_up", move |ui, p, _| {
                let _ = ui.set_prop(p, "cursor", "grab");
            });
        }
        "blur" => {
            ui.set_prop(form, "transparent", true)?;
            let settings = ui.create("WindowSettings", Some(form))?;
            ui.set_prop(settings, "blur", true)?;
            for (i, at) in [2000, 4000, 6000, 8000].into_iter().enumerate() {
                ui.set_timeout(at, move |ui| {
                    let on = i % 2 == 1;
                    let _ = ui.set_prop(settings, "blur", on);
                    say(format!("blur: {on}"));
                });
            }
        }
        _ => {}
    }
    if let Some(ms) = value("--close").and_then(|v| v.parse::<u64>().ok()) {
        ui.set_timeout(ms, |ui| ui.request_close());
    }
    aylis_platform_winit::run(ui, form)?;
    Ok(())
}
