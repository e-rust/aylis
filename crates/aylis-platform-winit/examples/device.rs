//! Recursos do aparelho: biometria (`Biometrics`), permissões (`Permissions`) e arquivos
//! (`OpenDialog`, `SaveDialog` e `aylis_core::files`), com o resultado de cada um na tela.
//!
//! ```text
//! cargo run -p aylis-platform-winit --example device --features d2d
//! ```

use aylis_controls::*;
use aylis_core::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    ui.set_theme(Theme::light());
    let form = ui.add_root(Form::new("aylis — aparelho", 520.0, 420.0));
    ui.control_mut(form).unwrap().set_padding(16.0);

    let log = ui.add(Label::new("Toque num botão."), form);
    ui.control_mut(log).unwrap().set_align(Align::Bottom).set_height(150.0);
    ui.set_prop(log, "wrap", true)?;
    let say = move |ui: &mut Ui, text: String| {
        let _ = ui.set_prop(log, "text", text);
    };

    let bio = ui.add(Biometrics::default(), form);
    ui.set_prop(bio, "reason", "Confirme para entrar no app")?;
    ui.set_prop(bio, "allow_device_credential", true)?;
    ui.on(bio, "on_success", move |ui, _, _| say(ui, "Biometria: confirmado.".into()));
    ui.on(bio, "on_cancel", move |ui, _, _| say(ui, "Biometria: cancelado.".into()));
    ui.on(bio, "on_fail", move |ui, _, args| say(ui, format!("Biometria: falhou ({})", args.text().unwrap_or_default().to_string())));

    let perm = ui.add(Permissions::default(), form);
    ui.set_prop(perm, "permissions", "camera;microphone;location;notifications")?;
    ui.on(perm, "on_result", move |ui, _, args| say(ui, format!("Permissões: {}", args.text().unwrap_or_default().to_string())));

    let open = ui.add(OpenDialog::default(), form);
    ui.set_prop(open, "multiple", true)?;
    ui.on(open, "on_select", move |ui, _, args| {
        let text = args.text().unwrap_or_default().to_string();
        let lines: Vec<String> = text
            .split(';')
            .map(|p| format!("{} ({} bytes)", files::file_name(p), files::read(p).map_or(0, |d| d.len())))
            .collect();
        say(ui, format!("Escolhidos:\n{}", lines.join("\n")));
    });
    ui.on(open, "on_cancel", move |ui, _, _| say(ui, "Nenhum arquivo escolhido.".into()));
    let save = ui.add(SaveDialog::default(), form);
    ui.set_prop(save, "file_name", "nota.txt")?;
    ui.on(save, "on_select", move |ui, _, args| {
        let path = args.text().unwrap_or_default().to_string();
        let result = files::write(&path, "Gravado pelo aylis.\n".as_bytes());
        say(ui, format!("Salvar em {path}: {}", result.map_or_else(|e| e.to_string(), |_| "ok".into())));
    });

    let buttons: [(&str, Box<dyn Fn(&mut Ui)>); 5] = [
        ("Biometria", Box::new(move |ui: &mut Ui| {
            let kind = Biometrics::availability(ui);
            say(ui, format!("Biometria disponível: {}", kind.name()));
            Biometrics::authenticate(ui, bio);
        })),
        ("Estado das permissões", Box::new(move |ui: &mut Ui| {
            let text = platform::Permission::ALL.iter().map(|&p| format!("{}: {}", p.name(), Permissions::state(ui, p).name())).collect::<Vec<_>>().join("\n");
            say(ui, text);
        })),
        ("Pedir permissões", Box::new(move |ui: &mut Ui| Permissions::request_all(ui, perm))),
        ("Abrir arquivos", Box::new(move |ui: &mut Ui| OpenDialog::execute(ui, open))),
        ("Salvar arquivo", Box::new(move |ui: &mut Ui| SaveDialog::execute(ui, save))),
    ];
    for (text, action) in buttons {
        let b = ui.add(Button::new(text), form);
        ui.control_mut(b).unwrap().set_align(Align::Top).set_height(40.0).set_margins([0.0, 0.0, 0.0, 8.0]);
        ui.on_click(b, move |ui, _| action(ui));
    }
    aylis_platform_winit::run(ui, form)
}
