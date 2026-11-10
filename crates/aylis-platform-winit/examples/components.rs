//! Os componentes da fase 16 numa tela só: abas, campos, listas, gráficos, timer,
//! diálogos e toast.
//!
//! ```text
//! cargo run -p aylis-platform-winit --example components [-- --dark]
//! ```

use aylis_controls::*;
use aylis_core::platform::LocaleInfo;
use aylis_core::*;

fn put(ui: &mut Ui, id: ComponentId, x: f32, y: f32, w: f32, h: f32) -> ComponentId {
    ui.control_mut(id).unwrap().set_rect(x, y, w, h);
    id
}

fn caption(ui: &mut Ui, parent: ComponentId, text: &str, x: f32, y: f32) {
    let l = ui.add(Label::new(text), parent);
    put(ui, l, x, y, 220.0, 20.0);
    let muted = ui.theme().color("text_muted");
    let _ = ui.set_prop(l, "color", muted);
    let _ = ui.set_prop(l, "font_size", 12);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dark = std::env::args().any(|a| a == "--dark");
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    ui.services_mut().locale = Some(Box::new(LocaleInfo::pt_br()));
    ui.set_theme(if dark { Theme::dark() } else { Theme::light() });
    let form = ui.add_root(Form::new("aylis — componentes", 1100.0, 720.0));
    ui.set_name(form, "frmComponentes")?;

    // ----- barra de ferramentas e de status -----
    let toolbar = ui.add(ToolBar::default(), form);
    let tool = |ui: &mut Ui, text: &str, w: f32| {
        let b = ui.add(Button::new(text), toolbar);
        ui.control_mut(b).unwrap().set_align(Align::Left).set_width(w).set_margins([0.0, 0.0, 6.0, 0.0]);
        b
    };
    let b_msg = tool(&mut ui, "MessageDlg…", 120.0);
    let b_toast = tool(&mut ui, "Toast", 80.0);
    let b_open = tool(&mut ui, "Abrir arquivo…", 130.0);
    let _ = ui.set_prop(b_toast, "style", "flat");
    let _ = ui.set_prop(b_open, "style", "flat");
    let status = ui.add(StatusBar::new("Pronto"), form);
    let clock = ui.add(Label::new(""), status);
    ui.control_mut(clock).unwrap().set_align(Align::Right).set_width(90.0);

    let open = ui.add(OpenDialog { filter: "Imagens|*.png;*.jpg|Todos|*.*".into(), title: "Escolha um arquivo".into(), ..Default::default() }, form);
    ui.on_click(b_open, move |ui, _| OpenDialog::execute(ui, open));
    ui.on(open, "on_select", move |ui, _, args| {
        if let EventArgs::Text(path) = args {
            let _ = ui.set_prop(status, "text", format!("Escolhido: {path}"));
        }
    });
    ui.on_click(b_toast, |ui, _| toast(ui, "Salvo com sucesso"));
    ui.on_click(b_msg, move |ui, _| {
        message_dlg(ui, "Deseja excluir o cliente Maria Silva?\nEsta ação não pode ser desfeita.", DlgKind::Confirmation, &[ModalResult::Yes, ModalResult::No], move |ui, r| {
            let _ = ui.set_prop(status, "text", format!("MessageDlg: {}", r.name()));
        });
    });

    // Relógio na barra de status (Timer de 1 s).
    let timer = ui.add(Timer::new(1000), form);
    ui.on(timer, "on_timer", move |ui, _, _| {
        let secs = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs());
        let _ = ui.set_prop(clock, "text", format!("{:02}:{:02}:{:02} UTC", secs / 3600 % 24, secs / 60 % 60, secs % 60));
    });

    // ----- abas -----
    let tabs = ui.add(TabControl::default(), form);
    ui.control_mut(tabs).unwrap().set_align(Align::Client).set_margins(12.0);
    let input = TabControl::add_tab(&mut ui, tabs, "Entrada de dados");
    let lists = TabControl::add_tab(&mut ui, tabs, "Listas e árvore");
    let charts = TabControl::add_tab(&mut ui, tabs, "Gráficos");

    // Entrada
    caption(&mut ui, input, "Memo", 8.0, 4.0);
    let memo = ui.add(Memo::new("Observações do pedido:\n- entregar pela manhã\n- conferir o CEP"), input);
    put(&mut ui, memo, 8.0, 26.0, 320.0, 130.0);
    caption(&mut ui, input, "NumberBox (0 a 100, 2 casas)", 8.0, 170.0);
    let qty = ui.add(NumberBox::new(12.5, 0.0, 100.0), input);
    let _ = ui.set_prop(qty, "decimals", 2u32);
    let _ = ui.set_prop(qty, "value", 12.5);
    put(&mut ui, qty, 8.0, 192.0, 150.0, 32.0);
    caption(&mut ui, input, "TrackBar", 8.0, 236.0);
    let track = ui.add(TrackBar::new(40.0, 0.0, 100.0), input);
    put(&mut ui, track, 8.0, 258.0, 320.0, 28.0);
    ui.on(track, "on_change", move |ui, _, args| {
        if let EventArgs::Value(v) = args {
            let _ = ui.set_prop(qty, "value", v.clone());
        }
    });
    caption(&mut ui, input, "DateEdit", 8.0, 298.0);
    let date = ui.add(DateEdit::new("2026-10-24"), input);
    put(&mut ui, date, 8.0, 320.0, 160.0, 32.0);
    caption(&mut ui, input, "TimeEdit", 180.0, 298.0);
    let time = ui.add(TimeEdit::new("14:30"), input);
    put(&mut ui, time, 180.0, 320.0, 110.0, 32.0);
    let radio = ui.add(RadioGroup::new("Forma de pagamento", &["Dinheiro", "Pix", "Cartão", "Boleto"]), input);
    let _ = ui.set_prop(radio, "columns", 2u32);
    let _ = ui.set_prop(radio, "item_index", 1i64);
    put(&mut ui, radio, 8.0, 368.0, 320.0, 100.0);
    let group = ui.add(GroupBox::new("Entrega"), input);
    put(&mut ui, group, 350.0, 4.0, 300.0, 150.0);
    for (i, text) in ["Retirar na loja", "Entregar no endereço", "Agendar"].iter().enumerate() {
        let c = ui.add(CheckBox::new(*text), group);
        put(&mut ui, c, 14.0, 30.0 + i as f32 * 34.0, 240.0, 26.0);
    }
    caption(&mut ui, input, "Calendar", 350.0, 170.0);
    let cal = ui.add(Calendar::new("2026-10-24"), input);
    put(&mut ui, cal, 350.0, 192.0, 300.0, 290.0);
    ui.on(cal, "on_change", move |ui, _, args| {
        if let EventArgs::Text(d) = args {
            let _ = ui.set_prop(date, "date", d.clone());
        }
    });
    let exp = ui.add(Expander::new("Opções avançadas"), input);
    put(&mut ui, exp, 680.0, 4.0, 360.0, 180.0);
    let sw = ui.add(Switch::new("Enviar e-mail ao cliente"), exp);
    put(&mut ui, sw, 12.0, 52.0, 300.0, 28.0);
    let pb = ui.add(ProgressBar::default(), exp);
    let _ = ui.set_prop(pb, "value", 65.0);
    put(&mut ui, pb, 12.0, 100.0, 320.0, 8.0);
    let exp2 = ui.add(Expander::new("Fechado (clique para abrir)"), input);
    put(&mut ui, exp2, 680.0, 200.0, 360.0, 160.0);
    let l = ui.add(Label::new("Conteúdo escondido"), exp2);
    put(&mut ui, l, 12.0, 56.0, 300.0, 24.0);
    let _ = ui.set_prop(exp2, "expanded", false);

    // Listas
    caption(&mut ui, lists, "ListBox", 8.0, 4.0);
    let lb = ui.add(ListBox::new(&["São Paulo", "Rio de Janeiro", "Belo Horizonte", "Curitiba", "Porto Alegre", "Salvador", "Recife", "Fortaleza", "Manaus", "Belém"]), lists);
    let _ = ui.set_prop(lb, "item_index", 3i64);
    put(&mut ui, lb, 8.0, 26.0, 220.0, 260.0);
    caption(&mut ui, lists, "ListView (5 mil itens, virtualizado)", 248.0, 4.0);
    let lv = ui.add(ListView::default(), lists);
    let names = ["Maria Silva", "João Pereira", "Ana Costa", "Carlos Souza", "Paula Nogueira"];
    let items = (0..5000).map(|i| ListItem::new(format!("{} #{}", names[i % 5], i + 1), format!("Pedido {} · R$ {},{:02}", 1000 + i, (i * 37) % 900 + 10, i % 100))).collect();
    ui.get_mut::<ListView>(lv).unwrap().set_items(items);
    let _ = ui.set_prop(lv, "accessory", true);
    let _ = ui.set_prop(lv, "item_index", 1i64);
    put(&mut ui, lv, 248.0, 26.0, 360.0, 460.0);
    caption(&mut ui, lists, "TreeView", 628.0, 4.0);
    let tv = ui.add(TreeView::default(), lists);
    let _ = ui.set_prop(tv, "items", "Brasil;>Sudeste;>>São Paulo;>>Rio de Janeiro;>>Minas Gerais;>Sul;>>Paraná;>>Santa Catarina;>Nordeste;>>Bahia;Portugal;>Lisboa;>Porto");
    ui.get_mut::<TreeView>(tv).unwrap().expand_all(true);
    put(&mut ui, tv, 628.0, 26.0, 300.0, 460.0);
    ui.on(tv, "on_change", move |ui, _, args| {
        if let EventArgs::Text(p) = args {
            let _ = ui.set_prop(status, "text", format!("TreeView: {p}"));
        }
    });

    // Gráficos
    let line = ui.add(Chart::new(ChartKind::Line), charts);
    let _ = ui.set_prop(line, "title", "Vendas x custos (mil R$)");
    put(&mut ui, line, 8.0, 8.0, 500.0, 260.0);
    let bars = ui.add(Chart::new(ChartKind::Bar), charts);
    let _ = ui.set_prop(bars, "title", "Por trimestre");
    let _ = ui.set_prop(bars, "labels", "T1;T2;T3;T4");
    let _ = ui.set_prop(bars, "series", "2025=120,140,135,180|2026=150,170,190,210");
    put(&mut ui, bars, 520.0, 8.0, 520.0, 260.0);
    let area = ui.add(Chart::new(ChartKind::Area), charts);
    let _ = ui.set_prop(area, "title", "Acessos por hora");
    let _ = ui.set_prop(area, "labels", "0h;3h;6h;9h;12h;15h;18h;21h");
    let _ = ui.set_prop(area, "series", "Acessos=12,5,9,48,77,64,90,40");
    put(&mut ui, area, 8.0, 280.0, 500.0, 220.0);
    let pie = ui.add(Chart::new(ChartKind::Pie), charts);
    let _ = ui.set_prop(pie, "title", "Pagamentos");
    let _ = ui.set_prop(pie, "labels", "Pix;Cartão;Dinheiro;Boleto");
    let _ = ui.set_prop(pie, "series", "Pagamentos=45,30,15,10");
    put(&mut ui, pie, 520.0, 280.0, 520.0, 220.0);

    // Para a verificação: AYLIS_TAB=1 abre direto na aba.
    if let Some(i) = std::env::var("AYLIS_TAB").ok().and_then(|t| t.parse::<i64>().ok()) {
        let _ = ui.set_prop(tabs, "tab_index", i);
    }
    aylis_platform_winit::run(ui, form)
}
