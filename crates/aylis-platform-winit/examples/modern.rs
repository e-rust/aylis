//! Componentes modernos, subtipos e mistura de temas numa tela só.
//!
//! - barra lateral com `theme = "Aylis Dark"` dentro de um form claro (vale para os filhos);
//! - botões de vários subtipos (padrão, secundário, contorno, ícone, pílula, FAB);
//! - busca, selos, avisos, avatares com presença, seletor segmentado;
//! - medidores de nível, barra de mídia, onda de áudio, direcional, rosca;
//! - grafo de nós e cabos e uma conversa.
//!
//! ```text
//! cargo run -p aylis-platform-winit --example modern [-- --dark]
//! ```

use aylis_controls::*;
use aylis_core::platform::LocaleInfo;
use aylis_core::*;

fn put(ui: &mut Ui, id: ComponentId, x: f32, y: f32, w: f32, h: f32) -> ComponentId {
    ui.control_mut(id).unwrap().set_rect(x, y, w, h);
    id
}

fn new(ui: &mut Ui, class: &str, parent: ComponentId, x: f32, y: f32, w: f32, h: f32) -> ComponentId {
    let id = ui.create(class, Some(parent)).expect("classe registrada");
    put(ui, id, x, y, w, h)
}

fn text(ui: &mut Ui, id: ComponentId, prop: &str, value: &str) {
    ui.set_prop(id, prop, value).expect("propriedade");
}

fn caption(ui: &mut Ui, parent: ComponentId, t: &str, x: f32, y: f32) {
    let l = new(ui, "Label.caption", parent, x, y, 300.0, 18.0);
    text(ui, l, "text", t);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dark = std::env::args().any(|a| a == "--dark");
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    ui.services_mut().locale = Some(Box::new(LocaleInfo::pt_br()));
    ui.set_theme(if dark { Theme::dark() } else { Theme::light() });
    let form = ui.add_root(Form::new("aylis — moderno", 1280.0, 820.0));
    ui.set_name(form, "frmModerno")?;

    // ----- barra lateral escura (tema por controle) -----
    let side = ui.create("Panel.sidebar", Some(form))?;
    ui.control_mut(side).unwrap().set_align(Align::Left).set_width(220.0);
    text(&mut ui, side, "theme", "Aylis Dark");
    let brand = new(&mut ui, "Label.heading", side, 20.0, 18.0, 180.0, 28.0);
    text(&mut ui, brand, "text", "aylis");
    for (i, (icon, label)) in [("home", "Início"), ("video", "Gravações"), ("folder", "Biblioteca"), ("sliders", "Mixer"), ("settings", "Configurações")]
        .into_iter()
        .enumerate()
    {
        let b = new(&mut ui, "Button.flat", side, 10.0, 64.0 + i as f32 * 40.0, 200.0, 36.0);
        text(&mut ui, b, "text", label);
        text(&mut ui, b, "icon", icon);
        if i == 0 {
            ui.set_prop(b, "style", "pill")?;
            ui.set_prop(b, "checked", true)?;
        }
    }
    let me = new(&mut ui, "Avatar", side, 20.0, 740.0, 36.0, 36.0);
    text(&mut ui, me, "display_name", "Eduardo Silva");
    ui.set_prop(me, "presence", PropValue::Enum("online".into()))?;
    let me_l = new(&mut ui, "Label", side, 64.0, 740.0, 140.0, 36.0);
    text(&mut ui, me_l, "text", "Eduardo Silva");

    // ----- coluna 1: botões e campos -----
    let x0 = 244.0;
    caption(&mut ui, form, "Button: subtipos", x0, 16.0);
    let mut x = x0;
    for (class, label, w) in [("Button", "Gravar", 96.0), ("Button.secondary", "Cancelar", 96.0), ("Button.outline", "Exportar", 96.0), ("Button.danger", "Excluir", 90.0)] {
        let b = new(&mut ui, class, form, x, 38.0, w, 34.0);
        text(&mut ui, b, "text", label);
        x += w + 8.0;
    }
    let b = new(&mut ui, "Button", form, x0, 80.0, 130.0, 34.0);
    text(&mut ui, b, "text", "Reproduzir");
    text(&mut ui, b, "icon", "play");
    let mute = new(&mut ui, "Button.icon", form, x0 + 140.0, 80.0, 34.0, 34.0);
    text(&mut ui, mute, "icon", "volume");
    ui.set_prop(mute, "checkable", true)?;
    text(&mut ui, mute, "checked_icon", "volume-off");
    text(&mut ui, mute, "hint", "Mudo");
    for (i, icon) in ["mic", "camera", "monitor", "refresh"].into_iter().enumerate() {
        let b = new(&mut ui, "Button.icon", form, x0 + 182.0 + i as f32 * 40.0, 80.0, 34.0, 34.0);
        text(&mut ui, b, "icon", icon);
    }
    let link = new(&mut ui, "Button.link", form, x0 + 350.0, 80.0, 70.0, 34.0);
    text(&mut ui, link, "text", "Ajuda");

    caption(&mut ui, form, "Edit: busca e Badge", x0, 126.0);
    let search = new(&mut ui, "Edit.search", form, x0, 148.0, 200.0, 34.0);
    text(&mut ui, search, "text", "reunião");
    let mut bx = x0 + 210.0;
    for (class, t, w) in [("Badge.live", "AO VIVO", 96.0), ("Badge.success", "Pronto", 66.0), ("Badge.warning", "5.0 ms", 66.0)] {
        let b = new(&mut ui, class, form, bx, 154.0, w, 22.0);
        text(&mut ui, b, "text", t);
        bx += w + 6.0;
    }

    caption(&mut ui, form, "SegmentedControl", x0, 192.0);
    let seg = new(&mut ui, "SegmentedControl", form, x0, 214.0, 300.0, 34.0);
    text(&mut ui, seg, "items", "Tela;Janela;Área");
    text(&mut ui, seg, "icons", "monitor;window;crop");

    caption(&mut ui, form, "Banner", x0, 258.0);
    let ban = new(&mut ui, "Banner.warning", form, x0, 280.0, 430.0, 62.0);
    text(&mut ui, ban, "text", "O disco de gravação está com menos de 5 GB livres.");
    ui.set_prop(ban, "closable", true)?;
    let ban2 = new(&mut ui, "Banner.success", form, x0, 350.0, 430.0, 62.0);
    text(&mut ui, ban2, "text", "Exportação concluída: video-2026-10-25.mp4");

    caption(&mut ui, form, "Avatar, presença e HotkeyEdit", x0, 422.0);
    for (i, (name, presence)) in [("Ana Lima", "online"), ("Bruno Costa", "away"), ("Carla Dias", "busy"), ("Diego", "offline")].into_iter().enumerate() {
        let a = new(&mut ui, "Avatar", form, x0 + i as f32 * 48.0, 444.0, 40.0, 40.0);
        text(&mut ui, a, "display_name", name);
        ui.set_prop(a, "presence", PropValue::Enum(presence.into()))?;
    }
    let hk = new(&mut ui, "HotkeyEdit", form, x0 + 200.0, 448.0, 200.0, 34.0);
    text(&mut ui, hk, "hotkey", "Ctrl+Shift+R");

    caption(&mut ui, form, "LevelMeter, SeekBar, Waveform", x0, 494.0);
    for (i, v) in [0.42f32, 0.78, 0.95].into_iter().enumerate() {
        let m = new(&mut ui, "LevelMeter", form, x0, 518.0 + i as f32 * 14.0, 300.0, 8.0);
        ui.set_prop(m, "value", v)?;
    }
    for (i, v) in [0.55f32, 0.86].into_iter().enumerate() {
        let m = new(&mut ui, "LevelMeter.vertical", form, x0 + 320.0 + i as f32 * 18.0, 506.0, 12.0, 110.0);
        ui.set_prop(m, "value", v)?;
    }
    let seek = new(&mut ui, "SeekBar", form, x0, 566.0, 300.0, 28.0);
    ui.set_prop(seek, "duration", 754.0)?;
    ui.set_prop(seek, "position", 230.0)?;
    ui.set_prop(seek, "buffered", 420.0)?;
    text(&mut ui, seek, "markers", "120;505");
    let wave = new(&mut ui, "Waveform", form, x0, 600.0, 300.0, 34.0);
    ui.set_prop(wave, "position", 4.0)?;
    new(&mut ui, "ActivityIndicator", form, x0 + 380.0, 508.0, 36.0, 36.0);

    caption(&mut ui, form, "DirectionPad e Chart (rosca)", x0, 646.0);
    new(&mut ui, "DirectionPad", form, x0, 668.0, 130.0, 130.0);
    let donut = ui.add(Chart::new(ChartKind::Donut), form);
    put(&mut ui, donut, x0 + 150.0, 668.0, 280.0, 140.0);
    text(&mut ui, donut, "labels", "Pessoas;Veículos;Animais");
    text(&mut ui, donut, "series", "Eventos=42,18,7");
    ui.set_prop(donut, "legend", false).ok();

    // ----- coluna 2: grafo e conversa -----
    let x1 = 700.0;
    caption(&mut ui, form, "NodeGraph", x1, 16.0);
    let mut g = NodeGraph::default();
    g.add_node(GraphNode::new("win", "Windows", 0.0, 0.0).subtitle("Tocando em Alto-falantes").icon("monitor").color(Color::rgb(0xF5, 0x9E, 0x0B)).source().level(0.6));
    g.add_node(GraphNode::new("mic", "Microfone (USB)", 0.0, 120.0).subtitle("Voz").icon("mic").source().level(0.8).badge("exclusivo").badge("5.0 ms"));
    g.add_node(GraphNode::new("fone", "Fones de ouvido", 360.0, 24.0).subtitle("Saída").icon("headphones").color(Color::rgb(0x22, 0xC5, 0x5E)).sink().level(0.7));
    g.add_node(GraphNode::new("cx", "Caixas de som", 360.0, 144.0).subtitle("Saída").icon("volume").color(Color::rgb(0x22, 0xC5, 0x5E)).sink().level(0.4).badge("padrão"));
    g.connect(GraphCable::new("c1", "mic", "fone").label("0.0 dB"));
    g.connect(GraphCable::new("c2", "win", "cx").label("padrão"));
    g.connect(GraphCable::new("c3", "mic", "cx").label("mudo").dashed(true));
    let graph = ui.add(g, form);
    put(&mut ui, graph, x1, 38.0, 560.0, 330.0);

    caption(&mut ui, form, "ChatView", x1, 380.0);
    let mut chat = ChatView::default();
    chat.add_message(ChatMessage::system("d1", "Hoje"));
    chat.add_message(ChatMessage::new("1", "Ana Lima", "Oi! Conseguiu ver a gravação de ontem?").at("09:12"));
    chat.add_message(ChatMessage::new("2", "Ana Lima", "A câmera da garagem pegou movimento às 3h").at("09:12"));
    chat.add_message(ChatMessage::new("3", "Eu", "Vi sim, era um gato 🐈").mine().at("09:15").status(MessageStatus::Read));
    chat.add_message(
        ChatMessage::new("4", "Bruno Costa", "Kkkk de novo esse gato")
            .at("09:16")
            .reply_to("Eu", "Vi sim, era um gato"),
    );
    chat.add_message(ChatMessage::new("5", "Eu", "Vou mudar a zona de detecção pra ignorar o muro").mine().at("09:18").status(MessageStatus::Delivered));
    let c = ui.add(chat, form);
    put(&mut ui, c, x1, 402.0, 560.0, 400.0);

    aylis_platform_winit::run(ui, form)
}
