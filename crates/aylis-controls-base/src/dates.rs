//! Datas e horas: Calendar, DateEdit (campo com calendário suspenso) e TimeEdit.
//!
//! As propriedades guardam datas como texto ISO (`2026-10-24`) e horas como `14:30`
//! (ou `14:30:15`): o mesmo no JSON, no protocolo e no código. A tela mostra no formato
//! do idioma (`24/10/2026` em pt-BR).

use std::any::Any;

use aylis_core::{
    Brush, Canvas, Color, Component, ComponentId, ControlBase, CursorShape, EventArgs, EventCtx, Key, KeyEvent,
    PaintCtx, PointF, PointerButton, PointerEvent, RectF, Stroke, TextAlign, WheelEvent, properties,
};

use crate::{Edit, leaf_control};

// ---------------------------------------------------------------------------
// Data simples (sem fuso: o calendário civil)

/// Uma data do calendário.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Day {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

impl Day {
    pub fn new(year: i32, month: u32, day: u32) -> Option<Self> {
        ((1..=12).contains(&month) && day >= 1 && day <= days_in_month(year, month)).then_some(Self { year, month, day })
    }

    /// `2026-10-24`.
    pub fn parse_iso(text: &str) -> Option<Self> {
        let mut parts = text.trim().splitn(3, '-');
        let y = parts.next()?.parse().ok()?;
        let m = parts.next()?.parse().ok()?;
        let d = parts.next()?.get(..2).unwrap_or("").trim_end_matches(|c: char| !c.is_ascii_digit()).parse().ok()?;
        Self::new(y, m, d)
    }

    pub fn iso(&self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }

    /// Hoje, no fuso do aparelho (a plataforma informa o fuso; sem ela, em UTC).
    pub fn today() -> Self {
        // web_time: no navegador o std::time::SystemTime não existe (entra em pânico).
        let secs = web_time::SystemTime::now().duration_since(web_time::UNIX_EPOCH).map_or(0, |d| d.as_secs()) as i64;
        Self::from_days((secs + aylis_core::platform::local_offset(secs)).div_euclid(86_400))
    }

    /// Dias desde 1970-01-01.
    pub fn days(&self) -> i64 {
        // Algoritmo de Howard Hinnant (calendário gregoriano proléptico).
        let (y, m, d) = (self.year as i64 - i64::from(self.month <= 2), self.month as i64, self.day as i64);
        let era = y.div_euclid(400);
        let yoe = y - era * 400;
        let doy = (153 * (m + if m > 2 { -3 } else { 9 }) + 2) / 5 + d - 1;
        let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
        era * 146_097 + doe - 719_468
    }

    pub fn from_days(days: i64) -> Self {
        let z = days + 719_468;
        let era = z.div_euclid(146_097);
        let doe = z - era * 146_097;
        let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
        let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
        let mp = (5 * doy + 2) / 153;
        let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
        let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
        let y = (yoe + era * 400 + i64::from(m <= 2)) as i32;
        Self { year: y, month: m, day: d }
    }

    /// Dia da semana: 0 = domingo.
    pub fn weekday(&self) -> u32 {
        (self.days() + 4).rem_euclid(7) as u32
    }

    pub fn add_days(&self, n: i64) -> Self {
        Self::from_days(self.days() + n)
    }

    /// Soma meses (o dia fica no último do mês se não existir: 31/01 + 1 = 28/02).
    pub fn add_months(&self, n: i32) -> Self {
        let total = self.year * 12 + self.month as i32 - 1 + n;
        let (year, month) = (total.div_euclid(12), total.rem_euclid(12) as u32 + 1);
        Self { year, month, day: self.day.min(days_in_month(year, month)) }
    }

    /// No formato do idioma (`dd/mm/yyyy`, `mm/dd/yyyy`, `yyyy-mm-dd`).
    pub fn format(&self, pattern: &str) -> String {
        pattern
            .to_lowercase()
            .replace("yyyy", &format!("{:04}", self.year))
            .replace("mm", &format!("{:02}", self.month))
            .replace("dd", &format!("{:02}", self.day))
    }

    /// Lê no formato do idioma (aceita `/`, `-` ou `.`; ano com 2 dígitos vira 20xx).
    pub fn parse(text: &str, pattern: &str) -> Option<Self> {
        let nums: Vec<i64> = text.split(['/', '-', '.', ' ']).filter(|s| !s.is_empty()).map(|s| s.parse().ok()).collect::<Option<_>>()?;
        if nums.len() != 3 {
            return None;
        }
        let order: Vec<char> = pattern.to_lowercase().chars().filter(|c| matches!(c, 'y' | 'm' | 'd')).fold(Vec::new(), |mut v, c| {
            if v.last() != Some(&c) {
                v.push(c);
            }
            v
        });
        let get = |k: char| order.iter().position(|&c| c == k).map(|i| nums[i]);
        let mut y = get('y')?;
        if y < 100 {
            y += 2000;
        }
        Self::new(y as i32, get('m')? as u32, get('d')? as u32)
    }
}

pub fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 => 29,
        2 => 28,
        _ => 0,
    }
}

// ---------------------------------------------------------------------------
// Calendar

const HEADER: f32 = 36.0;
const NAMES: f32 = 24.0;

/// Calendário de um mês (como o `TCalendar`): clique num dia escolhe; ‹ › trocam o mês;
/// setas andam pelos dias, PageUp/PageDown pelos meses. `on_change` traz a data ISO.
pub struct Calendar {
    pub base: ControlBase,
    /// Data escolhida (ISO, `2026-10-24`); vazia = nenhuma.
    pub date: String,
    /// Intervalo (o `v-date-picker` com `multiple="range"`): o 1º clique é `date`, o 2º
    /// `end_date`; `on_change` recebe `início;fim`.
    pub range: bool,
    pub end_date: String,
    /// Escolhe o mês (o `v-month-picker`): `date` fica no dia 1.
    pub month_mode: bool,
    /// Dias com eventos (ISO separados por `;`): ganham um ponto embaixo do número.
    pub events: String,
    /// Mês mostrado (primeiro dia).
    shown: Day,
    hovered: Option<Day>,
    /// Aberto por um DateEdit: avisa ele em vez de disparar `on_change`.
    owner: Option<ComponentId>,
}

impl Default for Calendar {
    fn default() -> Self {
        let mut base = ControlBase::new(280.0, 280.0);
        base.set_can_focus(true).set_cursor(CursorShape::Pointer);
        let today = Day::today();
        Self {
            base,
            date: String::new(),
            range: false,
            end_date: String::new(),
            month_mode: false,
            events: String::new(),
            shown: Day { day: 1, ..today },
            hovered: None,
            owner: None,
        }
    }
}

/// Mensagem do calendário suspenso para o DateEdit.
#[derive(Debug)]
struct PickedDay(Day);

impl Calendar {
    pub fn new(date: &str) -> Self {
        let mut c = Self { date: date.to_string(), ..Default::default() };
        c.sync("date");
        c
    }

    /// A data escolhida.
    pub fn selected(&self) -> Option<Day> {
        Day::parse_iso(&self.date)
    }

    fn sync(&mut self, _name: &str) {
        if let Some(d) = self.selected() {
            self.shown = Day { day: 1, ..d };
        }
    }

    fn grid(&self) -> (RectF, f32, f32) {
        let b = self.base.bounds();
        let area = RectF::new(6.0, HEADER + NAMES, b.width - 12.0, (b.height - HEADER - NAMES - 6.0).max(0.0));
        (area, area.width / 7.0, area.height / 6.0)
    }

    /// Primeira célula da grade (pode ser do mês anterior).
    fn first_cell(&self, first_weekday: u8) -> Day {
        let back = (self.shown.weekday() as i64 - first_weekday as i64).rem_euclid(7);
        self.shown.add_days(-back)
    }

    fn day_at(&self, p: PointF, first_weekday: u8) -> Option<Day> {
        let (area, cw, ch) = self.grid();
        if !area.contains(p) {
            return None;
        }
        let col = ((p.x - area.x) / cw) as i64;
        let row = ((p.y - area.y) / ch) as i64;
        Some(self.first_cell(first_weekday).add_days(row * 7 + col))
    }

    /// O mês na posição (modo mês: 3 colunas × 4 linhas).
    fn month_at(&self, p: PointF) -> Option<Day> {
        let b = self.base.bounds();
        let area = RectF::new(6.0, HEADER, b.width - 12.0, (b.height - HEADER - 6.0).max(0.0));
        if !area.contains(p) {
            return None;
        }
        let col = ((p.x - area.x) / (area.width / 3.0)) as u32;
        let row = ((p.y - area.y) / (area.height / 4.0)) as u32;
        Day::new(self.shown.year, (row * 3 + col + 1).min(12), 1)
    }

    fn choose(&mut self, ctx: &mut EventCtx<'_>, day: Day) {
        if self.range && self.owner.is_none() {
            let iso = day.iso();
            if self.date.is_empty() || !self.end_date.is_empty() {
                self.date = iso;
                self.end_date.clear();
            } else if iso < self.date {
                self.end_date = std::mem::replace(&mut self.date, iso);
            } else {
                self.end_date = iso;
            }
            ctx.fire("on_change", EventArgs::Text(format!("{};{}", self.date, self.end_date)));
            ctx.repaint();
            return;
        }
        self.date = day.iso();
        self.shown = Day { day: 1, ..day };
        match self.owner {
            Some(owner) => {
                ctx.ui.send_message(owner, &PickedDay(day));
            }
            None => ctx.fire("on_change", EventArgs::Text(self.date.clone())),
        }
        ctx.repaint();
    }
}

impl Component for Calendar {
    // O `on_change` do valor mudado pelo código leva o mesmo que o do usuário.
    fn change_args(&self, _name: &str, _value: &aylis_core::PropValue) -> Option<EventArgs> {
        match (self.owner, self.range) {
            (Some(_), _) => None,
            (None, true) => Some(EventArgs::Text(format!("{};{}", self.date, self.end_date))),
            (None, false) => Some(EventArgs::Text(self.date.clone())),
        }
    }

    leaf_control!("Calendar");

    properties! {
        date: String => "Data",
        range: bool => "Behavior",
        end_date: String => "Data",
        month_mode: bool => "Behavior",
        events: String => "Data";
        events: on_change;
        on_set: sync
    }

    // Para leitores de tela e automação: os botões de mês anterior e próximo e cada dia da
    // grade (o nome é a data ISO, `2026-10-27`; no modo mês, o nome do mês).
    fn access_items(&self, ui: &aylis_core::Ui, _: ComponentId) -> Vec<aylis_core::automation::AccessItem> {
        use aylis_core::automation::{AccessItem, ItemRole};
        let w = self.base.bounds().width;
        let mut out = vec![
            AccessItem::new(ItemRole::Button, ui.tr(if self.month_mode { "Ano anterior" } else { "Mês anterior" })).rect(RectF::new(0.0, 0.0, 40.0, HEADER)),
            AccessItem::new(ItemRole::Button, ui.tr(if self.month_mode { "Próximo ano" } else { "Próximo mês" })).rect(RectF::new(w - 40.0, 0.0, 40.0, HEADER)),
        ];
        let chosen = self.selected();
        if self.month_mode {
            let names = ui.locale().month_names;
            for m in 0..12u32 {
                let selected = chosen.is_some_and(|d| d.year == self.shown.year && d.month == m + 1);
                out.push(AccessItem::new(ItemRole::Cell, names[m as usize].to_string()).selected(selected));
            }
            return out;
        }
        let (area, cw, ch) = self.grid();
        let first = self.first_cell(ui.locale().first_day_of_week);
        let end = Day::parse_iso(&self.end_date);
        for i in 0..42i64 {
            let day = first.add_days(i);
            let selected = chosen == Some(day) || (self.range && chosen.zip(end).is_some_and(|(a, b)| day >= a && day <= b));
            let rect = RectF::new(area.x + (i % 7) as f32 * cw, area.y + (i / 7) as f32 * ch, cw, ch);
            out.push(AccessItem::new(ItemRole::Cell, day.iso()).selected(selected).cell((i / 7) as usize, (i % 7) as usize).rect(rect));
        }
        out
    }

    fn access_action(&mut self, ctx: &mut EventCtx<'_>, index: usize, _: aylis_core::automation::ItemAction) -> bool {
        let step = if self.month_mode { 12 } else { 1 };
        match index {
            0 => self.shown = self.shown.add_months(-step),
            1 => self.shown = self.shown.add_months(step),
            i if self.month_mode => {
                let Some(m) = Day::new(self.shown.year, (i - 2 + 1).min(12) as u32, 1) else { return false };
                self.choose(ctx, m);
            }
            i if i < 44 => {
                let first = self.first_cell(ctx.ui.locale().first_day_of_week);
                self.choose(ctx, first.add_days(i as i64 - 2));
            }
            _ => return false,
        }
        ctx.repaint();
        true
    }

    fn pointer_down(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if event.button != PointerButton::Primary {
            return;
        }
        let w = self.base.bounds().width;
        let step = if self.month_mode { 12 } else { 1 };
        if event.position.y < HEADER {
            if event.position.x < 40.0 {
                self.shown = self.shown.add_months(-step);
            } else if event.position.x > w - 40.0 {
                self.shown = self.shown.add_months(step);
            }
            ctx.repaint();
            return;
        }
        if self.month_mode {
            if let Some(m) = self.month_at(event.position) {
                self.choose(ctx, m);
            }
            return;
        }
        let first = ctx.ui.locale().first_day_of_week;
        if let Some(day) = self.day_at(event.position, first) {
            self.choose(ctx, day);
        }
    }

    fn pointer_move(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        let first = ctx.ui.locale().first_day_of_week;
        let hovered = self.day_at(event.position, first);
        if hovered != self.hovered {
            self.hovered = hovered;
            ctx.repaint();
        }
    }

    fn pointer_leave(&mut self, ctx: &mut EventCtx<'_>) {
        self.hovered = None;
        ctx.repaint();
    }

    fn wheel(&mut self, ctx: &mut EventCtx<'_>, event: &WheelEvent) {
        if event.touch {
            return;
        }
        self.shown = self.shown.add_months(if event.delta.y > 0.0 { -1 } else { 1 });
        ctx.repaint();
        ctx.set_handled();
    }

    fn key_down(&mut self, ctx: &mut EventCtx<'_>, event: &KeyEvent) {
        let current = self.selected().unwrap_or(self.shown);
        let next = match event.key {
            Key::Left => current.add_days(-1),
            Key::Right => current.add_days(1),
            Key::Up => current.add_days(-7),
            Key::Down => current.add_days(7),
            Key::PageUp => current.add_months(-1),
            Key::PageDown => current.add_months(1),
            Key::Home => Day::today(),
            Key::Enter | Key::Space if self.owner.is_some() => current,
            _ => return,
        };
        self.choose(ctx, next);
        ctx.set_handled();
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let s = ctx.style;
        let r = ctx.rect;
        let theme = ctx.theme;
        if self.owner.is_some() {
            canvas.fill_rect(r.offset(0.0, 3.0).inflate(1.0), s.radius + 1.0, &Brush::Solid(Color::BLACK.with_opacity(0.14)));
        }
        canvas.fill_rect(r, s.radius, &Brush::Solid(s.fill));
        if s.border_width > 0.0 {
            canvas.stroke_rect(r, s.radius, &Stroke::new(s.border_color, s.border_width));
        }
        let locale = ctx.ui.locale();
        let mut text = ctx.text_style();
        text.h_align = TextAlign::Center;
        // Cabeçalho: ‹ março de 2026 › (no modo mês, só o ano).
        let month = locale.month_names[(self.shown.month - 1) as usize];
        let mut title = text.clone();
        title.font.weight = aylis_core::FontWeight::Bold;
        let heading = if self.month_mode { self.shown.year.to_string() } else { format!("{month} {}", self.shown.year) };
        canvas.draw_text(&heading, RectF::new(r.x + 40.0, r.y, r.width - 80.0, HEADER), &title);
        let arrow = Stroke::new(theme.color("text_muted"), 1.8);
        for (x, d) in [(r.x + 20.0, 1.0f32), (r.right() - 20.0, -1.0)] {
            let cy = r.y + HEADER / 2.0;
            canvas.draw_line(PointF::new(x + 3.0 * d, cy - 5.0), PointF::new(x - 2.0 * d, cy), &arrow);
            canvas.draw_line(PointF::new(x - 2.0 * d, cy), PointF::new(x + 3.0 * d, cy + 5.0), &arrow);
        }
        if self.month_mode {
            let area = RectF::new(r.x + 6.0, r.y + HEADER, r.width - 12.0, (r.height - HEADER - 6.0).max(0.0));
            let (cw, ch) = (area.width / 3.0, area.height / 4.0);
            let chosen = self.selected().map(|d| (d.year, d.month));
            for m in 0..12u32 {
                let rect = RectF::new(area.x + (m % 3) as f32 * cw, area.y + (m / 3) as f32 * ch, cw, ch).inflate(-4.0);
                let mut style = text.clone();
                if chosen == Some((self.shown.year, m + 1)) {
                    canvas.fill_rect(rect, rect.height / 2.0, &Brush::Solid(theme.color("primary")));
                    style.color = theme.color("on_primary");
                }
                let name: String = locale.month_names[m as usize].chars().take(3).collect();
                canvas.draw_text(&name, rect, &style);
            }
            return;
        }
        let (area, cw, ch) = self.grid();
        let area = area.offset(r.x, r.y);
        let mut small = text.clone();
        small.font.size = (text.font.size - 2.0).max(9.0);
        small.color = theme.color("text_muted");
        for i in 0..7 {
            let name = locale.short_day_names[(i + locale.first_day_of_week as usize) % 7];
            canvas.draw_text(name, RectF::new(area.x + i as f32 * cw, r.y + HEADER, cw, NAMES), &small);
        }
        let first = self.first_cell(locale.first_day_of_week);
        let today = Day::today();
        let selected = self.selected();
        let end = if self.range { Day::parse_iso(&self.end_date) } else { None };
        let primary = theme.color("primary");
        let marked: Vec<&str> = self.events.split(';').filter(|e| !e.is_empty()).collect();
        for cell in 0..42 {
            let day = first.add_days(cell);
            let rect = RectF::new(area.x + (cell % 7) as f32 * cw, area.y + (cell / 7) as f32 * ch, cw, ch).inflate(-2.0);
            let mut style = text.clone();
            let iso = day.iso();
            // Os dias do meio do intervalo: uma faixa clara.
            if let (Some(a), Some(b)) = (selected, end) {
                if iso > a.iso() && iso < b.iso() {
                    canvas.fill_rect(RectF::new(rect.x - 2.0, rect.y, rect.width + 4.0, rect.height), 0.0, &Brush::Solid(primary.with_opacity(0.14)));
                }
            }
            let endpoint = Some(day) == selected || (end.is_some() && Some(day) == end);
            if marked.contains(&iso.as_str()) {
                let d = 4.0;
                let dot = RectF::new(rect.x + (rect.width - d) / 2.0, rect.bottom() - d - 2.0, d, d);
                canvas.fill_ellipse(dot, &Brush::Solid(if endpoint { theme.color("on_primary") } else { primary }));
            }
            if endpoint {
                canvas.fill_rect(rect, 6.0, &Brush::Solid(primary));
                style.color = theme.color("on_primary");
            } else if Some(day) == self.hovered {
                canvas.fill_rect(rect, 6.0, &Brush::Solid(theme.color("hover_overlay")));
            }
            if day == today && Some(day) != selected {
                canvas.stroke_rect(rect, 6.0, &Stroke::new(primary, 1.5));
            }
            if day.month != self.shown.month {
                style.color = theme.color("text_muted").with_opacity(0.6);
            }
            canvas.draw_text(&day.day.to_string(), rect, &style);
        }
    }
}

// ---------------------------------------------------------------------------
// DateEdit

const BUTTON: f32 = 28.0;

/// Campo de data com calendário suspenso (como o `TDateEdit`). Digite no formato do
/// idioma ou escolha no calendário (clique no ícone, Alt+↓ ou F4). `date` é ISO;
/// `on_change` traz a data ISO (vazia ao apagar).
pub struct DateEdit {
    pub edit: Edit,
    pub date: String,
    pattern: String,
    popup: Option<ComponentId>,
}

impl Default for DateEdit {
    fn default() -> Self {
        let mut edit = Edit::default();
        edit.base.set_width(150.0);
        edit.filter = Some(|c| c.is_ascii_digit() || matches!(c, '/' | '-' | '.'));
        edit.silent = true;
        edit.right_inset = BUTTON - 6.0;
        Self { edit, date: String::new(), pattern: "dd/mm/yyyy".into(), popup: None }
    }
}

impl DateEdit {
    pub fn new(date: &str) -> Self {
        let mut d = Self { date: date.to_string(), ..Default::default() };
        d.show();
        d
    }

    pub fn selected(&self) -> Option<Day> {
        Day::parse_iso(&self.date)
    }

    fn show(&mut self) {
        self.edit.text = self.selected().map(|d| d.format(&self.pattern)).unwrap_or_default();
    }

    fn after_set(&mut self, name: &str) {
        if name == "date" {
            self.show();
        }
    }

    fn set(&mut self, ctx: &mut EventCtx<'_>, day: Option<Day>) {
        let iso = day.map(|d| d.iso()).unwrap_or_default();
        let changed = iso != self.date;
        self.date = iso;
        self.show();
        if changed {
            ctx.fire("on_change", EventArgs::Text(self.date.clone()));
        }
        ctx.repaint();
    }

    fn commit(&mut self, ctx: &mut EventCtx<'_>) {
        let text = self.edit.text.trim().to_string();
        if text.is_empty() {
            self.set(ctx, None);
        } else {
            let parsed = Day::parse(&text, &self.pattern);
            match parsed {
                Some(d) => self.set(ctx, Some(d)),
                None => {
                    self.show();
                    ctx.repaint();
                }
            }
        }
    }

    fn open(&mut self, ctx: &mut EventCtx<'_>) {
        if self.popup.is_some() {
            return;
        }
        let root = ctx.ui.root_of(ctx.id);
        let rect = ctx.ui.to_form(ctx.id, self.edit.base.bounds());
        let (w, h) = (280.0, 290.0);
        let root_size = ctx.ui.bounds(root).map_or((f32::MAX, f32::MAX), |b| (b.width, b.height));
        let top = if rect.bottom() + h > root_size.1 && rect.y - h > 0.0 { rect.y - h - 2.0 } else { rect.bottom() + 2.0 };
        let left = rect.x.min(root_size.0 - w - 4.0).max(0.0);
        let mut calendar = Calendar::new(&self.date);
        calendar.owner = Some(ctx.id);
        calendar.base.set_can_focus(false);
        let id = ctx.ui.add(calendar, root);
        if let Some(cb) = ctx.ui.control_mut(id) {
            cb.set_rect(left, top, w, h);
        }
        ctx.ui.follow(id, ctx.id);
        self.popup = Some(id);
        ctx.repaint();
    }

    fn close(&mut self, ctx: &mut EventCtx<'_>) {
        if let Some(p) = self.popup.take() {
            ctx.ui.free(p);
            ctx.repaint();
        }
    }
}

impl Component for DateEdit {
    // O `on_change` do valor mudado pelo código leva o mesmo que o do usuário.
    // O `text` (do campo de dentro) só vale ao confirmar: a data é o valor.
    fn change_args(&self, name: &str, _value: &aylis_core::PropValue) -> Option<EventArgs> {
        (name == "date").then(|| EventArgs::Text(self.date.clone()))
    }

    fn class_name(&self) -> &'static str {
        "DateEdit"
    }

    fn inner(&self) -> Option<&dyn Component> {
        Some(&self.edit)
    }

    fn inner_mut(&mut self) -> Option<&mut dyn Component> {
        Some(&mut self.edit)
    }

    properties! {
        date: String => "Data";
        events: on_change;
        on_set: after_set
    }

    fn resized(&mut self, ctx: &mut EventCtx<'_>) {
        let pattern = ctx.ui.locale().short_date_format;
        if pattern != self.pattern {
            self.pattern = pattern;
            self.show();
        }
    }

    fn pointer_down(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        let width = self.edit.base.bounds().width;
        if event.button == PointerButton::Primary && event.position.x >= width - BUTTON {
            if self.popup.is_some() {
                self.close(ctx);
            } else {
                self.commit(ctx);
                self.open(ctx);
            }
            ctx.set_handled();
            return;
        }
        self.edit.pointer_down(ctx, event);
    }

    fn key_down(&mut self, ctx: &mut EventCtx<'_>, event: &KeyEvent) {
        match event.key {
            Key::F(4) => {}
            Key::Down if event.modifiers.alt => {}
            Key::Escape if self.popup.is_some() => {
                self.close(ctx);
                ctx.set_handled();
                return;
            }
            Key::Enter => {
                self.commit(ctx);
                self.close(ctx);
                return;
            }
            Key::Up | Key::Down => {
                let base = Day::parse(&self.edit.text, &self.pattern).or(self.selected()).unwrap_or_else(Day::today);
                let next = base.add_days(if event.key == Key::Up { 1 } else { -1 });
                self.set(ctx, Some(next));
                ctx.set_handled();
                return;
            }
            _ => return self.edit.key_down(ctx, event),
        }
        // F4 / Alt+↓: abre ou fecha o calendário.
        if self.popup.is_some() {
            self.close(ctx);
        } else {
            self.commit(ctx);
            self.open(ctx);
        }
        ctx.set_handled();
    }

    fn focus_exit(&mut self, ctx: &mut EventCtx<'_>) {
        self.commit(ctx);
        self.close(ctx);
        self.edit.focus_exit(ctx);
    }

    fn message(&mut self, ctx: &mut EventCtx<'_>, message: &dyn Any) {
        if let Some(PickedDay(day)) = message.downcast_ref::<PickedDay>() {
            self.set(ctx, Some(*day));
            self.close(ctx);
        }
    }

    fn notification(&mut self, freed: ComponentId) {
        if self.popup == Some(freed) {
            self.popup = None;
        }
    }

    fn paint_after(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        // Ícone de calendário.
        let r = ctx.rect;
        let icon = RectF::new(r.right() - BUTTON + 6.0, r.center().y - 7.0, 16.0, 14.0);
        let color = if self.popup.is_some() { ctx.theme.color("primary") } else { ctx.theme.color("text_muted") };
        let stroke = Stroke::new(color, 1.4);
        canvas.stroke_rect(icon, 2.5, &stroke);
        canvas.draw_line(PointF::new(icon.x, icon.y + 4.5), PointF::new(icon.right(), icon.y + 4.5), &stroke);
        for x in [icon.x + 4.0, icon.right() - 4.0] {
            canvas.draw_line(PointF::new(x, icon.y - 2.0), PointF::new(x, icon.y + 2.0), &stroke);
        }
    }
}

// ---------------------------------------------------------------------------
// TimeEdit

/// Campo de hora (como o `TTimeEdit`): `time` é `14:30` (ou `14:30:15` com `seconds`).
/// ↑/↓ e a roda (com o foco) somam `increment` minutos. `on_change` traz a hora. O botão
/// do relógio (ou F4, Alt+↓) abre o [`TimePicker`](crate::TimePicker).
pub struct TimeEdit {
    pub edit: Edit,
    pub time: String,
    pub seconds: bool,
    /// Minutos somados pelas setas.
    pub increment: u32,
    popup: Option<ComponentId>,
}

impl Default for TimeEdit {
    fn default() -> Self {
        let mut edit = Edit::default();
        edit.base.set_width(110.0);
        edit.filter = Some(|c| c.is_ascii_digit() || c == ':');
        edit.silent = true;
        edit.right_inset = BUTTON - 6.0;
        let mut t = Self { edit, time: String::new(), seconds: false, increment: 1, popup: None };
        t.show();
        t
    }
}

impl TimeEdit {
    pub fn new(time: &str) -> Self {
        let mut t = Self { time: time.to_string(), ..Default::default() };
        t.after_set("time");
        t
    }

    /// Segundos desde a meia-noite.
    pub fn parse(text: &str) -> Option<u32> {
        let parts: Vec<&str> = text.trim().split(':').collect();
        if parts.is_empty() || parts.len() > 3 || parts[0].is_empty() {
            return None;
        }
        let nums: Vec<u32> = parts.iter().map(|p| if p.is_empty() { Some(0) } else { p.parse().ok() }).collect::<Option<_>>()?;
        let (h, m, s) = (nums[0], nums.get(1).copied().unwrap_or(0), nums.get(2).copied().unwrap_or(0));
        // "1430" vira 14:30.
        let (h, m) = if parts.len() == 1 && parts[0].len() >= 3 { (h / 100, h % 100) } else { (h, m) };
        (h < 24 && m < 60 && s < 60).then_some(h * 3600 + m * 60 + s)
    }

    fn format(&self, secs: u32) -> String {
        let (h, m, s) = (secs / 3600, secs / 60 % 60, secs % 60);
        if self.seconds { format!("{h:02}:{m:02}:{s:02}") } else { format!("{h:02}:{m:02}") }
    }

    fn show(&mut self) {
        self.edit.text = Self::parse(&self.time).map(|s| self.format(s)).unwrap_or_default();
    }

    fn after_set(&mut self, _name: &str) {
        if let Some(s) = Self::parse(&self.time) {
            self.time = self.format(s);
        }
        self.show();
    }

    fn set(&mut self, ctx: &mut EventCtx<'_>, secs: Option<u32>) {
        let text = secs.map(|s| self.format(s)).unwrap_or_default();
        let changed = text != self.time;
        self.time = text;
        self.show();
        if changed {
            ctx.fire("on_change", EventArgs::Text(self.time.clone()));
        }
        ctx.repaint();
    }

    fn open(&mut self, ctx: &mut EventCtx<'_>) {
        if self.popup.is_some() {
            return;
        }
        let root = ctx.ui.root_of(ctx.id);
        let rect = ctx.ui.to_form(ctx.id, self.edit.base.bounds());
        let (w, h) = (280.0, 340.0);
        let root_size = ctx.ui.bounds(root).map_or((f32::MAX, f32::MAX), |b| (b.width, b.height));
        let top = if rect.bottom() + h > root_size.1 && rect.y - h > 0.0 { rect.y - h - 2.0 } else { rect.bottom() + 2.0 };
        let left = rect.x.min(root_size.0 - w - 4.0).max(0.0);
        let current = Self::parse(&self.time).map(|s| format!("{:02}:{:02}", s / 3600, s / 60 % 60)).unwrap_or_else(|| "12:00".into());
        let mut picker = crate::TimePicker::new(&current);
        picker.owner = Some(ctx.id);
        picker.base.set_can_focus(false);
        let id = ctx.ui.add(picker, root);
        if let Some(cb) = ctx.ui.control_mut(id) {
            cb.set_rect(left, top, w, h);
        }
        ctx.ui.follow(id, ctx.id);
        self.popup = Some(id);
        ctx.repaint();
    }

    fn close(&mut self, ctx: &mut EventCtx<'_>) {
        if let Some(p) = self.popup.take() {
            ctx.ui.free(p);
            ctx.repaint();
        }
    }

    fn step(&mut self, ctx: &mut EventCtx<'_>, minutes: i64) {
        let current = Self::parse(&self.edit.text).or(Self::parse(&self.time)).unwrap_or(0) as i64;
        let next = (current + minutes * 60).rem_euclid(86_400) as u32;
        self.set(ctx, Some(next));
    }
}

impl Component for TimeEdit {
    // O `on_change` do valor mudado pelo código leva o mesmo que o do usuário.
    // O `text` (do campo de dentro) só vale ao confirmar: a hora é o valor.
    fn change_args(&self, name: &str, _value: &aylis_core::PropValue) -> Option<EventArgs> {
        (name == "time").then(|| EventArgs::Text(self.time.clone()))
    }

    fn class_name(&self) -> &'static str {
        "TimeEdit"
    }

    fn inner(&self) -> Option<&dyn Component> {
        Some(&self.edit)
    }

    fn inner_mut(&mut self) -> Option<&mut dyn Component> {
        Some(&mut self.edit)
    }

    properties! {
        time: String => "Data",
        seconds: bool => "Appearance",
        increment: u32 => "Behavior";
        events: on_change;
        on_set: after_set
    }

    fn pointer_down(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        let width = self.edit.base.bounds().width;
        if event.button == PointerButton::Primary && event.position.x >= width - BUTTON {
            if self.popup.is_some() { self.close(ctx) } else { self.open(ctx) }
            ctx.set_handled();
            return;
        }
        self.edit.pointer_down(ctx, event);
    }

    fn message(&mut self, ctx: &mut EventCtx<'_>, message: &dyn Any) {
        if let Some(crate::clock::PickedTime(secs, done)) = message.downcast_ref::<crate::clock::PickedTime>() {
            self.set(ctx, Some(*secs));
            if *done {
                self.close(ctx);
            }
        }
    }

    fn notification(&mut self, freed: ComponentId) {
        if self.popup == Some(freed) {
            self.popup = None;
        }
    }

    fn paint_after(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let color = if self.popup.is_some() { ctx.theme.color("primary") } else { ctx.theme.color("text_muted") };
        ctx.draw_icon(canvas, "clock", RectF::new(r.right() - BUTTON + 5.0, r.center().y - 8.0, 16.0, 16.0), color);
    }

    fn key_down(&mut self, ctx: &mut EventCtx<'_>, event: &KeyEvent) {
        match event.key {
            Key::F(4) => {
                if self.popup.is_some() { self.close(ctx) } else { self.open(ctx) }
            }
            Key::Down if event.modifiers.alt => self.open(ctx),
            Key::Escape if self.popup.is_some() => self.close(ctx),
            Key::Up => self.step(ctx, self.increment.max(1) as i64),
            Key::Down => self.step(ctx, -(self.increment.max(1) as i64)),
            Key::PageUp => self.step(ctx, 60),
            Key::PageDown => self.step(ctx, -60),
            Key::Enter => {
                let parsed = Self::parse(&self.edit.text);
                self.set(ctx, parsed.or(Self::parse(&self.time)));
                return;
            }
            _ => return self.edit.key_down(ctx, event),
        }
        ctx.set_handled();
    }

    fn wheel(&mut self, ctx: &mut EventCtx<'_>, event: &WheelEvent) {
        if event.touch || !self.edit.base.state().focused {
            return;
        }
        let step = self.increment.max(1) as i64;
        self.step(ctx, if event.delta.y > 0.0 { step } else { -step });
        ctx.set_handled();
    }

    fn focus_exit(&mut self, ctx: &mut EventCtx<'_>) {
        let text = self.edit.text.trim().to_string();
        let parsed = if text.is_empty() { None } else { Self::parse(&text).or(Self::parse(&self.time)) };
        self.set(ctx, parsed);
        self.close(ctx);
        self.edit.focus_exit(ctx);
    }
}

// ---------------------------------------------------------------------------
// DateTimeEdit

/// Campo de dia e hora juntos (como o `TDateTimePicker` com data e hora): `date_time` é
/// `2026-11-12T14:30` (ISO, sem fuso), e a tela mostra no formato do idioma
/// (`12/11/2026 14:30`). O botão (ou F4, Alt+↓) abre o calendário, que troca o dia e mantém a
/// hora; ↑/↓ somam um dia com o cursor na data e `increment` minutos com o cursor na hora.
/// `on_change` traz o valor ISO. O relatório do Tabernáculo, C30.
pub struct DateTimeEdit {
    pub edit: Edit,
    pub date_time: String,
    /// Minutos somados pelas setas na parte da hora.
    pub increment: u32,
    pattern: String,
    popup: Option<ComponentId>,
}

impl Default for DateTimeEdit {
    fn default() -> Self {
        let mut edit = Edit::default();
        edit.base.set_width(190.0);
        edit.filter = Some(|c| c.is_ascii_digit() || matches!(c, '/' | '-' | '.' | ':' | ' '));
        edit.silent = true;
        edit.right_inset = BUTTON - 6.0;
        Self { edit, date_time: String::new(), increment: 1, pattern: "dd/mm/yyyy".into(), popup: None }
    }
}

impl DateTimeEdit {
    pub fn new(date_time: &str) -> Self {
        let mut d = Self::default();
        d.date_time = Self::normalize(date_time).unwrap_or_default();
        d.show();
        d
    }

    /// O dia e os minutos desde a meia-noite de um valor ISO (`2026-11-12T14:30` ou com espaço).
    pub fn parse_iso(text: &str) -> Option<(Day, u32)> {
        let text = text.trim();
        let (date, time) = text.split_once(['T', ' ']).unwrap_or((text, "00:00"));
        Some((Day::parse_iso(date)?, TimeEdit::parse(time)? / 60))
    }

    fn iso(day: Day, minutes: u32) -> String {
        format!("{}T{:02}:{:02}", day.iso(), minutes / 60, minutes % 60)
    }

    fn normalize(text: &str) -> Option<String> {
        Self::parse_iso(text).map(|(d, m)| Self::iso(d, m))
    }

    /// O dia e a hora escolhidos.
    pub fn selected(&self) -> Option<(Day, u32)> {
        Self::parse_iso(&self.date_time)
    }

    fn show(&mut self) {
        self.edit.text = self.selected().map(|(d, m)| format!("{} {:02}:{:02}", d.format(&self.pattern), m / 60, m % 60)).unwrap_or_default();
    }

    fn after_set(&mut self, name: &str) {
        if name == "date_time" {
            if let Some(v) = Self::normalize(&self.date_time) {
                self.date_time = v;
            }
            self.show();
        }
    }

    fn set(&mut self, ctx: &mut EventCtx<'_>, value: Option<(Day, u32)>) {
        let iso = value.map(|(d, m)| Self::iso(d, m)).unwrap_or_default();
        let changed = iso != self.date_time;
        self.date_time = iso;
        self.show();
        if changed {
            ctx.fire("on_change", EventArgs::Text(self.date_time.clone()));
        }
        ctx.repaint();
    }

    /// O texto digitado: o dia no formato do idioma e a hora depois de um espaço (sem hora,
    /// fica a que já estava, ou meia-noite).
    fn commit(&mut self, ctx: &mut EventCtx<'_>) {
        let text = self.edit.text.trim().to_string();
        if text.is_empty() {
            return self.set(ctx, None);
        }
        let (date, time) = text.split_once(' ').unwrap_or((&text, ""));
        let minutes = if time.trim().is_empty() { self.selected().map_or(Some(0), |(_, m)| Some(m)) } else { TimeEdit::parse(time).map(|s| s / 60) };
        match (Day::parse(date, &self.pattern), minutes) {
            (Some(d), Some(m)) => self.set(ctx, Some((d, m))),
            _ => {
                self.show();
                ctx.repaint();
            }
        }
    }

    fn open(&mut self, ctx: &mut EventCtx<'_>) {
        if self.popup.is_some() {
            return;
        }
        let root = ctx.ui.root_of(ctx.id);
        let rect = ctx.ui.to_form(ctx.id, self.edit.base.bounds());
        let (w, h) = (280.0, 290.0);
        let root_size = ctx.ui.bounds(root).map_or((f32::MAX, f32::MAX), |b| (b.width, b.height));
        let top = if rect.bottom() + h > root_size.1 && rect.y - h > 0.0 { rect.y - h - 2.0 } else { rect.bottom() + 2.0 };
        let left = rect.x.min(root_size.0 - w - 4.0).max(0.0);
        let day = self.selected().map(|(d, _)| d.iso()).unwrap_or_default();
        let mut calendar = Calendar::new(&day);
        calendar.owner = Some(ctx.id);
        calendar.base.set_can_focus(false);
        let id = ctx.ui.add(calendar, root);
        if let Some(cb) = ctx.ui.control_mut(id) {
            cb.set_rect(left, top, w, h);
        }
        ctx.ui.follow(id, ctx.id);
        self.popup = Some(id);
        ctx.repaint();
    }

    fn close(&mut self, ctx: &mut EventCtx<'_>) {
        if let Some(p) = self.popup.take() {
            ctx.ui.free(p);
            ctx.repaint();
        }
    }

    fn toggle(&mut self, ctx: &mut EventCtx<'_>) {
        if self.popup.is_some() {
            self.close(ctx);
        } else {
            self.commit(ctx);
            self.open(ctx);
        }
    }
}

impl Component for DateTimeEdit {
    // O `text` (do campo de dentro) só vale ao confirmar: o dia e a hora são o valor.
    fn change_args(&self, name: &str, _value: &aylis_core::PropValue) -> Option<EventArgs> {
        (name == "date_time").then(|| EventArgs::Text(self.date_time.clone()))
    }

    fn class_name(&self) -> &'static str {
        "DateTimeEdit"
    }

    fn inner(&self) -> Option<&dyn Component> {
        Some(&self.edit)
    }

    fn inner_mut(&mut self) -> Option<&mut dyn Component> {
        Some(&mut self.edit)
    }

    properties! {
        date_time: String => "Data",
        increment: u32 => "Behavior";
        events: on_change;
        on_set: after_set
    }

    fn resized(&mut self, ctx: &mut EventCtx<'_>) {
        let pattern = ctx.ui.locale().short_date_format;
        if pattern != self.pattern {
            self.pattern = pattern;
            self.show();
        }
    }

    fn pointer_down(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        let width = self.edit.base.bounds().width;
        if event.button == PointerButton::Primary && event.position.x >= width - BUTTON {
            self.toggle(ctx);
            ctx.set_handled();
            return;
        }
        self.edit.pointer_down(ctx, event);
    }

    fn key_down(&mut self, ctx: &mut EventCtx<'_>, event: &KeyEvent) {
        match event.key {
            Key::F(4) => self.toggle(ctx),
            Key::Down if event.modifiers.alt => self.toggle(ctx),
            Key::Escape if self.popup.is_some() => self.close(ctx),
            Key::Enter => {
                self.commit(ctx);
                self.close(ctx);
                return;
            }
            Key::Up | Key::Down => {
                self.commit(ctx);
                let (day, minutes) = self.selected().unwrap_or_else(|| (Day::today(), 12 * 60));
                let sign: i64 = if event.key == Key::Up { 1 } else { -1 };
                // O cursor depois do espaço: a hora; antes, o dia.
                let in_time = self.edit.text.find(' ').is_some_and(|space| self.edit.caret() > space);
                let next = if in_time {
                    let total = minutes as i64 + sign * self.increment.max(1) as i64;
                    (day.add_days(total.div_euclid(24 * 60)), total.rem_euclid(24 * 60) as u32)
                } else {
                    (day.add_days(sign), minutes)
                };
                let caret = self.edit.caret();
                self.set(ctx, Some(next));
                self.edit.select(caret, caret);
            }
            _ => return self.edit.key_down(ctx, event),
        }
        ctx.set_handled();
    }

    fn focus_exit(&mut self, ctx: &mut EventCtx<'_>) {
        self.commit(ctx);
        self.close(ctx);
        self.edit.focus_exit(ctx);
    }

    fn message(&mut self, ctx: &mut EventCtx<'_>, message: &dyn Any) {
        if let Some(PickedDay(day)) = message.downcast_ref::<PickedDay>() {
            let minutes = self.selected().map_or(0, |(_, m)| m);
            self.set(ctx, Some((*day, minutes)));
            self.close(ctx);
        }
    }

    fn notification(&mut self, freed: ComponentId) {
        if self.popup == Some(freed) {
            self.popup = None;
        }
    }

    fn paint_after(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        // O mesmo ícone de calendário do DateEdit.
        let r = ctx.rect;
        let icon = RectF::new(r.right() - BUTTON + 6.0, r.center().y - 7.0, 16.0, 14.0);
        let color = if self.popup.is_some() { ctx.theme.color("primary") } else { ctx.theme.color("text_muted") };
        let stroke = Stroke::new(color, 1.4);
        canvas.stroke_rect(icon, 2.5, &stroke);
        canvas.draw_line(PointF::new(icon.x, icon.y + 4.5), PointF::new(icon.right(), icon.y + 4.5), &stroke);
        for x in [icon.x + 4.0, icon.right() - 4.0] {
            canvas.draw_line(PointF::new(x, icon.y - 2.0), PointF::new(x, icon.y + 2.0), &stroke);
        }
    }
}
