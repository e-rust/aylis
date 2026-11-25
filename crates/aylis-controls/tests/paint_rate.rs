//! O `max_paint_rate` do form: o que muda sem a entrada do usuário (um quadro de vídeo, um
//! timer) se junta numa pintura no máximo N vezes por segundo; a entrada pinta na hora; parado,
//! nenhum relógio; desligado (0), como antes.

use aylis_core::*;

/// Um form com um texto (o que muda "sozinho", como uma célula de vídeo), já pintado uma vez
/// no tempo 0.
fn form(rate: f64) -> (Ui, ComponentId, ComponentId) {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.create("Form", None).unwrap();
    ui.set_prop(form, "max_paint_rate", PropValue::Float(rate)).unwrap();
    let label = ui.create("Label", Some(form)).unwrap();
    ui.set_prop(label, "text", PropValue::String("quadro 0".into())).unwrap();
    ui.set_time(0.0);
    ui.layout();
    let _ = ui.take_frame(form);
    (ui, form, label)
}

fn change(ui: &mut Ui, label: ComponentId, text: &str) {
    ui.set_prop(label, "text", PropValue::String(text.into())).unwrap();
}

#[test]
fn the_changes_without_input_wait_for_the_rate() {
    let (mut ui, form, label) = form(10.0);
    ui.set_time(0.01);
    change(&mut ui, label, "quadro 1");
    assert!(ui.forms_to_paint(&[form]).is_empty(), "antes da vez, a pintura espera");
    assert_eq!(ui.paint_held_until(form), Some(0.1));
    let wake = ui.next_wake().expect("o laço acorda na vez");
    assert!((wake - 0.1).abs() < 1e-9, "acorda em {wake}");
    // Mais mudanças antes da vez juntam na mesma pintura.
    ui.set_time(0.05);
    change(&mut ui, label, "quadro 2");
    assert!(ui.forms_to_paint(&[form]).is_empty());
    ui.set_time(0.1);
    assert_eq!(ui.forms_to_paint(&[form]), vec![form], "na vez, pinta");
    let _ = ui.take_frame(form);
    // Parado: nada pendente, nenhum relógio.
    ui.set_time(0.5);
    assert_eq!(ui.next_wake(), None, "parado não gasta");
    assert!(ui.forms_to_paint(&[form]).is_empty());
}

#[test]
fn the_input_paints_at_once() {
    let (mut ui, form, label) = form(10.0);
    ui.set_time(0.02);
    change(&mut ui, label, "quadro 1");
    assert!(ui.forms_to_paint(&[form]).is_empty());
    ui.pointer_move(form, PointerEvent { position: PointF::new(5.0, 5.0), ..Default::default() });
    assert_eq!(ui.forms_to_paint(&[form]), vec![form], "o mouse pinta na hora, com o vídeo junto");
    let _ = ui.take_frame(form);
    // Logo depois da entrada (o deslize da rolagem, o realce), ainda na hora.
    ui.set_time(0.03);
    change(&mut ui, label, "quadro 2");
    assert_eq!(ui.forms_to_paint(&[form]), vec![form], "o que a entrada puxa logo depois também");
    let _ = ui.take_frame(form);
    // Passado o tempo da entrada, volta a esperar a vez.
    ui.set_time(0.02 + aylis_core::INPUT_GRACE + 0.01);
    let _ = ui.take_frame(form);
    ui.set_time(ui.time() + 0.01);
    change(&mut ui, label, "quadro 3");
    assert!(ui.forms_to_paint(&[form]).is_empty(), "sem entrada recente, espera de novo");
    ui.key_down(form, KeyEvent::default());
    assert_eq!(ui.forms_to_paint(&[form]), vec![form], "a tecla pinta na hora");
}

#[test]
fn off_paints_every_change_as_before() {
    let (mut ui, form, label) = form(0.0);
    for i in 1..5 {
        ui.set_time(i as f64 * 0.001);
        change(&mut ui, label, &format!("quadro {i}"));
        assert_eq!(ui.forms_to_paint(&[form]), vec![form]);
        assert_eq!(ui.paint_held_until(form), None);
        let _ = ui.take_frame(form);
    }
    // Ligada com o app aberto, vale a partir da pintura seguinte.
    ui.set_prop(form, "max_paint_rate", PropValue::Float(20.0)).unwrap();
    let _ = ui.take_frame(form);
    ui.set_time(ui.time() + 0.001);
    change(&mut ui, label, "ligado");
    assert!(ui.forms_to_paint(&[form]).is_empty());
}
