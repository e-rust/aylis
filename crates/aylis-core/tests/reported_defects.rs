//! Os defeitos do núcleo vistos em apps reais: o layout refeito sem mudança, a âncora do
//! controle que nasce escondido, a dona do modal e a fonte do projeto ao trocar de tema.

mod common;

use std::cell::Cell;
use std::rc::Rc;

use aylis_core::*;
use common::boxy;

/// Um contêiner que conta quantas vezes o layout arrumou os filhos dele.
struct Counted {
    base: common::Boxy,
    arranged: Rc<Cell<u32>>,
}

impl Component for Counted {
    fn layout_hooks(&self) -> Option<&dyn LayoutHooks> {
        Some(self)
    }

    fn class_name(&self) -> &'static str {
        "Counted"
    }

    fn inner(&self) -> Option<&dyn Component> {
        Some(&self.base)
    }

    fn inner_mut(&mut self) -> Option<&mut dyn Component> {
        Some(&mut self.base)
    }
}

impl LayoutHooks for Counted {
    fn arrange(&self, _area: RectF, _children: &[ArrangeItem]) -> Option<Vec<RectF>> {
        self.arranged.set(self.arranged.get() + 1);
        None
    }
}

fn counted(ui: &mut Ui, parent: Option<ComponentId>, w: f32, h: f32) -> (ComponentId, Rc<Cell<u32>>) {
    let count = Rc::new(Cell::new(0));
    let c = Counted { base: boxy(w, h), arranged: count.clone() };
    let id = match parent {
        Some(p) => ui.add(c, p),
        None => ui.add_root(c),
    };
    (id, count)
}

/// Um form com uma grade de 64 frames (8×8), cada um com um rótulo: o caso de um app real.
struct Grid {
    ui: Ui,
    form_count: Rc<Cell<u32>>,
    frames: Vec<(ComponentId, Rc<Cell<u32>>)>,
    labels: Vec<ComponentId>,
}

fn grid() -> Grid {
    let mut ui = Ui::new();
    let (form, form_count) = counted(&mut ui, None, 1600.0, 900.0);
    let mut frames = Vec::new();
    let mut labels = Vec::new();
    for i in 0..64 {
        let (frame, count) = counted(&mut ui, Some(form), 200.0, 112.0);
        ui.control_mut(frame).unwrap().set_position((i % 8) as f32 * 200.0, (i / 8) as f32 * 112.0);
        let label = ui.add(boxy(80.0, 20.0), frame);
        ui.control_mut(label).unwrap().set_position(10.0, 10.0);
        frames.push((frame, count));
        labels.push(label);
    }
    ui.layout();
    Grid { ui, form_count, frames, labels }
}

#[test]
fn setting_the_value_a_property_already_has_does_not_lay_out_again() {
    let mut g = grid();
    let (form_before, frame_before) = (g.form_count.get(), g.frames[0].1.get());
    // A volta de um timer do app: o mesmo `visible` e a leitura do tamanho, em cada frame.
    for (&label, &(frame, _)) in g.labels.iter().zip(&g.frames) {
        g.ui.set_prop(label, "visible", true).unwrap();
        g.ui.set_prop(label, "left", 10.0).unwrap();
        assert!(!g.ui.needs_layout(), "o mesmo valor não marca o layout");
        assert_eq!(g.ui.bounds(frame).unwrap().width, 200.0);
    }
    assert_eq!((g.form_count.get(), g.frames[0].1.get()), (form_before, frame_before), "nada foi refeito");
}

#[test]
fn a_change_inside_a_frame_lays_out_only_that_frame() {
    let mut g = grid();
    let form_before = g.form_count.get();
    let others: Vec<u32> = g.frames.iter().map(|(_, c)| c.get()).collect();
    // Um rótulo de um frame muda de lugar: só aquele frame é refeito, e o `bounds` já o vê.
    g.ui.set_prop(g.labels[5], "left", 40.0).unwrap();
    assert!(g.ui.needs_layout());
    assert_eq!(g.ui.bounds(g.labels[5]).unwrap().x, 40.0);
    assert!(!g.ui.needs_layout());
    assert_eq!(g.form_count.get(), form_before, "o form (e os outros 63 frames) não foram refeitos");
    for (i, (_, c)) in g.frames.iter().enumerate() {
        let expected = if i == 5 { others[i] + 1 } else { others[i] };
        assert_eq!(c.get(), expected, "frame {i}");
    }
    // Esconder e mostrar também refaz só o frame (e o rótulo volta ao lugar dele).
    g.ui.set_prop(g.labels[7], "visible", false).unwrap();
    g.ui.update_layout();
    g.ui.set_prop(g.labels[7], "visible", true).unwrap();
    assert_eq!(g.ui.bounds(g.labels[7]).unwrap(), RectF::new(10.0, 10.0, 80.0, 20.0));
    assert_eq!(g.form_count.get(), form_before);
}

#[test]
fn a_frame_sized_by_its_content_lays_out_its_parent_too() {
    let mut g = grid();
    let form_before = g.form_count.get();
    // O frame mede a própria altura pelo conteúdo: a mudança dentro dele alcança o form.
    let (frame, _) = g.frames[3];
    g.ui.set_prop(frame, "auto_size", true).unwrap();
    g.ui.update_layout();
    let after_auto = g.form_count.get();
    assert!(after_auto > form_before);
    g.ui.set_prop(g.labels[3], "height", 40.0).unwrap();
    g.ui.update_layout();
    assert_eq!(g.form_count.get(), after_auto + 1, "o form foi refeito");
}

#[test]
fn a_change_in_a_control_of_the_form_itself_still_lays_out_the_form() {
    let mut g = grid();
    let form_before = g.form_count.get();
    let (frame, _) = g.frames[0];
    g.ui.set_prop(frame, "width", 150.0).unwrap();
    assert_eq!(g.ui.bounds(frame).unwrap().width, 150.0);
    assert_eq!(g.form_count.get(), form_before + 1);
}

#[test]
fn an_anchored_control_born_hidden_follows_the_parent_when_shown() {
    // Na janela: desenhado em 640×360, escondido, a janela cresce e só então ele aparece.
    let mut ui = Ui::new();
    let form = ui.add_root(boxy(640.0, 360.0));
    let corner = ui.add(boxy(30.0, 20.0), form);
    ui.control_mut(corner).unwrap().set_position(600.0, 330.0).set_anchors(Anchors::RIGHT | Anchors::BOTTOM).set_visible(false);
    ui.layout();
    ui.control_mut(form).unwrap().set_size(1600.0, 900.0);
    ui.layout();
    ui.set_prop(corner, "visible", true).unwrap();
    assert_eq!(ui.bounds(corner), Some(RectF::new(1560.0, 870.0, 30.0, 20.0)));

    // Num frame desenhado em 640×360 e embutido numa área de 1600×900.
    let frame = ui.add_root(boxy(640.0, 360.0));
    let label = ui.add(boxy(30.0, 20.0), frame);
    ui.control_mut(label).unwrap().set_position(600.0, 330.0).set_anchors(Anchors::RIGHT | Anchors::BOTTOM).set_visible(false);
    ui.layout();
    let host = ui.add_root(boxy(1600.0, 900.0));
    ui.set_parent(frame, Some(host)).unwrap();
    ui.control_mut(frame).unwrap().set_align(Align::Client);
    ui.layout();
    ui.set_prop(label, "visible", true).unwrap();
    assert_eq!(ui.bounds(label), Some(RectF::new(1560.0, 870.0, 30.0, 20.0)), "no canto da área, e não perto de (640, 360)");
}

#[test]
fn the_owner_of_a_modal_is_the_active_window() {
    let mut ui = Ui::new();
    let a = ui.add_root(boxy(400.0, 300.0));
    let b = ui.add_root(boxy(400.0, 300.0));
    let modal = ui.add_root(boxy(200.0, 100.0));
    ui.show(a).unwrap();
    ui.show(b).unwrap();
    // O usuário volta para a primeira janela (a plataforma avisa pelo `on_activate`) e pede o
    // modal de lá: ele fica preso a ela, e não à aberta por último.
    ui.fire(a, "on_activate", &EventArgs::None);
    ui.show_modal(modal, |_, _| {}).unwrap();
    assert_eq!(ui.form_owner(modal), Some(a));
    ui.close(modal);

    // Um clique na segunda também a faz ativa.
    let press = PointerEvent { position: PointF::new(10.0, 10.0), button: PointerButton::Primary, click_count: 1, ..Default::default() };
    ui.pointer_down(b, press);
    ui.pointer_up(b, press);
    ui.show_modal(modal, |_, _| {}).unwrap();
    assert_eq!(ui.form_owner(modal), Some(b));
}

#[test]
fn the_project_font_survives_a_theme_change() {
    let mut ui = Ui::new();
    ui.set_project_font("Roboto");
    ui.set_theme(Theme::dark());
    let fonts = &ui.theme().fonts;
    assert!(!fonts.is_empty());
    for (name, font) in fonts {
        if font.family.to_lowercase().contains("mono") || font.family == "monospace" {
            continue;
        }
        assert_eq!(font.family, "Roboto", "a fonte {name} do tema trocado");
    }
    // E num tema da biblioteca, pelo nome.
    for name in ["Windows 11 Dark", "Material 3 Light"] {
        let Some(theme) = themes::builtin(name).cloned() else { continue };
        ui.set_theme(theme);
        assert!(ui.theme().fonts.values().any(|f| f.family == "Roboto"), "tema {name}");
    }
}

/// A medição do layout refeito, em release: `cargo test --release -p aylis-core --test
/// reported_defects -- --ignored --nocapture`. Uma volta do timer (64 frames de 20 controles:
/// uma propriedade e a leitura do tamanho em cada um), pelo caminho antigo (o form inteiro
/// marcado), com o mesmo valor e com um valor que muda.
#[test]
#[ignore]
fn measure_the_timer_round_of_64_frames() {
    let mut ui = Ui::new();
    let form = ui.add_root(boxy(1600.0, 900.0));
    let mut cells = Vec::new();
    for i in 0..64 {
        let frame = ui.add(boxy(200.0, 112.0), form);
        ui.control_mut(frame).unwrap().set_position((i % 8) as f32 * 200.0, (i / 8) as f32 * 112.0);
        let mut first = None;
        for k in 0..20 {
            let c = ui.add(boxy(40.0, 20.0), frame);
            ui.control_mut(c).unwrap().set_position((k % 4) as f32 * 45.0, (k / 4) as f32 * 22.0).set_anchors(Anchors::RIGHT | Anchors::BOTTOM);
            first.get_or_insert(c);
        }
        cells.push((frame, first.unwrap()));
    }
    ui.layout();
    let rounds = 50;
    let time = |ui: &mut Ui, f: &dyn Fn(&mut Ui, ComponentId, ComponentId, usize)| {
        let start = std::time::Instant::now();
        for r in 0..rounds {
            for &(frame, label) in &cells {
                f(ui, frame, label, r);
            }
        }
        start.elapsed().as_secs_f64() * 1000.0 / rounds as f64
    };
    let old = time(&mut ui, &|ui, frame, label, _| {
        ui.invalidate_form(label);
        let _ = ui.bounds(frame);
    });
    let same = time(&mut ui, &|ui, frame, label, _| {
        ui.set_prop(label, "visible", true).unwrap();
        let _ = ui.bounds(frame);
    });
    let changed = time(&mut ui, &|ui, frame, label, r| {
        ui.set_prop(label, "left", if r % 2 == 0 { 1.0 } else { 0.0 }).unwrap();
        let _ = ui.bounds(frame);
    });
    println!("uma volta (64 frames): antes {old:.2} ms; o mesmo valor {same:.3} ms; um valor que muda {changed:.2} ms");
}
