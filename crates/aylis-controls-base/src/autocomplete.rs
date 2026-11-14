//! `AutoComplete`: campo de texto com sugestões enquanto digita (o `v-autocomplete` /
//! `v-combobox` do Vuetify). É um [`Edit`] (todas as propriedades dele valem) com a lista
//! `items` filtrada pelo que foi digitado.

use std::any::Any;

use aylis_core::{
    Brush, Canvas, Color, Component, ComponentId, ControlBase, EventArgs, EventCtx, Key, KeyEvent, PaintCtx, PointerEvent, RectF,
    Stroke, properties,
};

use crate::Edit;

const ROW: f32 = 30.0;

/// Escolha feita na lista (mensagem da lista para o campo).
struct Suggested(usize);

/// Campo com sugestões. `items` são as opções (separadas por `;`); a lista abre ao
/// digitar, filtrando por trecho (sem diferenciar maiúsculas e acentos). Setas escolhem,
/// Enter confirma, Esc fecha. `strict` só aceita um dos itens (o resto é apagado ao sair).
/// `on_select` quando um item é escolhido.
pub struct AutoComplete {
    pub base: Edit,
    /// As opções sugeridas, separadas por `;`; a lista mostra as que contêm o digitado.
    pub items: String,
    /// Quantas linhas a lista de sugestões mostra de uma vez (as outras rolam; mínimo 1).
    pub max_rows: u32,
    /// Só aceita um dos itens: o texto que não for um deles é apagado ao sair do campo.
    pub strict: bool,
    popup: Option<ComponentId>,
    matches: Vec<String>,
    active: Option<usize>,
}

impl Default for AutoComplete {
    fn default() -> Self {
        let mut base = Edit::default();
        base.placeholder = "Digite para buscar…".into();
        Self {
            base,
            items: "Belo Horizonte;Brasília;Curitiba;Florianópolis;Fortaleza;Manaus;Porto Alegre;Recife;Rio de Janeiro;Salvador;São Paulo".into(),
            max_rows: 6,
            strict: false,
            popup: None,
            matches: Vec::new(),
            active: None,
        }
    }
}

/// Minúsculas e sem acentos (para comparar "sao" com "São").
pub fn fold(text: &str) -> String {
    text.chars()
        .flat_map(char::to_lowercase)
        .map(|c| match c {
            'á' | 'à' | 'â' | 'ã' | 'ä' => 'a',
            'é' | 'è' | 'ê' | 'ë' => 'e',
            'í' | 'ì' | 'î' | 'ï' => 'i',
            'ó' | 'ò' | 'ô' | 'õ' | 'ö' => 'o',
            'ú' | 'ù' | 'û' | 'ü' => 'u',
            'ç' => 'c',
            'ñ' => 'n',
            c => c,
        })
        .collect()
}

impl AutoComplete {
    pub fn new(items: &str) -> Self {
        Self { items: items.into(), ..Default::default() }
    }

    /// As opções que contêm o texto (as que começam com ele primeiro).
    pub fn filter(&self, text: &str) -> Vec<String> {
        let q = fold(text.trim());
        if q.is_empty() {
            return Vec::new();
        }
        let mut starts = Vec::new();
        let mut contains = Vec::new();
        for item in self.items.split(';').filter(|i| !i.is_empty()) {
            let f = fold(item);
            if f == q {
                continue; // já está escrito por inteiro
            }
            if f.starts_with(&q) {
                starts.push(item.to_string());
            } else if f.contains(&q) {
                contains.push(item.to_string());
            }
        }
        starts.extend(contains);
        starts
    }

    fn refresh(&mut self, ctx: &mut EventCtx<'_>) {
        self.matches = self.filter(&self.base.text);
        self.active = None;
        if self.matches.is_empty() {
            return self.close(ctx);
        }
        let rows = self.matches.len().min(self.max_rows.max(1) as usize);
        let height = rows as f32 * ROW + 6.0;
        let root = ctx.ui.root_of(ctx.id);
        let rect = ctx.ui.to_form(ctx.id, self.base.base.bounds());
        let list = Suggestions { base: ControlBase::new(rect.width, height), owner: ctx.id, items: self.matches.clone(), active: None, query: self.base.text.clone() };
        match self.popup {
            Some(p) if ctx.ui.contains(p) => {
                if let Some(s) = ctx.ui.get_mut::<Suggestions>(p) {
                    s.items = list.items;
                    s.active = None;
                    s.query = list.query;
                }
                if let Some(cb) = ctx.ui.control_mut(p) {
                    cb.set_rect(rect.x, rect.bottom() + 2.0, rect.width, height);
                }
            }
            _ => {
                let id = ctx.ui.add(list, root);
                if let Some(cb) = ctx.ui.control_mut(id) {
                    cb.set_rect(rect.x, rect.bottom() + 2.0, rect.width, height);
                }
                ctx.ui.follow(id, ctx.id);
                self.popup = Some(id);
            }
        }
        ctx.repaint();
    }

    fn close(&mut self, ctx: &mut EventCtx<'_>) {
        if let Some(p) = self.popup.take() {
            ctx.ui.free(p);
            ctx.repaint();
        }
        self.active = None;
    }

    fn choose(&mut self, ctx: &mut EventCtx<'_>, i: usize) {
        if let Some(item) = self.matches.get(i).cloned() {
            let _ = self.base.set_prop("text", aylis_core::PropValue::String(item.clone()));
            ctx.fire("on_change", EventArgs::Value(item.clone().into()));
            ctx.fire("on_select", EventArgs::Text(item));
        }
        self.close(ctx);
    }

    fn highlight(&mut self, ctx: &mut EventCtx<'_>, i: Option<usize>) {
        self.active = i;
        if let Some(s) = self.popup.and_then(|p| ctx.ui.get_mut::<Suggestions>(p)) {
            s.active = i;
        }
        ctx.repaint();
    }
}

impl Component for AutoComplete {
    fn class_name(&self) -> &'static str {
        "AutoComplete"
    }

    fn inner(&self) -> Option<&dyn Component> {
        Some(&self.base)
    }

    fn inner_mut(&mut self) -> Option<&mut dyn Component> {
        Some(&mut self.base)
    }

    properties! {
        items: String => "Data",
        max_rows: u32 => "Appearance",
        strict: bool => "Behavior";
        events: on_select
    }

    fn text_input(&mut self, ctx: &mut EventCtx<'_>, text: &str) {
        self.base.text_input(ctx, text);
        self.refresh(ctx);
    }

    fn key_down(&mut self, ctx: &mut EventCtx<'_>, event: &KeyEvent) {
        let open = self.popup.is_some() && !self.matches.is_empty();
        match event.key {
            Key::Down if open => {
                let i = self.active.map_or(0, |i| (i + 1).min(self.matches.len() - 1));
                self.highlight(ctx, Some(i));
                ctx.set_handled();
            }
            Key::Up if open => {
                let i = self.active.map(|i| i.saturating_sub(1));
                self.highlight(ctx, i);
                ctx.set_handled();
            }
            Key::Enter if open && self.active.is_some() => {
                let i = self.active.unwrap_or(0);
                self.choose(ctx, i);
                ctx.set_handled();
            }
            Key::Escape if open => {
                self.close(ctx);
                ctx.set_handled();
            }
            _ => {
                let before = self.base.text.clone();
                self.base.key_down(ctx, event);
                if self.base.text != before {
                    self.refresh(ctx);
                }
            }
        }
    }

    fn focus_exit(&mut self, ctx: &mut EventCtx<'_>) {
        self.close(ctx);
        if self.strict && !self.base.text.is_empty() && !self.items.split(';').any(|i| i == self.base.text) {
            let _ = self.base.set_prop("text", aylis_core::PropValue::String(String::new()));
            ctx.fire("on_change", EventArgs::Value(String::new().into()));
        }
        self.base.focus_exit(ctx);
    }

    fn message(&mut self, ctx: &mut EventCtx<'_>, message: &dyn Any) {
        if let Some(Suggested(i)) = message.downcast_ref::<Suggested>() {
            self.choose(ctx, *i);
        }
    }

    fn notification(&mut self, freed: ComponentId) {
        if self.popup == Some(freed) {
            self.popup = None;
        }
    }
}

/// A lista de sugestões (filha do form, por cima de tudo).
struct Suggestions {
    base: ControlBase,
    owner: ComponentId,
    items: Vec<String>,
    active: Option<usize>,
    query: String,
}

impl Component for Suggestions {
    fn class_name(&self) -> &'static str {
        "Suggestions"
    }

    // Some junto com quem a abriu (a tela refeita não deixa a lista órfã).
    fn dies_with(&self) -> Option<ComponentId> {
        Some(self.owner)
    }

    // O clique fora (mesmo onde não recebe foco) fecha as sugestões.
    fn closes_on_outside_press(&self) -> bool {
        true
    }

    fn control(&self) -> Option<&ControlBase> {
        Some(&self.base)
    }

    fn control_mut(&mut self) -> Option<&mut ControlBase> {
        Some(&mut self.base)
    }

    fn pointer_move(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        let i = ((event.position.y - 3.0) / ROW).floor();
        let i = (i >= 0.0).then_some(i as usize).filter(|i| *i < self.items.len());
        if i != self.active {
            self.active = i;
            ctx.repaint();
        }
    }

    fn pointer_up(&mut self, ctx: &mut EventCtx<'_>, _event: &PointerEvent) {
        if let Some(i) = self.active {
            ctx.ui.send_message(self.owner, &Suggested(i));
        }
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let theme = ctx.theme;
        let r = ctx.rect;
        canvas.fill_rect(r.offset(0.0, 3.0).inflate(1.0), 7.0, &Brush::Solid(Color::BLACK.with_opacity(0.12)));
        canvas.fill_rect(r, 6.0, &Brush::Solid(theme.color("surface")));
        canvas.stroke_rect(r, 6.0, &Stroke::new(theme.color("border"), 1.0));
        let q = fold(&self.query);
        for (row, item) in self.items.iter().enumerate() {
            let line = RectF::new(r.x + 3.0, r.y + 3.0 + row as f32 * ROW, r.width - 6.0, ROW);
            if line.bottom() > r.bottom() {
                break;
            }
            if Some(row) == self.active {
                canvas.fill_rect(line, 4.0, &Brush::Solid(theme.color("primary").with_opacity(0.15)));
            }
            // O trecho digitado em negrito.
            let mut t = ctx.text_style();
            let text_r = RectF::new(line.x + 10.0, line.y, line.width - 20.0, line.height);
            let folded = fold(item);
            match folded.find(&q).filter(|_| !q.is_empty() && folded.len() == item.len()) {
                Some(at) => {
                    let (a, rest) = item.split_at(at);
                    let (b, c) = rest.split_at(q.len().min(rest.len()));
                    let mut x = text_r.x;
                    for (part, bold) in [(a, false), (b, true), (c, false)] {
                        if part.is_empty() {
                            continue;
                        }
                        t.font.weight = if bold { aylis_core::FontWeight::Bold } else { aylis_core::FontWeight::Normal };
                        let w = canvas.measure_text(part, &t).width;
                        canvas.draw_text(part, RectF::new(x, text_r.y, w + 1.0, text_r.height), &t);
                        x += w;
                    }
                }
                None => canvas.draw_text(item, text_r, &t),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_ignoring_case_and_accents() {
        let a = AutoComplete::default();
        assert_eq!(a.filter("sao"), vec!["São Paulo".to_string()]);
        let f = a.filter("r");
        assert_eq!(f.first().map(String::as_str), Some("Recife"), "começa com primeiro: {f:?}");
        assert!(f.contains(&"Curitiba".to_string()));
        assert!(a.filter("").is_empty());
        assert!(a.filter("Recife").is_empty(), "escrito por inteiro: fecha a lista");
    }
}
