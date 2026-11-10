//! O zoom do app e o de cada form: o do form multiplica o do app; mudar qualquer um avisa a
//! plataforma (que refaz o tamanho dos forms e repinta).

use aylis_controls::Form;
use aylis_core::*;

#[test]
fn form_zoom_multiplies_the_app_zoom() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let a = ui.add_root(Form::new("a", 100.0, 100.0));
    let b = ui.add_root(Form::new("b", 100.0, 100.0));
    assert_eq!(ui.form_zoom(a), 1.0);
    assert!(!ui.take_zoom_changed());

    ui.set_zoom(1.5);
    assert!(ui.take_zoom_changed());
    assert!(!ui.take_zoom_changed(), "avisa uma vez");
    ui.set_prop(b, "zoom", 2.0).unwrap();
    assert!(ui.take_zoom_changed(), "o zoom do form também avisa");
    assert_eq!(ui.form_zoom(a), 1.5);
    assert_eq!(ui.form_zoom(b), 3.0);

    // Limites (de 0,25 a 5) e valores inválidos.
    ui.set_zoom(100.0);
    assert_eq!(ui.zoom(), 5.0);
    assert_eq!(ui.form_zoom(b), 5.0, "o total também tem limite");
    ui.set_zoom(f32::NAN);
    assert_eq!(ui.zoom(), 1.0);
    ui.set_zoom(0.0);
    assert_eq!(ui.zoom(), 0.25);
}
