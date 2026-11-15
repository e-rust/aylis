//! O texto "só letras" que cresce enquanto se digita (o editor de print do NoOBS, itens 2 e
//! 3): o `Memo` e o `Edit` com `auto_size` e `plain`, e o Memo sem fundo sobre uma imagem
//! colorida, pintado no Skia e (no Windows) no Direct2D.
//!
//! A imagem é comparada com `tests/visual/plain_memo-<renderizador>-<sistema>.png`, com a
//! mesma tolerância do teste visual da IDE. Sem a referência (outro sistema), o teste grava
//! a imagem e passa: confira e faça o commit dela. Mudou de propósito: `AYLIS_UPDATE_VISUAL=1`
//! regrava. Uma falha grava a nova em `plain_memo/` dentro do `CARGO_TARGET_TMPDIR`.

use std::path::PathBuf;

use aylis_controls::*;
use aylis_core::*;

const W: u32 = 360;
const H: u32 = 120;
/// Onde a anotação é criada (o clique do usuário no print).
const AT: (f32, f32) = (40.0, 30.0);
const LINE1: &str = "Anotação";
const LINE2: &str = "segunda linha, bem mais comprida";

const TONES: u8 = 48;
const SHARE: f64 = 0.005;

/// Um print colorido: a metade esquerda escura (azul para roxo), a direita clara (amarelo
/// para verde-água).
fn picture() -> Image {
    let mut px = Vec::with_capacity((W * H * 4) as usize);
    for y in 0..H {
        for x in 0..W {
            let (t, v) = (x as f32 / W as f32, y as f32 / H as f32);
            let (r, g, b) = if x < W / 2 {
                (20.0 + 140.0 * t, 25.0 + 30.0 * v, 80.0 + 50.0 * v)
            } else {
                (255.0 - 120.0 * (t - 0.5), 230.0 - 20.0 * v, 110.0 + 110.0 * v)
            };
            px.extend([r as u8, g as u8, b as u8, 255]);
        }
    }
    Image::rgba(W, H, px)
}

struct Scene {
    ui: Ui,
    form: ComponentId,
    memo: Option<ComponentId>,
}

/// O print num form do tamanho dele e, com `memo`, a anotação criada em [`AT`] com as duas
/// linhas digitadas (a segunda depois do Shift+Enter).
fn scene(text: Box<dyn TextService>, memo: bool) -> Scene {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    ui.services_mut().text = Some(text);
    let form = ui.add_root(Form::new("print", W as f32, H as f32));
    let _ = ui.set_prop(form, "padding", 0i64);
    ui.show(form).unwrap();
    let mut view = ImageView::default();
    view.fit = ImageFit::Stretch;
    view.set_picture(Some(picture()));
    let image = ui.add(view, form);
    ui.control_mut(image).unwrap().set_align(Align::Client);
    let memo = memo.then(|| {
        let m = ui.add(Memo::default(), form);
        ui.control_mut(m).unwrap().set_rect(AT.0, AT.1, 10.0, 10.0).set_auto_size(true);
        ui.set_prop(m, "plain", true).unwrap();
        // O cursor sem piscar: a imagem não depende do relógio.
        ui.get_mut::<Memo>(m).unwrap().caret_blink = 0;
        ui.set_focus(m);
        ui.text_input(form, LINE1);
        ui.key_down(form, KeyEvent { key: Key::Enter, modifiers: Modifiers { shift: true, ..Default::default() }, ..Default::default() });
        ui.text_input(form, LINE2);
        m
    });
    ui.layout();
    Scene { ui, form, memo }
}

impl Scene {
    fn font(&self) -> FontSpec {
        self.ui.font_of(self.memo.expect("com a anotação"))
    }

    fn line_height(&self) -> f32 {
        (self.font().size * 1.45).round().max(10.0)
    }

    /// A largura de `text` na fonte da anotação.
    fn width(&self, text: &str) -> f32 {
        self.ui.measure_text(text, &TextStyle { font: self.font(), ..Default::default() }).width
    }

    fn select(&mut self, start: usize, end: usize) {
        let memo = self.memo.unwrap();
        self.ui.get_mut::<Memo>(memo).unwrap().select(start, end);
        self.ui.layout();
    }
}

/// Um renderizador: a exportação (a imagem fora da tela, como o `render_png`) e o caminho da
/// janela (o quadro pintado por cima do anterior), em RGBA.
trait Painter {
    fn name(&self) -> &'static str;
    fn text_service(&self) -> Box<dyn TextService>;
    fn export(&self, ui: &mut Ui, form: ComponentId) -> Vec<u8>;
    fn screen(&self, ui: &mut Ui, form: ComponentId) -> Vec<u8>;
}

impl Painter for aylis_skia::SkiaRenderer {
    fn name(&self) -> &'static str {
        "skia"
    }

    fn text_service(&self) -> Box<dyn TextService> {
        Box::new(aylis_skia::SkiaRenderer::text_service(self))
    }

    fn export(&self, ui: &mut Ui, form: ComponentId) -> Vec<u8> {
        let background = ui.theme().color("background");
        let png = self.render_png(W, H, 1.0, background, |c| ui.paint(form, c)).expect("PNG");
        aylis_skia::decode_rgba(&png).expect("PNG lido").0
    }

    fn screen(&self, ui: &mut Ui, form: ComponentId) -> Vec<u8> {
        let background = ui.theme().color("background");
        let mut pixels = vec![0u32; (W * H) as usize];
        assert!(self.render_to_pixels(&mut pixels, W, H, 1.0, background, |c| ui.paint(form, c)));
        pixels.iter().flat_map(|&p| [(p >> 16) as u8, (p >> 8) as u8, p as u8, 255]).collect()
    }
}

#[cfg(windows)]
impl Painter for aylis_d2d::D2dRenderer {
    fn name(&self) -> &'static str {
        "d2d"
    }

    fn text_service(&self) -> Box<dyn TextService> {
        Box::new(aylis_d2d::D2dRenderer::text_service(self))
    }

    fn export(&self, ui: &mut Ui, form: ComponentId) -> Vec<u8> {
        let background = ui.theme().color("background");
        self.render_rgba(W, H, 1.0, background, |c| ui.paint(form, c)).expect("Direct2D")
    }

    fn screen(&self, ui: &mut Ui, form: ComponentId) -> Vec<u8> {
        // A janela repinta por cima do quadro anterior (aqui, um quadro vazio).
        let background = ui.theme().color("background");
        let previous = vec![0u8; (W * H * 4) as usize];
        let area = RectF::new(0.0, 0.0, W as f32, H as f32);
        self.render_area_rgba(W, H, 1.0, background, Some((&previous, area)), |c| ui.paint_region(form, c, Some(area))).expect("Direct2D")
    }
}

fn px(rgba: &[u8], x: f32, y: f32) -> [i32; 3] {
    let i = ((y as u32).min(H - 1) * W + (x as u32).min(W - 1)) as usize * 4;
    [rgba[i] as i32, rgba[i + 1] as i32, rgba[i + 2] as i32]
}

fn diff(a: [i32; 3], b: [i32; 3]) -> i32 {
    (0..3).map(|i| (a[i] - b[i]).abs()).max().unwrap()
}

fn luma(c: [i32; 3]) -> f32 {
    0.2126 * c[0] as f32 + 0.7152 * c[1] as f32 + 0.0722 * c[2] as f32
}

/// Compara com a referência (ou grava, se não houver ou se pedirem).
fn check_reference(name: &str, rgba: &[u8]) {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/visual").join(format!("{name}-{}.png", std::env::consts::OS));
    let png = aylis_skia::encode_png(rgba, W, H).expect("PNG");
    if std::env::var_os("AYLIS_UPDATE_VISUAL").is_some() || !path.exists() {
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(&path, &png).unwrap();
        eprintln!("{name}: referência gravada em {} (confira e faça o commit)", path.display());
        return;
    }
    let (expected, ew, eh) = aylis_skia::decode_rgba(&std::fs::read(&path).unwrap()).expect("referência");
    let failed = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("plain_memo");
    let save = || {
        std::fs::create_dir_all(&failed).unwrap();
        std::fs::write(failed.join(format!("{name}.png")), &png).unwrap();
    };
    if (ew, eh) != (W, H) {
        save();
        panic!("{name}: a referência tem {ew}x{eh} (a nova está em {})", failed.display());
    }
    let changed = rgba.chunks_exact(4).zip(expected.chunks_exact(4)).filter(|(a, b)| a.iter().zip(b.iter()).take(3).any(|(x, y)| x.abs_diff(*y) > TONES)).count();
    let share = changed as f64 / (W * H) as f64;
    if share > SHARE {
        save();
        panic!("{name}: {:.2}% dos pixels mudaram (limite {:.1}%); a nova está em {}", share * 100.0, SHARE * 100.0, failed.join(format!("{name}.png")).display());
    }
}

/// O Memo sem fundo, com texto selecionado, sobre o print: o print aparece onde não há letra,
/// a seleção é translúcida, o cursor aparece no escuro e no claro, e a tela e a exportação
/// saem com os mesmos pixels (o mesmo antialiasing do texto).
fn memo_over_picture(painter: &dyn Painter) {
    let name = painter.name();
    let bare = {
        let mut s = scene(painter.text_service(), false);
        painter.export(&mut s.ui, s.form)
    };
    let mut s = scene(painter.text_service(), true);
    let memo = s.memo.unwrap();
    let lh = s.line_height();
    let (w1, w2) = (s.width(LINE1), s.width(LINE2));
    let mid = |line: usize| AT.1 + line as f32 * lh + lh / 2.0;

    // Seleção do meio da primeira linha ao meio da segunda; o cursor no fim dela.
    let n1 = LINE1.chars().count();
    s.select(n1 / 2, n1 + 1 + 6);
    let shot = painter.export(&mut s.ui, s.form);
    assert_eq!(shot, painter.screen(&mut s.ui, s.form), "{name}: a tela e a exportação têm de sair iguais (o mesmo antialiasing)");
    check_reference(&format!("plain_memo-{name}"), &shot);

    // O fundo aparece: à direita do fim da primeira linha (depois da faixa da seleção, que
    // passa 6 pontos do fim), dentro da caixa, o print está intacto.
    let bounds = s.ui.bounds(memo).unwrap();
    assert!(bounds.right() > AT.0 + w1 + 20.0, "{name}: a segunda linha alarga a caixa");
    let clear = (AT.0 + w1 + 14.0, mid(0));
    assert!(diff(px(&shot, clear.0, clear.1), px(&bare, clear.0, clear.1)) <= 2, "{name}: sem fundo, o print aparece na caixa");

    // A seleção é translúcida: na faixa depois do fim da primeira linha (sem letra), o print
    // tingido pela cor da seleção, nem o print puro nem a cor sólida.
    let selection = s.ui.theme().color("selection");
    let tinted = (AT.0 + w1 + 3.0, mid(0));
    let (under, got) = (px(&bare, tinted.0, tinted.1), px(&shot, tinted.0, tinted.1));
    let a = selection.a as f32 / 255.0;
    assert!(a < 0.9, "{name}: a seleção do tema já é translúcida ({selection:?})");
    let blend = [selection.r, selection.g, selection.b].map(|c| c as f32 * a);
    let expected = [0, 1, 2].map(|i| (blend[i] + under[i] as f32 * (1.0 - a)).round() as i32);
    assert!(diff(got, expected) <= 4, "{name}: seleção {got:?}, esperado {expected:?} (o print {under:?} por baixo)");
    assert!(diff(got, under) > 8, "{name}: a seleção aparece por cima do print");
    // Num tema de seleção opaca (o Windows 7), ela também fica translúcida sobre o print, e
    // o texto selecionado fica na cor dele (sem o branco do `selection_text`).
    for theme in ["Windows 7", "Win32"] {
        let mut t = scene(painter.text_service(), true);
        t.ui.set_theme(aylis_core::themes::builtin(theme).expect("tema").clone());
        t.ui.layout();
        t.select(n1 / 2, n1 + 1 + 6);
        let shot = painter.export(&mut t.ui, t.form);
        let x = AT.0 + t.width(LINE1) + 3.0;
        let bare = {
            let mut b = scene(painter.text_service(), false);
            b.ui.set_theme(aylis_core::themes::builtin(theme).unwrap().clone());
            painter.export(&mut b.ui, b.form)
        };
        let solid = t.ui.theme().color("selection");
        let (got, under) = (px(&shot, x, mid(0)), px(&bare, x, mid(0)));
        assert!(diff(got, [solid.r as i32, solid.g as i32, solid.b as i32]) > 8 && diff(got, under) > 8, "{name}, {theme}: seleção {got:?} sobre {under:?}, translúcida");
    }

    // O cursor aparece no escuro (fim da primeira linha, na metade escura) e no claro (fim
    // da segunda, na metade clara): a coluna dele contrasta com o print.
    for (line, end, half) in [(0usize, w1, "escura"), (1, w2, "clara")] {
        let caret = if line == 0 { n1 } else { n1 + 1 + LINE2.chars().count() };
        s.select(caret, caret);
        let shot = painter.export(&mut s.ui, s.form);
        let x = AT.0 + end;
        assert_eq!(x < W as f32 / 2.0, line == 0, "{name}: a linha {line} termina na metade {half}");
        let contrast = (-1..=2)
            .flat_map(|dx| (-3..=3).map(move |dy| (x + dx as f32, mid(line) + dy as f32)))
            .map(|(x, y)| (luma(px(&shot, x, y)) - luma(px(&bare, x, y))).abs())
            .fold(0.0f32, f32::max);
        assert!(contrast > 80.0, "{name}: o cursor na metade {half} contrasta {contrast:.0} tons com o print");
    }
}

#[test]
fn a_plain_memo_over_a_picture_in_skia() {
    memo_over_picture(&aylis_skia::SkiaRenderer::new());
}

#[cfg(windows)]
#[test]
fn a_plain_memo_over_a_picture_in_direct2d() {
    memo_over_picture(&aylis_d2d::D2dRenderer::new().expect("Direct2D"));
}

/// O Memo com `auto_size` acompanha o texto: cresce para a direita e para baixo a partir de
/// onde foi criado (Shift+Enter, várias linhas), encolhe ao apagar, e não rola nem quebra.
#[test]
fn an_auto_sized_memo_follows_the_text() {
    let renderer = aylis_skia::SkiaRenderer::new();
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    ui.services_mut().text = Some(Box::new(renderer.text_service()));
    let form = ui.add_root(Form::new("print", 800.0, 600.0));
    ui.show(form).unwrap();
    let memo = ui.add(Memo::default(), form);
    ui.control_mut(memo).unwrap().set_rect(100.0, 50.0, 240.0, 120.0).set_auto_size(true);
    ui.set_prop(memo, "plain", true).unwrap();
    ui.set_focus(memo);
    ui.layout();
    let font = ui.font_of(memo);
    let lh = (font.size * 1.45).round().max(10.0);
    let measure = |ui: &Ui, s: &str| ui.measure_text(s, &TextStyle { font: font.clone(), ..Default::default() }).width;
    let room = 3.0; // o cursor (1,5 arredondado para cima) e um pixel
    // Vazio: o lugar do cursor e uma linha.
    let b = ui.bounds(memo).unwrap();
    assert_eq!((b.x, b.y, b.width, b.height), (100.0, 50.0, room, lh));

    let typed = |ui: &mut Ui, text: &str| {
        ui.text_input(form, text);
        ui.update_layout();
        ui.bounds(memo).unwrap()
    };
    let b = typed(&mut ui, "Olá");
    assert_eq!((b.x, b.y), (100.0, 50.0), "cresce a partir de onde foi criado");
    assert_eq!(b.width, (measure(&ui, "Olá") + room).ceil());
    let b = typed(&mut ui, ", mundo");
    assert_eq!(b.width, (measure(&ui, "Olá, mundo") + room).ceil(), "para a direita");
    ui.key_down(form, KeyEvent { key: Key::Enter, modifiers: Modifiers { shift: true, ..Default::default() }, ..Default::default() });
    let b = typed(&mut ui, "ok");
    assert_eq!(b.height, 2.0 * lh, "para baixo (Shift+Enter)");
    assert_eq!(b.width, (measure(&ui, "Olá, mundo") + room).ceil(), "a linha mais larga manda");
    // Apagar a linha mais larga encolhe a caixa.
    ui.key_down(form, KeyEvent { key: Key::Up, ..Default::default() });
    ui.key_down(form, KeyEvent { key: Key::End, ..Default::default() });
    for _ in 0..7 {
        ui.key_down(form, KeyEvent { key: Key::Backspace, ..Default::default() });
    }
    ui.update_layout();
    let b = ui.bounds(memo).unwrap();
    assert_eq!(b.width, (measure(&ui, "Olá").max(measure(&ui, "ok")) + room).ceil(), "apagar encolhe");
    assert_eq!(ui.get::<Memo>(memo).unwrap().text, "Olá\nok");
    // O máximo vale (e o resto rola, sem quebrar).
    ui.control_mut(memo).unwrap().set_max_width(60.0);
    let b = typed(&mut ui, " — e uma frase bem comprida");
    assert_eq!(b.width, 60.0);
    // Com `wrap`, a largura fica e a altura acompanha as linhas quebradas.
    ui.set_prop(memo, "wrap", true).unwrap();
    ui.update_layout();
    assert!(ui.bounds(memo).unwrap().height > 2.0 * lh, "quebrada na largura, mais linhas");
}

/// O Edit com `auto_size` fica da largura do texto (com o padding, se não for `plain`).
#[test]
fn an_auto_sized_edit_follows_the_text() {
    let renderer = aylis_skia::SkiaRenderer::new();
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    ui.services_mut().text = Some(Box::new(renderer.text_service()));
    let form = ui.add_root(Form::new("t", 800.0, 600.0));
    ui.show(form).unwrap();
    let edit = ui.add(Edit::default(), form);
    ui.control_mut(edit).unwrap().set_rect(10.0, 10.0, 200.0, 32.0).set_auto_size(true);
    ui.set_focus(edit);
    ui.text_input(form, "abc");
    ui.update_layout();
    let font = ui.font_of(edit);
    let width = |ui: &Ui, s: &str| ui.measure_text(s, &TextStyle { font: font.clone(), ..Default::default() }).width;
    let pad = ui.style_padding(edit).horizontal();
    assert_eq!(ui.bounds(edit).unwrap().width, (width(&ui, "abc") + 3.0 + pad).ceil());
    ui.text_input(form, "defgh");
    ui.update_layout();
    assert_eq!(ui.bounds(edit).unwrap().width, (width(&ui, "abcdefgh") + 3.0 + pad).ceil());
    // `plain`: sem o padding.
    ui.set_prop(edit, "plain", true).unwrap();
    ui.update_layout();
    assert_eq!(ui.bounds(edit).unwrap().width, (width(&ui, "abcdefgh") + 3.0).ceil());
    assert_eq!(ui.bounds(edit).unwrap().height, (font.size * 1.45).ceil());
}
