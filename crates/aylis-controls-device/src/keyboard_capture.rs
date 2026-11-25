//! A captura das teclas do sistema com a janela em foco (decidido em 24/11): quem mostra outra
//! máquina precisa que o Windows/Super, o Alt+Tab e o Ctrl+Esc vão para ela, e não para o
//! sistema daqui.

use aylis_core::prop::PropKind;
use aylis_core::{Component, EventCtx, KeyboardCaptureRequest, MethodInfo, PropValue, properties};

/// Com `active` ligado, as teclas que o sistema guarda para si vão para a janela do form deste
/// componente enquanto ela está em primeiro plano: a tecla Windows/Super (sozinha e com outra,
/// como Win+R), o Alt+Tab, o Alt+Esc, o Ctrl+Esc, o Ctrl+Shift+Esc, o Alt+Espaço, o PrintScreen
/// e, com `alt_f4`, o Alt+F4 (sem ele, o Alt+F4 fecha a janela como sempre). Elas chegam pelos
/// eventos de tecla de sempre (`on_key_down`/`on_key_up` do form e do controle com foco). Ao
/// perder o foco, ao desligar `active` ou ao liberar o componente, solta tudo.
///
/// Nasce desligado: prender o teclado mexe em segurança. O `on_capture_change` avisa quando o
/// teclado foi preso (`true`, a janela ganhou o foco) e solto (`false`); `is_capturing()` diz o
/// estado agora.
///
/// | Plataforma | Como |
/// |---|---|
/// | Windows | o gancho `WH_KEYBOARD_LL`, só com a janela em primeiro plano; o Ctrl+Alt+Del não (é do sistema, sempre) |
/// | Linux X11 | o `XGrabKeyboard` da janela |
/// | Linux Wayland | o `zwp_keyboard_shortcuts_inhibit_manager_v1` (o GNOME pergunta ao usuário; o compositor pode recusar) |
/// | macOS, celular, navegador | nada (`available` falso) |
///
/// No GNOME, a resposta só fica lembrada com o app instalado: um `.desktop` com o nome do
/// `app_id` da janela (o nome do pacote do projeto, o mesmo que os pacotes da publicação para
/// Linux e o `menu_shortcut` gravam). Sem ele (o app compilado e rodado direto), o GNOME
/// pergunta a cada abertura, e o Aylis avisa uma vez no stderr.
#[derive(Default)]
pub struct KeyboardCapture {
    /// Prende as teclas do sistema enquanto a janela do form está em foco.
    pub active: bool,
    /// O Alt+F4 também vai para o app (sem isso, fecha a janela).
    pub alt_f4: bool,
}

impl KeyboardCapture {
    /// O pedido deste componente (`None`: desligado).
    pub fn request(&self) -> Option<KeyboardCaptureRequest> {
        self.active.then_some(KeyboardCaptureRequest { alt_f4: self.alt_f4 })
    }

    /// A plataforma prende o teclado aqui (Windows, Linux).
    fn available(&self) -> bool {
        aylis_core::system_keys::capture_available()
    }

    fn set_b(&mut self, _: bool) {}
}

impl Component for KeyboardCapture {
    fn class_name(&self) -> &'static str {
        "KeyboardCapture"
    }

    properties! {
        active: bool => "Behavior",
        alt_f4: bool => "Behavior";
        events: on_capture_change: bool;
        computed: available: bool => available / set_b
    }

    fn methods(&self) -> &'static [MethodInfo] {
        const METHODS: &[MethodInfo] = &[MethodInfo::new("is_capturing", &[], Some(PropKind::Bool))];
        METHODS
    }

    fn invoke(&mut self, ctx: &mut EventCtx<'_>, method: &str, _args: &[PropValue]) -> Result<PropValue, String> {
        match method {
            "is_capturing" => {
                let form = ctx.ui.root_of(ctx.id);
                Ok(PropValue::Bool(self.active && ctx.ui.keyboard_captured() == Some(form)))
            }
            _ => Err(format!("KeyboardCapture não tem o método '{method}'")),
        }
    }

    fn property_changed(&mut self, ctx: &mut EventCtx<'_>, name: &str) {
        // No designer, nunca: a IDE ficaria com o teclado preso.
        if name == "available" || ctx.ui.design_mode() {
            return;
        }
        ctx.ui.set_keyboard_capture(ctx.id, self.request());
    }
}
