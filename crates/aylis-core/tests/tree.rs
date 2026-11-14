mod common;

use common::{Boxy, Fancy, Link, boxy};
use aylis_core::*;

#[test]
fn free_removes_subtree_and_invalidates_ids() {
    let mut ui = Ui::new();
    let form = ui.add_root(boxy(100.0, 100.0));
    let panel = ui.add(boxy(10.0, 10.0), form);
    let inner = ui.add(boxy(10.0, 10.0), panel);

    assert!(ui.free(panel));
    assert!(!ui.contains(panel));
    assert!(!ui.contains(inner), "filhos são liberados junto");
    assert!(ui.children(form).is_empty());
    assert!(ui.get::<Boxy>(inner).is_none());
    assert!(!ui.free(panel), "liberar de novo não faz nada");

    // o slot é reaproveitado, mas o id antigo continua inválido
    let reused = ui.add(boxy(1.0, 1.0), form);
    assert_ne!(reused, panel);
    assert!(!ui.contains(panel));
}

#[test]
fn set_parent_moves_and_rejects_cycles() {
    let mut ui = Ui::new();
    let form = ui.add_root(boxy(100.0, 100.0));
    let a = ui.add(boxy(10.0, 10.0), form);
    let b = ui.add(boxy(10.0, 10.0), a);

    assert_eq!(ui.set_parent(a, Some(b)), Err(Error::Cycle));
    assert_eq!(ui.set_parent(a, Some(a)), Err(Error::Cycle));

    ui.set_parent(b, Some(form)).unwrap();
    assert_eq!(ui.parent(b), Some(form));
    assert_eq!(ui.children(form), &[a, b]);
    assert!(ui.children(a).is_empty());

    ui.set_parent(b, None).unwrap();
    assert!(ui.roots().contains(&b));
}

#[test]
fn names_are_unique_per_form_and_case_insensitive() {
    let mut ui = Ui::new();
    let form1 = ui.add_root(boxy(100.0, 100.0));
    let form2 = ui.add_root(boxy(100.0, 100.0));
    let a = ui.add(boxy(10.0, 10.0), form1);
    let b = ui.add(boxy(10.0, 10.0), form1);
    let c = ui.add(boxy(10.0, 10.0), form2);

    ui.set_name(a, "btnSalvar").unwrap();
    assert_eq!(ui.set_name(b, "BTNSALVAR"), Err(Error::DuplicateName("BTNSALVAR".into())));
    ui.set_name(c, "btnSalvar").unwrap(); // outro form, pode

    assert_eq!(ui.find_in(form1, "btnsalvar"), Some(a));
    assert_eq!(ui.find_in(form2, "btnSalvar"), Some(c));
    assert_eq!(ui.set_name(b, "1abc"), Err(Error::InvalidName("1abc".into())));

    // mover para um form que já tem o nome é recusado
    assert_eq!(ui.set_parent(c, Some(form1)), Err(Error::DuplicateName("btnSalvar".into())));
}

#[test]
fn create_by_class_name() {
    let mut ui = Ui::new();
    ui.register_class::<Boxy>("Standard");
    ui.register_class::<Fancy>("Meus");

    let form = ui.create("Boxy", None).unwrap();
    let fancy = ui.create("fancy", Some(form)).unwrap();
    assert_eq!(ui.class_name(fancy), Some("Fancy"));
    assert_eq!(ui.create("Nada", None), Err(Error::UnknownClass("Nada".into())));

    let names: Vec<_> = ui.registry().iter().map(|c| (c.name, c.category)).collect();
    assert_eq!(names, vec![("Boxy", "Standard"), ("Fancy", "Meus")]);
}

#[test]
fn typed_access_reaches_composed_layers() {
    let mut ui = Ui::new();
    let form = ui.add_root(boxy(100.0, 100.0));
    let fancy = ui.add(Fancy::default(), form);

    assert!(ui.get::<Fancy>(fancy).is_some());
    ui.get_mut::<Boxy>(fancy).unwrap().caption = "OK".into();
    assert_eq!(ui.get::<Fancy>(fancy).unwrap().base.caption, "OK");
    assert!(ui.control(fancy).is_some(), "o ControlBase vem da camada base");
}

#[test]
fn notification_clears_references_to_freed_components() {
    let mut ui = Ui::new();
    let module = ui.add_root(Link::default());
    let target = ui.add(boxy(1.0, 1.0), module);
    ui.set_name(target, "alvo").unwrap();
    let link = ui.add(Link::default(), module);

    ui.set_prop(link, "target", "alvo").unwrap();
    assert_eq!(ui.get::<Link>(link).unwrap().target, Some(target));

    ui.free(target);
    assert_eq!(ui.get::<Link>(link).unwrap().target, None);
}

#[test]
fn click_handler_can_change_and_free_the_sender() {
    let mut ui = Ui::new();
    let form = ui.add_root(boxy(100.0, 100.0));
    let btn = ui.add(boxy(10.0, 10.0), form);

    ui.on_click(btn, |ui, sender| {
        // o sender é o próprio botão
        ui.get_mut::<Boxy>(sender).unwrap().caption = "clicado".into();
        ui.add(boxy(5.0, 5.0), ui.parent(sender).unwrap());
    });
    assert!(ui.click(btn));
    assert_eq!(ui.get::<Boxy>(btn).unwrap().caption, "clicado");
    assert_eq!(ui.children(form).len(), 2);

    ui.on_click(btn, |ui, sender| {
        ui.free(sender);
    });
    ui.click(btn);
    assert!(!ui.contains(btn));
}

#[test]
fn creating_many_components_at_runtime() {
    let mut ui = Ui::new();
    let form = ui.add_root(boxy(800.0, 10_000.0));
    for i in 0..1000 {
        let b = ui.add(boxy(0.0, 8.0), form);
        ui.control_mut(b).unwrap().set_align(Align::Top);
        ui.set_name(b, &format!("item{i}")).unwrap();
    }
    ui.layout();
    let last = ui.find("item999").unwrap();
    assert_eq!(ui.bounds(last).unwrap().y, 999.0 * 8.0);
}

#[test]
fn the_tree_has_a_maximum_depth() {
    let mut ui = Ui::new();
    let mut parent = ui.add_root(boxy(10.0, 10.0));
    for _ in 1..MAX_TREE_DEPTH {
        parent = ui.add_boxed(Box::new(boxy(1.0, 1.0)), Some(parent)).unwrap();
    }
    // Uma cadeia criada em laço (pelo protocolo, por exemplo) estouraria a pilha no layout.
    let err = ui.add_boxed(Box::new(boxy(1.0, 1.0)), Some(parent)).unwrap_err();
    assert!(err.to_string().contains("profundidade"), "{err}");
    ui.layout();
    // Mover uma subárvore para baixo de um nível fundo também confere a altura dela.
    let other = ui.add_root(boxy(10.0, 10.0));
    let child = ui.add(boxy(1.0, 1.0), other);
    ui.add(boxy(1.0, 1.0), child);
    let deep = ui.parent(parent).unwrap();
    assert!(ui.set_parent(child, Some(deep)).is_err());
    assert!(ui.set_parent(child, Some(ui.parent(ui.parent(deep).unwrap()).unwrap())).is_ok());
}

#[test]
fn an_event_that_requeues_itself_does_not_freeze_the_ui() {
    use std::cell::RefCell;
    use std::rc::Rc;
    let mut ui = Ui::new();
    let form = ui.add_root(boxy(100.0, 100.0));
    let btn = ui.add(boxy(10.0, 10.0), form);
    ui.set_name(btn, "btnLaco").unwrap();
    let count = Rc::new(RefCell::new(0usize));
    let c = count.clone();
    ui.register_handler("again", move |ui, sender, _| {
        *c.borrow_mut() += 1;
        ui.queue_event(sender, "on_click", EventArgs::None);
    });
    let warnings = Rc::new(RefCell::new(Vec::<String>::new()));
    let w = warnings.clone();
    ui.add_observer(move |_, activity| {
        if let aylis_core::activity::Activity::Error { message } = activity {
            w.borrow_mut().push(message.to_string());
        }
    });
    ui.set_prop(btn, "on_click", "again").unwrap();
    ui.queue_event(btn, "on_click", EventArgs::None);
    ui.process_events();
    // Volta para o laço da janela depois de uma rodada; o resto continua na próxima.
    assert_eq!(*count.borrow(), EVENTS_PER_ROUND);
    assert!(ui.has_pending_events());
    ui.process_events();
    assert_eq!(*count.borrow(), 2 * EVENTS_PER_ROUND);
    let warnings = warnings.borrow();
    assert_eq!(warnings.len(), 1, "um aviso só: {warnings:?}");
    assert!(warnings[0].contains("possível laço") && warnings[0].contains("btnLaco.on_click"), "{}", warnings[0]);
}

#[test]
fn handlers_are_reentrant() {
    use std::cell::RefCell;
    use std::rc::Rc;
    let mut ui = Ui::new();
    let form = ui.add_root(boxy(100.0, 100.0));
    let all = ui.add(boxy(10.0, 10.0), form);
    ui.set_name(all, "chkTodos").unwrap();
    let items: Vec<ComponentId> = (0..3).map(|_| ui.add(boxy(10.0, 10.0), form)).collect();
    let seen = Rc::new(RefCell::new(Vec::new()));
    let s = seen.clone();
    // Um handler só para todos: o "todos" dispara o mesmo handler nos outros, na hora.
    ui.register_handler("chk_click", move |ui, sender, _| {
        s.borrow_mut().push(sender);
        if ui.name(sender) == Some("chkTodos") {
            for i in ui.children(ui.parent(sender).unwrap()).to_vec() {
                if i != sender {
                    ui.fire(i, "on_click", &EventArgs::None);
                }
            }
        }
    });
    for &c in std::iter::once(&all).chain(&items) {
        ui.set_prop(c, "on_click", "chk_click").unwrap();
    }
    ui.fire(all, "on_click", &EventArgs::None);
    assert_eq!(*seen.borrow(), [vec![all], items.clone()].concat());

    // Um handler que dispara a si mesmo sem fim para no limite, com aviso, sem estourar a pilha.
    let errors = Rc::new(RefCell::new(Vec::<String>::new()));
    let e = errors.clone();
    ui.add_observer(move |_, activity| {
        if let aylis_core::activity::Activity::Error { message } = activity {
            e.borrow_mut().push(message.to_string());
        }
    });
    let depth = Rc::new(std::cell::Cell::new(0));
    let d = depth.clone();
    ui.on(items[0], "on_change", move |ui, sender, _| {
        d.set(d.get() + 1);
        ui.fire(sender, "on_change", &EventArgs::None);
    });
    ui.fire(items[0], "on_change", &EventArgs::None);
    assert_eq!(depth.get(), MAX_EVENT_NESTING);
    assert_eq!(errors.borrow().len(), 1);
    assert!(errors.borrow()[0].contains("recursivo demais"), "{:?}", errors.borrow());
}

#[test]
fn the_project_sets_the_limits_and_passing_them_is_an_error() {
    let mut ui = Ui::new();
    ui.set_limits(10, 8);
    assert_eq!(ui.limits(), (10, 8));
    let mut parent = ui.add_root(boxy(10.0, 10.0));
    for _ in 1..10 {
        parent = ui.add_boxed(Box::new(boxy(1.0, 1.0)), Some(parent)).unwrap();
    }
    assert!(ui.add_boxed(Box::new(boxy(1.0, 1.0)), Some(parent)).unwrap_err().to_string().contains("10 níveis"));

    // A recursão vira uma exceção (a caixa de erro), uma vez só.
    let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::<String>::new()));
    let s = seen.clone();
    ui.on_exception(move |_, e| {
        s.borrow_mut().push(e.message.clone());
        true
    });
    let depth = std::rc::Rc::new(std::cell::Cell::new(0));
    let d = depth.clone();
    ui.on(parent, "on_change", move |ui, sender, _| {
        d.set(d.get() + 1);
        ui.fire(sender, "on_change", &EventArgs::None);
    });
    ui.fire(parent, "on_change", &EventArgs::None);
    assert_eq!(depth.get(), 8);
    assert_eq!(seen.borrow().len(), 1, "{:?}", seen.borrow());
    assert!(seen.borrow()[0].contains("8 níveis"));
}
