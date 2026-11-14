//! Pintar só a área que mudou (`Ui::damage`) tem de dar a mesma imagem que pintar tudo: o
//! controle que desenha fora da própria área (uma sombra, um anel de foco, um balão) deixaria
//! resto na tela. Cada controle registrado passa pelo mouse em cima e pelo foco, e a imagem
//! pintada por parte (por cima do quadro anterior) é comparada com a pintada inteira, no Skia
//! e, no Windows, no Direct2D.

use aylis_core::*;

const W: u32 = 1280;
const H: u32 = 960;
const CELL: (f32, f32) = (150.0, 70.0);

/// Um renderizador: pinta inteiro ou só uma área por cima da imagem anterior (pixels
/// `0x00RRGGBB`).
trait Painter {
    fn text_service(&self) -> Box<dyn TextService>;
    fn paint(&self, ui: &mut Ui, form: ComponentId, over: Option<(&[u32], RectF)>) -> Vec<u32>;
}

impl Painter for aylis_skia::SkiaRenderer {
    fn text_service(&self) -> Box<dyn TextService> {
        Box::new(aylis_skia::SkiaRenderer::text_service(self))
    }

    fn paint(&self, ui: &mut Ui, form: ComponentId, over: Option<(&[u32], RectF)>) -> Vec<u32> {
        let mut pixels = over.map_or_else(|| vec![0u32; (W * H) as usize], |(p, _)| p.to_vec());
        let area = over.map(|(_, a)| a);
        self.render_area_to_pixels(&mut pixels, W, H, 1.0, Color::WHITE, area, |c| ui.paint_region(form, c, area));
        pixels
    }
}

#[cfg(windows)]
impl Painter for aylis_d2d::D2dRenderer {
    fn text_service(&self) -> Box<dyn TextService> {
        Box::new(aylis_d2d::D2dRenderer::text_service(self))
    }

    fn paint(&self, ui: &mut Ui, form: ComponentId, over: Option<(&[u32], RectF)>) -> Vec<u32> {
        let previous: Option<Vec<u8>> = over.map(|(p, _)| p.iter().flat_map(|&x| [(x >> 16) as u8, (x >> 8) as u8, x as u8, 255]).collect());
        let area = over.map(|(_, a)| a);
        let rgba = self.render_area_rgba(W, H, 1.0, Color::WHITE, previous.as_deref().zip(area), |c| ui.paint_region(form, c, area)).expect("Direct2D");
        rgba.chunks_exact(4).map(|p| (p[0] as u32) << 16 | (p[1] as u32) << 8 | p[2] as u32).collect()
    }
}

struct Screen {
    ui: Ui,
    form: ComponentId,
    painter: Box<dyn Painter>,
}

impl Screen {
    fn full(&mut self) -> Vec<u32> {
        self.painter.paint(&mut self.ui, self.form, None)
    }

    /// Por parte, por cima de `previous`, e a área repintada; `None` se o `Ui` pediu a janela
    /// inteira.
    fn partial(&mut self, previous: &[u32]) -> Option<(Vec<u32>, RectF)> {
        let area = self.ui.take_damage(self.form)?;
        Some((self.painter.paint(&mut self.ui, self.form, Some((previous, area))), area))
    }
}

/// Quanto um pixel pode mudar sem contar como resto. Um desenho cortado pela borda da
/// superfície da pintura por parte passa por outro caminho de rasterização no Skia (as curvas
/// cortadas são refeitas em retas), e a beirada anti-aliasing dele muda alguns tons (até uns
/// 40, só nos pixels da beirada; o navegador, que repinta por partes, tem o mesmo efeito). O
/// corte fica 16 pixels fora da área (`SkiaRenderer::draw_region`); sem isso, um círculo
/// cortado pela área passava de 50 tons. Resto de verdade (o desenho velho que ficou, o que
/// passou da área) muda o pixel inteiro: mais de 200 tons nos defeitos que o teste já pegou.
const TOLERANCE: u32 = 48;

/// Os pixels que mudaram mais que [`TOLERANCE`] entre as duas imagens (quantos e o primeiro,
/// para a mensagem).
fn differences(a: &[u32], b: &[u32]) -> (usize, Option<(u32, u32)>) {
    let mut first = None;
    let count = a
        .iter()
        .zip(b)
        .enumerate()
        .filter(|(_, (x, y))| tones(**x, **y) > TOLERANCE)
        .inspect(|(i, _)| {
            first.get_or_insert(((*i as u32) % W, (*i as u32) / W));
        })
        .count();
    (count, first)
}

/// A maior diferença entre os canais de dois pixels.
fn tones(a: u32, b: u32) -> u32 {
    (0..3).map(|s| ((a >> (s * 8)) & 0xFF).abs_diff((b >> (s * 8)) & 0xFF)).max().unwrap_or(0)
}

#[test]
fn painting_only_the_damaged_area_gives_the_same_image() {
    compare(Box::new(aylis_skia::SkiaRenderer::new()), "Skia");
}

#[cfg(windows)]
#[test]
fn painting_only_the_damaged_area_gives_the_same_image_in_direct2d() {
    compare(Box::new(aylis_d2d::D2dRenderer::new().expect("Direct2D")), "Direct2D");
}

fn compare(painter: Box<dyn Painter>, name: &str) {
    compare_in(painter, name, TextMode::Native);
}

/// Também com o núcleo desenhando o texto (o modo `core` do item 10), com a fonte do projeto.
#[test]
fn painting_only_the_damaged_area_gives_the_same_image_with_the_core_text() {
    let font = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../resources/fonts/inter/Inter[opsz,wght].ttf");
    if let Ok(data) = std::fs::read(font) {
        aylis_core::fonts::register(data);
    }
    compare_in(Box::new(aylis_skia::SkiaRenderer::new()), "Skia (texto core)", TextMode::Core);
}

fn compare_in(painter: Box<dyn Painter>, name: &str, mode: TextMode) {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    ui.set_text_mode(mode);
    ui.services_mut().text = Some(painter.text_service());
    let form = ui.create("Form", None).unwrap();
    ui.set_prop(form, "width", W as f64).unwrap();
    ui.set_prop(form, "height", H as f64).unwrap();
    ui.show(form).unwrap();
    // Um de cada classe visual, numa grade.
    let classes: Vec<&'static str> = ui.registry().iter().map(|c| c.name).filter(|n| *n != "Form").collect();
    let mut controls = Vec::new();
    for class in classes {
        let Ok(id) = ui.create(class, Some(form)) else { continue };
        if ui.control(id).is_none() {
            continue;
        }
        let n = controls.len();
        let (col, row) = (n % 8, n / 8);
        if row >= 13 {
            ui.free(id);
            continue;
        }
        let _ = ui.set_prop(id, "align", "none");
        let _ = ui.set_prop(id, "left", 4.0 + col as f64 * CELL.0 as f64);
        let _ = ui.set_prop(id, "top", 4.0 + row as f64 * CELL.1 as f64);
        let _ = ui.set_prop(id, "width", (CELL.0 - 12.0) as f64);
        let _ = ui.set_prop(id, "height", (CELL.1 - 12.0) as f64);
        controls.push((class, id));
    }
    ui.set_time(1000.0);
    ui.layout();
    let mut screen = Screen { ui, form, painter };
    let mut previous = screen.full();
    let _ = screen.ui.take_damage(form); // o primeiro quadro é inteiro

    let mut failures = Vec::new();
    let mut compared = 0;
    let mut covered = 0.0;
    for &(class, id) in &controls {
        if !screen.ui.contains(id) {
            continue;
        }
        let Some(r) = screen.ui.absolute_rect(id) else { continue };
        let center = PointF::new(r.x + r.width / 2.0, r.y + r.height / 2.0);
        // O mouse em cima, o foco (o anel e o cursor de texto) e um deslocamento de animação
        // (ida e volta: o lugar antigo, dele e dos filhos, não pode ficar na tela).
        for step in ["hover", "focus", "slide", "back"] {
            match step {
                "hover" => screen.ui.pointer_move(form, PointerEvent { position: center, ..Default::default() }),
                "focus" => {
                    screen.ui.set_focus(id);
                }
                _ => {
                    let shift = if step == "slide" { PointF::new(15.0, 10.0) } else { PointF::ZERO };
                    screen.ui.control_mut(id).unwrap().set_translate(shift);
                    screen.ui.damage(id);
                }
            }
            let partial = screen.partial(&previous);
            let full = screen.full();
            let _ = screen.ui.take_damage(form);
            if let Some((partial, area)) = partial {
                compared += 1;
                covered += (area.width * area.height) as f64 / (W * H) as f64;
                if std::env::var("AYLIS_PARTIAL_AREAS").is_ok() {
                    eprintln!("{class} ({step}): {area:?}");
                }
                let (count, first) = differences(&partial, &full);
                if count > 0 {
                    let worst = partial.iter().zip(&full).map(|(a, b)| tones(*a, *b)).max().unwrap_or(0);
                    failures.push(format!("{class} ({step}): {count} pixels diferentes, o primeiro em {first:?}, até {worst} tons"));
                    // Para investigar: AYLIS_PARTIAL_DUMP=pasta grava as imagens da primeira falha
                    // (ou da classe em AYLIS_PARTIAL_CLASS).
                    let wanted = std::env::var("AYLIS_PARTIAL_CLASS").map_or(failures.len() == 1, |c| c == class);
                    if wanted {
                        if let Ok(dir) = std::env::var("AYLIS_PARTIAL_DUMP") {
                            let png = |p: &[u32]| {
                                let rgba: Vec<u8> = p.iter().flat_map(|&x| [(x >> 16) as u8, (x >> 8) as u8, x as u8, 255]).collect();
                                aylis_skia::encode_png(&rgba, W, H).unwrap()
                            };
                            let diff: Vec<u32> = partial.iter().zip(&full).map(|(a, b)| if a == b { 0x00FF_FFFF } else { 0x00FF_0000 }).collect();
                            let _ = std::fs::write(format!("{dir}/parcial.png"), png(&partial));
                            let _ = std::fs::write(format!("{dir}/inteira.png"), png(&full));
                            let _ = std::fs::write(format!("{dir}/anterior.png"), png(&previous));
                            let _ = std::fs::write(format!("{dir}/diferenca.png"), png(&diff));
                            let _ = std::fs::write(format!("{dir}/area.txt"), format!("{:?}", screen.ui.absolute_rect(id)));
                        }
                    }
                }
            }
            previous = full;
        }
    }
    let average = covered / compared.max(1) as f64 * 100.0;
    eprintln!("{name}: {compared} quadros por parte comparados, {} controles, área média {average:.1}% da janela", controls.len());
    // Cada passo muda um ou dois controles numa grade de 104. O mouse saindo de um controle
    // para o fundo do form não repinta o form: o ensaio do desenho dele (`paint_is_pure`) diz
    // que nada mudou (sem isso, a média ia a 17%; com, fica perto de 4%).
    assert!(average < 8.0, "{name}: a área repintada está grande demais ({average:.1}% da janela, em média)");
    assert!(compared > controls.len() / 2, "poucos quadros por parte ({compared}): o teste não está exercitando a pintura por área");
    assert!(failures.is_empty(), "{name}: pintar por parte deixou resto:\n{}", failures.join("\n"));
}

/// O mouse sai de um botão para o fundo do form (e de um painel sem estilo de hover): só o
/// botão é repintado, não o form inteiro.
#[test]
fn hovering_the_form_background_repaints_only_the_control_left() {
    let painter: Box<dyn Painter> = Box::new(aylis_skia::SkiaRenderer::new());
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    ui.services_mut().text = Some(painter.text_service());
    let form = ui.create("Form", None).unwrap();
    ui.set_prop(form, "width", W as f64).unwrap();
    ui.set_prop(form, "height", H as f64).unwrap();
    ui.show(form).unwrap();
    let panel = ui.create("Panel", Some(form)).unwrap();
    for (prop, value) in [("align", PropValue::from("none")), ("left", 600.0.into()), ("top", 400.0.into()), ("width", 300.0.into()), ("height", 200.0.into())] {
        ui.set_prop(panel, prop, value).unwrap();
    }
    let button = ui.create("Button", Some(form)).unwrap();
    for (prop, value) in [("align", PropValue::from("none")), ("left", 20.0.into()), ("top", 20.0.into()), ("width", 120.0.into()), ("height", 40.0.into())] {
        ui.set_prop(button, prop, value).unwrap();
    }
    ui.set_time(1000.0);
    ui.layout();
    let mut screen = Screen { ui, form, painter };
    let at = |x: f32, y: f32| PointerEvent { position: PointF::new(x, y), ..Default::default() };
    screen.ui.pointer_move(form, at(80.0, 40.0));
    let mut previous = screen.full();
    let _ = screen.ui.take_damage(form);
    screen.ui.set_time(1010.0);

    screen.ui.pointer_move(form, at(400.0, 300.0));
    let (partial, area) = screen.partial(&previous).expect("só uma parte da janela");
    assert!(area.width < 200.0 && area.height < 100.0, "repintou além do botão: {area:?}");
    let full = screen.full();
    assert_eq!(differences(&partial, &full).0, 0, "a pintura por parte deixou resto");
    // O fim da transição do botão (o tempo anda e ela termina).
    screen.ui.set_time(1020.0);
    previous = screen.full();
    let _ = screen.ui.take_damage(form);

    // Para dentro do painel e de volta ao fundo: nenhum dos dois muda de desenho.
    screen.ui.pointer_move(form, at(700.0, 500.0));
    screen.ui.pointer_move(form, at(400.0, 300.0));
    assert!(!screen.ui.needs_repaint(), "nada mudou de desenho: nem pede quadro");
    if let Some((partial, area)) = screen.partial(&previous) {
        assert!(area.width < 50.0 && area.height < 50.0, "repintou o painel ou o form: {area:?}");
        assert_eq!(differences(&partial, &screen.full()).0, 0);
    }
}

/// Um painel com `cache_layer` deslizando: os filhos não são pintados de novo a cada quadro
/// (a imagem guardada é recomposta no lugar novo), e a imagem por parte é a mesma da pintura
/// inteira. Um filho que muda refaz a camada.
#[test]
fn a_cached_layer_slides_without_repainting_its_children() {
    let mut painters: Vec<(Box<dyn Painter>, &str)> = vec![(Box::new(aylis_skia::SkiaRenderer::new()), "Skia")];
    #[cfg(windows)]
    painters.push((Box::new(aylis_d2d::D2dRenderer::new().expect("Direct2D")), "Direct2D"));
    for (painter, name) in painters {
        let mut ui = Ui::new();
        aylis_controls::register(&mut ui);
        ui.services_mut().text = Some(painter.text_service());
        let form = ui.create("Form", None).unwrap();
        ui.set_prop(form, "width", W as f64).unwrap();
        ui.set_prop(form, "height", H as f64).unwrap();
        ui.show(form).unwrap();
        let panel = ui.create("Panel", Some(form)).unwrap();
        for (prop, value) in [("align", PropValue::from("none")), ("left", 100.0.into()), ("top", 100.0.into()), ("width", 400.0.into()), ("height", 300.0.into()), ("cache_layer", true.into())] {
            ui.set_prop(panel, prop, value).unwrap();
        }
        let painted = std::rc::Rc::new(std::cell::Cell::new(0));
        let mut label = None;
        for i in 0..6 {
            let child = ui.create("Label", Some(panel)).unwrap();
            for (prop, value) in [("align", PropValue::from("top")), ("height", 30.0.into()), ("text", format!("Linha {i}").into())] {
                ui.set_prop(child, prop, value).unwrap();
            }
            let count = painted.clone();
            ui.on_paint(child, move |canvas, ctx, base| {
                count.set(count.get() + 1);
                base(canvas, ctx);
            });
            label.get_or_insert(child);
        }
        ui.set_time(1000.0);
        ui.layout();
        let mut screen = Screen { ui, form, painter };
        // Como a plataforma: a área do quadro antes de pintá-lo.
        let _ = screen.ui.take_damage(form);
        let mut previous = screen.full();
        for step in 1..=4 {
            screen.ui.set_prop(panel, "translate_x", (step * 25) as f64).unwrap();
            let before = painted.get();
            let (partial, area) = screen.partial(&previous).unwrap_or_else(|| panic!("{name}: só a área do painel"));
            // O Direct2D do teste cria um alvo novo por quadro (na janela, o cache é dela): lá
            // só a imagem é conferida.
            if name == "Skia" {
                assert_eq!(painted.get(), before, "{name}: os filhos foram pintados de novo (a camada não valeu)");
            }
            assert!(area.width < 600.0, "{name}: {area:?}");
            let full = screen.full();
            let _ = screen.ui.take_damage(form);
            assert_eq!(differences(&partial, &full).0, 0, "{name}: a camada deslocada saiu diferente da pintura inteira");
            previous = full;
        }
        // Um filho muda: a camada é refeita (o texto novo aparece).
        screen.ui.set_prop(label.unwrap(), "text", "Mudou").unwrap();
        let before = painted.get();
        let (partial, _) = screen.partial(&previous).unwrap_or_else(|| panic!("{name}: só a área do filho"));
        assert!(painted.get() > before, "{name}: a camada não foi refeita");
        assert_eq!(differences(&partial, &screen.full()).0, 0, "{name}: a camada refeita saiu diferente da pintura inteira");
    }
}

/// A troca de aba com transição (deslizar e esmaecer): as duas páginas andam como imagens
/// guardadas, sem repintar os filhos a cada quadro, e cada quadro por parte sai igual à
/// pintura inteira (antes, a transição repintava a janela inteira e as duas páginas, controle
/// por controle; `medicoes/transicoes.json`).
#[test]
fn a_tab_transition_moves_the_pages_without_repainting_their_children() {
    for transition in ["slide_left", "fade"] {
        let mut painters: Vec<(Box<dyn Painter>, &str)> = vec![(Box::new(aylis_skia::SkiaRenderer::new()), "Skia")];
        #[cfg(windows)]
        painters.push((Box::new(aylis_d2d::D2dRenderer::new().expect("Direct2D")), "Direct2D"));
        for (painter, name) in painters {
            let mut ui = Ui::new();
            aylis_controls::register(&mut ui);
            ui.services_mut().text = Some(painter.text_service());
            let form = ui.create("Form", None).unwrap();
            ui.set_prop(form, "width", W as f64).unwrap();
            ui.set_prop(form, "height", H as f64).unwrap();
            ui.show(form).unwrap();
            let tabs = ui.create("TabControl", Some(form)).unwrap();
            ui.set_prop(tabs, "align", "client").unwrap();
            ui.set_prop(tabs, "transition", transition).unwrap();
            let painted = std::rc::Rc::new(std::cell::Cell::new(0));
            for page in ["A", "B"] {
                let item = ui.create("TabItem", Some(tabs)).unwrap();
                ui.set_prop(item, "caption", format!("Página {page}")).unwrap();
                for i in 0..8 {
                    let child = ui.create(if i % 2 == 0 { "Button" } else { "Edit" }, Some(item)).unwrap();
                    for (prop, value) in [("align", PropValue::from("top")), ("height", 40.0.into()), ("text", format!("{page} {i}").into())] {
                        ui.set_prop(child, prop, value).unwrap();
                    }
                    let count = painted.clone();
                    ui.on_paint(child, move |canvas, ctx, base| {
                        count.set(count.get() + 1);
                        base(canvas, ctx);
                    });
                }
            }
            ui.set_time(1000.0);
            ui.layout();
            let mut screen = Screen { ui, form, painter };
            let _ = screen.ui.take_damage(form);
            let _ = screen.full();
            // A troca começa: o primeiro quadro é inteiro (a página nova aparece).
            screen.ui.set_prop(tabs, "tab_index", 1i64).unwrap();
            screen.ui.tick();
            let _ = screen.ui.take_damage(form);
            let mut previous = screen.full();
            let _ = screen.ui.take_damage(form);
            for step in 1..=20 {
                let t = screen.ui.time() + 1.0 / 60.0;
                screen.ui.set_time(t);
                screen.ui.tick();
                let before = painted.get();
                let running = screen.ui.is_animating();
                let Some((partial, _)) = screen.partial(&previous) else {
                    // O fim da transição (a página antiga some, o layout é refeito) pinta tudo.
                    assert!(!running, "{name} {transition}: quadro {step} da transição pediu a janela inteira");
                    previous = screen.full();
                    let _ = screen.ui.take_damage(form);
                    continue;
                };
                // O Direct2D do teste cria um alvo novo por quadro (na janela, o cache é dela):
                // lá só a imagem é conferida.
                if name == "Skia" && running && step > 1 {
                    assert_eq!(painted.get(), before, "{name} {transition}: quadro {step}, os filhos foram pintados de novo");
                }
                let full = screen.full();
                let _ = screen.ui.take_damage(form);
                let (count, first) = differences(&partial, &full);
                assert_eq!(count, 0, "{name} {transition}: quadro {step} por parte saiu diferente da pintura inteira (o primeiro em {first:?})");
                previous = full;
            }
            assert!(!screen.ui.is_animating(), "{name} {transition}: a transição não acabou");
        }
    }
}

/// Um painel que desliza por cima de uma página (com e sem `cache_layer`): a página de trás,
/// que o movimento descobre, vira camada guardada e não é repintada controle por controle a
/// cada quadro, e cada quadro por parte sai igual à pintura inteira (antes, 342 a 1488
/// controles por quadro; `medicoes/transicoes.json`).
#[test]
fn a_sliding_panel_does_not_repaint_the_page_behind_it() {
    for cache_panel in [true, false] {
        let mut painters: Vec<(Box<dyn Painter>, &str)> = vec![(Box::new(aylis_skia::SkiaRenderer::new()), "Skia")];
        #[cfg(windows)]
        painters.push((Box::new(aylis_d2d::D2dRenderer::new().expect("Direct2D")), "Direct2D"));
        for (painter, name) in painters {
            let mut ui = Ui::new();
            aylis_controls::register(&mut ui);
            ui.services_mut().text = Some(painter.text_service());
            let form = ui.create("Form", None).unwrap();
            ui.set_prop(form, "width", W as f64).unwrap();
            ui.set_prop(form, "height", H as f64).unwrap();
            ui.show(form).unwrap();
            let tabs = ui.create("TabControl", Some(form)).unwrap();
            ui.set_prop(tabs, "align", "client").unwrap();
            let page = ui.create("TabItem", Some(tabs)).unwrap();
            ui.set_prop(page, "caption", "Página").unwrap();
            let painted = std::rc::Rc::new(std::cell::Cell::new(0));
            for i in 0..12 {
                let child = ui.create(if i % 2 == 0 { "Button" } else { "Edit" }, Some(page)).unwrap();
                for (prop, value) in [("align", PropValue::from("top")), ("height", 40.0.into()), ("text", format!("Campo {i}").into())] {
                    ui.set_prop(child, prop, value).unwrap();
                }
                let count = painted.clone();
                ui.on_paint(child, move |canvas, ctx, base| {
                    count.set(count.get() + 1);
                    base(canvas, ctx);
                });
            }
            let panel = ui.create("Panel", Some(form)).unwrap();
            for (prop, value) in [("left", PropValue::from(0.0)), ("top", 60.0.into()), ("width", 400.0.into()), ("height", 500.0.into()), ("cache_layer", cache_panel.into())] {
                ui.set_prop(panel, prop, value).unwrap();
            }
            for i in 0..4 {
                let child = ui.create("Label", Some(panel)).unwrap();
                for (prop, value) in [("align", PropValue::from("top")), ("height", 30.0.into()), ("text", format!("Painel {i}").into())] {
                    ui.set_prop(child, prop, value).unwrap();
                }
            }
            ui.set_time(1000.0);
            ui.layout();
            let mut screen = Screen { ui, form, painter };
            let _ = screen.ui.take_damage(form);
            let _ = screen.full();
            // O movimento começa: o primeiro quadro é inteiro.
            screen.ui.animate(panel, "translate_x", 600.0, AnimationOptions::new(250.0, Easing::OutCubic)).unwrap();
            screen.ui.tick();
            let _ = screen.ui.take_damage(form);
            let mut previous = screen.full();
            let _ = screen.ui.take_damage(form);
            let label = format!("{name}, painel {}", if cache_panel { "com cache_layer" } else { "sem cache_layer" });
            for step in 1..=20 {
                let t = screen.ui.time() + 1.0 / 60.0;
                screen.ui.set_time(t);
                screen.ui.tick();
                let before = painted.get();
                let running = screen.ui.is_animating();
                let Some((partial, _)) = screen.partial(&previous) else {
                    assert!(!running, "{label}: quadro {step} do movimento pediu a janela inteira");
                    previous = screen.full();
                    let _ = screen.ui.take_damage(form);
                    continue;
                };
                // O primeiro quadro monta a camada da página; o Direct2D do teste cria um alvo
                // novo por quadro (na janela, o cache é dela): lá só a imagem é conferida.
                if name == "Skia" && running && step > 1 {
                    assert_eq!(painted.get(), before, "{label}: quadro {step}, a página de trás foi repintada");
                }
                let full = screen.full();
                let _ = screen.ui.take_damage(form);
                let (count, first) = differences(&partial, &full);
                assert_eq!(count, 0, "{label}: quadro {step} por parte saiu diferente da pintura inteira (o primeiro em {first:?})");
                previous = full;
            }
            assert!(!screen.ui.is_animating(), "{label}: o movimento não acabou");
        }
    }
}

/// O fundo atrás do painel que muda a cada quadro (um campo da página trocando de texto): a
/// camada seria remontada com a página inteira em todo quadro, então depois de duas ele volta
/// a ser pintado por parte (só o campo que mudou), e a imagem continua igual à inteira.
#[test]
fn a_backdrop_that_keeps_changing_is_painted_by_parts_again() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let painter: Box<dyn Painter> = Box::new(aylis_skia::SkiaRenderer::new());
    ui.services_mut().text = Some(painter.text_service());
    let form = ui.create("Form", None).unwrap();
    ui.set_prop(form, "width", W as f64).unwrap();
    ui.set_prop(form, "height", H as f64).unwrap();
    ui.show(form).unwrap();
    let tabs = ui.create("TabControl", Some(form)).unwrap();
    ui.set_prop(tabs, "align", "client").unwrap();
    let page = ui.create("TabItem", Some(tabs)).unwrap();
    let painted = std::rc::Rc::new(std::cell::Cell::new(0));
    let mut fields = Vec::new();
    for i in 0..12 {
        let child = ui.create("Label", Some(page)).unwrap();
        for (prop, value) in [("align", PropValue::from("top")), ("height", 40.0.into()), ("text", format!("Campo {i}").into())] {
            ui.set_prop(child, prop, value).unwrap();
        }
        let count = painted.clone();
        ui.on_paint(child, move |canvas, ctx, base| {
            count.set(count.get() + 1);
            base(canvas, ctx);
        });
        fields.push(child);
    }
    let panel = ui.create("Panel", Some(form)).unwrap();
    for (prop, value) in [("left", PropValue::from(0.0)), ("top", 600.0.into()), ("width", 400.0.into()), ("height", 300.0.into()), ("cache_layer", true.into())] {
        ui.set_prop(panel, prop, value).unwrap();
    }
    ui.set_time(1000.0);
    ui.layout();
    let mut screen = Screen { ui, form, painter };
    let _ = screen.ui.take_damage(form);
    let _ = screen.full();
    screen.ui.animate(panel, "translate_x", 600.0, AnimationOptions::new(250.0, Easing::OutCubic)).unwrap();
    screen.ui.tick();
    let _ = screen.ui.take_damage(form);
    let mut previous = screen.full();
    let _ = screen.ui.take_damage(form);
    for step in 1..=12 {
        let t = screen.ui.time() + 1.0 / 60.0;
        screen.ui.set_time(t);
        screen.ui.tick();
        // O último campo (os campos ficam acima do painel) muda a cada quadro.
        screen.ui.set_prop(fields[11], "text", format!("Mudou {step}")).unwrap();
        let before = painted.get();
        let Some((partial, _)) = screen.partial(&previous) else { panic!("quadro {step} pediu a janela inteira") };
        if step > 3 {
            // O campo que mudou e os vizinhos pela margem do recorte; a camada pintaria os 12.
            assert!(painted.get() - before <= 3, "quadro {step}: a página inteira foi repintada ({} campos)", painted.get() - before);
        }
        let full = screen.full();
        let _ = screen.ui.take_damage(form);
        let (count, first) = differences(&partial, &full);
        assert_eq!(count, 0, "quadro {step} por parte saiu diferente da pintura inteira (o primeiro em {first:?})");
        previous = full;
    }
}

/// Rolar um Memo com a roda: o texto já pintado é copiado (só a faixa nova e a barra são
/// pintadas), e a imagem sai igual à pintura inteira.
#[test]
fn scrolling_a_memo_copies_the_pixels_already_painted() {
    let renderer = aylis_skia::SkiaRenderer::new();
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    ui.services_mut().text = Some(Box::new(renderer.text_service()));
    let form = ui.create("Form", None).unwrap();
    ui.set_prop(form, "width", W as f64).unwrap();
    ui.set_prop(form, "height", H as f64).unwrap();
    ui.show(form).unwrap();
    let memo = ui.create("Memo", Some(form)).unwrap();
    let text: Vec<String> = (0..400).map(|i| format!("Linha {i} do texto comprido")).collect();
    for (prop, value) in [("align", PropValue::from("none")), ("left", 40.0.into()), ("top", 40.0.into()), ("width", 400.0.into()), ("height", 600.0.into()), ("text", text.join("\n").into())] {
        ui.set_prop(memo, prop, value).unwrap();
    }
    ui.set_time(1000.0);
    ui.layout();
    let mut pixels = vec![0u32; (W * H) as usize];
    let full = |ui: &mut Ui, pixels: &mut Vec<u32>| {
        let _ = ui.take_frame(form);
        renderer.render_area_to_pixels(pixels, W, H, 1.0, Color::WHITE, None, |c| ui.paint_region(form, c, None));
    };
    full(&mut ui, &mut pixels);
    let at = PointF::new(240.0, 340.0);
    let mut copied = 0;
    for step in 0..6 {
        let delta = if step < 4 { -40.0 } else { 40.0 };
        ui.wheel(form, WheelEvent { position: at, delta: PointF::new(0.0, delta), ..Default::default() });
        let frame = ui.take_frame(form).snapped(1.0);
        let FrameDamage::Parts { moves, areas } = frame else { panic!("rolar não pode repintar a janela inteira") };
        copied += moves.len();
        let painted: f32 = areas.iter().map(|a| a.width * a.height).sum();
        assert!(painted < 400.0 * 600.0 * 0.4, "pintou demais ao rolar: {areas:?}");
        let mut partial = pixels.clone();
        renderer.render_parts_to_pixels(&mut partial, W, H, 1.0, Color::WHITE, &moves, &areas, |c, a| ui.paint_regions(form, c, a));
        full(&mut ui, &mut pixels);
        let (count, first) = differences(&partial, &pixels);
        assert_eq!(count, 0, "passo {step}: a cópia deixou resto, o primeiro em {first:?}");
    }
    assert!(copied >= 5, "as rolagens não viraram cópia ({copied})");
}

/// Girar a roda além do fim do Memo (a altura à vista sem número inteiro de linhas): a imagem
/// por partes segue igual à inteira, e no fim nada é copiado nem deslocado. A roda calculava
/// o fim com as linhas inteiras à vista, passava dele, a cópia era pedida e a pintura voltava
/// a rolagem: a imagem ficava deslocada, com cópias cortadas da última linha.
#[test]
fn the_wheel_past_the_end_of_a_memo_copies_nothing() {
    let renderer = aylis_skia::SkiaRenderer::new();
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    ui.services_mut().text = Some(Box::new(renderer.text_service()));
    let form = ui.create("Form", None).unwrap();
    ui.set_prop(form, "width", W as f64).unwrap();
    ui.set_prop(form, "height", H as f64).unwrap();
    ui.show(form).unwrap();
    let memo = ui.create("Memo", Some(form)).unwrap();
    let text: Vec<String> = (0..120).map(|i| format!("Linha {i} do texto, L")).collect();
    for (prop, value) in [("align", PropValue::from("none")), ("left", 40.0.into()), ("top", 40.0.into()), ("width", 400.0.into()), ("height", 603.0.into()), ("text", text.join("\n").into())] {
        ui.set_prop(memo, prop, value).unwrap();
    }
    ui.set_time(1000.0);
    ui.layout();
    let mut pixels = vec![0u32; (W * H) as usize];
    let full = |ui: &mut Ui, pixels: &mut Vec<u32>| {
        let _ = ui.take_frame(form);
        renderer.render_area_to_pixels(pixels, W, H, 1.0, Color::WHITE, None, |c| ui.paint_region(form, c, None));
    };
    full(&mut ui, &mut pixels);
    let at = PointF::new(240.0, 340.0);
    let mut idle = 0;
    for step in 0..40 {
        ui.wheel(form, WheelEvent { position: at, delta: PointF::new(0.0, -40.0), ..Default::default() });
        if !ui.needs_repaint() {
            idle += 1;
            continue;
        }
        let frame = ui.take_frame(form).snapped(1.0);
        let FrameDamage::Parts { moves, areas } = frame else { panic!("passo {step}: rolar não pode repintar a janela inteira") };
        assert_eq!(idle, 0, "passo {step}: andou de novo depois de parar no fim ({moves:?})");
        let mut partial = pixels.clone();
        renderer.render_parts_to_pixels(&mut partial, W, H, 1.0, Color::WHITE, &moves, &areas, |c, a| ui.paint_regions(form, c, a));
        full(&mut ui, &mut pixels);
        let (count, first) = differences(&partial, &pixels);
        assert_eq!(count, 0, "passo {step}: a cópia deixou resto, o primeiro em {first:?}");
    }
    assert!(idle >= 5, "a roda não chegou ao fim do texto ({idle} passos parados)");
}

/// O mesmo no Direct2D: a cópia e as várias áreas pintadas de uma vez (o recorte pela união
/// delas) dão a imagem da pintura inteira.
#[cfg(windows)]
#[test]
fn scrolling_a_memo_by_parts_in_direct2d_gives_the_full_image() {
    let renderer = aylis_d2d::D2dRenderer::new().expect("Direct2D");
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    ui.services_mut().text = Some(Box::new(renderer.text_service()));
    let form = ui.create("Form", None).unwrap();
    ui.set_prop(form, "width", W as f64).unwrap();
    ui.set_prop(form, "height", H as f64).unwrap();
    ui.show(form).unwrap();
    let memo = ui.create("Memo", Some(form)).unwrap();
    let text: Vec<String> = (0..400).map(|i| format!("Linha {i} do texto comprido")).collect();
    for (prop, value) in [("align", PropValue::from("none")), ("left", 40.0.into()), ("top", 40.0.into()), ("width", 400.0.into()), ("height", 600.0.into()), ("text", text.join("\n").into())] {
        ui.set_prop(memo, prop, value).unwrap();
    }
    ui.set_time(1000.0);
    ui.layout();
    let full = |ui: &mut Ui| {
        let _ = ui.take_frame(form);
        renderer.render_area_rgba(W, H, 1.0, Color::WHITE, None, |c| ui.paint_region(form, c, None)).expect("Direct2D")
    };
    let mut previous = full(&mut ui);
    for step in 0..4 {
        let delta = if step < 3 { -40.0 } else { 40.0 };
        ui.wheel(form, WheelEvent { position: PointF::new(240.0, 340.0), delta: PointF::new(0.0, delta), ..Default::default() });
        let FrameDamage::Parts { moves, areas } = ui.take_frame(form).snapped(1.0) else { panic!("rolar não pode repintar a janela inteira") };
        assert!(!moves.is_empty() && areas.len() > 1, "passo {step}: sem cópia ou com uma área só");
        let partial = renderer.render_parts_rgba(W, H, 1.0, Color::WHITE, &previous, &moves, &areas, |c, a| ui.paint_regions(form, c, a)).expect("Direct2D");
        let whole = full(&mut ui);
        let worst = partial.iter().zip(&whole).map(|(a, b)| a.abs_diff(*b) as u32).max().unwrap_or(0);
        let count = partial.chunks_exact(4).zip(whole.chunks_exact(4)).filter(|(a, b)| a.iter().zip(b.iter()).any(|(x, y)| x.abs_diff(*y) as u32 > TOLERANCE)).count();
        assert_eq!(count, 0, "passo {step}: {count} pixels diferentes (até {worst} tons)");
        previous = whole;
    }
}

/// O polegar (um círculo) de um TrackBar cortado pela borda da área: o Skia corta as curvas no
/// recorte e as refaz em retas, e a beirada saía até uns 100 tons diferente da pintura
/// inteira, até 4 pixels para dentro (com o Popover e o DateTimeEdit no começo da grade, o
/// TrackBar e o Switch caíam bem na borda da área do vizinho). A pintura por parte vai para uma
/// superfície com margem, e só a área volta.
#[test]
fn a_circle_cut_by_the_area_edge_paints_like_the_full_frame() {
    for painter in painters() {
        let mut ui = Ui::new();
        aylis_controls::register(&mut ui);
        ui.services_mut().text = Some(painter.text_service());
        let form = ui.create("Form", None).unwrap();
        ui.set_prop(form, "width", W as f64).unwrap();
        ui.set_prop(form, "height", H as f64).unwrap();
        ui.show(form).unwrap();
        let track = ui.create("TrackBar", Some(form)).unwrap();
        for (prop, value) in [("align", PropValue::from("none")), ("left", 154.0.into()), ("top", 494.0.into()), ("width", 138.0.into()), ("height", 58.0.into())] {
            ui.set_prop(track, prop, value).unwrap();
        }
        ui.set_time(1000.0);
        ui.layout();
        let mut screen = Screen { ui, form, painter };
        let full = screen.full();
        let mut worst = 0;
        // A borda direita da área passando pelo polegar, pixel a pixel (e a de baixo, idem).
        for cut in 0..24 {
            for area in [RectF::new(0.0, 349.0, 154.0 + cut as f32, 209.25), RectF::new(100.0, 300.0, 300.0, 210.0 + cut as f32)] {
                let partial = screen.painter.paint(&mut screen.ui, form, Some((&full, area)));
                worst = worst.max(partial.iter().zip(&full).map(|(a, b)| tones(*a, *b)).max().unwrap_or(0));
            }
        }
        assert!(worst <= 16, "a beirada do polegar mudou {worst} tons na pintura por parte");
    }
}

fn painters() -> Vec<Box<dyn Painter>> {
    #[allow(unused_mut)]
    let mut list: Vec<Box<dyn Painter>> = vec![Box::new(aylis_skia::SkiaRenderer::new())];
    #[cfg(windows)]
    list.push(Box::new(aylis_d2d::D2dRenderer::new().expect("Direct2D")));
    list
}

/// O efeito de fim da rolagem (decidido em 14/11): o dedo puxa um `ScrollBox` além do topo, e
/// cada quadro por parte do brilho, do esticar e da mola sai igual à pintura inteira; ao
/// soltar, o puxão volta sozinho e a tela para de pedir quadros, igual a antes de puxar.
#[test]
fn the_end_of_scroll_effect_paints_by_parts_and_goes_back() {
    let touch = |y: f32| PointerEvent { position: PointF::new(200.0, y), kind: PointerKind::Touch, pointer_id: 3, ..Default::default() };
    for style in [Overscroll::Glow, Overscroll::Stretch, Overscroll::Bounce] {
        let mut painters: Vec<(Box<dyn Painter>, &str)> = vec![(Box::new(aylis_skia::SkiaRenderer::new()), "Skia")];
        #[cfg(windows)]
        painters.push((Box::new(aylis_d2d::D2dRenderer::new().expect("Direct2D")), "Direct2D"));
        for (painter, name) in painters {
            let mut ui = Ui::new();
            aylis_controls::register(&mut ui);
            ui.services_mut().text = Some(painter.text_service());
            ui.set_overscroll_style(style);
            let form = ui.create("Form", None).unwrap();
            ui.set_prop(form, "width", W as f64).unwrap();
            ui.set_prop(form, "height", H as f64).unwrap();
            ui.show(form).unwrap();
            let scroll = ui.create("ScrollBox", Some(form)).unwrap();
            ui.set_prop(scroll, "align", "client").unwrap();
            for i in 0..30 {
                let child = ui.create("Button", Some(scroll)).unwrap();
                for (prop, value) in [("align", PropValue::from("top")), ("height", 40.0.into()), ("text", format!("Linha {i}").into())] {
                    ui.set_prop(child, prop, value).unwrap();
                }
            }
            ui.set_time(1000.0);
            ui.layout();
            let mut screen = Screen { ui, form, painter };
            let _ = screen.ui.take_damage(form);
            let before = screen.full();
            let _ = screen.ui.take_damage(form);
            let mut previous = before.clone();
            let label = format!("{name}, {style:?}");
            // O dedo desce no topo: o conteúdo já está no começo, então abre o puxão.
            screen.ui.pointer_down(form, touch(100.0));
            let mut frame = |screen: &mut Screen, previous: &mut Vec<u32>, step: &str| {
                screen.ui.tick();
                let Some((partial, _)) = screen.partial(previous) else {
                    // Soltar com o dedo andando começa a inércia, que pede a janela inteira.
                    assert!(step.starts_with("voltando"), "{label} {step}: pediu a janela inteira");
                    *previous = screen.full();
                    let _ = screen.ui.take_damage(form);
                    return;
                };
                let full = screen.full();
                let _ = screen.ui.take_damage(form);
                let (count, first) = differences(&partial, &full);
                assert_eq!(count, 0, "{label} {step}: por parte saiu diferente da inteira (o primeiro em {first:?})");
                *previous = full;
            };
            for (i, y) in [130.0, 170.0, 220.0, 280.0].into_iter().enumerate() {
                let t = screen.ui.time() + 1.0 / 60.0;
                screen.ui.set_time(t);
                screen.ui.pointer_move(form, touch(y));
                frame(&mut screen, &mut previous, &format!("puxando {i}"));
            }
            assert_ne!(differences(&previous, &before).0, 0, "{label}: o puxão aparece");
            screen.ui.pointer_up(form, touch(280.0));
            for i in 0..90 {
                if !screen.ui.is_animating() {
                    break;
                }
                let t = screen.ui.time() + 1.0 / 60.0;
                screen.ui.set_time(t);
                frame(&mut screen, &mut previous, &format!("voltando {i}"));
            }
            assert!(!screen.ui.is_animating(), "{label}: o puxão não voltou");
            let end = screen.full();
            assert_eq!(differences(&end, &before).0, 0, "{label}: voltou ao que era");
        }
    }
}
