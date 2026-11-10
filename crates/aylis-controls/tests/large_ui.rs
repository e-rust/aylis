//! O caso grande do item 5 com limite: nas situações do dia a dia, a área repintada fica
//! pequena (a área, e não o tempo, para o limite valer em qualquer máquina e em debug). A
//! medição de tempo é o exemplo `large_ui`.

#[path = "../examples/large_ui.rs"]
mod large_ui;

#[test]
fn everyday_changes_repaint_a_small_part_of_a_large_screen() {
    let mut large = large_ui::build(2, 30);
    assert!(large.controls > 1000, "{} controles", large.controls);
    let limits = [("digitar", 0.02), ("relogio", 0.02), ("progresso", 0.02), ("mouse", 0.02), ("deslizar", 0.20), ("rolar", 0.10)];
    for (name, _, _, area) in large_ui::measure(&mut large, 4) {
        let limit = limits.iter().find(|(n, _)| *n == name).map(|(_, l)| *l).unwrap();
        assert!(area <= limit, "{name}: {:.1}% da janela repintada (limite {:.0}%)", area * 100.0, limit * 100.0);
    }
}
