//! [`ChartSync`]: liga vários [`Chart`](crate::Chart) pelo eixo X. Componente não visual, como o
//! `DataSource`: cada gráfico aponta para ele pela propriedade `sync`.
//!
//! Ele guarda dois estados, pelo **valor do X** (não pelo índice: gráficos com pontos
//! diferentes se acham pelo mesmo horário):
//!
//! - o `cursor`: um ponto do eixo X. O mouse num gráfico muda; todos mostram sozinhos a linha
//!   e a dica nele;
//! - o `range`: uma janela `from`/`to` do eixo X. A seleção por faixa e o zoom de um gráfico
//!   mudam; todos mostram só essa janela.
//!
//! O valor do X é o que o app deu ao gráfico: o rótulo (eixo de categorias, `Jan`), a data do
//! rótulo (eixo de tempo, `2026-11-23 14:30`) ou o número (dispersão, `12.5`). Um gráfico com
//! outro tipo de eixo X do que o do gráfico que mudou o estado o ignora (com um aviso em
//! debug). O app muda pelo código com [`ChartSync::set_cursor`] e [`ChartSync::set_range`]
//! (nas outras linguagens, as propriedades `cursor` e `range`; sem métodos com o mesmo nome,
//! que no acesso tipado bateriam com os setters delas)
//! (sem `on_sync`: o evento é do que o usuário faz) e fica sabendo pelo `on_sync`
//! ([`SyncEvent`]).

use aylis_core::{Component, ComponentId, EventArgs, EventCtx, PropEnum, Ui, properties};

use crate::records::record_event;

/// A janela do `range` (os valores do X, como texto; veja o módulo).
pub const SYNC_RANGE: aylis_core::RecordInfo = aylis_core::RecordInfo {
    name: "SyncRange",
    fields: &[("from", aylis_core::PropKind::String), ("to", aylis_core::PropKind::String)],
};

/// O que o `on_sync` leva.
pub const SYNC_EVENT: aylis_core::RecordInfo = aylis_core::RecordInfo {
    name: "SyncEvent",
    fields: &[
        ("kind", aylis_core::PropKind::Enum(SyncKind::NAMES)),
        ("cursor", aylis_core::PropKind::String),
        ("from", aylis_core::PropKind::String),
        ("to", aylis_core::PropKind::String),
        ("source", aylis_core::PropKind::String),
    ],
};

/// O que mudou no [`ChartSync`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SyncKind {
    /// O ponto do eixo X (o mouse).
    #[default]
    Cursor,
    /// A janela do eixo X (a seleção por faixa, o zoom).
    Range,
}

impl PropEnum for SyncKind {
    const NAMES: &'static [&'static str] = &["cursor", "range"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Cursor, Self::Range].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// O argumento do `on_sync`: o que mudou, os valores (vazios: sem cursor, ou a janela toda) e
/// o nome do gráfico de onde veio.
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct SyncEvent {
    pub kind: SyncKind,
    pub cursor: String,
    pub from: String,
    pub to: String,
    pub source: String,
}

record_event!(SyncEvent, SYNC_EVENT);

/// O tipo do eixo X de quem mudou o estado: um gráfico de outro tipo não o entende.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum XKind {
    Category,
    Time,
    Value,
}

/// Mensagem aos gráficos ligados: o estado mudou (repintar).
pub(crate) struct SyncChanged;

/// Liga gráficos pelo eixo X: o cursor e a janela (veja o módulo).
#[derive(Default)]
pub struct ChartSync {
    cursor: Option<String>,
    range: Option<(String, String)>,
    /// O tipo de eixo de quem mudou (`None`: o app, pelo código; vale para todos).
    pub(crate) cursor_axis: Option<XKind>,
    pub(crate) range_axis: Option<XKind>,
}

impl ChartSync {
    pub fn new() -> Self {
        Self::default()
    }

    /// O ponto do eixo X (`None`: nenhum).
    pub fn cursor(&self) -> Option<&str> {
        self.cursor.as_deref()
    }

    /// A janela do eixo X (`None`: o eixo todo).
    pub fn range(&self) -> Option<(&str, &str)> {
        self.range.as_ref().map(|(a, b)| (a.as_str(), b.as_str()))
    }

    /// Muda o cursor pelo código (`None` tira) e os gráficos ligados repintam.
    pub fn set_cursor(ui: &mut Ui, id: ComponentId, x: Option<&str>) {
        if let Some(s) = ui.get_mut::<ChartSync>(id) {
            s.cursor = x.filter(|x| !x.is_empty()).map(str::to_string);
            s.cursor_axis = None;
        }
        Self::refresh_charts(ui, id);
    }

    /// Muda a janela pelo código (`None`: o eixo todo) e os gráficos ligados mostram só ela.
    pub fn set_range(ui: &mut Ui, id: ComponentId, range: Option<(&str, &str)>) {
        if let Some(s) = ui.get_mut::<ChartSync>(id) {
            s.range = range.filter(|(a, b)| !a.is_empty() && !b.is_empty()).map(|(a, b)| (a.to_string(), b.to_string()));
            s.range_axis = None;
        }
        Self::refresh_charts(ui, id);
    }

    /// Os gráficos que apontam para este (pelo índice de referências, sem percorrer o form).
    pub fn charts(ui: &Ui, id: ComponentId) -> Vec<ComponentId> {
        ui.referrers(id).into_iter().filter(|&(_, prop)| prop == "sync").map(|(c, _)| c).collect()
    }

    pub(crate) fn refresh_charts(ui: &mut Ui, id: ComponentId) {
        for chart in Self::charts(ui, id) {
            ui.send_message(chart, &SyncChanged);
        }
    }

    /// Um gráfico mudou o estado (o usuário): grava, avisa os outros e dispara o `on_sync`
    /// depois do evento atual (o gráfico de origem está emprestado agora).
    pub(crate) fn publish(ui: &mut Ui, id: ComponentId, source: ComponentId, kind: SyncKind, axis: XKind, value: Option<(String, Option<String>)>) {
        let Some(s) = ui.get_mut::<ChartSync>(id) else { return };
        match kind {
            SyncKind::Cursor => {
                let cursor = value.map(|(x, _)| x);
                if s.cursor == cursor {
                    return;
                }
                s.cursor = cursor;
                s.cursor_axis = Some(axis);
            }
            SyncKind::Range => {
                let range = value.and_then(|(a, b)| Some((a, b?)));
                if s.range == range {
                    return;
                }
                s.range = range;
                s.range_axis = Some(axis);
            }
        }
        let event = SyncEvent {
            kind,
            cursor: s.cursor.clone().unwrap_or_default(),
            from: s.range.as_ref().map(|r| r.0.clone()).unwrap_or_default(),
            to: s.range.as_ref().map(|r| r.1.clone()).unwrap_or_default(),
            source: ui.name(source).unwrap_or_default().to_string(),
        };
        for chart in Self::charts(ui, id) {
            if chart != source {
                ui.send_message(chart, &SyncChanged);
            }
        }
        let json = serde_json::to_string(&event).unwrap_or_default();
        ui.poster().post(move |ui| {
            ui.fire(id, "on_sync", &EventArgs::Text(json));
        });
    }

    fn range_text(&self) -> String {
        match &self.range {
            Some((a, b)) => serde_json::json!({ "from": a, "to": b }).to_string(),
            None => String::new(),
        }
    }

    fn set_range_text(&mut self, text: String) {
        let json: serde_json::Value = serde_json::from_str(&text).unwrap_or_default();
        let part = |k: &str| json.get(k).and_then(|v| v.as_str()).map(str::to_string).filter(|s| !s.is_empty());
        self.range = part("from").zip(part("to"));
        self.range_axis = None;
    }

    fn cursor_text(&self) -> String {
        self.cursor.clone().unwrap_or_default()
    }

    fn set_cursor_text(&mut self, text: String) {
        self.cursor = Some(text).filter(|t| !t.is_empty());
        self.cursor_axis = None;
    }
}

impl Component for ChartSync {
    fn class_name(&self) -> &'static str {
        "ChartSync"
    }

    properties! {
        ; events: on_sync: SyncEvent
        ; computed: cursor: String => cursor_text / set_cursor_text,
            range: String as Record(&SYNC_RANGE) => range_text / set_range_text
    }

    /// O `cursor` e o `range` mudados por nome (o protocolo, as outras linguagens): os
    /// gráficos repintam.
    fn property_changed(&mut self, ctx: &mut EventCtx<'_>, name: &str) {
        if matches!(name, "cursor" | "range") {
            let id = ctx.id;
            ctx.ui.poster().post(move |ui| ChartSync::refresh_charts(ui, id));
        }
    }
}
