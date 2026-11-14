//! Recursos do aparelho: [`Permissions`] (consultar e pedir permissões) e [`Biometrics`]
//! (digital, rosto, Windows Hello). Não visuais; quem fala com o sistema é a plataforma
//! (`Services::permissions`, `Services::biometrics`).
//!
//! | Plataforma | Permissões | Biometria |
//! |---|---|---|
//! | Android | as do sistema (a pergunta na tela), declaradas no manifesto pela publicação | `BiometricPrompt` |
//! | iOS | as do sistema, com os textos de `reasons` no Info.plist | `LocalAuthentication` (Touch ID, Face ID) |
//! | Navegador | a Permissions API (`navigator.permissions`) e a pergunta do navegador | WebAuthn com o autenticador do aparelho |
//! | Windows | liberadas (o Windows controla nas configurações de privacidade) | Windows Hello |
//! | macOS, Linux | liberadas | ainda sem (responde `none`) |

use aylis_core::platform::{BiometricResult, Biometry, Permission, PermissionState};
use aylis_core::prop::PropKind;
use aylis_core::{Component, ComponentId, EventArgs, EventCtx, MethodInfo, PropValue, Ui, properties};

/// Consulta e pede as permissões do aparelho (câmera, microfone, localização, notificações,
/// fotos, contatos, agenda, bluetooth).
///
/// `permissions` lista as que o app usa (`camera;location`): a publicação as declara no
/// AndroidManifest e no Info.plist, e [`Permissions::request_all`] pede todas. No iOS cada
/// uma precisa de um texto que diz por que o app a usa: `reasons`
/// (`camera=Para a foto do perfil;location=Para achar a loja mais perto`).
///
/// ```no_run
/// # use aylis_core::Ui; use aylis_core::platform::Permission; use aylis_controls_device::Permissions;
/// # let mut ui = Ui::new(); let perm = ui.find("prmApp").unwrap();
/// if Permissions::state(&ui, Permission::Camera) != aylis_core::platform::PermissionState::Granted {
///     // a resposta chega no on_result: "camera=granted"
///     Permissions::request(&mut ui, perm, &[Permission::Camera]);
/// }
/// ```
#[derive(Default)]
pub struct Permissions {
    /// As permissões que o app usa, separadas por `;` (`camera;location`): a publicação as
    /// declara e o `request_all` pede todas.
    pub permissions: String,
    /// Por que o app usa cada uma (o iOS mostra ao pedir): `camera=Para a foto do perfil;...`.
    pub reasons: String,
}

impl Permissions {
    /// A situação de uma permissão agora. Sem o serviço da plataforma (desktop), liberada.
    pub fn state(ui: &Ui, permission: Permission) -> PermissionState {
        ui.services().permissions.as_ref().map_or(PermissionState::Granted, |s| s.state(permission))
    }

    /// Pede as permissões (o sistema pergunta só as que ainda não foram respondidas). A
    /// resposta chega no `on_result` do componente `id`, como `camera=granted;location=denied`.
    pub fn request(ui: &mut Ui, id: ComponentId, permissions: &[Permission]) {
        let poster = ui.poster();
        let done = Box::new(move |answers: Vec<(Permission, PermissionState)>| {
            poster.post(move |ui| {
                let text = answers.iter().map(|(p, s)| format!("{}={}", p.name(), s.name())).collect::<Vec<_>>().join(";");
                ui.fire(id, "on_result", &EventArgs::Text(text));
            });
        });
        match ui.services_mut().permissions.as_mut() {
            Some(service) => service.request(permissions.to_vec(), done),
            None => done(permissions.iter().map(|&p| (p, PermissionState::Granted)).collect()),
        }
    }

    /// Pede todas as de `permissions`.
    pub fn request_all(ui: &mut Ui, id: ComponentId) {
        let list = ui.get::<Permissions>(id).map(|p| Permission::parse_list(&p.permissions)).unwrap_or_default();
        Self::request(ui, id, &list);
    }

    /// O texto de `reasons` para uma permissão (vazio se não houver).
    pub fn reason(&self, permission: Permission) -> String {
        self.reasons
            .split(';')
            .filter_map(|pair| pair.split_once('='))
            .find(|(name, _)| Permission::parse(name) == Some(permission))
            .map(|(_, text)| text.trim().to_string())
            .unwrap_or_default()
    }
}

impl Component for Permissions {
    fn class_name(&self) -> &'static str {
        "Permissions"
    }

    properties! {
        permissions: String => "Behavior",
        reasons: String => "Behavior";
        events: on_result
    }

    fn methods(&self) -> &'static [MethodInfo] {
        const METHODS: &[MethodInfo] = &[
            MethodInfo::new("state", &[PropKind::String], Some(PropKind::String)).params(&["permission"]),
            MethodInfo::new("request", &[PropKind::String], Some(PropKind::Bool)).params(&["permission"]),
            MethodInfo::new("request_all", &[], Some(PropKind::Bool)),
        ];
        METHODS
    }

    fn invoke(&mut self, ctx: &mut EventCtx<'_>, method: &str, args: &[PropValue]) -> Result<PropValue, String> {
        let text = |i: usize| args.get(i).and_then(|v| v.to_str().ok()).unwrap_or_default().to_string();
        match method {
            "state" => {
                let p = Permission::parse(&text(0)).ok_or_else(|| format!("permissão desconhecida: {}", text(0)))?;
                Ok(PropValue::String(Self::state(ctx.ui, p).name().to_string()))
            }
            "request" | "request_all" => {
                // O pedido usa os serviços do Ui: sai depois deste evento, pela fila.
                let id = ctx.id;
                let list = if method == "request" { Permission::parse_list(&text(0)) } else { Permission::parse_list(&self.permissions) };
                ctx.ui.poster().post(move |ui| Permissions::request(ui, id, &list));
                Ok(PropValue::Bool(true))
            }
            _ => Err(format!("Permissions não tem o método '{method}'")),
        }
    }
}

/// A biometria do aparelho: a digital ou o rosto no celular, o Windows Hello, o
/// autenticador do aparelho no navegador. [`Biometrics::authenticate`] mostra a verificação
/// do sistema com `reason`; o resultado chega em `on_success`, `on_fail` (com o motivo) ou
/// `on_cancel`. Com `allow_device_credential`, o PIN ou o padrão do aparelho também valem.
#[derive(Default)]
pub struct Biometrics {
    /// O texto que a verificação do sistema mostra (por que o app pede a biometria).
    pub reason: String,
    /// O PIN, o padrão ou a senha do aparelho também valem, além da biometria.
    pub allow_device_credential: bool,
}

impl Biometrics {
    /// O tipo de biometria disponível (`Biometry::None` sem nenhuma cadastrada).
    pub fn availability(ui: &Ui) -> Biometry {
        ui.services().biometrics.as_ref().map_or(Biometry::None, |s| s.availability())
    }

    pub fn available(ui: &Ui) -> bool {
        Self::availability(ui) != Biometry::None
    }

    /// Pede a verificação (o texto é o `reason` do componente).
    pub fn authenticate(ui: &mut Ui, id: ComponentId) {
        let (reason, allow) = match ui.get::<Biometrics>(id) {
            Some(b) => (b.reason.clone(), b.allow_device_credential),
            None => return,
        };
        let reason = if reason.trim().is_empty() { ui.tr("Confirme que é você") } else { reason };
        let poster = ui.poster();
        let done = Box::new(move |result: BiometricResult| {
            poster.post(move |ui| match result {
                BiometricResult::Success => {
                    ui.fire(id, "on_success", &EventArgs::None);
                }
                BiometricResult::Cancelled => {
                    ui.fire(id, "on_cancel", &EventArgs::None);
                }
                BiometricResult::Failed(why) => {
                    ui.fire(id, "on_fail", &EventArgs::Text(why));
                }
            });
        });
        match ui.services_mut().biometrics.as_mut() {
            Some(service) => service.authenticate(&reason, allow, done),
            None => done(BiometricResult::Failed(aylis_core::messages::t("este aparelho não tem biometria"))),
        }
    }
}

impl Component for Biometrics {
    fn class_name(&self) -> &'static str {
        "Biometrics"
    }

    properties! {
        reason: String => "Behavior",
        allow_device_credential: bool => "Behavior";
        events: on_success, on_fail, on_cancel
    }

    fn methods(&self) -> &'static [MethodInfo] {
        const METHODS: &[MethodInfo] = &[
            MethodInfo::new("availability", &[], Some(PropKind::String)),
            MethodInfo::new("available", &[], Some(PropKind::Bool)),
            MethodInfo::new("authenticate", &[], Some(PropKind::Bool)),
        ];
        METHODS
    }

    fn invoke(&mut self, ctx: &mut EventCtx<'_>, method: &str, _args: &[PropValue]) -> Result<PropValue, String> {
        match method {
            "availability" => Ok(PropValue::String(Self::availability(ctx.ui).name().to_string())),
            "available" => Ok(PropValue::Bool(Self::available(ctx.ui))),
            "authenticate" => {
                let id = ctx.id;
                ctx.ui.poster().post(move |ui| Biometrics::authenticate(ui, id));
                Ok(PropValue::Bool(true))
            }
            _ => Err(format!("Biometrics não tem o método '{method}'")),
        }
    }
}

/// Fala com outros apps: abre um endereço (`https:`, `mailto:`, `tel:`, `geo:`) ou outro
/// app, compartilha texto e arquivos pela folha do sistema e pede algo a outro app
/// (`request`, com a resposta em `on_result`). Também recebe o que chega de fora: `on_link`
/// (o link que abriu o app, dos `deep_links` do projeto) e `on_shared` (o que outro app
/// compartilhou com este: o texto, ou os arquivos separados por `;`). Com o `on_shared` num
/// form, a publicação do Android põe o app na lista de compartilhar.
///
/// | Plataforma | Abrir endereço | Abrir app | Compartilhar | Pedir | Receber |
/// |---|---|---|---|---|---|
/// | Android | `ACTION_VIEW` | pelo pacote | `ACTION_SEND` | `Intent` com resposta | links e compartilhar |
/// | Windows, macOS, Linux | o programa padrão | executável ou nome registrado | não | não | o link na linha de comando |
/// | Navegador | na mesma aba (relativo) ou noutra | não | `navigator.share` | não | não |
#[derive(Default)]
pub struct AppLink;

impl AppLink {
    /// Abre `url` no app que cuida dele.
    pub fn open_url(ui: &mut Ui, url: &str) -> Result<(), String> {
        match ui.services_mut().links.as_mut() {
            Some(service) => service.open_url(url),
            None => Err(aylis_core::messages::t("abrir endereços não existe aqui")),
        }
    }

    /// Abre outro app (o pacote no Android; o executável ou o nome registrado no desktop).
    pub fn open_app(ui: &mut Ui, app: &str) -> Result<(), String> {
        match ui.services_mut().links.as_mut() {
            Some(service) => service.open_app(app),
            None => Err(aylis_core::messages::t("abrir outro app não existe aqui")),
        }
    }

    /// Compartilha `text` e os arquivos `files` pela folha do sistema.
    pub fn share(ui: &mut Ui, text: &str, files: &[String]) -> Result<(), String> {
        match ui.services_mut().links.as_mut() {
            Some(service) => service.share(text, files),
            None => Err(aylis_core::messages::t("compartilhar não existe aqui")),
        }
    }

    /// Pede `action` a outro app; a resposta chega no `on_result` do componente `id` (ou o
    /// motivo, no `on_error`).
    pub fn request(ui: &mut Ui, id: ComponentId, action: &str, data: &str) {
        let poster = ui.poster();
        let done = Box::new(move |result: Result<String, String>| {
            poster.post(move |ui| {
                let (event, text) = match result {
                    Ok(text) => ("on_result", text),
                    Err(e) => ("on_error", e),
                };
                ui.fire(id, event, &EventArgs::Text(text));
            });
        });
        match ui.services_mut().links.as_mut() {
            Some(service) => service.request(action, data, done),
            None => done(Err(aylis_core::messages::t("pedir a outro app não existe aqui"))),
        }
    }
}

// Sem propriedades, só eventos: o `get_prop` gerado não tem casos.
#[allow(unreachable_code)]
impl Component for AppLink {
    fn class_name(&self) -> &'static str {
        "AppLink"
    }

    properties! {
        ;
        events: on_link, on_shared, on_result, on_error
    }

    fn methods(&self) -> &'static [MethodInfo] {
        const METHODS: &[MethodInfo] = &[
            MethodInfo::new("open_url", &[PropKind::String], Some(PropKind::Bool)).params(&["url"]),
            MethodInfo::new("open_app", &[PropKind::String], Some(PropKind::Bool)).params(&["uri"]),
            MethodInfo::new("share", &[PropKind::String, PropKind::String], Some(PropKind::Bool)).params(&["text", "files"]),
            MethodInfo::new("request", &[PropKind::String, PropKind::String], Some(PropKind::Bool)).params(&["action", "data"]),
        ];
        METHODS
    }

    fn invoke(&mut self, ctx: &mut EventCtx<'_>, method: &str, args: &[PropValue]) -> Result<PropValue, String> {
        let text = |i: usize| args.get(i).and_then(|v| v.to_str().ok()).unwrap_or_default().to_string();
        // Os serviços são do Ui: o pedido sai logo depois deste evento, pela fila; um erro
        // chega no `on_error`.
        let (id, name, a, b) = (ctx.id, method.to_string(), text(0), text(1));
        let run = move |ui: &mut Ui| {
            let result = match name.as_str() {
                "open_url" => AppLink::open_url(ui, &a),
                "open_app" => AppLink::open_app(ui, &a),
                "share" => AppLink::share(ui, &a, &b.split(';').filter(|f| !f.is_empty()).map(str::to_string).collect::<Vec<_>>()),
                _ => {
                    AppLink::request(ui, id, &a, &b);
                    Ok(())
                }
            };
            if let Err(e) = result {
                ui.fire(id, "on_error", &EventArgs::Text(e));
            }
        };
        match method {
            "open_url" | "open_app" | "share" | "request" => {
                ctx.ui.poster().post(run);
                Ok(PropValue::Bool(true))
            }
            _ => Err(format!("AppLink não tem o método '{method}'")),
        }
    }

    // O que chega de fora fica numa fila até um `AppLink` pegar (o app aberto pelo link ainda
    // não tinha o form quando ele chegou).
    fn timer_interval(&self) -> Option<f64> {
        Some(0.25)
    }

    fn timer(&mut self, ctx: &mut EventCtx<'_>) {
        for item in aylis_core::platform::take_incoming() {
            match item {
                aylis_core::platform::Incoming::Link(url) => ctx.fire("on_link", EventArgs::Text(url)),
                aylis_core::platform::Incoming::Shared(text) => ctx.fire("on_shared", EventArgs::Text(text)),
            }
        }
    }
}
