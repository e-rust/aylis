//! `PaintCanvas`: uma imagem para desenhar com o mouse, como a área do Paint do Windows.
//!
//! - **Ferramentas** (`tool`): `pencil` (lápis, quadrado, sem suavizar), `brush` (pincel
//!   redondo), `eraser` (borracha: pinta com `back_color`), `line`, `rectangle`, `ellipse`
//!   (as formas: arrastar mostra a prévia, soltar desenha), `fill` (balde: pinta a área da
//!   mesma cor), `picker` (conta-gotas: pega a cor do ponto) e `select` (seleção).
//! - **Cores:** o botão esquerdo usa `color`, o direito `back_color` (as "Cor 1" e "Cor 2"
//!   do Paint). Com `filled`, retângulo e elipse são preenchidos com `back_color` e
//!   contornados com `color`.
//! - **Tamanho:** `brush_size` (1 a 64 pixels); `zoom` aumenta a imagem na tela.
//! - **Eventos:** `on_change` (depois de cada traço), `on_pick` (o conta-gotas achou uma
//!   cor: `"#RRGGBB"`, e `"2"` no fim se foi com o botão direito), `on_cursor` (a posição do
//!   mouse na imagem, `"120, 45"`, ou `""` ao sair).
//! - **Métodos** (`Ui::invoke`, ou `invoke` pelo protocolo, iguais em todas as linguagens):
//!   `undo`, `redo`, `clear`, `new` (largura, altura), `resize` (largura, altura), `open` (arquivo), `save` (arquivo:
//!   PNG, JPEG ou BMP pela extensão), `can_undo`, `can_redo`, `size` (`"800 × 600"`) e
//!   `checksum` (um resumo dos pixels, para comparar imagens).
//! - **Tamanho da imagem:** as alças na borda (direita, embaixo, canto) arrastam o tamanho
//!   novo, como no Paint; o que cresce fica com a cor 2 e o que sobra é cortado.
//! - **Seleção** (`select`): puxar um retângulo seleciona; arrastar por dentro ou as setas
//!   movem (Shift+seta: 10 pixels), Delete apaga com a cor 2, Esc ou Enter soltam. Métodos
//!   `select_all`, `copy`, `cut`, `paste`, `delete`, `deselect` e `has_selection`
//!   (os atalhos Ctrl+A, Ctrl+C, Ctrl+X e Ctrl+V do app chamam esses).
//!
//! A imagem fica em pixels RGBA na memória; desenhar é feito aqui mesmo (não depende do
//! renderizador), então o resultado é igual em todas as plataformas.

use std::cell::RefCell;

use aylis_core::{
    Canvas, Color, Component, ControlBase, CursorShape, EventArgs, EventCtx, Image, Key, KeyEvent, PaintCtx, PointF, PointerButton, PointerEvent, Ui,
    MethodInfo, PropEnum, PropKind, PropValue, RectF, Stroke, properties,
};

/// A ferramenta do [`PaintCanvas`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PaintTool {
    #[default]
    Pencil,
    Brush,
    Eraser,
    Line,
    Rectangle,
    Ellipse,
    Fill,
    Picker,
    Select,
}

impl PropEnum for PaintTool {
    const NAMES: &'static [&'static str] = &["pencil", "brush", "eraser", "line", "rectangle", "ellipse", "fill", "picker", "select"];
    fn from_index(i: usize) -> Option<Self> {
        use PaintTool::*;
        [Pencil, Brush, Eraser, Line, Rectangle, Ellipse, Fill, Picker, Select].get(i).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

impl PaintTool {
    fn is_shape(self) -> bool {
        matches!(self, PaintTool::Line | PaintTool::Rectangle | PaintTool::Ellipse)
    }
}

/// Os pixels da imagem (RGBA, sem pré-multiplicar).
#[derive(Clone, Debug, PartialEq)]
pub struct Bitmap {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>,
}

impl Bitmap {
    pub fn new(width: u32, height: u32, color: Color) -> Self {
        let (width, height) = (width.clamp(1, 8192), height.clamp(1, 8192));
        let mut pixels = Vec::with_capacity((width * height * 4) as usize);
        for _ in 0..width * height {
            pixels.extend_from_slice(&[color.r, color.g, color.b, color.a]);
        }
        Self { width, height, pixels }
    }

    pub fn get(&self, x: i32, y: i32) -> Option<Color> {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return None;
        }
        let i = ((y as u32 * self.width + x as u32) * 4) as usize;
        Some(Color::rgba(self.pixels[i], self.pixels[i + 1], self.pixels[i + 2], self.pixels[i + 3]))
    }

    pub fn set(&mut self, x: i32, y: i32, c: Color) {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return;
        }
        let i = ((y as u32 * self.width + x as u32) * 4) as usize;
        self.pixels[i..i + 4].copy_from_slice(&[c.r, c.g, c.b, c.a]);
    }

    /// Um "carimbo" do tamanho do pincel centrado em (x, y): redondo ou quadrado.
    fn stamp(&mut self, x: i32, y: i32, size: i32, round: bool, c: Color) {
        let size = size.max(1);
        let start = -(size - 1) / 2;
        let r = size as f32 / 2.0;
        for dy in start..start + size {
            for dx in start..start + size {
                if round && size > 2 {
                    // Centro do pixel dentro do círculo.
                    let (fx, fy) = (dx as f32 - start as f32 + 0.5 - r, dy as f32 - start as f32 + 0.5 - r);
                    if fx * fx + fy * fy > r * r {
                        continue;
                    }
                }
                self.set(x + dx, y + dy, c);
            }
        }
    }

    /// Uma linha de carimbos de (x0, y0) a (x1, y1) (Bresenham).
    fn line(&mut self, (x0, y0): (i32, i32), (x1, y1): (i32, i32), size: i32, round: bool, c: Color) {
        let (dx, dy) = ((x1 - x0).abs(), -(y1 - y0).abs());
        let (sx, sy) = (if x0 < x1 { 1 } else { -1 }, if y0 < y1 { 1 } else { -1 });
        let (mut x, mut y, mut err) = (x0, y0, dx + dy);
        loop {
            self.stamp(x, y, size, round, c);
            if x == x1 && y == y1 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }

    fn rectangle(&mut self, a: (i32, i32), b: (i32, i32), size: i32, stroke: Color, fill: Option<Color>) {
        let (x0, x1) = (a.0.min(b.0), a.0.max(b.0));
        let (y0, y1) = (a.1.min(b.1), a.1.max(b.1));
        if let Some(f) = fill {
            for y in y0..=y1 {
                for x in x0..=x1 {
                    self.set(x, y, f);
                }
            }
        }
        self.line((x0, y0), (x1, y0), size, false, stroke);
        self.line((x1, y0), (x1, y1), size, false, stroke);
        self.line((x1, y1), (x0, y1), size, false, stroke);
        self.line((x0, y1), (x0, y0), size, false, stroke);
    }

    fn ellipse(&mut self, a: (i32, i32), b: (i32, i32), size: i32, stroke: Color, fill: Option<Color>) {
        let (x0, x1) = (a.0.min(b.0) as f32, a.0.max(b.0) as f32);
        let (y0, y1) = (a.1.min(b.1) as f32, a.1.max(b.1) as f32);
        let (cx, cy, rx, ry) = ((x0 + x1) / 2.0, (y0 + y1) / 2.0, (x1 - x0) / 2.0, (y1 - y0) / 2.0);
        if let Some(f) = fill {
            for y in y0 as i32..=y1 as i32 {
                let t = if ry > 0.0 { (y as f32 - cy) / ry } else { 0.0 };
                if t.abs() > 1.0 {
                    continue;
                }
                let half = rx * (1.0 - t * t).sqrt();
                for x in (cx - half).round() as i32..=(cx + half).round() as i32 {
                    self.set(x, y, f);
                }
            }
        }
        // O contorno: pontos ao longo da volta, ligados por linhas.
        let steps = ((rx + ry) * 4.0).max(8.0) as i32;
        let point = |i: i32| {
            let a = i as f32 / steps as f32 * std::f32::consts::TAU;
            ((cx + rx * a.cos()).round() as i32, (cy + ry * a.sin()).round() as i32)
        };
        let mut last = point(0);
        for i in 1..=steps {
            let p = point(i);
            self.line(last, p, size, true, stroke);
            last = p;
        }
    }

    /// O balde: pinta a região ligada da mesma cor de (x, y). Devolve `false` se nada mudou.
    fn flood(&mut self, x: i32, y: i32, c: Color) -> bool {
        let Some(target) = self.get(x, y) else { return false };
        if target == c {
            return false;
        }
        let mut stack = vec![(x, y)];
        while let Some((x, y)) = stack.pop() {
            if self.get(x, y) != Some(target) {
                continue;
            }
            // Anda para os lados na linha e empilha as de cima e de baixo.
            let mut left = x;
            while self.get(left - 1, y) == Some(target) {
                left -= 1;
            }
            let mut right = x;
            while self.get(right + 1, y) == Some(target) {
                right += 1;
            }
            for xx in left..=right {
                self.set(xx, y, c);
                for ny in [y - 1, y + 1] {
                    if self.get(xx, ny) == Some(target) {
                        stack.push((xx, ny));
                    }
                }
            }
        }
        true
    }
}

const UNDO_LIMIT: usize = 20;

/// A seleção retangular (ferramenta `select`): a área e, depois de mexida, os pixels soltos
/// por cima da imagem (o lugar de onde saíram já ficou com a cor 2, como no Paint).
#[derive(Clone, Debug)]
struct Selection {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    floating: Option<Bitmap>,
}

impl Selection {
    fn contains(&self, p: (i32, i32)) -> bool {
        p.0 >= self.x && p.1 >= self.y && p.0 < self.x + self.w && p.1 < self.y + self.h
    }
}

impl Bitmap {
    /// Os pixels RGBA do retângulo `(x, y, w, h)`, que precisa estar dentro da imagem.
    fn rows(&self, x: u32, y: u32, w: u32, h: u32) -> Vec<u8> {
        let mut out = Vec::with_capacity(w as usize * h as usize * 4);
        for yy in y..y + h {
            let start = (yy as usize * self.width as usize + x as usize) * 4;
            out.extend_from_slice(&self.pixels[start..start + w as usize * 4]);
        }
        out
    }

    /// O pedaço `(x, y, w, h)` (o que passar da borda fica transparente).
    fn crop(&self, x: i32, y: i32, w: i32, h: i32) -> Bitmap {
        let mut out = Bitmap::new(w.max(1) as u32, h.max(1) as u32, Color::rgba(0, 0, 0, 0));
        for yy in 0..h {
            for xx in 0..w {
                if let Some(c) = self.get(x + xx, y + yy) {
                    out.set(xx, yy, c);
                }
            }
        }
        out
    }

    /// Cola `other` com o canto em (x, y) (os pixels transparentes não cobrem).
    fn stamp_bitmap(&mut self, other: &Bitmap, x: i32, y: i32) {
        for yy in 0..other.height as i32 {
            for xx in 0..other.width as i32 {
                if let Some(c) = other.get(xx, yy).filter(|c| c.a > 0) {
                    self.set(x + xx, y + yy, c);
                }
            }
        }
    }

    fn fill_rect(&mut self, x: i32, y: i32, w: i32, h: i32, c: Color) {
        for yy in y..y + h {
            for xx in x..x + w {
                self.set(xx, yy, c);
            }
        }
    }
}

/// Uma imagem para desenhar com o mouse (o Paint). Veja o [módulo](self).
pub struct PaintCanvas {
    pub base: ControlBase,
    pub tool: PaintTool,
    /// Cor 1 (botão esquerdo).
    pub color: Color,
    /// Cor 2 (botão direito, borracha e preenchimento das formas).
    pub back_color: Color,
    pub brush_size: i64,
    /// Retângulo e elipse preenchidos com `back_color`.
    pub filled: bool,
    pub zoom: f32,
    bitmap: Bitmap,
    undo: Vec<Bitmap>,
    redo: Vec<Bitmap>,
    /// O traço em andamento: o botão e o último ponto (ou o início da forma).
    drag: Option<(PointerButton, (i32, i32), (i32, i32))>,
    /// Os dois últimos pontos do traço de mão livre (em pixels da imagem, com fração): o
    /// traço passa por curvas entre eles, sem as quinas de quando o mouse anda rápido.
    stroke: Vec<(f32, f32)>,
    /// A imagem em blocos de [`TILE`] pixels, para a tela: um traço refaz só os blocos que
    /// tocou (refazer a imagem inteira a cada movimento deixava uma pintura grande lenta,
    /// com poucos pontos do mouse). `None` é bloco a refazer.
    tiles: RefCell<Vec<Option<Image>>>,
    selection: Option<Selection>,
    /// Arrastando a seleção: onde ela foi pega (em relação ao canto dela).
    grab: Option<(i32, i32)>,
    /// Arrastando uma alça da borda: qual e o tamanho novo (em pixels da imagem).
    resizing: Option<(Edge, (u32, u32))>,
}

/// As alças da borda da imagem, como no Paint: à direita, embaixo e no canto.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Edge {
    Right,
    Bottom,
    Corner,
}

/// O lado dos blocos da imagem na tela. Cada bloco leva um pixel a mais à direita e embaixo
/// (o começo do vizinho), para não aparecer emenda com zoom.
const TILE: u32 = 256;

/// O lado das alças e o espaço que sobra à direita e embaixo da imagem para elas.
const HANDLE: f32 = 7.0;
const HANDLE_MARGIN: f32 = 12.0;

impl Default for PaintCanvas {
    fn default() -> Self {
        let mut base = ControlBase::new(640.0, 400.0);
        base.set_cursor(CursorShape::Crosshair).set_can_focus(true);
        let mut c = Self {
            base,
            tool: PaintTool::Pencil,
            color: Color::rgb(0, 0, 0),
            back_color: Color::rgb(255, 255, 255),
            brush_size: 3,
            filled: false,
            zoom: 1.0,
            bitmap: Bitmap::new(640, 400, Color::rgb(255, 255, 255)),
            undo: Vec::new(),
            redo: Vec::new(),
            drag: None,
            stroke: Vec::new(),
            tiles: RefCell::new(Vec::new()),
            selection: None,
            grab: None,
            resizing: None,
        };
        c.fit();
        c
    }
}

impl PaintCanvas {
    pub fn bitmap(&self) -> &Bitmap {
        &self.bitmap
    }

    /// O controle do tamanho da imagem (vezes o zoom), mais o espaço das alças da borda.
    fn fit(&mut self) {
        let z = self.zoom.clamp(0.1, 16.0);
        self.base.set_width(self.bitmap.width as f32 * z + HANDLE_MARGIN).set_height(self.bitmap.height as f32 * z + HANDLE_MARGIN);
    }

    /// A imagem na tela (sem o espaço das alças).
    fn image_rect(&self) -> RectF {
        let z = self.zoom.clamp(0.1, 16.0);
        RectF::new(0.0, 0.0, self.bitmap.width as f32 * z, self.bitmap.height as f32 * z)
    }

    /// Onde fica cada alça (nas coordenadas do controle).
    fn handles(&self) -> [(Edge, RectF); 3] {
        let r = self.image_rect();
        let (w, h) = (r.width, r.height);
        [
            (Edge::Right, RectF::new(w + 1.0, h / 2.0 - HANDLE / 2.0, HANDLE, HANDLE)),
            (Edge::Bottom, RectF::new(w / 2.0 - HANDLE / 2.0, h + 1.0, HANDLE, HANDLE)),
            (Edge::Corner, RectF::new(w + 1.0, h + 1.0, HANDLE, HANDLE)),
        ]
    }

    /// A alça sob o ponto (com uma folga para o dedo e o mouse).
    fn edge_at(&self, p: PointF) -> Option<Edge> {
        self.handles().into_iter().find(|(_, r)| r.inflate(4.0).contains(p)).map(|(e, _)| e)
    }

    /// Muda o tamanho da imagem, como as alças da borda do Paint: o que sobra é cortado e o
    /// que cresce fica com a cor 2. Dá para desfazer.
    pub fn resize_image(&mut self, width: u32, height: u32) {
        let (width, height) = (width.clamp(1, 8192), height.clamp(1, 8192));
        if (width, height) == (self.bitmap.width, self.bitmap.height) {
            return;
        }
        self.commit_selection();
        self.checkpoint();
        let mut resized = Bitmap::new(width, height, self.back_color);
        resized.stamp_bitmap(&self.bitmap, 0, 0);
        self.bitmap = resized;
        self.fit();
        self.changed();
    }

    fn changed(&mut self) {
        self.tiles.borrow_mut().clear();
    }

    /// Mudaram só os pixels de `(x0, y0)` a `(x1, y1)` (inclusive): refaz os blocos deles.
    fn changed_area(&mut self, x0: i32, y0: i32, x1: i32, y1: i32) {
        let (cols, rows) = self.tile_grid();
        let mut tiles = self.tiles.borrow_mut();
        if tiles.len() != (cols * rows) as usize {
            tiles.clear();
            return;
        }
        // Um pixel no começo de um bloco também está na sobra do bloco anterior.
        let t = TILE as i32;
        let (cx0, cy0) = (((x0 - 1).max(0) / t) as u32, ((y0 - 1).max(0) / t) as u32);
        let (cx1, cy1) = ((x1.max(0) / t) as u32, (y1.max(0) / t) as u32);
        for ty in cy0..=cy1.min(rows.saturating_sub(1)) {
            for tx in cx0..=cx1.min(cols.saturating_sub(1)) {
                tiles[(ty * cols + tx) as usize] = None;
            }
        }
    }

    fn tile_grid(&self) -> (u32, u32) {
        (self.bitmap.width.div_ceil(TILE), self.bitmap.height.div_ceil(TILE))
    }

    fn checkpoint(&mut self) {
        self.undo.push(self.bitmap.clone());
        if self.undo.len() > UNDO_LIMIT {
            self.undo.remove(0);
        }
        self.redo.clear();
    }

    fn to_image(&self, p: PointF) -> (i32, i32) {
        let z = self.zoom.clamp(0.1, 16.0);
        ((p.x / z).floor() as i32, (p.y / z).floor() as i32)
    }

    fn colors(&self, button: PointerButton) -> (Color, Color) {
        if button == PointerButton::Secondary { (self.back_color, self.color) } else { (self.color, self.back_color) }
    }

    fn size(&self) -> i32 {
        self.brush_size.clamp(1, 64) as i32
    }

    /// Desenha o traço de `from` a `to` com a ferramenta de mão livre.
    fn freehand(&mut self, button: PointerButton, from: (i32, i32), to: (i32, i32)) {
        let (fore, _) = self.colors(button);
        let size = self.size();
        match self.tool {
            PaintTool::Pencil => self.bitmap.line(from, to, size, false, fore),
            PaintTool::Brush => self.bitmap.line(from, to, size, true, fore),
            // A borracha é quadrada e maior, como no Paint.
            PaintTool::Eraser => self.bitmap.line(from, to, size * 2 + 2, false, self.back_color),
            _ => {}
        }
        let r = size * 2 + 4;
        self.changed_area(from.0.min(to.0) - r, from.1.min(to.1) - r, from.0.max(to.0) + r, from.1.max(to.1) + r);
    }

    /// O traço de mão livre por uma curva de `from` a `to` puxada por `control`, em
    /// pedaços de uns 2 pixels.
    fn freehand_curve(&mut self, button: PointerButton, from: (f32, f32), control: (f32, f32), to: (f32, f32)) {
        let dist = |a: (f32, f32), b: (f32, f32)| ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt();
        let steps = ((dist(from, control) + dist(control, to)) / 2.0).ceil().clamp(1.0, 2000.0) as usize;
        let pixel = |p: (f32, f32)| (p.0.floor() as i32, p.1.floor() as i32);
        let mut last = pixel(from);
        for i in 1..=steps {
            let t = i as f32 / steps as f32;
            let u = 1.0 - t;
            let p = (
                u * u * from.0 + 2.0 * u * t * control.0 + t * t * to.0,
                u * u * from.1 + 2.0 * u * t * control.1 + t * t * to.1,
            );
            let p = pixel(p);
            if p != last {
                self.freehand(button, last, p);
                last = p;
            }
        }
    }

    /// Mais um ponto do traço de mão livre: a curva vai do meio do segmento anterior ao meio
    /// do novo, puxada pelo ponto do meio (assim os pedaços se emendam sem quinas).
    fn stroke_to(&mut self, button: PointerButton, p: (f32, f32)) {
        let mid = |a: (f32, f32), b: (f32, f32)| ((a.0 + b.0) / 2.0, (a.1 + b.1) / 2.0);
        match self.stroke[..] {
            [a, b] => self.freehand_curve(button, mid(a, b), b, mid(b, p)),
            [b] => self.freehand_curve(button, b, b, mid(b, p)),
            _ => {}
        }
        if self.stroke.len() == 2 {
            self.stroke.remove(0);
        }
        self.stroke.push(p);
    }

    /// Fim do traço: do meio do último segmento até o último ponto.
    fn stroke_end(&mut self, button: PointerButton) {
        if let [a, b] = self.stroke[..] {
            let m = ((a.0 + b.0) / 2.0, (a.1 + b.1) / 2.0);
            self.freehand_curve(button, m, b, b);
        }
        self.stroke.clear();
    }

    fn to_image_f(&self, p: PointF) -> (f32, f32) {
        let z = self.zoom.clamp(0.1, 16.0);
        (p.x / z, p.y / z)
    }

    fn shape(&mut self, bitmap: &mut Bitmap, button: PointerButton, a: (i32, i32), b: (i32, i32)) {
        let (fore, back) = self.colors(button);
        let fill = self.filled.then_some(back);
        match self.tool {
            PaintTool::Line => bitmap.line(a, b, self.size(), true, fore),
            PaintTool::Rectangle => bitmap.rectangle(a, b, self.size(), fore, fill),
            PaintTool::Ellipse => bitmap.ellipse(a, b, self.size(), fore, fill),
            _ => {}
        }
    }

    // ----- seleção -----

    pub fn has_selection(&self) -> bool {
        self.selection.is_some()
    }

    /// A imagem como aparece (com a seleção solta por cima): o que se grava.
    fn composited(&self) -> std::borrow::Cow<'_, Bitmap> {
        match &self.selection {
            Some(Selection { x, y, floating: Some(f), .. }) => {
                let mut b = self.bitmap.clone();
                b.stamp_bitmap(f, *x, *y);
                std::borrow::Cow::Owned(b)
            }
            _ => std::borrow::Cow::Borrowed(&self.bitmap),
        }
    }

    /// Solta a seleção na imagem (Enter, Esc, clicar fora, trocar de ferramenta).
    pub fn commit_selection(&mut self) {
        if let Some(Selection { x, y, floating: Some(f), .. }) = self.selection.take() {
            self.bitmap.stamp_bitmap(&f, x, y);
            self.changed();
        }
    }

    /// Descola os pixels da área (a primeira vez que a seleção mexe): o lugar fica com a
    /// cor 2, e dá para desfazer.
    fn lift(&mut self) {
        let Some(sel) = &self.selection else { return };
        if sel.floating.is_some() {
            return;
        }
        let (x, y, w, h) = (sel.x, sel.y, sel.w, sel.h);
        self.checkpoint();
        let f = self.bitmap.crop(x, y, w, h);
        self.bitmap.fill_rect(x, y, w, h, self.back_color);
        if let Some(sel) = &mut self.selection {
            sel.floating = Some(f);
        }
        self.changed();
    }

    /// Anda com a seleção (as setas: 1 pixel; com Shift, 10).
    pub fn move_selection(&mut self, dx: i32, dy: i32) -> bool {
        if self.selection.is_none() {
            return false;
        }
        self.lift();
        if let Some(sel) = &mut self.selection {
            sel.x += dx;
            sel.y += dy;
        }
        true
    }

    /// Seleciona a imagem inteira (Ctrl+A).
    pub fn select_all(&mut self) {
        self.commit_selection();
        self.tool = PaintTool::Select;
        self.selection = Some(Selection { x: 0, y: 0, w: self.bitmap.width as i32, h: self.bitmap.height as i32, floating: None });
    }

    /// Copia a seleção para a área de transferência do Aylis (Ctrl+C): os outros apps
    /// colam a imagem também.
    pub fn copy_selection(&mut self, ui: &mut Ui) -> bool {
        let Some(sel) = &self.selection else { return false };
        let b = match &sel.floating {
            Some(f) => f.clone(),
            None => self.bitmap.crop(sel.x, sel.y, sel.w, sel.h),
        };
        aylis_core::clipboard::set(ui, aylis_core::clipboard::ClipboardContent::image(Image::rgba(b.width, b.height, b.pixels)));
        true
    }

    /// Apaga a área selecionada com a cor 2 (Delete).
    pub fn delete_selection(&mut self) -> bool {
        let Some(sel) = self.selection.take() else { return false };
        if sel.floating.is_none() {
            self.checkpoint();
            self.bitmap.fill_rect(sel.x, sel.y, sel.w, sel.h, self.back_color);
        }
        self.changed();
        true
    }

    /// Cola a imagem da área de transferência (a copiada aqui ou noutro app) no canto de
    /// cima, como seleção solta (Ctrl+V).
    pub fn paste(&mut self, ui: &mut Ui) -> bool {
        let Some(image) = aylis_core::clipboard::image(ui) else { return false };
        let (width, height, pixels) = match aylis_core::clipboard::rgba(&image) {
            Some((w, h, px)) => (w, h, px.into_owned()),
            None => match image.source() {
                aylis_core::ImageSource::Encoded(bytes) => {
                    let Ok(decoded) = ::image::load_from_memory(bytes) else { return false };
                    let decoded = decoded.to_rgba8();
                    (decoded.width(), decoded.height(), decoded.into_raw())
                }
                _ => return false,
            },
        };
        if width == 0 || height == 0 || width > 8192 || height > 8192 {
            return false;
        }
        let clip = Bitmap { width, height, pixels };
        self.commit_selection();
        self.checkpoint();
        self.tool = PaintTool::Select;
        self.selection = Some(Selection { x: 0, y: 0, w: clip.width as i32, h: clip.height as i32, floating: Some(clip) });
        true
    }

    pub fn can_undo(&self) -> bool {
        !self.undo.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo.is_empty()
    }

    pub fn undo(&mut self) -> bool {
        // Uma seleção solta volta para o lugar (o ponto de desfazer é de antes de ela sair).
        self.selection = None;
        let Some(b) = self.undo.pop() else { return false };
        self.redo.push(std::mem::replace(&mut self.bitmap, b));
        self.fit();
        self.changed();
        true
    }

    pub fn redo(&mut self) -> bool {
        self.commit_selection();
        let Some(b) = self.redo.pop() else { return false };
        self.undo.push(std::mem::replace(&mut self.bitmap, b));
        self.fit();
        self.changed();
        true
    }

    /// Uma imagem nova em branco (`back_color`), que também pode ser desfeita.
    pub fn new_image(&mut self, width: u32, height: u32) {
        self.selection = None;
        self.checkpoint();
        self.bitmap = Bitmap::new(width, height, self.back_color);
        self.fit();
        self.changed();
    }

    /// Abre um PNG, JPEG, BMP ou GIF.
    pub fn open(&mut self, path: &str) -> Result<(), String> {
        let img = ::image::open(path).map_err(|e| format!("{path}: {e}"))?.to_rgba8();
        self.commit_selection();
        self.checkpoint();
        self.bitmap = Bitmap { width: img.width(), height: img.height(), pixels: img.into_raw() };
        self.fit();
        self.changed();
        Ok(())
    }

    /// Grava em PNG, JPEG ou BMP (pela extensão; sem extensão conhecida, PNG).
    pub fn save(&self, path: &str) -> Result<(), String> {
        let b = self.composited();
        let lower = path.to_lowercase();
        let result = if lower.ends_with(".jpg") || lower.ends_with(".jpeg") || lower.ends_with(".bmp") {
            // Sem alfa nesses formatos.
            let rgb: Vec<u8> = b.pixels.chunks(4).flat_map(|p| [p[0], p[1], p[2]]).collect();
            ::image::save_buffer(path, &rgb, b.width, b.height, ::image::ExtendedColorType::Rgb8)
        } else {
            ::image::save_buffer_with_format(path, &b.pixels, b.width, b.height, ::image::ExtendedColorType::Rgba8, ::image::ImageFormat::Png)
        };
        result.map_err(|e| format!("{path}: {e}"))
    }

    fn after_set(&mut self, name: &str) {
        if name == "tool" && self.tool != PaintTool::Select {
            self.commit_selection();
        }
        if name == "zoom" {
            self.zoom = self.zoom.clamp(0.1, 16.0);
            self.fit();
        }
        self.brush_size = self.brush_size.clamp(1, 64);
    }
}

impl Component for PaintCanvas {
    crate::leaf_control!("PaintCanvas");

    fn methods(&self) -> &'static [MethodInfo] {
        use PropKind::{Bool, Int, String};
        const METHODS: &[MethodInfo] = &[
            MethodInfo::new("undo", &[], Some(Bool)),
            MethodInfo::new("redo", &[], Some(Bool)),
            MethodInfo::new("can_undo", &[], Some(Bool)),
            MethodInfo::new("can_redo", &[], Some(Bool)),
            MethodInfo::new("size", &[], Some(String)),
            MethodInfo::new("checksum", &[], Some(String)),
            MethodInfo::new("clear", &[], Some(Bool)),
            MethodInfo::new("resize", &[Int, Int], Some(Bool)).params(&["width", "height"]),
            MethodInfo::new("new", &[Int, Int], Some(Bool)).params(&["width", "height"]),
            MethodInfo::new("open", &[String], Some(Bool)).params(&["path"]),
            MethodInfo::new("save", &[String], Some(Bool)).params(&["path"]),
            MethodInfo::new("select_all", &[], Some(Bool)),
            MethodInfo::new("has_selection", &[], Some(Bool)),
            MethodInfo::new("copy", &[], Some(Bool)),
            MethodInfo::new("cut", &[], Some(Bool)),
            MethodInfo::new("paste", &[], Some(Bool)),
            MethodInfo::new("delete", &[], Some(Bool)),
            MethodInfo::new("deselect", &[], Some(Bool)),
        ];
        METHODS
    }

    properties! {
        tool: PaintTool => "Behavior",
        color: Color => "Appearance",
        back_color: Color => "Appearance",
        brush_size: i64 => "Behavior",
        filled: bool => "Behavior",
        zoom: f32 => "Appearance";
        events: on_change, on_pick, on_cursor;
        on_set: after_set
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let b = &self.bitmap;
        let z = self.zoom.clamp(0.1, 16.0);
        let (cols, rows) = self.tile_grid();
        let mut tiles = self.tiles.borrow_mut();
        if tiles.len() != (cols * rows) as usize {
            *tiles = vec![None; (cols * rows) as usize];
        }
        for ty in 0..rows {
            for tx in 0..cols {
                let (x, y) = (tx * TILE, ty * TILE);
                let (w, h) = ((TILE + 1).min(b.width - x), (TILE + 1).min(b.height - y));
                let image = tiles[(ty * cols + tx) as usize].get_or_insert_with(|| Image::rgba(w, h, b.rows(x, y, w, h))).clone();
                let dest = RectF::new(x as f32 * z, y as f32 * z, w as f32 * z, h as f32 * z);
                canvas.draw_image(&image, RectF::new(0.0, 0.0, w as f32, h as f32), dest, 1.0);
            }
        }
        drop(tiles);
        // A prévia da forma sendo arrastada, desenhada como vai ficar.
        if let Some((button, start, end)) = self.drag.filter(|_| self.tool.is_shape()) {
            let (x0, y0, x1, y1) = (start.0.min(end.0), start.1.min(end.1), start.0.max(end.0), start.1.max(end.1));
            let mut preview = Bitmap::new((x1 - x0 + 1 + 2 * self.size()) as u32, (y1 - y0 + 1 + 2 * self.size()) as u32, Color::rgba(0, 0, 0, 0));
            let off = (x0 - self.size(), y0 - self.size());
            let mut me = PaintCanvas { tool: self.tool, color: self.color, back_color: self.back_color, brush_size: self.brush_size, filled: self.filled, ..Default::default() };
            me.shape(&mut preview, button, (start.0 - off.0, start.1 - off.1), (end.0 - off.0, end.1 - off.1));
            let z = self.zoom.clamp(0.1, 16.0);
            let dest = RectF::new(off.0 as f32 * z, off.1 as f32 * z, preview.width as f32 * z, preview.height as f32 * z);
            let img = Image::rgba(preview.width, preview.height, preview.pixels);
            canvas.draw_image(&img, RectF::new(0.0, 0.0, preview.width as f32, preview.height as f32), dest, 1.0);
        }
        // A seleção: os pixels soltos e o contorno tracejado (o retângulo sendo puxado,
        // enquanto arrasta).
        let z = self.zoom.clamp(0.1, 16.0);
        let area = match (&self.selection, self.drag.filter(|_| self.tool == PaintTool::Select && self.grab.is_none())) {
            (_, Some((_, a, b))) => Some((a.0.min(b.0), a.1.min(b.1), (a.0 - b.0).abs() + 1, (a.1 - b.1).abs() + 1)),
            (Some(sel), None) => {
                if let Some(f) = &sel.floating {
                    let img = Image::rgba(f.width, f.height, f.pixels.clone());
                    let dest = RectF::new(sel.x as f32 * z, sel.y as f32 * z, f.width as f32 * z, f.height as f32 * z);
                    canvas.draw_image(&img, RectF::new(0.0, 0.0, f.width as f32, f.height as f32), dest, 1.0);
                }
                Some((sel.x, sel.y, sel.w, sel.h))
            }
            (None, None) => None,
        };
        if let Some((x, y, w, h)) = area {
            let r = RectF::new(x as f32 * z + 0.5, y as f32 * z + 0.5, w as f32 * z - 1.0, h as f32 * z - 1.0);
            canvas.stroke_rect(r, 0.0, &Stroke::new(Color::rgb(255, 255, 255), 1.0));
            canvas.stroke_rect(r, 0.0, &Stroke::new(Color::rgb(0, 120, 215), 1.0).dashed(vec![4.0, 4.0]));
        }
        // As alças da borda (quadradinhos brancos, como no Paint) e, arrastando uma, o
        // contorno do tamanho novo.
        if !ctx.design {
            for (_, h) in self.handles() {
                canvas.fill_rect(h, 0.0, &aylis_core::Brush::Solid(Color::rgb(255, 255, 255)));
                canvas.stroke_rect(h.inflate(-0.5), 0.0, &Stroke::new(Color::rgb(0x6E, 0x6E, 0x6E), 1.0));
            }
        }
        if let Some((_, (w, h))) = self.resizing {
            let r = RectF::new(0.5, 0.5, w as f32 * z - 1.0, h as f32 * z - 1.0);
            canvas.stroke_rect(r, 0.0, &Stroke::new(Color::rgb(0, 120, 215), 1.0).dashed(vec![3.0, 3.0]));
        }
    }

    fn key_down(&mut self, ctx: &mut EventCtx<'_>, e: &KeyEvent) {
        if self.selection.is_none() || e.modifiers.ctrl || e.modifiers.alt {
            return;
        }
        let step = if e.modifiers.shift { 10 } else { 1 };
        let done = match e.key {
            Key::Left => self.move_selection(-step, 0),
            Key::Right => self.move_selection(step, 0),
            Key::Up => self.move_selection(0, -step),
            Key::Down => self.move_selection(0, step),
            Key::Delete => self.delete_selection(),
            Key::Escape | Key::Enter => {
                self.commit_selection();
                true
            }
            _ => false,
        };
        if done {
            ctx.set_handled();
            ctx.fire("on_change", EventArgs::None);
            ctx.repaint();
        }
    }

    fn pointer_down(&mut self, ctx: &mut EventCtx<'_>, e: &PointerEvent) {
        if !matches!(e.button, PointerButton::Primary | PointerButton::Secondary) {
            return;
        }
        // Uma alça da borda: redimensionar a imagem, com qualquer ferramenta.
        if let Some(edge) = self.edge_at(e.position).filter(|_| e.button == PointerButton::Primary) {
            self.resizing = Some((edge, (self.bitmap.width, self.bitmap.height)));
            ctx.repaint();
            return;
        }
        let p = self.to_image(e.position);
        match self.tool {
            PaintTool::Picker => {
                if let Some(c) = self.bitmap.get(p.0, p.1) {
                    let c = Color::rgb(c.r, c.g, c.b);
                    let second = e.button == PointerButton::Secondary;
                    if second {
                        self.back_color = c;
                    } else {
                        self.color = c;
                    }
                    ctx.fire("on_pick", EventArgs::Text(format!("{}{}", c.to_hex(), if second { "2" } else { "" })));
                }
            }
            PaintTool::Fill => {
                let (fore, _) = self.colors(e.button);
                let before = self.bitmap.clone();
                if self.bitmap.flood(p.0, p.1, fore) {
                    self.undo.push(before);
                    if self.undo.len() > UNDO_LIMIT {
                        self.undo.remove(0);
                    }
                    self.redo.clear();
                    self.changed();
                    ctx.fire("on_change", EventArgs::None);
                }
            }
            PaintTool::Select => {
                // Dentro da seleção: pega para arrastar; fora: solta a anterior e começa outra.
                match &self.selection {
                    Some(sel) if sel.contains(p) => {
                        let grab = (p.0 - sel.x, p.1 - sel.y);
                        self.lift();
                        self.grab = Some(grab);
                    }
                    _ => {
                        self.commit_selection();
                        self.grab = None;
                    }
                }
                self.drag = Some((e.button, p, p));
                ctx.request_focus(ctx.id);
            }
            _ => {
                self.checkpoint();
                self.drag = Some((e.button, p, p));
                if !self.tool.is_shape() {
                    self.freehand(e.button, p, p);
                    self.stroke = vec![self.to_image_f(e.position)];
                }
            }
        }
        ctx.repaint();
    }

    fn pointer_move(&mut self, ctx: &mut EventCtx<'_>, e: &PointerEvent) {
        let p = self.to_image(e.position);
        if let Some((edge, _)) = self.resizing {
            // O tamanho novo acompanha o mouse; o rodapé mostra o tamanho.
            let (w, h) = ((p.0.max(0) + 1) as u32, (p.1.max(0) + 1) as u32);
            let size = match edge {
                Edge::Right => (w, self.bitmap.height),
                Edge::Bottom => (self.bitmap.width, h),
                Edge::Corner => (w, h),
            };
            self.resizing = Some((edge, size));
            ctx.fire("on_cursor", EventArgs::Text(format!("{} × {}", size.0, size.1)));
            ctx.repaint();
            return;
        }
        // Sobre uma alça, a seta de redimensionar; no resto, a cruz de desenhar.
        let cursor = match self.edge_at(e.position) {
            Some(Edge::Right) => CursorShape::ResizeHorizontal,
            Some(Edge::Bottom) => CursorShape::ResizeVertical,
            Some(Edge::Corner) => CursorShape::ResizeNwSe,
            None => CursorShape::Crosshair,
        };
        self.base.set_cursor(cursor);
        ctx.fire("on_cursor", EventArgs::Text(format!("{}, {}", p.0, p.1)));
        let Some((button, start, _)) = self.drag else { return };
        if self.tool == PaintTool::Select {
            if let (Some(g), Some(sel)) = (self.grab, &mut self.selection) {
                sel.x = p.0 - g.0;
                sel.y = p.1 - g.1;
            }
            self.drag = Some((button, start, p));
        } else if self.tool.is_shape() {
            self.drag = Some((button, start, p));
        } else {
            self.stroke_to(button, self.to_image_f(e.position));
            self.drag = Some((button, start, p));
        }
        ctx.repaint();
    }

    fn pointer_up(&mut self, ctx: &mut EventCtx<'_>, e: &PointerEvent) {
        if let Some((_, (w, h))) = self.resizing.take() {
            if (w, h) != (self.bitmap.width, self.bitmap.height) {
                self.resize_image(w, h);
                ctx.fire("on_change", EventArgs::None);
                ctx.invalidate();
            }
            ctx.repaint();
            return;
        }
        let Some((button, start, _)) = self.drag.take() else { return };
        if !self.tool.is_shape() && self.tool != PaintTool::Select {
            self.stroke_end(button);
        }
        if self.tool == PaintTool::Select {
            if self.grab.take().is_none() {
                // O retângulo puxado vira a seleção (dentro da imagem).
                let end = self.to_image(e.position);
                let (w, h) = (self.bitmap.width as i32, self.bitmap.height as i32);
                let (x0, y0) = (start.0.min(end.0).clamp(0, w), start.1.min(end.1).clamp(0, h));
                let (x1, y1) = ((start.0.max(end.0) + 1).clamp(0, w), (start.1.max(end.1) + 1).clamp(0, h));
                self.selection = (x1 - x0 > 1 && y1 - y0 > 1).then_some(Selection { x: x0, y: y0, w: x1 - x0, h: y1 - y0, floating: None });
            } else {
                ctx.fire("on_change", EventArgs::None);
            }
            ctx.repaint();
            return;
        }
        if self.tool.is_shape() {
            let end = self.to_image(e.position);
            let mut bitmap = std::mem::replace(&mut self.bitmap, Bitmap::new(1, 1, Color::rgb(0, 0, 0)));
            self.shape(&mut bitmap, button, start, end);
            self.bitmap = bitmap;
            self.changed();
        }
        ctx.fire("on_change", EventArgs::None);
        ctx.repaint();
    }

    fn pointer_leave(&mut self, ctx: &mut EventCtx<'_>) {
        ctx.fire("on_cursor", EventArgs::Text(String::new()));
    }

    fn invoke(&mut self, ctx: &mut EventCtx<'_>, method: &str, args: &[PropValue]) -> Result<PropValue, String> {
        let text = |i: usize| match args.get(i) {
            Some(PropValue::String(s)) => Ok(s.clone()),
            _ => Err(format!("{method}: falta o arquivo")),
        };
        let number = |i: usize, default: u32| args.get(i).and_then(|v| v.to_i64().ok()).map_or(default, |n| n.clamp(1, 8192) as u32);
        let result = match method {
            "undo" => PropValue::Bool(self.undo()),
            "redo" => PropValue::Bool(self.redo()),
            "can_undo" => return Ok(PropValue::Bool(self.can_undo())),
            "can_redo" => return Ok(PropValue::Bool(self.can_redo())),
            "size" => return Ok(PropValue::String(format!("{} × {}", self.bitmap.width, self.bitmap.height))),
            // Um resumo dos pixels (FNV-1a): duas imagens iguais dão o mesmo texto.
            "checksum" => {
                let b = self.composited();
                let hash = b.pixels.iter().fold(0xcbf2_9ce4_8422_2325u64, |h, &b| (h ^ b as u64).wrapping_mul(0x0100_0000_01b3));
                return Ok(PropValue::String(format!("{}x{}:{hash:016x}", b.width, b.height)));
            }
            "clear" => {
                self.new_image(self.bitmap.width, self.bitmap.height);
                PropValue::Bool(true)
            }
            "resize" => {
                let (w, h) = (number(0, self.bitmap.width), number(1, self.bitmap.height));
                self.resize_image(w, h);
                PropValue::Bool(true)
            }
            "new" => {
                let (w, h) = (number(0, self.bitmap.width), number(1, self.bitmap.height));
                self.new_image(w, h);
                PropValue::Bool(true)
            }
            "open" => {
                self.open(&text(0)?)?;
                PropValue::Bool(true)
            }
            "save" => {
                self.save(&text(0)?)?;
                return Ok(PropValue::Bool(true));
            }
            "select_all" => {
                self.select_all();
                PropValue::Bool(true)
            }
            "has_selection" => return Ok(PropValue::Bool(self.has_selection())),
            "copy" => return Ok(PropValue::Bool(self.copy_selection(ctx.ui))),
            "cut" => {
                let ok = self.copy_selection(ctx.ui) && self.delete_selection();
                if ok {
                    ctx.fire("on_change", EventArgs::None);
                }
                PropValue::Bool(ok)
            }
            "paste" => {
                let ok = self.paste(ctx.ui);
                if ok {
                    ctx.fire("on_change", EventArgs::None);
                }
                PropValue::Bool(ok)
            }
            "delete" => {
                let ok = self.delete_selection();
                if ok {
                    ctx.fire("on_change", EventArgs::None);
                }
                PropValue::Bool(ok)
            }
            "deselect" => {
                self.commit_selection();
                PropValue::Bool(true)
            }
            _ => return Err(format!("PaintCanvas não tem o método '{method}'")),
        };
        ctx.invalidate();
        ctx.repaint();
        Ok(result)
    }
}
