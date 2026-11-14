//! Diálogos da IDE, feitos com o próprio aylis: cada um é um form aberto com
//! `show_modal` (a IDE fica bloqueada até ele fechar) e liberado ao fechar.

use std::cell::RefCell;
use std::rc::Rc;

use aylis_controls::{
    Button, CheckBox, ColorPicker, ComboBox, Edit, Form, Label, Layout, ProgressBar, RadioButton, Rectangle, ScrollBox, Switch, TrackBar,
};
use aylis_core::{Align, Color, ComponentId, EventArgs, ModalResult, PropValue, Ui};
use aylis_core::gpu::GpuChoice;
use aylis_core::ImageCacheLimits;
use aylis_project::{ImageCacheSettings, Language, Platform, Project, RenderChoice, RenderOptions};

use crate::widgets::{label, muted};

/// Um diálogo montado: o form, a área de conteúdo, o rodapé e o label de erro. Embutido
/// ([`embed`]), `form` é a página onde ele foi montado.
pub struct Dialog {
    pub form: ComponentId,
    pub body: ComponentId,
    pub footer: ComponentId,
    pub error: ComponentId,
    embedded: bool,
}

/// O que um diálogo embutido deixa para quem o embutiu: o label de erro dele, o conferir e
/// o aplicar (o `validate` e o `on_ok` do [`run`]).
pub struct Embedded {
    pub error: ComponentId,
    pub validate: Box<dyn Fn(&mut Ui) -> Result<(), String>>,
    pub apply: Box<dyn FnOnce(&mut Ui)>,
}

thread_local! {
    /// A página onde o próximo [`new_dialog`] monta, em vez de abrir uma janela.
    static EMBED_IN: std::cell::Cell<Option<ComponentId>> = const { std::cell::Cell::new(None) };
    /// O que o [`run`] do diálogo embutido deixou.
    static EMBEDDED: RefCell<Option<Embedded>> = const { RefCell::new(None) };
}

/// Roda `open` (o editor de uma seção, que monta um diálogo com [`new_dialog`] e [`run`])
/// montando o diálogo dentro de `page`, sem janela nem OK/Cancelar: as abas das Opções do
/// projeto. Devolve o conferir e o aplicar dele, ou `None` se ele não montou diálogo (uma
/// seção que só pergunta, ou que não abriu). Só o primeiro diálogo vai para a página; os
/// que ele abrir depois (escolher uma cor...) são janelas.
pub fn embed(page: ComponentId, open: impl FnOnce()) -> Option<Embedded> {
    EMBED_IN.with(|e| e.set(Some(page)));
    EMBEDDED.with(|e| e.borrow_mut().take());
    open();
    EMBED_IN.with(|e| e.set(None));
    EMBEDDED.with(|e| e.borrow_mut().take())
}

/// Monta um form de diálogo (borda fixa, centralizado no dono) com OK e Cancelar.
pub fn new_dialog(ui: &mut Ui, title: &str, width: f32, height: f32) -> Dialog {
    if let Some(page) = EMBED_IN.with(|e| e.take()) {
        // Na página: a altura pedida (sem a moldura da janela), para rolar se não couber; o
        // rodapé só com o erro e o que o diálogo puser nele.
        ui.control_mut(page).unwrap().set_height((height - 60.0).max(200.0));
        let footer = ui.add(Layout::default(), page);
        ui.control_mut(footer).unwrap().set_align(Align::Bottom).set_height(36.0).set_margins([0.0, 12.0, 0.0, 0.0]);
        let error = ui.add(Label::new(""), footer);
        ui.control_mut(error).unwrap().set_align(Align::Client);
        let danger = ui.theme().color("danger");
        let _ = ui.set_prop(error, "color", danger);
        let body = ui.add(Layout::default(), page);
        ui.control_mut(body).unwrap().set_align(Align::Client);
        let _ = (title, width);
        return Dialog { form: page, body, footer, error, embedded: true };
    }
    let form = ui.add_root(Form::new(title, width, height));
    let _ = ui.set_prop(form, "border_style", "single");
    ui.control_mut(form).unwrap().set_padding(16.0);
    let footer = ui.add(Layout::default(), form);
    ui.control_mut(footer).unwrap().set_align(Align::Bottom).set_height(36.0).set_margins([0.0, 12.0, 0.0, 0.0]);
    // Com align right, o primeiro fica mais à direita: [OK] [Cancelar].
    for (text, result, style) in [("Cancelar", "cancel", "flat"), ("OK", "ok", "")] {
        let b = ui.add(Button::new(text), footer);
        ui.control_mut(b).unwrap().set_align(Align::Right).set_width(110.0).set_margins([8.0, 0.0, 0.0, 0.0]).set_style(style);
        let _ = ui.set_prop(b, "modal_result", result);
    }
    let error = ui.add(Label::new(""), footer);
    ui.control_mut(error).unwrap().set_align(Align::Client);
    let danger = ui.theme().color("danger");
    let _ = ui.set_prop(error, "color", danger);
    let body = ui.add(Layout::default(), form);
    ui.control_mut(body).unwrap().set_align(Align::Client);
    Dialog { form, body, footer, error, embedded: false }
}

/// Abre o diálogo como modal. Com OK, `validate` confere os dados (um erro mantém o
/// diálogo aberto e aparece no rodapé) e `on_ok` aplica. O form é liberado ao fechar.
pub fn run(
    ui: &mut Ui,
    dialog: Dialog,
    validate: impl Fn(&mut Ui) -> Result<(), String> + 'static,
    on_ok: impl FnOnce(&mut Ui) + 'static,
) {
    let error = dialog.error;
    if dialog.embedded {
        crate::lang::translate(ui);
        EMBEDDED.with(|e| *e.borrow_mut() = Some(Embedded { error, validate: Box::new(validate), apply: Box::new(on_ok) }));
        return;
    }
    ui.on(dialog.form, "on_close_query", move |ui, form, _| {
        if ui.modal_result(form) == ModalResult::Ok {
            if let Err(message) = validate(ui) {
                let _ = ui.set_prop(error, "text", format!("⚠ {message}"));
                ui.cancel_close();
            }
        }
    });
    let form = dialog.form;
    // Enter é o OK e Esc o Cancelar (o que nenhum controle usou chega ao form).
    ui.on(form, "on_key_down", move |ui, sender, args| {
        let EventArgs::Key(event) = args else { return };
        if sender != form || event.modifiers.ctrl || event.modifiers.alt {
            return;
        }
        match event.key {
            aylis_core::Key::Enter => ui.set_modal_result(form, ModalResult::Ok),
            aylis_core::Key::Escape => ui.set_modal_result(form, ModalResult::Cancel),
            _ => {}
        }
    });
    crate::lang::translate(ui);
    let _ = ui.show_modal(form, move |ui, result| {
        if result == ModalResult::Ok {
            on_ok(ui);
        }
        ui.free(form);
    });
}

/// O Esc fecha a janela modal (como o Cancelar), nas que não passam pelo `run`: a galeria, os
/// não visuais, as gravações... (o que nenhum controle usou chega ao form, como no `run`).
pub fn close_on_escape(ui: &mut Ui, form: ComponentId) {
    ui.on(form, "on_key_down", move |ui, sender, args| {
        let EventArgs::Key(event) = args else { return };
        if sender == form && matches!(event.key, aylis_core::Key::Escape) && !event.modifiers.ctrl && !event.modifiers.alt {
            ui.set_modal_result(form, ModalResult::Cancel);
        }
    });
}

/// Uma linha "rótulo: editor" no corpo do diálogo. Devolve o contêiner do editor.
pub fn field(ui: &mut Ui, body: ComponentId, text: &str) -> ComponentId {
    let line = ui.add(Layout::default(), body);
    ui.control_mut(line).unwrap().set_align(Align::Top).set_height(34.0).set_margins([0.0, 0.0, 0.0, 8.0]);
    let l = label(ui, line, text, Align::Left, 130.0);
    muted(ui, l);
    line
}

pub fn edit(ui: &mut Ui, parent: ComponentId, text: &str) -> ComponentId {
    let e = ui.add(Edit::new(text), parent);
    ui.control_mut(e).unwrap().set_align(Align::Client);
    e
}

pub fn text_of(ui: &Ui, id: ComponentId) -> String {
    ui.get::<Edit>(id).map(|e| e.text.trim().to_string()).unwrap_or_default()
}

pub fn combo_text(ui: &Ui, id: ComponentId) -> String {
    ui.get::<ComboBox>(id).map(|c| c.text().to_string()).unwrap_or_default()
}

pub fn combo(ui: &mut Ui, parent: ComponentId, items: &[&str], selected: &str) -> ComponentId {
    let c = ui.add(ComboBox::new(items), parent);
    ui.control_mut(c).unwrap().set_align(Align::Client);
    if let Some(i) = items.iter().position(|i| i.eq_ignore_ascii_case(selected)) {
        let _ = ui.set_prop(c, "item_index", i as i64);
    }
    c
}

// ---------------------------------------------------------------------------

/// Os combos da seção "Renderizador por plataforma".
pub struct RenderInputs {
    combos: Vec<(Platform, ComponentId)>,
}

/// O texto de uma escolha no combo (a recomendada para a plataforma vem marcada).
fn choice_label(platform: Platform, choice: RenderChoice) -> String {
    let recommended = platform.choices().first() == Some(&choice) && platform.choices().len() > 1;
    if recommended { format!("{} (recomendado)", choice.title()) } else { choice.title().to_string() }
}

/// Seção "Renderizador por plataforma": um combo por sistema e, embaixo, o que esperar da
/// escolha (tamanho do executável, velocidade e memória medidos).
pub fn render_section(ui: &mut Ui, body: ComponentId, render: RenderOptions) -> RenderInputs {
    crate::widgets::section(ui, body, "RENDERIZADOR POR PLATAFORMA (pode trocar depois em Projeto → Opções)");
    let mut combos = Vec::new();
    for platform in Platform::ALL {
        let line = field(ui, body, platform.title());
        let labels: Vec<String> = platform.choices().iter().map(|c| choice_label(platform, *c)).collect();
        let refs: Vec<&str> = labels.iter().map(String::as_str).collect();
        let selected = choice_label(platform, render.get(platform));
        let c = combo(ui, line, &refs, &selected);
        combos.push((platform, c));
    }
    let info = label(ui, body, "", Align::Top, 58.0);
    muted(ui, info);
    let _ = ui.set_prop(info, "font_size", 12);
    let _ = ui.set_prop(info, "wrap", true);
    let _ = ui.set_prop(info, "v_align", PropValue::Enum("leading".into()));
    let show = move |ui: &mut Ui, platform: Platform, combo: ComponentId| {
        let index = ui.get::<ComboBox>(combo).map_or(0, |c| c.item_index.max(0) as usize);
        let choice = platform.choices().get(index).copied().unwrap_or_default();
        // Cada pedaço pela tradução (em inglês saía o texto montado em português; conferência de 08/11).
        let text = format!("{} → {}: {} ({})", platform.title(), ui.tr(choice.title()), ui.tr(choice.summary()), ui.tr("medido no Windows; veja docs/renderizadores.md"));
        let _ = ui.set_prop(info, "text", text);
    };
    if let Some(&(platform, c)) = combos.first() {
        show(ui, platform, c);
    }
    for &(platform, c) in &combos {
        ui.on(c, "on_change", move |ui, _, _| show(ui, platform, c));
    }
    RenderInputs { combos }
}

/// As escolhas da seção.
pub fn render_value(ui: &Ui, inputs: &RenderInputs) -> RenderOptions {
    let mut render = RenderOptions::default();
    for &(platform, c) in &inputs.combos {
        let index = ui.get::<ComboBox>(c).map_or(0, |c| c.item_index.max(0) as usize);
        if let Some(choice) = platform.choices().get(index) {
            render.set(platform, *choice);
        }
    }
    render
}

/// As opções de texto do projeto (quem mede e desenha o texto em cada plataforma, item 10).
pub fn text_section(ui: &mut Ui, body: ComponentId, text: aylis_project::TextOptions) -> Vec<(Option<Platform>, ComponentId)> {
    use aylis_project::TextChoice;
    crate::widgets::section(ui, body, "TEXTO POR PLATAFORMA (o mesmo texto em todos os renderizadores: veja docs/fontes-e-icones.md)");
    let labels = ["Do renderizador (native)", "O núcleo mede, o renderizador desenha (layout)", "O núcleo mede e desenha (core)"];
    let label_of = |c: TextChoice| labels[match c { TextChoice::Native => 0, TextChoice::Layout => 1, TextChoice::Core => 2 }];
    let platforms: Vec<(Option<Platform>, &str)> = Platform::ALL.iter().map(|p| (Some(*p), p.title())).chain([(None, "Web")]).collect();
    let mut combos = Vec::new();
    for (platform, title) in platforms {
        let line = field(ui, body, title);
        let c = combo(ui, line, &labels, label_of(text.get(platform)));
        combos.push((platform, c));
    }
    combos
}

/// O que as caixas de [`text_section`] escolheram.
pub fn text_value(ui: &Ui, combos: &[(Option<Platform>, ComponentId)]) -> aylis_project::TextOptions {
    use aylis_project::TextChoice;
    let mut text = aylis_project::TextOptions::legacy();
    for &(platform, c) in combos {
        let choice = match ui.get::<ComboBox>(c).map_or(0, |c| c.item_index.max(0)) {
            1 => TextChoice::Layout,
            2 => TextChoice::Core,
            _ => TextChoice::Native,
        };
        text.set(platform, choice);
    }
    text
}

/// Pede um texto (nome de form, nome de handler...).
pub fn input_text(
    ui: &mut Ui,
    title: &str,
    prompt: &str,
    initial: &str,
    validate: impl Fn(&str) -> Result<(), String> + 'static,
    on_ok: impl FnOnce(&mut Ui, String) + 'static,
) {
    let d = new_dialog(ui, title, 440.0, 150.0);
    let line = field(ui, d.body, prompt);
    let e = edit(ui, line, initial);
    run(ui, d, move |ui| validate(&text_of(ui, e)), move |ui| {
        let text = text_of(ui, e);
        on_ok(ui, text);
    });
    ui.set_focus(e);
}

/// "Opções do projeto": nome, linguagem, form principal, tema, idioma, renderizador de
/// cada plataforma e quais forms são criados no início.
/// `ide_locale`: o idioma da IDE como idioma de app (`pt-BR`), o sugerido.
/// As escolhas fixas do combo "Placa de vídeo" (depois vêm as placas pelo nome). "Seguir o
/// monitor" troca de placa quando a janela passa para um monitor da outra (Windows).
pub const GPU_CHOICES: [&str; 4] = ["Automática (a do sistema)", "Alto desempenho (a dedicada)", "Economia de energia (a integrada)", "Seguir o monitor (a placa de cada tela)"];

/// As escolhas do combo "Apresentação (Windows)": a swap chain da janela (o padrão) e o
/// DirectComposition.
pub const PRESENT_CHOICES: [&str; 2] = ["Janela (como os jogos)", "DirectComposition (como os navegadores)"];

/// O que vai para o `.aylisproj` (`"gpu"`) pelo texto do combo.
pub fn gpu_value(text: &str) -> String {
    match GPU_CHOICES.iter().position(|c| *c == text) {
        Some(0) => String::new(),
        Some(1) => GpuChoice::HighPerformance.to_text(),
        Some(2) => GpuChoice::LowPower.to_text(),
        Some(3) => GpuChoice::FollowMonitor.to_text(),
        _ => text.to_string(),
    }
}

pub fn project_options(ui: &mut Ui, project: &Project, ide_locale: &str, on_ok: impl FnOnce(&mut Ui, Project) + 'static) {
    let d = new_dialog(ui, "Opções do projeto · Geral", 640.0, 1310.0);
    let line = field(ui, d.body, "Nome");
    let name = edit(ui, line, &project.name);
    let line = field(ui, d.body, "Linguagem");
    let names: Vec<&str> = Language::ALL.iter().map(|l| l.name()).collect();
    let lang = combo(ui, line, &names, project.language.name());
    // As plataformas de destino (vazio no projeto = todas), em duas linhas de três.
    let mut platform_checks = Vec::new();
    for (i, chunk) in aylis_project::new_project::TARGETS.chunks(3).enumerate() {
        let line = field(ui, d.body, if i == 0 { "Plataformas" } else { "" });
        for (key, title) in chunk {
            let c = ui.add(CheckBox::new(*title), line);
            ui.control_mut(c).unwrap().set_align(Align::Left).set_width(160.0);
            let _ = ui.set_prop(c, "checked", project.platforms.is_empty() || project.platforms.iter().any(|p| p == key));
            platform_checks.push((*key, c));
        }
    }
    let forms: Vec<String> = project.forms.iter().map(|f| f.name.clone()).collect();
    let form_refs: Vec<&str> = forms.iter().map(String::as_str).collect();
    let line = field(ui, d.body, "Form principal");
    let main = combo(ui, line, &form_refs, &project.main_form);
    // O tema fica em Projeto → Opções do projeto → Aparência (família, modo, mistura por classe).
    let line = field(ui, d.body, "Tema");
    let current = if project.theme.is_empty() { "Aylis Light".to_string() } else if project.theme == "dark" { "Aylis Dark".to_string() } else { project.theme.clone() };
    let change = ui.tr("(troque em Opções do projeto → Aparência)");
    let t = label(ui, line, &format!("{current}   {change}"), Align::Client, 0.0);
    muted(ui, t);
    let line = field(ui, d.body, "Idioma");
    // O idioma da IDE primeiro (é o sugerido); o do projeto e os traduzidos, se outros.
    let ide_locale = ide_locale.to_string();
    let mut languages = vec![ide_locale.clone()];
    for l in [project.locale.clone(), "pt-BR".into(), "en-US".into(), "es-ES".into()] {
        if !l.is_empty() && !languages.contains(&l) {
            languages.push(l);
        }
    }
    for l in aylis_project::i18n::languages(project) {
        if !languages.contains(&l) {
            languages.push(l);
        }
    }
    let language_refs: Vec<&str> = languages.iter().map(String::as_str).collect();
    let locale = combo(ui, line, &language_refs, if project.locale.is_empty() { &ide_locale } else { &project.locale });
    // O fuso do app (o `time_zone`): vazio, o do aparelho; com nome, todos veem a hora da empresa.
    let line = field(ui, d.body, "Fuso horário");
    let time_zone = edit(ui, line, &project.time_zone);
    let _ = ui.set_prop(time_zone, "placeholder", "(o do aparelho)   ex.: America/Sao_Paulo");
    let render = render_section(ui, d.body, project.render());
    let text_modes = text_section(ui, d.body, project.text());
    // A placa de vídeo: as quatro escolhas fixas e as desta máquina pelo nome.
    let line = field(ui, d.body, "Placa de vídeo");
    let mut gpus: Vec<String> = GPU_CHOICES.iter().map(|s| s.to_string()).collect();
    gpus.extend(crate::host::gpu_adapters());
    let selected = match GpuChoice::parse(&project.gpu) {
        GpuChoice::Auto => gpus[0].clone(),
        GpuChoice::HighPerformance => gpus[1].clone(),
        GpuChoice::LowPower => gpus[2].clone(),
        GpuChoice::FollowMonitor => gpus[3].clone(),
        GpuChoice::Named(name) => {
            if !gpus.contains(&name) {
                gpus.push(name.clone());
            }
            name
        }
    };
    let gpu_refs: Vec<&str> = gpus.iter().map(String::as_str).collect();
    let gpu = combo(ui, line, &gpu_refs, &selected);
    // Como o quadro chega à janela no Windows (o `"present"`): a swap chain da janela ou o
    // DirectComposition.
    let line = field(ui, d.body, "Apresentação (Windows)");
    let present_now = PRESENT_CHOICES[usize::from(aylis_core::gpu::PresentMode::parse(&project.present) == aylis_core::gpu::PresentMode::Composition)];
    let present = combo(ui, line, &PRESENT_CHOICES, present_now);
    let _ = ui.set_prop(
        present,
        "hint",
        "Janela: como os jogos, a menor latência em tela cheia. DirectComposition: como os navegadores, sempre pelo compositor do Windows.",
    );
    // Onde o Executar põe e roda o executável (ao lado das DLLs que o app precisa).
    let line = field(ui, d.body, "Pasta de saída");
    let out_dir = edit(ui, line, &project.output.dir);
    let _ = ui.set_prop(out_dir, "placeholder", "(o target do cargo)   ex.: rust/exe");
    let line = field(ui, d.body, "Nome do executável");
    let out_name = edit(ui, line, &project.output.name);
    let _ = ui.set_prop(out_name, "placeholder", &*format!("(o do cargo: {})", project.package_name()));
    // O binário que o Compilar, o Executar e o Publicar usam (I08 do Tabernáculo).
    let line = field(ui, d.body, "Binário (cargo --bin)");
    let out_bin = edit(ui, line, &project.output.bin);
    let _ = ui.set_prop(out_bin, "placeholder", "(o [[bin]] do Cargo.toml, ou o pacote)");
    // Rust: quantos dias o que é só deste projeto fica na pasta de compilação.
    let line = field(ui, d.body, "Limpeza (dias)");
    let cleanup_days = edit(ui, line, &project.output.cleanup_days.map(|d| d.to_string()).unwrap_or_default());
    let _ = ui.set_prop(
        cleanup_days,
        "placeholder",
        &*format!("(o das Opções da IDE: {})   projetos em Rust: o compilado só deste projeto sai depois desses dias sem compilar", aylis_tools::cleanup::Policy::from_settings().project_days),
    );
    // Encerrar o app já aberto antes de executar, e os comandos antes e depois da compilação
    // (pedidos do autor em 08/11; os comandos só rodam em projeto confiado).
    let line = field(ui, d.body, "");
    let close_running = ui.add(CheckBox::new("Encerrar o app aberto antes de executar"), line);
    ui.control_mut(close_running).unwrap().set_align(Align::Client);
    let _ = ui.set_prop(close_running, "checked", project.output.close_running);
    let _ = ui.set_prop(close_running, "hint", "Sem isso, com o app já aberto (o instalado, por exemplo), o Executar avisa e não compila");
    let commands_field = |ui: &mut Ui, title: &str, list: &[aylis_project::BuildCommand]| {
        let line = field(ui, d.body, title);
        ui.control_mut(line).unwrap().set_height(64.0);
        let m = ui.add(aylis_controls::Memo::new(&build_commands_text(list)), line);
        ui.control_mut(m).unwrap().set_align(Align::Client);
        let _ = ui.set_prop(m, "placeholder", "um comando por linha (cmd); \"ps: \" na frente roda no PowerShell");
        let _ = ui.set_prop(m, "hint", "Rodam na pasta do projeto, só em projeto confiado; o executável fica em %AYLIS_OUTPUT_EXE%. Um que falha para a compilação.");
        m
    };
    let before_build = commands_field(ui, "Antes de compilar", &project.output.before_build);
    let after_build = commands_field(ui, "Depois de compilar", &project.output.after_build);
    // O canal de desenvolvimento: o app que a IDE compila conecta neste endereço.
    let line = field(ui, d.body, "Endereço da IDE");
    let dev_address = edit(ui, line, &project.dev_address);
    let _ = ui.set_prop(dev_address, "placeholder", "127.0.0.1:7342   (o IP desta máquina para um celular sem cabo)");
    // De onde vem o Aylis no Cargo.toml (G11 do Tabernáculo): o crates.io ou uma pasta fixa.
    let line = field(ui, d.body, "Aylis do projeto");
    let sdk = edit(ui, line, &project.sdk);
    let _ = ui.set_prop(sdk, "placeholder", "crates.io, ou a pasta de um Aylis fixo (vazio: o instalado nesta máquina)");
    crate::widgets::section(ui, d.body, "CRIAR NO INÍCIO (os outros, com ui.create_form)");
    let list = ui.add(ScrollBox::default(), d.body);
    ui.control_mut(list).unwrap().set_align(Align::Client);
    let mut checks = Vec::new();
    for f in &project.forms {
        let c = ui.add(CheckBox::new(format!("{}   ({})", f.name, f.file)), list);
        ui.control_mut(c).unwrap().set_align(Align::Top).set_height(28.0);
        let _ = ui.set_prop(c, "checked", f.auto_create);
        checks.push(c);
    }
    let platforms_checked = platform_checks.clone();
    let original = project.clone();
    run(
        ui,
        d,
        move |ui| {
            if text_of(ui, name).is_empty() {
                return Err("informe o nome".into());
            }
            if !platforms_checked.iter().any(|(_, c)| ui.get::<CheckBox>(*c).is_some_and(|c| c.checked)) {
                return Err("marque ao menos uma plataforma".into());
            }
            let address = text_of(ui, dev_address);
            let address = address.trim();
            let valid = address.is_empty() || address.rsplit_once(':').is_some_and(|(host, port)| !host.is_empty() && port.parse::<u16>().is_ok_and(|p| p > 0));
            if !valid {
                return Err("o endereço da IDE é host:porta (ex.: 192.168.0.10:7342)".into());
            }
            // Um nome IANA: Região/Cidade (America/Sao_Paulo), sem espaços; ou UTC.
            let zone = text_of(ui, time_zone);
            let zone = zone.trim();
            let named = zone.is_empty() || zone == "UTC" || (zone.contains('/') && !zone.contains(char::is_whitespace) && !zone.starts_with('/') && !zone.ends_with('/'));
            if named { Ok(()) } else { Err("o fuso é um nome IANA (ex.: America/Sao_Paulo)".into()) }
        },
        move |ui| {
            let mut p = original;
            p.name = text_of(ui, name);
            p.language = Language::from_name(&combo_text(ui, lang)).unwrap_or_default();
            p.main_form = combo_text(ui, main);
            p.locale = combo_text(ui, locale);
            p.time_zone = text_of(ui, time_zone).trim().to_string();
            p.render = Some(render_value(ui, &render));
            // Sem `text` no arquivo e nada mudado, continua sem (antes, aplicar gravava o padrão).
            let text = text_value(ui, &text_modes);
            p.text = if p.text.is_none() && text == aylis_project::TextOptions::legacy() { None } else { Some(text) };
            // Os itens vêm traduzidos na tela: a escolha de lista volta ao texto original (a placa
            // também aceita um nome digitado).
            let gpu_text = combo_text(ui, gpu);
            let gpu_text = GPU_CHOICES.iter().find(|c| ui.tr(c) == gpu_text).map_or(gpu_text, |c| c.to_string());
            p.gpu = gpu_value(&gpu_text);
            p.present = if ui.get::<ComboBox>(present).is_some_and(|c| c.item_index == 1) { aylis_core::gpu::PresentMode::Composition } else { aylis_core::gpu::PresentMode::Window }
                .name()
                .to_string();
            p.output.dir = text_of(ui, out_dir).trim().to_string();
            p.output.name = text_of(ui, out_name).trim().trim_end_matches(".exe").to_string();
            p.output.bin = text_of(ui, out_bin).trim().to_string();
            p.output.cleanup_days = text_of(ui, cleanup_days).parse().ok().filter(|d| *d > 0);
            p.output.close_running = ui.get::<CheckBox>(close_running).is_some_and(|c| c.checked);
            p.output.before_build = parse_build_commands(&memo_text(ui, before_build));
            p.output.after_build = parse_build_commands(&memo_text(ui, after_build));
            p.dev_address = text_of(ui, dev_address).trim().to_string();
            p.sdk = text_of(ui, sdk).trim().to_string();
            let chosen: Vec<String> = platform_checks.iter().filter(|(_, c)| ui.get::<CheckBox>(*c).is_some_and(|c| c.checked)).map(|(k, _)| k.to_string()).collect();
            p.platforms = if chosen.len() == platform_checks.len() { Vec::new() } else { chosen };
            for (f, c) in p.forms.iter_mut().zip(&checks) {
                f.auto_create = ui.get::<CheckBox>(*c).is_some_and(|c| c.checked) || f.name == p.main_form;
            }
            on_ok(ui, p);
        },
    );
}

/// Os comandos da compilação como texto do campo: um por linha, `ps: ` nos do PowerShell.
fn build_commands_text(list: &[aylis_project::BuildCommand]) -> String {
    list.iter()
        .map(|c| match c.shell {
            aylis_project::BuildShell::Cmd => c.command.clone(),
            aylis_project::BuildShell::PowerShell => format!("ps: {}", c.command),
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// O texto do campo de volta aos comandos (linhas vazias saem; `cmd: ` também vale).
fn parse_build_commands(text: &str) -> Vec<aylis_project::BuildCommand> {
    text.lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(|l| {
            let lower = l.to_lowercase();
            if lower.starts_with("ps:") || lower.starts_with("powershell:") {
                aylis_project::BuildCommand { shell: aylis_project::BuildShell::PowerShell, command: l.split_once(':').map_or(l, |(_, c)| c).trim().to_string() }
            } else if lower.starts_with("cmd:") {
                aylis_project::BuildCommand { shell: aylis_project::BuildShell::Cmd, command: l[4..].trim().to_string() }
            } else {
                aylis_project::BuildCommand { shell: aylis_project::BuildShell::Cmd, command: l.to_string() }
            }
        })
        .collect()
}

fn memo_text(ui: &Ui, memo: ComponentId) -> String {
    ui.get::<aylis_controls::Memo>(memo).map(|m| m.text.clone()).unwrap_or_default()
}

#[cfg(test)]
mod build_command_tests {
    use super::*;

    #[test]
    fn the_build_commands_round_trip_through_the_field() {
        let list = parse_build_commands("copy a.dll %AYLIS_PROJECT_DIR%\\out\n\nps: Write-Host \"pronto\"\ncmd: echo ok");
        assert_eq!(list.len(), 3);
        assert_eq!(list[1].shell, aylis_project::BuildShell::PowerShell);
        assert_eq!(list[1].command, "Write-Host \"pronto\"");
        assert_eq!(list[2].command, "echo ok");
        assert_eq!(parse_build_commands(&build_commands_text(&list)), list);
    }
}

// ---------------------------------------------------------------------------
// Memória do projeto

/// Opções do projeto → Memória: os limites do cache de imagens (o `image_cache`), cada um
/// com o que guarda, o que acontece ao passar dele e quando aumentar. `on_ok` recebe `None`
/// quando todos ficam no padrão (o `.aylisproj` não grava a chave).
pub fn project_memory(ui: &mut Ui, project: &Project, on_ok: impl FnOnce(&mut Ui, Option<ImageCacheSettings>) + 'static) {
    let d = new_dialog(ui, "Opções do projeto · Memória", 640.0, 600.0);
    let current = project.image_cache.unwrap_or_default();
    let defaults = ImageCacheLimits::DEFAULT;
    let intro = label(
        ui,
        d.body,
        "Os renderizadores guardam as imagens já prontas (decodificadas e na placa de vídeo) para não refazer a cada pintura. O que o app solta sai sozinho: estes limites contam só para as imagens que ele ainda segura. Passar de um limite não erra nada na tela; a imagem usada há mais tempo sai e é refeita quando aparecer de novo, o que deixa a pintura mais lenta.",
        Align::Top,
        96.0,
    );
    let _ = ui.set_prop(intro, "wrap", true);
    let number = |ui: &mut Ui, body: ComponentId, title: &str, value: u32, default: u32, unit: &str, help: &str| {
        let line = field(ui, body, title);
        let e = ui.add(Edit::new(value.to_string()), line);
        ui.control_mut(e).unwrap().set_align(Align::Left).set_width(120.0);
        let _ = ui.set_prop(e, "placeholder", default.to_string());
        let u = label(ui, line, unit, Align::Left, 70.0);
        ui.control_mut(u).unwrap().set_margins([10.0, 0.0, 0.0, 0.0]);
        let dflt = label(ui, line, &format!("padrão: {}", default), Align::Client, 0.0);
        muted(ui, dflt);
        let h = label(ui, body, help, Align::Top, 78.0);
        ui.control_mut(h).unwrap().set_margins([130.0, 0.0, 0.0, 12.0]);
        muted(ui, h);
        let _ = ui.set_prop(h, "wrap", true);
        let _ = ui.set_prop(h, "font_size", 12);
        e
    };
    let pixels = number(
        ui,
        d.body,
        "Imagens em pixels",
        current.pixels_mb,
        defaults.pixels_mb,
        "MB",
        "Quadros de vídeo e de câmera, prévias de tela, os blocos de um desenho (PaintCanvas): tudo o que o app gera como pixels. Precisa caber o que aparece junto na tela; um desenho 4K inteiro visível ocupa ~33 MB. Aumente se a pintura ficar lenta com muitas dessas imagens à vista.",
    );
    let files = number(
        ui,
        d.body,
        "Imagens de arquivo",
        current.files,
        defaults.files,
        "imagens",
        "PNG, JPEG, SVG e outros arquivos de imagem, decodificados e na placa. Precisa caber os que aparecem juntos numa tela (ícones, miniaturas, uma galeria). Aumente se uma tela com muitas imagens ficar lenta ao rolar ou repintar.",
    );
    let layers = number(
        ui,
        d.body,
        "Camadas guardadas",
        current.layers_mb,
        defaults.layers_mb,
        "MB",
        "O desenho de um controle com cache_layer, ou das páginas numa transição, guardado como imagem para andar e esmaecer sem repintar os filhos. Uma transição de aba em 4K usa ~66 MB enquanto anda; a imagem sai quando a transição acaba. Com 0, nada fica guardado e tudo é repintado a cada quadro.",
    );
    let line = field(ui, d.body, "");
    let auto = ui.add(CheckBox::new("Camadas automáticas nas transições e animações"), line);
    ui.control_mut(auto).unwrap().set_align(Align::Client);
    let _ = ui.set_prop(auto, "checked", current.auto_layers);
    let _ = ui.set_prop(auto, "hint", "Vale em todos os forms; cada form pode trocar pela propriedade layer_cache");
    let read = |ui: &Ui, id: ComponentId, default: u32| {
        let text = text_of(ui, id);
        if text.is_empty() { Some(default) } else { text.parse::<u32>().ok() }
    };
    run(
        ui,
        d,
        move |ui| {
            let (min_mb, max_mb) = ImageCacheLimits::PIXELS_MB_RANGE;
            let (min_files, max_files) = ImageCacheLimits::FILES_RANGE;
            match read(ui, pixels, defaults.pixels_mb) {
                Some(v) if (min_mb..=max_mb).contains(&v) => {}
                _ => return Err(format!("imagens em pixels: de {} a {} MB", min_mb, max_mb)),
            }
            match read(ui, files, defaults.files) {
                Some(v) if (min_files..=max_files).contains(&v) => {}
                _ => return Err(format!("imagens de arquivo: de {} a {}", min_files, max_files)),
            }
            let (min_layers, max_layers) = ImageCacheLimits::LAYERS_MB_RANGE;
            match read(ui, layers, defaults.layers_mb) {
                Some(v) if (min_layers..=max_layers).contains(&v) => Ok(()),
                _ => Err(format!("camadas guardadas: de {} a {} MB", min_layers, max_layers)),
            }
        },
        move |ui| {
            let settings = ImageCacheSettings {
                pixels_mb: read(ui, pixels, defaults.pixels_mb).unwrap_or(defaults.pixels_mb),
                files: read(ui, files, defaults.files).unwrap_or(defaults.files),
                layers_mb: read(ui, layers, defaults.layers_mb).unwrap_or(defaults.layers_mb),
                auto_layers: ui.get::<CheckBox>(auto).is_none_or(|c| c.checked),
            };
            on_ok(ui, (settings != ImageCacheSettings::default()).then_some(settings));
        },
    );
}

// ---------------------------------------------------------------------------
// Aparência do projeto

/// Uma linha da lista "quem usa qual tema".
pub struct UsageRow {
    pub id: ComponentId,
    /// "btnSalvar: Button".
    pub label: String,
    /// Tema com que é pintado agora.
    pub theme: String,
    /// De onde vem ("do projeto", "próprio", "herdado de pnlLateral", "mistura da classe Button").
    pub source: String,
    /// A propriedade `theme` do próprio componente (vazia = herda).
    pub own: String,
}

/// O que o diálogo de aparência devolve.
pub struct Appearance {
    /// O `theme` do projeto ("", "dark", "Material 3 Light", "themes/empresa.json"...).
    pub theme: String,
    pub mix: std::collections::BTreeMap<String, String>,
    /// Componentes que mudaram o tema próprio: (id, tema; vazio = herdar).
    pub own: Vec<(ComponentId, String)>,
}

const INHERIT: &str = "(herdar)";

/// Os temas por nome (Aylis e os de plataforma).
fn theme_names() -> Vec<String> {
    let mut names = vec!["Aylis Light".to_string(), "Aylis Dark".to_string()];
    names.extend(aylis_core::themes::all().iter().map(|t| t.name.clone()));
    names
}

/// O nome do tema para uma família e um modo ("Aylis" + escuro → "Aylis Dark").
fn theme_of(family: &str, dark: bool) -> String {
    aylis_core::themes::of_family(family, dark).map_or_else(|| family.to_string(), |t| t.name.clone())
}

/// "Projeto → Opções do projeto → Aparência": o tema do projeto inteiro (família e modo), a mistura por classe,
/// uma prévia e a lista de quem usa qual tema no form aberto.
pub fn appearance(
    ui: &mut Ui,
    project: &Project,
    classes: Vec<String>,
    rows: Vec<UsageRow>,
    on_ok: impl FnOnce(&mut Ui, Appearance) + 'static,
) {
    let d = new_dialog(ui, "Aparência do projeto", 980.0, 720.0);
    let left = ui.add(Layout::default(), d.body);
    ui.control_mut(left).unwrap().set_align(Align::Left).set_width(440.0).set_margins([0.0, 0.0, 20.0, 0.0]);

    // ----- tema do projeto -----
    let mut families: Vec<String> = aylis_core::themes::FAMILIES.iter().map(|s| s.to_string()).collect();
    if let Ok(entries) = std::fs::read_dir(project.dir().join("themes")) {
        for e in entries.flatten() {
            let file = e.file_name().to_string_lossy().to_string();
            if file.ends_with(".json") {
                families.push(format!("themes/{file}"));
            }
        }
    }
    let (current_family, current_dark) = match project.theme.as_str() {
        "" | "light" => ("Aylis".to_string(), false),
        "dark" => ("Aylis".to_string(), true),
        name => match aylis_core::themes::builtin(name) {
            Some(t) => (t.family_name().to_string(), t.dark),
            None => (name.to_string(), false),
        },
    };
    crate::widgets::section(ui, left, "TEMA DO PROJETO (cores e forma dos controles)");
    let line = field(ui, left, "Família");
    let refs: Vec<&str> = families.iter().map(String::as_str).collect();
    let family = combo(ui, line, &refs, &current_family);
    let line = field(ui, left, "Modo");
    let mode = combo(ui, line, &["Claro", "Escuro"], if current_dark { "Escuro" } else { "Claro" });

    // ----- mistura por classe -----
    crate::widgets::section(ui, left, "MISTURAR POR CLASSE (estas classes usam outro tema)");
    let names = theme_names();
    let name_refs: Vec<String> = names.clone();
    let add_line = ui.add(Layout::default(), left);
    ui.control_mut(add_line).unwrap().set_align(Align::Top).set_height(34.0).set_margins([0.0, 0.0, 0.0, 6.0]);
    let add = ui.add(Button::new("Acrescentar"), add_line);
    ui.control_mut(add).unwrap().set_align(Align::Right).set_width(110.0).set_margins([6.0, 0.0, 0.0, 0.0]).set_style("secondary");
    let class_refs: Vec<&str> = classes.iter().map(String::as_str).collect();
    let new_class = ui.add(ComboBox::new(&class_refs), add_line);
    ui.control_mut(new_class).unwrap().set_align(Align::Left).set_width(140.0).set_margins([0.0, 0.0, 6.0, 0.0]);
    let _ = ui.set_prop(new_class, "item_index", 0i64);
    let theme_refs: Vec<&str> = name_refs.iter().map(String::as_str).collect();
    let new_theme = combo(ui, add_line, &theme_refs, "iOS 26 Light");
    let mix_list = ui.add(ScrollBox::default(), left);
    ui.control_mut(mix_list).unwrap().set_align(Align::Top).set_height(150.0);
    let mixes: Rc<RefCell<Vec<(String, ComponentId)>>> = Rc::new(RefCell::new(Vec::new()));

    // ----- prévia -----
    crate::widgets::section(ui, left, "PRÉVIA");
    let preview = ui.add(Rectangle::default(), left);
    ui.control_mut(preview).unwrap().set_align(Align::Client);
    let _ = ui.set_prop(preview, "radius", 8);
    {
        let put = |ui: &mut Ui, id: ComponentId, x: f32, y: f32, w: f32, h: f32| {
            ui.control_mut(id).unwrap().set_rect(x, y, w, h);
        };
        let b = ui.add(Button::new("Salvar"), preview);
        put(ui, b, 14.0, 14.0, 110.0, 34.0);
        let b2 = ui.add(Button::new("Cancelar"), preview);
        put(ui, b2, 132.0, 14.0, 110.0, 34.0);
        let _ = ui.set_prop(b2, "style", "secondary");
        let e = ui.add(Edit::new("Texto"), preview);
        put(ui, e, 252.0, 14.0, 160.0, 34.0);
        let c = ui.add(CheckBox::new("Marcado"), preview);
        put(ui, c, 14.0, 60.0, 120.0, 26.0);
        let _ = ui.set_prop(c, "checked", true);
        let r = ui.add(RadioButton::new("Opção"), preview);
        put(ui, r, 140.0, 60.0, 110.0, 26.0);
        let _ = ui.set_prop(r, "checked", true);
        let s = ui.add(Switch::new("Ligado"), preview);
        put(ui, s, 256.0, 60.0, 140.0, 28.0);
        let _ = ui.set_prop(s, "checked", true);
        let t = ui.add(TrackBar::new(60.0, 0.0, 100.0), preview);
        put(ui, t, 10.0, 98.0, 400.0, 30.0);
        let p = ui.add(ProgressBar::default(), preview);
        put(ui, p, 14.0, 138.0, 396.0, 6.0);
        let _ = ui.set_prop(p, "value", 45.0);
    }
    let update_preview = move |ui: &mut Ui| {
        let fam = combo_text(ui, family);
        // Pela posição (o item 1 é "Escuro"): o texto vem traduzido na tela.
        let dark = ui.get::<ComboBox>(mode).is_some_and(|c| c.item_index == 1);
        let name = theme_of(&fam, dark);
        let bg = ui.theme_named(&name).map(|t| t.color("background"));
        match bg {
            Some(bg) => {
                let _ = ui.set_prop(preview, "theme", name.as_str());
                let _ = ui.set_prop(preview, "fill", bg);
            }
            None => {
                let _ = ui.set_prop(preview, "theme", "");
            }
        }
    };
    update_preview(ui);
    ui.on(family, "on_change", move |ui, _, _| update_preview(ui));
    ui.on(mode, "on_change", move |ui, _, _| update_preview(ui));

    // Linhas da mistura: classe, tema e remover.
    fn mix_row(ui: &mut Ui, list: ComponentId, mixes: &Rc<RefCell<Vec<(String, ComponentId)>>>, class: &str, theme: &str, names: &[String]) {
        let line = ui.add(Layout::default(), list);
        ui.control_mut(line).unwrap().set_align(Align::Top).set_height(32.0).set_margins([0.0, 0.0, 0.0, 4.0]);
        let remove = ui.add(Button::with_icon("close"), line);
        ui.control_mut(remove).unwrap().set_align(Align::Right).set_width(32.0).set_style("icon");
        let l = label(ui, line, class, Align::Left, 140.0);
        let _ = l;
        let refs: Vec<&str> = names.iter().map(String::as_str).collect();
        let c = combo(ui, line, &refs, theme);
        mixes.borrow_mut().push((class.to_string(), c));
        let mixes2 = mixes.clone();
        let class = class.to_string();
        ui.on_click(remove, move |ui, _| {
            mixes2.borrow_mut().retain(|(k, _)| *k != class);
            ui.free(line);
        });
    }
    for (class, theme) in &project.theme_mix {
        mix_row(ui, mix_list, &mixes, class, theme, &names);
    }
    {
        let (mixes, names) = (mixes.clone(), names.clone());
        ui.on_click(add, move |ui, _| {
            let class = combo_text(ui, new_class);
            let theme = combo_text(ui, new_theme);
            if class.is_empty() || mixes.borrow().iter().any(|(k, _)| *k == class) {
                return;
            }
            mix_row(ui, mix_list, &mixes, &class, &theme, &names);
        });
    }

    // ----- quem usa qual tema -----
    let right = ui.add(Layout::default(), d.body);
    ui.control_mut(right).unwrap().set_align(Align::Client);
    crate::widgets::section(ui, right, "QUEM USA QUAL TEMA (form aberto) · troque o tema próprio aqui");
    let list = crate::widgets::framed_list(ui, right);
    let mut own_names = vec![INHERIT.to_string()];
    own_names.extend(names.iter().cloned());
    let own_refs: Vec<&str> = own_names.iter().map(String::as_str).collect();
    let mut own_combos = Vec::new();
    for row in &rows {
        let line = ui.add(Layout::default(), list);
        ui.control_mut(line).unwrap().set_align(Align::Top).set_height(44.0).set_margins([0.0, 0.0, 0.0, 2.0]);
        let c = ui.add(ComboBox::new(&own_refs), line);
        ui.control_mut(c).unwrap().set_align(Align::Right).set_width(170.0).set_margins([6.0, 6.0, 0.0, 6.0]);
        let own = if row.own.is_empty() { INHERIT } else { row.own.as_str() };
        if let Some(i) = own_refs.iter().position(|n| n.eq_ignore_ascii_case(own)) {
            let _ = ui.set_prop(c, "item_index", i as i64);
        }
        let texts = ui.add(Layout::default(), line);
        ui.control_mut(texts).unwrap().set_align(Align::Client);
        label(ui, texts, &row.label, Align::Top, 20.0);
        let info = label(ui, texts, &format!("{} · {}", row.theme, row.source), Align::Top, 18.0);
        muted(ui, info);
        let _ = ui.set_prop(info, "font_size", 12);
        own_combos.push((row.id, row.own.clone(), c));
    }
    if rows.is_empty() {
        let l = label(ui, list, "Abra um form para ver os componentes.", Align::Top, 24.0);
        muted(ui, l);
    }

    let original = (project.theme.clone(), current_family.clone(), current_dark);
    run(ui, d, |_| Ok(()), move |ui| {
        let fam = combo_text(ui, family);
        // Pela posição (o item 1 é "Escuro"): o texto vem traduzido na tela.
        let dark = ui.get::<ComboBox>(mode).is_some_and(|c| c.item_index == 1);
        // Sem mudar a família nem o modo, o tema fica como estava escrito ("Aylis Dark" virava
        // "dark" só por aplicar; achado em 07/11).
        let theme = if fam == original.1 && dark == original.2 {
            original.0.clone()
        } else if fam.ends_with(".json") {
            fam
        } else if fam == "Aylis" {
            if dark { "dark".to_string() } else { String::new() }
        } else {
            theme_of(&fam, dark)
        };
        let mix = mixes.borrow().iter().map(|(k, c)| (k.clone(), combo_text(ui, *c))).collect();
        let own = own_combos
            .iter()
            .filter_map(|(id, before, c)| {
                // "(herdar)" é o primeiro item: pela posição, que o texto pode vir traduzido.
                let inherit = ui.get::<ComboBox>(*c).is_some_and(|combo| combo.item_index <= 0);
                let now = if inherit { String::new() } else { combo_text(ui, *c) };
                (now != *before).then_some((*id, now))
            })
            .collect();
        on_ok(ui, Appearance { theme, mix, own });
    });
}

/// Escolher uma cor: tokens do tema (a cor acompanha o tema) ou uma cor fixa (#rrggbb).
pub fn color_picker(ui: &mut Ui, initial: &str, tokens: Vec<(String, Color)>, on_ok: impl FnOnce(&mut Ui, String) + 'static) {
    let d = new_dialog(ui, "Cor", 820.0, 480.0);
    // À esquerda o quadro de cores (matiz, saturação, brilho e transparência); à direita o
    // valor e as amostras do tema e fixas.
    let left = ui.add(Layout::default(), d.body);
    ui.control_mut(left).unwrap().set_align(Align::Left).set_width(280.0).set_margins([0.0, 0.0, 18.0, 0.0]);
    let picker = ui.add(ColorPicker::default(), left);
    ui.control_mut(picker).unwrap().set_align(Align::Top).set_height(262.0);
    let _ = ui.set_prop(picker, "swatches", false);
    let right = ui.add(Layout::default(), d.body);
    ui.control_mut(right).unwrap().set_align(Align::Client);
    let line = field(ui, right, "Cor");
    let preview = ui.add(Rectangle::new(40.0, 0.0), line);
    ui.control_mut(preview).unwrap().set_align(Align::Right).set_margins([8.0, 2.0, 0.0, 2.0]);
    let _ = ui.set_prop(preview, "radius", 6);
    let value = edit(ui, line, initial);
    let resolve = {
        let tokens = tokens.clone();
        move |text: &str| Color::from_hex(text).or_else(|| tokens.iter().find(|(n, _)| n == text).map(|(_, c)| *c))
    };
    let update = {
        let resolve = resolve.clone();
        move |ui: &mut Ui| {
            let color = resolve(&text_of(ui, value));
            let _ = ui.set_prop(preview, "fill", color.unwrap_or(Color::TRANSPARENT));
            if let Some(c) = color {
                // Sem disparar o `on_change` do quadro (que reescreveria o texto como #hex).
                let _ = ui.set_prop_quiet(picker, "color", c);
            }
        }
    };
    update(ui);
    {
        let update = update.clone();
        ui.on(value, "on_change", move |ui, _, _| update(ui));
    }
    // O quadro escreve o #hex (um token escolhido vira cor fixa ao mexer no quadro).
    ui.on(picker, "on_change", move |ui, _, args| {
        if let EventArgs::Value(PropValue::String(hex)) = args {
            let _ = ui.set_prop_quiet(value, "text", hex.clone());
            if let Some(c) = Color::from_hex(hex) {
                let _ = ui.set_prop(preview, "fill", c);
            }
        }
    });
    let swatches = |ui: &mut Ui, title: &str, colors: Vec<(String, Color)>| {
        crate::widgets::section(ui, right, title);
        let grid = ui.add(Layout::default(), right);
        let rows = colors.len().div_ceil(12);
        ui.control_mut(grid).unwrap().set_align(Align::Top).set_height(rows as f32 * 34.0);
        for (i, (text, color)) in colors.into_iter().enumerate() {
            let s = ui.add(Rectangle::new(30.0, 30.0), grid);
            ui.control_mut(s).unwrap().set_rect((i % 12) as f32 * 39.0, (i / 12) as f32 * 34.0, 30.0, 30.0).set_cursor(aylis_core::CursorShape::Pointer);
            let _ = ui.set_prop(s, "fill", color);
            let _ = ui.set_prop(s, "radius", 6);
            let border = ui.theme().color("border");
            let _ = ui.set_prop(s, "stroke", border);
            let update = update.clone();
            ui.on_click(s, move |ui, _| {
                let _ = ui.set_prop(value, "text", text.clone());
                update(ui);
            });
        }
    };
    swatches(ui, "DO TEMA (acompanham o tema claro/escuro)", tokens);
    let fixed = [
        "#000000", "#374151", "#6b7280", "#9ca3af", "#d1d5db", "#ffffff", "#7f1d1d", "#dc2626", "#f87171", "#7c2d12", "#ea580c",
        "#fb923c", "#713f12", "#ca8a04", "#facc15", "#14532d", "#16a34a", "#4ade80", "#134e4a", "#0d9488", "#2dd4bf", "#1e3a8a",
        "#2563eb", "#60a5fa", "#4c1d95", "#7c3aed", "#a78bfa", "#831843", "#db2777", "#f472b6",
    ];
    let fixed: Vec<(String, Color)> = fixed.iter().filter_map(|h| Color::from_hex(h).map(|c| (h.to_string(), c))).collect();
    swatches(ui, "FIXAS", fixed);
    run(
        ui,
        d,
        move |ui| {
            let t = text_of(ui, value);
            if t.is_empty() || resolve(&t).is_some() { Ok(()) } else { Err(format!("'{t}' não é uma cor (#rrggbb ou um token do tema)")) }
        },
        move |ui| {
            let t = text_of(ui, value);
            on_ok(ui, t);
        },
    );
}

/// Editor de uma lista de textos (itens de um ComboBox, colunas de um DataGrid...).
/// `suggestions` aparecem num botão que acrescenta as que faltam.
pub fn list_editor(
    ui: &mut Ui,
    title: &str,
    items: Vec<String>,
    suggestions: Vec<String>,
    on_ok: impl FnOnce(&mut Ui, Vec<String>) + 'static,
) {
    let d = new_dialog(ui, title, 480.0, 520.0);
    let tools = ui.add(Layout::default(), d.body);
    ui.control_mut(tools).unwrap().set_align(Align::Top).set_height(34.0).set_margins([0.0, 0.0, 0.0, 8.0]);
    let add = crate::widgets::tool(ui, tools, "+ Item", 90.0, "");
    let suggest = (!suggestions.is_empty()).then(|| crate::widgets::tool(ui, tools, "+ Sugeridos", 120.0, "flat"));
    let list = ui.add(ScrollBox::default(), d.body);
    ui.control_mut(list).unwrap().set_align(Align::Client);
    let items = Rc::new(RefCell::new(items));
    let edits: Rc<RefCell<Vec<ComponentId>>> = Rc::default();

    // Lê o que foi digitado antes de remontar a lista.
    let read = {
        let (items, edits) = (items.clone(), edits.clone());
        move |ui: &Ui| {
            let texts: Vec<String> = edits.borrow().iter().map(|&e| ui.get::<Edit>(e).map(|e| e.text.clone()).unwrap_or_default()).collect();
            if texts.len() == items.borrow().len() {
                *items.borrow_mut() = texts;
            }
        }
    };
    type Rebuild = Rc<RefCell<Option<Box<dyn Fn(&mut Ui)>>>>;
    let rebuild: Rebuild = Rc::default();
    {
        let (items, edits, rebuild2, read) = (items.clone(), edits.clone(), rebuild.clone(), read.clone());
        *rebuild.borrow_mut() = Some(Box::new(move |ui: &mut Ui| {
            crate::widgets::clear(ui, list);
            edits.borrow_mut().clear();
            let count = items.borrow().len();
            for i in 0..count {
                let line = ui.add(Layout::default(), list);
                ui.control_mut(line).unwrap().set_align(Align::Top).set_height(32.0).set_margins([0.0, 0.0, 0.0, 4.0]);
                for (text, action) in [("✕", 0), ("↓", 1), ("↑", 2)] {
                    let b = ui.add(Button::new(text), line);
                    ui.control_mut(b).unwrap().set_align(Align::Right).set_width(32.0).set_margins([4.0, 0.0, 0.0, 0.0]).set_style("flat");
                    let (items, rebuild, read) = (items.clone(), rebuild2.clone(), read.clone());
                    ui.on_click(b, move |ui, _| {
                        read(ui);
                        {
                            let mut list = items.borrow_mut();
                            match action {
                                0 => {
                                    list.remove(i);
                                }
                                1 if i + 1 < list.len() => list.swap(i, i + 1),
                                2 if i > 0 => list.swap(i, i - 1),
                                _ => {}
                            }
                        }
                        if let Some(f) = rebuild.borrow().as_ref() {
                            f(ui);
                        }
                    });
                }
                let text = items.borrow()[i].clone();
                let e = edit(ui, line, &text);
                edits.borrow_mut().push(e);
            }
        }));
    }
    if let Some(f) = rebuild.borrow().as_ref() {
        f(ui);
    }
    {
        let (items, rebuild, read) = (items.clone(), rebuild.clone(), read.clone());
        ui.on_click(add, move |ui, _| {
            read(ui);
            items.borrow_mut().push(String::new());
            if let Some(f) = rebuild.borrow().as_ref() {
                f(ui);
            }
        });
    }
    if let Some(suggest) = suggest {
        let (items, rebuild, read) = (items.clone(), rebuild.clone(), read.clone());
        ui.on_click(suggest, move |ui, _| {
            read(ui);
            for s in &suggestions {
                if !items.borrow().contains(s) {
                    items.borrow_mut().push(s.clone());
                }
            }
            if let Some(f) = rebuild.borrow().as_ref() {
                f(ui);
            }
        });
    }
    run(ui, d, |_| Ok(()), move |ui| {
        read(ui);
        let result: Vec<String> = items.borrow().iter().map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
        // A lista guarda a função que remonta a si mesma: quebra o ciclo.
        rebuild.borrow_mut().take();
        on_ok(ui, result);
    });
}

/// Um campo no editor de campos.
#[derive(Clone, Debug, PartialEq)]
pub struct FieldSpec {
    /// O `Field` que já existe (`None` = novo).
    pub id: Option<ComponentId>,
    pub name: String,
    pub kind: String,
    pub size: String,
    pub label: String,
}

/// Editor de campos de um ClientDataSet.
pub fn fields_editor(ui: &mut Ui, title: &str, fields: Vec<FieldSpec>, types: &'static [&'static str], on_ok: impl FnOnce(&mut Ui, Vec<FieldSpec>) + 'static) {
    let d = new_dialog(ui, title, 720.0, 540.0);
    let tools = ui.add(Layout::default(), d.body);
    ui.control_mut(tools).unwrap().set_align(Align::Top).set_height(34.0).set_margins([0.0, 0.0, 0.0, 8.0]);
    let add = crate::widgets::tool(ui, tools, "+ Campo", 100.0, "");
    let header = ui.add(Layout::default(), d.body);
    ui.control_mut(header).unwrap().set_align(Align::Top).set_height(22.0);
    for (text, width) in [("Nome", 150.0), ("Tipo", 150.0), ("Tamanho", 80.0), ("Rótulo", 200.0)] {
        let l = label(ui, header, text, Align::Left, width + 6.0);
        muted(ui, l);
        let _ = ui.set_prop(l, "font_size", 12);
    }
    let list = ui.add(ScrollBox::default(), d.body);
    ui.control_mut(list).unwrap().set_align(Align::Client);
    let fields = Rc::new(RefCell::new(fields));
    // (nome, tipo, tamanho, rótulo) de cada linha
    let rows: Rc<RefCell<Vec<[ComponentId; 4]>>> = Rc::default();
    let read = {
        let (fields, rows) = (fields.clone(), rows.clone());
        move |ui: &Ui| {
            let rows = rows.borrow();
            let mut fields = fields.borrow_mut();
            if rows.len() != fields.len() {
                return;
            }
            for (f, r) in fields.iter_mut().zip(rows.iter()) {
                f.name = ui.get::<Edit>(r[0]).map(|e| e.text.trim().to_string()).unwrap_or_default();
                f.kind = ui.get::<ComboBox>(r[1]).map(|c| c.text().to_string()).unwrap_or_default();
                f.size = ui.get::<Edit>(r[2]).map(|e| e.text.trim().to_string()).unwrap_or_default();
                f.label = ui.get::<Edit>(r[3]).map(|e| e.text.clone()).unwrap_or_default();
            }
        }
    };
    type Rebuild = Rc<RefCell<Option<Box<dyn Fn(&mut Ui)>>>>;
    let rebuild: Rebuild = Rc::default();
    {
        let (fields, rows, rebuild2, read) = (fields.clone(), rows.clone(), rebuild.clone(), read.clone());
        *rebuild.borrow_mut() = Some(Box::new(move |ui: &mut Ui| {
            crate::widgets::clear(ui, list);
            rows.borrow_mut().clear();
            let count = fields.borrow().len();
            for i in 0..count {
                let f = fields.borrow()[i].clone();
                let line = ui.add(Layout::default(), list);
                ui.control_mut(line).unwrap().set_align(Align::Top).set_height(32.0).set_margins([0.0, 0.0, 0.0, 4.0]);
                let cell = |ui: &mut Ui, w: f32| {
                    let c = ui.add(Layout::default(), line);
                    ui.control_mut(c).unwrap().set_align(Align::Left).set_width(w).set_margins([0.0, 0.0, 6.0, 0.0]);
                    c
                };
                let c = cell(ui, 150.0);
                let name = edit(ui, c, &f.name);
                let c = cell(ui, 150.0);
                let kind = combo(ui, c, types, &f.kind);
                let c = cell(ui, 80.0);
                let size = edit(ui, c, &f.size);
                let c = cell(ui, 200.0);
                let text = edit(ui, c, &f.label);
                for (t, action) in [("✕", 0), ("↓", 1), ("↑", 2)] {
                    let b = ui.add(Button::new(t), line);
                    ui.control_mut(b).unwrap().set_align(Align::Left).set_width(32.0).set_margins([0.0, 0.0, 4.0, 0.0]).set_style("flat");
                    let (fields, rebuild, read) = (fields.clone(), rebuild2.clone(), read.clone());
                    ui.on_click(b, move |ui, _| {
                        read(ui);
                        {
                            let mut list = fields.borrow_mut();
                            match action {
                                0 => {
                                    list.remove(i);
                                }
                                1 if i + 1 < list.len() => list.swap(i, i + 1),
                                2 if i > 0 => list.swap(i, i - 1),
                                _ => {}
                            }
                        }
                        if let Some(f) = rebuild.borrow().as_ref() {
                            f(ui);
                        }
                    });
                }
                rows.borrow_mut().push([name, kind, size, text]);
            }
        }));
    }
    if let Some(f) = rebuild.borrow().as_ref() {
        f(ui);
    }
    {
        let (fields, rebuild, read) = (fields.clone(), rebuild.clone(), read.clone());
        ui.on_click(add, move |ui, _| {
            read(ui);
            let n = fields.borrow().len() + 1;
            fields.borrow_mut().push(FieldSpec { id: None, name: format!("campo{n}"), kind: "string".into(), size: String::new(), label: String::new() });
            if let Some(f) = rebuild.borrow().as_ref() {
                f(ui);
            }
        });
    }
    let (fields2, read2) = (fields.clone(), read.clone());
    run(
        ui,
        d,
        move |ui| {
            read2(ui);
            let fields = fields2.borrow();
            for (i, f) in fields.iter().enumerate() {
                if !aylis_project::is_identifier(&f.name) {
                    return Err(format!("'{}' não é um nome de campo válido", f.name));
                }
                if fields.iter().skip(i + 1).any(|g| g.name.eq_ignore_ascii_case(&f.name)) {
                    return Err(format!("campo '{}' repetido", f.name));
                }
                if !f.size.is_empty() && f.size.parse::<u32>().is_err() {
                    return Err(format!("tamanho de '{}' deve ser um número", f.name));
                }
            }
            Ok(())
        },
        move |ui| {
            read(ui);
            rebuild.borrow_mut().take();
            let result = fields.borrow().clone();
            on_ok(ui, result);
        },
    );
}

/// Sobre o aylis.
pub fn about(ui: &mut Ui) {
    let d = new_dialog(ui, "Sobre o Aylis", 520.0, 270.0);
    let t = label(ui, d.body, "Aylis IDE", Align::Top, 34.0);
    let _ = ui.set_prop(t, "font_size", 22);
    let text = format!(
        "Interfaces visuais em Rust, com designer e desenho próprio igual em todas as plataformas.\nVersão {}.\nForms em JSON; eventos em Rust, Python, TypeScript, C#, Java, Go, C++ ou Pascal.",
        env!("CARGO_PKG_VERSION")
    );
    let l = label(ui, d.body, &text, Align::Client, 0.0);
    let _ = ui.set_prop(l, "wrap", true);
    muted(ui, l);
    // Só um botão: esconde o Cancelar.
    if let Some(&cancel) = ui.children(d.footer).first() {
        if let Some(cb) = ui.control_mut(cancel) {
            cb.set_visible(false);
        }
    }
    run(ui, d, |_| Ok(()), |_| {});
}

/// Valor de uma propriedade como texto (para os editores).
pub fn prop_text(ui: &Ui, id: ComponentId, name: &str) -> String {
    match ui.get_prop(id, name) {
        Ok(PropValue::String(s)) | Ok(PropValue::Enum(s)) => s,
        Ok(PropValue::Int(i)) => i.to_string(),
        Ok(PropValue::Float(f)) => f.to_string(),
        _ => String::new(),
    }
}

/// Evento da caixa de seleção: o texto escolhido.
pub fn on_text(args: &EventArgs) -> Option<&str> {
    args.text()
}

