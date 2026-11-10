//! `set_timeout`: uma função que roda uma vez, depois do tempo, na ordem do vencimento.

use std::cell::RefCell;
use std::rc::Rc;

use aylis_core::*;

#[test]
fn timeouts_run_once_in_order_and_can_be_cleared() {
    let mut ui = Ui::new();
    let log = Rc::new(RefCell::new(Vec::new()));
    let (a, b, c) = (log.clone(), log.clone(), log.clone());
    ui.set_timeout(200, move |_| a.borrow_mut().push("200"));
    ui.set_timeout(100, move |ui| {
        b.borrow_mut().push("100");
        // criado dentro de outro: roda na volta seguinte, não nesta
        let d = b.clone();
        ui.set_timeout(0, move |_| d.borrow_mut().push("encadeado"));
    });
    let cancel = ui.set_timeout(150, move |_| c.borrow_mut().push("cancelado"));
    ui.clear_timeout(cancel);
    ui.set_time(0.05);
    ui.run_timers();
    assert!(log.borrow().is_empty());
    ui.set_time(0.25);
    ui.run_timers();
    assert_eq!(*log.borrow(), ["100", "200"]);
    ui.run_timers();
    assert_eq!(*log.borrow(), ["100", "200", "encadeado"]);
    ui.set_time(1.0);
    ui.run_timers();
    assert_eq!(log.borrow().len(), 3, "cada um roda uma vez só");
}
