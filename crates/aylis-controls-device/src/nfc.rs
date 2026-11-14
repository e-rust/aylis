//! O componente `Nfc`: ler e gravar tags NDEF e dar o id da tag. A plataforma faz o
//! trabalho ([`aylis_core::platform::NfcService`]); o componente consulta num timer.

use aylis_core::ndef::{self, NdefRecord};
use aylis_core::platform::{NfcEvent, NfcService};
use aylis_core::prop::PropKind;
use aylis_core::{Component, EventArgs, EventCtx, MethodInfo, PropValue, Ui, properties};

/// Lê e grava tags NFC (NDEF): `active` lê enquanto o app está aberto; cada tag encostada
/// chega no `on_tag` (o texto é o id) com `tag_id`, `tag_type` e `records` (um registro por
/// linha: `text:Olá`, `uri:https://...`, `application/json:{...}`). `write(registros)` grava
/// na próxima tag que encostar (o resultado no `on_written` ou no `on_error`).
///
/// Android: o `NfcAdapter` em modo leitor (a publicação põe a permissão `NFC`). Windows,
/// Linux e macOS: um leitor USB pelo PC/SC (ACR122U e parecidos), com tags do tipo 2 (NTAG,
/// Ultralight). Navegador: Web NFC (só o Chrome do Android). Onde não houver, `available` é
/// falso.
#[derive(Default)]
pub struct Nfc {
    /// Lê as tags encostadas enquanto o app está aberto (cada uma chega no `on_tag`).
    pub active: bool,
    tag_id: String,
    tag_type: String,
    records: String,
    writable: bool,
    writing: bool,
    reading: bool,
    available: bool,
}

impl Nfc {
    fn service(ui: &mut Ui) -> Result<&mut Box<dyn NfcService>, String> {
        ui.services_mut().nfc.as_mut().ok_or_else(|| aylis_core::messages::t("este aparelho não tem NFC"))
    }

    pub fn available(&self) -> bool {
        self.available
    }
    pub fn tag_id(&self) -> String {
        self.tag_id.clone()
    }
    pub fn tag_type(&self) -> String {
        self.tag_type.clone()
    }
    pub fn records(&self) -> String {
        self.records.clone()
    }
    pub fn writable(&self) -> bool {
        self.writable
    }
    pub fn writing(&self) -> bool {
        self.writing
    }
    fn set_s(&mut self, _: String) {}
    fn set_b(&mut self, _: bool) {}

    /// Os registros da última tag.
    pub fn record_list(&self) -> Vec<NdefRecord> {
        ndef::records_from_text(&self.records)
    }

    /// Grava os registros na próxima tag que encostar.
    pub fn write(ui: &mut Ui, id: aylis_core::ComponentId, records: &[NdefRecord]) -> Result<(), String> {
        let message = ndef::encode(records);
        Self::service(ui)?.write(message);
        if let Some(n) = ui.get_mut::<Nfc>(id) {
            n.writing = true;
        }
        Ok(())
    }
}

impl Component for Nfc {
    fn class_name(&self) -> &'static str {
        "Nfc"
    }

    properties! {
        active: bool => "Behavior";
        events: on_tag, on_written, on_error;
        computed: tag_id: String => tag_id / set_s, tag_type: String => tag_type / set_s, records: String => records / set_s,
            writable: bool => writable / set_b, writing: bool => writing / set_b, available: bool => available / set_b
    }

    fn methods(&self) -> &'static [MethodInfo] {
        use PropKind::{Bool, String};
        const METHODS: &[MethodInfo] = &[
            MethodInfo::new("enabled", &[], Some(Bool)),
            MethodInfo::new("write", &[String], Some(Bool)).params(&["records"]),
            MethodInfo::new("cancel_write", &[], None),
        ];
        METHODS
    }

    fn invoke(&mut self, ctx: &mut EventCtx<'_>, method: &str, args: &[PropValue]) -> Result<PropValue, String> {
        match method {
            "enabled" => Ok(PropValue::Bool(ctx.ui.services().nfc.as_ref().is_some_and(|s| s.enabled()))),
            "write" => {
                let text = args.first().and_then(|v| v.to_str().ok()).unwrap_or_default();
                let records = ndef::records_from_text(text);
                if records.is_empty() {
                    return Err(aylis_core::messages::t("nada para gravar"));
                }
                Self::service(ctx.ui)?.write(ndef::encode(&records));
                self.writing = true;
                Ok(PropValue::Bool(true))
            }
            "cancel_write" => {
                if let Some(s) = ctx.ui.services_mut().nfc.as_mut() {
                    s.cancel_write();
                }
                self.writing = false;
                Ok(PropValue::Null)
            }
            _ => Err(format!("Nfc não tem o método '{method}'")),
        }
    }

    fn timer_interval(&self) -> Option<f64> {
        // Parado, só confere de vez em quando se há NFC (o `available`).
        Some(if self.active || self.writing || self.reading { 0.1 } else { 1.0 })
    }

    fn timer(&mut self, ctx: &mut EventCtx<'_>) {
        if ctx.ui.design_mode() {
            return;
        }
        let want = self.active || self.writing;
        let Some(s) = ctx.ui.services_mut().nfc.as_mut() else { return };
        self.available = s.available();
        if want != self.reading {
            self.reading = want;
            s.set_reading(want);
        }
        for event in s.poll() {
            match event {
                NfcEvent::Tag(tag) => {
                    self.tag_id = tag.id.clone();
                    self.tag_type = tag.tech;
                    self.records = ndef::records_to_text(&tag.records);
                    self.writable = tag.writable;
                    ctx.fire("on_tag", EventArgs::Text(tag.id));
                }
                NfcEvent::Written => {
                    self.writing = false;
                    ctx.fire("on_written", EventArgs::None);
                }
                NfcEvent::Error(e) => {
                    self.writing = false;
                    ctx.fire("on_error", EventArgs::Text(e));
                }
            }
        }
    }
}
