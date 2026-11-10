//! WebView: uma página em HTML conversando com o app (`window.ipc.postMessage` → `on_message`,
//! e o app responde com `eval`). Com `--url`, abre um endereço.
//!
//! ```text
//! cargo run -p aylis-platform-winit --example browser --features webview
//! cargo run -p aylis-platform-winit --example browser --features webview -- --url https://www.rust-lang.org
//! AYLIS_AUTO_CLOSE=1 ...   # fecha sozinho depois da conversa (teste)
//! ```

use aylis_controls::*;
use aylis_core::*;
use aylis_webview::WebView;

const PAGE: &str = r#"<!doctype html>
<html><body style="font-family: Segoe UI, sans-serif; background: #F3F6FB; margin: 24px">
<h2>Página dentro do aylis</h2>
<p>Esta página é um WebView2/WKWebView sobre o form.</p>
<button onclick="window.ipc.postMessage('clique na página')">Mandar para o app</button>
<p id="resposta"></p>
<script>window.addEventListener('load', () => window.ipc.postMessage('página carregada'));</script>
</body></html>"#;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    aylis_webview::register(&mut ui);
    let form = ui.add_root(Form::new("aylis — WebView", 900.0, 640.0));
    ui.set_name(form, "frmNavegador")?;
    ui.control_mut(form).unwrap().set_padding(12.0);

    let status = ui.add(Label::new("Esperando a página…"), form);
    ui.control_mut(status).unwrap().set_align(Align::Bottom).set_height(28.0);

    let web = match std::env::args().skip_while(|a| a != "--url").nth(1) {
        Some(url) => WebView::new(url),
        None => {
            let mut w = WebView::new("");
            w.load_html(PAGE);
            w
        }
    };
    let web = ui.add(web, form);
    ui.control_mut(web).unwrap().set_align(Align::Client);

    let auto_close = std::env::var("AYLIS_AUTO_CLOSE").is_ok();
    ui.on(web, "on_load", move |ui, _, args| {
        if let EventArgs::Text(url) = args {
            println!("on_load: {}", if url.len() > 60 { &url[..60] } else { url });
            let _ = ui.set_prop(status, "text", format!("Carregada: {url}"));
        }
    });
    ui.on(web, "on_message", move |ui, id, args| {
        let EventArgs::Text(text) = args else { return };
        println!("on_message: {text}");
        let _ = ui.set_prop(status, "text", format!("A página disse: {text}"));
        if text == "resposta mostrada" {
            if auto_close {
                ui.request_close();
            }
            return;
        }
        if let Some(w) = ui.get::<WebView>(id) {
            w.eval(format!("document.getElementById('resposta').textContent = 'O app recebeu: {text}'; window.ipc.postMessage('resposta mostrada')"));
        }
    });
    aylis_platform_winit::run(ui, form)
}
