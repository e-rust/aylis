//! MapView: o OpenStreetMap com marcadores. Arraste, use a roda (ou a pinça) e dê duplo
//! clique; clicar mostra a coordenada.
//!
//! ```text
//! cargo run -p aylis-platform-winit --example map --features cpu
//! ```

use aylis_controls::*;
use aylis_core::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("aylis — mapa", 900.0, 640.0));
    let status = ui.add(Label::new("Clique no mapa"), form);
    ui.control_mut(status).unwrap().set_align(Align::Bottom).set_height(30.0).set_margins([10.0, 0.0, 0.0, 0.0]);
    let mut map = MapView::default();
    map.latitude = -23.5614;
    map.longitude = -46.6559;
    map.zoom = 14.0;
    map.markers = r#"[
        {"latitude": -23.5614, "longitude": -46.6559, "label": "MASP"},
        {"latitude": -23.5874, "longitude": -46.6576, "label": "Parque Ibirapuera"},
        {"latitude": -23.5432, "longitude": -46.6292, "label": "Catedral da Sé"}
    ]"#
    .into();
    let map = ui.add(map, form);
    ui.control_mut(map).unwrap().set_align(Align::Client);
    ui.on(map, "on_map_click", move |ui, _, args| {
        if let Some(p) = <MapPoint as EventValue>::from_args(args) {
            let _ = ui.set_prop(status, "text", format!("Coordenada: {:.6}, {:.6}", p.latitude, p.longitude));
        }
    });
    aylis_platform_winit::run(ui, form)
}
