//! Os tokens novos do tema: o ícone do campo (`icon_size`, `icon_color`, `icon_gap`) e a barra
//! de rolagem (o estilo `ScrollBar` e o `scroll_bar` de um estilo), lidos, resolvidos com os
//! estados e gravados de volta.

use aylis_core::{Color, ControlState, Theme};

const THEME: &str = r##"{ "styles": {
    "ScrollBar": { "width": 6, "fill": "#00000010", "thumb": "#80808060", "radius": 3, "hover": { "thumb": "#808080C0" } },
    "ScrollBox": { "scroll_bar": { "width": 4, "hover": { "thumb": "#FF0000" } } },
    "Edit": { "icon_size": 18, "icon_color": "#123456", "icon_gap": 10, "focused": { "icon_color": "#00FF00" } }
} }"##;

#[test]
fn the_field_icon_tokens_follow_the_states() {
    let theme = aylis_json::load_theme(THEME).unwrap();
    let rest = theme.resolve(&["Edit"], "", ControlState::default());
    assert_eq!((rest.icon_size, rest.icon_gap), (Some(18.0), Some(10.0)));
    assert_eq!(rest.icon_color, Some(Color::rgb(0x12, 0x34, 0x56)));
    let focused = theme.resolve(&["Edit"], "", ControlState { focused: true, ..Default::default() });
    assert_eq!(focused.icon_color, Some(Color::rgb(0, 255, 0)), "o estado troca a cor do ícone");
    let other = theme.resolve(&["Label"], "", ControlState::default());
    assert_eq!((other.icon_size, other.icon_color, other.icon_gap), (None, None, None), "sem o token: o do controle");
}

#[test]
fn the_scroll_bar_style_is_shared_and_each_control_can_change_its_own() {
    let theme = aylis_json::load_theme(THEME).unwrap();
    // Um controle sem `scroll_bar` próprio: o `ScrollBar` do tema.
    let shared = theme.scroll_bar(None, false);
    assert_eq!(shared.width, Some(6.0));
    assert_eq!(shared.track, Some(Color::rgba(0, 0, 0, 0x10)));
    assert_eq!(shared.thumb, Some(Color::rgba(0x80, 0x80, 0x80, 0x60)));
    assert_eq!(shared.radius, Some(3.0));
    assert_eq!(theme.scroll_bar(None, true).thumb, Some(Color::rgba(0x80, 0x80, 0x80, 0xC0)), "o polegar com o mouse");
    // O ScrollBox muda só a barra dele: a largura e o polegar com o mouse; o resto vem do tema.
    let own = theme.resolve(&["ScrollBox"], "", ControlState::default()).scroll_bar;
    let bar = theme.scroll_bar(own.as_deref(), false);
    assert_eq!(bar.width, Some(4.0));
    assert_eq!(bar.thumb, shared.thumb);
    assert_eq!(theme.scroll_bar(own.as_deref(), true).thumb, Some(Color::rgb(255, 0, 0)));
    // Sem nada no tema: tudo `None` (cada controle fica com a barra de antes).
    assert_eq!(Theme::new("vazio").scroll_bar(None, true), Default::default());
}

#[test]
fn the_new_tokens_are_saved_back() {
    let theme = aylis_json::load_theme(THEME).unwrap();
    let back = aylis_json::load_theme(&aylis_json::save_theme(&theme)).unwrap();
    for state in [ControlState::default(), ControlState { focused: true, ..Default::default() }] {
        assert_eq!(back.resolve(&["Edit"], "", state), theme.resolve(&["Edit"], "", state));
        assert_eq!(back.resolve(&["ScrollBox"], "", state), theme.resolve(&["ScrollBox"], "", state));
    }
    assert_eq!(back.scroll_bar(None, true), theme.scroll_bar(None, true));
    assert!(aylis_json::load_theme(r#"{ "styles": { "Edit": { "icon_size": "grande" } } }"#).is_err(), "o tamanho é número");
}

#[test]
fn the_menu_sizes_are_read_and_saved_back() {
    let json = r#"{ "styles": { "PopupMenu": { "width": 260, "item_height": 28, "separator_height": 7, "fill": "surface" } } }"#;
    let theme = aylis_json::load_theme(json).unwrap();
    let m = theme.menu_metrics();
    assert_eq!((m.width, m.item_height, m.separator_height), (Some(260.0), Some(28.0), Some(7.0)));
    let back = aylis_json::load_theme(&aylis_json::save_theme(&theme)).unwrap();
    assert_eq!(back.menu_metrics(), m);
    assert_eq!(Theme::light().menu_metrics(), aylis_core::MenuMetrics::default(), "sem os tokens: os de cada menu");
    assert!(aylis_json::load_theme(r#"{ "styles": { "PopupMenu": { "item_height": "alto" } } }"#).is_err(), "a altura é número");
}
