//! As demonstrações da documentação: o componente aparece de verdade, no meio do form, e cada
//! classe que roda no navegador fica em exatamente uma família (um wasm).

use aylis::prelude::*;

#[test]
fn the_demo_shows_the_real_component_in_the_middle() {
    let mut ui = Ui::new();
    aylis::register_all(&mut ui);
    let form = aylis_demos::demo_form(&mut ui, "Button", true).expect("Button é visual");
    let button = ui.children(form).iter().copied().find(|&c| ui.class_name(c) == Some("Button")).unwrap();
    assert_eq!(ui.get_prop(button, "text").ok().and_then(|v| v.to_str().ok().map(str::to_string)).as_deref(), Some("Button"));
    // No meio: as sobras dos dois lados iguais.
    let (fw, b) = (ui.control(form).unwrap().width(), ui.control(button).unwrap().bounds());
    assert!((b.x - (fw - b.right())).abs() < 1.0, "centralizado: {b:?} em {fw}");
    // Um componente sem tela não tem demonstração.
    assert!(aylis_demos::demo_form(&mut ui, "Timer", true).is_none());
}

#[test]
fn every_web_class_is_in_exactly_one_family() {
    let families = aylis_tools::publish::demo_families();
    assert!(families.iter().any(|f| f.category == "Standard" && f.folder == "standard"));
    assert!(families.iter().any(|f| f.folder == "data-controls" && f.features == ["data"]));
    for (class, _) in aylis_build::CLASSES {
        let count = families.iter().filter(|f| f.classes.iter().any(|c| c == class)).count();
        assert_eq!(count, usize::from(aylis_build::demo_on_web(class)), "{class}");
    }
}

#[test]
fn the_page_query_gives_the_class_and_the_theme() {
    assert_eq!(aylis_demos::query_value("?class=Button&theme=light", "class").as_deref(), Some("Button"));
    assert_eq!(aylis_demos::query_value("?class=Button&theme=light", "theme").as_deref(), Some("light"));
    assert_eq!(aylis_demos::query_value("", "class"), None);
}
