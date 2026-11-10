//! A documentação da página do aylis (`examples/Landing`): os guias de `docs/` e a referência de
//! cada componente registrado (a paleta, a descrição, as propriedades com o nome do Object
//! Inspector e o tipo, os eventos e quais perguntam), num índice que a tela de documentação
//! lê. Refeita com `aylis-tool docs` (a referência sai do próprio registro, então não fica
//! velha); o teste confere que toda classe tem página.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::Path;

use aylis_core::{ControlBase, PropInfo, PropKind, prop_infos_of, question::question_fields};
use serde_json::{Value, json};

use crate::document::new_document_ui;
use crate::lang::property_label;
use crate::nonvisual;

/// Os guias de `docs/` que não vão para a página (histórico interno e roteiros de teste).
const PRIVATE_GUIDES: &[&str] = &["historico-decisoes.md", "testes-em-dispositivos.md", "roteiro-de-conferencia.md", "proposta-llm-ci.md", "plano.md", "ajustes-pendentes.md"];

/// "Comece aqui": a visão geral e os tutoriais, nesta ordem (o resto de `docs/` vai para os
/// guias).
const START_HERE: &[&str] = &["README.md", "tutorial-primeiro-projeto.md", "tutorial-componente.md"];

/// Para que serve, escrito para a página: onde o comentário do código é curto demais ("Botão.")
/// ou não existe. As outras classes usam o comentário da classe no código.
const DESCRIPTIONS: &[(&str, &str)] = &[
    ("Button", "Botão que dispara uma ação: texto, ícone opcional (à esquerda, à direita ou em cima) e os estilos do tema (contorno, plano, flutuante...). Pode ficar marcado (`checkable`) e, num diálogo, fechar o form com um `modal_result`."),
    ("Label", "Texto fixo na tela: um título, uma legenda, a explicação de um campo. Quebra linhas (`wrap`), cresce com o texto (`auto_size`) e segue a fonte e a cor do tema."),
    ("Edit", "Campo de texto de uma linha para o usuário digitar: nome, e-mail, busca. Tem texto de ajuda (`placeholder`), máscara de caracteres permitidos, senha e sugestões."),
    ("Memo", "Campo de texto de várias linhas: observações, descrições, código. Rola, quebra linhas e pode mostrar os números das linhas e realçar a sintaxe."),
    ("CheckBox", "Caixa de marcação para uma opção de sim ou não (\"Lembrar de mim\"), com o texto ao lado; clicar no texto também marca."),
    ("ComboBox", "Lista suspensa: mostra a opção escolhida e abre a lista para trocar. Os itens vêm de `items`, separados por `;`."),
    ("NumberBox", "Campo numérico com setas para aumentar e diminuir, mínimo, máximo, passo e casas decimais."),
    ("Chart", "Gráfico de linhas, barras, áreas, pizza e outros tipos, com legenda e eixos; as séries vêm por código ou ligadas aos dados."),
    ("DataGrid", "Grade de registros ligada a um `DataSource`: colunas pelos campos do dataset, edição na célula, ordenação, filtro e milhões de linhas sem travar."),
    ("MapView", "Mapa interativo de blocos (como os mapas da web): arrastar, aproximar com a roda e marcadores nas coordenadas."),
    ("NodeGraph", "Editor de nós ligados por cabos: fluxos, roteamento de áudio, automações. Arrastar os nós, ligar as portas e aproximar."),
    ("TitleBar", "Barra de título própria da janela, desenhada pelo Aylis: o título, os botões de minimizar, maximizar e fechar e espaço para os seus controles."),
    ("Markdown", "Mostra texto em Markdown com as cores e fontes do tema: títulos, listas, tabelas, código realçado, imagens e links, com seleção e cópia."),
    ("CodeEditor", "Editor de código com realce de sintaxe, números de linha, minimapa e dobra de blocos, para mostrar ou editar código dentro do app."),
    ("TabStrip", "Faixa de abas como as de um navegador: fechar, acrescentar, arrastar para reordenar e mostrar a página da aba escolhida."),
    ("Fan", "Ventilador de supervisório: gira quando ligado e muda de cor no alarme."),
    ("Motor", "Motor de supervisório: o símbolo com o estado (ligado, desligado, em alarme) nas cores da indústria."),
    ("Pump", "Bomba de supervisório: o símbolo com o rotor girando quando ligada e o estado em cor."),
    ("DragImage", "A imagem que acompanha o mouse durante um arrastar e soltar."),
    ("Frame", "Um pedaço de tela reaproveitável (um form dentro de outro), com os seus próprios componentes e eventos."),
    ("Layout", "Contêiner sem desenho próprio: agrupa os controles e os arruma pelo alinhamento (`align`), margens e espaçamento."),
];

/// Os guias de cada componente (o "Veja também" da página).
const SEE_ALSO: &[(&str, &str, &str)] = &[
    ("ClientDataSet", "Gravar as alterações no banco", "../guias/gravar-alteracoes.md"),
    ("DataSource", "Gravar as alterações no banco", "../guias/gravar-alteracoes.md"),
    ("EventInterceptor", "Interceptar eventos num lugar só", "../guias/interceptador.md"),
    ("ApplicationEvents", "Interceptar eventos num lugar só", "../guias/interceptador.md"),
];

/// Subcomponentes e raízes, que têm página mas ficam no fim do índice.
const SUBCOMPONENTS: &[&str] = &["Form", "DataModule", "Field", "Column", "MenuItem", "TabItem", "ReportBand"];

fn kind_text(kind: PropKind) -> String {
    match kind {
        PropKind::Bool => "lógico".into(),
        PropKind::Int => "inteiro".into(),
        PropKind::Float => "número".into(),
        PropKind::String => "texto".into(),
        PropKind::Color => "cor".into(),
        PropKind::Enum(names) => format!("um de: {}", names.iter().map(|n| format!("`{n}`")).collect::<Vec<_>>().join(", ")),
        PropKind::Set(names) => format!("vários de: {}", names.iter().map(|n| format!("`{n}`")).collect::<Vec<_>>().join(", ")),
        PropKind::Bounds => "posição e tamanho".into(),
        PropKind::Component => "componente".into(),
        PropKind::Event => "evento".into(),
        PropKind::Angle => "ângulo (graus)".into(),
        PropKind::Duration => "duração (ms)".into(),
        PropKind::Shadow => "sombra".into(),
        PropKind::Easing => "curva de suavização".into(),
    }
}

/// A categoria com o nome do Object Inspector (`@Layout` na tabela de nomes).
fn category_label(category: &str) -> String {
    let label = property_label(&format!("@{category}"));
    if label.starts_with('@') { category.to_string() } else { label }
}

fn prop_table(props: &[&PropInfo]) -> String {
    let mut by_category: BTreeMap<&str, Vec<&PropInfo>> = BTreeMap::new();
    for p in props {
        by_category.entry(p.category).or_default().push(p);
    }
    let mut s = String::new();
    for (category, list) in by_category {
        let _ = writeln!(s, "### {}\n\n| Propriedade | No Object Inspector | Tipo |\n|---|---|---|", category_label(category));
        for p in list {
            let mut kind = kind_text(p.kind);
            if p.read_only {
                kind.push_str(" (somente leitura)");
            }
            let _ = writeln!(s, "| `{}` | {} | {kind} |", p.name, property_label(p.name));
        }
        s.push('\n');
    }
    s
}

/// Quando o evento chega, para a tabela de eventos (a página do `ClientDataSet` só tinha o nome
/// do `on_server_query`; conferência de 08/11). Os que não estão aqui ficam com o rótulo.
const EVENT_NOTES: &[(&str, &str)] = &[
    ("on_click", "o clique (ou Enter e Espaço com o foco)"),
    ("on_dbl_click", "o duplo clique"),
    ("on_change", "o valor mudou (pelo usuário ou pelo código, como na VCL)"),
    ("on_show", "o form abriu"),
    ("on_close", "o form fechou"),
    ("on_close_query", "o form vai fechar: `Err` recusa (o X não fecha)"),
    ("on_create", "o componente foi criado (o DataModule, ao abrir o app)"),
    ("on_timer", "a cada `interval` ms, com `enabled`"),
    ("on_resize", "o tamanho mudou"),
    ("on_key_down", "uma tecla com o foco no componente"),
    ("before_post", "o registro vai ser gravado: `Err` recusa, com a mensagem"),
    ("after_post", "o registro foi gravado"),
    ("before_delete", "o registro vai ser apagado: `Err` recusa"),
    ("after_scroll", "o registro atual mudou"),
    (
        "on_server_query",
        "o dataset pede ao backend os registros (o filtro, a ordem e a página que a tela quer), em vez de filtrar na tela: o handler responde com as linhas (veja o guia [Gravar as alterações](../guias/gravar-alteracoes.md))",
    ),
];

fn event_table(events: &[&PropInfo]) -> String {
    let mut s = String::from("| Evento | Quando | No Object Inspector | Pergunta |\n|---|---|---|---|\n");
    for e in events {
        let question = match question_fields(e.name) {
            Some([]) => "sim (recusar com a exceção)".to_string(),
            Some(fields) => format!("sim: {}", fields.iter().map(|f| format!("`{f}`")).collect::<Vec<_>>().join(", ")),
            None => String::new(),
        };
        let label = property_label(e.name);
        let label = if label == e.name { "—".to_string() } else { label };
        let when = EVENT_NOTES.iter().find(|(n, _)| *n == e.name).map_or("—", |(_, w)| w);
        let _ = writeln!(s, "| `{}` | {when} | {label} | {question} |", e.name);
    }
    s
}

/// O título de um guia (a primeira linha `# ...`).
fn title_of(text: &str, fallback: &str) -> String {
    text.lines().find_map(|l| l.strip_prefix("# ")).map(str::trim).unwrap_or(fallback).to_string()
}

/// Grava a documentação em `out` (`guias/`, `componentes/` e `indice.json`) a partir de
/// `docs` (os guias). Devolve quantas páginas escreveu.
pub fn write(out: &Path, docs: &Path) -> std::io::Result<usize> {
    let guides_dir = out.join("guias");
    let components_dir = out.join("componentes");
    std::fs::create_dir_all(&guides_dir)?;
    std::fs::create_dir_all(&components_dir)?;
    let mut pages = 0;

    // Os guias: copiados de docs/, com o README como entrada.
    let mut guides = Vec::new();
    let mut names: Vec<String> = std::fs::read_dir(docs)?
        .flatten()
        .map(|e| e.file_name().to_string_lossy().to_string())
        .filter(|n| n.ends_with(".md") && !PRIVATE_GUIDES.contains(&n.as_str()))
        .collect();
    names.sort_by_key(|n| (n != "README.md", n.clone()));
    // "Comece aqui": a visão geral e os tutoriais, na ordem de leitura; o resto, nos guias.
    let mut start = Vec::new();
    let root = docs.parent().unwrap_or(docs);
    for name in names {
        let text = include_sources(&std::fs::read_to_string(docs.join(&name))?, root);
        std::fs::write(guides_dir.join(&name), &text)?;
        let title = if name == "README.md" { "Visão geral".to_string() } else { title_of(&text, &name) };
        let page = json!({ "titulo": title, "arquivo": format!("guias/{name}") });
        match START_HERE.iter().position(|s| *s == name) {
            Some(order) => start.push((order, page)),
            None => guides.push(page),
        }
        pages += 1;
    }
    start.sort_by_key(|(order, _)| *order);
    let start: Vec<Value> = start.into_iter().map(|(_, page)| page).collect();

    // A referência: uma página por classe, agrupada pela aba da paleta.
    let ui = new_document_ui(false);
    let classes: Vec<_> = ui.registry().iter().copied().collect();
    let all: Vec<(&str, Vec<PropInfo>, bool)> = classes
        .iter()
        .map(|c| {
            let component = (c.create)();
            (c.name, prop_infos_of(component.as_ref()), component.control().is_some())
        })
        .collect();
    // A imagem de cada componente visual, no topo da página dele.
    let visual: Vec<&str> = all.iter().filter(|(name, _, v)| *v && !SUBCOMPONENTS.contains(name)).map(|(name, _, _)| *name).collect();
    let pictured = component_images(&components_dir.join("img"), &visual)?;
    // O que todo controle visual tem (o `ControlBase`): numa página só, sem repetir em cada uma.
    let common: Vec<&'static str> = ControlBase::PROPS.iter().map(|p| p.name).collect();
    let mut by_category: BTreeMap<String, Vec<Value>> = BTreeMap::new();
    let from_sources = docs.parent().map(source_descriptions).unwrap_or_default();
    for (info, (name, props, is_visual)) in classes.iter().zip(&all) {
        let shown: Vec<&PropInfo> = props.iter().filter(|p| !p.runtime).collect();
        let own_props: Vec<&PropInfo> =
            shown.iter().copied().filter(|p| p.kind != PropKind::Event && !(*is_visual && common.contains(&p.name))).collect();
        let events: Vec<&PropInfo> = shown.iter().copied().filter(|p| p.kind == PropKind::Event && !(*is_visual && common.contains(&p.name))).collect();
        let mut s = format!("# {name}\n\n");
        if pictured.contains(*name) {
            let _ = writeln!(s, "![{name}](img/{name}.png)\n");
        }
        // Para que serve: a dos não visuais (escrita para a paleta) ou o comentário da classe
        // no código.
        let description = match (DESCRIPTIONS.iter().find(|(c, _)| c == name), nonvisual::description(name)) {
            (Some((_, d)), _) => d.to_string(),
            (None, "") => from_sources.get(*name).cloned().unwrap_or_default(),
            (None, d) => d.to_string(),
        };
        if !description.is_empty() {
            let _ = writeln!(s, "{description}\n");
        }
        let _ = writeln!(
            s,
            "**Paleta:** {} · **{}**{}\n",
            info.category,
            if *is_visual { "controle visual" } else { "componente não visual" },
            if *is_visual { " · também tem as [propriedades e eventos comuns a todo controle](controles.md)" } else { "" }
        );
        s.push_str("## Propriedades\n\n");
        if own_props.is_empty() {
            s.push_str("Só as comuns.\n\n");
        } else {
            s.push_str(&prop_table(&own_props));
        }
        if !events.is_empty() {
            s.push_str("## Eventos\n\n");
            s.push_str(&event_table(&events));
            s.push('\n');
        }
        for (_, title, link) in SEE_ALSO.iter().filter(|(c, _, _)| c == name) {
            let _ = writeln!(s, "**Veja também:** [{title}]({link})\n");
        }
        // Como no código do app: o componente do form já vem com o tipo dele (`self.Nome1`, o
        // acesso tipado que a IDE gera) e o evento chega num método do form, ligado no form.
        s.push_str("## Em código\n\n");
        let field = format!("{name}1");
        let (event, handler) = match events.first() {
            Some(e) => (e.name, aylis_project::codegen::default_handler_name(&field, e.name)),
            None => ("on_show", "form_show".to_string()),
        };
        let _ = writeln!(
            s,
            "No form, o `{field}` com o evento ligado (no Object Inspector, na aba Eventos):\n\n```json\n{{ \"class\": \"{name}\", \"name\": \"{field}\"{} }}\n```\n",
            if events.is_empty() { String::new() } else { format!(", \"{event}\": \"{handler}\"") }
        );
        let _ = writeln!(s, "E o handler, um método do form (`self.{field}` já é um `{name}`):\n\n```rust\nimpl FrmPrincipal {{");
        let _ = writeln!(s, "    pub fn {handler}(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {{");
        match own_props.iter().find(|p| matches!(p.kind, PropKind::String | PropKind::Bool | PropKind::Float | PropKind::Int)) {
            Some(p) => {
                // Lê e grava pelo componente tipado (o tipo do valor é o da propriedade).
                let novo = match p.kind {
                    PropKind::Bool => "!atual",
                    PropKind::Float => "atual + 10.0",
                    PropKind::Int => "atual + 1",
                    _ => "format!(\"{atual}!\")",
                };
                let _ = writeln!(s, "        let atual = self.{field}.{}(ui);", p.name);
                let _ = writeln!(s, "        self.{field}.set_{}(ui, {novo});", p.name);
            }
            None => {
                let _ = writeln!(s, "        let _ = &self.{field};");
            }
        }
        s.push_str("    }\n}\n```\n");
        if events.is_empty() {
            s.push_str("\n(Sem eventos próprios: o exemplo usa o `on_show` do form.)\n");
        }
        std::fs::write(components_dir.join(format!("{name}.md")), s)?;
        pages += 1;
        let section = if SUBCOMPONENTS.contains(name) { "Formulários e subcomponentes".to_string() } else { info.category.to_string() };
        by_category.entry(section).or_default().push(json!({ "titulo": name, "arquivo": format!("componentes/{name}.md") }));
    }
    // As comuns.
    let base: Vec<&PropInfo> = ControlBase::PROPS.iter().filter(|p| !p.runtime).collect();
    let (base_events, base_props): (Vec<&PropInfo>, Vec<&PropInfo>) = base.into_iter().partition(|p| p.kind == PropKind::Event);
    let mut s = String::from("# Propriedades e eventos comuns a todo controle\n\nTodo controle visual (o que tem `ControlBase`) tem estas, além das próprias.\n\n## Propriedades\n\n");
    s.push_str(&prop_table(&base_props));
    s.push_str("## Eventos\n\n");
    s.push_str(&event_table(&base_events));
    std::fs::write(components_dir.join("controles.md"), s)?;
    pages += 1;

    let mut sections = vec![json!({ "secao": "Comece aqui", "paginas": start }), json!({ "secao": "Guias", "paginas": guides })];
    sections.push(json!({ "secao": "Componentes", "paginas": [{ "titulo": "Comuns a todo controle", "arquivo": "componentes/controles.md" }] }));
    let last = by_category.remove("Formulários e subcomponentes");
    // Na ordem das abas da paleta da IDE (Standard primeiro); as que não estão nela, depois.
    let mut by_category: Vec<(String, Vec<Value>)> = by_category.into_iter().collect();
    by_category.sort_by_key(|(section, _)| (crate::panels::CATEGORY_ORDER.iter().position(|c| c == section).unwrap_or(usize::MAX), section.clone()));
    for (section, mut list) in by_category {
        list.sort_by(|a, b| a["titulo"].as_str().cmp(&b["titulo"].as_str()));
        sections.push(json!({ "secao": format!("Componentes: {section}"), "paginas": list }));
    }
    if let Some(list) = last {
        sections.push(json!({ "secao": "Componentes: formulários e subcomponentes", "paginas": list }));
    }
    // As telas dos tutoriais (o projeto do tutorial está no repositório).
    if let Some(root) = docs.parent() {
        tutorial_images(&docs.join("img"), root)?;
    }
    // As imagens dos guias (as dos tutoriais e as que os guias citarem) vão junto com eles.
    if let Ok(images) = std::fs::read_dir(docs.join("img")) {
        std::fs::create_dir_all(guides_dir.join("img"))?;
        for image in images.flatten() {
            std::fs::copy(image.path(), guides_dir.join("img").join(image.file_name()))?;
        }
    }
    std::fs::write(out.join("indice.json"), serde_json::to_string_pretty(&json!({ "secoes": sections })).unwrap_or_default() + "\n")?;
    Ok(pages)
}

/// O desenho de cada componente visual (o exemplo da galeria, no tema escuro do site), com uma
/// margem em volta: a imagem do topo da página dele. Devolve os que têm imagem (o desenho de um
/// componente que falhar fica sem ela, e a página também).
fn component_images(dir: &Path, names: &[&str]) -> std::io::Result<std::collections::HashSet<String>> {
    std::fs::create_dir_all(dir)?;
    let mut ui = new_document_ui(true);
    ui.set_design_mode(false);
    let renderer = aylis_skia::SkiaRenderer::new();
    ui.services_mut().text = Some(Box::new(renderer.text_service()));
    let margin = 16.0;
    let mut done = std::collections::HashSet::new();
    for &name in names {
        let png = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let form = crate::gallery::preview_form(&mut ui, name, name)?;
            ui.layout();
            let (w, h) = ui.control(form).map(|cb| (cb.width(), cb.height()))?;
            let background = ui.theme().color("background");
            let png = renderer.render_png((w + 2.0 * margin).ceil() as u32, (h + 2.0 * margin).ceil() as u32, 1.0, background, |c| {
                aylis_core::Canvas::translate(c, margin, margin);
                ui.paint(form, c);
            });
            ui.free(form);
            png
        }));
        if let Ok(Some(png)) = png {
            std::fs::write(dir.join(format!("{name}.png")), png)?;
            done.insert(name.to_string());
        }
    }
    Ok(done)
}

/// As telas do tutorial "Seu primeiro projeto", desenhadas sem janela com o projeto
/// `examples/OlaNome`: a tela inicial da IDE, o designer com o form e o app rodando. A IDE usa
/// uma configuração à parte (os recentes de quem roda o gerador ficam como estão).
fn tutorial_images(dir: &Path, root: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(dir)?;
    let config = std::env::temp_dir().join(format!("aylis-docs-{}", std::process::id()));
    std::fs::create_dir_all(&config)?;
    crate::actions::use_private_config(&config);
    let renderer = aylis_skia::SkiaRenderer::new();
    let project = root.join("examples/OlaNome/OlaNome.aylisproj");
    // A IDE: a tela inicial e o designer com o projeto aberto (o botão escolhido, para o
    // Object Inspector aparecer preenchido).
    for (file, open) in [("tutorial-inicio.png", false), ("tutorial-designer.png", true)] {
        let state = crate::state::shared(crate::document::Document::new());
        let mut ide = crate::app::build(state.clone());
        ide.ui.services_mut().text = Some(Box::new(renderer.text_service()));
        if let Some(cb) = ide.ui.control_mut(ide.form) {
            cb.set_size(1280.0, 800.0);
        }
        if open {
            crate::actions::open_project(&mut ide.ui, &state, &ide.ids, &project);
            let mut st = state.borrow_mut();
            let button = st.doc.tree().into_iter().map(|(id, _)| id).find(|&id| st.doc.ui.name(id) == Some("btnSaudar"));
            st.doc.select(button);
        }
        ide.ui.layout();
        crate::app::refresh(&mut ide.ui, &ide.state, &ide.ids);
        ide.ui.layout();
        let background = ide.ui.theme().color("background");
        let (ui, form) = (&mut ide.ui, ide.form);
        if let Some(png) = renderer.render_png(1280, 800, 1.0, background, |c| ui.paint(form, c)) {
            std::fs::write(dir.join(file), png)?;
        }
    }
    // O app rodando, com um nome digitado e a saudação.
    let json = std::fs::read_to_string(root.join("examples/OlaNome/forms/principal.json"))?;
    let mut ui = new_document_ui(false);
    ui.set_design_mode(false);
    ui.services_mut().text = Some(Box::new(renderer.text_service()));
    aylis_project::install_assets(&mut ui, &aylis_project::Files::Dir(root.join("examples/OlaNome")));
    let form = aylis_json::load(&mut ui, &json, None).map_err(|e| std::io::Error::other(e.to_string()))?;
    for (path, text) in [("frmPrincipal.edtNome", "Maria"), ("frmPrincipal.lblResposta", "Olá, Maria!")] {
        if let Some(id) = ui.find_path(path) {
            let _ = ui.set_prop(id, "text", text);
        }
    }
    ui.layout();
    let (w, h) = ui.control(form).map_or((420.0, 260.0), |cb| (cb.width(), cb.height()));
    let background = ui.theme().color("background");
    if let Some(png) = renderer.render_png(w as u32, h as u32, 1.0, background, |c| ui.paint(form, c)) {
        std::fs::write(dir.join("tutorial-app.png"), png)?;
    }
    // O componente do tutorial: o botão com três cliques, num form claro e num escuro.
    for (file, dark) in [("tutorial-componente.png", false), ("tutorial-componente-escuro.png", true)] {
        let mut ui = new_document_ui(dark);
        ui.set_design_mode(false);
        ui.services_mut().text = Some(Box::new(renderer.text_service()));
        counter_button::register(&mut ui);
        let form = ui.add_root(aylis_controls::Form::new("frmTeste", 260.0, 84.0));
        let _ = ui.set_prop(form, "padding", 20.0);
        if let Ok(button) = ui.create("CounterButton", Some(form)) {
            let _ = ui.set_prop(button, "align", "client");
            for _ in 0..3 {
                ui.click(button);
            }
        }
        let _ = ui.show(form);
        ui.layout();
        let background = ui.theme().color("background");
        if let Some(png) = renderer.render_png(260, 84, 1.0, background, |c| ui.paint(form, c)) {
            std::fs::write(dir.join(file), png)?;
        }
    }
    let _ = std::fs::remove_dir_all(&config);
    Ok(())
}

/// O componente do tutorial (o pacote de exemplo), compilado aqui para a imagem dele sair do
/// código de verdade.
#[path = "../../../examples/pacotes/BotaoContador/src/lib.rs"]
mod counter_button;

/// Troca cada `<!-- incluir: caminho -->` de um guia pelo arquivo (do repositório) num bloco
/// de código, sem os testes: o código do tutorial é o do exemplo, que compila e é testado.
/// Para que serve cada classe: o primeiro parágrafo do comentário de documentação (`///`) do
/// tipo dela no código, achado pela lista de classes da fachada (`Image` é o `ImageView`).
/// Fica só o texto: os links viram nome e as comparações com outras bibliotecas saem.
fn source_descriptions(root: &Path) -> std::collections::HashMap<String, String> {
    let facade = std::fs::read_to_string(root.join("crates/aylis/src/lib.rs")).unwrap_or_default();
    // Classe → (crate, tipo): `Ellipse: aylis_controls::shapes::Ellipse => ...` dá `aylis-controls` e `Ellipse`.
    let mut types: Vec<(String, String, String)> = Vec::new();
    for line in facade.lines() {
        let line = line.trim();
        let line = if line.starts_with("#[") { line.split_once("] ").map_or(line, |(_, r)| r) } else { line };
        if let Some((class, rest)) = line.split_once(": ").filter(|(c, r)| c.chars().all(char::is_alphanumeric) && r.contains(" => \"")) {
            let path = rest.split(" => ").next().unwrap_or_default();
            types.push((class.to_string(), path.split("::").next().unwrap_or_default().replace('_', "-"), path.rsplit("::").next().unwrap_or(path).to_string()));
        } else if let Some(path) = line.split_once("register_class::<").map(|(_, r)| r.split('>').next().unwrap_or_default()) {
            let ty = path.rsplit("::").next().unwrap_or(path).to_string();
            types.push((ty.clone(), path.split("::").next().unwrap_or_default().replace('_', "-"), ty));
        }
    }
    // (crate, tipo) → comentário, de todos os fontes dos crates.
    let mut docs: std::collections::HashMap<(String, String), String> = std::collections::HashMap::new();
    let mut stack = vec![root.join("crates")];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&dir) else { continue };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if !matches!(path.file_name().and_then(|n| n.to_str()), Some("target" | "tests" | "examples" | "benches")) {
                    stack.push(path);
                }
                continue;
            }
            if path.extension().is_none_or(|e| e != "rs") {
                continue;
            }
            let Ok(text) = std::fs::read_to_string(&path) else { continue };
            // O crate do arquivo (`crates/<crate>/src/...`).
            let krate = path.strip_prefix(root.join("crates")).ok().and_then(|p| p.components().next()).map(|c| c.as_os_str().to_string_lossy().to_string()).unwrap_or_default();
            let lines: Vec<&str> = text.lines().collect();
            for (i, line) in lines.iter().enumerate() {
                let Some(rest) = line.strip_prefix("pub struct ") else { continue };
                let name: String = rest.chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect();
                if docs.contains_key(&(krate.clone(), name.clone())) {
                    continue;
                }
                // As linhas `///` logo acima (pulando os atributos), na ordem.
                let mut doc = Vec::new();
                for prev in lines[..i].iter().rev() {
                    let prev = prev.trim();
                    if prev.starts_with("#[") {
                        continue;
                    }
                    match prev.strip_prefix("///") {
                        Some(d) => doc.push(d.trim()),
                        None => break,
                    }
                }
                doc.reverse();
                let paragraph: Vec<&str> = doc.into_iter().take_while(|l| !l.is_empty()).collect();
                if !paragraph.is_empty() {
                    docs.insert((krate.clone(), name), clean_description(&paragraph.join(" ")));
                }
            }
        }
    }
    // O tipo no crate de onde a classe vem (o `aylis-controls` junta os `aylis-controls-*`): um
    // `Layout` do Markdown ou um `Frame` de vídeo têm o mesmo nome de outra coisa.
    types
        .into_iter()
        .filter_map(|(class, krate, ty)| {
            let found = docs.iter().find(|((k, t), d)| *t == ty && k.starts_with(&krate) && !d.is_empty()).map(|(_, d)| d.clone());
            found.map(|d| (class, d))
        })
        .collect()
}

/// O comentário do código como texto da página: `[`X`](link)` e `[`X`]` viram `X`, os parênteses
/// que comparam com o Delphi/FMX (`(como o `TTreeView`)`) e o "Veja o módulo" saem.
fn clean_description(text: &str) -> String {
    let mut out = String::new();
    let mut rest = text;
    // Links: `[texto](destino)` → texto; `[texto]` → texto.
    while let Some(start) = rest.find('[') {
        out.push_str(&rest[..start]);
        let after = &rest[start + 1..];
        let Some(end) = after.find(']') else {
            out.push_str(&rest[start..]);
            rest = "";
            break;
        };
        out.push_str(&after[..end]);
        rest = &after[end + 1..];
        if rest.starts_with('(') {
            rest = rest.find(')').map_or("", |close| &rest[close + 1..]);
        }
    }
    out.push_str(rest);
    // Parênteses com comparações de outras bibliotecas.
    let mut cleaned = String::new();
    let mut rest = out.as_str();
    while let Some(open) = rest.find(" (") {
        let Some(close) = rest[open..].find(')').map(|c| open + c) else { break };
        let inside = &rest[open + 2..close];
        let foreign = inside.contains("FMX") || inside.contains("Delphi") || inside.contains("VCL") || inside.split('`').skip(1).step_by(2).any(|code| code.len() > 1 && code.starts_with('T') && code.chars().nth(1).is_some_and(char::is_uppercase));
        cleaned.push_str(&rest[..open]);
        if !foreign {
            cleaned.push_str(&rest[open..=close]);
        }
        rest = &rest[close + 1..];
    }
    cleaned.push_str(rest);
    // E fora de parênteses: ", como o `TLottieAnimation`" (com ou sem "do FMX").
    let mut cleaned = cleaned;
    for lead in [", como o `T", ", como a `T", " como o `T"] {
        while let Some(at) = cleaned.find(lead) {
            let code_start = at + lead.len() - 1;
            let Some(end) = cleaned[code_start + 1..].find('`').map(|e| code_start + 1 + e + 1) else { break };
            let tail = if cleaned[end..].starts_with(" do FMX") { end + " do FMX".len() } else { end };
            cleaned.replace_range(at..tail, "");
        }
    }
    let cleaned = cleaned.replace("Veja o módulo.", "").replace("Veja o módulo", "");
    cleaned.split_whitespace().collect::<Vec<_>>().join(" ").replace(" .", ".").replace(" ,", ",")
}

fn include_sources(text: &str, root: &Path) -> String {
    let mut out = String::new();
    for line in text.split_inclusive('\n') {
        let path = line.trim().strip_prefix("<!-- incluir:").and_then(|r| r.strip_suffix("-->")).map(str::trim);
        let Some(source) = path.and_then(|p| std::fs::read_to_string(root.join(p)).ok()) else {
            out.push_str(line);
            continue;
        };
        let code = source.split("\n#[cfg(test)]").next().unwrap_or_default().trim_end();
        let lang = if path.is_some_and(|p| p.ends_with(".toml")) { "toml" } else if path.is_some_and(|p| p.ends_with(".json")) { "json" } else { "rust" };
        let _ = writeln!(out, "```{lang}\n{code}\n```");
    }
    out
}
