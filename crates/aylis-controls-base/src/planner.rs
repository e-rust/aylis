//! `EventCalendar` (o `v-calendar`: um mês com os eventos de cada dia) e `CommandPalette`
//! (a busca de comandos, o `v-command-palette` / Ctrl+Shift+P do VS Code).

use std::cell::RefCell;

use aylis_core::{
    Brush, Canvas, Color, Component, ControlBase, CursorShape, EventArgs, EventCtx, FontWeight, Key, KeyEvent, PaintCtx, PointF,
    PointerButton, PointerEvent, RectF, Stroke, TextAlign, properties,
};

use crate::dates::Day;
use crate::leaf_control;

fn split(items: &str) -> Vec<&str> {
    if items.is_empty() { Vec::new() } else { items.split(';').collect() }
}

/// Cor de um evento pelo nome da cor do tema (`primary`, `success`...) ou `#hex`.
fn event_color(ctx: &PaintCtx<'_>, name: &str) -> Color {
    if name.starts_with('#') {
        return Color::from_hex(name).unwrap_or_else(|| ctx.theme.color("primary"));
    }
    ctx.theme.colors.get(if name.is_empty() { "primary" } else { name }).copied().unwrap_or_else(|| ctx.theme.color("primary"))
}

// ===========================================================================
// EventCalendar
// ===========================================================================

/// A vista do [`EventCalendar`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CalendarView {
    #[default]
    Month,
    Week,
    Day,
}

impl aylis_core::PropEnum for CalendarView {
    const NAMES: &'static [&'static str] = &["month", "week", "day"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Month, Self::Week, Self::Day].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Um evento lido de `events`.
#[derive(Clone, Debug, PartialEq)]
pub struct CalendarEvent {
    pub date: String,
    /// Início e fim em minutos do dia (`None` = o dia todo).
    pub time: Option<(u32, u32)>,
    pub title: String,
    pub color: String,
}

/// Lê `2026-10-25|Reunião|success` ou `2026-10-25 14:00-15:30|Reunião|success`.
pub fn parse_events(text: &str) -> Vec<CalendarEvent> {
    let minutes = |t: &str| {
        let (h, m) = t.trim().split_once(':')?;
        Some(h.parse::<u32>().ok()? * 60 + m.parse::<u32>().ok()?)
    };
    split(text)
        .into_iter()
        .filter_map(|e| {
            let mut p = e.split('|');
            let when = p.next()?.trim();
            let (date, time) = match when.split_once(' ') {
                Some((d, t)) => {
                    let (a, b) = t.split_once('-').unwrap_or((t, ""));
                    let start = minutes(a)?;
                    let end = minutes(b).unwrap_or(start + 60).max(start + 15);
                    (d.to_string(), Some((start, end)))
                }
                None => (when.to_string(), None),
            };
            Some(CalendarEvent { date, time, title: p.next().unwrap_or("").to_string(), color: p.next().unwrap_or("").to_string() })
        })
        .collect()
}

/// Calendário de eventos (o `v-calendar`): vista de mês, semana ou dia (`view`). `events`
/// separados por `;`, cada um `2026-10-25|Reunião|success` (o dia todo) ou
/// `2026-10-25 14:00-15:30|Reunião|success` (com horário; a cor é opcional). `date` é o
/// dia de referência (vazio = hoje); as setas andam um mês, uma semana ou um dia.
/// Eventos: `on_day` (data ISO), `on_event` (`data|título`) e `on_slot` (`data HH:00`, o
/// horário vazio clicado na semana ou no dia).
pub struct EventCalendar {
    pub base: ControlBase,
    /// A vista: `month` (o mês), `week` (a semana com as horas) ou `day` (um dia).
    pub view: CalendarView,
    /// O dia de referência (ISO, `2026-10-25`): o mês, a semana ou o dia mostrado. Vazio = hoje.
    pub date: String,
    /// Mês mostrado (`2026-10`): o mesmo que `date` no dia 1 (mantido para compatibilidade).
    pub month: String,
    /// Os eventos, separados por `;`: `2026-10-25|Reunião|success` (o dia todo) ou
    /// `2026-10-25 14:00-15:30|Reunião|success` (com horário; a cor é opcional).
    pub events: String,
    /// A primeira hora mostrada nas vistas de semana e de dia (0 a 23).
    pub start_hour: u32,
    /// A hora em que as vistas de semana e de dia terminam (até 24).
    pub end_hour: u32,
    hits: RefCell<Vec<(RectF, String, &'static str)>>,
}

impl Default for EventCalendar {
    fn default() -> Self {
        let mut base = ControlBase::new(560.0, 420.0);
        base.set_cursor(CursorShape::Pointer);
        Self {
            base,
            view: CalendarView::Month,
            date: String::new(),
            month: String::new(),
            events: String::new(),
            start_hour: 7,
            end_hour: 20,
            hits: RefCell::new(Vec::new()),
        }
    }
}

const TITLE: f32 = 40.0;
const NAMES_H: f32 = 22.0;
const GUTTER: f32 = 44.0;

impl EventCalendar {
    /// O dia de referência.
    pub fn anchor(&self) -> Day {
        Day::parse_iso(&self.date).or_else(|| Day::parse_iso(&format!("{}-01", self.month))).unwrap_or_else(Day::today)
    }

    fn shown(&self) -> Day {
        Day { day: 1, ..self.anchor() }
    }

    /// Os dias mostrados na semana (ou o dia).
    fn columns(&self, first_wd: u8) -> Vec<Day> {
        let a = self.anchor();
        match self.view {
            CalendarView::Day => vec![a],
            _ => {
                let back = (a.weekday() as i64 - first_wd as i64).rem_euclid(7);
                (0..7).map(|i| a.add_days(i - back)).collect()
            }
        }
    }

    fn go(&mut self, n: i32) {
        let a = self.anchor();
        let d = match self.view {
            CalendarView::Month => a.add_months(n),
            CalendarView::Week => a.add_days(7 * n as i64),
            CalendarView::Day => a.add_days(n as i64),
        };
        self.date = d.iso();
        self.month = format!("{:04}-{:02}", d.year, d.month);
    }

    fn sync(&mut self, name: &str) {
        // `month` trocado por fora: o dia de referência vai para lá.
        if name == "month" && !self.month.is_empty() && !self.date.starts_with(&self.month) {
            self.date = format!("{}-01", self.month);
        }
    }

    fn paint_month(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>, events: &[CalendarEvent], hits: &mut Vec<(RectF, String, &'static str)>) {
        let r = ctx.rect;
        let theme = ctx.theme;
        let locale = ctx.ui.locale();
        let shown = self.shown();
        let area = RectF::new(r.x, r.y + TITLE + NAMES_H, r.width, r.height - TITLE - NAMES_H);
        let (cw, ch) = (area.width / 7.0, area.height / 6.0);
        let mut small = ctx.text_style();
        small.h_align = TextAlign::Center;
        small.font.size = (small.font.size - 2.0).max(9.0);
        small.color = theme.color("text_muted");
        let first_wd = locale.first_day_of_week;
        for i in 0..7 {
            let name = locale.short_day_names[(i + first_wd as usize) % 7];
            canvas.draw_text(name, RectF::new(r.x + i as f32 * cw, r.y + TITLE, cw, NAMES_H), &small);
        }
        let back = (shown.weekday() as i64 - first_wd as i64).rem_euclid(7);
        let first = shown.add_days(-back);
        let today = Day::today();
        let line = theme.color("border");
        let mut day_hits = Vec::new();
        for cell in 0..42i64 {
            let day = first.add_days(cell);
            let cr = RectF::new(area.x + (cell % 7) as f32 * cw, area.y + (cell / 7) as f32 * ch, cw, ch);
            canvas.draw_line(PointF::new(cr.x, cr.y), PointF::new(cr.right(), cr.y), &Stroke::new(line, 0.5));
            if cell % 7 > 0 {
                canvas.draw_line(PointF::new(cr.x, cr.y), PointF::new(cr.x, cr.bottom()), &Stroke::new(line, 0.5));
            }
            let mut num = ctx.text_style();
            num.h_align = TextAlign::Center;
            num.font.size = (num.font.size - 1.0).max(9.0);
            let badge = RectF::new(cr.x + (cr.width - 22.0) / 2.0, cr.y + 3.0, 22.0, 22.0);
            if day == today {
                canvas.fill_ellipse(badge, &Brush::Solid(theme.color("primary")));
                num.color = theme.color("on_primary");
            } else if day.month != shown.month {
                num.color = theme.color("text_muted").with_opacity(0.6);
            }
            canvas.draw_text(&day.day.to_string(), badge, &num);
            let iso = day.iso();
            let todays: Vec<&CalendarEvent> = events.iter().filter(|e| e.date == iso).collect();
            let chip_h = 18.0;
            let room = ((ch - 28.0) / (chip_h + 2.0)).floor().max(0.0) as usize;
            if room == 0 {
                // Sem espaço para o título: uma faixa por evento (até 3), embaixo do número.
                for (k, e) in todays.iter().take(3).enumerate() {
                    let w = (cr.width - 12.0) / 3.0;
                    let er = RectF::new(cr.x + 6.0 + k as f32 * w, cr.bottom() - 7.0, w - 2.0, 4.0);
                    canvas.fill_rect(er, 2.0, &Brush::Solid(event_color(ctx, &e.color)));
                    hits.push((RectF::new(er.x, er.y - 4.0, er.width, 10.0), format!("{}|{}", e.date, e.title), "on_event"));
                }
                day_hits.push((cr, iso, "on_day"));
                continue;
            }
            let show = if todays.len() > room { room.saturating_sub(1) } else { todays.len() };
            for (k, e) in todays.iter().take(show).enumerate() {
                let er = RectF::new(cr.x + 3.0, cr.y + 27.0 + k as f32 * (chip_h + 2.0), cr.width - 6.0, chip_h);
                canvas.fill_rect(er, 4.0, &Brush::Solid(event_color(ctx, &e.color)));
                let mut et = ctx.text_style();
                et.font.size = (et.font.size - 3.0).max(9.0);
                et.color = Color::WHITE;
                let label = match e.time {
                    Some((s, _)) => format!("{:02}:{:02} {}", s / 60, s % 60, e.title),
                    None => e.title.clone(),
                };
                canvas.draw_text(&label, RectF::new(er.x + 4.0, er.y, er.width - 6.0, er.height), &et);
                hits.push((er, format!("{}|{}", e.date, e.title), "on_event"));
            }
            if todays.len() > show {
                let mut more = small.clone();
                more.h_align = TextAlign::Leading;
                canvas.draw_text(&format!("+{}", todays.len() - show), RectF::new(cr.x + 6.0, cr.y + 27.0 + show as f32 * (chip_h + 2.0), cr.width, chip_h), &more);
            }
            day_hits.push((cr, iso, "on_day"));
        }
        hits.extend(day_hits);
    }

    /// Semana ou dia: uma coluna por dia, a grade de horas e os eventos com horário.
    fn paint_days(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>, events: &[CalendarEvent], hits: &mut Vec<(RectF, String, &'static str)>) {
        let r = ctx.rect;
        let theme = ctx.theme;
        let locale = ctx.ui.locale();
        let days = self.columns(locale.first_day_of_week);
        let today = Day::today();
        let line = theme.color("border");
        let (start, end) = (self.start_hour.min(23), self.end_hour.clamp(self.start_hour.min(23) + 1, 24));
        // Cabeçalho dos dias, faixa do "dia todo" e a grade.
        let head = RectF::new(r.x + GUTTER, r.y + TITLE, r.width - GUTTER, 40.0);
        let all_day = RectF::new(head.x, head.bottom(), head.width, 24.0);
        let grid = RectF::new(head.x, all_day.bottom(), head.width, r.bottom() - all_day.bottom() - 4.0);
        let cw = head.width / days.len() as f32;
        let hour_h = grid.height / (end - start) as f32;
        let mut small = ctx.text_style();
        small.font.size = (small.font.size - 2.0).max(9.0);
        small.color = theme.color("text_muted");
        // Horas na margem e as linhas.
        for (i, h) in (start..end).enumerate() {
            let y = grid.y + i as f32 * hour_h;
            canvas.draw_line(PointF::new(grid.x, y), PointF::new(grid.right(), y), &Stroke::new(line, 0.5));
            let mut t = small.clone();
            t.h_align = TextAlign::Trailing;
            canvas.draw_text(&format!("{h:02}:00"), RectF::new(r.x, y - 8.0, GUTTER - 6.0, 16.0), &t);
        }
        for (i, day) in days.iter().enumerate() {
            let x = head.x + i as f32 * cw;
            canvas.draw_line(PointF::new(x, head.y), PointF::new(x, grid.bottom()), &Stroke::new(line, 0.5));
            let iso = day.iso();
            // Nome e número do dia.
            let mut name = small.clone();
            name.h_align = TextAlign::Center;
            canvas.draw_text(locale.short_day_names[day.weekday() as usize % 7], RectF::new(x, head.y, cw, 16.0), &name);
            let badge = RectF::new(x + (cw - 24.0) / 2.0, head.y + 16.0, 24.0, 24.0);
            let mut num = ctx.text_style();
            num.h_align = TextAlign::Center;
            num.font.weight = FontWeight::SemiBold;
            if *day == today {
                canvas.fill_ellipse(badge, &Brush::Solid(theme.color("primary")));
                num.color = theme.color("on_primary");
            }
            canvas.draw_text(&day.day.to_string(), badge, &num);
            hits.push((RectF::new(x, head.y, cw, head.height), iso.clone(), "on_day"));
            // Eventos do dia todo.
            let whole: Vec<&CalendarEvent> = events.iter().filter(|e| e.date == iso && e.time.is_none()).collect();
            for (k, e) in whole.iter().take(2).enumerate() {
                let w = (cw - 4.0) / whole.len().min(2) as f32;
                let er = RectF::new(x + 2.0 + k as f32 * w, all_day.y + 3.0, w - 2.0, all_day.height - 6.0);
                canvas.fill_rect(er, 4.0, &Brush::Solid(event_color(ctx, &e.color)));
                let mut et = small.clone();
                et.color = Color::WHITE;
                canvas.draw_text(&e.title, RectF::new(er.x + 4.0, er.y, er.width - 6.0, er.height), &et);
                hits.push((er, format!("{}|{}", e.date, e.title), "on_event"));
            }
            // Eventos com horário, lado a lado quando se sobrepõem.
            let mut timed: Vec<&CalendarEvent> = events.iter().filter(|e| e.date == iso && e.time.is_some()).collect();
            timed.sort_by_key(|e| e.time.map(|t| t.0));
            let mut lanes: Vec<u32> = Vec::new();
            let mut placed: Vec<(usize, &CalendarEvent)> = Vec::new();
            for e in &timed {
                let (s, f) = e.time.unwrap_or((0, 0));
                let lane = match lanes.iter().position(|end| *end <= s) {
                    Some(l) => {
                        lanes[l] = f;
                        l
                    }
                    None => {
                        lanes.push(f);
                        lanes.len() - 1
                    }
                };
                placed.push((lane, e));
            }
            let n_lanes = lanes.len().max(1) as f32;
            for (lane, e) in placed {
                let (s, f) = e.time.unwrap_or((0, 0));
                let y0 = grid.y + (s as f32 / 60.0 - start as f32) * hour_h;
                let y1 = grid.y + (f as f32 / 60.0 - start as f32) * hour_h;
                let (y0, y1) = (y0.max(grid.y), y1.min(grid.bottom()));
                if y1 <= y0 {
                    continue;
                }
                let w = (cw - 4.0) / n_lanes;
                let er = RectF::new(x + 2.0 + lane as f32 * w, y0 + 1.0, w - 2.0, y1 - y0 - 2.0);
                let color = event_color(ctx, &e.color);
                canvas.fill_rect(er, 4.0, &Brush::Solid(color.with_opacity(0.9)));
                let mut et = small.clone();
                et.color = Color::WHITE;
                et.font.weight = FontWeight::SemiBold;
                canvas.draw_text(&e.title, RectF::new(er.x + 4.0, er.y + 1.0, er.width - 6.0, 16.0), &et);
                if er.height > 32.0 {
                    let mut tt = small.clone();
                    tt.color = Color::WHITE.with_opacity(0.85);
                    let label = format!("{:02}:{:02}–{:02}:{:02}", s / 60, s % 60, f / 60, f % 60);
                    canvas.draw_text(&label, RectF::new(er.x + 4.0, er.y + 16.0, er.width - 6.0, 16.0), &tt);
                }
                hits.push((er, format!("{}|{}", e.date, e.title), "on_event"));
            }
            // Horários vazios.
            for (j, h) in (start..end).enumerate() {
                hits.push((RectF::new(x, grid.y + j as f32 * hour_h, cw, hour_h), format!("{iso} {h:02}:00"), "on_slot"));
            }
        }
        // A linha da hora atual.
        if let Some(i) = days.iter().position(|d| *d == today) {
            let now = now_minutes();
            let y = grid.y + (now as f32 / 60.0 - start as f32) * hour_h;
            if y > grid.y && y < grid.bottom() {
                let x = head.x + i as f32 * cw;
                let danger = theme.color("danger");
                canvas.draw_line(PointF::new(x, y), PointF::new(x + cw, y), &Stroke::new(danger, 2.0));
                canvas.fill_ellipse(RectF::new(x - 4.0, y - 4.0, 8.0, 8.0), &Brush::Solid(danger));
            }
        }
        let mut whole_day = small.clone();
        whole_day.h_align = TextAlign::Trailing;
        canvas.draw_text("dia", RectF::new(r.x, all_day.y, GUTTER - 6.0, all_day.height), &whole_day);
    }
}

impl Component for EventCalendar {
    leaf_control!("EventCalendar");

    properties! {
        view: CalendarView => "Appearance",
        date: String => "Data",
        month: String => "Data",
        events: String => "Data",
        start_hour: u32 => "Appearance",
        end_hour: u32 => "Appearance";
        events: on_day, on_event, on_slot;
        on_set: sync
    }

    fn pointer_up(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if event.button != PointerButton::Primary {
            return;
        }
        let w = self.base.bounds().width;
        if event.position.y < TITLE {
            if event.position.x < 44.0 {
                self.go(-1);
            } else if event.position.x > w - 44.0 {
                self.go(1);
            }
            ctx.repaint();
            return;
        }
        // Os eventos vêm antes na lista: ficam por cima dos dias e horários.
        let hit = self.hits.borrow().iter().find(|(r, _, _)| r.contains(event.position)).map(|(_, v, k)| (v.clone(), *k));
        if let Some((value, kind)) = hit {
            ctx.fire(kind, EventArgs::Text(value));
        }
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let theme = ctx.theme;
        let locale = ctx.ui.locale();
        canvas.fill_rect(r, 8.0, &Brush::Solid(theme.color("surface")));
        canvas.stroke_rect(r.inflate(-0.5), 8.0, &Stroke::new(theme.color("border"), 1.0));
        let a = self.anchor();
        let title = match self.view {
            CalendarView::Month => format!("{} {}", locale.month_names[(a.month - 1) as usize], a.year),
            CalendarView::Week => {
                let days = self.columns(locale.first_day_of_week);
                let (f, l) = (days[0], days[6]);
                let short = |d: Day| -> String { locale.month_names[(d.month - 1) as usize].chars().take(3).collect() };
                if f.month == l.month {
                    format!("{} – {} {} {}", f.day, l.day, short(l), l.year)
                } else {
                    format!("{} {} – {} {} {}", f.day, short(f), l.day, short(l), l.year)
                }
            }
            CalendarView::Day => {
                format!("{}, {} {} {}", locale.short_day_names[a.weekday() as usize % 7], a.day, locale.month_names[(a.month - 1) as usize], a.year)
            }
        };
        let mut t = ctx.text_style();
        t.h_align = TextAlign::Center;
        t.font.weight = FontWeight::SemiBold;
        t.font.size += 2.0;
        canvas.draw_text(&title, RectF::new(r.x + 44.0, r.y, r.width - 88.0, TITLE), &t);
        for (x, icon) in [(r.x + 10.0, "chevron-left"), (r.right() - 34.0, "chevron-right")] {
            ctx.draw_icon(canvas, icon, RectF::new(x, r.y + 10.0, 22.0, 22.0), theme.color("text_muted"));
        }
        let events = parse_events(&self.events);
        let mut hits = Vec::new();
        match self.view {
            CalendarView::Month => self.paint_month(canvas, ctx, &events, &mut hits),
            _ => self.paint_days(canvas, ctx, &events, &mut hits),
        }
        // Os eventos primeiro (por cima de dias e horários no clique).
        hits.sort_by_key(|(_, _, kind)| if *kind == "on_event" { 0 } else { 1 });
        *self.hits.borrow_mut() = hits;
    }
}

/// Minutos do dia agora (no mesmo relógio do `Day::today`).
fn now_minutes() -> u32 {
    let secs = web_time::SystemTime::now().duration_since(web_time::UNIX_EPOCH).map_or(0, |d| d.as_secs());
    (secs % 86_400 / 60) as u32
}

// ===========================================================================
// CommandPalette
// ===========================================================================

/// Busca de comandos: digite e a lista filtra; setas escolhem, Enter executa
/// (`on_select` com o comando). `items` separados por `;`, cada um `nome|atalho|ícone`
/// (atalho e ícone opcionais).
pub struct CommandPalette {
    pub base: ControlBase,
    /// Os comandos, separados por `;`, cada um `nome|atalho|ícone` (atalho e ícone opcionais).
    pub items: String,
    /// O texto digitado na busca: a lista mostra só os comandos que o contêm. Vazio: todos.
    pub query: String,
    /// O texto apagado da busca vazia.
    pub placeholder: String,
    active: usize,
    rows: RefCell<Vec<(RectF, usize)>>,
}

impl Default for CommandPalette {
    fn default() -> Self {
        let mut base = ControlBase::new(480.0, 320.0);
        base.set_can_focus(true).set_cursor(CursorShape::Pointer);
        Self {
            base,
            items: "Abrir arquivo|Ctrl+O|folder;Salvar|Ctrl+S|download;Procurar|Ctrl+F|search;Configurações|Ctrl+,|settings;Sair|Alt+F4|logout".into(),
            query: String::new(),
            placeholder: "Digite um comando…".into(),
            active: 0,
            rows: RefCell::new(Vec::new()),
        }
    }
}

const ROW: f32 = 36.0;

impl CommandPalette {
    /// Os comandos que combinam com a busca: (nome, atalho, ícone).
    pub fn matches(&self) -> Vec<(String, String, String)> {
        let q = crate::autocomplete::fold(self.query.trim());
        split(&self.items)
            .into_iter()
            .map(|i| {
                let mut p = i.split('|');
                (p.next().unwrap_or("").to_string(), p.next().unwrap_or("").to_string(), p.next().unwrap_or("").to_string())
            })
            .filter(|(name, _, _)| q.is_empty() || crate::autocomplete::fold(name).contains(&q))
            .collect()
    }

    fn run(&mut self, ctx: &mut EventCtx<'_>, i: usize) {
        if let Some((name, _, _)) = self.matches().get(i) {
            ctx.fire("on_select", EventArgs::Text(name.clone()));
        }
    }
}

impl Component for CommandPalette {
    leaf_control!("CommandPalette");

    properties! {
        items: String => "Data",
        query: String => "Data",
        placeholder: String => "Appearance";
        events: on_select
    }

    fn accepts_text(&self) -> bool {
        true
    }

    fn text_input(&mut self, ctx: &mut EventCtx<'_>, text: &str) {
        self.query.extend(text.chars().filter(|c| !c.is_control()));
        self.active = 0;
        ctx.repaint();
    }

    fn key_down(&mut self, ctx: &mut EventCtx<'_>, event: &KeyEvent) {
        let n = self.matches().len();
        match event.key {
            Key::Backspace => {
                self.query.pop();
                self.active = 0;
            }
            Key::Down if n > 0 => self.active = (self.active + 1).min(n - 1),
            Key::Up => self.active = self.active.saturating_sub(1),
            Key::Enter if n > 0 => {
                let i = self.active;
                self.run(ctx, i);
            }
            Key::Escape => self.query.clear(),
            _ => return,
        }
        ctx.set_handled();
        ctx.repaint();
    }

    fn pointer_up(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        ctx.request_focus(ctx.id);
        let hit = self.rows.borrow().iter().find(|(r, _)| r.contains(event.position)).map(|(_, i)| *i);
        if let Some(i) = hit {
            self.active = i;
            self.run(ctx, i);
        }
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let theme = ctx.theme;
        canvas.draw_shadow(r, 12.0, &aylis_core::Shadow::new(Color::rgba(0, 0, 0, 50), 18.0, 0.0, 6.0));
        canvas.fill_rect(r, 12.0, &Brush::Solid(theme.color("surface")));
        canvas.stroke_rect(r.inflate(-0.5), 12.0, &Stroke::new(theme.color("border"), 1.0));
        let search = RectF::new(r.x + 12.0, r.y + 10.0, r.width - 24.0, 36.0);
        ctx.draw_icon(canvas, "search", RectF::new(search.x + 4.0, search.y + 9.0, 18.0, 18.0), theme.color("text_muted"));
        let mut t = ctx.text_style();
        let text_r = RectF::new(search.x + 30.0, search.y, search.width - 30.0, search.height);
        if self.query.is_empty() {
            t.color = theme.color("text_muted");
            canvas.draw_text(&ctx.ui.tr(&self.placeholder), text_r, &t);
        } else {
            canvas.draw_text(&self.query, text_r, &t);
        }
        if ctx.state.focused {
            let w = canvas.measure_text(&self.query, &ctx.text_style()).width;
            let x = text_r.x + w + 1.0;
            canvas.draw_line(PointF::new(x, search.y + 9.0), PointF::new(x, search.bottom() - 9.0), &Stroke::new(theme.color("text"), 1.5));
        }
        canvas.draw_line(PointF::new(r.x, search.bottom() + 6.0), PointF::new(r.right(), search.bottom() + 6.0), &Stroke::new(theme.color("border"), 1.0));
        let mut rows = Vec::new();
        let mut y = search.bottom() + 12.0;
        for (i, (name, shortcut, icon)) in self.matches().iter().enumerate() {
            if y + ROW > r.bottom() - 6.0 {
                break;
            }
            let row = RectF::new(r.x + 6.0, y, r.width - 12.0, ROW);
            if i == self.active {
                canvas.fill_rect(row, 6.0, &Brush::Solid(theme.color("primary").with_opacity(0.14)));
            }
            if !icon.is_empty() {
                ctx.draw_icon(canvas, icon, RectF::new(row.x + 10.0, row.y + 9.0, 18.0, 18.0), theme.color("text_muted"));
            }
            let mut nt = ctx.text_style();
            canvas.draw_text(name, RectF::new(row.x + 38.0, row.y, row.width - 140.0, row.height), &nt);
            if !shortcut.is_empty() {
                nt.h_align = TextAlign::Trailing;
                nt.color = theme.color("text_muted");
                nt.font = ctx.monospace_font(nt.font.size);
                nt.font.size = (nt.font.size - 1.0).max(9.0);
                canvas.draw_text(shortcut, RectF::new(row.x, row.y, row.width - 12.0, row.height), &nt);
            }
            rows.push((row, i));
            y += ROW;
        }
        *self.rows.borrow_mut() = rows;
    }
}
