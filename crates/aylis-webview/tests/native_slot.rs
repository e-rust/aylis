//! Onde a vista nativa (o WebView do sistema, o `<iframe>` no navegador) fica dentro de uma
//! caixa rolada: acompanha a rolagem uma vez só e leva o recorte da caixa (fora dela, não
//! aparece por cima do resto).

use aylis_controls::{Form, ScrollBox};
use aylis_core::{Align, PointF, Ui};
use aylis_webview::WebView;

#[test]
fn a_view_in_a_scrolled_box_moves_with_it_and_is_clipped_by_it() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("f", 400.0, 300.0));
    let scroll = ui.add(ScrollBox::default(), form);
    ui.control_mut(scroll).unwrap().set_align(Align::None).set_rect(0.0, 60.0, 400.0, 240.0);
    let web = ui.add(WebView::new("https://aylis.dev"), scroll);
    ui.control_mut(web).unwrap().set_align(Align::None).set_rect(10.0, 20.0, 200.0, 400.0);
    ui.layout();
    let slot = |ui: &Ui| ui.native_views(form).into_iter().find(|s| s.id == web).unwrap();
    assert_eq!((slot(&ui).rect.y, slot(&ui).clip.map(|c| c.y)), (80.0, Some(60.0)), "dentro da caixa, recortada pela borda de baixo");

    // Rola 50: a vista sobe 50 (não 100) e o recorte continua a caixa.
    ui.control_mut(scroll).unwrap().set_scroll_offset(PointF::new(0.0, 50.0));
    ui.invalidate();
    ui.layout();
    let s = slot(&ui);
    assert_eq!(s.rect.y, 30.0);
    assert_eq!(s.clip.map(|c| (c.y, c.bottom())), Some((60.0, 300.0)));
    assert!(s.visible);
}
