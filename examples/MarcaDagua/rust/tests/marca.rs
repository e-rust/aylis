//! O texto da marca d'água: repetido em linhas, em xadrez, com o número da célula, e nada
//! com o texto vazio.

use marca_dagua::generated::principal::{cell_mark, watermark_text};

#[test]
fn the_text_repeats_in_lines_shifted_every_other_one() {
    let marca = watermark_text("  SIGILOSO ", 30, 8);
    let lines: Vec<&str> = marca.lines().collect();
    assert_eq!(lines.len(), 30);
    assert!(lines.iter().all(|l| l.matches("SIGILOSO").count() == 8), "{marca}");
    assert!(!lines[0].starts_with(' ') && lines[1].starts_with(' '), "o xadrez: {marca}");
}

#[test]
fn each_cell_has_its_own_mark() {
    assert_eq!(cell_mark("CÂMERA", 0), "CÂMERA 01");
    assert_eq!(cell_mark(" CÂMERA ", 35), "CÂMERA 36");
}

#[test]
fn an_empty_text_hides_the_mark() {
    assert_eq!(cell_mark("   ", 3), "");
    assert_eq!(watermark_text("   ", 10, 4), "");
}

#[test]
fn the_grid_is_almost_square_up_to_a_thousand() {
    use marca_dagua::generated::principal::grid_shape;
    assert_eq!(grid_shape(1), (1, 1));
    assert_eq!(grid_shape(36), (6, 6));
    assert_eq!(grid_shape(1000), (32, 32));
    let (columns, rows) = grid_shape(1000);
    assert!(columns * rows >= 1000 && columns * (rows - 1) < 1000);
}
