//! DBChart: o `Chart` com os dados de um DataSource (como o `TDBChart`).

use std::any::Any;

use aylis_controls::{Chart, Series};
use aylis_core::{Canvas, Component, ComponentId, EventCtx, PaintCtx, Ui, properties};
use aylis_data::{DataEvent, DataSource, view};

use crate::{data_message, link_of};

/// Gráfico ligado a um DataSource: um ponto por registro, o rótulo de `label_field` e uma
/// série por campo de `value_fields` (`Vendas;Custos`). Acompanha o dataset (abrir,
/// filtrar, gravar). O tipo, as escalas e o resto são os do [`Chart`].
#[derive(Default)]
pub struct DBChart {
    pub base: Chart,
    pub data_source: Option<ComponentId>,
    pub label_field: String,
    pub value_fields: String,
}

impl DBChart {
    pub fn new(data_source: ComponentId, label_field: &str, value_fields: &[&str]) -> Self {
        Self { data_source: Some(data_source), label_field: label_field.to_string(), value_fields: value_fields.join(";"), ..Default::default() }
    }

    /// Lê os registros do dataset para o gráfico.
    fn load(&mut self, ui: &Ui) {
        let Some(ds) = self.data_source.and_then(|s| ui.get::<DataSource>(s)).and_then(|s| s.data_set) else { return };
        let v = view(ui, ds);
        let fields: Vec<String> = self.value_fields.split(';').map(|f| f.trim().to_string()).filter(|f| !f.is_empty()).collect();
        let mut labels = Vec::with_capacity(v.record_count());
        let mut series: Vec<Series> = fields.iter().map(|f| Series::new(f.clone(), Vec::new())).collect();
        // A cor, o tipo e o resto de cada série continuam os que estavam (pelo nome).
        for s in series.iter_mut() {
            if let Some(old) = self.base.data().iter().find(|o| o.name == s.name) {
                *s = Series { values: Vec::new(), x: Vec::new(), radius: Vec::new(), ..old.clone() };
            }
        }
        for i in 0..v.record_count() {
            labels.push(if self.label_field.is_empty() { (i + 1).to_string() } else { v.value_at(i, &self.label_field).map(|x| x.to_string()).unwrap_or_default() });
            for (s, f) in series.iter_mut().zip(&fields) {
                s.values.push(v.value_at(i, f).ok().and_then(|x| x.as_f64()).unwrap_or(0.0));
            }
        }
        self.base.set_data(labels, series);
    }
}

impl Component for DBChart {
    fn class_name(&self) -> &'static str {
        "DBChart"
    }

    fn inner(&self) -> Option<&dyn Component> {
        Some(&self.base)
    }

    fn inner_mut(&mut self) -> Option<&mut dyn Component> {
        Some(&mut self.base)
    }

    properties! {
        data_source: Option<ComponentId> => "Data",
        label_field: String => "Data",
        value_fields: String => "Data",
    }

    fn notification(&mut self, freed: ComponentId) {
        if self.data_source == Some(freed) {
            self.data_source = None;
        }
        self.base.notification(freed);
    }

    fn property_changed(&mut self, ctx: &mut EventCtx<'_>, name: &str) {
        if matches!(name, "data_source" | "label_field" | "value_fields") {
            self.load(ctx.ui);
            ctx.repaint();
        }
        self.base.property_changed(ctx, name);
    }

    fn message(&mut self, ctx: &mut EventCtx<'_>, message: &dyn Any) {
        if let Some(m) = data_message(message) {
            if link_of(self.data_source, &self.label_field).concerns(ctx.ui, m) && m.event != DataEvent::UpdateData {
                self.load(ctx.ui);
                ctx.repaint();
            }
            return;
        }
        self.base.message(ctx, message);
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        self.base.paint(canvas, ctx);
    }
}
