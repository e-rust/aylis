//! O componente `Nfc`: ler e gravar tags NDEF e dar o id da tag. A plataforma faz o
//! trabalho ([`aylis_core::platform::NfcService`]); o componente consulta num timer.

use aylis_core::ndef::{self, NdefRecord};
use aylis_core::platform::{NfcEvent, NfcService};
use aylis_core::prop::PropKind;
use aylis_core::{Component, EventArgs, EventCtx, MethodInfo, PropValue, Ui, properties};

use crate::records::{parse_list, records_text};

/// Os tipos de registro NDEF (o `kind` do [`NfcRecord`]).
const NFC_KINDS: &[&str] = &["text", "uri", "mime", "other"];

/// O registro de um registro NDEF (o `records` do `Nfc` e o argumento do `write`).
pub const NFC_RECORD: aylis_core::RecordInfo = aylis_core::RecordInfo {
    name: "NfcRecord",
    fields: &[("kind", PropKind::Enum(NFC_KINDS)), ("value", PropKind::String), ("mime", PropKind::String), ("lang", PropKind::String)],
};

/// Um registro de uma tag: `text` (o texto, com o idioma em `lang`), `uri` (o endereço),
/// `mime` (os dados como texto, com o tipo em `mime`) ou `other` (o que não é dos outros: o
/// tipo em `mime` como `tnf/tipo` e os bytes em hexadecimal; só na leitura).
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct NfcRecord {
    pub kind: String,
    pub value: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub mime: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub lang: String,
}

impl NfcRecord {
    pub fn from_ndef(record: &NdefRecord) -> NfcRecord {
        let r = |kind: &str, value: String, mime: String, lang: String| NfcRecord { kind: kind.into(), value, mime, lang };
        match record {
            NdefRecord::Text { text, lang } => r("text", text.clone(), String::new(), lang.clone()),
            NdefRecord::Uri(uri) => r("uri", uri.clone(), String::new(), String::new()),
            NdefRecord::Mime { mime, data } => r("mime", String::from_utf8_lossy(data).into_owned(), mime.clone(), String::new()),
            NdefRecord::Other { tnf, kind, data } => {
                r("other", data.iter().map(|b| format!("{b:02X}")).collect(), format!("{tnf}/{}", String::from_utf8_lossy(kind)), String::new())
            }
        }
    }

    /// O registro NDEF para gravar (`other` não grava: `None`).
    pub fn to_ndef(&self) -> Option<NdefRecord> {
        Some(match self.kind.trim() {
            "" | "text" => NdefRecord::Text { text: self.value.clone(), lang: if self.lang.trim().is_empty() { "pt".into() } else { self.lang.trim().to_string() } },
            "uri" => NdefRecord::Uri(self.value.clone()),
            "mime" if !self.mime.trim().is_empty() => NdefRecord::Mime { mime: self.mime.trim().to_string(), data: self.value.clone().into_bytes() },
            _ => return None,
        })
    }
}

records_text!(NfcRecordList, NFC_RECORD);

/// Os registros em JSON ([`NFC_RECORD`]).
fn records_json(records: &[NdefRecord]) -> String {
    serde_json::to_string(&records.iter().map(NfcRecord::from_ndef).collect::<Vec<_>>()).unwrap_or_default()
}

/// Os registros NDEF de um JSON ([`NFC_RECORD`]); os que não gravam ficam de fora.
fn records_from_json(text: &str) -> Vec<NdefRecord> {
    parse_list::<NfcRecord>(text).iter().filter_map(NfcRecord::to_ndef).collect()
}

/// Lê e grava tags NFC (NDEF): `active` lê enquanto o app está aberto; cada tag encostada
/// chega no `on_tag` (o texto é o id) com `tag_id`, `tag_type` e `records` (os registros em
/// JSON, [`NfcRecord`]: `[{"kind": "text", "value": "Olá", "lang": "pt"}, {"kind": "uri",
/// "value": "https://aylis.dev"}]`). `write(registros)` grava na próxima tag que encostar (o
/// resultado no `on_written` ou no `on_error`).
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
    records: Vec<NdefRecord>,
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
    /// Os registros da última tag, em JSON ([`NFC_RECORD`]).
    pub fn records(&self) -> NfcRecordList {
        NfcRecordList(records_json(&self.records))
    }
    pub fn writable(&self) -> bool {
        self.writable
    }
    pub fn writing(&self) -> bool {
        self.writing
    }
    fn set_s(&mut self, _: String) {}
    fn set_b(&mut self, _: bool) {}
    fn set_records<T>(&mut self, _: T) {}

    /// Os registros da última tag.
    pub fn record_list(&self) -> Vec<NdefRecord> {
        self.records.clone()
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
    fn timed(&self) -> Option<&dyn aylis_core::Timed> {
        Some(self)
    }

    fn timed_mut(&mut self) -> Option<&mut dyn aylis_core::Timed> {
        Some(self)
    }

    fn class_name(&self) -> &'static str {
        "Nfc"
    }

    properties! {
        active: bool => "Behavior";
        events: on_tag: String, on_written: (), on_error: String;
        computed: tag_id: String => tag_id / set_s, tag_type: String => tag_type / set_s, records: NfcRecordList => records / set_records,
            writable: bool => writable / set_b, writing: bool => writing / set_b, available: bool => available / set_b
    }

    fn methods(&self) -> &'static [MethodInfo] {
        use PropKind::Bool;
        const METHODS: &[MethodInfo] = &[
            MethodInfo::new("enabled", &[], Some(Bool)),
            MethodInfo::new("write", &[PropKind::RecordList(&NFC_RECORD)], Some(Bool)).params(&["records"]),
            MethodInfo::new("cancel_write", &[], None),
        ];
        METHODS
    }

    fn invoke(&mut self, ctx: &mut EventCtx<'_>, method: &str, args: &[PropValue]) -> Result<PropValue, String> {
        match method {
            "enabled" => Ok(PropValue::Bool(ctx.ui.services().nfc.as_ref().is_some_and(|s| s.enabled()))),
            "write" => {
                let text = args.first().and_then(|v| v.to_str().ok()).unwrap_or_default();
                let records = records_from_json(text);
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

}

impl aylis_core::Timed for Nfc {
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
                    self.records = tag.records;
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
