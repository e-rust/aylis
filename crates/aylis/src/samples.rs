//! O componente com conteúdo de exemplo: o que a galeria da IDE, as imagens da documentação e
//! as demonstrações ao vivo da página do Aylis mostram. Sem isso, uma lista, uma grade ou um
//! contêiner aparecem vazios e não dá para ver como são.
//!
//! O conteúdo fica em `samples.json` (por classe: o tamanho, as propriedades, os filhos e as
//! referências a componentes de apoio) e, no que só dá para montar por código (as linhas de um
//! dataset, as mensagens de um chat, os nós de um grafo), em [`fill`].

use aylis_core::{Align, ComponentId, PropValue, Ui};
use serde_json::Value;

/// O tamanho de um componente sem tamanho próprio (o mesmo da galeria da IDE).
const DEFAULT: (f32, f32) = (240.0, 140.0);

fn samples() -> &'static Value {
    static SAMPLES: std::sync::OnceLock<Value> = std::sync::OnceLock::new();
    SAMPLES.get_or_init(|| serde_json::from_str(include_str!("samples.json")).unwrap_or(Value::Null))
}

/// Os arquivos dos exemplos (`samples/rust.svg`, `samples/coracao.json`), na frente do serviço
/// de arquivos que já houver (o do projeto, na IDE): o `Svg` e o `Lottie` leem pelo caminho.
struct SampleFiles(Option<Box<dyn aylis_core::platform::ResourceService>>);

impl aylis_core::platform::ResourceService for SampleFiles {
    fn read(&self, path: &str) -> Option<Vec<u8>> {
        match path {
            "samples/rust.svg" => Some(include_bytes!("samples/rust.svg").to_vec()),
            "samples/coracao.json" => Some(include_bytes!("samples/coracao.json").to_vec()),
            _ => self.0.as_ref()?.read(path),
        }
    }
}

/// Liga os arquivos dos exemplos no `Ui` (uma vez; [`create`] chama).
fn install_files(ui: &mut Ui) {
    let services = ui.services_mut();
    if services.resources.as_ref().is_some_and(|r| r.read("samples/rust.svg").is_some()) {
        return;
    }
    let previous = services.resources.take();
    services.resources = Some(Box::new(SampleFiles(previous)));
}

/// Uma "foto" desenhada por código para o `Image`: o céu do fim de tarde com o sol e os morros.
fn photo(width: u32, height: u32) -> aylis_core::Image {
    let mut pixels = Vec::with_capacity((width * height * 4) as usize);
    for y in 0..height {
        for x in 0..width {
            let (fx, fy) = (x as f32 / width as f32, y as f32 / height as f32);
            let hill = 0.72 - 0.10 * (fx * 6.0).sin() - 0.05 * (fx * 13.0 + 1.0).sin();
            let sun = ((x as f32 - 0.68 * width as f32).powi(2) + (y as f32 - 0.42 * height as f32).powi(2)).sqrt() / height as f32;
            let (r, g, b) = if fy > hill {
                (30.0 + 40.0 * fy, 24.0 + 30.0 * fy, 60.0 + 20.0 * fy)
            } else if sun < 0.13 {
                (255.0, 214.0, 120.0)
            } else {
                (255.0 - 120.0 * fy, 120.0 + 40.0 * (1.0 - fy) - 60.0 * fy, 90.0 + 120.0 * fy)
            };
            pixels.extend([r.clamp(0.0, 255.0) as u8, g.clamp(0.0, 255.0) as u8, b.clamp(0.0, 255.0) as u8, 255]);
        }
    }
    aylis_core::Image::rgba(width, height, pixels)
}

/// Cria `class` em `form` com o nome `demo` e o conteúdo de exemplo (e os componentes de
/// apoio, como o dataset de uma grade). `None` se a classe não existe no registro.
pub fn create(ui: &mut Ui, class: &str, form: ComponentId) -> Option<ComponentId> {
    let id = ui.create(class, Some(form)).ok()?;
    let _ = ui.set_prop(id, "name", "demo");
    // O texto vazio vira o nome da classe (um botão sem texto não diz nada).
    for prop in ["text", "caption"] {
        if let Ok(PropValue::String(current)) = ui.get_prop(id, prop) {
            if current.is_empty() {
                let _ = ui.set_prop(id, prop, class);
            }
        }
    }
    apply(ui, id, class, form);
    Some(id)
}

/// Tem conteúdo de exemplo para `class` (a classe sem o subtipo, `Button` de `Button.outline`).
pub fn has_sample(class: &str) -> bool {
    samples().get(class).is_some()
}

/// Põe o conteúdo de exemplo de `class` num componente já criado (`id`, dentro de `form`): as
/// propriedades, os filhos, os componentes de apoio e o que vem por código. A galeria da IDE
/// usa também, e as imagens da documentação saem iguais às demonstrações ao vivo.
pub fn apply(ui: &mut Ui, id: ComponentId, class: &str, form: ComponentId) {
    install_files(ui);
    let sample = &samples()[class];
    // Os de apoio antes, para as referências acharem pelo nome.
    if sample.get("refs").is_some() {
        support(ui, form);
    }
    if let Some(props) = sample.get("props").and_then(Value::as_object) {
        let _ = aylis_json::set_props(ui, id, props);
    }
    for child in sample.get("children").and_then(Value::as_array).into_iter().flatten() {
        let _ = aylis_json::load_value(ui, &serde_json::json!({ "root": child }), Some(id));
    }
    for (prop, target) in sample.get("refs").and_then(Value::as_object).into_iter().flatten() {
        if let Some(target) = target.as_str().and_then(|name| ui.find_in(form, name)) {
            let _ = ui.set_prop(id, prop, PropValue::Component(Some(target)));
        }
    }
    #[cfg(all(feature = "dbcontrols", feature = "providers"))]
    if class == "ReportPreview" {
        if let Some(report) = sample_report(ui, form) {
            let _ = ui.set_prop(id, "report", PropValue::Component(Some(report)));
        }
    }
    fill(ui, id, class);
}

/// O relatório de exemplo do `ReportPreview` (`rptDemo`, com os clientes do `cdsDemo`): o
/// título e a banda de detalhe com o nome, a cidade e o código de barras de cada cliente (o
/// `Barcode` com `[codigo]` no texto, como um `Label`, sai com o valor de cada registro).
#[cfg(all(feature = "dbcontrols", feature = "providers"))]
fn sample_report(ui: &mut Ui, form: ComponentId) -> Option<ComponentId> {
    if let Some(report) = ui.find_in(form, "rptDemo") {
        return Some(report);
    }
    support(ui, form);
    let source = ui.find_in(form, "dsDemo")?;
    let report = serde_json::json!({ "root": { "class": "Report", "name": "rptDemo", "title": "Clientes", "children": [
        { "class": "ReportBand", "name": "bndDemoTitulo", "kind": "title", "height": 44, "children": [
            { "class": "Label", "name": "lblDemoTitulo", "text": "Clientes · [COUNT()]", "bold": true, "color": "#1F2937", "width": 480, "height": 30, "font_size": 18 } ] },
        { "class": "ReportBand", "name": "bndDemoDetalhe", "kind": "detail", "height": 58, "children": [
            { "class": "Label", "name": "lblDemoNome", "text": "[nome]", "color": "#1F2937", "top": 8, "width": 220, "height": 18 },
            { "class": "Label", "name": "lblDemoCidade", "text": "[cidade]", "color": "#6B7280", "top": 28, "width": 220, "height": 16, "font_size": 10 },
            { "class": "Barcode", "name": "bcDemoCodigo", "text": "CLI-000[codigo]", "format": "code128", "show_text": true, "left": 260, "top": 4, "width": 200, "height": 50 } ] } ] } });
    let report = aylis_json::load_value(ui, &report, Some(form)).ok()?;
    let _ = ui.set_prop(report, "data_source", PropValue::Component(Some(source)));
    Some(report)
}

/// O tamanho de `samples.json` para `class`, se houver.
pub fn sample_size(class: &str) -> Option<(f32, f32)> {
    let s = samples()[class]["size"].as_array()?;
    Some((s.first()?.as_f64()? as f32, s.get(1)?.as_f64()? as f32))
}

/// O tamanho natural do exemplo: o de `samples.json`, o próprio do componente ou um padrão
/// pelo alinhamento (uma barra no topo tem a altura dela e a largura de um form pequeno).
pub fn size(ui: &Ui, id: ComponentId) -> (f32, f32) {
    let class = ui.class_name(id).unwrap_or_default();
    if let Some(size) = sample_size(class) {
        return size;
    }
    let (w, h, align) = ui.control(id).map(|cb| (cb.width(), cb.height(), cb.align())).unwrap_or((0.0, 0.0, Align::None));
    let own = |v: f32, sample: f32| if v >= 1.0 { v } else { sample };
    match align {
        Align::None => (own(w, DEFAULT.0), own(h, DEFAULT.1)),
        Align::Top | Align::Bottom => (360.0, own(h, 40.0)),
        Align::Left | Align::Right => (own(w, 200.0), DEFAULT.1),
        _ => DEFAULT,
    }
}

/// O dataset de exemplo (`cdsDemo`, com `dsDemo`): clientes com cidade, saldo e situação.
#[cfg(feature = "data")]
fn support(ui: &mut Ui, form: ComponentId) {
    if ui.find_in(form, "cdsDemo").is_some() {
        return;
    }
    let fields = serde_json::json!({ "root": { "class": "ClientDataSet", "name": "cdsDemo", "children": [
        { "class": "Field", "name": "cdsDemoCodigo", "field_name": "codigo", "type": "integer", "display_label": "Código", "display_width": 70 },
        { "class": "Field", "name": "cdsDemoNome", "field_name": "nome", "type": "string", "display_label": "Cliente", "display_width": 150 },
        { "class": "Field", "name": "cdsDemoCidade", "field_name": "cidade", "type": "string", "display_label": "Cidade", "display_width": 120 },
        { "class": "Field", "name": "cdsDemoSaldo", "field_name": "saldo", "type": "float", "display_label": "Saldo", "display_width": 100 }
    ] } });
    let Ok(cds) = aylis_json::load_value(ui, &fields, Some(form)) else { return };
    let _ = aylis_json::load_value(ui, &serde_json::json!({ "root": { "class": "DataSource", "name": "dsDemo", "data_set": "cdsDemo" } }), Some(form));
    if let Some(ds) = ui.find_in(form, "dsDemo") {
        let _ = ui.set_prop(ds, "data_set", PropValue::Component(Some(cds)));
    }
    let rows = serde_json::json!([
        { "codigo": 1, "nome": "Ana Souza", "cidade": "Campinas", "saldo": 1250.5 },
        { "codigo": 2, "nome": "Bruno Lima", "cidade": "Santos", "saldo": 380.0 },
        { "codigo": 3, "nome": "Carla Dias", "cidade": "Curitiba", "saldo": 2730.9 },
        { "codigo": 4, "nome": "Diego Alves", "cidade": "Recife", "saldo": 95.2 },
        { "codigo": 5, "nome": "Elisa Rocha", "cidade": "Porto Alegre", "saldo": 4100.0 },
        { "codigo": 6, "nome": "Fábio Melo", "cidade": "Salvador", "saldo": 712.4 },
        { "codigo": 7, "nome": "Gabi Torres", "cidade": "Belém", "saldo": 1530.0 }
    ]);
    let _ = aylis_data::data_set(ui, cds).load_json(&rows);
}

#[cfg(not(feature = "data"))]
fn support(_ui: &mut Ui, _form: ComponentId) {}

/// O que só dá para montar por código.
fn fill(ui: &mut Ui, id: ComponentId, class: &str) {
    use aylis_controls::*;
    match class {
        "ChatView" => {
            if let Some(chat) = ui.get_mut::<ChatView>(id) {
                let message = |id: &str, author: &str, text: &str, time: &str, mine: bool| ChatMessage {
                    id: id.into(),
                    author: author.into(),
                    text: text.into(),
                    time: time.into(),
                    mine,
                    status: if mine { MessageStatus::Read } else { MessageStatus::None },
                    ..Default::default()
                };
                chat.add_message(ChatMessage { id: "0".into(), text: "Hoje".into(), system: true, ..Default::default() });
                chat.add_message(message("1", "Ana", "Oi! O relatório de vendas já saiu?", "09:12", false));
                chat.add_message(message("2", "Você", "Saiu sim, mandei agora há pouco.", "09:14", true));
                chat.add_message(message("3", "Ana", "Perfeito, obrigada! Vou revisar até o almoço.", "09:15", false));
                chat.add_message(message("4", "Você", "Qualquer dúvida, me chama.", "09:16", true));
            }
        }
        "TimeAndSales" => {
            let trades = [(38.52, 300.0, true), (38.51, 1200.0, false), (38.52, 500.0, true), (38.53, 4400.0, true), (38.52, 200.0, false), (38.51, 900.0, false), (38.50, 3100.0, false), (38.51, 700.0, true), (38.52, 1500.0, true), (38.53, 600.0, true), (38.52, 2200.0, false), (38.51, 400.0, false)];
            for (n, (price, size, buy)) in trades.into_iter().enumerate() {
                trader::TimeAndSales::add(ui, id, trader::Trade { time: format!("10:{:02}:{:02}", 32 - n / 4, 59 - n * 4), price, size, buy });
            }
        }
        "NodeGraph" => {
            if let Some(graph) = ui.get_mut::<NodeGraph>(id) {
                for (id, title, subtitle, x, y) in [("mic", "Microfone", "Entrada", 20.0, 40.0), ("mix", "Mixer", "2 canais", 200.0, 150.0), ("out", "Fones", "Saída", 420.0, 60.0)] {
                    let mut node = GraphNode::new(id, title, x, y).subtitle(subtitle);
                    node.width = 200.0;
                    graph.add_node(node);
                }
                graph.connect(GraphCable::new("c1", "mic", "mix"));
                graph.connect(GraphCable::new("c2", "mix", "out").label("estéreo"));
            }
        }
        "TreeView" => {
            // Com os nós abertos (no texto de `items` eles começam fechados).
            let node = |text: &str, children: Vec<TreeNode>| TreeNode { text: text.into(), expanded: !children.is_empty(), children, data: String::new() };
            let nodes = vec![
                node("Brasil", vec![node("São Paulo", vec![node("Campinas", vec![]), node("Santos", vec![])]), node("Rio de Janeiro", vec![]), node("Minas Gerais", vec![])]),
                node("Portugal", vec![node("Lisboa", vec![]), node("Porto", vec![])]),
            ];
            if let Some(tree) = ui.get_mut::<TreeView>(id) {
                tree.set_nodes(nodes);
            }
        }
        "Image" => {
            if let Some(image) = ui.get_mut::<ImageView>(id) {
                image.set_picture(Some(photo(480, 280)));
            }
        }
        "ScreenLayout" => {
            if let Some(screens) = ui.get_mut::<ScreenLayout>(id) {
                let screen = |id: &str, name: &str, x: i32, w: i32, h: i32, enabled: bool| ScreenItem { id: id.into(), name: name.into(), x, y: 0, width: w, height: h, enabled, ..Default::default() };
                screens.set_screens(vec![screen("1", "Monitor 1", 0, 1920, 1080, true), screen("2", "Monitor 2", 1920, 1280, 1024, false)]);
            }
        }
        #[cfg(feature = "3d")]
        "Model3D" => {
            if let Some(view) = ui.get_mut::<crate::scene3d::Model3D>(id) {
                crate::scene3d::ingot::apply(view);
            }
        }
        _ => {}
    }
}
