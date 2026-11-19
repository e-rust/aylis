//! O apoio do acesso tipado gerado em Rust (`aylis_core::handles`): as datas, as horas e as
//! listas separadas por `;` saem e entram tipadas, e a tela continua gravando o texto de sempre.

use aylis_controls::{ActivityRings, ChipGroup, DateEdit, Form, TimeEdit, register};
use aylis_core::{Color, Date, PropValue, Time, Ui, handles};

#[test]
fn dates_times_and_lists_are_typed() {
    let mut ui = Ui::new();
    register(&mut ui);
    let form = ui.add_root(Form::default());
    let date = ui.add(DateEdit::default(), form);
    let time = ui.add(TimeEdit::default(), form);
    let chips = ui.add(ChipGroup::default(), form);
    let rings = ui.add(ActivityRings::default(), form);

    handles::write(&mut ui, date, "date", handles::date_value(Date::new(2026, 10, 24)));
    assert_eq!(ui.get_prop(date, "date").unwrap(), PropValue::String("2026-10-24".into()));
    assert_eq!(handles::read_date(&ui, date, "date"), Date::new(2026, 10, 24));
    handles::write(&mut ui, date, "date", handles::date_value(None));
    assert_eq!(handles::read_date(&ui, date, "date"), None);

    // A hora sem segundos sai como `HH:MM` (o que o TimeEdit e o TimePicker gravam).
    handles::write(&mut ui, time, "time", handles::time_value(Time::new(9, 5, 0)));
    assert_eq!(ui.get_prop(time, "time").unwrap(), PropValue::String("09:05".into()));
    assert_eq!(handles::read_time(&ui, time, "time"), Time::new(9, 5, 0));

    handles::write(&mut ui, chips, "items", handles::list_value(&["A", "B", "C"]));
    assert_eq!(ui.get_prop(chips, "items").unwrap(), PropValue::String("A;B;C".into()));
    assert_eq!(handles::read_list(&ui, chips, "items"), ["A", "B", "C"]);

    handles::write(&mut ui, rings, "values", handles::numbers_value(&[0.5, 1.25]));
    assert_eq!(handles::read_numbers(&ui, rings, "values"), [0.5, 1.25]);
    let colors = [Color::rgb(255, 0, 0), Color::rgba(0, 170, 0, 128)];
    handles::write(&mut ui, rings, "ring_colors", handles::colors_value(&colors));
    assert_eq!(handles::read_colors(&ui, rings, "ring_colors"), colors);
}
