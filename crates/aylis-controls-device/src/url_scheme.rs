//! O [`UrlScheme`]: um prefixo de URL próprio (`meuapp://...`) que abre este app.

use aylis_core::platform::{check_scheme, link_scheme, take_startup_links};
use aylis_core::prop::PropKind;
use aylis_core::{Component, ComponentId, EventArgs, EventCtx, MethodInfo, PropValue, Ui, WindowCommand, properties};

/// Um prefixo de URL próprio, não visual: com `active`, o app registra `scheme` no sistema, e
/// `meuapp://pedido/42` chamado por qualquer programa (o navegador, um e-mail, outro app)
/// abre este app e chega no `on_link` com o endereço inteiro. Com o app já aberto, a cópia
/// nova aberta pelo link repassa o endereço para a aberta e fecha; a aberta vem para a
/// frente (`bring_to_front`). Cada `UrlScheme` recebe só os links do seu prefixo.
///
/// O endereço vem de fora (qualquer programa pode chamar o link): o app confere o que
/// recebe antes de agir. `register()`, `unregister()` e `is_registered()` fazem o mesmo
/// por código; um erro vai para o `on_error`.
///
/// | Plataforma | Registro | Com o app aberto |
/// |---|---|---|
/// | Windows | em execução, no registro do usuário (`HKCU\Software\Classes`) | chega no app aberto |
/// | Android | na publicação (os `deep_links` do projeto) | chega no app aberto |
/// | macOS, iOS, Linux, navegador | na publicação (ainda não) | — |
pub struct UrlScheme {
    /// O prefixo, sem `://` (`meuapp`): letras, números, `+`, `-` e `.`.
    pub scheme: String,
    /// O nome que o sistema mostra ao perguntar com que app abrir.
    pub description: String,
    pub active: bool,
    /// Trazer a janela do form para a frente quando um link chega com o app aberto.
    pub bring_to_front: bool,
    /// Já pediu à plataforma os links das outras cópias.
    listening: bool,
}

impl Default for UrlScheme {
    fn default() -> Self {
        Self { scheme: String::new(), description: String::new(), active: false, bring_to_front: true, listening: false }
    }
}

impl UrlScheme {
    /// Registra o prefixo deste app no sistema.
    pub fn register(ui: &mut Ui, scheme: &str, description: &str) -> Result<(), String> {
        let scheme = check_scheme(scheme)?;
        match ui.services_mut().links.as_mut() {
            Some(links) => links.register_scheme(&scheme, description),
            None => Err(aylis_core::messages::t("registrar um prefixo não existe aqui")),
        }
    }

    /// Tira o registro do prefixo (se for deste app).
    pub fn unregister(ui: &mut Ui, scheme: &str) -> Result<(), String> {
        let scheme = check_scheme(scheme)?;
        match ui.services_mut().links.as_mut() {
            Some(links) => links.unregister_scheme(&scheme),
            None => Err(aylis_core::messages::t("registrar um prefixo não existe aqui")),
        }
    }

    /// Se o prefixo abre este app.
    pub fn is_registered(ui: &mut Ui, scheme: &str) -> bool {
        let Ok(scheme) = check_scheme(scheme) else { return false };
        ui.services_mut().links.as_mut().is_some_and(|links| links.scheme_registered(&scheme))
    }

    /// Ligado: registra (onde o sistema deixa em execução), passa a receber os links das
    /// outras cópias e entrega o link que abriu o app. Roda depois de o form carregar, com os
    /// serviços da plataforma já postos.
    fn activate(ui: &mut Ui, id: ComponentId) {
        let Some(this) = ui.get::<UrlScheme>(id) else { return };
        if !this.active {
            return;
        }
        let (scheme, description, listening) = (this.scheme.clone(), this.description.clone(), this.listening);
        let scheme = match check_scheme(&scheme) {
            Ok(s) => s,
            Err(e) => {
                ui.fire(id, "on_error", &EventArgs::Text(e));
                return;
            }
        };
        let poster = ui.poster();
        let mut error = None;
        if let Some(links) = ui.services_mut().links.as_mut() {
            if links.can_register_schemes() && !links.scheme_registered(&scheme) {
                error = links.register_scheme(&scheme, &description).err();
            }
            if !listening {
                links.receive_links(Box::new(move |url| poster.post(move |ui| Self::deliver(ui, id, url))));
            }
        }
        if let Some(this) = ui.get_mut::<UrlScheme>(id) {
            this.listening = true;
        }
        if let Some(e) = error {
            ui.fire(id, "on_error", &EventArgs::Text(e));
        }
        for url in take_startup_links(&scheme) {
            ui.fire(id, "on_link", &EventArgs::Text(url));
        }
    }

    /// Um link que outra cópia repassou: só se for do prefixo e o componente estiver ligado.
    fn deliver(ui: &mut Ui, id: ComponentId, url: String) {
        let Some(this) = ui.get::<UrlScheme>(id) else { return };
        let mine = this.active && check_scheme(&this.scheme).ok() == link_scheme(&url);
        if !mine {
            return;
        }
        let bring = this.bring_to_front;
        ui.fire(id, "on_link", &EventArgs::Text(url));
        if bring {
            let form = ui.root_of(id);
            ui.window_command(form, WindowCommand::Show);
        }
    }
}

impl Component for UrlScheme {
    fn class_name(&self) -> &'static str {
        "UrlScheme"
    }

    properties! {
        scheme: String => "Behavior",
        description: String => "Behavior",
        active: bool => "Behavior",
        bring_to_front: bool => "Behavior";
        events: on_link, on_error
    }

    fn property_changed(&mut self, ctx: &mut EventCtx<'_>, name: &str) {
        if matches!(name, "active" | "scheme") && self.active && !ctx.ui.design_mode() {
            let id = ctx.id;
            ctx.ui.set_timeout(0, move |ui| Self::activate(ui, id));
        }
    }

    fn methods(&self) -> &'static [MethodInfo] {
        const METHODS: &[MethodInfo] = &[
            MethodInfo::new("register", &[], Some(PropKind::Bool)),
            MethodInfo::new("unregister", &[], Some(PropKind::Bool)),
            MethodInfo::new("is_registered", &[], Some(PropKind::Bool)),
        ];
        METHODS
    }

    fn invoke(&mut self, ctx: &mut EventCtx<'_>, method: &str, _args: &[PropValue]) -> Result<PropValue, String> {
        match method {
            "register" => Self::register(ctx.ui, &self.scheme, &self.description).map(|_| PropValue::Bool(true)),
            "unregister" => Self::unregister(ctx.ui, &self.scheme).map(|_| PropValue::Bool(true)),
            "is_registered" => Ok(PropValue::Bool(Self::is_registered(ctx.ui, &self.scheme))),
            _ => Err(format!("UrlScheme não tem o método '{method}'")),
        }
    }
}
