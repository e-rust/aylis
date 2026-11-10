//! O componente [`DocViewer`].

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use aylis_core::{
    Brush, Canvas, Color, Component, ComponentId, ControlBase, EventArgs, EventCtx, FontWeight, GestureEvent, Image, KeyEvent, PaintCtx,
    PointF, PointerButton, PointerEvent, RectF, Stroke, TextAlign, TextStyle, Ui, WheelEvent, properties,
};

use crate::office::{self, Block, Sheet};
use crate::pdf::{self, Renderer};

const GAP: f32 = 12.0;
const ROW: f32 = 26.0;
const HEADER: f32 = 24.0;
const ROW_NUMBERS: f32 = 48.0;
const TABS: f32 = 30.0;

/// O que está aberto.
enum Doc {
    None,
    /// Lendo numa thread (planilha ou texto grande não congela a tela).
    Loading,
    Error(String),
    Pdf { sizes: Vec<(f32, f32)>, renderer: Renderer },
    Sheets(Vec<Sheet>),
    Text(Vec<Block>),
}

/// Visualizador de documentos: PDF, planilhas (XLSX, XLS, ODS, CSV) e textos (DOCX, ODT).
/// Roda/dedo rolam; Ctrl+roda e a pinça dão zoom.
pub struct DocViewer {
    pub base: ControlBase,
    /// Arquivo do aplicativo (`assets/docs/manual.pdf`) ou caminho no disco.
    pub file: String,
    /// 1 = a página na largura do controle (PDF) ou o tamanho normal (planilha, texto).
    pub zoom: f32,
    /// A planilha mostrada (das abas de baixo).
    pub sheet: i64,
    /// O maior conteúdo descompactado aceito, em MB (0: o padrão, 64 para textos e 256 para
    /// planilhas). Arquivos de fora podem ser bombas de ZIP.
    pub max_document_size: i64,
    /// A maior página de PDF rasterizada, em megapixels; acima disso a página sai menos nítida.
    pub max_pixels: i64,
    doc: Doc,
    loaded: String,
    scroll: PointF,
    /// PDF: páginas prontas por (página, escala) e as já pedidas.
    pages: RefCell<HashMap<(usize, u32), Image>>,
    asked: RefCell<HashSet<(usize, u32)>>,
    /// Da última pintura: as abas das planilhas, o tamanho do conteúdo e as larguras das
    /// colunas.
    tabs: RefCell<Vec<(RectF, usize)>>,
    content: std::cell::Cell<(f32, f32)>,
    widths: RefCell<(usize, Vec<f32>)>,
    text_layout: RefCell<(f32, Vec<Line>)>,
}

/// Uma linha já quebrada do texto (ou uma linha de tabela).
#[derive(Clone)]
enum Line {
    Text { text: String, size: f32, bold: bool, y: f32 },
    Row { cells: Vec<String>, y: f32, height: f32, first: bool },
}

impl Default for DocViewer {
    fn default() -> Self {
        let mut base = ControlBase::new(480.0, 600.0);
        base.set_can_focus(true).set_clip_children(true);
        Self {
            base,
            file: String::new(),
            zoom: 1.0,
            sheet: 0,
            max_document_size: 0,
            max_pixels: 64,
            doc: Doc::None,
            loaded: String::new(),
            scroll: PointF::ZERO,
            pages: RefCell::default(),
            asked: RefCell::default(),
            tabs: RefCell::default(),
            content: std::cell::Cell::new((0.0, 0.0)),
            widths: RefCell::default(),
            text_layout: RefCell::default(),
        }
    }
}

fn extension(file: &str) -> String {
    std::path::Path::new(file).extension().and_then(|e| e.to_str()).unwrap_or_default().to_lowercase()
}

impl DocViewer {
    pub fn new(file: impl Into<String>) -> Self {
        Self { file: file.into(), ..Default::default() }
    }

    /// Quantas páginas (PDF) ou planilhas.
    pub fn page_count(&self) -> usize {
        match &self.doc {
            Doc::Pdf { sizes, .. } => sizes.len(),
            Doc::Sheets(s) => s.len(),
            _ => 0,
        }
    }

    /// A página do PDF no meio da área visível (a partir de 0).
    pub fn current_page(&self) -> usize {
        let Doc::Pdf { sizes, .. } = &self.doc else { return 0 };
        let middle = self.scroll.y + self.base.bounds().height / 2.0;
        let mut y = GAP;
        for (i, (w, h)) in sizes.iter().enumerate() {
            y += h * self.page_scale(*w) + GAP;
            if y > middle {
                return i;
            }
        }
        sizes.len().saturating_sub(1)
    }

    /// Erro ao abrir, se houve.
    pub fn error(&self) -> Option<&str> {
        match &self.doc {
            Doc::Error(e) => Some(e),
            _ => None,
        }
    }

    /// Abre o arquivo (`file`) agora. O `aylis` chama sozinho quando `file` muda. Planilhas e
    /// textos são lidos numa thread (fora do navegador): o resultado chega depois, com
    /// `on_load` ou `on_error`.
    pub fn load(ui: &mut Ui, id: ComponentId) {
        let Some((file, max_mb, max_pixels)) = ui.get::<DocViewer>(id).map(|d| (d.file.clone(), d.max_document_size, d.max_pixels)) else { return };
        let limit = |default: usize| if max_mb > 0 { max_mb as usize * 1024 * 1024 } else { default };
        let max_pixels = (max_pixels.max(1) as f32) * 1_000_000.0;
        let bytes = ui
            .services()
            .resources
            .as_ref()
            .and_then(|r| r.read(&file))
            .map(Ok)
            .unwrap_or_else(|| std::fs::read(&file).map_err(|e| format!("{file}: {e}")));
        let ext = extension(&file);
        let doc = match bytes {
            Err(e) => Doc::Error(e),
            Ok(bytes) => match ext.as_str() {
                "pdf" => {
                    let poster = ui.poster();
                    let bytes = Arc::new(bytes);
                    match pdf::page_sizes(&bytes).and_then(|sizes| {
                        let renderer = Renderer::new(bytes.clone(), max_pixels, move |page| {
                            poster.post(move |ui| DocViewer::page_ready(ui, id, page));
                        })?;
                        Ok(Doc::Pdf { sizes, renderer })
                    }) {
                        Ok(d) => d,
                        Err(e) => Doc::Error(e),
                    }
                }
                "xlsx" | "xlsm" | "xls" | "xlsb" | "ods" | "csv" | "tsv" | "docx" | "odt" => {
                    let (max_sheets, max_text) = (limit(office::MAX_SHEETS), limit(office::MAX_DOCUMENT));
                    let read = move || match ext.as_str() {
                        "docx" | "odt" => office::read_document_with(&bytes, &ext, max_text).map_or_else(Doc::Error, Doc::Text),
                        _ => office::read_sheets_with(&bytes, &ext, max_sheets).map_or_else(Doc::Error, Doc::Sheets),
                    };
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        let poster = ui.poster();
                        let wanted = file.clone();
                        std::thread::Builder::new()
                            .name("aylis-docview".into())
                            .spawn(move || {
                                let doc = read();
                                poster.post(move |ui| {
                                    // Outro arquivo foi pedido no meio: este resultado não vale mais.
                                    if ui.get::<DocViewer>(id).is_some_and(|d| d.file == wanted) {
                                        DocViewer::show(ui, id, wanted, doc);
                                    }
                                });
                            })
                            .map_or_else(|e| Doc::Error(e.to_string()), |_| Doc::Loading)
                    }
                    #[cfg(target_arch = "wasm32")]
                    read()
                }
                other => Doc::Error(format!("formato não suportado: .{other}")),
            },
        };
        DocViewer::show(ui, id, file, doc);
    }

    /// Mostra o que foi lido e avisa (`on_load`, `on_error`); `Loading` só marca o arquivo.
    fn show(ui: &mut Ui, id: ComponentId, file: String, doc: Doc) {
        let loading = matches!(doc, Doc::Loading);
        let error = if let Doc::Error(e) = &doc { Some(e.clone()) } else { None };
        if let Some(d) = ui.get_mut::<DocViewer>(id) {
            d.doc = doc;
            d.loaded = file;
            d.scroll = PointF::ZERO;
            d.sheet = 0;
            d.pages.borrow_mut().clear();
            d.asked.borrow_mut().clear();
            d.widths.borrow_mut().1.clear();
            d.text_layout.borrow_mut().1.clear();
        }
        if let Some(e) = error {
            ui.fire(id, "on_error", &EventArgs::Text(e));
        } else if !loading {
            ui.fire(id, "on_load", &EventArgs::None);
        }
        ui.request_repaint();
    }

    fn page_ready(ui: &mut Ui, id: ComponentId, (page, scale, w, h, rgba): pdf::Rendered) {
        if let Some(d) = ui.get::<DocViewer>(id) {
            d.pages.borrow_mut().insert((page, scale), Image::rgba(w, h, rgba));
        }
        ui.request_repaint();
    }

    /// Pixels de tela por ponto do PDF para uma página de largura `width` (pontos).
    fn page_scale(&self, width: f32) -> f32 {
        let view = (self.base.bounds().width - 2.0 * GAP).max(50.0);
        view / width.max(1.0) * self.zoom
    }

    fn zoom_by(&mut self, factor: f32, at: PointF) {
        let old = self.zoom;
        self.zoom = (self.zoom * factor).clamp(0.25, 6.0);
        let k = self.zoom / old;
        // O ponto sob o mouse continua sob ele.
        self.scroll = PointF::new(((self.scroll.x + at.x) * k - at.x).max(0.0), ((self.scroll.y + at.y) * k - at.y).max(0.0));
    }

    fn clamp_scroll(&mut self) {
        let b = self.base.bounds();
        let (w, h) = self.content.get();
        let bottom = if matches!(&self.doc, Doc::Sheets(s) if s.len() > 1) { TABS } else { 0.0 };
        self.scroll.x = self.scroll.x.clamp(0.0, (w - b.width).max(0.0));
        self.scroll.y = self.scroll.y.clamp(0.0, (h - b.height + bottom).max(0.0));
    }

    fn paint_pdf(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>, sizes: &[(f32, f32)], renderer: &Renderer) {
        let r = ctx.rect;
        canvas.fill_rect(r, 0.0, &Brush::Solid(Color::rgb(0x52, 0x56, 0x5D)));
        let mut y = GAP;
        let mut widest: f32 = 0.0;
        let mut ready = Vec::new();
        for (i, (pw, ph)) in sizes.iter().enumerate() {
            let s = self.page_scale(*pw);
            let (w, h) = (pw * s, ph * s);
            widest = widest.max(w);
            let x = ((r.width - w) / 2.0).max(GAP) - self.scroll.x;
            let page = RectF::new(r.x + x, r.y + y - self.scroll.y, w, h);
            y += h + GAP;
            if page.bottom() < r.y || page.y > r.bottom() {
                continue;
            }
            canvas.draw_shadow(page, 0.0, &aylis_core::Shadow::new(Color::rgba(0, 0, 0, 90), 8.0, 0.0, 2.0));
            canvas.fill_rect(page, 0.0, &Brush::Solid(Color::WHITE));
            // Rasteriza no dobro (telas de alta densidade), em degraus de 25% do zoom.
            let key = (i, ((s * 2.0 * 4.0).round() / 4.0 * 100.0) as u32);
            let image = self.pages.borrow().get(&key).cloned().or_else(|| {
                // Enquanto não chega: a mesma página em outra escala, se houver.
                self.pages.borrow().iter().find(|((p, _), _)| *p == i).map(|(_, img)| img.clone())
            });
            if let Some(img) = image {
                if let Some(size) = canvas.image_size(&img) {
                    canvas.draw_image(&img, RectF::from_size(size), page, 1.0);
                }
            }
            if !self.pages.borrow().contains_key(&key) && self.asked.borrow_mut().insert(key) {
                if let Some(done) = renderer.request(key.0, key.1) {
                    ready.push(done);
                }
            }
        }
        self.content.set((widest + 2.0 * GAP, y));
        // No navegador a página já veio: guarda depois da pintura.
        if !ready.is_empty() {
            let id = ctx.id;
            ctx.ui.poster().post(move |ui| {
                for p in ready {
                    DocViewer::page_ready(ui, id, p);
                }
            });
        }
        // Página atual.
        let label = format!("{} / {}", self.current_page() + 1, sizes.len());
        let mut t = ctx.text_style();
        t.color = Color::WHITE;
        t.h_align = TextAlign::Center;
        t.font.size = 12.0;
        let w = canvas.measure_text(&label, &t).width + 20.0;
        let pill = RectF::new(r.right() - w - 12.0, r.bottom() - 34.0, w, 24.0);
        canvas.fill_rect(pill, 12.0, &Brush::Solid(Color::rgba(0, 0, 0, 150)));
        canvas.draw_text(&label, pill, &t);
    }

    fn column_name(mut i: usize) -> String {
        let mut s = String::new();
        loop {
            s.insert(0, (b'A' + (i % 26) as u8) as char);
            if i < 26 {
                return s;
            }
            i = i / 26 - 1;
        }
    }

    fn paint_sheets(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>, sheets: &[Sheet]) {
        let r = ctx.rect;
        let theme = ctx.theme;
        canvas.fill_rect(r, 0.0, &Brush::Solid(theme.color("surface")));
        let index = (self.sheet.max(0) as usize).min(sheets.len().saturating_sub(1));
        let Some(sheet) = sheets.get(index) else { return };
        let z = self.zoom;
        let mut t = ctx.text_style();
        t.font.size *= z;
        let cols = sheet.rows.iter().map(Vec::len).max().unwrap_or(0);
        // Larguras pelo maior texto das 300 primeiras linhas (medidas uma vez por planilha).
        {
            let mut widths = self.widths.borrow_mut();
            if widths.0 != index || widths.1.len() != cols {
                let mut w = vec![60.0f32; cols];
                for row in sheet.rows.iter().take(300) {
                    for (c, cell) in row.iter().enumerate() {
                        w[c] = w[c].max((canvas.measure_text(cell, &ctx.text_style()).width + 16.0).min(320.0));
                    }
                }
                *widths = (index, w);
            }
        }
        let widths: Vec<f32> = self.widths.borrow().1.iter().map(|w| w * z).collect();
        let (row_h, header_h, nums_w) = (ROW * z, HEADER * z, ROW_NUMBERS * z);
        let total_w = nums_w + widths.iter().sum::<f32>();
        let total_h = header_h + sheet.rows.len() as f32 * row_h;
        self.content.set((total_w, total_h));
        let grid = Stroke::new(theme.color("border"), 1.0);
        let head_bg = Brush::Solid(theme.color("background"));
        let bottom = if sheets.len() > 1 { r.bottom() - TABS } else { r.bottom() };
        canvas.save();
        canvas.clip_rect(RectF::new(r.x, r.y, r.width, bottom - r.y));
        // Linhas visíveis.
        let first = ((self.scroll.y / row_h).floor() as usize).min(sheet.rows.len());
        let visible = ((bottom - r.y - header_h) / row_h).ceil() as usize + 1;
        let mut ht = t.clone();
        ht.h_align = TextAlign::Center;
        ht.color = theme.color("text_muted");
        for (n, row) in sheet.rows.iter().enumerate().skip(first).take(visible) {
            let y = r.y + header_h + n as f32 * row_h - self.scroll.y;
            let mut x = r.x + nums_w - self.scroll.x;
            for (c, w) in widths.iter().enumerate() {
                if x + w >= r.x && x <= r.right() {
                    let cell = RectF::new(x, y, *w, row_h);
                    canvas.stroke_rect(cell, 0.0, &grid);
                    if let Some(text) = row.get(c).filter(|s| !s.is_empty()) {
                        let mut ct = t.clone();
                        // Números à direita, como numa planilha.
                        if text.replace(['.', ',', '-'], "").chars().all(|ch| ch.is_ascii_digit()) {
                            ct.h_align = TextAlign::Trailing;
                        }
                        canvas.draw_text(text, cell.inflate(-6.0 * z), &ct);
                    }
                }
                x += w;
            }
            let num = RectF::new(r.x, y, nums_w, row_h);
            canvas.fill_rect(num, 0.0, &head_bg);
            canvas.stroke_rect(num, 0.0, &grid);
            canvas.draw_text(&(n + 1).to_string(), num, &ht);
        }
        // Cabeçalho das colunas.
        let mut x = r.x + nums_w - self.scroll.x;
        for (c, w) in widths.iter().enumerate() {
            let cell = RectF::new(x, r.y, *w, header_h);
            if cell.right() >= r.x && cell.x <= r.right() {
                canvas.fill_rect(cell, 0.0, &head_bg);
                canvas.stroke_rect(cell, 0.0, &grid);
                canvas.draw_text(&Self::column_name(c), cell, &ht);
            }
            x += w;
        }
        canvas.fill_rect(RectF::new(r.x, r.y, nums_w, header_h), 0.0, &head_bg);
        canvas.restore();
        // Abas das planilhas.
        let mut tabs = Vec::new();
        if sheets.len() > 1 {
            let bar = RectF::new(r.x, bottom, r.width, TABS);
            canvas.fill_rect(bar, 0.0, &head_bg);
            canvas.draw_line(PointF::new(bar.x, bar.y), PointF::new(bar.right(), bar.y), &grid);
            let mut x = bar.x + 8.0;
            let mut tt = ctx.text_style();
            tt.h_align = TextAlign::Center;
            for (i, s) in sheets.iter().enumerate() {
                let w = canvas.measure_text(&s.name, &tt).width + 24.0;
                let tab = RectF::new(x, bar.y + 3.0, w, TABS - 6.0);
                if i == index {
                    canvas.fill_rect(tab, 6.0, &Brush::Solid(theme.color("surface")));
                    canvas.stroke_rect(tab, 6.0, &Stroke::new(theme.color("primary"), 1.5));
                    tt.font.weight = FontWeight::SemiBold;
                } else {
                    tt.font.weight = FontWeight::Normal;
                }
                canvas.draw_text(&s.name, tab, &tt);
                tabs.push((tab, i));
                x += w + 4.0;
            }
        }
        *self.tabs.borrow_mut() = tabs;
    }

    /// Quebra o texto em linhas da largura `width`.
    fn layout_text(canvas: &mut dyn Canvas, blocks: &[Block], width: f32, base: &TextStyle, zoom: f32) -> Vec<Line> {
        let mut lines = Vec::new();
        let mut y = 16.0;
        for block in blocks {
            match block {
                Block::Paragraph { text, level } => {
                    let size = match level {
                        0 => base.font.size,
                        1 => base.font.size * 1.8,
                        2 => base.font.size * 1.45,
                        _ => base.font.size * 1.2,
                    } * zoom;
                    let mut style = base.clone();
                    style.font.size = size;
                    if *level > 0 {
                        style.font.weight = FontWeight::Bold;
                        y += size * 0.4;
                    }
                    let line_h = size * 1.45;
                    for para in text.split('\n') {
                        let mut current = String::new();
                        for word in para.split(' ') {
                            let candidate = if current.is_empty() { word.to_string() } else { format!("{current} {word}") };
                            if !current.is_empty() && canvas.measure_text(&candidate, &style).width > width {
                                lines.push(Line::Text { text: std::mem::take(&mut current), size, bold: *level > 0, y });
                                y += line_h;
                                current = word.to_string();
                            } else {
                                current = candidate;
                            }
                        }
                        lines.push(Line::Text { text: current, size, bold: *level > 0, y });
                        y += line_h;
                    }
                    y += size * 0.5;
                }
                Block::Table(rows) => {
                    let height = base.font.size * zoom * 1.9;
                    for (i, row) in rows.iter().enumerate() {
                        let cells = row.iter().map(|c| c.replace('\n', " ")).collect();
                        lines.push(Line::Row { cells, y, height, first: i == 0 });
                        y += height;
                    }
                    y += 12.0;
                }
            }
        }
        lines.push(Line::Text { text: String::new(), size: 0.0, bold: false, y: y + 16.0 });
        lines
    }

    fn paint_text(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>, blocks: &[Block]) {
        let r = ctx.rect;
        let theme = ctx.theme;
        canvas.fill_rect(r, 0.0, &Brush::Solid(theme.color("surface")));
        let margin = 24.0;
        let width = (r.width - 2.0 * margin).max(80.0);
        let key = width * 1000.0 + self.zoom;
        if self.text_layout.borrow().0 != key || self.text_layout.borrow().1.is_empty() {
            *self.text_layout.borrow_mut() = (key, Self::layout_text(canvas, blocks, width, &ctx.text_style(), self.zoom));
        }
        let layout = self.text_layout.borrow();
        let bottom = layout.1.last().map_or(0.0, |l| match l {
            Line::Text { y, .. } => *y,
            Line::Row { y, height, .. } => y + height,
        });
        self.content.set((r.width, bottom));
        let grid = Stroke::new(theme.color("border"), 1.0);
        for line in &layout.1 {
            match line {
                Line::Text { text, size, bold, y } => {
                    let top = r.y + y - self.scroll.y;
                    if top + size * 2.0 < r.y || top > r.bottom() || text.is_empty() {
                        continue;
                    }
                    let mut t = ctx.text_style();
                    t.font.size = *size;
                    if *bold {
                        t.font.weight = FontWeight::Bold;
                    }
                    canvas.draw_text(text, RectF::new(r.x + margin, top, width, size * 1.45), &t);
                }
                Line::Row { cells, y, height, first } => {
                    let top = r.y + y - self.scroll.y;
                    if top + height < r.y || top > r.bottom() || cells.is_empty() {
                        continue;
                    }
                    let w = width / cells.len() as f32;
                    let mut t = ctx.text_style();
                    t.font.size *= self.zoom;
                    if *first {
                        t.font.weight = FontWeight::SemiBold;
                    }
                    for (i, c) in cells.iter().enumerate() {
                        let cell = RectF::new(r.x + margin + w * i as f32, top, w, *height);
                        canvas.stroke_rect(cell, 0.0, &grid);
                        canvas.draw_text(c, cell.inflate(-6.0), &t);
                    }
                }
            }
        }
    }
}

impl Component for DocViewer {
    fn class_name(&self) -> &'static str {
        "DocViewer"
    }

    fn control(&self) -> Option<&ControlBase> {
        Some(&self.base)
    }

    fn control_mut(&mut self) -> Option<&mut ControlBase> {
        Some(&mut self.base)
    }

    properties! {
        file: String => "Behavior",
        zoom: f32 => "Appearance",
        sheet: i64 => "Behavior",
        max_document_size: i64 => "Behavior",
        max_pixels: i64 => "Behavior";
        events: on_load, on_error, on_change
    }

    fn property_changed(&mut self, ctx: &mut EventCtx<'_>, name: &str) {
        match name {
            "sheet" => {
                self.scroll = PointF::ZERO;
                ctx.repaint();
            }
            "zoom" => {
                self.zoom = self.zoom.clamp(0.25, 6.0);
                ctx.repaint();
            }
            _ => {}
        }
    }

    fn wheel(&mut self, ctx: &mut EventCtx<'_>, event: &WheelEvent) {
        if event.modifiers.ctrl {
            self.zoom_by(if event.delta.y > 0.0 { 1.1 } else { 1.0 / 1.1 }, event.position);
        } else if event.modifiers.shift && !event.touch {
            self.scroll.x -= event.delta.y;
        } else {
            self.scroll.x -= event.delta.x;
            self.scroll.y -= event.delta.y;
        }
        self.clamp_scroll();
        ctx.repaint();
        ctx.set_handled();
    }

    fn gesture(&mut self, ctx: &mut EventCtx<'_>, event: &GestureEvent) {
        self.scroll = PointF::new(self.scroll.x - event.pan.x, self.scroll.y - event.pan.y);
        self.zoom_by(event.delta_scale.max(0.01), event.center);
        self.clamp_scroll();
        ctx.repaint();
        ctx.set_handled();
    }

    fn key_down(&mut self, ctx: &mut EventCtx<'_>, event: &KeyEvent) {
        let page = self.base.bounds().height - 40.0;
        match event.key {
            aylis_core::Key::Down => self.scroll.y += 40.0,
            aylis_core::Key::Up => self.scroll.y -= 40.0,
            aylis_core::Key::PageDown | aylis_core::Key::Space => self.scroll.y += page,
            aylis_core::Key::PageUp => self.scroll.y -= page,
            aylis_core::Key::Home => self.scroll = PointF::ZERO,
            aylis_core::Key::End => self.scroll.y = f32::MAX,
            _ => return,
        }
        self.clamp_scroll();
        ctx.repaint();
        ctx.set_handled();
    }

    fn pointer_up(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if event.button != PointerButton::Primary {
            return;
        }
        let hit = self.tabs.borrow().iter().find(|(r, _)| r.contains(event.position)).map(|(_, i)| *i);
        if let Some(i) = hit {
            self.sheet = i as i64;
            self.scroll = PointF::ZERO;
            ctx.fire("on_change", EventArgs::Value((i as i64).into()));
            ctx.repaint();
        }
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        if self.loaded != self.file && !ctx.design && !self.file.is_empty() {
            let id = ctx.id;
            ctx.ui.poster().post(move |ui| DocViewer::load(ui, id));
        }
        canvas.save();
        canvas.clip_rect(r);
        match &self.doc {
            Doc::Pdf { sizes, renderer } => self.paint_pdf(canvas, ctx, sizes, renderer),
            Doc::Sheets(s) => self.paint_sheets(canvas, ctx, s),
            Doc::Text(b) => self.paint_text(canvas, ctx, b),
            Doc::Error(e) => {
                canvas.fill_rect(r, 0.0, &Brush::Solid(ctx.theme.color("surface")));
                let mut t = ctx.text_style();
                t.h_align = TextAlign::Center;
                t.wrap = true;
                t.color = ctx.theme.color("error");
                canvas.draw_text(&format!("⚠ {e}"), r.inflate(-16.0), &t);
            }
            Doc::None | Doc::Loading => {
                canvas.fill_rect(r, 0.0, &Brush::Solid(ctx.theme.color("surface")));
                let mut t = ctx.text_style();
                t.h_align = TextAlign::Center;
                t.color = ctx.theme.color("text_muted");
                let text = match (&self.doc, self.file.is_empty()) {
                    (_, true) => "DocViewer (PDF, XLSX, ODS, CSV, DOCX, ODT)".to_string(),
                    (Doc::Loading, _) => format!("{} {}…", aylis_core::messages::t("Carregando"), self.file),
                    _ => format!("📄 {}", self.file),
                };
                canvas.draw_text(&text, r, &t);
            }
        }
        canvas.restore();
    }
}
