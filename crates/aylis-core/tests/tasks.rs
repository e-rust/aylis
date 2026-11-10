//! Trabalho em segundo plano ligado a um dono: o resultado só chega se o dono ainda existe.

mod common;

use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, Instant};

use common::boxy;
use aylis_core::*;

/// Roda o que foi agendado até `done()` ou o prazo (o laço da janela, no teste).
fn pump(ui: &mut Ui, done: impl Fn(&Ui) -> bool) {
    let until = Instant::now() + Duration::from_secs(5);
    while !done(ui) && Instant::now() < until {
        ui.process_posted();
        std::thread::sleep(Duration::from_millis(2));
    }
    ui.process_posted();
}

#[test]
fn post_to_drops_the_job_of_a_freed_owner() {
    let mut ui = Ui::new();
    let form = ui.add_root(boxy(100.0, 100.0));
    let ran = Rc::new(RefCell::new(0));
    let poster = ui.poster();
    let (alive, closed) = (ui.add(boxy(1.0, 1.0), form), ui.add(boxy(1.0, 1.0), form));
    std::thread::spawn({
        let poster = poster.clone();
        move || {
            poster.post_to(alive, |ui| ui.set_prop(ui.roots()[0], "width", 200.0).unwrap());
            poster.post_to(closed, |_| panic!("o dono fechou: não roda"));
        }
    })
    .join()
    .unwrap();
    ui.free(closed);
    ui.process_posted();
    assert_eq!(ui.get_prop(form, "width"), Ok(PropValue::Float(200.0)));
    let _ = ran;
}

#[test]
fn background_runs_the_work_in_a_thread_and_ends_on_the_ui() {
    let mut ui = Ui::new();
    let form = ui.add_root(boxy(100.0, 100.0));
    let ui_thread = std::thread::current().id();
    ui.background(
        form,
        move || (std::thread::current().id() != ui_thread, 21 * 2),
        |ui, (other_thread, value)| {
            assert!(other_thread, "o trabalho roda noutra thread");
            ui.set_prop(ui.roots()[0], "width", value as f64).unwrap();
        },
    );
    pump(&mut ui, |ui| ui.get_prop(ui.roots()[0], "width") == Ok(PropValue::Float(42.0)));
    assert_eq!(ui.get_prop(form, "width"), Ok(PropValue::Float(42.0)));
}

#[test]
fn spawn_local_waits_without_blocking_and_dies_with_its_owner() {
    let mut ui = Ui::new();
    let form = ui.add_root(boxy(100.0, 100.0));
    let result = Rc::new(RefCell::new(None));
    // Um serviço que responde de outra thread, depois de um tempo (como o HTTP).
    let slow = || {
        tasks::from_callback(|done| {
            std::thread::spawn(move || {
                std::thread::sleep(Duration::from_millis(20));
                done("resposta".to_string());
            });
        })
    };
    let r = result.clone();
    ui.spawn_local(form, slow(), move |_, text| *r.borrow_mut() = Some(text));
    assert_eq!(ui.running_tasks(), 1, "esperando, sem travar");
    pump(&mut ui, |_| result.borrow().is_some());
    assert_eq!(result.borrow().as_deref(), Some("resposta"));
    assert_eq!(ui.running_tasks(), 0);

    // Com o dono liberado no meio da espera, a tarefa morre e o fim não roda.
    let closed = ui.add(boxy(1.0, 1.0), form);
    ui.spawn_local(closed, slow(), |_, _| panic!("o dono fechou: não roda"));
    ui.free(closed);
    assert_eq!(ui.running_tasks(), 0);
    std::thread::sleep(Duration::from_millis(50));
    ui.process_posted();
}
