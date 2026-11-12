//! Os temas embutidos só citam cores que eles têm: um nome desconhecido pinta transparente
//! sem erro (o relatório do Tabernáculo, C07), e a verificação do projeto avisaria em todo
//! projeto.

#[test]
fn the_builtin_themes_only_use_their_own_colors() {
    for theme in aylis_core::themes::all() {
        let unknown = theme.unknown_color_tokens();
        assert!(unknown.is_empty(), "{}: {unknown:?}", theme.name);
    }
    let mut broken = aylis_core::themes::all()[0].clone();
    broken.styles.insert("Panel".into(), aylis_core::theme::Style::new(aylis_core::theme::StyleProps { fill: Some(aylis_core::theme::ColorRef::Token("primay".into())), ..Default::default() }));
    assert_eq!(broken.unknown_color_tokens(), vec![("Panel".to_string(), "primay".to_string())]);
}
