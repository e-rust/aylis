//! Um form em vários monitores (`span_monitors` do `Form`): com monitores em placas de vídeo
//! diferentes, uma janela que cobre todos faz o Windows copiar cada quadro de uma placa para a
//! outra (veja `docs/gpu-por-monitor.md`). Por isso o form vira **uma janela por monitor**, cada
//! uma na placa do seu, e o `Ui` continua com um form só:
//!
//! - [`split`] divide o retângulo do form na tela pelos monitores (os pedaços);
//! - cada janela pinta o seu pedaço com o deslocamento dele ([`Ui::paint_piece`]);
//! - a entrada de uma janela vira um ponto do form ([`Piece::to_form`]);
//! - os menus e as listas abrem dentro do pedaço em que nascem ([`Ui::popup_area`]).
//!
//! Tudo aqui é puro (sem sistema): a plataforma passa os retângulos dos monitores e as
//! janelas.

use crate::component::ComponentId;
use crate::geometry::{PointF, RectF};
use crate::ui::Ui;

/// Um retângulo da tela em pixels físicos (a área de trabalho, como o sistema dá).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ScreenRect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl ScreenRect {
    pub const fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self { x, y, width, height }
    }

    pub fn right(&self) -> i64 {
        self.x as i64 + self.width as i64
    }

    pub fn bottom(&self) -> i64 {
        self.y as i64 + self.height as i64
    }

    pub fn is_empty(&self) -> bool {
        self.width == 0 || self.height == 0
    }

    /// A parte comum (`None` se não se tocam).
    pub fn intersect(&self, other: ScreenRect) -> Option<ScreenRect> {
        let (x, y) = (self.x.max(other.x), self.y.max(other.y));
        let (r, b) = (self.right().min(other.right()), self.bottom().min(other.bottom()));
        (r > x as i64 && b > y as i64).then(|| ScreenRect::new(x, y, (r - x as i64) as u32, (b - y as i64) as u32))
    }

    /// `other` cabe inteiro neste.
    pub fn contains_rect(&self, other: ScreenRect) -> bool {
        other.x >= self.x && other.y >= self.y && other.right() <= self.right() && other.bottom() <= self.bottom()
    }

    /// O menor retângulo que contém todos (`None` sem nenhum): o form em tela cheia em todos
    /// os monitores.
    pub fn bounding(rects: &[ScreenRect]) -> Option<ScreenRect> {
        let rects: Vec<&ScreenRect> = rects.iter().filter(|r| !r.is_empty()).collect();
        let x = rects.iter().map(|r| r.x).min()?;
        let y = rects.iter().map(|r| r.y).min()?;
        let r = rects.iter().map(|r| r.right()).max()?;
        let b = rects.iter().map(|r| r.bottom()).max()?;
        Some(ScreenRect::new(x, y, (r - x as i64) as u32, (b - y as i64) as u32))
    }
}

/// Um pedaço do form: a parte que cai num monitor, mostrada por uma janela.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Piece {
    /// O monitor (o índice na lista passada a [`split`]).
    pub monitor: usize,
    /// Onde a janela fica na tela.
    pub screen: ScreenRect,
    /// Onde o pedaço começa dentro do form, em pixels físicos (o canto da janela menos o do
    /// form).
    pub origin_x: i32,
    pub origin_y: i32,
}

impl Piece {
    /// O pedaço nas coordenadas do form (unidades lógicas), com `scale` pixels por unidade
    /// (a escala do monitor vezes o zoom).
    pub fn rect_in_form(&self, scale: f64) -> RectF {
        let s = scale.max(0.01);
        RectF::new((self.origin_x as f64 / s) as f32, (self.origin_y as f64 / s) as f32, (self.screen.width as f64 / s) as f32, (self.screen.height as f64 / s) as f32)
    }

    /// Um ponto da janela do pedaço (pixels físicos a partir do canto dela, como o sistema
    /// manda o mouse e o toque) no form, em unidades lógicas. Fora da janela também vale (o
    /// mouse preso nela durante um arrasto que passou para o monitor vizinho).
    pub fn to_form(&self, x: f64, y: f64, scale: f64) -> PointF {
        let s = scale.max(0.01);
        PointF::new(((x + self.origin_x as f64) / s) as f32, ((y + self.origin_y as f64) / s) as f32)
    }
}

/// Divide o form (o retângulo dele na tela, em pixels físicos) pelos monitores: um pedaço por
/// monitor que ele toca, na ordem de cima para baixo e da esquerda para a direita. A parte do
/// form fora de todos os monitores não aparece. Um monitor espelhado (o mesmo retângulo de
/// outro, ou dentro dele) não ganha outra janela.
pub fn split(form: ScreenRect, monitors: &[ScreenRect]) -> Vec<Piece> {
    let mut pieces: Vec<Piece> = Vec::new();
    for (monitor, m) in monitors.iter().enumerate() {
        let Some(screen) = form.intersect(*m) else { continue };
        if pieces.iter().any(|p| p.screen.contains_rect(screen)) {
            continue;
        }
        pieces.push(Piece { monitor, screen, origin_x: screen.x - form.x, origin_y: screen.y - form.y });
    }
    pieces.sort_by_key(|p| (p.screen.y, p.screen.x));
    pieces
}

/// O pedaço maior (o que fica com a janela principal: o título, a barra de tarefas, o
/// leitor de tela). Empate: o primeiro.
pub fn largest(pieces: &[Piece]) -> Option<usize> {
    let area = |p: &Piece| p.screen.width as u64 * p.screen.height as u64;
    pieces.iter().enumerate().fold(None, |best: Option<(usize, u64)>, (i, p)| match best {
        Some((_, a)) if a >= area(p) => best,
        _ => Some((i, area(p))),
    }).map(|(i, _)| i)
}

impl Ui {
    /// Os pedaços do form em janelas separadas (nas coordenadas dele): a plataforma informa
    /// quando o form ocupa vários monitores (`span_monitors`). Vazio: uma janela só.
    pub fn form_pieces(&self, form: ComponentId) -> &[RectF] {
        self.form_pieces.get(&self.root_of(form)).map_or(&[], Vec::as_slice)
    }

    /// A plataforma informa os pedaços do form (vazio: voltou a uma janela).
    pub fn set_form_pieces(&mut self, form: ComponentId, pieces: Vec<RectF>) {
        if pieces.len() <= 1 {
            self.form_pieces.remove(&form);
        } else {
            self.form_pieces.insert(form, pieces);
        }
    }

    /// Onde um menu, uma lista ou um balão que nasce em `at` (nas coordenadas do form) pode
    /// ficar: o form inteiro ou, com ele em vários monitores, o pedaço que contém `at` (o
    /// mais perto, se `at` cai num vão entre eles). Assim ele não fica partido entre duas
    /// janelas.
    pub fn popup_area(&self, form: ComponentId, at: PointF) -> RectF {
        let root = self.root_of(form);
        let whole = self.control(root).map_or(RectF::new(0.0, 0.0, f32::MAX, f32::MAX), |cb| RectF::new(0.0, 0.0, cb.width(), cb.height()));
        let pieces = self.form_pieces(root);
        if let Some(p) = pieces.iter().find(|p| p.contains(at)) {
            return *p;
        }
        let distance = |r: &RectF| {
            let dx = (r.x - at.x).max(at.x - r.right()).max(0.0);
            let dy = (r.y - at.y).max(at.y - r.bottom()).max(0.0);
            dx * dx + dy * dy
        };
        pieces.iter().copied().min_by(|a, b| distance(a).total_cmp(&distance(b))).unwrap_or(whole)
    }

    /// Pinta o pedaço `piece` do form (nas coordenadas dele) numa janela que começa nele: o
    /// canvas anda `-piece.x, -piece.y`, e só os controles que tocam o pedaço são pintados.
    /// `areas`: o que repintar, nas coordenadas da janela (as que a plataforma passou ao
    /// renderizador); `None`, o pedaço inteiro. `start`: o primeiro pedaço do quadro (os
    /// pedidos de quadro, como o cursor piscando, valem somados para todos os pedaços até o
    /// próximo `start`).
    pub fn paint_piece(&mut self, root: ComponentId, canvas: &mut dyn crate::Canvas, piece: RectF, areas: Option<&[RectF]>, start: bool) {
        let in_form: Vec<RectF> = match areas {
            Some(a) => a.iter().filter_map(|r| r.offset(piece.x, piece.y).intersect(piece)).collect(),
            None => vec![piece],
        };
        if in_form.is_empty() {
            return;
        }
        canvas.save();
        canvas.translate(-piece.x, -piece.y);
        // O quadro inteiro (sem áreas) no primeiro pedaço: as camadas guardadas são refeitas,
        // como na pintura inteira de uma janela só.
        self.paint_regions_in(root, canvas, &in_form, start && areas.is_none());
        canvas.restore();
        let mine = self.frame_requests.borrow().get(&root).copied().unwrap_or((false, None));
        let merged = match self.piece_requests.get().filter(|_| !start) {
            Some((animating, wake)) => {
                let wake = [wake, mine.1].into_iter().flatten().reduce(f64::min);
                (animating || mine.0, wake)
            }
            None => mine,
        };
        self.piece_requests.set(Some(merged));
        self.frame_requests.borrow_mut().insert(root, merged);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const LEFT: ScreenRect = ScreenRect::new(0, 0, 1920, 1080);
    const RIGHT: ScreenRect = ScreenRect::new(1920, 0, 2560, 1440);

    #[test]
    fn a_form_inside_one_monitor_is_one_piece() {
        let pieces = split(ScreenRect::new(100, 100, 800, 600), &[LEFT, RIGHT]);
        assert_eq!(pieces, vec![Piece { monitor: 0, screen: ScreenRect::new(100, 100, 800, 600), origin_x: 0, origin_y: 0 }]);
    }

    #[test]
    fn a_form_across_two_monitors_is_split_at_the_border() {
        let pieces = split(ScreenRect::new(1500, 200, 1000, 500), &[LEFT, RIGHT]);
        assert_eq!(pieces.len(), 2);
        assert_eq!(pieces[0], Piece { monitor: 0, screen: ScreenRect::new(1500, 200, 420, 500), origin_x: 0, origin_y: 0 });
        assert_eq!(pieces[1], Piece { monitor: 1, screen: ScreenRect::new(1920, 200, 580, 500), origin_x: 420, origin_y: 0 });
        assert_eq!(largest(&pieces), Some(1));
    }

    #[test]
    fn the_monitor_on_the_left_has_negative_coordinates() {
        let left = ScreenRect::new(-1280, 100, 1280, 1024);
        let pieces = split(ScreenRect::new(-300, 300, 900, 400), &[LEFT, left]);
        assert_eq!(pieces.len(), 2);
        // Da esquerda para a direita: o monitor da esquerda primeiro.
        assert_eq!(pieces[0].monitor, 1);
        assert_eq!((pieces[0].origin_x, pieces[0].screen.x, pieces[0].screen.width), (0, -300, 300));
        assert_eq!(pieces[1].monitor, 0);
        assert_eq!((pieces[1].origin_x, pieces[1].screen.width), (300, 600));
    }

    #[test]
    fn a_video_wall_covers_every_monitor_and_skips_the_gaps() {
        // Seis monitores 2×3, e um sétimo menor embaixo do primeiro (o vão ao lado dele fica
        // sem janela).
        let mut monitors: Vec<ScreenRect> = (0..6).map(|i| ScreenRect::new((i % 3) * 1920, (i / 3) * 1080, 1920, 1080)).collect();
        monitors.push(ScreenRect::new(0, 2160, 1280, 720));
        let all = ScreenRect::bounding(&monitors).unwrap();
        assert_eq!(all, ScreenRect::new(0, 0, 5760, 2880));
        let pieces = split(all, &monitors);
        assert_eq!(pieces.len(), 7);
        let covered: u64 = pieces.iter().map(|p| p.screen.width as u64 * p.screen.height as u64).sum();
        assert_eq!(covered, 6 * 1920 * 1080 + 1280 * 720);
        // Na ordem de cima para baixo, da esquerda para a direita.
        assert_eq!(pieces.iter().map(|p| p.monitor).collect::<Vec<_>>(), vec![0, 1, 2, 3, 4, 5, 6]);
        assert_eq!((pieces[4].origin_x, pieces[4].origin_y), (1920, 1080));
    }

    #[test]
    fn a_mirrored_monitor_does_not_get_another_window() {
        let pieces = split(ScreenRect::new(1000, 0, 2000, 800), &[LEFT, LEFT, RIGHT]);
        assert_eq!(pieces.iter().map(|p| p.monitor).collect::<Vec<_>>(), vec![0, 2]);
    }

    #[test]
    fn the_input_of_a_window_becomes_a_point_in_the_form() {
        let pieces = split(ScreenRect::new(1500, 200, 1000, 500), &[LEFT, RIGHT]);
        // O canto da segunda janela, a 150%: 420 px do form são 280 unidades.
        assert_eq!(pieces[1].to_form(0.0, 0.0, 1.5), PointF::new(280.0, 0.0));
        assert_eq!(pieces[1].to_form(30.0, 15.0, 1.5), PointF::new(300.0, 10.0));
        // O mouse preso na primeira durante um arrasto, já sobre a segunda (x além da largura).
        assert_eq!(pieces[0].to_form(450.0, 0.0, 1.5), pieces[1].to_form(30.0, 0.0, 1.5));
        assert_eq!(pieces[1].rect_in_form(1.5), RectF::new(280.0, 0.0, 580.0 / 1.5, 500.0 / 1.5));
    }
}
