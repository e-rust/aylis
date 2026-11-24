//! O foco do teclado ao esconder ou liberar um `WebView` que o tinha: 2,5 s depois de carregar,
//! a página põe o cursor num campo sozinha (com o app em primeiro plano, o WebView2 toma o foco
//! do sistema sem clique); 1,5 s depois o WebView some, e as teclas têm de voltar ao `Edit` do
//! form. Escreve no console onde está o foco do sistema (`GetFocus`) antes e depois, e cada texto
//! que o `Edit` recebe.
//!
//! ```text
//! cargo run -p aylis-platform-winit --features webview --example webview_focus -- [--free] [--close N]
//! ```
//!
//! `--free` libera o WebView em vez de escondê-lo. Para conferir com o teclado simulado: clique
//! no `Edit` (o app vai para a frente), espere o `passo:` e digite.

use aylis_core::*;
use aylis_webview::WebView;

const PAGE: &str = r#"<!doctype html>
<html><body style="font-family: Segoe UI, sans-serif; margin: 24px">
<p>O campo abaixo ganha o cursor sozinho.</p>
<input id="campo" style="width: 90%; height: 120px">
<script>
window.addEventListener('load', () => setTimeout(() => {
  document.getElementById('campo').focus();
  window.ipc.postMessage('campo com foco');
}, 2500));
</script>
</body></html>"#;

/// Onde está o foco do teclado do sistema, visto da thread da interface.
fn system_focus() -> &'static str {
    #[cfg(windows)]
    {
        use windows::Win32::UI::Input::KeyboardAndMouse::GetFocus;
        use windows::Win32::UI::WindowsAndMessaging::{GA_ROOT, GetAncestor, GetForegroundWindow};
        // SAFETY: só leituras de janelas; todas aceitam a janela nula.
        unsafe {
            let (focus, top) = (GetFocus(), GetForegroundWindow());
            if focus.is_invalid() {
                "nenhuma janela desta thread"
            } else if focus == top {
                "a janela do form"
            } else if GetAncestor(focus, GA_ROOT) == top {
                "uma janela filha (a vista)"
            } else {
                "outra janela"
            }
        }
    }
    #[cfg(not(windows))]
    "(só no Windows)"
}

fn say(text: String) {
    println!("{text}");
    let _ = std::io::Write::flush(&mut std::io::stdout());
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let free = args.iter().any(|a| a == "--free");
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    aylis_webview::register(&mut ui);
    let form = ui.create("Form", None)?;
    ui.set_prop(form, "caption", "webview_focus")?;
    ui.set_prop(form, "width", 640.0)?;
    ui.set_prop(form, "height", 420.0)?;
    let edit = ui.create("Edit", Some(form))?;
    ui.set_prop(edit, "align", "top")?;
    ui.on(edit, "on_change", move |ui, e, _| say(format!("texto: {:?}", ui.get_prop(e, "text").map(|v| format!("{v:?}")).unwrap_or_default())));
    let mut web = WebView::new("");
    web.load_html(PAGE);
    let web = ui.add(web, form);
    ui.control_mut(web).unwrap().set_align(Align::Client);
    ui.set_focus(edit);
    ui.on(web, "on_message", move |ui, _, args| {
        let EventArgs::Text(text) = args else { return };
        say(format!("página: {text}"));
        ui.set_timeout(1000, move |_| say(format!("foco antes: {}", system_focus())));
        ui.set_timeout(1500, move |ui| {
            if free {
                ui.free(web);
                say("passo: liberado".into());
            } else {
                let _ = ui.set_prop(web, "visible", false);
                say("passo: escondido".into());
            }
        });
        ui.set_timeout(2000, move |_| say(format!("foco depois: {}", system_focus())));
    });
    if let Some(ms) = args.iter().position(|a| a == "--close").and_then(|i| args.get(i + 1)).and_then(|v| v.parse::<u64>().ok()) {
        ui.set_timeout(ms, |ui| ui.request_close());
    }
    aylis_platform_winit::run(ui, form)?;
    Ok(())
}
