//! Os componentes de verdade na documentação. Cada família de componentes (a categoria da
//! paleta: Standard, Additional, BI, Data Controls...) vira um wasm separado,
//! e a página de um componente carrega só o da família dele, num `<iframe>`: a documentação
//! abre rápido (sem os componentes) e cada família só é baixada quando alguém abre uma página
//! dela (depois fica no cache do navegador). Monte com `aylis-tool demos --out pasta`.
//!
//! A página `index.html?class=Button&theme=dark` mostra um `Button` de verdade, no meio, no
//! tema escuro.

use aylis::prelude::*;

mod family {
    include!(concat!(env!("OUT_DIR"), "/family.rs"));
}

pub use family::register;

/// A folga em volta do componente (a mesma das imagens da documentação).
pub const MARGIN: f32 = aylis::samples::DEMO_MARGIN;

/// As classes visuais sem demonstração ao vivo (veja [`aylis::samples::WITHOUT_DEMO`]).
pub const WITHOUT_DEMO: &[&str] = aylis::samples::WITHOUT_DEMO;

/// O form da demonstração: o componente `class` (o nome dele é `demo`) com o conteúdo de
/// exemplo ([`aylis::samples::demo`]), num form do tamanho dele mais [`MARGIN`] de cada lado.
/// `None` se a classe não é visual (um `Timer` não tem o que mostrar) ou não é desta família.
pub fn demo_form(ui: &mut Ui, class: &str, dark: bool) -> Option<ComponentId> {
    ui.set_theme(if dark { Theme::dark() } else { Theme::light() });
    let form = ui.add_root(aylis::controls::Form::new(class, 640.0, 240.0));
    let Some((_, (w, h))) = aylis::samples::demo(ui, class, form, form) else {
        ui.free(form);
        return None;
    };
    if let Some(cb) = ui.control_mut(form) {
        cb.set_width(w).set_height(h);
    }
    ui.layout();
    Some(form)
}

/// O tamanho natural da demonstração (o componente mais a folga): a documentação ajusta o
/// quadro a ele (no navegador, a página manda por `postMessage`).
pub fn natural_size(ui: &Ui, form: ComponentId) -> (f32, f32) {
    ui.find_in(form, "demo").and_then(|id| ui.control(id)).map_or((0.0, 0.0), |cb| (cb.max_width() + 2.0 * MARGIN, cb.max_height() + 2.0 * MARGIN))
}

/// O que a página pede além da classe: `fill=1` (o componente ocupa o quadro inteiro, como o
/// lingote no topo da página do aylis), `bg=0F1117` (o fundo, o mesmo da página em volta) e
/// `scene=ingot` (no `Model3D`, o lingote da abertura da IDE).
pub fn configure(ui: &mut Ui, form: ComponentId, query: &str) {
    let Some(component) = ui.find_in(form, "demo") else { return };
    if query_value(query, "fill").is_some_and(|v| v == "1") {
        if let Some(cb) = ui.control_mut(component) {
            cb.set_align(Align::Client).set_margins(0.0);
            cb.set_max_width(0.0).set_max_height(0.0);
        }
    }
    if let Some(bg) = query_value(query, "bg") {
        let _ = ui.set_prop(form, "fill", format!("#{bg}"));
    }
    #[cfg(feature = "3d")]
    if query_value(query, "scene").as_deref() == Some("ingot") {
        if let Some(view) = ui.get_mut::<aylis::scene3d::Model3D>(component) {
            aylis::scene3d::ingot::apply(view);
        }
    }
    ui.layout();
}

/// O valor de `key` na consulta da página (`?class=Button&theme=dark`).
pub fn query_value(query: &str, key: &str) -> Option<String> {
    query.trim_start_matches('?').split('&').find_map(|pair| {
        let (k, v) = pair.split_once('=')?;
        (k == key).then(|| v.replace("%20", " "))
    })
}

/// A página chama depois de carregar o WebAssembly.
#[cfg(target_arch = "wasm32")]
#[unsafe(no_mangle)]
pub extern "C" fn aylis_demo_start() {
    std::panic::set_hook(Box::new(|info| web_sys::console::error_1(&info.to_string().into())));
    let query = web_sys::window().and_then(|w| w.location().search().ok()).unwrap_or_default();
    let class = query_value(&query, "class").unwrap_or_else(|| "Button".into());
    let dark = query_value(&query, "theme").is_none_or(|t| t != "light");
    let mut ui = Ui::new();
    register(&mut ui);
    let Some(form) = demo_form(&mut ui, &class, dark) else {
        // Sem demonstração: a página em volta fecha o quadro e mostra a imagem.
        if let Some(parent) = web_sys::window().and_then(|w| w.parent().ok().flatten()) {
            let _ = parent.post_message(&"{\"aylis_demo_size\":[0,0]}".into(), "*");
        }
        return;
    };
    // O tamanho natural para a página em volta (o quadro "Ao vivo" da documentação ajusta a
    // altura a ele, sem cortar nem sobrar).
    let (w, h) = natural_size(&ui, form);
    if let Some(parent) = web_sys::window().and_then(|w| w.parent().ok().flatten()) {
        let _ = parent.post_message(&format!("{{\"aylis_demo_size\":[{w},{h}]}}").into(), "*");
    }
    configure(&mut ui, form, &query);
    if let Err(e) = aylis::platform::run(ui, form) {
        web_sys::console::error_1(&format!("aylis-demos: {e}").into());
    }
}
