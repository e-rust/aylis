//! Relatório de bandas, como o FastReport e o QuickReport.
//!
//! - [`Report`]: a página, desenhada no designer como um form (só aparece no designer; em
//!   execução, pela visualização, pelo PDF ou pela impressora). Ligada a um DataSource.
//! - [`ReportBand`]: as faixas, de cima para baixo: título, cabeçalho de página, cabeçalho
//!   de grupo, detalhe (uma por registro), rodapé de grupo, sumário e rodapé de página. Dentro
//!   delas, qualquer controle (texto, imagem, linha, forma, gráfico).
//! - Nos textos, entre colchetes: um campo (`[nome]`), uma expressão (`[preco * qtd]`), um
//!   total (`[SUM(valor)]`, `COUNT`, `AVG`, `MIN`, `MAX`: no rodapé de grupo, do grupo; no
//!   sumário, de tudo; no rodapé de página, da página), `[PAGE]`, `[PAGES]`, `[DATE]`,
//!   `[TIME]` e `[LINE]` (o número do registro).
//! - [`ReportPreview`]: as páginas prontas, com zoom e navegação, e os botões de PDF e de
//!   impressão.

use std::any::Any;
use std::cell::RefCell;
use std::sync::Arc;

use aylis_core::pages::{Op, Page, PageSet, RecordingCanvas};
use aylis_core::{
    Brush, Canvas, Color, Component, ComponentId, ControlBase, EventArgs, EventCtx, ImageSource, PaintCtx, PointF, PointerEvent, PropEnum, PropValue,
    RectF, Stroke, TextAlign, Ui, WheelEvent, properties,
};
use aylis_data::{DataEvent, DataSource, Value, view};

use crate::data_message;

/// O tipo de uma banda.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BandKind {
    /// No começo do relatório (uma vez).
    Title,
    /// No alto de cada página.
    PageHeader,
    /// Quando o valor de `group_field` muda.
    GroupHeader,
    /// Uma vez por registro.
    #[default]
    Detail,
    /// Depois do último registro do grupo (com os totais dele).
    GroupFooter,
    /// No fim do relatório (com os totais de tudo).
    Summary,
    /// No pé de cada página.
    PageFooter,
    /// Mestre-detalhe: antes dos registros do detalhe de cada registro do mestre.
    SubHeader,
    /// Mestre-detalhe: uma vez por registro do detalhe (o `data_source` da banda, ligado ao
    /// mestre pelo `master_source`), logo depois do registro do mestre.
    SubDetail,
    /// Mestre-detalhe: depois do último registro do detalhe (com os totais dele).
    SubFooter,
}

impl PropEnum for BandKind {
    const NAMES: &'static [&'static str] =
        &["title", "page_header", "group_header", "detail", "group_footer", "summary", "page_footer", "sub_header", "sub_detail", "sub_footer"];
    fn from_index(index: usize) -> Option<Self> {
        [
            Self::Title,
            Self::PageHeader,
            Self::GroupHeader,
            Self::Detail,
            Self::GroupFooter,
            Self::Summary,
            Self::PageFooter,
            Self::SubHeader,
            Self::SubDetail,
            Self::SubFooter,
        ]
        .get(index)
        .copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

impl BandKind {
    fn title(self) -> &'static str {
        match self {
            BandKind::Title => "Título",
            BandKind::PageHeader => "Cabeçalho de página",
            BandKind::GroupHeader => "Cabeçalho de grupo",
            BandKind::Detail => "Detalhe",
            BandKind::GroupFooter => "Rodapé de grupo",
            BandKind::Summary => "Sumário",
            BandKind::PageFooter => "Rodapé de página",
            BandKind::SubHeader => "Cabeçalho do detalhe",
            BandKind::SubDetail => "Sub-detalhe",
            BandKind::SubFooter => "Rodapé do detalhe",
        }
    }
}

/// A página do relatório (como o `TfrxReport`/`TQuickRep`). Tamanho em pontos (1/72"):
/// A4 = 595 × 842; `landscape` deita a página. As bandas ficam dentro dela, de cima para
/// baixo, na largura da área útil (a página menos as margens).
pub struct Report {
    pub base: ControlBase,
    pub data_source: Option<ComponentId>,
    /// O nome do documento (no PDF e na fila da impressora).
    pub title: String,
    pub page_width: f32,
    pub page_height: f32,
    pub landscape: bool,
    pub margin: f32,
    /// Quem gera o PDF (um provedor de PDF, item 25); sem ele, o PDF do próprio Aylis.
    pub provider: Option<ComponentId>,
}

impl Default for Report {
    fn default() -> Self {
        let mut base = ControlBase::new(523.0, 400.0);
        base.set_design_only(true);
        Self { base, data_source: None, title: "Relatório".into(), page_width: 595.0, page_height: 842.0, landscape: false, margin: 36.0, provider: None }
    }
}

impl Report {
    pub fn new(data_source: ComponentId) -> Self {
        Self { data_source: Some(data_source), ..Default::default() }
    }

    /// Largura e altura da página (deitada, trocadas).
    pub fn page_size(&self) -> (f32, f32) {
        if self.landscape { (self.page_height, self.page_width) } else { (self.page_width, self.page_height) }
    }
}

impl Component for Report {
    fn class_name(&self) -> &'static str {
        "Report"
    }

    fn control(&self) -> Option<&ControlBase> {
        Some(&self.base)
    }

    fn control_mut(&mut self) -> Option<&mut ControlBase> {
        Some(&mut self.base)
    }

    properties! {
        data_source: Option<ComponentId> => "Data",
        title: String => "Appearance",
        page_width: f32 => "Layout",
        page_height: f32 => "Layout",
        landscape: bool => "Layout",
        margin: f32 => "Layout",
        provider: Option<ComponentId> => "Data",
    }

    fn notification(&mut self, freed: ComponentId) {
        if self.data_source == Some(freed) {
            self.data_source = None;
        }
        if self.provider == Some(freed) {
            self.provider = None;
        }
    }

    fn message(&mut self, ctx: &mut EventCtx<'_>, message: &dyn Any) {
        // Os dados mudaram: as visualizações deste relatório montam de novo (a mensagem vem
        // para quem tem o `data_source`, que é o relatório).
        let Some(m) = data_message(message) else { return };
        if m.event == DataEvent::UpdateData {
            return;
        }
        refresh_previews(ctx.ui, ctx.id);
    }

    fn property_changed(&mut self, ctx: &mut EventCtx<'_>, name: &str) {
        // No designer, a largura acompanha a área útil da página.
        if matches!(name, "page_width" | "page_height" | "landscape" | "margin") {
            let width = (self.page_size().0 - self.margin * 2.0).max(50.0);
            let b = self.base.bounds();
            self.base.set_rect(b.x, b.y, width, b.height);
            ctx.invalidate();
        }
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        // A folha: branca, com a borda da área útil tracejada.
        let r = ctx.rect;
        canvas.fill_rect(r, 0.0, &Brush::Solid(Color::WHITE));
        canvas.stroke_rect(r, 0.0, &Stroke::new(ctx.theme.color("border"), 1.0).dashed(vec![4.0, 3.0]));
    }
}

thread_local! {
    /// Montando as páginas: o cursor do mestre anda (para o detalhe acompanhar) e os avisos
    /// disso não montam de novo.
    static PREPARING: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// Os dados mudaram: as visualizações do relatório `report` montam de novo (na fila).
fn refresh_previews(ui: &mut Ui, report: ComponentId) {
    if PREPARING.with(|p| p.get()) {
        return;
    }
    let previews: Vec<ComponentId> = ui.components().filter(|c| ui.get::<ReportPreview>(*c).is_some_and(|p| p.report == Some(report) && !p.pending.get())).collect();
    for id in previews {
        if let Some(p) = ui.get::<ReportPreview>(id) {
            p.pending.set(true);
        }
        ui.poster().post(move |ui| ReportPreview::refresh(ui, id));
    }
}

/// Uma banda do relatório: um contêiner com o tipo dela. `group_field`, nos cabeçalhos e
/// rodapés de grupo: o campo que forma o grupo; `new_page`: o grupo começa numa página nova.
/// `data_source`, nas bandas de sub-detalhe: o dataset do detalhe (ligado ao do relatório
/// pelo `master_source`); nos textos delas, um campo que o detalhe não tem vem do mestre.
pub struct ReportBand {
    pub base: ControlBase,
    pub kind: BandKind,
    pub group_field: String,
    pub new_page: bool,
    /// Fundo da banda (sem cor: transparente).
    pub fill: Option<Color>,
    pub data_source: Option<ComponentId>,
}

impl Default for ReportBand {
    fn default() -> Self {
        let mut base = ControlBase::new(523.0, 32.0);
        base.set_align(aylis_core::Align::Top);
        Self { base, kind: BandKind::Detail, group_field: String::new(), new_page: false, fill: None, data_source: None }
    }
}

impl ReportBand {
    pub fn new(kind: BandKind, height: f32) -> Self {
        let mut b = Self { kind, ..Default::default() };
        let r = b.base.bounds();
        b.base.set_rect(r.x, r.y, r.width, height);
        b
    }

    pub fn grouped_by(mut self, field: &str) -> Self {
        self.group_field = field.to_string();
        self
    }

    /// Uma banda de sub-detalhe com os registros de `data_source`.
    pub fn sub(kind: BandKind, height: f32, data_source: ComponentId) -> Self {
        Self { data_source: Some(data_source), ..Self::new(kind, height) }
    }
}

impl Component for ReportBand {
    fn class_name(&self) -> &'static str {
        "ReportBand"
    }

    fn control(&self) -> Option<&ControlBase> {
        Some(&self.base)
    }

    fn control_mut(&mut self) -> Option<&mut ControlBase> {
        Some(&mut self.base)
    }

    properties! {
        kind: BandKind => "Appearance",
        group_field: String => "Data",
        new_page: bool => "Behavior",
        fill: Option<Color> => "Appearance",
        data_source: Option<ComponentId> => "Data",
    }

    fn notification(&mut self, freed: ComponentId) {
        if self.data_source == Some(freed) {
            self.data_source = None;
        }
    }

    fn message(&mut self, ctx: &mut EventCtx<'_>, message: &dyn Any) {
        // Os dados do detalhe mudaram: o relatório monta de novo.
        let Some(m) = data_message(message) else { return };
        if m.event == DataEvent::UpdateData {
            return;
        }
        if let Some(report) = ctx.ui.parent(ctx.id) {
            refresh_previews(ctx.ui, report);
        }
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        if let Some(fill) = self.fill {
            canvas.fill_rect(r, 0.0, &Brush::Solid(fill));
        }
        if ctx.design {
            // No designer: o nome da banda e a divisa de baixo.
            let mut t = ctx.text_style();
            t.font.size = 9.0;
            t.color = ctx.theme.color("text_muted");
            t.h_align = TextAlign::Trailing;
            t.v_align = TextAlign::Leading;
            let name = if self.group_field.is_empty() { self.kind.title().to_string() } else { format!("{}: {}", self.kind.title(), self.group_field) };
            canvas.draw_text(&name, RectF::new(r.x, r.y + 1.0, r.width - 4.0, 12.0), &t);
            canvas.draw_line(PointF::new(r.x, r.bottom() - 0.5), PointF::new(r.right(), r.bottom() - 0.5), &Stroke::new(ctx.theme.color("primary").with_opacity(0.5), 1.0).dashed(vec![3.0, 3.0]));
        }
    }
}

// ---------------------------------------------------------------------------
// Montagem das páginas

/// Marca do total de páginas, trocada no fim (quando já se sabe quantas são).
const PAGES_MARK: &str = "\u{1}PAGES\u{1}";

/// O que uma banda vê ao ser impressa.
#[derive(Clone, Debug)]
struct Scope {
    /// O registro atual.
    row: Option<usize>,
    /// Os registros dos totais (`SUM`...).
    rows: std::ops::Range<usize>,
    page: usize,
    /// Nas bandas de sub-detalhe: o registro do mestre (os campos que o detalhe não tem).
    master: Option<usize>,
}

/// Uma banda e o que ela precisa.
#[derive(Clone, Debug)]
struct Band {
    id: ComponentId,
    kind: BandKind,
    field: String,
    new_page: bool,
    height: f32,
    /// O dataset dos textos (`None`: o do relatório).
    data_set: Option<ComponentId>,
}

/// Uma seção de sub-detalhe: o dataset do detalhe e as bandas dele.
struct SubSection {
    data_set: ComponentId,
    header: Option<Band>,
    detail: Band,
    footer: Option<Band>,
}

/// Os textos com colchetes dentro de `band` (o componente e o texto original).
fn templates(ui: &Ui, band: ComponentId) -> Vec<(ComponentId, String)> {
    let mut out = Vec::new();
    let mut stack = vec![band];
    while let Some(id) = stack.pop() {
        if id != band {
            if let Ok(PropValue::String(text)) = ui.get_prop(id, "text") {
                if text.contains('[') {
                    out.push((id, text));
                }
            }
        }
        stack.extend(ui.children(id).iter().copied());
    }
    out
}

/// O dia e a hora de agora, no fuso do aparelho.
fn now() -> (aylis_controls::Day, u32, u32) {
    let secs = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs()) as i64;
    let local = secs + aylis_core::platform::local_offset(secs);
    let in_day = local.rem_euclid(86_400);
    (aylis_controls::Day::from_days(local.div_euclid(86_400)), (in_day / 3600) as u32, (in_day % 3600 / 60) as u32)
}

/// Um número curto (os totais sem campo para formatar).
fn number(v: f64) -> String {
    if v.fract().abs() < 1e-9 { format!("{v:.0}") } else { format!("{v:.2}").replace('.', ",") }
}

/// Avalia o que está entre colchetes.
fn evaluate(ui: &Ui, data_set: Option<ComponentId>, master_set: Option<ComponentId>, expr: &str, scope: &Scope) -> String {
    let expr = expr.trim();
    let upper = expr.to_uppercase();
    match upper.as_str() {
        "PAGE" => return (scope.page + 1).to_string(),
        "PAGES" => return PAGES_MARK.to_string(),
        "DATE" => {
            let (d, _, _) = now();
            return format!("{:02}/{:02}/{}", d.day, d.month, d.year);
        }
        "TIME" => {
            let (_, h, m) = now();
            return format!("{h:02}:{m:02}");
        }
        "LINE" => return scope.row.map_or(String::new(), |r| (r + 1).to_string()),
        _ => {}
    }
    let Some(ds) = data_set else { return String::new() };
    let v = view(ui, ds);
    // Totais: SUM(campo), COUNT(), AVG, MIN, MAX (outras funções vão para as expressões).
    if let Some((name, _)) = upper.split_once('(').filter(|(n, _)| upper.ends_with(')') && matches!(n.trim(), "SUM" | "COUNT" | "AVG" | "MIN" | "MAX")) {
        let arg = expr[name.len() + 1..expr.len() - 1].trim();
        if name.trim() == "COUNT" {
            return scope.rows.len().to_string();
        }
        // Somados à medida que passam (sem juntar os valores numa lista: o total geral de um
        // relatório de milhões de registros não ocupa memória por registro).
        let (mut sum, mut count, mut min, mut max) = (0.0, 0usize, f64::INFINITY, f64::NEG_INFINITY);
        for x in scope.rows.clone().filter_map(|r| v.value_at(r, arg).ok().and_then(|x| x.as_f64())) {
            sum += x;
            count += 1;
            min = min.min(x);
            max = max.max(x);
        }
        let result = match name.trim() {
            "SUM" => Some(sum),
            "AVG" => (count > 0).then(|| sum / count as f64),
            "MIN" => (count > 0).then_some(min),
            _ => (count > 0).then_some(max),
        };
        let Some(result) = result else { return String::new() };
        // No formato do campo (moeda continua moeda).
        return v.format(arg, &Value::Float(result)).unwrap_or_else(|_| number(result));
    }
    // No sub-detalhe, o campo que o detalhe não tem vem do registro do mestre.
    let master = master_set.filter(|&m| m != ds).zip(scope.master).map(|(m, r)| (view(ui, m), r));
    let has = |names: Vec<String>, name: &str| names.iter().any(|f| f.eq_ignore_ascii_case(name));
    if let Some((mv, mrow)) = &master {
        if !has(v.field_names(), expr) && has(mv.field_names(), expr) {
            return mv.text_at(*mrow, expr).unwrap_or_default();
        }
    }
    let Some(row) = scope.row.or_else(|| scope.rows.clone().last()) else { return String::new() };
    if has(v.field_names(), expr) {
        return v.text_at(row, expr).unwrap_or_default();
    }
    match aylis_data::expr::Expr::parse(expr) {
        Ok(e) => e
            .eval(&|name| v.value_at(row, name).ok().or_else(|| master.as_ref().and_then(|(mv, mrow)| mv.value_at(*mrow, name).ok())))
            .map(|x| x.to_string())
            .unwrap_or_else(|e| format!("[{e}]")),
        Err(e) => format!("[{e}]"),
    }
}

/// Troca os `[...]` de `text` pelos valores.
fn fill_template(ui: &Ui, data_set: Option<ComponentId>, master_set: Option<ComponentId>, text: &str, scope: &Scope) -> String {
    let mut out = String::new();
    let mut rest = text;
    while let Some(start) = rest.find('[') {
        out.push_str(&rest[..start]);
        match rest[start + 1..].find(']') {
            Some(end) => {
                out.push_str(&evaluate(ui, data_set, master_set, &rest[start + 1..start + 1 + end], scope));
                rest = &rest[start + 2 + end..];
            }
            None => {
                out.push_str(&rest[start..]);
                rest = "";
            }
        }
    }
    out.push_str(rest);
    out
}

/// Monta as páginas do relatório `report` com os registros do dataset dele.
///
/// Com bandas de sub-detalhe, o cursor do mestre passa por cada registro (os detalhes ligados
/// pelo `master_source` acompanham) e volta para onde estava no fim.
pub fn prepare(ui: &mut Ui, report: ComponentId) -> PageSet {
    PREPARING.with(|p| p.set(true));
    let pages = prepare_pages(ui, report);
    PREPARING.with(|p| p.set(false));
    pages
}

fn prepare_pages(ui: &mut Ui, report: ComponentId) -> PageSet {
    build(ui, report, None, None).0
}

/// Monta as páginas. Com `sink`, cada página pronta vai para ele e não fica guardada (o
/// relatório grande), com o `[PAGES]` trocado por `total`. Devolve as páginas guardadas e
/// quantas foram montadas.
fn build(ui: &mut Ui, report: ComponentId, sink: Option<&mut dyn FnMut(Page)>, total: Option<usize>) -> (PageSet, usize) {
    let Some(r) = ui.get::<Report>(report) else { return (PageSet::default(), 0) };
    let (page_w, page_h) = r.page_size();
    let margin = r.margin;
    let data_set = r.data_source.and_then(|s| ui.get::<DataSource>(s)).and_then(|s| s.data_set);
    let bands: Vec<Band> = ui
        .children(report)
        .iter()
        .filter_map(|&id| {
            let b = ui.get::<ReportBand>(id)?;
            let band_set = b.data_source.and_then(|s| ui.get::<DataSource>(s)).and_then(|s| s.data_set);
            b.base.visible().then(|| Band {
                id,
                kind: b.kind,
                field: b.group_field.trim().to_string(),
                new_page: b.new_page,
                height: b.base.bounds().height,
                data_set: band_set,
            })
        })
        .collect();
    let find = |kind: BandKind| bands.iter().find(|b| b.kind == kind).cloned();
    // As seções de sub-detalhe, na ordem das bandas; cabeçalho e rodapé sem `data_source`
    // ficam com o primeiro sub-detalhe.
    let first_sub = bands.iter().find(|b| b.kind == BandKind::SubDetail).and_then(|b| b.data_set);
    let subs: Vec<SubSection> = bands
        .iter()
        .filter(|b| b.kind == BandKind::SubDetail && b.data_set.is_some() && b.data_set != data_set)
        .filter_map(|d| {
            let ds = d.data_set?;
            let of = |kind: BandKind| {
                bands.iter().find(|b| b.kind == kind && b.data_set.or(first_sub) == Some(ds)).map(|b| Band { data_set: Some(ds), ..b.clone() })
            };
            Some(SubSection { data_set: ds, header: of(BandKind::SubHeader), detail: d.clone(), footer: of(BandKind::SubFooter) })
        })
        .collect();
    // O mestre é percorrido por dentro (`browse_to`): as grades ligadas a ele não rolam junto.
    let saved_row = match (data_set, subs.is_empty()) {
        (Some(ds), false) => Some(aylis_data::data_set(ui, ds).browse_mark()),
        _ => None,
    };
    // Os grupos, de fora para dentro (a ordem dos cabeçalhos); o rodapé, pelo campo.
    let groups: Vec<(Band, Option<Band>)> = bands
        .iter()
        .filter(|b| b.kind == BandKind::GroupHeader)
        .map(|h| (h.clone(), bands.iter().find(|f| f.kind == BandKind::GroupFooter && f.field.eq_ignore_ascii_case(&h.field)).cloned()))
        .collect();
    let rows = data_set.map_or(0, |ds| view(ui, ds).record_count());
    let mut builder = Builder {
        sink,
        total,
        count: 0,
        pages: Vec::new(),
        page: Page::default(),
        y: 0.0,
        page_index: 0,
        page_first_row: 0,
        page_h,
        margin,
        data_set,
        footer: find(BandKind::PageFooter),
        header: find(BandKind::PageHeader),
        last_row: 0,
    };
    builder.start_page(ui, true);
    if let Some(title) = find(BandKind::Title) {
        builder.emit(ui, &title, Scope { row: None, rows: 0..rows, page: 0, master: None });
    }
    let key = |ui: &Ui, field: &str, row: usize| data_set.map_or(String::new(), |ds| view(ui, ds).value_at(row, field).map(|v| v.to_string()).unwrap_or_default());
    let mut keys: Vec<String> = vec![String::new(); groups.len()];
    let mut starts: Vec<usize> = vec![0; groups.len()];
    let detail = find(BandKind::Detail);
    let total = if data_set.is_some() { rows } else { usize::from(detail.is_some()) };
    for row in 0..total {
        builder.last_row = row;
        let current: Vec<String> = groups.iter().map(|(h, _)| key(ui, &h.field, row)).collect();
        // O primeiro nível que mudou: os de dentro dele fecham e abrem de novo.
        let changed = if row == 0 { Some(0) } else { (0..groups.len()).find(|&l| current[l] != keys[l]) };
        if let Some(level) = changed {
            if row > 0 {
                for l in (level..groups.len()).rev() {
                    if let Some(f) = &groups[l].1 {
                        builder.emit(ui, f, Scope { row: Some(row - 1), rows: starts[l]..row, page: builder.page_index, master: None });
                    }
                }
            }
            for l in level..groups.len() {
                starts[l] = row;
                let header = &groups[l].0;
                if header.new_page && row > 0 {
                    builder.finish_page(ui);
                    builder.start_page(ui, false);
                }
                builder.emit(ui, header, Scope { row: Some(row), rows: row..row + 1, page: builder.page_index, master: None });
            }
        }
        keys = current;
        if let Some(d) = &detail {
            builder.emit(ui, d, Scope { row: data_set.map(|_| row), rows: starts.first().copied().unwrap_or(0)..row + 1, page: builder.page_index, master: None });
        }
        // Os registros do detalhe deste registro do mestre.
        if let (Some(master), false) = (data_set, subs.is_empty()) {
            let _ = aylis_data::data_set(ui, master).browse_to(row);
            for sub in &subs {
                let count = view(ui, sub.data_set).record_count();
                if count == 0 {
                    continue;
                }
                let scope = |r: Option<usize>, rows: std::ops::Range<usize>| Scope { row: r, rows, page: 0, master: Some(row) };
                if let Some(h) = &sub.header {
                    builder.emit(ui, h, scope(Some(0), 0..count));
                }
                for r in 0..count {
                    builder.emit(ui, &sub.detail, scope(Some(r), 0..r + 1));
                }
                if let Some(f) = &sub.footer {
                    builder.emit(ui, f, scope(Some(count - 1), 0..count));
                }
            }
        }
    }
    if let (Some(master), Some(at)) = (data_set, saved_row) {
        let _ = aylis_data::data_set(ui, master).restore_browse(&at);
    }
    if total > 0 {
        for l in (0..groups.len()).rev() {
            if let Some(f) = &groups[l].1 {
                builder.emit(ui, f, Scope { row: Some(total - 1), rows: starts[l]..total, page: builder.page_index, master: None });
            }
        }
    }
    if let Some(summary) = find(BandKind::Summary) {
        builder.emit(ui, &summary, Scope { row: total.checked_sub(1), rows: 0..rows, page: builder.page_index, master: None });
    }
    builder.finish_page(ui);
    // O total de páginas, agora que se sabe.
    let count = builder.count;
    for page in &mut builder.pages {
        fill_pages(page, count);
    }
    ui.request_repaint();
    (PageSet { width: page_w, height: page_h, pages: builder.pages }, count)
}

/// Troca a marca do `[PAGES]` pelo total de páginas.
fn fill_pages(page: &mut Page, total: usize) {
    let count = total.to_string();
    for op in &mut page.ops {
        if let Op::Text { text, .. } = op {
            if text.contains(PAGES_MARK) {
                *text = text.replace(PAGES_MARK, &count);
            }
        }
    }
}

/// As páginas sendo montadas.
struct Builder<'s> {
    /// Quem recebe cada página pronta (o relatório grande); sem ele, ficam em `pages`.
    sink: Option<&'s mut dyn FnMut(Page)>,
    /// O total de páginas já sabido (a segunda passada do relatório grande).
    total: Option<usize>,
    count: usize,
    pages: Vec<Page>,
    page: Page,
    /// Onde a próxima banda entra, a partir do topo da área útil.
    y: f32,
    page_index: usize,
    page_first_row: usize,
    page_h: f32,
    margin: f32,
    data_set: Option<ComponentId>,
    header: Option<Band>,
    footer: Option<Band>,
    last_row: usize,
}

impl Builder<'_> {
    fn usable(&self) -> f32 {
        self.page_h - self.margin * 2.0 - self.footer.as_ref().map_or(0.0, |f| f.height)
    }

    fn start_page(&mut self, ui: &mut Ui, first: bool) {
        if !first {
            self.page_index += 1;
        }
        self.page_first_row = self.last_row;
        self.y = 0.0;
        if let Some(h) = self.header.clone() {
            let scope = Scope { row: Some(self.last_row), rows: self.last_row..self.last_row + 1, page: self.page_index, master: None };
            self.paint(ui, &h, scope);
        }
    }

    fn finish_page(&mut self, ui: &mut Ui) {
        if let Some(f) = self.footer.clone() {
            let scope = Scope { row: Some(self.last_row), rows: self.page_first_row..self.last_row + 1, page: self.page_index, master: None };
            self.y = self.usable();
            self.paint(ui, &f, scope);
        }
        let mut page = std::mem::take(&mut self.page);
        self.count += 1;
        if let Some(total) = self.total {
            fill_pages(&mut page, total);
        }
        match self.sink.as_mut() {
            Some(sink) => sink(page),
            None => self.pages.push(page),
        }
    }

    /// Põe a banda na página (se não couber, numa página nova).
    fn emit(&mut self, ui: &mut Ui, band: &Band, scope: Scope) {
        if self.y + band.height > self.usable() && self.y > 0.0 {
            self.finish_page(ui);
            self.start_page(ui, false);
        }
        let scope = Scope { page: self.page_index, ..scope };
        self.paint(ui, band, scope);
    }

    /// Desenha a banda em `y`, com os textos preenchidos.
    fn paint(&mut self, ui: &mut Ui, band: &Band, scope: Scope) {
        let originals = templates(ui, band.id);
        for (id, text) in &originals {
            let filled = fill_template(ui, band.data_set.or(self.data_set), self.data_set, text, &scope);
            let _ = ui.set_prop(*id, "text", filled);
        }
        let bounds = ui.control(band.id).map(|c| c.bounds()).unwrap_or_default();
        let mut canvas = RecordingCanvas::new(None);
        canvas.translate(self.margin - bounds.x, self.margin + self.y - bounds.y);
        ui.paint(band.id, &mut canvas);
        self.page.ops.push(Op::Save);
        self.page.ops.extend(canvas.ops);
        self.page.ops.push(Op::Restore);
        for (id, text) in originals {
            let _ = ui.set_prop(id, "text", text);
        }
        self.y += band.height;
    }
}

/// As imagens guardadas como arquivo (PNG, JPEG...) viram pixels (para o PDF e a impressora).
fn decode_images(pages: &mut PageSet) {
    for page in &mut pages.pages {
        for op in &mut page.ops {
            if let Op::Image { image, .. } = op {
                if let ImageSource::Encoded(bytes) = image.source() {
                    if let Ok(img) = image::load_from_memory(bytes) {
                        let rgba = img.to_rgba8();
                        *image = aylis_core::Image::rgba(rgba.width(), rgba.height(), rgba.into_raw());
                    }
                }
            }
        }
    }
}

/// O PDF do relatório: pelo provedor de PDF ligado em `provider`, ou pelo do Aylis.
pub fn to_pdf(ui: &mut Ui, report: ComponentId) -> Vec<u8> {
    let mut pages = prepare(ui, report);
    decode_images(&mut pages);
    let provider = ui.get::<Report>(report).and_then(|r| r.provider);
    match provider.and_then(|p| ui.provided(p)) {
        Some(aylis_core::providers::Provided::Pdf(pdf)) => pdf.write(&pages).unwrap_or_else(|e| {
            eprintln!("aylis: Report: o provedor de PDF falhou ({e}); usando o PDF do Aylis");
            pages.to_pdf()
        }),
        _ => pages.to_pdf(),
    }
}

/// O PDF do relatório grande, gravado em `out` página por página: as páginas não ficam na
/// memória (com um milhão de registros, o `to_pdf` guardaria todas). Com `[PAGES]` nos
/// textos, monta duas vezes (a primeira só conta as páginas, como o FastReport). Usa o PDF
/// do Aylis (um provedor de PDF recebe as páginas todas: use o `to_pdf`). Devolve quantas
/// páginas gravou.
pub fn write_pdf(ui: &mut Ui, report: ComponentId, out: impl std::io::Write) -> std::io::Result<usize> {
    let (width, height) = ui.get::<Report>(report).map_or((595.0, 842.0), |r| r.page_size());
    let needs_total = descendants(ui, report).into_iter().any(|id| matches!(ui.get_prop(id, "text"), Ok(PropValue::String(t)) if t.to_uppercase().contains("[PAGES]")));
    PREPARING.with(|p| p.set(true));
    let total = if needs_total {
        let mut discard = |_: Page| {};
        Some(build(ui, report, Some(&mut discard), None).1)
    } else {
        None
    };
    let mut writer = aylis_core::pages::PdfWriter::new(out, width, height, None)?;
    let mut failed = None;
    {
        let mut write = |page: Page| {
            // As imagens desta página viram pixels (como no `to_pdf`), uma página por vez.
            let mut one = PageSet { width, height, pages: vec![page] };
            decode_images(&mut one);
            if failed.is_none() {
                if let Err(e) = writer.add_page(&one.pages[0]) {
                    failed = Some(e);
                }
            }
        };
        build(ui, report, Some(&mut write), total);
    }
    PREPARING.with(|p| p.set(false));
    if let Some(e) = failed {
        return Err(e);
    }
    let pages = writer.pages();
    writer.finish()?;
    Ok(pages)
}

/// O componente e todos os de dentro.
fn descendants(ui: &Ui, root: ComponentId) -> Vec<ComponentId> {
    let mut out = vec![root];
    let mut i = 0;
    while i < out.len() {
        out.extend(ui.children(out[i]).iter().copied());
        i += 1;
    }
    out
}

/// Imprime o relatório (o diálogo da impressora do sistema). Sem o serviço, `Err`.
pub fn print(ui: &mut Ui, report: ComponentId, done: impl FnOnce(Result<bool, String>) + Send + 'static) {
    let mut pages = prepare(ui, report);
    decode_images(&mut pages);
    let title = ui.get::<Report>(report).map(|r| r.title.clone()).unwrap_or_default();
    match ui.services_mut().printer.as_mut() {
        Some(p) => p.print(&title, pages, Box::new(done)),
        None => done(Err("sem impressora nesta plataforma".into())),
    }
}

// ---------------------------------------------------------------------------
// Visualização

/// Os botões da barra da visualização.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tool {
    First,
    Previous,
    Next,
    Last,
    ZoomOut,
    ZoomIn,
    Fit,
    Pdf,
    Print,
}

/// A visualização de um [`Report`] (como o preview do FastReport): uma página por vez, com
/// a barra para navegar (`page`), aproximar (`zoom`; 0 = a página inteira na largura),
/// exportar para PDF e imprimir. Atualiza sozinha quando os dados mudam; por código,
/// [`ReportPreview::refresh`].
pub struct ReportPreview {
    pub base: ControlBase,
    pub report: Option<ComponentId>,
    pub page: i64,
    pub zoom: f32,
    /// Mostra a barra de botões.
    pub toolbar: bool,
    pages: Option<Arc<PageSet>>,
    pending: std::cell::Cell<bool>,
    tools: RefCell<Vec<(RectF, Tool)>>,
}

impl Default for ReportPreview {
    fn default() -> Self {
        Self { base: ControlBase::new(420.0, 520.0), report: None, page: 0, zoom: 0.0, toolbar: true, pages: None, pending: std::cell::Cell::new(false), tools: RefCell::new(Vec::new()) }
    }
}

const BAR: f32 = 40.0;

impl ReportPreview {
    pub fn new(report: ComponentId) -> Self {
        Self { report: Some(report), ..Default::default() }
    }

    /// Monta as páginas de novo (na fila: logo depois do evento atual).
    pub fn refresh(ui: &mut Ui, id: ComponentId) {
        let Some(report) = ui.get::<ReportPreview>(id).and_then(|p| p.report) else { return };
        let pages = prepare(ui, report);
        if let Some(p) = ui.get_mut::<ReportPreview>(id) {
            p.page = p.page.clamp(0, pages.pages.len().saturating_sub(1) as i64);
            p.pages = Some(Arc::new(pages));
            p.pending.set(false);
        }
        ui.request_repaint();
    }

    /// As páginas prontas (depois do primeiro [`ReportPreview::refresh`]).
    pub fn pages(&self) -> Option<&PageSet> {
        self.pages.as_deref()
    }

    fn schedule(&mut self, ctx: &mut EventCtx<'_>) {
        if self.pending.replace(true) {
            return;
        }
        let id = ctx.id;
        ctx.ui.poster().post(move |ui| ReportPreview::refresh(ui, id));
    }

    fn count(&self) -> usize {
        self.pages.as_ref().map_or(0, |p| p.pages.len())
    }

    fn use_tool(&mut self, ctx: &mut EventCtx<'_>, tool: Tool) {
        let last = self.count().saturating_sub(1) as i64;
        match tool {
            Tool::First => self.page = 0,
            Tool::Previous => self.page = (self.page - 1).max(0),
            Tool::Next => self.page = (self.page + 1).min(last),
            Tool::Last => self.page = last,
            Tool::ZoomOut => self.zoom = (self.current_zoom() / 1.25).max(0.2),
            Tool::ZoomIn => self.zoom = (self.current_zoom() * 1.25).min(5.0),
            Tool::Fit => self.zoom = 0.0,
            Tool::Pdf => {
                let Some(report) = self.report else { return };
                let poster = ctx.ui.poster();
                let title = ctx.ui.get::<Report>(report).map(|r| r.title.clone()).unwrap_or_default();
                let options = aylis_core::platform::FileDialogOptions {
                    title: "Exportar para PDF".into(),
                    filters: vec![("PDF".into(), vec!["pdf".into()])],
                    file_name: format!("{}.pdf", if title.is_empty() { "relatorio" } else { &title }),
                    save: true,
                    ..Default::default()
                };
                let id = ctx.id;
                if let Some(dialogs) = ctx.ui.services_mut().file_dialogs.as_mut() {
                    dialogs.show(
                        options,
                        Box::new(move |paths| {
                            let Some(path) = paths.into_iter().next() else { return };
                            poster.post(move |ui| {
                                let pdf = to_pdf(ui, report);
                                let result = std::fs::write(&path, pdf).map_err(|e| format!("{path}: {e}"));
                                ui.fire(id, "on_export", &EventArgs::Text(result.map_or_else(|e| e, |_| path)));
                            });
                        }),
                    );
                }
            }
            Tool::Print => {
                let Some(report) = self.report else { return };
                let poster = ctx.ui.poster();
                let id = ctx.id;
                ctx.ui.poster().post(move |ui| {
                    print(ui, report, move |result| {
                        poster.post(move |ui| {
                            let text = match result {
                                Ok(true) => "impresso".to_string(),
                                Ok(false) => "cancelado".to_string(),
                                Err(e) => e,
                            };
                            ui.fire(id, "on_print", &EventArgs::Text(text));
                        });
                    })
                });
            }
        }
        ctx.repaint();
    }

    /// O zoom em uso (com 0, o que faz a página caber na largura).
    fn current_zoom(&self) -> f32 {
        if self.zoom > 0.0 {
            return self.zoom;
        }
        let Some(p) = &self.pages else { return 1.0 };
        let b = self.base.bounds();
        let (w, h) = (b.width - 32.0, b.height - if self.toolbar { BAR } else { 0.0 } - 32.0);
        (w / p.width).min(h / p.height).max(0.1)
    }
}

impl Component for ReportPreview {
    fn class_name(&self) -> &'static str {
        "ReportPreview"
    }

    fn control(&self) -> Option<&ControlBase> {
        Some(&self.base)
    }

    fn control_mut(&mut self) -> Option<&mut ControlBase> {
        Some(&mut self.base)
    }

    properties! {
        report: Option<ComponentId> => "Data",
        page: i64 => "Behavior",
        zoom: f32 => "Behavior",
        toolbar: bool => "Appearance";
        events: on_export, on_print
    }

    fn notification(&mut self, freed: ComponentId) {
        if self.report == Some(freed) {
            self.report = None;
            self.pages = None;
        }
    }

    fn property_changed(&mut self, ctx: &mut EventCtx<'_>, name: &str) {
        if name == "report" {
            self.schedule(ctx);
        }
        ctx.repaint();
    }

    fn pointer_down(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        let tool = self.tools.borrow().iter().find(|(r, _)| r.contains(event.position)).map(|(_, t)| *t);
        if let Some(t) = tool {
            self.use_tool(ctx, t);
        }
    }

    fn wheel(&mut self, ctx: &mut EventCtx<'_>, event: &WheelEvent) {
        if event.modifiers.ctrl {
            self.use_tool(ctx, if event.delta.y > 0.0 { Tool::ZoomIn } else { Tool::ZoomOut });
        } else {
            self.use_tool(ctx, if event.delta.y > 0.0 { Tool::Previous } else { Tool::Next });
        }
        ctx.set_handled();
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let theme = ctx.theme;
        canvas.fill_rect(r, 0.0, &Brush::Solid(theme.color("surface_2")));
        let mut tools = self.tools.borrow_mut();
        tools.clear();
        let mut text = ctx.text_style();
        text.h_align = TextAlign::Center;
        let mut top = r.y;
        if self.toolbar {
            let bar = RectF::new(r.x, r.y, r.width, BAR);
            canvas.fill_rect(bar, 0.0, &Brush::Solid(theme.color("surface")));
            canvas.draw_line(PointF::new(r.x, bar.bottom() - 0.5), PointF::new(r.right(), bar.bottom() - 0.5), &Stroke::new(theme.color("border"), 1.0));
            let count = self.count();
            let at = self.page.max(0) as usize;
            let label = if count == 0 { "sem páginas".to_string() } else { format!("{} de {count}", at + 1) };
            // (texto, largura, botão e se está habilitado); sem botão: texto solto; vazio: espaço.
            let items: Vec<(&str, f32, Option<Tool>, bool)> = vec![
                ("«", 30.0, Some(Tool::First), at > 0),
                ("‹", 30.0, Some(Tool::Previous), at > 0),
                (label.as_str(), canvas.measure_text(&label, &text).width + 16.0, None, true),
                ("›", 30.0, Some(Tool::Next), at + 1 < count),
                ("»", 30.0, Some(Tool::Last), at + 1 < count),
                ("", 8.0, None, true),
                ("−", 30.0, Some(Tool::ZoomOut), count > 0),
                ("+", 30.0, Some(Tool::ZoomIn), count > 0),
                ("Ajustar", 64.0, Some(Tool::Fit), count > 0),
                ("", 8.0, None, true),
                ("PDF…", 56.0, Some(Tool::Pdf), count > 0),
                ("Imprimir…", 82.0, Some(Tool::Print), count > 0),
            ];
            let mut x = r.x + 8.0;
            for (caption, width, tool, enabled) in items {
                let rect = RectF::new(x, r.y + 6.0, width, BAR - 12.0);
                let mut t = text.clone();
                t.color = if enabled { theme.color("text") } else { theme.color("text_muted") };
                if let Some(tool) = tool {
                    canvas.fill_rect(rect, 6.0, &Brush::Solid(theme.color("surface_2")));
                    if enabled {
                        tools.push((rect, tool));
                    }
                }
                canvas.draw_text(caption, rect, &t);
                x += width + 6.0;
            }
            top += BAR;
        }
        let Some(pages) = &self.pages else {
            // Criada pelo código já com o relatório: monta no primeiro desenho.
            if self.report.is_some() && !self.pending.replace(true) {
                let id = ctx.id;
                ctx.ui.poster().post(move |ui| ReportPreview::refresh(ui, id));
            }
            let mut t = text.clone();
            t.color = theme.color("text_muted");
            let message = if self.report.is_none() { "ReportPreview: escolha o relatório em report" } else { "montando…" };
            canvas.draw_text(message, RectF::new(r.x, top, r.width, r.bottom() - top), &t);
            return;
        };
        let Some(page) = pages.pages.get(self.page.max(0) as usize) else { return };
        let zoom = self.current_zoom();
        let (w, h) = (pages.width * zoom, pages.height * zoom);
        let x = r.x + ((r.width - w) / 2.0).max(16.0);
        let y = top + 16.0;
        let sheet = RectF::new(x, y, w, h);
        canvas.save();
        canvas.clip_rect(RectF::new(r.x, top, r.width, r.bottom() - top));
        canvas.fill_rect(sheet.offset(0.0, 3.0), 2.0, &Brush::Solid(Color::BLACK.with_opacity(0.15)));
        canvas.fill_rect(sheet, 0.0, &Brush::Solid(Color::WHITE));
        canvas.save();
        canvas.clip_rect(sheet);
        canvas.translate(x, y);
        canvas.scale(zoom, zoom);
        page.replay(canvas);
        canvas.restore();
        canvas.restore();
    }
}
