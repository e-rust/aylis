//! # aylis-dbcontrols
//!
//! Os controles de dados que não têm um controle comum equivalente (os outros — texto,
//! caixa de marcação, lista... — são o controle comum com "Ligar a campo…", veja
//! `aylis_data::binding`):
//!
//! | Controle | O que faz |
//! |---|---|
//! | [`DataGrid`] | grade do dataset (ordena pelo título, colunas fixas, linha de filtro) |
//! | [`DataNavigator`] | os botões de navegar e editar |
//! | [`DataRepeater`] | repete um modelo por registro, com rolagem virtual |
//! | [`DBImage`] | a imagem de um campo |
//! | [`DBChart`] | gráfico das séries do dataset |
//!
//! E os relatórios ([`Report`], [`ReportBand`], [`ReportPreview`]).

mod chart;
mod column;
mod grid;
mod more;
mod navigator;
pub mod report;
mod virtual_list;

use std::any::Any;

use aylis_controls::Edit;
use aylis_core::{Canvas, Component, ComponentId, EventCtx, Key, KeyEvent, PaintCtx, PropValue, Stroke, Ui, properties};
use aylis_data::{DataEvent, DataMessage, FieldLink, data_set};

pub use chart::DBChart;
pub use column::Column;
pub use grid::{CellInfo, CellPainter, CellStyle, DataGrid, GridColumn};
pub use more::DBImage;
pub use navigator::{DataNavigator, NavButton};
pub use report::{BandKind, Report, ReportBand, ReportPreview};
pub use virtual_list::DataRepeater;

/// Comandos que o [`DataGrid`] manda ao [`CellEditor`] que ele usa como editor de célula.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EditorCommand {
    /// Digita este texto (a primeira tecla que abriu o editor).
    Type(String),
    /// Grava o texto pendente no campo agora.
    Commit,
}

/// Registra os controles de dados (para layouts JSON e para a IDE).
pub fn register(ui: &mut Ui) {
    ui.register_class::<DataNavigator>("Data Controls");
    ui.register_class::<DataGrid>("Data Controls");
    ui.register_class::<Column>("Data Controls");
    ui.register_class::<DataRepeater>("Data Controls");
    ui.register_class::<DBImage>("Data Controls");
    ui.register_class::<DBChart>("Data Controls");
    ui.register_class::<Report>("Reports");
    ui.register_class::<ReportBand>("Reports");
    ui.register_class::<ReportPreview>("Reports");
}

fn link_of(data_source: Option<ComponentId>, data_field: &str) -> FieldLink {
    FieldLink { data_source, data_field: data_field.to_string() }
}

fn data_message(message: &dyn Any) -> Option<&DataMessage> {
    message.downcast_ref::<DataMessage>()
}

// ---------------------------------------------------------------------------

/// O editor de célula do [`DataGrid`]: um campo de texto ligado a um campo do dataset (não
/// fica na paleta; no form, é o `Edit` com "Ligar a campo…").
///
/// - mostra o texto de exibição; ao receber o foco, troca pelo texto de edição;
/// - começar a digitar coloca o dataset em edição (`auto_edit` do `DataSource`);
/// - ao sair do campo (ou no `post`), grava no campo com as regras dele; um valor
///   inválido deixa a borda vermelha e impede o `post`;
/// - Esc desfaz o que foi digitado.
#[derive(Default)]
pub struct CellEditor {
    pub base: Edit,
    pub data_source: Option<ComponentId>,
    pub data_field: String,
    /// O campo ligado é dado pessoal (`sensitive` do Field): conta como `sensitive` para as
    /// gravações. Acompanha o campo (só leitura).
    pub field_sensitive: bool,
    modified: bool,
    error: Option<String>,
}

impl CellEditor {
    pub fn new(data_source: ComponentId, data_field: &str) -> Self {
        Self { data_source: Some(data_source), data_field: data_field.to_string(), ..Default::default() }
    }

    fn link(&self) -> FieldLink {
        link_of(self.data_source, &self.data_field)
    }

    /// Há texto digitado ainda não gravado no campo.
    pub fn is_modified(&self) -> bool {
        self.modified
    }

    /// Mensagem de erro do último valor digitado, se ele foi recusado.
    pub fn error(&self) -> Option<&str> {
        self.error.as_deref()
    }

    fn focused(&self) -> bool {
        self.base.base.state().focused
    }

    /// Carrega o texto do campo (de edição se tiver foco).
    fn load(&mut self, ui: &Ui) {
        let link = self.link();
        let text = if self.focused() { link.edit_text(ui) } else { link.display_text(ui) };
        let _ = self.base.set_prop("text", PropValue::String(text));
        self.base.read_only = !link.can_modify(ui);
        self.base.max_length = link.field(ui).map_or(0, |f| f.size);
        self.field_sensitive = link.field(ui).is_some_and(|f| f.sensitive);
        self.modified = false;
        self.error = None;
    }

    /// Grava o texto pendente no campo. Devolve a mensagem de erro, se recusado.
    fn write(&mut self, ctx: &mut EventCtx<'_>) -> Option<String> {
        if !self.modified {
            return None;
        }
        match self.link().write_text(ctx.ui, &self.base.text) {
            Ok(()) => {
                self.modified = false;
                self.error = None;
                None
            }
            Err(e) => {
                self.error = Some(e.message.clone());
                ctx.repaint();
                Some(e.message)
            }
        }
    }

    /// Depois de uma tecla: se o texto mudou, entra em edição (e encaixa na máscara do
    /// campo, se houver: os literais aparecem sozinhos).
    fn after_change(&mut self, ctx: &mut EventCtx<'_>, before: String) {
        if self.base.text == before {
            return;
        }
        let mask = self.link().field(ctx.ui).and_then(|f| aylis_data::mask::Mask::parse(&f.edit_mask));
        if let Some(mask) = mask {
            // Os literais aparecem quando chega o caractere seguinte a eles.
            let formatted = mask.apply(&self.base.text);
            if formatted != self.base.text {
                let _ = self.base.set_prop("text", PropValue::String(formatted));
            }
        }
        if !self.modified {
            if let Err(e) = self.link().begin_edit(ctx.ui) {
                let _ = self.base.set_prop("text", PropValue::String(before));
                self.error = Some(e.message);
                return;
            }
        }
        self.modified = true;
        self.error = None;
    }
}

impl Component for CellEditor {
    fn class_name(&self) -> &'static str {
        "CellEditor"
    }

    fn inner(&self) -> Option<&dyn Component> {
        Some(&self.base)
    }

    fn inner_mut(&mut self) -> Option<&mut dyn Component> {
        Some(&mut self.base)
    }

    properties! {
        data_source: Option<ComponentId> => "Data",
        data_field: String => "Data",
        field_sensitive: bool => "Data",
    }

    fn message(&mut self, ctx: &mut EventCtx<'_>, message: &dyn Any) {
        if let Some(command) = message.downcast_ref::<EditorCommand>() {
            match command {
                EditorCommand::Type(text) => self.text_input(ctx, text),
                EditorCommand::Commit => {
                    self.write(ctx);
                }
            }
            return;
        }
        let Some(m) = data_message(message) else { return };
        if !self.link().concerns(ctx.ui, m) {
            return;
        }
        match &m.event {
            DataEvent::UpdateData => {
                if let Some(error) = self.write(ctx) {
                    if let Some(ds) = self.link().data_set(ctx.ui) {
                        data_set(ctx.ui, ds).report_pending_error(error);
                    }
                }
            }
            // O usuário está digitando neste campo: não sobrescreve.
            DataEvent::RecordChanged(_) if self.modified => {}
            _ => {
                self.load(ctx.ui);
                ctx.repaint();
            }
        }
    }

    fn text_input(&mut self, ctx: &mut EventCtx<'_>, text: &str) {
        let before = self.base.text.clone();
        self.base.text_input(ctx, text);
        self.after_change(ctx, before);
    }

    fn key_down(&mut self, ctx: &mut EventCtx<'_>, event: &KeyEvent) {
        if event.key == Key::Escape && self.modified {
            self.load(ctx.ui);
            self.base.select_all();
            ctx.set_handled();
            ctx.repaint();
            return;
        }
        let before = self.base.text.clone();
        self.base.key_down(ctx, event);
        self.after_change(ctx, before);
    }

    fn focus_enter(&mut self, ctx: &mut EventCtx<'_>) {
        // Precisa do estado `focused` já ligado para carregar o texto de edição.
        if !self.modified {
            self.base.base.state_mut().focused = true;
            self.load(ctx.ui);
        }
        self.base.focus_enter(ctx);
        if self.modified {
            // Já há algo digitado (por exemplo a tecla que abriu o editor do grid):
            // cursor no fim, sem selecionar.
            let end = self.base.text.chars().count();
            self.base.select(end, end);
        }
    }

    fn focus_exit(&mut self, ctx: &mut EventCtx<'_>) {
        self.write(ctx);
        self.base.focus_exit(ctx);
        if !self.modified {
            self.base.base.state_mut().focused = false;
            self.load(ctx.ui);
        }
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        self.base.paint(canvas, ctx);
        if self.error.is_some() {
            let danger = ctx.theme.color("danger");
            canvas.stroke_rect(ctx.rect, ctx.style.radius, &Stroke::new(danger, 2.0));
        }
    }
}

