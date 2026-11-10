//! Repintar só o que mudou: a área de um controle, a janela inteira quando não se sabe onde.

mod common;

use common::{RecordingCanvas, boxy};
use aylis_core::*;

fn setup() -> (Ui, ComponentId, ComponentId) {
    let mut ui = Ui::new();
    let form = ui.add_root(boxy(800.0, 600.0));
    let child = ui.add(boxy(100.0, 50.0), form);
    ui.control_mut(child).unwrap().set_rect(200.0, 300.0, 100.0, 50.0);
    ui.show(form).unwrap();
    ui.layout();
    // O primeiro quadro é sempre inteiro, e é ele que mede onde cada controle desenha.
    assert_eq!(ui.take_damage(form), None);
    ui.paint(form, &mut RecordingCanvas::default());
    (ui, form, child)
}

#[test]
fn a_changed_control_repaints_only_its_area() {
    let (mut ui, form, child) = setup();
    ui.damage(child);
    assert!(ui.needs_repaint());
    let area = ui.take_damage(form).expect("só a área do controle");
    assert!(area.x <= 200.0 && area.y <= 300.0 && area.right() >= 300.0 && area.bottom() >= 350.0, "{area:?}");
    assert!(area.width < 200.0 && area.height < 150.0, "com uma folga pequena: {area:?}");
}

#[test]
fn a_control_never_painted_repaints_its_rectangle() {
    let (mut ui, form, child) = setup();
    // Um controle novo, fora da vista até agora (nunca pintado): não há desenho velho para
    // apagar, então a área é o retângulo dele.
    let other = ui.add(boxy(40.0, 20.0), form);
    ui.control_mut(other).unwrap().set_rect(10.0, 10.0, 40.0, 20.0);
    let _ = ui.take_damage(form);
    ui.damage(other);
    let area = ui.take_damage(form).expect("só o retângulo dele");
    assert!(area.width < 60.0 && area.height < 40.0, "{area:?}");
    let _ = child;
}

#[test]
fn the_area_covers_what_the_control_draws_outside_itself() {
    let (mut ui, form, child) = setup();
    // Um desenho que passa 40 da borda de baixo (o bulbo de um termômetro, uma lista maior
    // que a caixa): o núcleo mede na pintura, sem o controle declarar.
    ui.on_paint(child, |canvas, ctx, base| {
        base(canvas, ctx);
        canvas.fill_rect(RectF::new(0.0, ctx.rect.height, ctx.rect.width, 40.0), 0.0, &Brush::Solid(Color::BLACK));
    });
    ui.request_repaint();
    let _ = ui.take_damage(form);
    ui.paint(form, &mut RecordingCanvas::default());
    ui.damage(child);
    let area = ui.take_damage(form).expect("só a área do controle");
    assert!(area.bottom() >= 390.0, "inclui o que passa da borda: {area:?}");
}

#[test]
fn drawing_beyond_the_area_repaints_everything_next() {
    let (mut ui, form, child) = setup();
    // Desta vez o controle desenha bem mais longe que antes (a área sai do que ele desenhou
    // na última pintura): o que passou da área ficou recortado, e o próximo quadro é inteiro.
    ui.on_paint(child, |canvas, ctx, base| {
        base(canvas, ctx);
        canvas.fill_rect(RectF::new(0.0, ctx.rect.height, ctx.rect.width, 100.0), 0.0, &Brush::Solid(Color::BLACK));
    });
    let _ = ui.take_damage(form);
    ui.damage(child);
    let area = ui.take_damage(form).expect("só a área do controle");
    ui.paint_region(form, &mut RecordingCanvas::default(), Some(area));
    assert!(ui.needs_repaint());
    assert_eq!(ui.take_damage(form), None);
}

#[test]
fn not_knowing_where_repaints_the_whole_window() {
    let (mut ui, form, child) = setup();
    ui.damage(child);
    ui.request_repaint();
    assert_eq!(ui.take_damage(form), None, "pedido sem dizer onde");
    // Girado: a área não é um retângulo do layout.
    ui.control_mut(child).unwrap().set_rotation(15.0);
    ui.damage(child);
    assert_eq!(ui.take_damage(form), None);
}

#[test]
fn a_translated_control_repaints_where_it_was_and_where_it_is() {
    let (mut ui, form, child) = setup();
    let grandchild = ui.add(boxy(20.0, 20.0), child);
    ui.control_mut(grandchild).unwrap().set_rect(10.0, 10.0, 20.0, 20.0);
    ui.request_repaint();
    let _ = ui.take_damage(form);
    ui.paint(form, &mut RecordingCanvas::default());
    // Deslocado (uma animação): o lugar novo e o antigo, sem repintar a janela inteira.
    ui.control_mut(child).unwrap().set_translate(PointF::new(40.0, 0.0));
    ui.damage(child);
    let area = ui.take_damage(form).expect("só os dois lugares");
    assert!(area.x <= 200.0 && area.right() >= 340.0, "{area:?}");
    assert!(area.width < 200.0, "{area:?}");
    ui.paint_region(form, &mut RecordingCanvas::default(), Some(area));
    // De volta: o lugar onde ele e o filho estavam deslocados também.
    ui.control_mut(child).unwrap().set_translate(PointF::ZERO);
    ui.damage(child);
    let area = ui.take_damage(form).expect("só os dois lugares");
    assert!(area.x <= 200.0 && area.right() >= 340.0, "{area:?}");
}

#[test]
fn a_form_without_a_window_marks_the_whole_window() {
    let mut ui = Ui::new();
    let form = ui.add_root(boxy(800.0, 600.0));
    let child = ui.add(boxy(100.0, 50.0), form);
    ui.layout();
    let _ = ui.take_damage(form);
    // O form do designer não é uma janela: quem o mostra repinta inteiro.
    ui.damage(child);
    assert_eq!(ui.take_damage(form), None);
}
