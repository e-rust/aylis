//! O componente do tutorial "Seu primeiro componente": um botão que conta os cliques e mostra
//! o número num selo. Ele "herda" do `Button`: guarda um botão e o devolve em `inner`, então
//! o texto, o estilo, o hover, o foco e o `on_click` vêm dele; aqui só entra o que é novo.

use aylis_controls::Button;
use aylis_core::{Brush, Canvas, Component, EventCtx, PaintCtx, RectF, TextAlign, Ui, properties};

/// Um botão que conta quantas vezes foi clicado (pelo mouse ou pelo teclado).
pub struct CounterButton {
    /// O botão de base: o que não for sobrescrito aqui vai para ele.
    pub button: Button,
    /// Quantos cliques até agora.
    pub count: i64,
}

impl Default for CounterButton {
    fn default() -> Self {
        Self { button: Button::new("Curtir"), count: 0 }
    }
}

impl Component for CounterButton {
    fn class_name(&self) -> &'static str {
        "CounterButton"
    }

    // A "herança": o resto (propriedades, eventos, estilo) é do botão.
    fn inner(&self) -> Option<&dyn Component> {
        Some(&self.button)
    }

    fn inner_mut(&mut self) -> Option<&mut dyn Component> {
        Some(&mut self.button)
    }

    // Só a propriedade nova; `text` e as outras são do botão.
    properties! {
        count: i64 => "Behavior"
    }

    /// O clique conta e depois chama o do botão (o `inherited`), que dispara o `on_click`.
    fn click(&mut self, ctx: &mut EventCtx<'_>) {
        self.count += 1;
        ctx.repaint();
        self.button.click(ctx);
    }

    /// O botão desenha a si mesmo; por cima, o selo com o número.
    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        self.button.paint(canvas, ctx);
        let r = ctx.rect;
        let size = (r.height * 0.55).max(18.0);
        let badge = RectF::new(r.right() - size - 8.0, r.y + (r.height - size) / 2.0, size, size);
        canvas.fill_ellipse(badge, &Brush::Solid(ctx.theme.color("danger")));
        let mut text = ctx.text_style();
        text.h_align = TextAlign::Center;
        text.v_align = TextAlign::Center;
        text.color = aylis_core::Color::WHITE;
        text.font.size = size * 0.55;
        canvas.draw_text(&self.count.to_string(), badge, &text);
    }
}

/// Registra a classe na paleta (o código gerado do projeto que usa o pacote chama).
pub fn register(ui: &mut Ui) {
    ui.register_class::<CounterButton>("Standard");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_click_counts_and_the_on_click_still_fires() {
        let mut ui = Ui::new();
        aylis_controls::register(&mut ui);
        register(&mut ui);
        let button = ui.create("CounterButton", None).unwrap();
        let fired = std::rc::Rc::new(std::cell::Cell::new(0));
        let f = fired.clone();
        ui.on(button, "on_click", move |_, _, _| f.set(f.get() + 1));
        for _ in 0..3 {
            ui.click(button);
        }
        assert_eq!(ui.get_prop(button, "count").unwrap(), aylis_core::PropValue::Int(3));
        assert_eq!(fired.get(), 3, "o on_click do botão continua");
        assert_eq!(ui.get_prop(button, "text").unwrap(), aylis_core::PropValue::String("Curtir".into()), "as propriedades do botão");
    }
}
