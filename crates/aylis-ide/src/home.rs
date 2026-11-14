//! A tela inicial: sem projeto aberto, no lugar do designer, os projetos (e forms) abertos
//! recentemente, o "Novo projeto" e o "Abrir projeto". Some ao abrir um projeto ou um form,
//! ou com o "Novo form solto"; volta ao fechar o projeto.

use std::cell::OnceCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use aylis_controls::{Button, Label, Layout, ListItem, ListView, Panel};
use aylis_core::{Align, ComponentId, EventArgs, Ui};

use crate::app::{Ids, refresh};
use crate::state::Shared;
use crate::widgets::muted;

thread_local! {
    /// O que some na tela inicial (registrado ao montar a IDE).
    static HIDDEN: std::cell::RefCell<Vec<ComponentId>> = const { std::cell::RefCell::new(Vec::new()) };
}

/// O que fica escondido enquanto a tela inicial aparece (painéis, barra de ferramentas e itens
/// de menu); volta ao abrir um projeto ou um form.
pub fn hide_on_home(ids: Vec<ComponentId>) {
    HIDDEN.with(|h| *h.borrow_mut() = ids);
}

/// Monta a tela inicial dentro de `parent` (escondida; [`sync`] decide). Devolve o painel e a
/// lista dos recentes.
pub fn build(ui: &mut Ui, state: &Shared, parent: ComponentId, final_ids: &Rc<OnceCell<Ids>>) -> (ComponentId, ComponentId) {
    let home = ui.add(Panel::new(0.0, 0.0), parent);
    ui.control_mut(home).unwrap().set_align(Align::Client).set_padding([48.0, 36.0, 48.0, 24.0]).set_visible(false);
    // O conteúdo numa coluna de largura máxima, no meio da janela (numa tela larga a lista
    // esticava de ponta a ponta; pedido do autor em 07/11).
    let column = ui.add(Layout::default(), home);
    ui.control_mut(column).unwrap().set_align(Align::Client);
    let _ = ui.set_prop(column, "max_width", 960.0);

    let title = ui.add(Label::new("Aylis IDE"), column);
    ui.control_mut(title).unwrap().set_align(Align::Top).set_height(44.0);
    let _ = ui.set_prop(title, "font_size", 28);
    let subtitle = ui.add(Label::new("Interfaces visuais em Rust e nas linguagens do Aylis, com designer"), column);
    ui.control_mut(subtitle).unwrap().set_align(Align::Top).set_height(26.0).set_margins([0.0, 0.0, 0.0, 18.0]);
    muted(ui, subtitle);

    let actions = ui.add(Layout::default(), column);
    ui.control_mut(actions).unwrap().set_align(Align::Top).set_height(40.0).set_margins([0.0, 0.0, 0.0, 22.0]);
    let button = |ui: &mut Ui, text: &str, icon: &str, width: f32, style: &str| {
        let b = ui.add(Button::new(text), actions);
        ui.control_mut(b).unwrap().set_align(Align::Left).set_width(width).set_margins([0.0, 0.0, 10.0, 0.0]).set_style(style);
        let _ = ui.set_prop(b, "icon", icon);
        b
    };
    let b_new = button(ui, "Novo projeto…", "plus", 170.0, "");
    let b_open = button(ui, "Abrir projeto…", "folder", 170.0, "flat");
    let b_form = button(ui, "Novo form solto", "file", 170.0, "flat");
    let b_options = button(ui, "Opções da IDE…", "settings", 170.0, "flat");

    let recent_title = ui.add(Label::new("RECENTES"), column);
    ui.control_mut(recent_title).unwrap().set_align(Align::Top).set_height(24.0);
    let _ = ui.set_prop(recent_title, "font_size", 11);
    muted(ui, recent_title);
    let list = ui.add(ListView::default(), column);
    ui.control_mut(list).unwrap().set_align(Align::Client).set_margins([0.0, 4.0, 0.0, 0.0]);
    let _ = ui.set_prop(list, "accessory", true);
    // Sem a moldura (nem a do foco ao clicar): a lista é parte da página.
    let _ = ui.set_prop(list, "style", "flat");

    // Os botões leem os ids da IDE no clique (ainda não estão prontos aqui).
    let ids = |cell: &Rc<OnceCell<Ids>>| cell.get().copied();
    {
        let (state, cell) = (state.clone(), final_ids.clone());
        ui.on_click(b_new, move |ui, _| {
            if let Some(ids) = ids(&cell) {
                crate::actions::new_project(ui, &state, &ids);
                refresh(ui, &state, &ids);
            }
        });
    }
    {
        let (state, cell) = (state.clone(), final_ids.clone());
        ui.on_click(b_open, move |ui, _| {
            if let Some(ids) = ids(&cell) {
                crate::actions::open_project_dialog(&state, &ids);
                refresh(ui, &state, &ids);
            }
        });
    }
    {
        let (state, cell) = (state.clone(), final_ids.clone());
        ui.on_click(b_form, move |ui, _| {
            state.borrow_mut().home = false;
            if let Some(ids) = ids(&cell) {
                refresh(ui, &state, &ids);
            }
        });
    }
    {
        let (state, cell) = (state.clone(), final_ids.clone());
        ui.on_click(b_options, move |ui, _| {
            if let Some(ids) = ids(&cell) {
                crate::options::open(ui, &state, &ids);
            }
        });
    }
    {
        let (state, cell) = (state.clone(), final_ids.clone());
        ui.on(list, "on_item_click", move |ui, _, args| {
            let EventArgs::Value(v) = args else { return };
            let Ok(index) = v.to_i64() else { return };
            let path = state.borrow().recent.get(index as usize).cloned();
            if let (Some(path), Some(ids)) = (path, ids(&cell)) {
                crate::actions::open_recent(ui, &state, &ids, &path);
                refresh(ui, &state, &ids);
            }
        });
    }
    (home, list)
}

/// A tela inicial aparece sem projeto, com só o form vazio de partida (sem arquivo e sem
/// mudanças) e enquanto o "Novo form solto" não foi pedido.
pub fn visible(state: &Shared) -> bool {
    let st = state.borrow();
    st.home && st.project.is_none() && st.tab_count() <= 1 && st.doc.path.is_none() && !st.doc.modified
}

/// O centro da IDE: a tela inicial, o designer (com as abas dos forms) ou o código embutido
/// (o "Design | Código" embaixo). Atualiza também os recentes da tela inicial.
pub fn sync(ui: &mut Ui, state: &Shared, ids: &Ids) {
    // O código só fica no centro enquanto o editor embutido tem abas.
    let code_open = crate::codewindow::embedded_has_tabs(ui);
    let code = code_open && state.borrow().code_view;
    let show = visible(state) && !code;
    // "Design | Código" só onde há o editor de código (a lite do navegador não tem).
    let code_tab = crate::lite::can(crate::lite::Capability::Code);
    // As abas ficam em cima do designer e do código (forms e código juntos).
    let views = [(ids.surface, !show && !code), (ids.tabs, !show), (ids.code_host, code), (ids.view_bar, !show && code_tab)];
    for (id, on) in views {
        if ui.control(id).is_some_and(|c| c.visible() != on) {
            ui.control_mut(id).unwrap().set_visible(on);
        }
    }
    // O botão da vista atual em destaque.
    for (id, on) in [(ids.view_design, !code), (ids.view_code, code)] {
        let style = if on { "" } else { "flat" };
        if ui.control(id).is_some_and(|c| c.style() != style) {
            let _ = ui.set_prop(id, "style", style);
        }
    }
    // Um fonte sem form (um módulo do app): o Design não tem para onde ir, como o F12 (I05).
    let design = !code || crate::actions::active_code_has_form(ui, state);
    if ui.control(ids.view_design).is_some_and(|c| c.enabled() != design) {
        ui.control_mut(ids.view_design).unwrap().set_enabled(design);
    }
    if ui.control(ids.home).is_some_and(|c| c.visible()) != show {
        ui.control_mut(ids.home).unwrap().set_visible(show);
        // Sem projeto não há o que editar: somem a barra de ferramentas, os painéis dos lados
        // e os menus que precisam de um projeto ou de um form ([`hide_on_home`]).
        for id in HIDDEN.with(|h| h.borrow().clone()) {
            let _ = ui.set_prop(id, "visible", !show);
        }
    }
    if !show {
        return;
    }
    let items: Vec<ListItem> = state.borrow().recent.iter().map(|p| item(p)).collect();
    if let Some(list) = ui.get_mut::<ListView>(ids.home_list) {
        if (0..list.count()).filter_map(|i| list.item(i).cloned()).collect::<Vec<_>>() != items {
            list.set_items(items);
        }
    }
}

/// Um recente na lista: o nome e, embaixo, o que é e onde está.
fn item(path: &Path) -> ListItem {
    let name = path.file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or_default();
    let kind = if path.extension().is_some_and(|e| e.eq_ignore_ascii_case("aylisproj")) { "Projeto" } else { "Form" };
    ListItem::new(name, format!("{kind} · {}", folder(path).display()))
}

fn folder(path: &Path) -> PathBuf {
    path.parent().map(Path::to_path_buf).unwrap_or_default()
}
