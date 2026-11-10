mod common;

use common::boxy;
use aylis_core::*;

fn root(ui: &mut Ui, w: f32, h: f32) -> ComponentId {
    ui.add_root(boxy(w, h))
}

fn child(ui: &mut Ui, parent: ComponentId, align: Align, w: f32, h: f32) -> ComponentId {
    let id = ui.add(boxy(w, h), parent);
    ui.control_mut(id).unwrap().set_align(align);
    id
}

#[test]
fn dock_order_top_bottom_left_right_client() {
    let mut ui = Ui::new();
    let form = root(&mut ui, 400.0, 300.0);
    ui.control_mut(form).unwrap().set_padding(10.0);

    let top = child(&mut ui, form, Align::Top, 0.0, 50.0);
    ui.control_mut(top).unwrap().set_margins(5.0);
    let bottom = child(&mut ui, form, Align::Bottom, 0.0, 40.0);
    let left = child(&mut ui, form, Align::Left, 80.0, 0.0);
    let right = child(&mut ui, form, Align::Right, 60.0, 0.0);
    let client = child(&mut ui, form, Align::Client, 0.0, 0.0);

    ui.layout();
    assert_eq!(ui.bounds(top), Some(RectF::new(15.0, 15.0, 370.0, 50.0)));
    assert_eq!(ui.bounds(bottom), Some(RectF::new(10.0, 250.0, 380.0, 40.0)));
    assert_eq!(ui.bounds(left), Some(RectF::new(10.0, 70.0, 80.0, 180.0)));
    assert_eq!(ui.bounds(right), Some(RectF::new(330.0, 70.0, 60.0, 180.0)));
    assert_eq!(ui.bounds(client), Some(RectF::new(90.0, 70.0, 240.0, 180.0)));
}

#[test]
fn most_left_wins_over_top_regardless_of_child_order() {
    let mut ui = Ui::new();
    let form = root(&mut ui, 200.0, 100.0);
    let top = child(&mut ui, form, Align::Top, 0.0, 20.0);
    let side = child(&mut ui, form, Align::MostLeft, 30.0, 0.0);
    ui.layout();
    assert_eq!(ui.bounds(side), Some(RectF::new(0.0, 0.0, 30.0, 100.0)));
    assert_eq!(ui.bounds(top), Some(RectF::new(30.0, 0.0, 170.0, 20.0)));
}

#[test]
fn same_align_follows_child_order() {
    let mut ui = Ui::new();
    let form = root(&mut ui, 100.0, 100.0);
    let a = child(&mut ui, form, Align::Top, 0.0, 10.0);
    let b = child(&mut ui, form, Align::Top, 0.0, 20.0);
    let c = child(&mut ui, form, Align::Bottom, 0.0, 5.0);
    let d = child(&mut ui, form, Align::Bottom, 0.0, 7.0);
    ui.layout();
    assert_eq!(ui.bounds(a).unwrap().y, 0.0);
    assert_eq!(ui.bounds(b).unwrap().y, 10.0);
    assert_eq!(ui.bounds(c).unwrap().y, 95.0);
    assert_eq!(ui.bounds(d).unwrap().y, 88.0);

    // trocar a ordem dos filhos troca a ordem do encaixe
    ui.set_child_index(b, 0).unwrap();
    ui.layout();
    assert_eq!(ui.bounds(b).unwrap().y, 0.0);
    assert_eq!(ui.bounds(a).unwrap().y, 20.0);
}

#[test]
fn center_variants_contents_and_fit() {
    let mut ui = Ui::new();
    let form = root(&mut ui, 200.0, 100.0);
    ui.control_mut(form).unwrap().set_padding(10.0);
    let center = child(&mut ui, form, Align::Center, 50.0, 20.0);
    let vcenter = child(&mut ui, form, Align::VertCenter, 50.0, 20.0);
    let hcenter = child(&mut ui, form, Align::HorzCenter, 50.0, 20.0);
    let contents = child(&mut ui, form, Align::Contents, 1.0, 1.0);
    ui.layout();
    // área livre: x10 y10 w180 h80
    assert_eq!(ui.bounds(center), Some(RectF::new(75.0, 40.0, 50.0, 20.0)));
    assert_eq!(ui.bounds(vcenter), Some(RectF::new(10.0, 40.0, 180.0, 20.0)));
    assert_eq!(ui.bounds(hcenter), Some(RectF::new(75.0, 10.0, 50.0, 80.0)));
    assert_eq!(ui.bounds(contents), Some(RectF::new(0.0, 0.0, 200.0, 100.0)));

    let mut ui = Ui::new();
    let form = root(&mut ui, 200.0, 200.0);
    let fit = child(&mut ui, form, Align::Fit, 100.0, 50.0);
    ui.layout();
    assert_eq!(ui.bounds(fit), Some(RectF::new(0.0, 50.0, 200.0, 100.0)));
    // o tamanho pedido não é alterado, então recalcular dá o mesmo resultado
    ui.layout();
    assert_eq!(ui.bounds(fit), Some(RectF::new(0.0, 50.0, 200.0, 100.0)));
}

#[test]
fn horizontal_and_vertical_keep_the_other_axis() {
    let mut ui = Ui::new();
    let form = root(&mut ui, 200.0, 100.0);
    let h = child(&mut ui, form, Align::Horizontal, 10.0, 20.0);
    ui.control_mut(h).unwrap().set_position(5.0, 30.0);
    let v = child(&mut ui, form, Align::Vertical, 10.0, 20.0);
    ui.control_mut(v).unwrap().set_position(40.0, 7.0);
    ui.layout();
    assert_eq!(ui.bounds(h), Some(RectF::new(0.0, 30.0, 200.0, 20.0)));
    assert_eq!(ui.bounds(v), Some(RectF::new(40.0, 0.0, 10.0, 100.0)));
}

#[test]
fn invisible_children_do_not_take_space() {
    let mut ui = Ui::new();
    let form = root(&mut ui, 100.0, 100.0);
    let hidden = child(&mut ui, form, Align::Top, 0.0, 30.0);
    ui.control_mut(hidden).unwrap().set_visible(false);
    let client = child(&mut ui, form, Align::Client, 0.0, 0.0);
    ui.layout();
    assert_eq!(ui.bounds(client), Some(RectF::new(0.0, 0.0, 100.0, 100.0)));
}

#[test]
fn nested_coordinates_are_relative_to_parent() {
    let mut ui = Ui::new();
    let form = root(&mut ui, 300.0, 200.0);
    let side = child(&mut ui, form, Align::Left, 100.0, 0.0);
    let panel = child(&mut ui, form, Align::Client, 0.0, 0.0);
    ui.control_mut(panel).unwrap().set_padding(8.0);
    let inner = child(&mut ui, panel, Align::Top, 0.0, 24.0);
    ui.layout();
    assert_eq!(ui.bounds(side), Some(RectF::new(0.0, 0.0, 100.0, 200.0)));
    assert_eq!(ui.bounds(inner), Some(RectF::new(8.0, 8.0, 184.0, 24.0)));
    assert_eq!(ui.absolute_rect(inner), Some(RectF::new(108.0, 8.0, 184.0, 24.0)));
}

#[test]
fn anchors_follow_parent_resize() {
    let mut ui = Ui::new();
    let form = root(&mut ui, 200.0, 100.0);

    let right = ui.add(boxy(40.0, 20.0), form);
    ui.control_mut(right).unwrap().set_position(150.0, 10.0).set_anchors(Anchors::RIGHT | Anchors::TOP);

    let stretch = ui.add(boxy(180.0, 20.0), form);
    ui.control_mut(stretch).unwrap().set_position(10.0, 40.0).set_anchors(Anchors::LEFT | Anchors::TOP | Anchors::RIGHT);

    let floating = ui.add(boxy(20.0, 20.0), form);
    ui.control_mut(floating).unwrap().set_position(90.0, 70.0).set_anchors(Anchors::NONE);

    let corner = ui.add(boxy(30.0, 10.0), form);
    ui.control_mut(corner).unwrap().set_position(160.0, 80.0).set_anchors(Anchors::RIGHT | Anchors::BOTTOM);

    ui.layout();
    assert_eq!(ui.bounds(right), Some(RectF::new(150.0, 10.0, 40.0, 20.0)));

    ui.control_mut(form).unwrap().set_size(300.0, 150.0);
    ui.layout();
    assert_eq!(ui.bounds(right), Some(RectF::new(250.0, 10.0, 40.0, 20.0)));
    assert_eq!(ui.bounds(stretch), Some(RectF::new(10.0, 40.0, 280.0, 20.0)));
    assert_eq!(ui.bounds(floating), Some(RectF::new(140.0, 95.0, 20.0, 20.0)));
    assert_eq!(ui.bounds(corner), Some(RectF::new(260.0, 130.0, 30.0, 10.0)));

    // encolher abaixo do tamanho original e voltar não perde a referência
    ui.control_mut(form).unwrap().set_size(50.0, 100.0);
    ui.layout();
    assert_eq!(ui.bounds(stretch).unwrap().width, 30.0);
    ui.control_mut(form).unwrap().set_size(200.0, 100.0);
    ui.layout();
    assert_eq!(ui.bounds(stretch), Some(RectF::new(10.0, 40.0, 180.0, 20.0)));
}

#[test]
fn on_resize_fires_when_size_changes() {
    use std::cell::Cell;
    use std::rc::Rc;

    let mut ui = Ui::new();
    let form = root(&mut ui, 100.0, 100.0);
    let client = child(&mut ui, form, Align::Client, 0.0, 0.0);
    let count = Rc::new(Cell::new(0));
    let c = count.clone();
    ui.on(client, "on_resize", move |_, _, _| c.set(c.get() + 1));

    ui.layout();
    assert_eq!(count.get(), 1);
    ui.layout();
    assert_eq!(count.get(), 1, "sem mudança de tamanho, sem evento");
    ui.control_mut(form).unwrap().set_width(150.0);
    ui.layout();
    assert_eq!(count.get(), 2);
}

#[test]
fn max_width_centers_top_and_client_children_in_a_wide_area() {
    let mut ui = Ui::new();
    let form = root(&mut ui, 1000.0, 300.0);
    let top = child(&mut ui, form, Align::Top, 0.0, 50.0);
    ui.control_mut(top).unwrap().set_max_width(600.0);
    let client = child(&mut ui, form, Align::Client, 0.0, 0.0);
    ui.set_prop(client, "max_width", 400.0).unwrap();
    ui.layout();
    assert_eq!(ui.bounds(top), Some(RectF::new(200.0, 0.0, 600.0, 50.0)));
    assert_eq!(ui.bounds(client), Some(RectF::new(300.0, 50.0, 400.0, 250.0)));
    // Numa área mais estreita que o limite, ocupa a largura toda.
    ui.control_mut(form).unwrap().set_size(500.0, 300.0);
    ui.layout();
    assert_eq!(ui.bounds(top), Some(RectF::new(0.0, 0.0, 500.0, 50.0)));
}

#[test]
fn min_width_stops_shrinking_and_the_rest_is_cut() {
    let mut ui = Ui::new();
    let form = root(&mut ui, 600.0, 300.0);
    let top = child(&mut ui, form, Align::Top, 0.0, 50.0);
    ui.set_prop(top, "min_width", 400.0).unwrap();
    let client = child(&mut ui, form, Align::Client, 0.0, 0.0);
    ui.set_prop(client, "min_width", 300.0).unwrap();
    ui.set_prop(client, "max_height", 100.0).unwrap();
    let left = child(&mut ui, form, Align::Left, 20.0, 0.0);
    ui.set_prop(left, "min_width", 40.0).unwrap();
    ui.set_prop(left, "min_height", 400.0).unwrap();
    ui.layout();
    // Com espaço: a largura da área (a mínima não estica).
    assert_eq!(ui.bounds(top), Some(RectF::new(0.0, 0.0, 600.0, 50.0)));
    assert_eq!(ui.bounds(left), Some(RectF::new(0.0, 50.0, 40.0, 400.0)), "a largura e a altura mínimas");
    assert_eq!(ui.bounds(client).map(|b| b.height), Some(100.0), "a altura máxima");
    // Mais estreito que o mínimo: para nele, encostado à esquerda (o resto fica cortado).
    ui.control_mut(form).unwrap().set_size(200.0, 300.0);
    ui.layout();
    assert_eq!(ui.bounds(top), Some(RectF::new(0.0, 0.0, 400.0, 50.0)));
    assert_eq!(ui.bounds(client), Some(RectF::new(40.0, 50.0, 300.0, 100.0)));
    // O conteúdo passa da área: um ScrollBox rola até lá.
    assert_eq!(ui.control(form).unwrap().content_size().width, 400.0);
}

#[test]
fn self_align_keeps_the_own_size_at_the_start_center_or_end_of_the_band() {
    let mut ui = Ui::new();
    let form = root(&mut ui, 600.0, 300.0);
    let start = child(&mut ui, form, Align::Top, 200.0, 30.0);
    ui.set_prop(start, "self_align", "start").unwrap();
    let center = child(&mut ui, form, Align::Top, 200.0, 30.0);
    ui.set_prop(center, "self_align", "center").unwrap();
    let end = child(&mut ui, form, Align::Top, 200.0, 30.0);
    ui.set_prop(end, "self_align", "end").unwrap();
    let side = child(&mut ui, form, Align::Left, 50.0, 100.0);
    ui.set_prop(side, "self_align", "end").unwrap();
    ui.layout();
    assert_eq!(ui.bounds(start), Some(RectF::new(0.0, 0.0, 200.0, 30.0)));
    assert_eq!(ui.bounds(center), Some(RectF::new(200.0, 30.0, 200.0, 30.0)));
    assert_eq!(ui.bounds(end), Some(RectF::new(400.0, 60.0, 200.0, 30.0)));
    assert_eq!(ui.bounds(side), Some(RectF::new(0.0, 200.0, 50.0, 100.0)), "à esquerda, a própria altura no fim");
    // A janela diminui: o do começo fica onde está, sem conta nenhuma do app.
    ui.control_mut(form).unwrap().set_width(300.0);
    assert_eq!(ui.bounds(start), Some(RectF::new(0.0, 0.0, 200.0, 30.0)));
    assert_eq!(ui.bounds(end).map(|b| b.x), Some(100.0));
}

#[test]
fn self_align_with_max_width_fills_up_to_the_maximum_at_the_start() {
    let mut ui = Ui::new();
    let form = root(&mut ui, 1000.0, 300.0);
    let column = child(&mut ui, form, Align::Top, 0.0, 30.0);
    ui.set_prop(column, "self_align", "start").unwrap();
    ui.set_prop(column, "max_width", 760.0).unwrap();
    ui.set_prop(column, "min_width", 280.0).unwrap();
    for (w, expected) in [(1000.0, 760.0), (900.0, 760.0), (500.0, 500.0), (200.0, 280.0)] {
        ui.control_mut(form).unwrap().set_width(w);
        assert_eq!(ui.bounds(column), Some(RectF::new(0.0, 0.0, expected, 30.0)), "janela de {w}");
    }
}

#[test]
fn bounds_read_right_after_a_change_are_already_laid_out() {
    let mut ui = Ui::new();
    let form = root(&mut ui, 800.0, 300.0);
    let page = child(&mut ui, form, Align::Client, 0.0, 0.0);
    let inner = child(&mut ui, page, Align::Top, 0.0, 40.0);
    ui.layout();
    // Escondida, a página fica fora do layout; a janela diminui nesse meio tempo.
    ui.set_prop(page, "visible", false).unwrap();
    ui.control_mut(form).unwrap().set_size(500.0, 300.0);
    ui.layout();
    assert_eq!(ui.laid_out_bounds(inner).map(|b| b.width), Some(800.0), "escondida, a medida é a velha");
    // Mostrou e já lê, sem esperar o quadro: a largura nova (antes vinha a velha).
    ui.set_prop(page, "visible", true).unwrap();
    assert_eq!(ui.bounds(inner).map(|b| b.width), Some(500.0));
    ui.control_mut(form).unwrap().set_width(400.0);
    assert_eq!(ui.bounds(page).map(|b| b.width), Some(400.0));
}

#[test]
fn a_form_is_never_smaller_than_its_minimum() {
    let mut ui = Ui::new();
    let form = root(&mut ui, 600.0, 300.0);
    ui.set_prop(form, "min_width", 500.0).unwrap();
    ui.set_prop(form, "min_height", 250.0).unwrap();
    let client = child(&mut ui, form, Align::Client, 0.0, 0.0);
    ui.control_mut(form).unwrap().set_size(300.0, 100.0);
    ui.layout();
    assert_eq!(ui.bounds(form), Some(RectF::new(0.0, 0.0, 500.0, 250.0)));
    assert_eq!(ui.bounds(client), Some(RectF::new(0.0, 0.0, 500.0, 250.0)));
}

/// Conta quantas vezes o layout organizou os filhos dele.
struct Counted {
    base: common::Boxy,
    arranged: std::rc::Rc<std::cell::Cell<u32>>,
}

impl Component for Counted {
    fn class_name(&self) -> &'static str {
        "Counted"
    }

    fn inner(&self) -> Option<&dyn Component> {
        Some(&self.base)
    }

    fn inner_mut(&mut self) -> Option<&mut dyn Component> {
        Some(&mut self.base)
    }

    fn arrange(&self, _area: RectF, _children: &[ArrangeItem]) -> Option<Vec<RectF>> {
        self.arranged.set(self.arranged.get() + 1);
        None
    }
}

#[test]
fn a_change_in_one_form_lays_out_only_that_form() {
    use std::cell::Cell;
    use std::rc::Rc;

    let mut ui = Ui::new();
    let counts: Vec<Rc<Cell<u32>>> = (0..2).map(|_| Rc::new(Cell::new(0))).collect();
    let mut forms = Vec::new();
    for count in &counts {
        let form = ui.add_root(Counted { base: boxy(300.0, 200.0), arranged: count.clone() });
        child(&mut ui, form, Align::None, 50.0, 20.0);
        forms.push(form);
    }
    ui.layout();
    assert_eq!((counts[0].get(), counts[1].get()), (1, 1));
    // Mudar um controle do primeiro form refaz só o primeiro.
    let first = ui.children(forms[0])[0];
    ui.set_prop(first, "width", 80.0).unwrap();
    assert!(ui.needs_layout());
    ui.update_layout();
    assert_eq!((counts[0].get(), counts[1].get()), (2, 1));
    assert!(!ui.needs_layout());
    // O pedido sem dizer onde (`invalidate`) refaz todos.
    ui.invalidate();
    ui.update_layout();
    assert_eq!((counts[0].get(), counts[1].get()), (3, 2));
}
