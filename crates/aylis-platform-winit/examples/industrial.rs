//! Componentes industriais e elétricos: uma tela de supervisão com tanque, bomba, válvula e
//! canos presos (o fluxo anda), instrumentos, comandos e um circuito com símbolos e fios.
//! `AYLIS_AUTO_CLOSE=1` fecha sozinho (para a captura do `AYLIS_SNAPSHOT`).
//!
//! ```text
//! cargo run -p aylis-platform-winit --example industrial --features cpu
//! ```

use aylis_controls::*;
use aylis_core::*;

fn place(ui: &mut Ui, id: ComponentId, x: f32, y: f32) -> ComponentId {
    let (w, h) = ui.control(id).map(|c| (c.width(), c.height())).unwrap_or((80.0, 80.0));
    ui.control_mut(id).unwrap().set_rect(x, y, w, h);
    id
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("Supervisão", 1000.0, 640.0));

    // Processo: tanque → bomba → válvula → silo, com canos presos.
    let tank = ui.add(TankLevel::default(), form);
    place(&mut ui, tank, 30.0, 40.0);
    let pump = ui.add(Pump::default(), form);
    place(&mut ui, pump, 230.0, 150.0);
    let valve = ui.add(Valve::default(), form);
    place(&mut ui, valve, 400.0, 60.0);
    let silo = ui.add(Silo::default(), form);
    place(&mut ui, silo, 560.0, 20.0);
    for (from, to, fp, tp) in [(tank, pump, "right", "left"), (pump, valve, "right", "left"), (valve, silo, "right", "left")] {
        let pipe = ui.add(Pipe::default(), form);
        ui.set_prop(pipe, "from", PropValue::Component(Some(from)))?;
        ui.set_prop(pipe, "to", PropValue::Component(Some(to)))?;
        ui.set_prop(pipe, "from_port", fp)?;
        ui.set_prop(pipe, "to_port", tp)?;
    }

    // Instrumentos.
    let c = ui.add(Manometer::default(), form);
    place(&mut ui, c, 700.0, 30.0);
    let thermo = ui.add(Thermometer::default(), form);
    place(&mut ui, thermo, 860.0, 20.0);
    let c = ui.add(FlowMeter::default(), form);
    place(&mut ui, c, 30.0, 240.0);
    let c = ui.add(SevenSegment::default(), form);
    place(&mut ui, c, 170.0, 270.0);
    let light = ui.add(SignalLight::default(), form);
    place(&mut ui, light, 340.0, 220.0);
    ui.set_prop(light, "lights", "red;green")?;

    // Equipamentos e comandos.
    let motor = ui.add(Motor::default(), form);
    place(&mut ui, motor, 420.0, 250.0);
    let c = ui.add(Fan::default(), form);
    place(&mut ui, c, 560.0, 240.0);
    let conveyor = ui.add(Conveyor::default(), form);
    place(&mut ui, conveyor, 680.0, 260.0);
    let stop = ui.add(EmergencyStop::default(), form);
    place(&mut ui, stop, 30.0, 400.0);
    let c = ui.add(SelectorSwitch::default(), form);
    place(&mut ui, c, 140.0, 400.0);
    let c = ui.add(PushButtons::default(), form);
    place(&mut ui, c, 270.0, 410.0);
    // A emergência para tudo.
    ui.on(stop, "on_change", move |ui, _, args| {
        let on = !matches!(args, EventArgs::Value(PropValue::Bool(true)));
        for id in [pump, motor, conveyor] {
            let _ = ui.set_prop(id, "running", on);
        }
        let _ = ui.set_prop(valve, "state", if on { "open" } else { "closed" });
    });

    // Circuito: fonte → chave → resistor → lâmpada, com fios.
    let mut parts = Vec::new();
    for (i, (symbol, label)) in [("source", "24 V"), ("switch", "S1"), ("resistor", "10 Ω"), ("lamp", "L1"), ("ground", "")].iter().enumerate() {
        let s = ui.add(ElectricalSymbol::default(), form);
        ui.set_prop(s, "symbol", *symbol)?;
        ui.set_prop(s, "label", *label)?;
        ui.set_prop(s, "state", true)?;
        place(&mut ui, s, 470.0 + i as f32 * 105.0, 420.0);
        parts.push(s);
    }
    for pair in parts.windows(2) {
        let w = ui.add(Wire::default(), form);
        ui.set_prop(w, "from", PropValue::Component(Some(pair[0])))?;
        ui.set_prop(w, "to", PropValue::Component(Some(pair[1])))?;
        ui.set_prop(w, "energized", true)?;
    }
    for (i, symbol) in ["capacitor", "inductor", "diode", "led", "transistor", "relay", "motor"].iter().enumerate() {
        let s = ui.add(ElectricalSymbol::default(), form);
        ui.set_prop(s, "symbol", *symbol)?;
        ui.set_prop(s, "state", true)?;
        ui.set_prop(s, "label", *symbol)?;
        place(&mut ui, s, 30.0 + i as f32 * 105.0, 540.0);
    }

    if std::env::var("AYLIS_AUTO_CLOSE").is_ok() {
        let timer = ui.add(Timer::new(1500), form);
        ui.on(timer, "on_timer", |ui, _, _| ui.request_close());
    }
    aylis_platform_winit::run(ui, form)
}
