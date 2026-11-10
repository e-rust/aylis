mod common;

use std::cell::RefCell;
use std::rc::Rc;

use common::{Boxy, Fancy, boxy};
use aylis_core::*;

#[test]
fn base_props_by_name() {
    let mut ui = Ui::new();
    let form = ui.add_root(boxy(100.0, 100.0));
    let b = ui.add(boxy(10.0, 10.0), form);

    ui.set_prop(b, "width", 120).unwrap();
    ui.set_prop(b, "height", 32.5).unwrap();
    ui.set_prop(b, "align", "top").unwrap();
    ui.set_prop(b, "anchors", "left, right").unwrap();
    ui.set_prop(b, "margins", 4).unwrap();
    ui.set_prop(b, "margins.left", 10).unwrap();
    ui.set_prop(b, "visible", false).unwrap();
    ui.set_prop(b, "style", "danger").unwrap();

    assert_eq!(ui.get_prop(b, "width").unwrap(), PropValue::Float(120.0));
    assert_eq!(ui.get_prop(b, "align").unwrap(), PropValue::Enum("top".into()));
    assert_eq!(ui.get_prop(b, "anchors").unwrap(), PropValue::Set(vec!["left".into(), "right".into()]));
    assert_eq!(ui.get_prop(b, "margins").unwrap(), PropValue::Bounds(Bounds::new(10.0, 4.0, 4.0, 4.0)));
    assert_eq!(ui.get_prop(b, "margins.left").unwrap(), PropValue::Float(10.0));

    let cb = ui.control(b).unwrap();
    assert_eq!(cb.align(), Align::Top);
    assert!(!cb.visible());
    assert_eq!(cb.style(), "danger");
}

#[test]
fn errors_name_the_property() {
    let mut ui = Ui::new();
    let form = ui.add_root(boxy(100.0, 100.0));

    assert_eq!(ui.set_prop(form, "nada", 1), Err(Error::UnknownProperty("nada".into())));
    assert_eq!(
        ui.set_prop(form, "width", "abc"),
        Err(Error::TypeMismatch { property: "width".into(), expected: "float", got: "string" })
    );
    let err = ui.set_prop(form, "align", "meio").unwrap_err();
    assert!(matches!(&err, Error::InvalidValue { property, .. } if property == "align"));
    assert!(err.to_string().contains("client"), "a mensagem lista os valores válidos: {err}");
}

#[test]
fn composed_props_include_every_layer() {
    let mut ui = Ui::new();
    let form = ui.add_root(boxy(100.0, 100.0));
    let f = ui.add(Fancy::default(), form);

    ui.set_prop(f, "glow", 3).unwrap(); // da Fancy
    ui.set_prop(f, "caption", "Oi").unwrap(); // da Boxy (camada base)
    ui.set_prop(f, "width", 50).unwrap(); // do ControlBase

    assert_eq!(ui.get_prop(f, "glow").unwrap(), PropValue::Float(3.0));
    assert_eq!(ui.get::<Boxy>(f).unwrap().caption, "Oi");

    let names: Vec<_> = ui.prop_infos(f).iter().map(|p| p.name).collect();
    assert_eq!(&names[..2], &["glow", "caption"]);
    assert!(names.contains(&"align") && names.contains(&"on_click"));
}

#[test]
fn name_is_a_property_too() {
    let mut ui = Ui::new();
    let form = ui.add_root(boxy(100.0, 100.0));
    ui.set_prop(form, "name", "FrmClientes").unwrap();
    assert_eq!(ui.get_prop(form, "name").unwrap(), PropValue::String("FrmClientes".into()));
    assert_eq!(ui.find("frmclientes"), Some(form));
}

#[test]
fn events_by_handler_name() {
    let mut ui = Ui::new();
    let form = ui.add_root(boxy(100.0, 100.0));
    let btn = ui.add(boxy(10.0, 10.0), form);
    ui.set_name(btn, "btnSalvar").unwrap();

    let log = Rc::new(RefCell::new(Vec::new()));
    let l = log.clone();
    ui.register_handler("btn_salvar_click", move |ui, sender, _| {
        l.borrow_mut().push(ui.name(sender).unwrap().to_string());
    });

    ui.set_prop(btn, "on_click", "btn_salvar_click").unwrap();
    assert_eq!(ui.get_prop(btn, "on_click").unwrap(), PropValue::Event(Some("btn_salvar_click".into())));
    ui.click(btn);
    assert_eq!(*log.borrow(), vec!["btnSalvar".to_string()]);

    ui.set_prop(btn, "on_click", PropValue::Null).unwrap();
    ui.click(btn);
    assert_eq!(log.borrow().len(), 1, "evento desligado");
}

#[test]
fn handler_names_must_be_identifiers() {
    let mut ui = Ui::new();
    let form = ui.add_root(boxy(100.0, 100.0));
    let btn = ui.add(boxy(10.0, 10.0), form);
    // O nome vira função no código gerado: nada que injete código.
    for bad in ["x() {} pub fn y", "a b", "1abc", "frm.salvar", "salvar;"] {
        let err = ui.set_prop(btn, "on_click", bad).unwrap_err().to_string();
        assert!(err.contains("on_click") && err.contains("handler"), "{bad}: {err}");
    }
    assert_eq!(ui.get_prop(btn, "on_click").unwrap(), PropValue::Event(None));
    ui.set_prop(btn, "on_click", "_btn_salvar_click2").unwrap();
    ui.set_prop(btn, "on_click", "").unwrap();
    assert_eq!(ui.get_prop(btn, "on_click").unwrap(), PropValue::Event(None));
}

#[test]
fn unregistered_handlers_go_to_the_event_sink() {
    let mut ui = Ui::new();
    let form = ui.add_root(boxy(100.0, 100.0));
    let btn = ui.add(boxy(10.0, 10.0), form);
    ui.set_prop(btn, "on_click", "remoto_click").unwrap();

    let seen = Rc::new(RefCell::new(None));
    let s = seen.clone();
    ui.set_event_sink(move |_, sender, event, handler, _| {
        *s.borrow_mut() = Some((sender, event.to_string(), handler.to_string()));
    });
    ui.click(btn);
    assert_eq!(*seen.borrow(), Some((btn, "on_click".into(), "remoto_click".into())));
}

#[test]
fn closure_handlers_are_not_named() {
    let mut ui = Ui::new();
    let form = ui.add_root(boxy(100.0, 100.0));
    ui.on_click(form, |_, _| {});
    assert_eq!(ui.get_prop(form, "on_click").unwrap(), PropValue::Event(None));
}

#[test]
fn messages_built_with_values_are_translated_by_their_template() {
    let mut ui = Ui::new();
    let map: std::collections::HashMap<String, String> = [
        ("Form {} adicionado", "Form {} added"),
        ("Projeto {} aberto ({})", "Project {} opened ({})"),
        ("{} min {} s", "{1} s, {0} min"),
        ("Salvo", "Saved"),
        ("Salvo: {}", "Saved: {}"),
        ("Rust", ""),
    ]
    .into_iter()
    .map(|(k, v)| (k.to_string(), v.to_string()))
    .collect();
    ui.set_translations(map);
    assert_eq!(ui.tr("Form Cadastro adicionado"), "Form Cadastro added");
    assert_eq!(ui.tr("Projeto Loja aberto (Rust)"), "Project Loja opened (Rust)");
    assert_eq!(ui.tr("3 min 12 s"), "12 s, 3 min", "a ordem pela posição");
    assert_eq!(ui.tr("Salvo"), "Saved", "a chave exata vale mais que o modelo");
    assert_eq!(ui.tr("Salvo: Salvo"), "Saved: Saved", "um valor que é texto do catálogo também traduz");
    assert_eq!(ui.tr("Outra coisa"), "Outra coisa");
}

/// Cor, opacidade e giro só repintam; largura e texto refazem o layout.
#[test]
fn paint_only_properties_do_not_redo_the_layout() {
    let mut ui = Ui::new();
    let form = ui.add_root(boxy(100.0, 100.0));
    let b = ui.add(boxy(10.0, 10.0), form);
    ui.layout();
    assert!(!ui.needs_layout());
    for (name, value) in [("fill", PropValue::from("#ff0000")), ("opacity", PropValue::from(0.5)), ("rotation", PropValue::from(10.0))] {
        ui.set_prop(b, name, value).unwrap();
        assert!(!ui.needs_layout(), "{name} refez o layout");
        assert!(ui.needs_repaint(), "{name} não repintou");
    }
    ui.set_prop(b, "width", 40).unwrap();
    assert!(ui.needs_layout());
}
