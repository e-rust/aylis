//! A janela da IDE, montada com os próprios controles do aylis.

use std::rc::Rc;

use aylis_controls::{ComboBox, Edit, Form, Label, Layout, MainMenu, MenuItem, Panel, PopupMenu, ScrollBox, SplitButton, Splitter};
use aylis_core::platform::LocaleInfo;
use aylis_core::{Align, ComponentId, EventArgs, PointF, Theme, Ui};

use crate::actions;
use crate::document::AlignOp;
use crate::inspector;
use crate::panels;
use crate::state::Shared;
use crate::surface::DesignSurface;
use crate::widgets::{caption, label, muted, tool};

/// Ids dos painéis e menus que são atualizados quando algo muda.
#[derive(Clone, Copy)]
pub struct Ids {
    pub form: ComponentId,
    pub surface: ComponentId,
    pub palette: ComponentId,
    /// As abas da paleta (Windows, Apple, Android, aylis).
    pub palette_tabs: ComponentId,
    pub structure: ComponentId,
    pub project: ComponentId,
    pub inspector: ComponentId,
    pub inspector_title: ComponentId,
    /// A busca do Object Inspector (limpa ao trocar de componente).
    pub inspector_search: ComponentId,
    /// O combo do alvo (ao lado do ▶ Executar): só as saídas da linguagem do projeto.
    pub target_combo: ComponentId,
    /// Os aparelhos Android (só aparece com a saída Android; veja `devices.rs`).
    pub device_combo: ComponentId,
    pub tab_props: ComponentId,
    pub tab_events: ComponentId,
    pub status: ComponentId,
    /// O progresso estimado da compilação (pelo tempo da última no mesmo modo).
    pub build_progress: ComponentId,
    /// A tela inicial (os recentes) e a lista dela.
    pub home: ComponentId,
    pub home_list: ComponentId,
    /// O editor de código embutido e os botões "Design | Código" embaixo do centro.
    pub code_host: ComponentId,
    pub view_bar: ComponentId,
    pub view_design: ComponentId,
    pub view_code: ComponentId,
    /// O relógio da barra de status (só liga durante um trabalho).
    pub clock: ComponentId,
    pub doc_title: ComponentId,
    pub tabs: ComponentId,
    pub messages: ComponentId,
    pub messages_panel: ComponentId,
    pub messages_splitter: ComponentId,
    pub theme_combo: ComponentId,
    /// A vista por largura em edição no designer (Desktop, Tablet, Celular).
    pub view_combo: ComponentId,
    pub mi_lock: ComponentId,
    pub mi_hidden: ComponentId,
    pub mi_trace: ComponentId,
    pub mi_recent: ComponentId,
    pub mi_record: ComponentId,
    pub design_menu: ComponentId,
}

pub struct Ide {
    pub ui: Ui,
    pub form: ComponentId,
    pub state: Shared,
    pub ids: Rc<Ids>,
}

type Action = fn(&mut Ui, &Shared, &Ids);

/// Monta a IDE para o estado.
pub fn build(state: Shared) -> Ide {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    // As linguagens e os temas do VS Code no editor (embutidos e importados).
    crate::grammars::register();
    // Os ícones das plataformas na paleta ("platform:android"...).
    let _ = ui.register_icon_pack("platform", include_str!("../resources/platforms.txt"));
    ui.set_theme(Theme::dark());
    ui.services_mut().locale = Some(Box::new(LocaleInfo::pt_br()));
    // A roda como nos navegadores e editores: 100 pontos por dente, deslizando (com 40 e
    // saltando, as listas pareciam travar).
    ui.set_scroll_options(aylis_core::ScrollOptions { wheel_step: 100.0, smooth: true, smooth_ms: 160.0 });

    let form = ui.add_root(Form::new("Aylis IDE", 1440.0, 900.0));
    let _ = ui.set_name(form, "frmIde");

    // ----- menu (os itens são ligados mais abaixo) -----
    let menu = ui.add(MainMenu::default(), form);

    // ----- barra de ferramentas -----
    let toolbar = ui.add(Panel::new(0.0, 44.0), form);
    ui.control_mut(toolbar).unwrap().set_align(Align::Top).set_padding(6.0);
    let b_new = tool(&mut ui, toolbar, "Novo projeto", 112.0, "flat");
    let b_open = tool(&mut ui, toolbar, "Abrir…", 72.0, "flat");
    let b_save = tool(&mut ui, toolbar, "Salvar", 70.0, "flat");
    let b_save_all = tool(&mut ui, toolbar, "Salvar tudo", 100.0, "flat");
    let b_undo = tool(&mut ui, toolbar, "Desfazer", 84.0, "flat");
    let b_redo = tool(&mut ui, toolbar, "Refazer", 76.0, "flat");
    let b_form = tool(&mut ui, toolbar, "+ Form", 74.0, "flat");
    let b_code = tool(&mut ui, toolbar, "Código (F12)", 112.0, "flat");
    // As dicas dos botões de texto, com o atalho (pedido do autor na conferência de 08/11).
    for (b, hint) in [
        (b_new, "Novo projeto (Ctrl+Shift+N)"),
        (b_open, "Abrir um projeto (Ctrl+Shift+O) ou, com um aberto, um form (Ctrl+O)"),
        (b_save, "Salvar (Ctrl+S)"),
        (b_save_all, "Salvar tudo (Ctrl+Shift+S)"),
        (b_undo, "Desfazer (Ctrl+Z)"),
        (b_redo, "Refazer (Ctrl+Y)"),
        (b_form, "Novo form (Ctrl+N)"),
        (b_code, "O código do form (F12)"),
    ] {
        let _ = ui.set_prop(b, "hint", hint);
    }
    // Executar: o clique roda sem depurar (compila só se os fontes mudaram); a seta tem
    // depurar e compilar em release.
    let b_run = ui.add(SplitButton::default(), toolbar);
    let _ = ui.set_prop(b_run, "text", "Executar");
    let _ = ui.set_prop(b_run, "icon", "play");
    let _ = ui.set_prop(b_run, "hint", "Executar sem depurar (Ctrl+Shift+F9)");
    ui.control_mut(b_run).unwrap().set_align(Align::Left).set_width(150.0).set_margins([0.0, 0.0, 4.0, 0.0]);
    let run_popup = ui.add(PopupMenu::default(), form);
    let mi_run_debug = ui.add(MenuItem::new("Depurar"), run_popup);
    let mi_run_release = ui.add(MenuItem::new("Compilar e executar em release"), run_popup);
    let _ = ui.set_prop(b_run, "popup_menu", aylis_core::PropValue::Component(Some(run_popup)));
    let target_names: Vec<&str> = run_targets(&state).iter().map(|t| t.name()).collect();
    let target_combo = ui.add(ComboBox::new(&target_names), toolbar);
    ui.control_mut(target_combo).unwrap().set_align(Align::Left).set_width(150.0).set_margins([4.0, 0.0, 0.0, 0.0]);
    let _ = ui.set_prop(target_combo, "item_index", 0_i64);
    let device_combo = ui.add(ComboBox::new(&[]), toolbar);
    ui.control_mut(device_combo).unwrap().set_align(Align::Left).set_width(240.0).set_margins([4.0, 0.0, 0.0, 0.0]).set_visible(false);
    let _ = ui.set_prop(device_combo, "hint", "O aparelho do Executar (plugue o cabo e ele aparece aqui)");
    let b_publish = tool(&mut ui, toolbar, "Publicar…", 90.0, "flat");
    ui.control_mut(b_publish).unwrap().set_margins([8.0, 0.0, 4.0, 0.0]);
    let _ = ui.set_prop(b_publish, "hint", "Publicar o app para a plataforma escolhida (Ctrl+Alt+F9)");
    let theme_combo = ui.add(ComboBox::new(&["Form claro", "Form escuro"]), toolbar);
    ui.control_mut(theme_combo).unwrap().set_align(Align::Left).set_width(130.0).set_margins([8.0, 0.0, 0.0, 0.0]);
    let _ = ui.set_prop(theme_combo, "item_index", i64::from(state.borrow().doc.dark));
    // As vistas do form, o editor delas e, depois, os aparelhos (a moldura, o tamanho e a
    // área segura de cada um).
    let view_items: Vec<String> = view_entries(&state.borrow().doc).into_iter().map(|(_, text)| text).collect();
    let view_items: Vec<&str> = view_items.iter().map(String::as_str).collect();
    let view_combo = ui.add(ComboBox::new(&view_items), toolbar);
    ui.control_mut(view_combo).unwrap().set_align(Align::Left).set_width(140.0).set_margins([8.0, 0.0, 0.0, 0.0]);
    // A lite do navegador: sem projeto no disco, código, executar nem publicar na barra.
    if !crate::lite::can(crate::lite::Capability::Disk) {
        for id in [b_new, b_open, b_save_all, b_form, b_code, b_run, target_combo, b_publish] {
            if let Some(cb) = ui.control_mut(id) {
                cb.set_visible(false);
            }
        }
    }
    let _ = ui.set_prop(view_combo, "item_index", 0_i64);
    let _ = ui.set_prop(view_combo, "hint", "A vista do form (o que mudar nela vale só nela), o editor das vistas ou, em Telas…, um aparelho, com a moldura e a área segura dele");
    let doc_title = label(&mut ui, toolbar, "", Align::Client, 0.0);
    let _ = ui.set_prop(doc_title, "h_align", "trailing");
    muted(&mut ui, doc_title);
    // O aviso de versão nova (escondido), logo abaixo da barra.
    crate::updates::build_banner(&mut ui, form);

    // ----- status (à direita, o progresso estimado da compilação) -----
    let status_bar = ui.add(Layout::default(), form);
    ui.control_mut(status_bar).unwrap().set_align(Align::Bottom).set_height(24.0);
    let build_progress = ui.add(aylis_controls::ProgressBar::default(), status_bar);
    ui.control_mut(build_progress).unwrap().set_align(Align::Right).set_width(220.0).set_margins([0.0, 8.0, 12.0, 8.0]).set_visible(false);
    let status = label(&mut ui, status_bar, "", Align::Client, 0.0);
    ui.control_mut(status).unwrap().set_margins([10.0, 0.0, 10.0, 0.0]);
    let _ = ui.set_prop(status, "font_size", 12);

    // ----- mensagens (saída do Executar) -----
    let messages_panel = ui.add(Panel::new(0.0, 190.0), form);
    ui.control_mut(messages_panel).unwrap().set_align(Align::Bottom).set_padding(6.0).set_visible(false);
    let mbar = ui.add(Layout::default(), messages_panel);
    // Um respiro entre o título e as linhas (estavam coladas; pedido do autor em 07/11).
    ui.control_mut(mbar).unwrap().set_align(Align::Top).set_height(28.0).set_margins([0.0, 0.0, 0.0, 8.0]);
    let m_close = tool(&mut ui, mbar, "✕", 32.0, "flat");
    ui.control_mut(m_close).unwrap().set_align(Align::Right);
    let m_stop = tool(&mut ui, mbar, "■ Parar", 80.0, "flat");
    ui.control_mut(m_stop).unwrap().set_align(Align::Right);
    let m_clear = tool(&mut ui, mbar, "Limpar", 72.0, "flat");
    ui.control_mut(m_clear).unwrap().set_align(Align::Right);
    let m_copy = tool(&mut ui, mbar, "Copiar tudo", 100.0, "flat");
    ui.control_mut(m_copy).unwrap().set_align(Align::Right);
    let mt = label(&mut ui, mbar, "MENSAGENS", Align::Client, 0.0);
    muted(&mut ui, mt);
    let _ = ui.set_prop(mt, "font_size", 11);
    // As linhas selecionáveis e copiáveis; o clique direito abre "Copiar" e "Copiar tudo".
    let messages = ui.add(crate::messages::MessageList::new(state.clone()), messages_panel);
    ui.control_mut(messages).unwrap().set_align(Align::Client);
    let messages_menu = ui.add(PopupMenu::default(), form);
    let mi_copy = ui.add(MenuItem::new("Copiar"), messages_menu);
    let _ = ui.set_prop(mi_copy, "shortcut", "Ctrl+C");
    let mi_copy_all = ui.add(MenuItem::new("Copiar tudo"), messages_menu);
    if let Some(list) = ui.get_mut::<crate::messages::MessageList>(messages) {
        list.set_menu(messages_menu);
    }
    let copy_messages = move |ui: &mut Ui, all: bool| {
        let text = ui.get::<crate::messages::MessageList>(messages).map(|l| if all { l.all_text() } else { l.selected_text() });
        if let Some(text) = text {
            crate::messages::copy_text(ui, &text);
        }
    };
    ui.on_click(mi_copy, move |ui, _| copy_messages(ui, false));
    ui.on_click(mi_copy_all, move |ui, _| copy_messages(ui, true));
    ui.on_click(m_copy, move |ui, _| copy_messages(ui, true));
    let messages_splitter = ui.add(Splitter::new(Align::Bottom), form);
    ui.control_mut(messages_splitter).unwrap().set_visible(false);

    // Estrutura e Object Inspector à esquerda; projeto e paleta à direita.
    // ----- esquerda: estrutura e Object Inspector -----
    let left = ui.add(Panel::new(350.0, 0.0), form);
    ui.control_mut(left).unwrap().set_align(Align::Left).set_padding(8.0);
    caption(&mut ui, left, "ESTRUTURA");
    let structure = ui.add(ScrollBox::default(), left);
    ui.control_mut(structure).unwrap().set_align(Align::Top).set_height(220.0);
    ui.add(Splitter::new(Align::Top), left);
    let inspector_title = label(&mut ui, left, "", Align::Top, 30.0);
    let _ = ui.set_prop(inspector_title, "font_size", 15);
    let tabs_bar = ui.add(Layout::default(), left);
    ui.control_mut(tabs_bar).unwrap().set_align(Align::Top).set_height(32.0).set_margins([0.0, 0.0, 0.0, 6.0]);
    let tab_props = tool(&mut ui, tabs_bar, "Propriedades", 120.0, "");
    let tab_events = tool(&mut ui, tabs_bar, "Eventos", 90.0, "flat");
    let inspector_search = ui.add(Edit::new(""), left);
    ui.control_mut(inspector_search).unwrap().set_align(Align::Top).set_height(30.0).set_margins([0.0, 0.0, 0.0, 6.0]);
    let _ = ui.set_prop(inspector_search, "placeholder", "Buscar propriedade");
    let inspector = ui.add(ScrollBox::default(), left);
    ui.control_mut(inspector).unwrap().set_align(Align::Client);
    let left_splitter = ui.add(Splitter::new(Align::Left), form);

    // ----- direita: projeto e paleta -----
    let right = ui.add(Panel::new(260.0, 0.0), form);
    ui.control_mut(right).unwrap().set_align(Align::Right).set_padding(8.0);
    caption(&mut ui, right, "PROJETO");
    let project = ui.add(ScrollBox::default(), right);
    ui.control_mut(project).unwrap().set_align(Align::Top).set_height(220.0);
    ui.add(Splitter::new(Align::Top), right);
    caption(&mut ui, right, "PALETA");
    // A busca e, ao lado, a galeria (os visuais desenhados) e a lista dos não visuais.
    let palette_bar = ui.add(Layout::default(), right);
    ui.control_mut(palette_bar).unwrap().set_align(Align::Top).set_height(30.0).set_margins([0.0, 0.0, 0.0, 6.0]);
    let icon_button = |ui: &mut Ui, icon: &str, hint: &str| {
        let b = ui.add(aylis_controls::Button::new(""), palette_bar);
        ui.control_mut(b).unwrap().set_align(Align::Right).set_width(32.0).set_margins([4.0, 0.0, 0.0, 0.0]).set_style("flat");
        let _ = ui.set_prop(b, "icon", icon);
        let _ = ui.set_prop(b, "icon_size", 18.0);
        let _ = ui.set_prop(b, "hint", hint);
        b
    };
    let nonvisual_button = icon_button(&mut ui, "plug", "Componentes não visuais (Timer, diálogos, dados...)");
    let gallery_button = icon_button(&mut ui, "grid", "Galeria: os componentes visuais desenhados");
    let palette_search = ui.add(Edit::new(""), palette_bar);
    ui.control_mut(palette_search).unwrap().set_align(Align::Client);
    let _ = ui.set_prop(palette_search, "placeholder", "Buscar componente");
    let palette_tabs = ui.add(Layout::default(), right);
    ui.control_mut(palette_tabs).unwrap().set_align(Align::Top).set_height(28.0).set_margins([0.0, 0.0, 0.0, 6.0]);
    let palette = ui.add(ScrollBox::default(), right);
    ui.control_mut(palette).unwrap().set_align(Align::Client);
    let right_splitter = ui.add(Splitter::new(Align::Right), form);

    // ----- centro: abas e o designer -----
    let center = ui.add(Layout::default(), form);
    ui.control_mut(center).unwrap().set_align(Align::Client);
    let tabs = ui.add(Layout::default(), center);
    // As abas do navegador (`TabStrip`, montada em `panels::tabs`): a escolhida se junta ao
    // designer embaixo.
    ui.control_mut(tabs).unwrap().set_align(Align::Top).set_height(38.0);
    // Embaixo: alternar entre o form (Design) e o código embutido.
    let view_bar = ui.add(Layout::default(), center);
    ui.control_mut(view_bar).unwrap().set_align(Align::Bottom).set_height(32.0).set_padding([6.0, 2.0, 6.0, 2.0]).set_visible(false);
    let view_design = tool(&mut ui, view_bar, "Design", 90.0, "");
    let view_code = tool(&mut ui, view_bar, "Código", 90.0, "flat");
    let _ = ui.set_prop(view_design, "hint", "O form no designer");
    let _ = ui.set_prop(view_code, "hint", "O código no editor (F12 no designer abre o do form)");
    let surface = ui.add(DesignSurface::new(state.clone()), center);
    // O designer mostra o form como ele é, mesmo com a IDE espelhada (árabe).
    ui.control_mut(surface).unwrap().set_align(Align::Client).set_keep_left_to_right(true);
    // O editor de código embutido (no lugar do designer quando "Código" está escolhido).
    let code_host = ui.add(Layout::default(), center);
    ui.control_mut(code_host).unwrap().set_align(Align::Client).set_visible(false);
    crate::codewindow::set_host(Some(code_host));

    // Menu de contexto do designer (os itens chamam os mesmos comandos do menu Editar).
    let design_menu = ui.add(PopupMenu::default(), form);

    let mut ids = Ids {
        form,
        surface,
        palette,
        palette_tabs,
        structure,
        project,
        inspector,
        inspector_title,
        inspector_search,
        target_combo,
        device_combo,
        tab_props,
        tab_events,
        status,
        build_progress,
        home: form,
        home_list: form,
        code_host,
        view_bar,
        view_design,
        view_code,
        clock: form,
        doc_title,
        tabs,
        messages,
        messages_panel,
        messages_splitter,
        theme_combo,
        view_combo,
        mi_lock: form,
        mi_hidden: form,
        mi_trace: form,
        mi_recent: form,
        mi_record: form,
        design_menu,
    };

    // ----- menus -----
    // Os ids ficam prontos só no fim da montagem (o "Recentes", o "Travar controles"...
    // são criados depois dos primeiros itens). Cada item lê os ids finais no clique: com a
    // cópia de quando foi criado, o "Novo projeto" (Ctrl+Shift+N) via `mi_recent` ainda
    // apontando para o form, e o refresh apagava a IDE inteira para pôr os recentes.
    let final_ids: Rc<std::cell::OnceCell<Ids>> = Rc::default();
    // O painel ESTRUTURA pelo teclado (as setas, com o foco nele; pedido do autor em 08/11).
    let _ = ui.set_prop(structure, "can_focus", true);
    {
        let (state, final_ids) = (state.clone(), final_ids.clone());
        ui.on(structure, "on_key_down", move |ui, _, args| {
            let Some(ids) = final_ids.get().copied() else { return };
            if let aylis_core::EventArgs::Key(k) = args {
                if !k.modifiers.ctrl && !k.modifiers.alt && !k.modifiers.shift {
                    panels::structure_arrow(ui, &state, &ids, k.key);
                }
            }
        });
    }
    let add_item =|ui: &mut Ui, parent: ComponentId, text: &str, shortcut: &str, action: Option<Action>, _ids: &Ids| {
        let item = ui.add(MenuItem::new(text).with_shortcut(shortcut), parent);
        // O que a IDE não faz onde está rodando (a lite do navegador não compila nem abre o
        // disco) some do menu, com o atalho junto.
        if crate::lite::needs(text).is_some_and(|need| !crate::lite::can(need)) {
            let _ = ui.set_prop(item, "visible", false);
            let _ = ui.set_prop(item, "enabled", false);
            return item;
        }
        if let Some(action) = action {
            let (state, final_ids) = (state.clone(), final_ids.clone());
            ui.on_click(item, move |ui, _| {
                let Some(ids) = final_ids.get().copied() else { return };
                action(ui, &state, &ids);
                refresh(ui, &state, &ids);
            });
        }
        item
    };
    let separator = |ui: &mut Ui, parent: ComponentId| {
        ui.add(MenuItem::new("-"), parent);
    };

    let file_menu = ui.add(MenuItem::new("Arquivo"), menu);
    add_item(&mut ui, file_menu, "Novo projeto…", "Ctrl+Shift+N", Some(actions::new_project), &ids);
    add_item(&mut ui, file_menu, "Abrir projeto…", "Ctrl+Shift+O", Some(|_, s, i| actions::open_project_dialog(s, i)), &ids);
    add_item(&mut ui, file_menu, "Abrir form…", "Ctrl+O", Some(|_, s, _| actions::open_form_dialog(s)), &ids);
    // Outra IDE (para dois projetos lado a lado): abre na tela inicial.
    add_item(&mut ui, file_menu, "Nova janela", "Ctrl+Shift+W", Some(|_, s, _| {
        let started = std::env::current_exe().and_then(|exe| std::process::Command::new(exe).arg("--new-window").spawn());
        if let Err(e) = started {
            s.borrow_mut().status = format!("⚠ não consegui abrir outra janela: {e}");
        }
    }), &ids);
    add_item(&mut ui, file_menu, "Abrir gravação…", "", Some(|_, s, _| crate::recordings::open_dialog(s)), &ids);
    let recent = add_item(&mut ui, file_menu, "Recentes", "", None, &ids);
    ids.mi_recent = recent;
    separator(&mut ui, file_menu);
    add_item(&mut ui, file_menu, "Salvar", "Ctrl+S", Some(actions::save_current), &ids);
    // A lite do navegador: o projeto sai em ZIP (lá não há pasta para salvar).
    if !crate::lite::can(crate::lite::Capability::Disk) {
        add_item(&mut ui, file_menu, "Abrir projeto (ZIP)…", "", Some(crate::lite::open_zip), &ids);
        add_item(&mut ui, file_menu, "Baixar projeto (ZIP)", "", Some(crate::lite::download_project), &ids);
    }
    add_item(&mut ui, file_menu, "Salvar como…", "", Some(|_, s, _| actions::save(s, true)), &ids);
    add_item(&mut ui, file_menu, "Salvar tudo", "Ctrl+Shift+S", Some(|ui, s, _| {
        crate::inspector::commit_pending(ui);
        crate::codewindow::save_embedded(ui, true);
        actions::save_all(s);
    }), &ids);
    separator(&mut ui, file_menu);
    add_item(&mut ui, file_menu, "Fechar aba", "Ctrl+W", Some(|ui, s, i| {
        let active = s.borrow().active;
        actions::close_tab(ui, s, i, active);
    }), &ids);
    add_item(&mut ui, file_menu, "Fechar projeto", "", Some(|ui, s, _| {
        actions::close_project(ui, s);
    }), &ids);
    separator(&mut ui, file_menu);
    add_item(&mut ui, file_menu, "Sair", "Ctrl+Q", Some(|ui, s, i| actions::quit(ui, s, i)), &ids);

    let edit_menu = ui.add(MenuItem::new("Editar"), menu);
    add_item(&mut ui, edit_menu, "Desfazer", "Ctrl+Z", Some(|ui, s, _| actions::edit(ui, s, "undo")), &ids);
    add_item(&mut ui, edit_menu, "Refazer", "Ctrl+Y", Some(|ui, s, _| actions::edit(ui, s, "redo")), &ids);
    separator(&mut ui, edit_menu);
    add_item(&mut ui, edit_menu, "Recortar", "Ctrl+X", Some(|ui, s, _| actions::edit(ui, s, "cut")), &ids);
    add_item(&mut ui, edit_menu, "Copiar", "Ctrl+C", Some(|ui, s, _| actions::edit(ui, s, "copy")), &ids);
    add_item(&mut ui, edit_menu, "Colar", "Ctrl+V", Some(|ui, s, _| actions::edit(ui, s, "paste")), &ids);
    add_item(&mut ui, edit_menu, "Excluir", "Delete", Some(|ui, s, _| actions::edit(ui, s, "delete")), &ids);
    add_item(&mut ui, edit_menu, "Selecionar tudo", "Ctrl+A", Some(|ui, s, _| actions::edit(ui, s, "select_all")), &ids);
    add_item(&mut ui, edit_menu, "Pesquisar no projeto…", "Ctrl+Shift+F", Some(|ui, s, i| crate::projectsearch::open(ui, s, i, "")), &ids);
    add_item(&mut ui, edit_menu, "Onde é usado…", "", Some(|ui, s, i| actions::show_usages(ui, s, i)), &ids);
    add_item(&mut ui, edit_menu, "Selecionar o pai", "", Some(|ui, s, _| actions::edit(ui, s, "parent")), &ids);
    separator(&mut ui, edit_menu);
    let align_menu = add_item(&mut ui, edit_menu, "Alinhar", "", None, &ids);
    let align_items: [(&str, Action); 11] = [
        ("Pela esquerda", |_, s, _| actions::align(s, AlignOp::Left)),
        ("Pelos centros (horizontal)", |_, s, _| actions::align(s, AlignOp::CenterH)),
        ("Pela direita", |_, s, _| actions::align(s, AlignOp::Right)),
        ("Pelo topo", |_, s, _| actions::align(s, AlignOp::Top)),
        ("Pelos centros (vertical)", |_, s, _| actions::align(s, AlignOp::CenterV)),
        ("Pela base", |_, s, _| actions::align(s, AlignOp::Bottom)),
        ("Distribuir na horizontal", |_, s, _| actions::align(s, AlignOp::DistributeH)),
        ("Distribuir na vertical", |_, s, _| actions::align(s, AlignOp::DistributeV)),
        ("Mesma largura", |_, s, _| actions::align(s, AlignOp::SameWidth)),
        ("Mesma altura", |_, s, _| actions::align(s, AlignOp::SameHeight)),
        ("Mesmo tamanho", |_, s, _| actions::align(s, AlignOp::SameSize)),
    ];
    for (i, (text, action)) in align_items.into_iter().enumerate() {
        if i == 6 || i == 8 {
            separator(&mut ui, align_menu);
        }
        add_item(&mut ui, align_menu, text, "", Some(action), &ids);
    }
    let order_menu = add_item(&mut ui, edit_menu, "Ordem", "", None, &ids);
    add_item(&mut ui, order_menu, "Trazer para frente", "", Some(|ui, s, _| actions::edit(ui, s, "front")), &ids);
    add_item(&mut ui, order_menu, "Enviar para trás", "", Some(|ui, s, _| actions::edit(ui, s, "back")), &ids);
    separator(&mut ui, edit_menu);
    ids.mi_lock = add_item(&mut ui, edit_menu, "Travar controles", "", Some(|ui, s, _| actions::edit(ui, s, "lock")), &ids);
    add_item(&mut ui, edit_menu, "Ordem de tabulação…", "", Some(|ui, s, i| crate::taborder::open(ui, s, i)), &ids);

    let view_menu = ui.add(MenuItem::new("Exibir"), menu);
    add_item(&mut ui, view_menu, "Aumentar zoom", "Ctrl+=", Some(|_, s, _| actions::zoom(s, Some(1.25))), &ids);
    add_item(&mut ui, view_menu, "Diminuir zoom", "Ctrl+-", Some(|_, s, _| actions::zoom(s, Some(0.8))), &ids);
    add_item(&mut ui, view_menu, "Zoom 100%", "Ctrl+0", Some(|_, s, _| actions::zoom(s, None)), &ids);
    separator(&mut ui, view_menu);
    add_item(&mut ui, view_menu, "Galeria de componentes…", "", Some(|ui, s, i| {
        crate::gallery::open(ui, s, i);
    }), &ids);
    add_item(&mut ui, view_menu, "Componentes não visuais…", "", Some(|ui, s, i| {
        crate::nonvisual::open(ui, s, i);
    }), &ids);
    // Os escondidos (popups, avisos, o que só aparece em certas situações): por padrão só
    // o selecionado aparece, para não ficar tudo um por cima do outro.
    ids.mi_hidden = add_item(&mut ui, view_menu, "Mostrar componentes escondidos", "", Some(|_, s, _| {
        let mut st = s.borrow_mut();
        st.show_hidden = !st.show_hidden;
    }), &ids);
    add_item(&mut ui, view_menu, "Propriedades", "F11", Some(|_, s, _| s.borrow_mut().events_tab = false), &ids);
    add_item(&mut ui, view_menu, "Eventos", "Shift+F11", Some(|_, s, _| s.borrow_mut().events_tab = true), &ids);
    add_item(&mut ui, view_menu, "Mensagens", "Ctrl+M", Some(|_, s, _| {
        let mut st = s.borrow_mut();
        st.show_messages = !st.show_messages;
    }), &ids);
    separator(&mut ui, view_menu);
    // Idioma: o do sistema ou um dos da IDE (cada um no próprio idioma).
    let language_menu = ui.add(MenuItem::new("Idioma"), view_menu);
    let choices: Vec<(&'static str, &'static str)> = std::iter::once(("", "Idioma do sistema")).chain(crate::lang::LANGUAGES.iter().copied()).collect();
    // O escolhido fica marcado (em inglês, "English" não ficava; conferência de 08/11).
    let language_items: std::rc::Rc<std::cell::RefCell<Vec<(&'static str, ComponentId)>>> = Default::default();
    let chosen = state.borrow().language.clone();
    for (code, name) in choices {
        let item = ui.add(MenuItem::new(name), language_menu);
        let _ = ui.set_prop(item, "checked", chosen == code);
        language_items.borrow_mut().push((code, item));
        let (state, final_ids, language_items) = (state.clone(), final_ids.clone(), language_items.clone());
        ui.on_click(item, move |ui, _| {
            let Some(ids) = final_ids.get().copied() else { return };
            actions::set_language(ui, &state, code);
            for (c, i) in language_items.borrow().iter() {
                let _ = ui.set_prop(*i, "checked", *c == code);
            }
            refresh(ui, &state, &ids);
        });
    }

    let project_menu = ui.add(MenuItem::new("Projeto"), menu);
    add_item(&mut ui, project_menu, "Novo form…", "Ctrl+N", Some(actions::new_form), &ids);
    // Os que só fazem sentido com um projeto aberto (o refresh desabilita sem projeto).
    let mut needs_project = vec![b_publish];
    needs_project.push(add_item(&mut ui, project_menu, "Novo DataModule…", "", Some(actions::new_data_module), &ids));
    needs_project.push(add_item(&mut ui, project_menu, "Nova variante do form…", "", Some(actions::new_variant), &ids));
    needs_project.push(add_item(&mut ui, project_menu, "Adicionar form existente…", "", Some(|_, s, i| actions::add_existing_form(s, i)), &ids));
    needs_project.push(add_item(&mut ui, project_menu, "Remover form do projeto…", "", Some(|ui, s, i| {
        let form = s.borrow().doc.form_name();
        actions::remove_form(ui, s, i, &form);
    }), &ids));
    needs_project.push(add_item(&mut ui, project_menu, "Definir como principal", "", Some(|_, s, _| {
        let form = s.borrow().doc.form_name();
        actions::set_main_form(s, &form);
    }), &ids));
    separator(&mut ui, project_menu);
    add_item(&mut ui, project_menu, "Gerar código", "", Some(|_, s, _| actions::generate_code_command(s)), &ids);
    add_item(&mut ui, project_menu, "Abrir código do form", "F12", Some(|ui, s, _| actions::open_code(ui, s, None)), &ids);
    // Os fontes do projeto pelo nome.
    add_item(&mut ui, project_menu, "Abrir fonte…", "Ctrl+F12", Some(|ui, _, _| crate::codewindow::open_source_dialog(ui)), &ids);
    needs_project.push(add_item(&mut ui, project_menu, "Novo fonte…", "", Some(actions::new_source_dialog), &ids));
    needs_project.push(add_item(&mut ui, project_menu, "Pendências entre linguagens…", "", Some(|ui, s, _| {
        crate::code_sync::open(ui, s, false);
    }), &ids));
    // Os handlers que nenhum uso ou teste do app em execução disparou (4b).
    needs_project.push(add_item(&mut ui, project_menu, "Cobertura de handlers…", "", Some(crate::coverage::open), &ids));
    separator(&mut ui, project_menu);
    // O que se escolhe enquanto desenha a tela (ícones e animações para os componentes).
    needs_project.push(add_item(&mut ui, project_menu, "Ícones…", "Ctrl+Alt+I", Some(actions::project_icons), &ids));
    add_item(&mut ui, project_menu, "Animações…", "", Some(actions::project_animations), &ids);
    separator(&mut ui, project_menu);
    // O que é do projeto e muda pouco (geral, aparência, fontes, ícone do app, Android,
    // traduções, DataModules usados, confiança) fica junto, como as "Project Options".
    needs_project.push(add_item(&mut ui, project_menu, "Opções do projeto…", "Ctrl+Shift+F11", Some(crate::projectsettings::open), &ids));
    needs_project.push(add_item(&mut ui, project_menu, "Abrir pasta do projeto", "", Some(|_, s, _| actions::open_project_folder(s)), &ids));

    let run_menu = ui.add(MenuItem::new("Executar"), menu);
    // Com ponto de parada, F9 executa no depurador; depurando, continua.
    add_item(&mut ui, run_menu, "Executar", "F9", Some(|ui, s, _| crate::debugger::run(ui, s)), &ids);
    // Roda sem o depurador mesmo com pontos de parada.
    add_item(&mut ui, run_menu, "Executar sem depurar", "Ctrl+Shift+F9", Some(|ui, s, _| actions::run_project(ui, s)), &ids);
    add_item(&mut ui, run_menu, "Compilar e executar em release", "", Some(|ui, s, _| actions::run_release(ui, s)), &ids);
    add_item(&mut ui, run_menu, "Compilar", "Ctrl+F9", Some(|ui, s, _| {
        crate::codewindow::save_all(ui);
        crate::jobs::build(s, false)
    }), &ids);
    add_item(&mut ui, run_menu, "Compilar em release", "", Some(|ui, s, _| {
        crate::codewindow::save_all(ui);
        crate::jobs::build(s, true)
    }), &ids);
    add_item(&mut ui, run_menu, "Prévia do form", "Shift+F9", Some(|ui, s, _| {
        // Sem disco (a lite do navegador), a prévia abre por cima da IDE.
        if crate::lite::can(crate::lite::Capability::Disk) {
            actions::run_preview(s);
        } else {
            crate::lite::preview(ui, s);
        }
    }), &ids);
    add_item(&mut ui, run_menu, "Parar", "Ctrl+F2", Some(|ui, s, _| {
        if s.borrow().debugger.active() {
            crate::debugger::command(ui, s, crate::debugger::Cmd::Stop);
        } else {
            actions::stop(s);
        }
    }), &ids);
    separator(&mut ui, run_menu);
    needs_project.push(add_item(&mut ui, run_menu, "Depurar na IDE", "Ctrl+F5", Some(|ui, s, _| crate::debugger::start(ui, s)), &ids));
    add_item(&mut ui, run_menu, "Próxima linha", "F8", Some(|ui, s, _| crate::debugger::command(ui, s, crate::debugger::Cmd::StepOver)), &ids);
    add_item(&mut ui, run_menu, "Entrar na função", "F7", Some(|ui, s, _| crate::debugger::command(ui, s, crate::debugger::Cmd::StepIn)), &ids);
    add_item(&mut ui, run_menu, "Sair da função", "Shift+F8", Some(|ui, s, _| crate::debugger::command(ui, s, crate::debugger::Cmd::StepOut)), &ids);
    needs_project.push(add_item(&mut ui, run_menu, "Depurar no aparelho", "", Some(|_, s, _| crate::jobs::debug_device(s)), &ids));
    add_item(&mut ui, run_menu, "Inspetor do app em execução…", "Ctrl+Shift+I", Some(crate::live::open), &ids);
    add_item(&mut ui, run_menu, "Profiler…", "", Some(crate::profiler::open), &ids);
    ids.mi_trace = add_item(&mut ui, run_menu, "Rastro de eventos", "", Some(|_, s, _| {
        let mut st = s.borrow_mut();
        st.trace_events = !st.trace_events;
        // Vale já no app conectado e nos próximos.
        if let Some(dev) = &st.dev {
            dev.set_trace(st.trace_events);
        }
        st.status = if st.trace_events { "Rastro de eventos ligado".into() } else { "Rastro de eventos desligado".into() };
    }), &ids);
    separator(&mut ui, run_menu);
    needs_project.push(add_item(&mut ui, run_menu, "Publicar…", "Ctrl+Alt+F9", Some(crate::tooldialogs::publish), &ids));

    // Na tela inicial, só o que faz sentido sem projeto: o resto do Arquivo e do Exibir, os
    // menus de edição, projeto e execução, a barra de ferramentas e os painéis dos lados.
    {
        let mut off = vec![toolbar, left, left_splitter, right, right_splitter, edit_menu, project_menu, run_menu];
        let keep = ["Novo projeto…", "Abrir projeto…", "Abrir form…", "Nova janela", "Abrir gravação…", "Recentes", "Sair"];
        let file_items = ui.children(file_menu).to_vec();
        let last_separator = file_items.iter().rposition(|&c| ui.get::<MenuItem>(c).is_some_and(|m| m.text == "-"));
        for (i, &c) in file_items.iter().enumerate() {
            let text = ui.get::<MenuItem>(c).map(|m| m.text.clone()).unwrap_or_default();
            if !keep.contains(&text.as_str()) && Some(i) != last_separator {
                off.push(c);
            }
        }
        off.extend(ui.children(view_menu).iter().copied().filter(|&c| c != language_menu));
        crate::home::hide_on_home(off);
    }

    let tools_menu = ui.add(MenuItem::new("Ferramentas"), menu);
    add_item(&mut ui, tools_menu, "Opções da IDE…", "", Some(crate::options::open), &ids);
    // O ambiente da máquina (compiladores, SDKs), não do projeto.
    add_item(&mut ui, tools_menu, "Verificar ambiente…", "", Some(|ui, s, i| crate::tooldialogs::environment(ui, s, i, true)), &ids);
    add_item(&mut ui, tools_menu, "Configurar linguagens…", "", Some(|ui, s, i| crate::tooldialogs::languages(ui, s, i)), &ids);
    // Os pacotes de ferramentas (o Rust, o JDK, o SDK do Android...), instalados pelo Aylis.
    add_item(&mut ui, tools_menu, "Pacotes de ferramentas…", "", Some(crate::toolpacksview::open), &ids);
    separator(&mut ui, tools_menu);
    // Os pacotes de componente (`.aylispkg`), instalados na IDE.
    let packages_menu = add_item(&mut ui, tools_menu, "Pacotes de componentes", "", None, &ids);
    add_item(&mut ui, packages_menu, "Pacotes…", "", Some(crate::packagesview::manage_dialog), &ids);
    add_item(&mut ui, packages_menu, "Instalar pacote…", "", Some(crate::packagesview::install_dialog), &ids);
    add_item(&mut ui, packages_menu, "Criar pacote deste form…", "", Some(crate::packagesview::create_dialog), &ids);
    separator(&mut ui, tools_menu);
    add_item(&mut ui, tools_menu, "Servidor MCP para agentes de IA (liga/desliga)", "", Some(|ui, s, _| {
        let on = s.borrow().mcp.is_none();
        actions::set_mcp(ui, s, on, 0, true);
    }), &ids);

    let help_menu = ui.add(MenuItem::new("Ajuda"), menu);
    // A gravação do uso (para achar a causa de um problema): liga/desliga e a pasta.
    ids.mi_record = add_item(&mut ui, help_menu, "Gravar o uso da IDE", "", Some(|ui, s, i| {
        let on = !crate::recordings::recording(ui, i);
        crate::recordings::set_recording(ui, s, i, on);
    }), &ids);
    add_item(&mut ui, help_menu, "Abrir pasta das gravações", "", Some(|_, s, _| {
        let dir = crate::recordings::folder();
        let _ = std::fs::create_dir_all(&dir);
        if let Err(e) = crate::jobs::open_folder(&dir) {
            s.borrow_mut().status = format!("⚠ {e}");
        }
    }), &ids);
    separator(&mut ui, help_menu);
    add_item(&mut ui, help_menu, "Relatar problema…", "", Some(crate::reportview::open), &ids);
    // Versão nova: a consulta pelo menu e a diária (como no NoOBS).
    add_item(&mut ui, help_menu, "Verificar atualizações", "", Some(|ui, _, _| crate::updates::check(ui, true)), &ids);
    let mi_updates = add_item(&mut ui, help_menu, "Avisar de novas versões", "", Some(|ui, _, _| crate::updates::toggle(ui)), &ids);
    crate::updates::set_menu_item(&mut ui, mi_updates);
    add_item(&mut ui, help_menu, "Sobre o Aylis", "", Some(|ui, _, _| crate::dialogs::about(ui)), &ids);

    // Menu de contexto do designer.
    let context_items: [(&str, &str, Action); 9] = [
        ("Recortar", "Ctrl+X", |ui, s, _| actions::edit(ui, s, "cut")),
        ("Copiar", "Ctrl+C", |ui, s, _| actions::edit(ui, s, "copy")),
        ("Colar", "Ctrl+V", |ui, s, _| actions::edit(ui, s, "paste")),
        ("Excluir", "Delete", |ui, s, _| actions::edit(ui, s, "delete")),
        ("Trazer para frente", "", |ui, s, _| actions::edit(ui, s, "front")),
        ("Enviar para trás", "", |ui, s, _| actions::edit(ui, s, "back")),
        ("Selecionar o pai", "", |ui, s, _| actions::edit(ui, s, "parent")),
        ("Ir para o código (evento padrão)", "", |ui, s, _| {
            let handler = actions::default_event(s);
            actions::open_code(ui, s, handler.as_deref());
        }),
        ("Onde é usado…", "", |ui, s, i| actions::show_usages(ui, s, i)),
    ];
    for (i, (text, _, action)) in context_items.into_iter().enumerate() {
        if i == 4 || i == 6 {
            separator(&mut ui, design_menu);
        }
        // Sem atalho aqui: os do menu Editar já valem (senão seriam disparados duas vezes).
        add_item(&mut ui, design_menu, text, "", Some(action), &ids);
    }

    state.borrow_mut().project_only = needs_project;

    // A cada quarto de segundo: o tempo decorrido (e o progresso) da compilação na barra de status.
    // Só liga com um trabalho em andamento (o refresh liga e desliga): parada, a IDE não acorda.
    let mut timer = aylis_controls::Timer::new(250);
    timer.enabled = false;
    let clock = ui.add(timer, form);
    {
        let (state, final_ids) = (state.clone(), final_ids.clone());
        ui.on(clock, "on_timer", move |ui, _, _| {
            if let Some(ids) = final_ids.get().copied() {
                crate::jobs::tick(ui, &state, &ids);
            }
        });
    }

    ids.clock = clock;
    // A cada 30 s, a cópia dos forms alterados para a recuperação (veja `recovery.rs`).
    // No navegador (a lite), o form vai para o armazenamento dele a cada alteração (`lite.rs`).
    let desktop = crate::lite::can(crate::lite::Capability::Disk);
    if desktop {
        let backup = ui.add(aylis_controls::Timer::new(30_000), form);
        let state = state.clone();
        ui.on(backup, "on_timer", move |_, _, _| crate::recovery::snapshot(&state));
    }
    // A IDE grava o próprio uso (Ajuda → Gravar o uso da IDE).
    if desktop {
        crate::recordings::install(&mut ui, form);
    }
    // A tela inicial (sem projeto), no centro, no lugar do designer.
    let (home, home_list) = crate::home::build(&mut ui, &state, center, &final_ids);
    ids.home = home;
    ids.home_list = home_list;
    let _ = final_ids.set(ids);
    let ids = Rc::new(ids);

    // ----- barra de ferramentas -----
    let on = |ui: &mut Ui, id: ComponentId, action: Action| {
        let (state, ids) = (state.clone(), ids.clone());
        ui.on_click(id, move |ui, _| {
            action(ui, &state, &ids);
            refresh(ui, &state, &ids);
        });
    };
    on(&mut ui, b_new, actions::new_project);
    on(&mut ui, b_open, |_, s, i| {
        if s.borrow().project.is_some() { actions::open_form_dialog(s) } else { actions::open_project_dialog(s, i) }
    });
    on(&mut ui, b_save, actions::save_current);
    on(&mut ui, b_save_all, |ui, s, _| {
        crate::inspector::commit_pending(ui);
        crate::codewindow::save_embedded(ui, true);
        actions::save_all(s);
    });
    on(&mut ui, b_undo, |ui, s, _| actions::edit(ui, s, "undo"));
    on(&mut ui, b_redo, |ui, s, _| actions::edit(ui, s, "redo"));
    on(&mut ui, b_form, actions::new_form);
    on(&mut ui, b_code, |ui, s, _| actions::open_code(ui, s, None));
    on(&mut ui, b_run, |ui, s, _| actions::run_project(ui, s));
    on(&mut ui, mi_run_debug, |ui, s, _| crate::debugger::start(ui, s));
    on(&mut ui, mi_run_release, |ui, s, _| actions::run_release(ui, s));
    on(&mut ui, tab_props, |_, s, _| s.borrow_mut().events_tab = false);
    on(&mut ui, tab_events, |_, s, _| s.borrow_mut().events_tab = true);
    on(&mut ui, m_close, |_, s, _| s.borrow_mut().show_messages = false);
    on(&mut ui, m_clear, |_, s, _| s.borrow_mut().clear_output());
    on(&mut ui, b_publish, crate::tooldialogs::publish);
    {
        let (state, ids) = (state.clone(), ids.clone());
        ui.on(target_combo, "on_change", move |ui, sender, _| {
            let index = ui.get::<ComboBox>(sender).map_or(0, |c| c.item_index.max(0) as usize);
            let target = run_targets(&state).get(index).copied().unwrap_or(aylis_tools::Target::Desktop);
            state.borrow_mut().run_target = target;
            refresh(ui, &state, &ids);
        });
    }
    {
        let state = state.clone();
        ui.on(device_combo, "on_change", move |ui, sender, _| {
            let index = ui.get::<ComboBox>(sender).map_or(0, |c| c.item_index.max(0) as usize);
            crate::devices::chosen(&state, index);
        });
    }
    on(&mut ui, m_stop, |_, s, _| actions::stop(s));
    {
        let (state, ids) = (state.clone(), ids.clone());
        ui.on(theme_combo, "on_change", move |ui, sender, _| {
            let dark = ui.get::<ComboBox>(sender).is_some_and(|c| c.item_index == 1);
            actions::set_dark(&state, dark);
            refresh(ui, &state, &ids);
        });
    }
    {
        let (state, ids) = (state.clone(), ids.clone());
        let screens = screens_menu(&mut ui, form, &state, &ids);
        ui.on(view_combo, "on_change", move |ui, sender, _| {
            let index = ui.get::<ComboBox>(sender).map_or(0, |c| c.item_index);
            let entry = usize::try_from(index).ok().and_then(|i| view_entries(&state.borrow().doc).into_iter().nth(i)).map(|(e, _)| e);
            let mut st = state.borrow_mut();
            match entry {
                // Um aparelho: a vista vem do tamanho dele.
                Some(ViewEntry::Device(i)) => {
                    let landscape = st.doc.device.is_some_and(|(_, l)| l);
                    st.doc.show_device(Some((&aylis_core::devices::DEVICES[i], landscape)));
                }
                Some(ViewEntry::Edit) => {
                    drop(st);
                    crate::viewsdialog::open(ui, &state, &ids);
                    refresh(ui, &state, &ids);
                    return;
                }
                // O combo volta ao que estava e o menu abre logo abaixo dele (depois que a
                // lista do combo fechar).
                Some(ViewEntry::Screens) => {
                    drop(st);
                    refresh(ui, &state, &ids);
                    let at = ui.absolute_rect(sender).map_or(PointF::ZERO, |r| PointF::new(r.x, r.bottom()));
                    ui.poster().post(move |ui| PopupMenu::popup(ui, screens, at));
                    return;
                }
                // Uma vista, na prévia dela (390 o celular, 820 o tablet); vazio, o base.
                Some(ViewEntry::View(name)) => {
                    st.doc.show_device(None);
                    st.doc.enter_named_view(&name);
                }
                None => {}
            }
            drop(st);
            refresh(ui, &state, &ids);
        });
    }
    {
        let (state, ids) = (state.clone(), ids.clone());
        ui.on_click(nonvisual_button, move |ui, _| {
            crate::nonvisual::open(ui, &state, &ids);
        });
    }
    {
        let (state, ids) = (state.clone(), ids.clone());
        ui.on_click(gallery_button, move |ui, _| {
            crate::gallery::open(ui, &state, &ids);
        });
    }
    for (search, palette_filter) in [(palette_search, true), (inspector_search, false)] {
        let (state, ids) = (state.clone(), ids.clone());
        ui.on(search, "on_change", move |ui, _, args| {
            // O Edit manda o texto novo como valor (não como `Text`).
            let text = args.text().unwrap_or_default().to_string();
            {
                let mut st = state.borrow_mut();
                if palette_filter { st.palette_filter = text } else { st.inspector_filter = text }
            }
            if palette_filter { panels::palette(ui, &state, &ids) } else { inspector::rebuild(ui, &state, &ids) }
        });
    }
    {
        let (state, ids) = (state.clone(), ids.clone());
        ui.on(surface, "on_change", move |ui, _, _| refresh(ui, &state, &ids));
    }
    {
        let (state, ids) = (state.clone(), ids.clone());
        ui.on(surface, "on_dbl_click", move |ui, _, _| {
            actions::designer_dbl_click(ui, &state, &ids);
            refresh(ui, &state, &ids);
        });
    }
    {
        let ids = ids.clone();
        ui.on(surface, "on_context_menu", move |ui, sender, args| {
            if let EventArgs::Pointer(p) = args {
                let at = panels::popup_at(ui, sender, p.position);
                PopupMenu::popup(ui, ids.design_menu, at);
            }
        });
    }
    {
        // Ao fechar, guarda onde estavam a janela e os painéis.
        let ids = ids.clone();
        ui.on(form, "on_close", move |ui, _, _| actions::save_layout(ui, &ids));
    }
    {
        // A IDE volta a ter o foco: o que mudou no disco por fora (outro editor, um agente)
        // entra nos forms e fontes abertos; com alteração dos dois lados, pergunta.
        let (state, ids) = (state.clone(), ids.clone());
        ui.on(form, "on_activate", move |ui, _, _| {
            let (_, conflicts) = crate::disk::check(ui, &state);
            if !conflicts.is_empty() {
                crate::disk::ask(ui, &state, &ids, conflicts);
            }
        });
    }
    {
        // Fechar a IDE com alterações não salvas: pergunta antes.
        let (state, ids) = (state.clone(), ids.clone());
        ui.on(form, "on_close_query", move |ui, _, _| {
            // O código embutido com alteração não salva: avisa uma vez (mostra o código).
            if !crate::codewindow::confirm_close(ui) {
                ui.cancel_close();
                return;
            }
            let ask = !state.borrow().quit_confirmed && state.borrow().any_modified();
            if ask {
                ui.cancel_close();
                actions::quit(ui, &state, &ids);
            }
        });
    }

    // A janela de código chama a depuração (pontos de parada, F7/F8/F9).
    {
        let (s1, s2, s3, s4, s5, s6) = (state.clone(), state.clone(), state.clone(), state.clone(), state.clone(), state.clone());
        let (s7, s9) = (state.clone(), state.clone());
        let (s10, view_ids) = (state.clone(), final_ids.clone());
        crate::codewindow::set_hooks(crate::codewindow::Hooks {
            toggle: Box::new(move |ui, file, line| crate::debugger::toggle_breakpoint(ui, &s1, file, line)),
            command: Box::new(move |ui, cmd| crate::debugger::command(ui, &s2, cmd)),
            breakpoints: Box::new(move |file| crate::debugger::breakpoints_of(&s3, file)),
            condition: Box::new(move |file, line| crate::debugger::condition_of(&s4, file, line)),
            set_condition: Box::new(move |ui, file, line, c, n| crate::debugger::set_condition(ui, &s5, file, line, c, n)),
            run_to: Box::new(move |ui, file, line| crate::debugger::run_to_cursor(ui, &s6, file, line)),
            // Os fontes do projeto (na linguagem ativa) e onde cada nome é declarado.
            sources: Box::new(move || match s7.borrow().project.as_ref() {
                Some(p) => (Some(p.code_dir()), aylis_project::symbols::sources(p)),
                None => (None, Vec::new()),
            }),
            // O servidor de linguagem (o editor avisa o que mudou e pede sugestões, dicas...).
            lsp: Box::new(move |ui, event| crate::intel::handle(ui, &s9, event)),
            // O código embutido no centro (ou de volta ao designer).
            view: Box::new(move |ui, code| {
                // De volta ao Design (F12 no código): o form do código que estava à vista.
                let form = if code { None } else { actions::form_of_active_code(ui, &s10) };
                // Um fonte sem form (um módulo comum): não há Design para onde ir, e o F12 fica no
                // código (antes mostrava o designer de outro form; relatado pelo autor em 07/11).
                if !code && form.is_none() && crate::codewindow::embedded_active(ui).is_some() {
                    if let Ok(mut st) = s10.try_borrow_mut() {
                        st.status = "Este fonte não tem form: não há Design para ele".into();
                    }
                    if let Some(ids) = view_ids.get().copied() {
                        refresh(ui, &s10, &ids);
                    }
                    return;
                }
                if let Ok(mut st) = s10.try_borrow_mut() {
                    st.code_view = code;
                    if let Some(i) = form {
                        st.switch_to(i);
                    }
                }
                if let Some(ids) = view_ids.get().copied() {
                    crate::home::sync(ui, &s10, &ids);
                    // De volta ao Design, o foco vai para o designer: preso no editor escondido, o
                    // próximo F12 caía no "Ir para o Design" do código e não abria mais o código
                    // (relatado pelo autor em 07/11).
                    if !code {
                        ui.set_focus(ids.surface);
                    }
                }
            }),
            search: {
                let (state, ids) = (state.clone(), final_ids.clone());
                Box::new(move |ui, text| {
                    if let Some(ids) = ids.get().copied() {
                        crate::projectsearch::open(ui, &state, &ids, text);
                    }
                })
            },
            tabs: {
                let (state, ids) = (state.clone(), final_ids.clone());
                Box::new(move |ui| {
                    // No meio de uma ação que mexe no estado, o refresh do fim refaz as abas.
                    if state.try_borrow_mut().is_err() {
                        return;
                    }
                    if let Some(ids) = ids.get().copied() {
                        panels::tabs(ui, &state, &ids);
                    }
                })
            },
            saved: {
                let (state, ids) = (state.clone(), final_ids.clone());
                Box::new(move |ui, file| {
                    // O form deste fonte (o da aba ativa), se alterado, vai junto.
                    let form_changed = {
                        let Ok(st) = state.try_borrow() else { return };
                        !st.no_form && st.doc.modified && actions::code_file_of(&st, &st.doc).is_some_and(|f| crate::debugger::norm(&f) == crate::debugger::norm(file))
                    };
                    if form_changed {
                        actions::save(&state, false);
                        if let Some(ids) = ids.get().copied() {
                            refresh(ui, &state, &ids);
                        }
                    }
                })
            },
        });
    }
    // Design | Código (o F12 alterna): "Código" mostra o código do form da aba (abre se
    // preciso); "Design", o form do código que estava à vista.
    for (button, code) in [(ids.view_design, false), (ids.view_code, true)] {
        let (state, ids) = (state.clone(), *ids);
        ui.on_click(button, move |ui, _| {
            if code {
                if !state.borrow().code_view {
                    actions::open_code(ui, &state, None);
                }
                if crate::codewindow::embedded_has_tabs(ui) {
                    state.borrow_mut().code_view = true;
                }
            } else {
                let form = if state.borrow().code_view { actions::form_of_active_code(ui, &state) } else { None };
                let mut st = state.borrow_mut();
                if let Some(i) = form {
                    st.switch_to(i);
                }
                st.code_view = false;
                drop(st);
                ui.set_focus(ids.surface);
            }
            refresh(ui, &state, &ids);
        });
    }
    // MCP: as chamadas dos agentes rodam aqui (na thread da interface). Liga se estava
    // ligado, ou com AYLIS_IDE_MCP=porta.
    if desktop {
        crate::mcp::install(&state, &ids);
        if let Some(port) = std::env::var("AYLIS_IDE_MCP").ok().and_then(|p| p.parse::<u16>().ok()) {
            actions::set_mcp(&mut ui, &state, true, port, false);
        } else if actions::mcp_was_on() && std::env::var_os("AYLIS_IDE_NO_MCP").is_none() {
            actions::set_mcp(&mut ui, &state, true, 0, false);
        }
    }
    let language = crate::lang::resolve(&state.borrow().language);
    if language != "pt" {
        crate::lang::apply(&mut ui, language);
    }
    refresh(&mut ui, &state, &ids);
    // Instalação nova (nenhuma linguagem instalada): o assistente abre sozinho.
    if desktop {
        crate::tooldialogs::first_run(&ui, &state, &ids);
    }
    // Um menu de cima sem nenhum item que valha aqui (a lite do navegador: Ferramentas) some.
    for top in ui.children(menu).to_vec() {
        let items = ui.children(top).to_vec();
        if !items.is_empty() && items.iter().all(|&i| matches!(ui.get_prop(i, "visible"), Ok(aylis_core::PropValue::Bool(false)))) {
            let _ = ui.set_prop(top, "visible", false);
        }
    }
    Ide { ui, form, state, ids }
}

thread_local! {
    /// Com as Opções do projeto abertas, os painéis atrás (bloqueados pelo modal) não são
    /// refeitos a cada aba aplicada: o `refresh` só anota e roda uma vez ao fechar.
    static DEFERRED_REFRESH: std::cell::Cell<Option<bool>> = const { std::cell::Cell::new(None) };
}

/// Liga (`true`) ou desliga o adiamento do [`refresh`]; ao desligar, diz se algum ficou
/// pendente.
pub fn defer_refresh(on: bool) -> bool {
    DEFERRED_REFRESH.with(|d| d.replace(on.then_some(false))).unwrap_or(false)
}

/// Refaz os painéis e atualiza textos e menus.
pub fn refresh(ui: &mut Ui, state: &Shared, ids: &Ids) {
    if DEFERRED_REFRESH.with(|d| d.get().is_some()) {
        DEFERRED_REFRESH.with(|d| d.set(Some(true)));
        return;
    }
    let desktop = crate::lite::can(crate::lite::Capability::Disk);
    if desktop {
        crate::instance::sync(state);
        actions::sync_project_device(state);
    }
    // O que os controles do form guardam do documento (as colunas da grade) refaz.
    {
        let mut st = state.borrow_mut();
        let root = st.doc.root;
        st.doc.ui.design_changed(root);
        // Editando numa vista por largura: o que mudou vai para ela.
        st.doc.sync_view();
        // O form aberto segue o Exibir → Mostrar componentes escondidos.
        if st.doc.show_hidden != st.show_hidden {
            st.doc.show_hidden = st.show_hidden;
            st.doc.sync_reveal();
        }
    }
    sync_targets(ui, state, ids);
    if desktop {
        crate::devices::sync(ui, state, ids);
    }
    // As vistas do form aberto e a em edição (outro form aberto volta ao base).
    let entries = view_entries(&state.borrow().doc);
    let texts: Vec<String> = entries.iter().map(|(_, t)| ui.tr(t)).collect();
    if ui.get::<ComboBox>(ids.view_combo).is_some_and(|c| c.item_list().into_iter().ne(texts.iter().map(String::as_str))) {
        if let Some(c) = ui.get_mut::<ComboBox>(ids.view_combo) {
            c.set_items(&texts);
        }
    }
    let current = {
        let st = state.borrow();
        match (st.doc.device, st.doc.view_mode.as_ref()) {
            (Some((d, _)), _) => aylis_core::devices::DEVICES.iter().position(|x| x.name == d.name).map(ViewEntry::Device),
            (None, Some(m)) => Some(ViewEntry::View(m.name.clone())),
            (None, None) => Some(ViewEntry::View(String::new())),
        }
    };
    let view_index = entries.iter().position(|(e, _)| Some(e) == current.as_ref()).map_or(0, |i| i as i64);
    if ui.get::<ComboBox>(ids.view_combo).is_some_and(|c| c.item_index != view_index) {
        let _ = ui.set_prop(ids.view_combo, "item_index", view_index);
    }
    panels::palette(ui, state, ids);
    panels::structure(ui, state, ids);
    panels::project(ui, state, ids);
    panels::tabs(ui, state, ids);
    inspector::rebuild(ui, state, ids);
    // As letras de "&" repetidas no form: o Alt+letra só pega o primeiro (avisa ao mudar).
    {
        let mut st = state.borrow_mut();
        let conflicts = st.doc.ui.mnemonic_conflicts(st.doc.root);
        let warning = conflicts
            .iter()
            .map(|(c, ids)| {
                let names: Vec<&str> = ids.iter().map(|&id| st.doc.ui.name(id).unwrap_or("?")).collect();
                format!("Alt+{} em {}", c.to_uppercase(), names.join(", "))
            })
            .collect::<Vec<_>>()
            .join("; ");
        if warning != st.mnemonic_warning {
            if !warning.is_empty() {
                let text = format!("⚠ Atalho repetido no form: {warning}");
                st.push(crate::jobs::OutputLine::new(text.clone(), crate::jobs::OutKind::Warning));
                st.status = text;
            }
            st.mnemonic_warning = warning;
        }
    }
    panels::messages(ui, state, ids);
    // O relógio da compilação só anda com um trabalho em andamento (ou um app conectado no
    // canal, que manda o rastro de eventos).
    let running = state.borrow().job.is_some() || state.borrow().dev.as_ref().is_some_and(|d| d.connected());
    if ui.get::<aylis_controls::Timer>(ids.clock).is_some_and(|t| t.enabled != running) {
        let _ = ui.set_prop(ids.clock, "enabled", running);
    }
    crate::home::sync(ui, state, ids);
    let (title, status, events, locked, dark, caption, recent) = {
        let st = state.borrow();
        let where_ = match &st.project {
            Some(p) => format!("{}  ·  {}", p.name, st.doc.form_name()),
            None => st.doc.title(),
        };
        let mut caption = match &st.project {
            Some(p) => format!("Aylis IDE — {}", p.name),
            None => "Aylis IDE".to_string(),
        };
        // A sessão de agente ligada pelo MCP (pedido do autor em 08/11: mostrar no Aylis a sessão do
        // Claude conectada nele).
        for (_, client, title) in &st.agents {
            let who = if title.is_empty() { client.clone() } else { format!("{client}: {title}") };
            caption = format!("{caption}  ·  {who}");
        }
        (where_, st.status.clone(), st.events_tab, st.doc.locked, st.doc.dark, caption, st.recent.clone())
    };
    let _ = ui.set_prop(ids.doc_title, "text", title);
    let _ = ui.set_prop(ids.status, "text", ui.tr(&status));
    // A janela de código repete a situação enquanto há execução ou depuração.
    let busy = {
        let st = state.borrow();
        st.job.is_some() || st.debugger.active() || st.status.starts_with('⚠')
    };
    let activity = if busy { ui.tr(&status).to_string() } else { String::new() };
    crate::codewindow::show_activity(ui, &activity);
    let _ = ui.set_prop(ids.tab_props, "style", if events { "flat" } else { "" });
    let _ = ui.set_prop(ids.tab_events, "style", if events { "" } else { "flat" });
    let _ = ui.set_prop(ids.mi_lock, "checked", locked);
    let _ = ui.set_prop(ids.mi_hidden, "checked", state.borrow().show_hidden);
    let (has_project, project_only) = {
        let st = state.borrow();
        (st.project.is_some(), st.project_only.clone())
    };
    for id in project_only {
        let _ = ui.set_prop(id, "enabled", has_project);
    }
    let trace = state.borrow().trace_events;
    let _ = ui.set_prop(ids.mi_trace, "checked", trace);
    let recording = crate::recordings::recording(ui, ids);
    let _ = ui.set_prop(ids.mi_record, "checked", recording);
    crate::recordings::report_status(ui, &status);
    crate::fieldsview::sync(ui, state, ids);
    crate::columnsview::sync(ui, state, ids);
    crate::menuview::sync(ui, state, ids);
    let _ = ui.set_prop(ids.form, "caption", caption);
    if ui.get::<ComboBox>(ids.theme_combo).is_some_and(|c| (c.item_index == 1) != dark) {
        let _ = ui.set_prop(ids.theme_combo, "item_index", i64::from(dark));
    }
    // Recentes (submenu refeito).
    let children = ui.children(ids.mi_recent).to_vec();
    ui.free_many(&children);
    if recent.is_empty() {
        let item = ui.add(MenuItem::new("(nenhum)"), ids.mi_recent);
        let _ = ui.set_prop(item, "enabled", false);
    }
    for path in recent {
        let item = ui.add(MenuItem::new(path.display().to_string()), ids.mi_recent);
        let (state, ids) = (state.clone(), *ids);
        ui.on_click(item, move |ui, _| {
            actions::open_recent(ui, &state, &ids, &path);
            refresh(ui, &state, &ids);
        });
    }
    crate::lang::translate(ui);
    ui.request_repaint();
}

/// Usado pelos testes: o label do status.
pub fn status_text(ui: &Ui, ids: &Ids) -> String {
    ui.get::<Label>(ids.status).map(|l| l.text.clone()).unwrap_or_default()
}

/// Posição de um ponto do designer no form da IDE.
pub fn surface_point(ui: &Ui, ids: &Ids, p: PointF) -> PointF {
    panels::popup_at(ui, ids.surface, p)
}

/// As saídas do combo ao lado do ▶ Executar: as que valem para a linguagem do projeto neste
/// sistema (sem projeto, o form solto só tem a prévia no desktop).
pub fn run_targets(state: &Shared) -> Vec<aylis_tools::Target> {
    match &state.borrow().project {
        Some(p) => aylis_tools::Target::available(p.language),
        None => vec![aylis_tools::Target::Desktop],
    }
}

/// Acerta o combo do alvo com a linguagem do projeto: troca a lista quando ela muda, e um
/// alvo que deixou de valer volta para o desktop.
fn sync_targets(ui: &mut Ui, state: &Shared, ids: &Ids) {
    let targets = run_targets(state);
    let names = targets.iter().map(|t| t.name()).collect::<Vec<_>>().join(";");
    let current = state.borrow().run_target;
    let index = match targets.iter().position(|t| *t == current) {
        Some(i) => i,
        None => {
            state.borrow_mut().run_target = aylis_tools::Target::Desktop;
            0
        }
    };
    if ui.get::<ComboBox>(ids.target_combo).is_some_and(|c| c.items != names) {
        let _ = ui.set_prop(ids.target_combo, "items", names);
    }
    if ui.get::<ComboBox>(ids.target_combo).is_some_and(|c| c.item_index != index as i64) {
        let _ = ui.set_prop(ids.target_combo, "item_index", index as i64);
    }
}

/// O que cada item do combo de vistas da barra abre.
#[derive(Clone, Debug, PartialEq)]
pub enum ViewEntry {
    /// Uma vista do form (vazio: o base).
    View(String),
    /// O editor das vistas.
    Edit,
    /// O aparelho em uso (a moldura e o tamanho dele), pelo índice em `DEVICES`.
    Device(usize),
    /// Abre o menu "Telas…" com os aparelhos por tipo.
    Screens,
}

/// Os itens do combo de vistas: o base, as vistas do form (da mais larga para a mais
/// estreita; Tablet e Celular sempre, criados ao entrar), o editor e os aparelhos.
pub fn view_entries(doc: &crate::document::Document) -> Vec<(ViewEntry, String)> {
    let mut names: Vec<String> = doc.views().iter().rev().map(|v| v.name.clone()).collect();
    for (default, after) in [("tablet", None), ("phone", Some("tablet"))] {
        if !names.iter().any(|n| n == default) {
            let at = after.and_then(|a| names.iter().position(|n| n == a)).map_or(names.len(), |i| i + 1);
            names.insert(at, default.to_string());
        }
    }
    let mut out = vec![(ViewEntry::View(String::new()), crate::lang::view_label(""))];
    out.extend(names.into_iter().map(|n| {
        let label = crate::lang::view_label(&n);
        (ViewEntry::View(n), label)
    }));
    // O aparelho em uso (escolhido no menu "Telas…") aparece na lista para ficar à vista.
    if let Some((d, _)) = doc.device {
        if let Some(i) = aylis_core::devices::DEVICES.iter().position(|x| x.name == d.name) {
            out.push((ViewEntry::Device(i), d.name.to_string()));
        }
    }
    out.push((ViewEntry::Edit, "Editar vistas…".to_string()));
    out.push((ViewEntry::Screens, "Telas…".to_string()));
    out
}

/// O menu "Telas…": os aparelhos por tipo — PC (as resoluções), Tablet e Smartphone, este
/// separado em Android e iOS. Escolher um mostra o form na moldura e no tamanho dele.
fn screens_menu(ui: &mut Ui, form: ComponentId, state: &Shared, ids: &Ids) -> ComponentId {
    use aylis_core::devices::{DeviceKind, DeviceOs, DEVICES};
    let menu = ui.add(PopupMenu::default(), form);
    let pc = ui.add(MenuItem::new("PC"), menu);
    let tablet = ui.add(MenuItem::new("Tablet"), menu);
    let phone = ui.add(MenuItem::new("Smartphone"), menu);
    let android = ui.add(MenuItem::new("Android"), phone);
    let ios = ui.add(MenuItem::new("iOS"), phone);
    for (i, d) in DEVICES.iter().enumerate() {
        let parent = match (d.kind, d.os) {
            (DeviceKind::Desktop, _) => pc,
            (DeviceKind::Tablet, _) => tablet,
            (DeviceKind::Phone, DeviceOs::Ios) => ios,
            (DeviceKind::Phone, _) => android,
        };
        let item = ui.add(MenuItem::new(d.name), parent);
        let (state, ids) = (state.clone(), ids.clone());
        ui.on_click(item, move |ui, _| {
            {
                let mut st = state.borrow_mut();
                let landscape = st.doc.device.is_some_and(|(_, l)| l);
                st.doc.show_device(Some((&DEVICES[i], landscape)));
            }
            refresh(ui, &state, &ids);
        });
    }
    menu
}
