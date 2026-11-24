//! A `CommandPalette` com `id`, `category`, `description`, a busca sem acento, os recentes, o
//! `on_query_change` com espera e o `open` por código.

use std::cell::RefCell;
use std::rc::Rc;

use aylis_controls::*;
use aylis_core::*;

const ITEMS: &str = r#"[
    {"id": "open", "name": "Abrir arquivo", "category": "Arquivo", "shortcut": "Ctrl+O"},
    {"id": "save", "name": "Salvar", "category": "Arquivo", "description": "Grava o documento"},
    {"name": "Configurações", "category": "Editor", "description": "Preferências e opções"},
    {"id": "zoom", "name": "Aumentar", "category": "Exibir"}
]"#;

fn setup() -> (Ui, ComponentId, ComponentId, Rc<RefCell<Vec<String>>>) {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("Teste", 800.0, 600.0));
    ui.show(form).unwrap();
    let palette = ui.create("CommandPalette", Some(form)).unwrap();
    ui.set_prop(palette, "items", ITEMS).unwrap();
    let log = Rc::new(RefCell::new(Vec::new()));
    for event in ["on_select", "on_query_change", "on_recent_change"] {
        let log = log.clone();
        ui.on(palette, event, move |_, _, args| {
            if let EventArgs::Text(t) = args {
                log.borrow_mut().push(format!("{event} {t}"));
            }
        });
    }
    ui.set_focus(palette);
    (ui, form, palette, log)
}

fn palette(ui: &Ui, id: ComponentId) -> &CommandPalette {
    ui.get::<CommandPalette>(id).unwrap()
}

fn key(ui: &mut Ui, root: ComponentId, key: Key) {
    let event = KeyEvent { key, ..Default::default() };
    ui.key_down(root, event);
    ui.key_up(root, event);
}

#[test]
fn the_search_ignores_accents_and_also_looks_at_description_and_category() {
    let (mut ui, form, id, _) = setup();
    ui.text_input(form, "configuracoes");
    assert_eq!(palette(&ui, id).shown_ids(), ["Configurações"], "sem acento; sem id, o id é o nome");
    ui.set_prop(id, "query", "documento").unwrap();
    assert_eq!(palette(&ui, id).shown_ids(), ["save"], "pela descrição");
    ui.set_prop(id, "query", "EXIBIR").unwrap();
    assert_eq!(palette(&ui, id).shown_ids(), ["zoom"], "pela categoria");
    // Sem filtro (a busca é do app): a lista como veio.
    ui.set_prop(id, "filter", false).unwrap();
    assert_eq!(palette(&ui, id).shown_ids().len(), 4);
}

#[test]
fn enter_selects_with_id_and_name_and_the_choice_becomes_recent() {
    let (mut ui, form, id, log) = setup();
    ui.text_input(form, "salv");
    key(&mut ui, form, Key::Enter);
    assert!(log.borrow().iter().any(|l| l == r#"on_select {"id":"save","name":"Salvar"}"#), "{:?}", log.borrow());
    assert!(log.borrow().iter().any(|l| l == "on_recent_change save"));
    assert_eq!(palette(&ui, id).recent_ids(), ["save"]);
    // Com a busca vazia, o recente vem primeiro (e não repete no grupo dele).
    ui.set_prop(id, "query", "").unwrap();
    let shown = palette(&ui, id).shown_ids();
    assert_eq!(shown[0], "save");
    assert_eq!(shown.iter().filter(|s| *s == "save").count(), 1);
    // O app devolve os recentes gravados; o `max_recent` limita.
    ui.set_prop(id, "recent", "zoom;open;save").unwrap();
    ui.set_prop(id, "max_recent", 2).unwrap();
    assert_eq!(&palette(&ui, id).shown_ids()[..2], ["zoom", "open"]);
    ui.set_prop(id, "max_recent", 0).unwrap();
    assert_eq!(palette(&ui, id).shown_ids()[0], "open", "sem recentes: a ordem do items");
}

#[test]
fn query_change_waits_for_the_typing_to_stop_and_cancels_the_previous_one() {
    let (mut ui, form, _, log) = setup();
    ui.set_time(10.0);
    ui.text_input(form, "s");
    ui.set_time(10.1);
    ui.run_timers();
    ui.text_input(form, "a");
    ui.set_time(10.25);
    ui.run_timers();
    assert!(!log.borrow().iter().any(|l| l.starts_with("on_query_change")), "ainda digitando: nenhum aviso");
    ui.set_time(10.35);
    ui.run_timers();
    let changes: Vec<String> = log.borrow().iter().filter(|l| l.starts_with("on_query_change")).cloned().collect();
    assert_eq!(changes, ["on_query_change sa"], "um aviso só, com o texto final");
}

#[test]
fn open_shows_focuses_and_closes_after_the_choice() {
    let (mut ui, form, id, log) = setup();
    ui.control_mut(id).unwrap().set_visible(false);
    ui.set_prop(id, "query", "abc").unwrap();
    assert!(CommandPalette::open(&mut ui, id));
    assert!(ui.control(id).unwrap().visible());
    assert_eq!(palette(&ui, id).query, "", "abre com a busca limpa");
    assert_eq!(ui.focused(form), Some(id));
    key(&mut ui, form, Key::Enter);
    assert!(log.borrow().iter().any(|l| l.starts_with("on_select")));
    assert!(!ui.control(id).unwrap().visible(), "aberta pelo open, some ao escolher");
    // O Esc com a busca vazia fecha; pelo nome também abre.
    ui.invoke(id, "open", &[]).unwrap();
    assert!(ui.control(id).unwrap().visible());
    key(&mut ui, form, Key::Escape);
    assert!(!ui.control(id).unwrap().visible());
}
