//! A documentação na página inteira (05/11, no jeito dos sites de documentação): o botão abre
//! o form maximizado; à esquerda, as seções em árvore com a página aberta marcada; a busca
//! acha no título e no texto; "Nesta página" leva aos títulos; "Anterior" e "Próxima" andam
//! pelo índice; e um link relativo de um guia leva a outro (com ou sem `#titulo`).

use aylis::controls::TreeView;
use aylis::core::{EventArgs, PropValue, Ui};

fn text(ui: &Ui, path: &str, prop: &str) -> String {
    match ui.get_prop(ui.find_path(path).unwrap(), prop).unwrap() {
        PropValue::String(s) => s,
        other => format!("{other:?}"),
    }
}

/// Escolhe o item `item` de uma lista (como o clique) e dispara o `on_change`.
fn choose(ui: &mut Ui, path: &str, item: &str) {
    let list = ui.find_path(path).unwrap();
    let items = text(ui, path, "items");
    let index = items.split(';').position(|i| i == item).unwrap_or_else(|| panic!("{item} não está em {items}"));
    ui.set_prop(list, "item_index", index as i64).unwrap();
    ui.fire(list, "on_change", &EventArgs::Text(item.into()));
}

#[test]
fn a_documentacao_abre_em_tela_cheia_navega_busca_e_segue_os_links() {
    let mut app = landing::app().expect("o app abre");
    let ui = &mut app.ui;
    let botao = ui.find_path("frmPrincipal.btnDocumentacao").unwrap();
    ui.fire(botao, "on_click", &EventArgs::None);
    let form = ui.find_path("frmDocumentacao").expect("o form da documentação abriu");
    assert_eq!(text(ui, "frmDocumentacao", "window_state"), "Enum(\"maximized\")", "a página inteira");
    assert_eq!(text(ui, "frmDocumentacao.mdPagina", "source"), "assets/docs/guias/README.md");

    // A árvore: as seções, com a dos guias aberta e a visão geral marcada.
    let tree = ui.find_path("frmDocumentacao.tvIndice").unwrap();
    {
        let t = ui.get::<TreeView>(tree).unwrap();
        let secoes: Vec<&str> = t.nodes().iter().map(|n| n.text.as_str()).collect();
        assert_eq!(secoes.first(), Some(&"Comece aqui"), "os tutoriais primeiro: {secoes:?}");
        assert!(secoes.contains(&"Guias") && secoes.contains(&"Componentes: Standard"), "{secoes:?}");
        assert!(t.nodes()[0].expanded, "a seção da página aberta fica aberta");
        assert_eq!(t.selected().map(|n| n.data.as_str()), Some("guias/README.md"));
    }
    assert!(text(ui, "frmDocumentacao.lblTrilha", "text").contains("Comece aqui"), "a trilha");

    // "Próxima" leva à página seguinte do índice, e "Anterior" volta.
    let proxima = ui.find_path("frmDocumentacao.btnProxima").unwrap();
    ui.fire(proxima, "on_click", &EventArgs::None);
    let segunda = text(ui, "frmDocumentacao.mdPagina", "source");
    assert_ne!(segunda, "assets/docs/guias/README.md");
    let anterior = ui.find_path("frmDocumentacao.btnAnterior").unwrap();
    ui.fire(anterior, "on_click", &EventArgs::None);
    assert_eq!(text(ui, "frmDocumentacao.mdPagina", "source"), "assets/docs/guias/README.md");

    // A busca acha a página do componente; os achados ficam no lugar da árvore.
    let busca = ui.find_path("frmDocumentacao.edtBusca").unwrap();
    ui.fire(busca, "on_change", &EventArgs::Text("interceptor".into()));
    assert_eq!(text(ui, "frmDocumentacao.tvIndice", "visible"), "Bool(false)");
    choose(ui, "frmDocumentacao.lbBusca", "EventInterceptor — Componentes: System");
    assert_eq!(text(ui, "frmDocumentacao.mdPagina", "source"), "assets/docs/componentes/EventInterceptor.md");

    // "Nesta página": os títulos da página; escolher um rola até ele.
    ui.fire(busca, "on_change", &EventArgs::Text(String::new()));
    let md = ui.find_path("frmDocumentacao.mdPagina").unwrap();
    ui.fire(md, "on_link", &EventArgs::Text("../guias/threads.md".into()));
    assert_eq!(text(ui, "frmDocumentacao.mdPagina", "source"), "assets/docs/guias/threads.md");
    let titulos = text(ui, "frmDocumentacao.lbTitulos", "items");
    let primeiro = titulos.split(';').next().unwrap().trim().to_string();
    assert!(!primeiro.is_empty(), "a página tem títulos: {titulos}");
    choose(ui, "frmDocumentacao.lbTitulos", titulos.split(';').next().unwrap());
    assert_eq!(text(ui, "frmDocumentacao.mdPagina", "anchor"), primeiro);

    // Um link para outra página, num título dela.
    ui.fire(md, "on_link", &EventArgs::Text("interceptador.md#exemplo".into()));
    assert_eq!(text(ui, "frmDocumentacao.mdPagina", "source"), "assets/docs/guias/interceptador.md");
    assert_eq!(text(ui, "frmDocumentacao.mdPagina", "anchor"), "exemplo");
    let _ = form;
}

/// Voltar e avançar (07/11): os botões extras do mouse e Alt+← / Alt+→ andam pelas páginas
/// por onde a pessoa passou; voltar da primeira volta ao site.
#[test]
fn os_botoes_extras_do_mouse_voltam_e_avancam_pelas_paginas() {
    use aylis::core::{Key, KeyEvent, Modifiers, PointerButton, PointerEvent};
    let mut app = landing::app().expect("o app abre");
    let ui = &mut app.ui;
    let botao = ui.find_path("frmPrincipal.btnDocumentacao").unwrap();
    ui.fire(botao, "on_click", &EventArgs::None);
    let form = ui.find_path("frmDocumentacao").unwrap();
    let md = ui.find_path("frmDocumentacao.mdPagina").unwrap();
    ui.fire(md, "on_link", &EventArgs::Text("../guias/threads.md".into()));
    ui.fire(md, "on_link", &EventArgs::Text("interceptador.md#exemplo".into()));
    let pagina = |ui: &Ui| text(ui, "frmDocumentacao.mdPagina", "source");
    let mouse = |button| PointerEvent { button, ..Default::default() };

    ui.pointer_down(form, mouse(PointerButton::Back));
    ui.pointer_up(form, mouse(PointerButton::Back));
    assert_eq!(pagina(ui), "assets/docs/guias/threads.md");
    let alt = |key| KeyEvent { key, modifiers: Modifiers { alt: true, ..Default::default() }, repeat: false };
    ui.key_down(form, alt(Key::Left));
    assert_eq!(pagina(ui), "assets/docs/guias/README.md");
    ui.pointer_down(form, mouse(PointerButton::Forward));
    assert_eq!(pagina(ui), "assets/docs/guias/threads.md");
    ui.key_down(form, alt(Key::Right));
    assert_eq!(pagina(ui), "assets/docs/guias/interceptador.md");
    assert_eq!(text(ui, "frmDocumentacao.mdPagina", "anchor"), "exemplo", "volta ao mesmo título");

    // Uma página nova depois de voltar: o "avançar" que havia some.
    ui.key_down(form, alt(Key::Left));
    ui.fire(md, "on_link", &EventArgs::Text("../guias/README.md".into()));
    ui.key_down(form, alt(Key::Right));
    assert_eq!(pagina(ui), "assets/docs/guias/README.md");

    // Voltar da primeira página fecha a documentação.
    for _ in 0..3 {
        ui.pointer_down(form, mouse(PointerButton::Back));
    }
    assert!(!ui.open_forms().contains(&form), "voltou ao site");
}

/// A página de um componente de uma família leve mostra o componente de verdade montado na
/// própria página (11/11: sem baixar o wasm da família, que traria o núcleo de novo); a
/// próxima página tira o anterior; sem a classe registrada (fora do navegador, as que os forms
/// não usam), volta a imagem.
#[test]
fn a_component_page_shows_the_live_component_inside_the_page() {
    let mut app = landing::app().expect("o app abre");
    let ui = &mut app.ui;
    let botao = ui.find_path("frmPrincipal.btnDocumentacao").unwrap();
    ui.fire(botao, "on_click", &EventArgs::None);
    let md = ui.find_path("frmDocumentacao.mdPagina").unwrap();
    let painel = ui.find_path("frmDocumentacao.pnlDemo").unwrap();

    ui.fire(md, "on_link", &EventArgs::Text("../componentes/Button.md".into()));
    let demo = ui.find_path("frmDocumentacao.demo").expect("o Button ao vivo na página");
    assert_eq!(ui.parent(demo), Some(painel));
    assert_eq!(ui.class_name(demo), Some("Button"));
    assert_eq!(text(ui, "frmDocumentacao.pnlDemo", "visible"), "Bool(true)");
    assert_eq!(text(ui, "frmDocumentacao.wbDemo", "url"), "", "sem o wasm da família");
    assert_eq!(text(ui, "frmDocumentacao.mdPagina", "source"), "");
    assert!(!text(ui, "frmDocumentacao.mdPagina", "text").contains("](img/"), "sem a imagem do componente");

    // Uma página sem componente visual: a demonstração sai.
    ui.fire(md, "on_link", &EventArgs::Text("Timer.md".into()));
    assert!(ui.find_path("frmDocumentacao.demo").is_none());
    assert!(!ui.contains(demo));
    assert_eq!(text(ui, "frmDocumentacao.pnlDemo", "visible"), "Bool(false)");
    assert_eq!(text(ui, "frmDocumentacao.mdPagina", "source"), "assets/docs/componentes/Timer.md");

    // O Gauge: fora do navegador, a Landing sozinha não o registra e mostra a imagem. Na suíte
    // do workspace o cargo une as features do `aylis` de todos os crates, ele vem registrado e
    // a página o monta ao vivo, como no navegador.
    ui.fire(md, "on_link", &EventArgs::Text("Gauge.md".into()));
    if ui.registry().get("Gauge").is_some() {
        let demo = ui.find_path("frmDocumentacao.demo").expect("o Gauge registrado: ao vivo na página");
        assert_eq!(ui.class_name(demo), Some("Gauge"));
        assert_eq!(text(ui, "frmDocumentacao.mdPagina", "source"), "");
    } else {
        assert!(ui.find_path("frmDocumentacao.demo").is_none());
        assert_eq!(text(ui, "frmDocumentacao.mdPagina", "source"), "assets/docs/componentes/Gauge.md");
    }
}

/// No celular (a vista `phone`), o índice sai da esquerda e vira um menu de tela inteira: o
/// botão do topo abre e fecha, escolher uma página fecha, a busca abre com os achados. De
/// volta à tela larga, o índice fica à esquerda de novo.
#[test]
fn on_the_phone_the_index_is_a_full_screen_menu() {
    let app = landing::app().expect("o app abre");
    let mut h = app.headless();
    let botao = h.ui.find_path("frmPrincipal.btnDocumentacao").unwrap();
    h.ui.fire(botao, "on_click", &EventArgs::None);
    let form = h.ui.find_path("frmDocumentacao").unwrap();
    h.set_form(form);
    h.resize(390.0, 844.0);
    assert_eq!(h.ui.active_view(form), Some("phone"));
    let visible = |h: &aylis::headless::Headless, name: &str| text(&h.ui, &format!("frmDocumentacao.{name}"), "visible") == "Bool(true)";
    assert!(visible(&h, "btnMenu") && !visible(&h, "pnlLateral") && visible(&h, "layConteudo"), "a página, com o menu fechado");
    let pagina = h.ui.absolute_rect(h.ui.find_path("frmDocumentacao.mdPagina").unwrap()).unwrap();
    assert!(pagina.width > 340.0, "a página ocupa a largura: {pagina:?}");

    // O menu abre no lugar da página; uma seção só abre na árvore; uma página fecha o menu.
    let menu = h.ui.find_path("frmDocumentacao.btnMenu").unwrap();
    h.ui.fire(menu, "on_click", &EventArgs::None);
    h.settle();
    assert!(visible(&h, "pnlLateral") && !visible(&h, "layConteudo"));
    let tree = h.ui.find_path("frmDocumentacao.tvIndice").unwrap();
    h.ui.get_mut::<TreeView>(tree).unwrap().set_selected_path(Some(vec![1]));
    h.ui.fire(tree, "on_change", &EventArgs::None);
    assert!(visible(&h, "pnlLateral"), "a seção não fecha o menu");
    h.ui.get_mut::<TreeView>(tree).unwrap().set_selected_path(Some(vec![1, 0]));
    h.ui.fire(tree, "on_change", &EventArgs::None);
    assert!(!visible(&h, "pnlLateral") && visible(&h, "layConteudo"), "a página fecha o menu");

    // A busca abre o menu com os achados; escolher um fecha.
    let busca = h.ui.find_path("frmDocumentacao.edtBusca").unwrap();
    h.ui.fire(busca, "on_change", &EventArgs::Text("interceptor".into()));
    assert!(visible(&h, "pnlLateral") && visible(&h, "lbBusca"));
    choose(&mut h.ui, "frmDocumentacao.lbBusca", "EventInterceptor — Componentes: System");
    assert!(!visible(&h, "pnlLateral") && visible(&h, "layConteudo"));
    assert_eq!(text(&h.ui, "frmDocumentacao.mdPagina", "source"), "assets/docs/componentes/EventInterceptor.md");

    // Com o menu aberto, a tela alarga: o índice à esquerda e a página ao lado, sem o botão.
    h.ui.fire(menu, "on_click", &EventArgs::None);
    h.resize(1440.0, 900.0);
    assert_eq!(h.ui.active_view(form), None);
    assert!(!visible(&h, "btnMenu") && visible(&h, "pnlLateral") && visible(&h, "layConteudo"));
    assert_eq!(h.ui.bounds(h.ui.find_path("frmDocumentacao.pnlLateral").unwrap()).unwrap().width, 300.0);
}
