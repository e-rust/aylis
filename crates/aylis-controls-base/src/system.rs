//! Componentes de sistema: Timer, OpenDialog/SaveDialog, MessageDialog/MessageDlg e Toast.

use aylis_core::platform::{FileDialogOptions, NotificationPriority};
use aylis_core::{PropEnum, TimerPrecision};
use aylis_core::{
    Align, Brush, Canvas, Color, Component, ComponentId, ControlBase, EventArgs, EventCtx, Key, KeyEvent, LineCap, ModalResult,
    PaintCtx, PointF, RectF, Stroke, TextAlign, Ui, properties,
};

use crate::{Button, Form, Label, Layout, ScrollBox, leaf_control};

// ---------------------------------------------------------------------------
// Timer

/// Dispara `on_timer` a cada `interval` milissegundos enquanto `enabled` (como o
/// `TTimer`). Não visual. Parado, não gasta nada: a plataforma dorme até o próximo tique.
///
/// `precision` diz quanto pode atrasar: `normal` (o relógio do sistema; no Windows, degraus
/// de ~15,6 ms), `high` (~1 ms) ou `exact` (abaixo de 0,1 ms, esperando ativo no fim).
/// `background` conta numa thread própria, sem esperar o laço da tela (que pode estar
/// ocupado); o `on_timer` continua chegando na tela. `drift` é quanto o último disparo
/// atrasou, em milissegundos.
pub struct Timer {
    pub interval: u32,
    pub enabled: bool,
    pub precision: TimerPrecision,
    pub background: bool,
    /// Dispara uma vez e se desliga (`enabled` volta a `false`); ligar de novo conta outra vez.
    pub one_shot: bool,
    drift: f64,
}

impl Default for Timer {
    fn default() -> Self {
        Self::new(1000)
    }
}

impl Timer {
    pub fn new(interval_ms: u32) -> Self {
        Self { interval: interval_ms, enabled: true, precision: TimerPrecision::Normal, background: false, one_shot: false, drift: 0.0 }
    }

    /// Recomeça a contar o intervalo a partir de agora (o próximo disparo fica a um intervalo
    /// inteiro), ligando o timer se estava desligado.
    pub fn restart(ui: &mut Ui, id: ComponentId) {
        let _ = ui.set_prop(id, "enabled", true);
        ui.restart_timer(id);
    }

    /// Liga ou desliga pelo código.
    pub fn set_enabled(ui: &mut Ui, id: ComponentId, enabled: bool) {
        let _ = ui.set_prop(id, "enabled", enabled);
    }

    /// Quanto o último disparo atrasou, em milissegundos.
    pub fn drift(&self) -> f64 {
        self.drift
    }

    // Só leitura: o Object Inspector mostra, gravar não muda nada.
    fn set_drift(&mut self, _: f64) {}
}

impl Component for Timer {
    fn class_name(&self) -> &'static str {
        "Timer"
    }

    properties! {
        interval: u32 as Duration => "Behavior",
        enabled: bool => "Behavior",
        precision: TimerPrecision => "Behavior",
        background: bool => "Behavior",
        one_shot: bool => "Behavior";
        events: on_timer;
        computed: drift: f64 => drift / set_drift
    }

    fn methods(&self) -> &'static [aylis_core::MethodInfo] {
        const METHODS: &[aylis_core::MethodInfo] = &[aylis_core::MethodInfo::new("restart", &[], None)];
        METHODS
    }

    /// `restart()`: recomeça a contar o intervalo agora (o relatório do Tabernáculo, P10).
    fn invoke(&mut self, ctx: &mut EventCtx<'_>, method: &str, _args: &[aylis_core::PropValue]) -> Result<aylis_core::PropValue, String> {
        match method {
            "restart" => {
                self.enabled = true;
                // O componente está fora do `Ui` durante o método: o agendamento vem depois.
                let id = ctx.id;
                ctx.ui.poster().post(move |ui| ui.restart_timer(id));
                Ok(aylis_core::PropValue::Null)
            }
            _ => Err(format!("Timer não tem o método '{method}'")),
        }
    }

    fn timer(&mut self, ctx: &mut EventCtx<'_>) {
        if self.one_shot {
            self.enabled = false;
        }
        ctx.fire("on_timer", EventArgs::None);
    }

    fn timer_interval(&self) -> Option<f64> {
        (self.enabled && self.interval > 0).then(|| f64::from(self.interval) / 1000.0)
    }

    fn timer_precision(&self) -> TimerPrecision {
        self.precision
    }

    fn timer_background(&self) -> bool {
        self.background
    }

    fn timer_late(&mut self, late: f64) {
        self.drift = late * 1000.0;
    }
}

// ---------------------------------------------------------------------------
// FrameTimer

/// Um evento a cada quadro do monitor (`on_frame`, com os segundos desde o anterior), para a
/// animação que o app desenha: como o `Timer`, mas no ritmo da tela, e só enquanto ligado
/// (desligado, o laço dorme; começa desligado). O relatório do Tabernáculo, P05.
#[derive(Default)]
pub struct FrameTimer {
    pub enabled: bool,
    last: Option<f64>,
}

impl FrameTimer {
    /// Liga pelo código (o primeiro quadro vem a seguir).
    pub fn set_enabled(ui: &mut Ui, id: ComponentId, enabled: bool) {
        let _ = ui.set_prop(id, "enabled", enabled);
    }
}

impl Component for FrameTimer {
    fn class_name(&self) -> &'static str {
        "FrameTimer"
    }

    properties! {
        enabled: bool => "Behavior";
        events: on_frame
    }

    fn property_changed(&mut self, ctx: &mut EventCtx<'_>, name: &str) {
        if name == "enabled" && self.enabled {
            self.last = None;
            ctx.ui.request_frames(ctx.id);
        }
    }

    fn frame(&mut self, ctx: &mut EventCtx<'_>) -> bool {
        if !self.enabled {
            self.last = None;
            return false;
        }
        let now = ctx.ui.time();
        let dt = self.last.map_or(0.0, |last| (now - last).max(0.0));
        self.last = Some(now);
        ctx.fire("on_frame", EventArgs::Value(aylis_core::PropValue::Float(dt)));
        true
    }
}

// ---------------------------------------------------------------------------
// DataModule

/// Um contêiner só de componentes não visuais e da regra de negócio, fora da tela (como o
/// `TDataModule` do Delphi): conexões, ClientDataSets, Timers, diálogos... É uma raiz
/// como um form, mas nunca abre janela. Os forms que usam um DataModule o declaram em
/// `"uses"` no layout e falam com os componentes dele pelo caminho (`dmDados.cdsClientes`);
/// o aplicativo cria os DataModules antes dos forms. `on_create` dispara depois de carregado.
#[derive(Default)]
pub struct DataModule {
    /// Um número livre para o código (como o `Tag` do Delphi).
    pub tag: i64,
}

impl Component for DataModule {
    fn class_name(&self) -> &'static str {
        "DataModule"
    }

    properties! {
        tag: i64 => "Misc";
        events: on_create
    }
}

// ---------------------------------------------------------------------------
// ApplicationEvents

/// Os eventos da aplicação inteira (como o `TApplicationEvents` do Delphi), num componente
/// não visual de qualquer form: `on_key_down` e `on_key_press` recebem as teclas antes de todos
/// (antes do form com `key_preview` e do controle com foco; engolir ou trocar a tecla vale
/// como lá), e `on_exception` recebe todo erro de handler e de componente: com ele ligado, a
/// caixa de erro padrão não aparece.
#[derive(Default)]
pub struct ApplicationEvents {
    pub tag: i64,
}

impl Component for ApplicationEvents {
    fn class_name(&self) -> &'static str {
        "ApplicationEvents"
    }

    properties! {
        tag: i64 => "Misc";
        events: on_key_down, on_key_press, on_exception, on_terminate
    }
}

// ---------------------------------------------------------------------------
// EventInterceptor

/// Onde um `EventInterceptor` vale.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum InterceptScope {
    /// Só os componentes do form onde ele está.
    #[default]
    Form,
    /// Os componentes de todos os forms.
    Application,
}

impl PropEnum for InterceptScope {
    const NAMES: &'static [&'static str] = &["form", "application"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Form, Self::Application].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Recebe num lugar só os eventos de qualquer componente do form (ou da aplicação, com
/// `scope = application`), antes do handler: `on_intercept` traz o form, o componente, o
/// evento, os argumentos e o handler ligado, e pode cancelar (`ui.cancel_event()`, o
/// `Cancel := True`, ou a exceção com mensagem); `after_intercept` roda depois do handler.
/// `events` e `classes` filtram (nomes separados por `;`; vazio = todos, menos os contínuos,
/// como mover o mouse). Veja `aylis_core` (`intercept.rs`) para a ordem e o backend remoto.
pub struct EventInterceptor {
    pub scope: InterceptScope,
    pub events: String,
    pub classes: String,
    pub enabled: bool,
    pub tag: i64,
}

impl Default for EventInterceptor {
    fn default() -> Self {
        Self { scope: InterceptScope::Form, events: String::new(), classes: String::new(), enabled: true, tag: 0 }
    }
}

impl Component for EventInterceptor {
    fn class_name(&self) -> &'static str {
        "EventInterceptor"
    }

    properties! {
        scope: InterceptScope => "Behavior",
        events: String => "Behavior",
        classes: String => "Behavior",
        enabled: bool => "Behavior",
        tag: i64 => "Misc";
        events: on_intercept, after_intercept
    }
}

// ---------------------------------------------------------------------------
// BusyWatcher

/// O que a tela mostra enquanto o backend está ocupado.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BusyAction {
    /// O cursor de espera e, depois de uns 3 s, o indicador do aylis.
    #[default]
    Default,
    /// O `busy_form` (um form com o progresso, a animação, o que for).
    Form,
    /// Nada visível (a entrada continua recusada, com `block_input`).
    None,
}

impl PropEnum for BusyAction {
    const NAMES: &'static [&'static str] = &["default", "form", "none"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Default, Self::Form, Self::None].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// A espera pelo backend em outro processo (não visual, no form principal): o que a tela faz
/// quando um handler do backend passa do `delay` sem terminar. Sem ele, o padrão é o cursor de
/// espera e, depois de uns 3 s, um indicador discreto. Conveniência para quem não pôs o
/// trabalho pesado numa thread (veja `docs/threads.md`). `on_busy_start` roda antes de
/// mostrar (recusar, `ui.refuse("")`, pula o `busy_form`) e `on_busy_end` depois de esconder,
/// do lado da tela (em Rust ou no próprio `busy_form`).
pub struct BusyWatcher {
    /// Quanto esperar antes de mostrar, em milissegundos.
    pub delay: i64,
    pub action: BusyAction,
    /// O form mostrado com `action = form`.
    pub busy_form: Option<ComponentId>,
    /// Recusa o clique (e guarda as teclas) enquanto o backend está ocupado.
    pub block_input: bool,
    /// O `busy_form` fica pelo menos isto à mostra (sem piscar), em milissegundos.
    pub min_visible: i64,
}

impl Default for BusyWatcher {
    fn default() -> Self {
        Self { delay: 1000, action: BusyAction::Default, busy_form: None, block_input: true, min_visible: 300 }
    }
}

impl Component for BusyWatcher {
    fn class_name(&self) -> &'static str {
        "BusyWatcher"
    }

    properties! {
        delay: i64 => "Behavior",
        action: BusyAction => "Behavior",
        busy_form: Option<ComponentId> => "Behavior",
        block_input: bool => "Behavior",
        min_visible: i64 => "Behavior";
        events: on_busy_start, on_busy_end
    }
}

// ---------------------------------------------------------------------------
// OpenDialog / SaveDialog

/// Lê o filtro no formato do Delphi: `Imagens|*.png;*.jpg|Todos|*.*`. O `*.*` (ou `*`) vira a
/// extensão `"*"`: todos os arquivos.
pub fn parse_filter(filter: &str) -> Vec<(String, Vec<String>)> {
    let parts: Vec<&str> = filter.split('|').collect();
    parts
        .chunks(2)
        .filter_map(|pair| {
            let name = pair.first()?.trim();
            let exts: Vec<String> = pair
                .get(1)?
                .split(';')
                .map(|e| e.trim().trim_start_matches("*.").trim_start_matches('.').to_string())
                .filter(|e| !e.is_empty())
                .collect();
            Some((name.to_string(), exts))
        })
        .filter(|(_, exts)| !exts.is_empty())
        .collect()
}

/// Diálogo de abrir arquivo do sistema (como o `TOpenDialog`). Assíncrono:
/// [`OpenDialog::execute`] abre e, ao escolher, grava `file_name` e dispara `on_select`
/// (com o caminho; com `multiple`, os caminhos separados por `;`). Cancelar dispara
/// `on_cancel`. Em toda plataforma: o diálogo do sistema no desktop, o seletor do navegador,
/// o seletor de documentos do Android e do iOS. Leia e grave o caminho escolhido com
/// [`aylis_core::files`] (no navegador o arquivo fica na memória; salvar baixa).
#[derive(Default)]
pub struct OpenDialog {
    pub title: String,
    /// `Imagens|*.png;*.jpg|Todos|*.*`
    pub filter: String,
    pub initial_dir: String,
    pub file_name: String,
    pub multiple: bool,
}


/// Diálogo de salvar arquivo do sistema (como o `TSaveDialog`). Igual ao [`OpenDialog`].
#[derive(Default)]
pub struct SaveDialog {
    pub dialog: OpenDialog,
}

fn run_file_dialog(ui: &mut Ui, id: ComponentId, options: FileDialogOptions) {
    let poster = ui.poster();
    let done = Box::new(move |paths: Vec<String>| {
        poster.post(move |ui| {
            if paths.is_empty() {
                ui.fire(id, "on_cancel", &EventArgs::None);
            } else {
                let joined = paths.join(";");
                let _ = ui.set_prop(id, "file_name", joined.clone());
                ui.fire(id, "on_select", &EventArgs::Text(joined));
            }
        });
    });
    match ui.services_mut().file_dialogs.as_mut() {
        Some(service) => service.show(options, done),
        None => done(Vec::new()),
    }
}

impl OpenDialog {
    fn options(&self, save: bool) -> FileDialogOptions {
        FileDialogOptions {
            title: self.title.clone(),
            filters: parse_filter(&self.filter),
            directory: self.initial_dir.clone(),
            file_name: self.file_name.clone(),
            save,
            multiple: self.multiple && !save,
            owner: None,
        }
    }

    /// Abre o diálogo (o `OpenDialog` ou o `SaveDialog` de `id`).
    pub fn execute(ui: &mut Ui, id: ComponentId) {
        // O SaveDialog primeiro: o `get::<OpenDialog>` também acha o de dentro dele.
        let options = if let Some(d) = ui.get::<SaveDialog>(id) {
            d.dialog.options(true)
        } else if let Some(d) = ui.get::<OpenDialog>(id) {
            d.options(false)
        } else {
            return;
        };
        // A janela do form do diálogo é a dona (o relatório do Tabernáculo, J02).
        let options = FileDialogOptions { owner: Some(ui.root_of(id)), ..options };
        run_file_dialog(ui, id, options);
    }
}

impl Component for OpenDialog {
    fn class_name(&self) -> &'static str {
        "OpenDialog"
    }

    properties! {
        title: String => "Appearance",
        filter: String => "Behavior",
        initial_dir: String => "Behavior",
        file_name: String => "Data",
        multiple: bool => "Behavior";
        events: on_select, on_cancel
    }
}

impl SaveDialog {
    pub fn execute(ui: &mut Ui, id: ComponentId) {
        OpenDialog::execute(ui, id);
    }
}

impl Component for SaveDialog {
    fn class_name(&self) -> &'static str {
        "SaveDialog"
    }

    fn inner(&self) -> Option<&dyn Component> {
        Some(&self.dialog)
    }

    fn inner_mut(&mut self) -> Option<&mut dyn Component> {
        Some(&mut self.dialog)
    }
}

// ---------------------------------------------------------------------------
// MessageDlg

/// Tipo da mensagem (o ícone).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DlgKind {
    #[default]
    Information,
    Warning,
    Error,
    Confirmation,
}

impl PropEnum for DlgKind {
    const NAMES: &'static [&'static str] = &["information", "warning", "error", "confirmation"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Information, Self::Warning, Self::Error, Self::Confirmation].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Caixa de mensagem como componente (como o `TTaskDialog`): fica no form com o texto, o
/// tipo e os botões definidos no Object Inspector, e abre com [`MessageDialog::execute`].
/// Feita com o próprio aylis (igual no desktop, no celular e no navegador). Ao fechar,
/// dispara `on_close` com o botão escolhido (`"yes"`, `"no"`, `"ok"`, `"cancel"`...).
pub struct MessageDialog {
    /// Título da janela; vazio: o do tipo ("Confirmação", "Aviso"...).
    pub title: String,
    pub text: String,
    pub kind: DlgKind,
    /// Os botões, na ordem em que aparecem: `"yes,no"`, `"ok,cancel"`, `"yes,no,cancel"`...
    pub buttons: String,
}

impl Default for MessageDialog {
    fn default() -> Self {
        Self { title: String::new(), text: String::new(), kind: DlgKind::Information, buttons: "ok".into() }
    }
}

impl MessageDialog {
    pub fn new(text: impl Into<String>, kind: DlgKind, buttons: &[ModalResult]) -> Self {
        let buttons = buttons.iter().map(|b| b.name()).collect::<Vec<_>>().join(",");
        Self { text: text.into(), kind, buttons, ..Default::default() }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Os botões de `buttons` (nomes desconhecidos ficam de fora; nenhum: OK).
    pub fn button_list(&self) -> Vec<ModalResult> {
        self.buttons
            .split(',')
            .filter_map(|b| ModalResult::NAMES.iter().position(|n| n.eq_ignore_ascii_case(b.trim())).and_then(ModalResult::from_index))
            .filter(|r| *r != ModalResult::None)
            .collect()
    }

    /// Abre a caixa do componente `id`; a resposta chega em `on_close`.
    pub fn execute(ui: &mut Ui, id: ComponentId) -> Option<ComponentId> {
        Self::execute_then(ui, id, |_, _| {})
    }

    /// Como [`execute`](Self::execute), e também chama `then` com o botão escolhido (para
    /// quem prefere tratar a resposta no lugar em que abriu).
    pub fn execute_then(ui: &mut Ui, id: ComponentId, then: impl FnOnce(&mut Ui, ModalResult) + 'static) -> Option<ComponentId> {
        let d = ui.get::<MessageDialog>(id)?;
        let (title, text, kind, buttons) = (d.title.clone(), d.text.clone(), d.kind, d.button_list());
        Some(message_box(ui, &title, &text, kind, &buttons, &[], false, move |ui, result| {
            ui.fire(id, "on_close", &EventArgs::Text(result.name().to_string()));
            then(ui, result);
        }))
    }
}

impl Component for MessageDialog {
    fn class_name(&self) -> &'static str {
        "MessageDialog"
    }

    properties! {
        title: String => "Appearance",
        text: String => "Appearance",
        kind: DlgKind => "Appearance",
        buttons: String => "Behavior";
        events: on_close
    }
}

/// Uma caixa de mensagem (como o `MessageDlg`), feita com o próprio aylis: funciona igual
/// no desktop, no celular e no navegador. Não bloqueia: `on_close` recebe o botão
/// escolhido (Esc = `Cancel`, ou `No` se não houver Cancelar).
///
/// ```ignore
/// message_dlg(ui, "Excluir o cliente?", DlgKind::Confirmation, &[ModalResult::Yes, ModalResult::No], |ui, r| {
///     if r == ModalResult::Yes { /* ... */ }
/// });
/// ```
pub fn message_dlg(
    ui: &mut Ui,
    text: &str,
    kind: DlgKind,
    buttons: &[ModalResult],
    on_close: impl FnOnce(&mut Ui, ModalResult) + 'static,
) -> ComponentId {
    message_box(ui, "", text, kind, buttons, &[], false, on_close)
}

/// Pergunta de sim ou não com os textos do app: `title`, `text`, o botão `ok` (ao lado do
/// "Cancelar"); `danger` pinta o botão de vermelho (excluir, apagar). `on_close` recebe
/// `true` no OK.
pub fn confirm(ui: &mut Ui, title: &str, text: &str, ok: &str, danger: bool, on_close: impl FnOnce(&mut Ui, bool) + 'static) -> ComponentId {
    let kind = if danger { DlgKind::Warning } else { DlgKind::Confirmation };
    let cancel = aylis_core::messages::t("Cancelar").to_string();
    message_box(ui, title, text, kind, &[ModalResult::Ok, ModalResult::Cancel], &[ok, &cancel], danger, move |ui, r| {
        on_close(ui, r == ModalResult::Ok)
    })
}

/// A caixa de mensagem (do [`message_dlg`] e do [`MessageDialog`]); `title` vazio: o do tipo.
fn message_box(
    ui: &mut Ui,
    title: &str,
    text: &str,
    kind: DlgKind,
    buttons: &[ModalResult],
    captions: &[&str],
    danger: bool,
    on_close: impl FnOnce(&mut Ui, ModalResult) + 'static,
) -> ComponentId {
    let title = if title.is_empty() {
        aylis_core::messages::t(match kind {
            DlgKind::Information => "Informação",
            DlgKind::Warning => "Aviso",
            DlgKind::Error => "Erro",
            DlgKind::Confirmation => "Confirmação",
        })
        .to_string()
    } else {
        title.to_string()
    };
    let form = ui.add_root(Form::new(title.clone(), 440.0, 200.0));
    let _ = ui.set_prop(form, "border_style", "single");
    if let Some(cb) = ui.control_mut(form) {
        cb.set_padding(BOX_PADDING);
    }
    let footer = ui.add(Layout::default(), form);
    if let Some(cb) = ui.control_mut(footer) {
        cb.set_align(Align::Bottom).set_height(FOOTER_HEIGHT).set_margins([0.0, FOOTER_GAP, 0.0, 0.0]);
    }
    let buttons: Vec<ModalResult> = if buttons.is_empty() { vec![ModalResult::Ok] } else { buttons.to_vec() };
    // Com align right, o primeiro fica mais à direita: a ordem visual é a da lista.
    let mut button_captions = Vec::with_capacity(buttons.len());
    for (i, result) in buttons.iter().rev().enumerate() {
        let caption = captions.get(buttons.len() - 1 - i).map_or_else(|| aylis_core::messages::t(button_text(*result)).to_string(), |c| c.to_string());
        button_captions.insert(0, caption.clone());
        let b = ui.add(Button::new(caption), footer);
        if let Some(cb) = ui.control_mut(b) {
            cb.set_align(Align::Right).set_width(BUTTON_WIDTH).set_margins([BUTTON_GAP, 0.0, 0.0, 0.0]);
        }
        let _ = ui.set_prop(b, "modal_result", result.name());
        if i + 1 < buttons.len() {
            let _ = ui.set_prop(b, "style", "flat");
        } else {
            if danger {
                let _ = ui.set_prop(b, "style", "danger");
            }
            ui.set_focus(b);
        }
    }
    // O Ctrl+C copia a caixa inteira, no formato do Windows (o ícone atende o atalho).
    let rule = "---------------------------";
    let copy = format!("{rule}\n{title}\n{rule}\n{text}\n{rule}\n{}\n{rule}\n", button_captions.join("   "));
    let icon = ui.add(DlgIcon { base: ControlBase::new(ICON_SIZE, ICON_SIZE), kind, copy }, form);
    if let Some(cb) = ui.control_mut(icon) {
        cb.set_align(Align::Left).set_margins([0.0, 0.0, ICON_GAP, 0.0]);
    }
    // A janela no tamanho do texto: a largura da linha mais comprida (até um máximo, depois
    // quebra) e a altura das linhas; passou da altura máxima, o texto rola.
    let label = ui.add(Label::new(text), form);
    let _ = ui.set_prop(label, "wrap", true);
    let _ = ui.set_prop(label, "v_align", "leading");
    let style = aylis_core::TextStyle { font: ui.font_of(label), wrap: true, ..Default::default() };
    let text_width = (ui.measure_text_wrapped(text, &style, TEXT_MAX_WIDTH).width.ceil() + 2.0).clamp(TEXT_MIN_WIDTH, TEXT_MAX_WIDTH);
    let text_height = ui.measure_text_wrapped(text, &style, text_width).height.ceil() + 2.0;
    let mut client_width = text_width;
    if text_height > TEXT_MAX_HEIGHT {
        let scroller = ui.add(ScrollBox::default(), form);
        let bar = ui.control(scroller).map_or(0.0, |cb| cb.padding().right);
        if let Some(cb) = ui.control_mut(scroller) {
            cb.set_align(Align::Client);
        }
        let _ = ui.set_parent(label, Some(scroller));
        if let Some(cb) = ui.control_mut(label) {
            cb.set_align(Align::Top).set_height(text_height);
        }
        client_width += bar;
    } else if let Some(cb) = ui.control_mut(label) {
        cb.set_align(Align::Client);
    }
    let buttons_width = buttons.len() as f32 * (BUTTON_WIDTH + BUTTON_GAP);
    let width = (BOX_PADDING * 2.0 + ICON_SIZE + ICON_GAP + client_width).max(BOX_PADDING * 2.0 + buttons_width);
    let height = BOX_PADDING * 2.0 + text_height.min(TEXT_MAX_HEIGHT).max(ICON_SIZE) + FOOTER_GAP + FOOTER_HEIGHT;
    if let Some(cb) = ui.control_mut(form) {
        cb.set_width(width.ceil()).set_height(height.ceil());
    }
    let escape = if buttons.contains(&ModalResult::Cancel) {
        ModalResult::Cancel
    } else if buttons.contains(&ModalResult::No) {
        ModalResult::No
    } else {
        buttons[0]
    };
    let _ = ui.show_modal(form, move |ui, result| {
        let result = if result == ModalResult::None { escape } else { result };
        ui.free(form);
        on_close(ui, result);
    });
    form
}

/// Mensagem simples com OK (como o `ShowMessage`).
pub fn show_message(ui: &mut Ui, text: &str) -> ComponentId {
    message_dlg(ui, text, DlgKind::Information, &[ModalResult::Ok], |_, _| {})
}

fn button_text(result: ModalResult) -> &'static str {
    match result {
        ModalResult::Ok => "OK",
        ModalResult::Cancel => "Cancelar",
        ModalResult::Yes => "Sim",
        ModalResult::No => "Não",
        ModalResult::Abort => "Abortar",
        ModalResult::Retry => "Repetir",
        ModalResult::Ignore => "Ignorar",
        ModalResult::Close => "Fechar",
        ModalResult::None => "",
    }
}

// As medidas da caixa de mensagem.
const BOX_PADDING: f32 = 18.0;
const ICON_SIZE: f32 = 36.0;
const ICON_GAP: f32 = 14.0;
const FOOTER_HEIGHT: f32 = 36.0;
const FOOTER_GAP: f32 = 14.0;
const BUTTON_WIDTH: f32 = 100.0;
const BUTTON_GAP: f32 = 8.0;
const TEXT_MIN_WIDTH: f32 = 240.0;
const TEXT_MAX_WIDTH: f32 = 520.0;
/// Acima disto o texto rola (a caixa de pânico mostra a mensagem inteira).
const TEXT_MAX_HEIGHT: f32 = 360.0;

/// O ícone redondo da caixa de mensagem. Atende também o Ctrl+C da caixa (`copy`).
struct DlgIcon {
    base: ControlBase,
    kind: DlgKind,
    copy: String,
}

impl Component for DlgIcon {
    leaf_control!("DlgIcon");

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let (token, on, glyph) = match self.kind {
            DlgKind::Information => ("primary", "on_primary", "i"),
            DlgKind::Warning => ("warning", "on_warning", "!"),
            DlgKind::Error => ("danger", "on_danger", ""),
            DlgKind::Confirmation => ("primary", "on_primary", "?"),
        };
        // O círculo ocupa o controle inteiro (sem sobra em volta).
        let side = ctx.rect.width.min(ctx.rect.height);
        let r = RectF::new(ctx.rect.x + (ctx.rect.width - side) / 2.0, ctx.rect.y + (ctx.rect.height - side) / 2.0, side, side);
        canvas.fill_ellipse(r, &Brush::Solid(ctx.theme.color(token)));
        // A cor sobre o círculo; o tema sem ela usa a do texto sobre o primário.
        let color = ctx.theme.colors.get(on).copied().unwrap_or_else(|| ctx.theme.color("on_primary"));
        if glyph.is_empty() {
            // O X em dois traços, no centro exato do círculo (o "✕" da fonte fica fora do
            // centro, e cada fonte num lugar).
            let (c, arm) = (r.center(), side * 0.2);
            let stroke = Stroke { cap: LineCap::Round, ..Stroke::new(color, (side * 0.09).max(2.0)) };
            canvas.draw_line(PointF::new(c.x - arm, c.y - arm), PointF::new(c.x + arm, c.y + arm), &stroke);
            canvas.draw_line(PointF::new(c.x - arm, c.y + arm), PointF::new(c.x + arm, c.y - arm), &stroke);
            return;
        }
        let mut text = ctx.text_style();
        text.h_align = TextAlign::Center;
        text.v_align = TextAlign::Center;
        text.color = color;
        text.font.size = (side * 0.56).round();
        text.font.weight = aylis_core::FontWeight::Bold;
        canvas.draw_text(glyph, r, &text);
    }

    fn shortcut(&mut self, ctx: &mut EventCtx<'_>, event: &KeyEvent) {
        let ctrl = event.modifiers.ctrl || event.modifiers.meta;
        if ctrl && !event.modifiers.alt && matches!(event.key, Key::Char('c' | 'C')) {
            if let Some(clipboard) = ctx.ui.services_mut().clipboard.as_mut() {
                clipboard.set_text(&self.copy);
            }
            ctx.set_handled();
        }
    }
}

// ---------------------------------------------------------------------------
// Toast

/// Aviso rápido no rodapé da tela, que some sozinho (como o toast do Android). Funciona
/// em qualquer plataforma; não rouba o foco.
pub fn toast(ui: &mut Ui, text: &str) {
    let Some(form) = ui.open_forms().last().copied().or_else(|| ui.roots().first().copied()) else { return };
    let size = ui.bounds(form).map_or((360.0, 640.0), |b| (b.width, b.height));
    let width = (text.chars().count() as f32 * 8.0 + 48.0).clamp(160.0, (size.0 - 32.0).max(160.0));
    let mut base = ControlBase::new(width, 44.0);
    base.set_rect((size.0 - width) / 2.0, size.1 - 44.0 - 32.0, width, 44.0).set_hit_test(false);
    ui.add(Toast { base, text: text.to_string() }, form);
}

struct Toast {
    base: ControlBase,
    text: String,
}

impl Component for Toast {
    leaf_control!("Toast");

    fn timer_interval(&self) -> Option<f64> {
        Some(2.5)
    }

    fn timer(&mut self, ctx: &mut EventCtx<'_>) {
        let id = ctx.id;
        ctx.ui.poster().post(move |ui| {
            ui.free(id);
        });
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        canvas.fill_rect(r.offset(0.0, 2.0), r.height / 2.0, &Brush::Solid(Color::BLACK.with_opacity(0.18)));
        canvas.fill_rect(r, r.height / 2.0, &Brush::Solid(Color::from_hex("#323232").unwrap_or(Color::BLACK).with_opacity(0.94)));
        let mut text = ctx.text_style();
        text.h_align = TextAlign::Center;
        text.color = Color::WHITE;
        canvas.draw_text(&self.text, r.inflate(-12.0), &text);
    }
}

// ---------------------------------------------------------------------------
// Notification

/// Notificação do sistema (o toast do Windows, que fica na central de notificações). Não
/// visual: `Notification::show(ui, id)` mostra. Onde o sistema não oferece (navegador),
/// vira um aviso dentro do app ([`toast`]).
///
/// No Android: canal, prioridade, botões (`on_action`), toque (`on_click`), progresso,
/// fixa, imagem grande e até um form inteiro desenhado como imagem (`form`).
pub struct Notification {
    pub title: String,
    pub text: String,
    /// Imagem (PNG) ao lado do texto, opcional: arquivo ou asset do projeto.
    pub image: String,
    /// Um form inteiro desenhado como a imagem grande da notificação (o nome dele; criado
    /// se preciso), no tamanho `form_width` x `form_height`.
    pub form: String,
    pub form_width: f32,
    pub form_height: f32,
    /// O nome do app na notificação (vazio: o título do form principal).
    pub app_name: String,
    /// O identificador do app no sistema (`empresa.app`; vazio: derivado do nome).
    pub app_id: String,
    /// Mostrar de novo com o mesmo número atualiza a notificação em vez de criar outra.
    pub notification_id: u32,
    /// Canal do Android (o usuário configura som e destaque por canal).
    pub channel: String,
    pub channel_name: String,
    pub priority: NotificationPriority,
    /// Botões: `"responder:Responder;arquivar:Arquivar"` (identificador:texto); o toque
    /// chega em `on_action` com o identificador.
    pub actions: String,
    /// Fixa (o usuário não descarta), como um download em andamento.
    pub ongoing: bool,
    /// Barra de progresso de 0 a 1; negativo: sem barra.
    pub progress: f32,
    /// Android: abre o app por cima de tudo, até da tela de bloqueio (ligações, alarmes).
    pub full_screen: bool,
}

impl Default for Notification {
    fn default() -> Self {
        Self {
            title: String::new(),
            text: String::new(),
            image: String::new(),
            form: String::new(),
            form_width: 360.0,
            form_height: 180.0,
            app_name: String::new(),
            app_id: String::new(),
            notification_id: 1,
            channel: "geral".into(),
            channel_name: "Geral".into(),
            priority: NotificationPriority::Default,
            actions: String::new(),
            ongoing: false,
            progress: -1.0,
            full_screen: false,
        }
    }
}

impl Notification {
    /// Mostra a notificação.
    pub fn show(ui: &mut Ui, id: ComponentId) -> Result<(), String> {
        let Some(n) = ui.get::<Notification>(id) else { return Err("não é uma Notification".into()) };
        let main_caption = ui.main_form().and_then(|f| match ui.get_prop(f, "caption") {
            Ok(aylis_core::PropValue::String(c)) => Some(c),
            _ => None,
        });
        let app_name = if n.app_name.is_empty() { main_caption.unwrap_or_else(|| "aylis".into()) } else { n.app_name.clone() };
        let app_id = if n.app_id.is_empty() {
            format!("aylis.{}", app_name.to_lowercase().chars().filter(|c| c.is_ascii_alphanumeric()).collect::<String>())
        } else {
            n.app_id.clone()
        };
        let actions = n
            .actions
            .split(';')
            .filter_map(|a| {
                let (id, text) = a.split_once(':').unwrap_or((a, a));
                (!id.trim().is_empty()).then(|| (id.trim().to_string(), text.trim().to_string()))
            })
            .collect();
        let (form, form_size, image) = (n.form.clone(), (n.form_width, n.form_height), n.image.clone());
        let mut message = aylis_core::platform::SystemNotification {
            app_id,
            app_name,
            title: n.title.clone(),
            body: n.text.clone(),
            image: n.image.clone(),
            id: n.notification_id,
            channel: n.channel.clone(),
            channel_name: n.channel_name.clone(),
            priority: n.priority,
            actions,
            ongoing: n.ongoing,
            progress: (n.progress >= 0.0).then(|| n.progress.min(1.0)),
            full_screen: n.full_screen,
            component: Some(id),
            ..Default::default()
        };
        // A imagem: um form desenhado, ou o arquivo/asset (no celular, os assets vêm de
        // dentro do app, sem arquivo).
        if !form.is_empty() {
            let f = ui.create_form(&form).map_err(|e| e.to_string())?;
            message.image_png = ui.render_png(f, form_size.0, form_size.1, 2.0).unwrap_or_default();
            message.big_image = true;
        } else if !image.is_empty() && !std::path::Path::new(&image).is_file() {
            message.image_png = ui.services().resources.as_ref().and_then(|r| r.read(&image)).unwrap_or_default();
        }
        match ui.services_mut().notifications.as_mut() {
            Some(service) => service.notify(&message),
            None => {
                let text = if message.body.is_empty() { message.title.clone() } else { format!("{}: {}", message.title, message.body) };
                toast(ui, &text);
                Ok(())
            }
        }
    }
}

impl Notification {
    /// Tira a notificação da lista do sistema.
    pub fn cancel(ui: &mut Ui, id: ComponentId) -> Result<(), String> {
        let n = ui.get::<Notification>(id).ok_or("não é uma Notification")?.notification_id;
        match ui.services_mut().notifications.as_mut() {
            Some(service) => service.cancel(n),
            None => Ok(()),
        }
    }
}

impl Component for Notification {
    fn class_name(&self) -> &'static str {
        "Notification"
    }

    properties! {
        title: String => "Behavior",
        text: String => "Behavior",
        image: String => "Appearance",
        form: String => "Appearance",
        form_width: f32 => "Appearance",
        form_height: f32 => "Appearance",
        app_name: String => "Behavior",
        app_id: String => "Behavior",
        notification_id: u32 => "Behavior",
        channel: String => "Behavior",
        channel_name: String => "Behavior",
        priority: NotificationPriority => "Behavior",
        actions: String => "Behavior",
        ongoing: bool => "Behavior",
        progress: f32 => "Behavior",
        full_screen: bool => "Behavior";
        events: on_click, on_action
    }
}
