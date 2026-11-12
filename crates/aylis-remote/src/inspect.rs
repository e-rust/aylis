//! Inspeção do aplicativo em execução: a árvore de componentes, as propriedades ao vivo e o
//! destaque de um controle na tela. Os comandos `tree`, `props` e `highlight` de
//! [`execute`](crate::execute); a IDE chega neles pelo canal de desenvolvimento cifrado
//! (a feature `dev` do app, veja `aylis-devlink`), nunca por uma porta aberta no app.

use aylis_core::{Brush, Canvas, Component, ComponentId, ControlBase, EventCtx, PaintCtx, PropKind, Stroke, Ui};
use serde_json::{Map, Value, json};

/// Árvore de componentes a partir de `root` (ou de todos os forms).
pub fn tree(ui: &Ui, root: Option<ComponentId>) -> Value {
    tree_with(ui, root, usize::MAX, false)
}

/// O [`tree`] até `depth` níveis abaixo da raiz (os de baixo viram só a contagem, em
/// `more`) e, com `visible_only`, sem os escondidos: a árvore inteira de um app médio passava
/// de 130 mil caracteres (relatório do RTSPlayer, 10/11).
pub fn tree_with(ui: &Ui, root: Option<ComponentId>, depth: usize, visible_only: bool) -> Value {
    fn node(ui: &Ui, id: ComponentId, depth: usize, visible_only: bool) -> Value {
        let class = ui.component(id).map(|c| c.class_name()).unwrap_or_default();
        let visible = ui.control(id).map(|c| c.visible());
        let kids: Vec<ComponentId> = ui.children(id).iter().copied().filter(|&c| !visible_only || ui.control(c).is_none_or(|cb| cb.visible())).collect();
        let mut v = json!({
            "id": id.to_u64(),
            "name": ui.name(id).unwrap_or_default(),
            "class": class,
            "path": ui.path_of(id),
            "visible": visible,
        });
        // Onde ficou depois do layout, relativo ao form (`left`/`width` são os pedidos no JSON;
        // relatório do InsightDB, 11/11).
        if let Some(r) = ui.absolute_rect(id) {
            v["rect"] = json!([r.x, r.y, r.width, r.height]);
        }
        if depth == 0 {
            if !kids.is_empty() {
                v["more"] = json!(kids.len());
            }
        } else {
            v["children"] = Value::Array(kids.iter().map(|&c| node(ui, c, depth - 1, visible_only)).collect());
        }
        v
    }
    match root {
        Some(r) => node(ui, r, depth, visible_only),
        None => Value::Array(ui.roots().iter().map(|&r| node(ui, r, depth, visible_only)).collect()),
    }
}

/// Todas as propriedades de `id`: nome, tipo, categoria e valor.
pub fn props(ui: &Ui, id: ComponentId) -> Value {
    let list: Vec<Value> = ui
        .prop_infos(id)
        .iter()
        .map(|info| {
            let value = ui.get_prop(id, info.name).map(|v| aylis_json::prop_to_json(ui, &v)).unwrap_or(Value::Null);
            let (kind, options): (&str, &[&str]) = match info.kind {
                PropKind::Bool => ("bool", &[]),
                PropKind::Int => ("int", &[]),
                PropKind::Float => ("float", &[]),
                PropKind::String => ("string", &[]),
                PropKind::Color => ("color", &[]),
                PropKind::Enum(o) => ("enum", o),
                PropKind::Set(o) => ("set", o),
                PropKind::Bounds => ("bounds", &[]),
                PropKind::Component => ("component", &[]),
                PropKind::Event => ("event", &[]),
                PropKind::Angle => ("angle", &[]),
                PropKind::Duration => ("duration", &[]),
                PropKind::Shadow => ("shadow", &[]),
                PropKind::Easing => ("easing", &[]),
            };
            let mut m = Map::new();
            m.insert("name".into(), json!(info.name));
            m.insert("kind".into(), json!(kind));
            m.insert("category".into(), json!(info.category));
            m.insert("read_only".into(), json!(info.read_only));
            if !options.is_empty() {
                m.insert("options".into(), json!(options));
            }
            m.insert("value".into(), value);
            Value::Object(m)
        })
        .collect();
    Value::Array(list)
}

/// Destaca o controle `id` por um instante (uma moldura por cima dele).
pub fn highlight(ui: &mut Ui, id: ComponentId) -> Result<(), String> {
    let rect = ui.absolute_rect(id).ok_or("o componente não é um controle visível")?;
    let root = ui.root_of(id);
    let mut base = ControlBase::new(rect.width, rect.height);
    base.set_rect(rect.x - 3.0, rect.y - 3.0, rect.width + 6.0, rect.height + 6.0).set_hit_test(false);
    ui.add(Highlight { base }, root);
    ui.request_repaint();
    Ok(())
}

struct Highlight {
    base: ControlBase,
}

impl Component for Highlight {
    fn class_name(&self) -> &'static str {
        "InspectorHighlight"
    }

    fn control(&self) -> Option<&ControlBase> {
        Some(&self.base)
    }

    fn control_mut(&mut self) -> Option<&mut ControlBase> {
        Some(&mut self.base)
    }

    fn timer_interval(&self) -> Option<f64> {
        Some(1.5)
    }

    fn timer(&mut self, ctx: &mut EventCtx<'_>) {
        let id = ctx.id;
        ctx.ui.poster().post(move |ui| {
            ui.free(id);
        });
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let color = aylis_core::Color::from_hex("#F43F5E").unwrap_or(aylis_core::Color::BLACK);
        canvas.fill_rect(ctx.rect, 4.0, &Brush::Solid(color.with_opacity(0.12)));
        canvas.stroke_rect(ctx.rect, 4.0, &Stroke::new(color, 2.5));
    }
}
