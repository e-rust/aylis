//! Os componentes não visuais (Timer, diálogos, dados, menus...): um ícone próprio para
//! cada um (paleta, bandeja do designer, lista) e uma descrição curta; e a janela que
//! lista todos com ícone, nome e descrição (o botão ao lado da galeria). A galeria mostra
//! só os visuais.

use aylis_controls::{Button, Edit, IconView, Label, Layout, Rectangle, ScrollBox};
use aylis_core::{Align, ComponentId, ModalResult, Ui};

use crate::app::{Ids, refresh};
use crate::dialogs::new_dialog;
use crate::panels::choose;
use crate::state::Shared;
use crate::widgets::{caption, clear, muted};

/// (classe, ícone, descrição) dos não visuais do aylis.
const INFO: &[(&str, &str, &str)] = &[
    ("Timer", "clock", "Dispara on_timer a cada intervalo."),
    ("EventInterceptor", "filter", "Recebe num lugar só os eventos de todos os componentes do form ou da aplicação, e pode cancelar."),
    ("PopupMenu", "menu", "Menu de contexto: abre no clique direito do controle que o usa em popup_menu."),
    ("MenuItem", "dot", "Item de um menu (texto, ícone, atalho, marca); dentro de outro vira submenu."),
    ("OpenDialog", "folder", "Escolher um arquivo para abrir (diálogo do sistema)."),
    ("SaveDialog", "download", "Escolher onde salvar um arquivo (diálogo do sistema)."),
    ("Clipboard", "copy", "Copiar e colar texto, HTML, imagem e os formatos do app pela área de transferência."),
    ("UrlScheme", "link", "Um prefixo de URL próprio (meuapp://) que abre o app e entrega o endereço no on_link."),
    ("MessageDialog", "chat", "Caixa de mensagem com título, texto, tipo e botões; on_close recebe a resposta."),
    ("Notification", "bell", "Notificação do sistema (Windows, Android), com ações e imagem."),
    ("TrayIcon", "pin", "Ícone na bandeja do sistema, com menu e clique."),
    ("Taskbar", "window", "Progresso, selo e botões no ícone do app na barra de tarefas."),
    ("EventRecorder", "record", "Grava tudo o que o usuário faz num .aylisrec (a IDE abre e mostra)."),
    ("Analytics", "grid", "Métricas de uso somadas entre as sessões (cliques, telas, erros)."),
    ("ClientDataSet", "list", "Tabela de dados em memória: campos, registros, filtros."),
    ("Field", "edit", "Um campo do ClientDataSet: nome, tipo, tamanho, rótulo, máscara."),
    ("Column", "grid", "Uma coluna do DataGrid: campo, título, largura, alinhamento, cores."),
    ("DataSource", "link", "Liga um ClientDataSet aos controles de dados (DataGrid, \"Ligar a campo…\"...)."),
    ("FloatAnimation", "sliders", "Anima uma propriedade numérica (posição, tamanho, opacidade)."),
    ("ColorAnimation", "sun", "Anima uma propriedade de cor."),
    ("MediaPlayer", "play", "Toca áudio e vídeo sem tela (sons, música)."),
    ("Sound", "bell", "Som curto (efeito, alerta): toca por cima de si mesmo, na memória."),
    ("AudioOutput", "list", "As saídas de áudio do sistema e o aviso quando uma entra ou sai."),
    ("AudioMixer", "sliders", "Mistura microfones, o som das saídas e players: volume, mudo, panorâmica; grava em WAV."),
];

/// O ícone do componente não visual (`None`: não é um dos conhecidos).
pub fn icon(class: &str) -> Option<&'static str> {
    INFO.iter().find(|(c, _, _)| *c == class).map(|(_, i, _)| *i)
}

/// A descrição curta do componente não visual.
pub fn description(class: &str) -> &'static str {
    INFO.iter().find(|(c, _, _)| *c == class).map_or("", |(_, _, d)| *d)
}

/// Os não visuais que a paleta oferece (o DataModule é uma raiz, como um form): classe e
/// categoria, na ordem da paleta.
pub fn classes(state: &Shared) -> Vec<(String, String)> {
    let st = state.borrow();
    let mut out: Vec<(String, String)> = st
        .doc
        .ui
        .registry()
        .iter()
        // Field, Column e MenuItem são subcomponentes: entram pela janela de campos, pela de
        // colunas e pelo editor de menu.
        .filter(|c| !matches!(c.name, "Form" | "DataModule" | "Field" | "Column" | "MenuItem") && !st.doc.class_is_visual(c.name))
        .map(|c| (c.name.to_string(), c.category.to_string()))
        .collect();
    out.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));
    out
}

/// A janela "Componentes não visuais": ícone, nome, categoria e descrição de cada um, com
/// busca; "Usar" escolhe o componente (como clicar na paleta).
pub fn open(ui: &mut Ui, state: &Shared, ids: &Ids) -> ComponentId {
    let d = new_dialog(ui, "Componentes não visuais", 640.0, 620.0);
    for child in ui.children(d.footer).to_vec() {
        if ui.class_name(child) == Some("Button") {
            ui.free(child);
        }
    }
    let close = ui.add(Button::new("Fechar"), d.footer);
    ui.control_mut(close).unwrap().set_align(Align::Right).set_width(110.0);
    let _ = ui.set_prop(close, "modal_result", "cancel");
    let search = ui.add(Edit::new(""), d.body);
    ui.control_mut(search).unwrap().set_align(Align::Top).set_height(32.0).set_margins([0.0, 0.0, 0.0, 10.0]);
    let _ = ui.set_prop(search, "placeholder", "Buscar componente");
    let list = ui.add(ScrollBox::default(), d.body);
    ui.control_mut(list).unwrap().set_align(Align::Client);
    let form = d.form;
    let classes = classes(state);
    let fill = {
        let (state, ids) = (state.clone(), *ids);
        std::rc::Rc::new(move |ui: &mut Ui, filter: &str| fill(ui, list, form, &classes, filter, &state, &ids))
    };
    fill(ui, "");
    {
        let fill = fill.clone();
        ui.on(search, "on_change", move |ui, _, args| fill(ui, &args.text().unwrap_or_default().to_lowercase()));
    }
    crate::lang::translate(ui);
    crate::dialogs::close_on_escape(ui, form);
    let _ = ui.show_modal(form, move |ui, _| {
        ui.free(form);
    });
    form
}

fn fill(ui: &mut Ui, list: ComponentId, dialog: ComponentId, classes: &[(String, String)], filter: &str, state: &Shared, ids: &Ids) {
    clear(ui, list);
    let mut category = String::new();
    for (class, cat) in classes {
        let text = description(class);
        if !filter.is_empty() && !class.to_lowercase().contains(filter) && !text.to_lowercase().contains(filter) {
            continue;
        }
        if *cat != category {
            caption(ui, list, cat);
            category = cat.clone();
        }
        let card = ui.add(Rectangle::new(0.0, 58.0), list);
        ui.control_mut(card).unwrap().set_align(Align::Top).set_margins([0.0, 0.0, 0.0, 6.0]).set_padding([10.0, 6.0, 8.0, 6.0]);
        let _ = ui.set_prop(card, "radius", 8);
        let _ = ui.set_prop(card, "fill", ui.theme().color("text").with_opacity(0.04));
        let _ = ui.set_prop(card, "stroke", ui.theme().color("border"));
        let glyph = ui.add(IconView::new(icon(class).unwrap_or("plug")), card);
        ui.control_mut(glyph).unwrap().set_align(Align::Left).set_width(28.0).set_margins([0.0, 8.0, 12.0, 8.0]);
        let primary = ui.theme().color("primary");
        let _ = ui.set_prop(glyph, "color", primary);
        let use_button = ui.add(Button::new("Usar"), card);
        ui.control_mut(use_button).unwrap().set_align(Align::Right).set_width(70.0).set_margins([8.0, 10.0, 0.0, 10.0]).set_style("flat");
        let texts = ui.add(Layout::default(), card);
        ui.control_mut(texts).unwrap().set_align(Align::Client);
        let name = ui.add(Label::new(class), texts);
        ui.control_mut(name).unwrap().set_align(Align::Top).set_height(22.0);
        let _ = ui.set_prop(name, "bold", true);
        let about = ui.add(Label::new(text), texts);
        ui.control_mut(about).unwrap().set_align(Align::Client);
        let _ = ui.set_prop(about, "wrap", true);
        muted(ui, about);
        let (class, state, ids) = (class.clone(), state.clone(), *ids);
        ui.on_click(use_button, move |ui, _| {
            choose(&state, &class);
            ui.set_modal_result(dialog, ModalResult::Ok);
            refresh(ui, &state, &ids);
        });
    }
}
