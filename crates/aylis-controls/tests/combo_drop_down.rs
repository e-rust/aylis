//! A lista aberta do ComboBox: quantas opções mostra (`drop_down_count`, o `DropDownCount`
//! do Delphi), a rolagem pela roda e pela barra, e o encolher quando o form não tem lugar.

use aylis_controls::*;
use aylis_core::*;

fn at(x: f32, y: f32) -> PointerEvent {
    PointerEvent { position: PointF::new(x, y), ..Default::default() }
}

fn click(ui: &mut Ui, form: ComponentId, x: f32, y: f32) {
    ui.pointer_move(form, at(x, y));
    ui.pointer_down(form, at(x, y));
    ui.pointer_up(form, at(x, y));
}

/// Um form 400x300 com um ComboBox de 20 opções (`opção 0`...) em `(10, y)`, 150x32.
fn combo_at(y: f32) -> (Ui, ComponentId, ComponentId) {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("teste", 400.0, 300.0));
    let items: Vec<String> = (0..20).map(|i| format!("opção {i}")).collect();
    let combo = ui.add(ComboBox::new(&items.iter().map(String::as_str).collect::<Vec<_>>()), form);
    ui.control_mut(combo).unwrap().set_rect(10.0, y, 150.0, 32.0);
    ui.layout();
    (ui, form, combo)
}

/// A lista aberta (o último filho do form).
fn popup(ui: &mut Ui, form: ComponentId) -> RectF {
    let id = *ui.children(form).last().unwrap();
    ui.bounds(id).unwrap()
}

fn open(ui: &mut Ui, form: ComponentId, combo: ComponentId) {
    let b = ui.bounds(combo).unwrap();
    // Aberta: o clique no combo fecha, e o seguinte abre de novo (com o tamanho de agora).
    if ui.get::<ComboBox>(combo).unwrap().is_open() {
        click(ui, form, b.x + 20.0, b.y + 10.0);
    }
    click(ui, form, b.x + 20.0, b.y + 10.0);
    ui.layout();
    assert!(ui.get::<ComboBox>(combo).unwrap().is_open());
}

#[test]
fn drop_down_count_says_how_many_rows_show() {
    let (mut ui, form, combo) = combo_at(10.0);
    assert_eq!(ui.get_prop(combo, "drop_down_count").unwrap(), PropValue::Int(10), "o padrão, como antes");
    ui.set_prop(combo, "drop_down_count", 4i64).unwrap();
    open(&mut ui, form, combo);
    let list = popup(&mut ui, form);
    assert_eq!(list.height, 4.0 * 28.0 + 4.0, "4 linhas");
    assert_eq!(list.y, 10.0 + 32.0 + 2.0, "embaixo do combo");
    // A quarta linha é a última à vista: a quinta não está lá (um clique abaixo não escolhe).
    click(&mut ui, form, 50.0, list.y + 2.0 + 3.0 * 28.0 + 10.0);
    assert_eq!(ui.get::<ComboBox>(combo).unwrap().text(), "opção 3");
    // Zero ou negativo vale uma linha.
    ui.set_prop(combo, "drop_down_count", 0i64).unwrap();
    open(&mut ui, form, combo);
    assert_eq!(popup(&mut ui, form).height, 28.0 + 4.0);
}

#[test]
fn the_wheel_scrolls_the_list_and_the_click_picks_what_shows() {
    let (mut ui, form, combo) = combo_at(10.0);
    ui.set_prop(combo, "drop_down_count", 5i64).unwrap();
    open(&mut ui, form, combo);
    let list = popup(&mut ui, form);
    for _ in 0..3 {
        ui.wheel(form, WheelEvent { position: PointF::new(50.0, list.y + 20.0), delta: PointF::new(0.0, -40.0), ..Default::default() });
    }
    // Três linhas abaixo: a primeira à vista é a `opção 3`.
    click(&mut ui, form, 50.0, list.y + 2.0 + 10.0);
    assert_eq!(ui.get::<ComboBox>(combo).unwrap().text(), "opção 3");
}

#[test]
fn dragging_the_bar_goes_to_the_end_without_picking() {
    let (mut ui, form, combo) = combo_at(10.0);
    ui.set_prop(combo, "drop_down_count", 5i64).unwrap();
    open(&mut ui, form, combo);
    let list = popup(&mut ui, form);
    // A barra fica à direita da lista; o polegar começa em cima.
    let bar_x = list.right() - 6.0;
    ui.pointer_move(form, at(bar_x, list.y + 8.0));
    ui.pointer_down(form, at(bar_x, list.y + 8.0));
    ui.pointer_move(form, at(bar_x, list.bottom() + 40.0));
    ui.pointer_up(form, at(bar_x, list.bottom() + 40.0));
    let c = ui.get::<ComboBox>(combo).unwrap();
    assert!(c.is_open(), "soltar a barra não fecha");
    assert_eq!(c.item_index, -1, "nem escolhe");
    // No fim: a última linha à vista é a `opção 19`.
    click(&mut ui, form, 50.0, list.y + 2.0 + 4.0 * 28.0 + 10.0);
    assert_eq!(ui.get::<ComboBox>(combo).unwrap().text(), "opção 19");
}

#[test]
fn without_room_the_list_shrinks_to_fit_the_form() {
    // Em y = 200, num form de 300: embaixo cabem 2 linhas, em cima 6. As 10 não cabem em
    // nenhum dos lados: abre em cima, com 6.
    let (mut ui, form, combo) = combo_at(200.0);
    open(&mut ui, form, combo);
    let list = popup(&mut ui, form);
    assert_eq!(list.height, 6.0 * 28.0 + 4.0, "{list:?}");
    assert_eq!(list.bottom(), 200.0 - 2.0, "logo acima do combo");
    assert!(list.y >= 0.0);
    // Com 4, cabe em cima inteira; com 2, cabe embaixo.
    ui.set_prop(combo, "drop_down_count", 4i64).unwrap();
    open(&mut ui, form, combo);
    assert_eq!(popup(&mut ui, form).height, 4.0 * 28.0 + 4.0);
    assert!(popup(&mut ui, form).bottom() <= 200.0);
    ui.set_prop(combo, "drop_down_count", 2i64).unwrap();
    open(&mut ui, form, combo);
    assert_eq!(popup(&mut ui, form).y, 232.0 + 2.0, "embaixo");
}

/// Com mais opções que as à vista, a barra aparece à direita da lista (o polegar em cima, na
/// cor do tema); com todas à vista, não.
#[test]
fn the_bar_is_painted_only_when_the_list_scrolls() {
    let paint = |count: i64, items: &str| {
        let (mut ui, form, combo) = combo_at(10.0);
        ui.set_prop(combo, "drop_down_count", count).unwrap();
        if !items.is_empty() {
            ui.set_prop(combo, "items", items).unwrap();
        }
        open(&mut ui, form, combo);
        let list = popup(&mut ui, form);
        let renderer = aylis_skia::SkiaRenderer::new();
        ui.services_mut().text = Some(Box::new(renderer.text_service()));
        let (w, h) = (400u32, 300u32);
        let mut pixels = vec![0u32; (w * h) as usize];
        renderer.render_area_to_pixels(&mut pixels, w, h, 1.0, Color::WHITE, None, |c| ui.paint_region(form, c, None));
        if let Ok(dir) = std::env::var("AYLIS_COMBO_PNG") {
            let rgba: Vec<u8> = pixels.iter().flat_map(|p| [(p >> 16) as u8, (p >> 8) as u8, *p as u8, 255]).collect();
            let _ = aylis_skia::encode_png(&rgba, w, h).map(|png| std::fs::write(format!("{dir}/combo-{count}.png"), png));
        }
        let at = |x: f32, y: f32| pixels[(y as u32 * w + x as u32) as usize] & 0xFFFFFF;
        let x = list.right() - 6.0;
        (at(x, list.y + 10.0), at(x, list.bottom() - 10.0))
    };
    let (thumb, track) = paint(5, "");
    assert_ne!(thumb, track, "o polegar em cima, a calha vazia embaixo");
    let (top, bottom) = paint(10, "a;b;c;d;e");
    assert_eq!(top, bottom, "todas à vista: sem barra");
}
