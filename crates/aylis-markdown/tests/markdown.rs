//! O componente num form: desenha, rola, e o clique num link chega ao `on_link`.

use std::cell::RefCell;
use std::rc::Rc;

use aylis_controls::Form;
use aylis_core::{Align, EventArgs, PointF, PointerEvent, Ui, WheelEvent};
use aylis_markdown::{Item, Markdown};

const SAMPLE: &str = r##"# Manual do aylis

O **aylis** desenha a interface com o *próprio* canvas: `Skia`, `Direct2D` ou o navegador.
Veja a [documentação](https://aylis.dev/docs) e ~~o jeito antigo~~.

## Lista de tarefas

- [x] Temas de plataforma
- [x] Fontes e ícones offline
- [ ] Componente **WebView**
  1. Windows (WebView2)
  2. Navegador (iframe)

> Dica: no FMX seria um `TWebBrowser`; aqui é o `WebView`.

| Componente | Crate | Tamanho |
|:-----------|:-----:|--------:|
| Lottie     | aylis-vector | 120 KB |
| Markdown   | aylis-markdown | 90 KB |

```rust
let md = ui.add(Markdown::new("# Olá"), form);
ui.on(md, "on_link", |_, _, args| println!("{args:?}"));
```

---

Fim.
"##;

fn setup() -> (Ui, aylis_core::ComponentId, aylis_core::ComponentId) {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    aylis_markdown::register(&mut ui);
    let form = ui.add_root(Form::new("md", 560.0, 520.0));
    let md = ui.add(Markdown::new(SAMPLE), form);
    ui.control_mut(md).unwrap().set_align(Align::Client);
    (ui, form, md)
}

#[test]
fn renders_scrolls_and_links() {
    let (mut ui, form, md) = setup();
    let renderer = aylis_skia::SkiaRenderer::new();
    ui.services_mut().text = Some(Box::new(renderer.text_service()));
    ui.layout();
    let png = renderer.render_png(560, 520, 1.0, ui.theme().color("background"), |c| ui.paint(form, c)).unwrap();
    if let Ok(dir) = std::env::var("AYLIS_SHOTS") {
        std::fs::write(std::path::Path::new(&dir).join("markdown.png"), &png).unwrap();
    }
    let height = ui.get::<Markdown>(md).unwrap().content_height();
    assert!(height > 520.0, "o texto passa da altura: {height}");

    // O clique no link (posição do primeiro item com link, mais o padding de 8).
    let links: Rc<RefCell<Vec<String>>> = Rc::default();
    let seen = links.clone();
    ui.on(md, "on_link", move |_, _, args| {
        if let EventArgs::Text(url) = args {
            seen.borrow_mut().push(url.clone());
        }
    });
    let layout = {
        let doc = aylis_markdown::parse(SAMPLE);
        let pal = aylis_markdown::Palette::from_theme(ui.theme(), 0.0);
        let mut measure = |s: &str, st: &aylis_core::TextStyle| aylis_core::Canvas::measure_text(&mut Measure(renderer.text_service()), s, st).width;
        let mut images = |_: &str| None;
        aylis_markdown::Layout::build(&doc, 560.0 - 16.0, &pal, &mut measure, &mut images)
    };
    let at = layout.items.iter().find_map(|i| if let Item::Text { rect, link: Some(_), .. } = i { Some(rect.center()) } else { None }).unwrap();
    let p = PointF::new(at.x + 8.0, at.y + 8.0);
    let e = PointerEvent { position: p, ..Default::default() };
    ui.pointer_move(form, e);
    ui.pointer_down(form, e);
    ui.pointer_up(form, e);
    assert_eq!(links.borrow().as_slice(), ["https://aylis.dev/docs"]);

    // A roda rola o texto.
    ui.wheel(form, WheelEvent { position: PointF::new(200.0, 200.0), delta: PointF::new(0.0, -120.0), ..Default::default() });
    let png2 = renderer.render_png(560, 520, 1.0, ui.theme().color("background"), |c| ui.paint(form, c)).unwrap();
    assert_ne!(png, png2, "rolou");
}

/// A barra de rolagem aceita o clique e o arrasto (antes só a roda rolava, e a barra da página
/// da documentação não respondia ao mouse): um clique embaixo da faixa leva perto do fim, e
/// arrastar o polegar até o topo volta ao começo.
#[test]
fn the_scrollbar_is_clickable_and_draggable() {
    let (mut ui, form, _md) = setup();
    let renderer = aylis_skia::SkiaRenderer::new();
    ui.services_mut().text = Some(Box::new(renderer.text_service()));
    ui.layout();
    let paint = |ui: &mut Ui| renderer.render_png(560, 520, 1.0, ui.theme().color("background"), |c| ui.paint(form, c)).unwrap();
    let top = paint(&mut ui);
    let at = |x: f32, y: f32| PointerEvent { position: PointF::new(x, y), ..Default::default() };
    // Na faixa da barra (os últimos 12 px da direita), embaixo.
    ui.pointer_down(form, at(556.0, 515.0));
    ui.pointer_up(form, at(556.0, 515.0));
    let clicked = paint(&mut ui);
    assert_ne!(top, clicked, "o clique na faixa da barra rolou");
    // Arrastando o polegar até o topo, volta ao começo.
    ui.pointer_down(form, at(556.0, 515.0));
    ui.pointer_move(form, at(556.0, 260.0));
    ui.pointer_move(form, at(556.0, -40.0));
    ui.pointer_up(form, at(556.0, -40.0));
    assert_eq!(paint(&mut ui), top, "o arrasto até o topo voltou ao começo");
}

/// O `highlight` (o achado de uma busca na documentação): sem ocorrência, nada muda; com
/// ocorrência só no fim, a página rola até ela e a pinta.
#[test]
fn the_highlight_marks_the_found_text_and_scrolls_to_it() {
    let (mut ui, form, md) = setup();
    let renderer = aylis_skia::SkiaRenderer::new();
    ui.services_mut().text = Some(Box::new(renderer.text_service()));
    ui.layout();
    let paint = |ui: &mut Ui| renderer.render_png(560, 520, 1.0, ui.theme().color("background"), |c| ui.paint(form, c)).unwrap();
    let top = paint(&mut ui);
    ui.set_prop(md, "highlight", "texto que não existe").unwrap();
    assert_eq!(paint(&mut ui), top, "sem ocorrência, a página fica igual");
    ui.set_prop(md, "highlight", "FIM").unwrap();
    let found = paint(&mut ui);
    assert_ne!(found, top, "rolou até o \"Fim.\" e o destacou");
    assert!(ui.get::<Markdown>(md).unwrap().scroll_offset() > 0.0, "rolou");
    // Sem o destaque, a mesma posição sem a marca.
    ui.set_prop(md, "highlight", "").unwrap();
    assert_ne!(paint(&mut ui), found, "a marca saiu");
}

/// Mede como o renderizador (para achar a posição do link igual à pintura).
struct Measure(aylis_skia::SkiaTextService);

impl aylis_core::Canvas for Measure {
    fn save(&mut self) {}
    fn restore(&mut self) {}
    fn translate(&mut self, _: f32, _: f32) {}
    fn scale(&mut self, _: f32, _: f32) {}
    fn rotate(&mut self, _: f32) {}
    fn clip_rect(&mut self, _: aylis_core::RectF) {}
    fn save_layer(&mut self, _: f32) {}
    fn fill_rect(&mut self, _: aylis_core::RectF, _: f32, _: &aylis_core::Brush) {}
    fn stroke_rect(&mut self, _: aylis_core::RectF, _: f32, _: &aylis_core::Stroke) {}
    fn fill_ellipse(&mut self, _: aylis_core::RectF, _: &aylis_core::Brush) {}
    fn stroke_ellipse(&mut self, _: aylis_core::RectF, _: &aylis_core::Stroke) {}
    fn draw_line(&mut self, _: PointF, _: PointF, _: &aylis_core::Stroke) {}
    fn draw_text(&mut self, _: &str, _: aylis_core::RectF, _: &aylis_core::TextStyle) {}
    fn measure_text(&mut self, text: &str, style: &aylis_core::TextStyle) -> aylis_core::SizeF {
        aylis_core::TextService::measure(&self.0, text, style, None)
    }
}

struct FakeClipboard(Rc<RefCell<String>>);

impl aylis_core::platform::ClipboardService for FakeClipboard {
    fn get_text(&mut self) -> Option<String> {
        Some(self.0.borrow().clone())
    }
    fn set_text(&mut self, text: &str) {
        *self.0.borrow_mut() = text.to_string();
    }
}

/// O texto da documentação é copiável: arrastar seleciona e o Ctrl+C copia, o duplo clique
/// pega a palavra, o Ctrl+A tudo, o botão direito abre o menu Copiar, e o botão Copiar do
/// bloco leva o código inteiro.
#[test]
fn the_text_is_selectable_and_copyable() {
    let (mut ui, form, md) = setup();
    let renderer = aylis_skia::SkiaRenderer::new();
    ui.services_mut().text = Some(Box::new(renderer.text_service()));
    let copied: Rc<RefCell<String>> = Rc::default();
    ui.services_mut().clipboard = Some(Box::new(FakeClipboard(copied.clone())));
    ui.layout();
    let paint = |ui: &mut Ui| renderer.render_png(560, 520, 1.0, ui.theme().color("background"), |c| ui.paint(form, c)).unwrap();
    let before = paint(&mut ui);
    let at = |x: f32, y: f32| PointerEvent { position: PointF::new(x, y), ..Default::default() };
    let ctrl = |c: char| aylis_core::KeyEvent { key: aylis_core::Key::Char(c), modifiers: aylis_core::Modifiers { ctrl: true, ..Default::default() }, repeat: false };
    // Os trechos do layout (coordenadas do conteúdo; o padding é 8).
    let items = {
        let doc = aylis_markdown::parse(SAMPLE);
        let pal = aylis_markdown::Palette::from_theme(ui.theme(), 0.0);
        let mut measure = |s: &str, st: &aylis_core::TextStyle| aylis_core::Canvas::measure_text(&mut Measure(renderer.text_service()), s, st).width;
        let mut images = |_: &str| None;
        aylis_markdown::Layout::build(&doc, 560.0 - 16.0, &pal, &mut measure, &mut images).items
    };
    let title = items.iter().find_map(|i| if let Item::Text { rect, text, .. } = i { (text == "Manual do aylis").then_some(*rect) } else { None }).unwrap();

    // Arrastando do começo ao fim do título.
    ui.pointer_move(form, at(title.x + 9.0, title.center().y + 8.0));
    ui.pointer_down(form, at(title.x + 9.0, title.center().y + 8.0));
    ui.pointer_move(form, at(title.right() + 7.0, title.center().y + 8.0));
    ui.pointer_up(form, at(title.right() + 7.0, title.center().y + 8.0));
    assert_eq!(ui.get::<Markdown>(md).unwrap().selected_text(), "Manual do aylis");
    assert_ne!(paint(&mut ui), before, "a seleção aparece");
    ui.key_down(form, ctrl('c'));
    assert_eq!(copied.borrow().as_str(), "Manual do aylis");

    // Duplo clique: a palavra.
    let word = PointF::new(title.x + title.width * 0.9 + 8.0, title.center().y + 8.0);
    ui.pointer_down(form, at(word.x, word.y));
    ui.pointer_up(form, at(word.x, word.y));
    ui.pointer_down(form, at(word.x, word.y));
    ui.pointer_up(form, at(word.x, word.y));
    assert_eq!(ui.get::<Markdown>(md).unwrap().selected_text(), "aylis");

    // O botão direito abre o menu; o clique nele copia.
    let right = PointerEvent { button: aylis_core::PointerButton::Secondary, ..at(word.x, word.y) };
    copied.borrow_mut().clear();
    ui.pointer_down(form, right);
    ui.pointer_up(form, right);
    ui.pointer_down(form, at(word.x + 10.0, word.y + 10.0));
    ui.pointer_up(form, at(word.x + 10.0, word.y + 10.0));
    assert_eq!(copied.borrow().as_str(), "aylis");

    // Ctrl+A: tudo, com a tabela em colunas separadas por tabulação.
    ui.key_down(form, ctrl('a'));
    ui.key_down(form, ctrl('c'));
    let all = copied.borrow().clone();
    assert!(all.starts_with("Manual do aylis\n\nO aylis desenha"), "{all}");
    assert!(all.contains("Lottie\taylis-vector\t120 KB"), "{all}");
    assert!(all.trim_end().ends_with("Fim."), "{all}");

    // O botão Copiar do bloco de código (o bloco aparece depois de rolar).
    let button = items.iter().find_map(|i| if let Item::Copy { rect, .. } = i { Some(*rect) } else { None }).unwrap();
    ui.wheel(form, WheelEvent { position: PointF::new(200.0, 200.0), delta: PointF::new(0.0, -(button.y - 40.0)), ..Default::default() });
    paint(&mut ui);
    let scroll = (button.y - 40.0).min(ui.get::<Markdown>(md).unwrap().content_height() - (520.0 - 16.0));
    let p = at(button.center().x + 8.0, button.center().y + 8.0 - scroll);
    ui.pointer_move(form, p);
    ui.pointer_down(form, p);
    ui.pointer_up(form, p);
    assert!(copied.borrow().starts_with("let md = ui.add(Markdown::new(\"# Olá\"), form);\nui.on("), "{}", copied.borrow());
}
