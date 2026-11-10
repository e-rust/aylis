//! Monta o form de cadastro usado pelos exemplos `cadastro` e `compare`.

use std::f32::consts::PI;

use aylis_controls::{Button, CheckBox, Edit, Form, Label, Layout, Panel, ProgressBar, RadioButton, Rectangle, Switch};
use aylis_core::*;

/// Cria o `Ui` com o form de cadastro (tema claro ou escuro) e devolve o form.
pub fn build(dark: bool) -> Result<(Ui, ComponentId), Box<dyn std::error::Error>> {
    let mut ui = Ui::new();
    if dark {
        ui.set_theme(Theme::dark());
    }
    aylis_controls::register(&mut ui);
    ui.register_class::<GlowButton>("Exemplo");
    let theme = ui.theme().clone();

    let form = ui.add_root(Form::new("aylis — Cadastro de Clientes", 900.0, 600.0));
    ui.set_name(form, "FrmClientes")?;
    ui.control_mut(form).unwrap().set_padding(8.0);

    // ----- status (criado antes para os botões poderem escrever nele) -----
    let status = ui.add(Layout::default(), form);
    ui.control_mut(status).unwrap().set_align(Align::Bottom).set_height(24.0);
    let status_text = ui.add(Label::new("Passe o mouse e clique nos botões · Tab muda o foco · Enter/Espaço clicam"), status);
    ui.control_mut(status_text).unwrap().set_align(Align::Client);
    ui.set_name(status_text, "lblStatus")?;
    ui.set_prop(status_text, "color", theme.color("text_muted"))?;
    ui.set_prop(status_text, "font_size", 12)?;

    // ----- toolbar -----
    let toolbar = ui.add(Panel::new(0.0, 48.0), form);
    ui.control_mut(toolbar).unwrap().set_align(Align::Top).set_padding(6.0).set_margins([0.0, 0.0, 0.0, 8.0]);
    ui.set_child_index(toolbar, 0)?;

    let new_item = toolbar_button(&mut ui, toolbar, Button::new("Novo"));
    ui.on_click(new_item, |ui, _| say(ui, "Novo cliente"));

    let save = toolbar_button(&mut ui, toolbar, GlowButton::new("Salvar"));
    let saves = std::cell::Cell::new(0);
    ui.on_click(save, move |ui, _| {
        saves.set(saves.get() + 1);
        say(ui, &format!("Salvo ({}x) — o GlowButton é um Button \"herdado\" com borda animada", saves.get()));
    });

    let delete = toolbar_button(&mut ui, toolbar, Button::new("Excluir"));
    ui.control_mut(delete).unwrap().set_style("danger");
    ui.on_click(delete, |ui, _| say(ui, "Excluir clicado (variante \"danger\" do tema)"));

    let help = toolbar_button(&mut ui, toolbar, Button::new("Ajuda"));
    ui.control_mut(help).unwrap().set_style("flat");
    ui.on_click(help, |ui, _| say(ui, "Botão \"flat\": transparente, só aparece no hover"));

    let disabled = toolbar_button(&mut ui, toolbar, Button::new("Desabilitado"));
    ui.control_mut(disabled).unwrap().set_enabled(false).set_width(110.0);

    let search = ui.add(Edit::default().with_placeholder("Pesquisar cliente…"), toolbar);
    ui.control_mut(search).unwrap().set_align(Align::Right).set_width(220.0);
    ui.on(search, "on_change", |ui, _, args| {
        if let EventArgs::Value(PropValue::String(text)) = args {
            say(ui, &format!("Pesquisando: {text}"));
        }
    });

    // ----- menu lateral: cada item clareia no hover com uma ColorAnimation -----
    let menu = ui.add(Panel::new(180.0, 0.0), form);
    ui.control_mut(menu).unwrap().set_align(Align::Left).set_padding(8.0).set_margins([0.0, 0.0, 8.0, 0.0]);
    let selected = theme.color("primary").with_opacity(0.15);
    let hover = theme.color("text").with_opacity(0.07);
    for (i, item) in ["Clientes", "Pedidos", "Produtos", "Financeiro", "Relatórios"].into_iter().enumerate() {
        let row = ui.add(Rectangle::new(0.0, 34.0), menu);
        ui.control_mut(row).unwrap().set_align(Align::Top).set_margins([0.0, 0.0, 0.0, 2.0]).set_cursor(CursorShape::Pointer);
        ui.set_prop(row, "radius", 6)?;
        ui.set_prop(row, "fill", if i == 0 { selected } else { Color::TRANSPARENT })?;
        if i != 0 {
            ui.add(PropertyAnimation::color("fill", hover).with_trigger(Trigger::Hover).with_duration(150.0), row);
        }
        let text = ui.add(Label::new(item), row);
        ui.control_mut(text).unwrap().set_align(Align::Client).set_margins([12.0, 0.0, 8.0, 0.0]);
        ui.on_click(row, move |ui, _| say(ui, &format!("Menu: {item}")));
    }

    // ----- conteúdo -----
    let content = ui.add(Panel::new(0.0, 0.0), form);
    ui.control_mut(content).unwrap().set_align(Align::Client).set_padding(16.0);

    let title = ui.add(Label::new("Cliente"), content);
    ui.control_mut(title).unwrap().set_align(Align::Top).set_height(32.0);
    ui.set_prop(title, "font_size", 22)?;

    // Card com pintura própria; no hover desliza para a direita (FloatAnimation em
    // "margins.left") e no duplo clique avisa.
    let card = ui.add(Layout::default(), content);
    ui.control_mut(card).unwrap().set_align(Align::Top).set_height(90.0).set_margins([0.0, 8.0, 0.0, 16.0]).set_hit_test(true);
    ui.add(PropertyAnimation::float("margins.left", 10.0).with_trigger(Trigger::Hover).with_duration(250.0).with_easing(Easing::OutBack), card);
    ui.on(card, "on_dbl_click", |ui, _, _| say(ui, "Duplo clique no card"));
    ui.on_paint(card, |canvas, ctx, _default| {
        let r = ctx.rect;
        let brush = Brush::Linear {
            start: PointF::new(0.0, 0.0),
            end: PointF::new(r.width, r.height),
            stops: vec![
                GradientStop::new(0.0, ctx.theme.color("primary")),
                GradientStop::new(1.0, ctx.theme.color("accent")),
            ],
        };
        canvas.fill_rect(r, 10.0, &brush);
        let mut text = ctx.text_style();
        text.color = Color::WHITE;
        text.font.size = 18.0;
        canvas.draw_text("Maria da Silva", RectF::new(20.0, 14.0, r.width - 40.0, 30.0), &text);
        text.font.size = 13.0;
        text.color = Color::WHITE.with_opacity(0.85);
        canvas.draw_text("Cliente desde 2019  ·  limite R$ 12.500,00", RectF::new(20.0, 46.0, r.width - 40.0, 24.0), &text);
    });

    let mut edits = Vec::new();
    for (caption, edit) in [
        ("Nome", Edit::new("Maria da Silva")),
        ("CPF", Edit::default().with_placeholder("000.000.000-00")),
        ("E-mail", Edit::new("maria@exemplo.com.br")),
        ("Cidade", Edit::default().with_placeholder("Cidade / UF")),
        ("Senha", Edit::new("segredo123")),
    ] {
        edits.push(field(&mut ui, content, caption, edit, &theme));
    }
    ui.set_prop(edits[1], "max_length", 14)?;
    ui.set_prop(edits[4], "password", true)?;

    // Opções: CheckBox, Switch e RadioButtons lado a lado.
    let options = ui.add(Layout::default(), content);
    ui.control_mut(options).unwrap().set_align(Align::Top).set_height(28.0).set_margins([90.0, 6.0, 0.0, 0.0]);
    let active = ui.add(CheckBox::new("Ativo"), options);
    ui.control_mut(active).unwrap().set_align(Align::Left).set_width(80.0);
    ui.set_prop(active, "checked", true)?;
    let emails = ui.add(Switch::new("Receber e-mails"), options);
    ui.control_mut(emails).unwrap().set_align(Align::Left).set_width(170.0);
    for (i, kind) in ["Pessoa física", "Pessoa jurídica"].into_iter().enumerate() {
        let radio = ui.add(RadioButton::new(kind), options);
        ui.control_mut(radio).unwrap().set_align(Align::Left).set_width(140.0);
        ui.set_prop(radio, "checked", i == 0)?;
    }
    for id in ui.children(options).to_vec() {
        ui.on(id, "on_change", move |ui, sender, args| {
            let name = ui.get_prop(sender, "text").ok().and_then(|v| v.to_str().ok().map(str::to_string)).unwrap_or_default();
            if let EventArgs::Value(PropValue::Bool(on)) = args {
                say(ui, &format!("{name}: {}", if *on { "marcado" } else { "desmarcado" }));
            }
        });
    }

    // Progresso do cadastro: fração de campos preenchidos, atualizada a cada digitação.
    let progress_row = ui.add(Layout::default(), content);
    ui.control_mut(progress_row).unwrap().set_align(Align::Top).set_height(20.0).set_margins([90.0, 14.0, 0.0, 0.0]);
    let progress_label = ui.add(Label::new(""), progress_row);
    ui.control_mut(progress_label).unwrap().set_align(Align::Right).set_width(150.0).set_margins([10.0, 0.0, 0.0, 0.0]);
    ui.set_prop(progress_label, "color", theme.color("text_muted"))?;
    ui.set_prop(progress_label, "font_size", 12)?;
    let progress = ui.add(ProgressBar::default(), progress_row);
    ui.control_mut(progress).unwrap().set_align(Align::VertCenter).set_height(8.0);
    let update_progress = move |ui: &mut Ui, edits: &[ComponentId]| {
        let filled = edits.iter().filter(|&&e| ui.get::<Edit>(e).is_some_and(|e| !e.text.is_empty())).count();
        let value = 100.0 * filled as f32 / edits.len() as f32;
        let _ = ui.animate(progress, "value", value, AnimationOptions::new(300.0, Easing::OutCubic));
        let _ = ui.set_prop(progress_label, "text", format!("{filled} de {} campos preenchidos", edits.len()));
    };
    update_progress(&mut ui, &edits);
    for &edit in &edits {
        let edits = edits.clone();
        ui.on(edit, "on_change", move |ui, _, _| update_progress(ui, &edits));
    }

    // Ancorado embaixo à direita: acompanha o canto quando a janela muda de tamanho.
    let ok = ui.add(Button::new("OK"), content);
    ui.control_mut(ok).unwrap().set_rect(530.0, 440.0, 110.0, 34.0).set_anchors(Anchors::RIGHT | Anchors::BOTTOM);
    ui.on_click(ok, |ui, _| say(ui, "OK!"));

    Ok((ui, form))
}

fn say(ui: &mut Ui, text: &str) {
    if let Some(label) = ui.find("lblStatus") {
        let _ = ui.set_prop(label, "text", text);
    }
}

fn toolbar_button(ui: &mut Ui, parent: ComponentId, button: impl Component) -> ComponentId {
    let b = ui.add(button, parent);
    ui.control_mut(b).unwrap().set_align(Align::Left).set_margins([0.0, 0.0, 6.0, 0.0]);
    b
}

/// Linha "rótulo: campo". Devolve o campo.
fn field(ui: &mut Ui, parent: ComponentId, caption: &str, edit: Edit, theme: &Theme) -> ComponentId {
    let row = ui.add(Layout::default(), parent);
    ui.control_mut(row).unwrap().set_align(Align::Top).set_height(34.0).set_margins([0.0, 0.0, 0.0, 6.0]);
    let label = ui.add(Label::new(caption), row);
    ui.control_mut(label).unwrap().set_align(Align::Left).set_width(90.0);
    ui.set_prop(label, "color", theme.color("text_muted")).unwrap();
    let edit = ui.add(edit, row);
    ui.control_mut(edit).unwrap().set_align(Align::Client);
    edit
}

/// Um `Button` "herdado": todo o comportamento (hover, pressed, foco, Enter, `on_click`,
/// propriedades, estilo do tema) vem do `Button`; só acrescenta uma borda em gradiente
/// cônico que gira enquanto o mouse está em cima.
#[derive(Default)]
pub struct GlowButton {
    base: Button,
}

impl GlowButton {
    pub fn new(text: &str) -> Self {
        Self { base: Button::new(text) }
    }
}

impl Component for GlowButton {
    fn class_name(&self) -> &'static str {
        "GlowButton"
    }

    fn inner(&self) -> Option<&dyn Component> {
        Some(&self.base)
    }

    fn inner_mut(&mut self) -> Option<&mut dyn Component> {
        Some(&mut self.base)
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        self.base.paint(canvas, ctx); // inherited Paint
        if !ctx.state.hover || ctx.state.disabled {
            return;
        }
        let angle = (ctx.time as f32 * 240.0) % 360.0;
        let accent = ctx.theme.color("accent");
        let primary = ctx.theme.color("primary");
        let brush = Brush::Sweep {
            center: ctx.rect.center(),
            start_angle: angle,
            stops: vec![
                GradientStop::new(0.0, accent),
                GradientStop::new(0.5, Color::rgb(34, 211, 238)),
                GradientStop::new(1.0, accent),
            ],
        };
        canvas.stroke_rect(ctx.rect, ctx.style.radius, &Stroke::new(brush, 2.5));
        // Brilho suave que "respira".
        let glow = 0.25 + 0.15 * (ctx.time as f32 * 2.0 * PI).sin();
        canvas.stroke_rect(ctx.rect.inflate(2.0), ctx.style.radius + 2.0, &Stroke::new(primary.with_opacity(glow), 2.0));
        ctx.request_frame(); // continua animando enquanto estiver em hover
    }
}
