//! `EventCalendar` (o `v-calendar`: um mês com os eventos de cada dia) e `CommandPalette`
//! (a busca de comandos, o `v-command-palette` / Ctrl+Shift+P do VS Code).

use std::cell::RefCell;

use aylis_core::{
    Brush, Canvas, Color, Component, ComponentId, ControlBase, CursorShape, EventArgs, EventCtx, FontWeight, Key, KeyEvent, PaintCtx, PointF,
    PointerButton, PointerEvent, RectF, Stroke, TextAlign, Ui, properties,
};

use crate::dates::Day;
use crate::leaf_control;
use crate::records::{from_event, parse_list, to_json};

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
    /// A posição no `events` (o `index` do `on_event`).
    pub index: usize,
    /// O dia, em ISO (`2026-10-25`).
    pub date: String,
    /// Início e fim em minutos do dia (`None` = o dia todo).
    pub time: Option<(u32, u32)>,
    pub title: String,
    /// A cor: o nome de uma cor do tema (`success`) ou `#RRGGBB`; vazio = `primary`.
    pub color: String,
}

/// Um evento no JSON do `events` (o [`CALENDAR_EVENT`]).
#[derive(serde::Deserialize, Default)]
#[serde(default)]
struct CalendarEventRecord {
    start: String,
    end: String,
    all_day: bool,
    title: String,
    color: String,
}

/// Lê o JSON do `events`: `[{"start": "2026-10-25T14:00", "end": "2026-10-25T15:30", "title":
/// "Reunião", "color": "success"}, {"start": "2026-10-25", "all_day": true, "title": "Feriado"}]`.
/// Sem `end`, uma hora; o fim noutro dia vai até a meia-noite. Evento sem `start` válido fica
/// de fora.
pub fn parse_events(text: &str) -> Vec<CalendarEvent> {
    parse_list::<CalendarEventRecord>(text)
        .into_iter()
        .enumerate()
        .filter_map(|(index, e)| {
            let start = aylis_core::DateTime::parse_iso(&e.start)?;
            let date = start.date.to_string();
            let time = (!e.all_day).then(|| {
                let from = start.time.hour() as u32 * 60 + start.time.minute() as u32;
                let to = match aylis_core::DateTime::parse_iso(&e.end) {
                    Some(end) if end.date == start.date => end.time.hour() as u32 * 60 + end.time.minute() as u32,
                    Some(end) if end.date > start.date => 24 * 60,
                    _ => from + 60,
                };
                (from, to.max(from + 15))
            });
            Some(CalendarEvent { index, date, time, title: e.title, color: e.color })
        })
        .collect()
}

/// O evento clicado no `EventCalendar` (o `on_event`): a posição dele no `events`, o dia e o
/// título.
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CalendarEventClick {
    pub index: i64,
    #[serde(with = "iso_date")]
    pub date: Option<aylis_core::Date>,
    pub title: String,
}

/// O registro do [`CalendarEventClick`] no acesso tipado.
pub const CALENDAR_EVENT_CLICK: aylis_core::RecordInfo = aylis_core::RecordInfo {
    name: "CalendarEventClick",
    fields: &[("index", aylis_core::PropKind::Int), ("date", aylis_core::PropKind::Date), ("title", aylis_core::PropKind::String)],
};

impl aylis_core::EventValue for CalendarEventClick {
    const ARG: aylis_core::EventArg = aylis_core::EventArg::Value(aylis_core::PropKind::Record(&CALENDAR_EVENT_CLICK));
    fn from_args(args: &EventArgs) -> Option<Self> {
        from_event(args)
    }
}

/// A data em ISO no JSON (vazio = nenhuma).
mod iso_date {
    pub fn serialize<S: serde::Serializer>(date: &Option<aylis_core::Date>, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&date.map(|d| d.to_string()).unwrap_or_default())
    }

    pub fn deserialize<'de, D: serde::Deserializer<'de>>(d: D) -> Result<Option<aylis_core::Date>, D::Error> {
        let text: String = serde::Deserialize::deserialize(d)?;
        Ok(aylis_core::Date::parse_iso(&text))
    }
}

/// Calendário de eventos (o `v-calendar`): vista de mês, semana ou dia (`view`). `events` é a
/// lista de registros [`CALENDAR_EVENT`] (veja [`parse_events`]). `date` é o dia de referência
/// (vazio = hoje); as setas andam um mês, uma semana ou um dia.
/// Eventos: `on_day` (a data), `on_event` (o [`CalendarEventClick`]: posição, dia e título) e
/// `on_slot` (o dia e a hora do horário vazio clicado na semana ou no dia).
pub struct EventCalendar {
    pub base: ControlBase,
    /// A vista: `month` (o mês), `week` (a semana com as horas) ou `day` (um dia).
    pub view: CalendarView,
    /// O dia de referência (ISO, `2026-10-25`): o mês, a semana ou o dia mostrado. Vazio = hoje.
    pub date: String,
    /// Mês mostrado (`2026-10`): o mesmo que `date` no dia 1 (mantido para compatibilidade).
    pub month: String,
    /// Os eventos, em JSON: `[{"start": "2026-10-25T14:00", "end": "2026-10-25T15:30",
    /// "title": "Reunião", "color": "success"}]` (`all_day: true` = o dia todo; a cor é opcional).
    pub events: String,
    /// A primeira hora mostrada nas vistas de semana e de dia (0 a 23).
    pub start_hour: u32,
    /// A hora em que as vistas de semana e de dia terminam (até 24).
    pub end_hour: u32,
    /// Os eventos lidos do `events` (refeitos só quando ele muda).
    parsed: Vec<CalendarEvent>,
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
            parsed: Vec::new(),
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
        if name == "events" {
            self.parsed = parse_events(&self.events);
        }
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
                    hits.push((RectF::new(er.x, er.y - 4.0, er.width, 10.0), event_click(e), "on_event"));
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
                hits.push((er, event_click(e), "on_event"));
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
                hits.push((er, event_click(e), "on_event"));
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
                hits.push((er, event_click(e), "on_event"));
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

/// O JSON do `on_event` de um evento.
fn event_click(e: &CalendarEvent) -> String {
    to_json(&CalendarEventClick { index: e.index as i64, date: aylis_core::Date::parse_iso(&e.date), title: e.title.clone() })
}

/// Um evento do `EventCalendar`: início e fim (`2026-10-25T14:00`), o dia todo (`all_day`, o
/// horário do início não conta), o título e a cor (do tema, `success`, ou `#RRGGBB`).
pub const CALENDAR_EVENT: aylis_core::RecordInfo = aylis_core::RecordInfo {
    name: "CalendarEvent",
    fields: &[
        ("start", aylis_core::PropKind::DateTime),
        ("end", aylis_core::PropKind::DateTime),
        ("all_day", aylis_core::PropKind::Bool),
        ("title", aylis_core::PropKind::String),
        ("color", aylis_core::PropKind::Color),
    ],
};

impl Component for EventCalendar {
    leaf_control!("EventCalendar");

    properties! {
        view: CalendarView => "Appearance",
        date: String as Date => "Data",
        month: String => "Data",
        events: String as RecordList(&CALENDAR_EVENT) => "Data",
        start_hour: u32 => "Appearance",
        end_hour: u32 => "Appearance";
        events: on_day: Option<aylis_core::Date>, on_event: CalendarEventClick, on_slot: Option<aylis_core::DateTime>;
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
        let events = &self.parsed;
        let mut hits = Vec::new();
        match self.view {
            CalendarView::Month => self.paint_month(canvas, ctx, events, &mut hits),
            _ => self.paint_days(canvas, ctx, events, &mut hits),
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

/// Busca de comandos: digite e a lista filtra (sem acento e sem maiúscula, o
/// [`fold`](crate::fold)); setas escolhem, Enter executa (`on_select` com o
/// [`PaletteSelection`]: o `id` e o nome do comando). `items` é a lista de registros
/// [`PALETTE_COMMAND`] (`[{"id": "save", "name": "Salvar", "category": "Arquivo",
/// "description": "Grava o documento", "shortcut": "Ctrl+S", "icon": "download"}]`; só o nome é
/// obrigatório, e o `id` vazio é o nome). Os comandos de mesma `category` ficam juntos, com o
/// título dela em cima.
///
/// Recentes: os últimos escolhidos (até `max_recent`) aparecem primeiro com a busca vazia,
/// só enquanto o app está aberto; o `on_recent_change` manda a lista (`recent`, os `id`
/// separados por `;`) para o app gravar, se quiser, e devolver no `recent` da próxima vez.
///
/// Busca no servidor: o `on_query_change` chega com o texto quando a digitação para por
/// `query_delay` ms (uma tecla nova antes disso cancela o aviso pendente); o app troca o
/// `items` com o resultado (e confere se o `query` ainda é o mesmo). Com `filter` desligado, a
/// lista mostra o `items` como veio, sem filtrar pelo texto.
///
/// [`CommandPalette::open`] (o `open` por nome) mostra a paleta por cima do form, com a busca
/// limpa e o foco; aberta assim, ela some ao escolher, no Esc com a busca vazia e ao perder o
/// foco.
pub struct CommandPalette {
    pub base: ControlBase,
    /// Os comandos, em JSON: `[{"id": "save", "name": "Salvar", "category": "Arquivo",
    /// "description": "...", "shortcut": "Ctrl+S", "icon": "download"}]`.
    pub items: String,
    /// O texto digitado na busca: a lista mostra só os comandos que o contêm (no nome, na
    /// descrição ou na categoria). Vazio: todos.
    pub query: String,
    /// O texto apagado da busca vazia.
    pub placeholder: String,
    /// Filtra o `items` pelo texto da busca (desligado: o app filtra, como na busca no
    /// servidor pelo `on_query_change`).
    pub filter: bool,
    /// Quantos milissegundos a digitação parada espera antes do `on_query_change`.
    pub query_delay: u32,
    /// Quantos recentes a paleta guarda e mostra (0: nenhum).
    pub max_recent: u32,
    /// Os `id` dos recentes, do mais novo ao mais velho, separados por `;` (o app grava e
    /// devolve; o `on_recent_change` avisa quando muda).
    pub recent: String,
    /// Os comandos lidos do `items` (refeitos só quando ele muda).
    commands: Vec<PaletteCommandRecord>,
    /// Os comandos que combinam com a busca, na ordem da tela (refeitos quando a busca, os
    /// comandos ou os recentes mudam; a pintura e as teclas só leem).
    shown: Vec<PaletteRow>,
    active: usize,
    rows: RefCell<Vec<(RectF, usize)>>,
    /// A primeira linha à vista (a lista rola para mostrar a escolhida).
    first_row: std::cell::Cell<usize>,
    /// O aviso do `on_query_change` que espera a digitação parar.
    pending_query: Option<u64>,
    /// Aberta pelo [`CommandPalette::open`]: some ao escolher, no Esc e ao perder o foco.
    opened: bool,
}

/// Um comando no JSON do `items` (o [`PALETTE_COMMAND`]).
#[derive(serde::Deserialize, Default, Clone)]
#[serde(default)]
struct PaletteCommandRecord {
    id: String,
    name: String,
    category: String,
    description: String,
    shortcut: String,
    icon: String,
}

impl PaletteCommandRecord {
    /// O `id`, ou o nome quando o `id` não veio.
    fn key(&self) -> &str {
        if self.id.is_empty() { &self.name } else { &self.id }
    }
}

/// Uma linha da lista: o título de um grupo ou um comando (o índice no `commands`).
#[derive(Clone, Debug, PartialEq)]
enum PaletteRow {
    Header(String),
    Command(usize),
}

/// O comando escolhido (o `on_select`): o `id` (o nome, quando o comando não tem) e o nome.
#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PaletteSelection {
    pub id: String,
    pub name: String,
}

/// O registro do [`PaletteSelection`] no acesso tipado.
pub const PALETTE_SELECTION: aylis_core::RecordInfo =
    aylis_core::RecordInfo { name: "PaletteSelection", fields: &[("id", aylis_core::PropKind::String), ("name", aylis_core::PropKind::String)] };

impl aylis_core::EventValue for PaletteSelection {
    const ARG: aylis_core::EventArg = aylis_core::EventArg::Value(aylis_core::PropKind::Record(&PALETTE_SELECTION));
    fn from_args(args: &EventArgs) -> Option<Self> {
        from_event(args)
    }
}

impl Default for CommandPalette {
    fn default() -> Self {
        let mut base = ControlBase::new(480.0, 320.0);
        base.set_can_focus(true).set_cursor(CursorShape::Pointer);
        let mut p = Self {
            base,
            items: r#"[{"name":"Abrir arquivo","shortcut":"Ctrl+O","icon":"folder"},{"name":"Salvar","shortcut":"Ctrl+S","icon":"download"},{"name":"Procurar","shortcut":"Ctrl+F","icon":"search"},{"name":"Configurações","shortcut":"Ctrl+,","icon":"settings"},{"name":"Sair","shortcut":"Alt+F4","icon":"logout"}]"#.into(),
            query: String::new(),
            placeholder: "Digite um comando…".into(),
            filter: true,
            query_delay: 200,
            max_recent: 5,
            recent: String::new(),
            commands: Vec::new(),
            shown: Vec::new(),
            active: 0,
            rows: RefCell::new(Vec::new()),
            first_row: std::cell::Cell::new(0),
            pending_query: None,
            opened: false,
        };
        p.sync("items");
        p
    }
}

const ROW: f32 = 36.0;
/// A linha de um comando com descrição (o nome e a descrição embaixo).
const ROW_TALL: f32 = 50.0;
/// O título de um grupo (a categoria, os recentes).
const ROW_HEADER: f32 = 26.0;

impl CommandPalette {
    /// Os comandos que combinam com a busca, na ordem da tela: (nome, atalho, ícone).
    pub fn matches(&self) -> Vec<(String, String, String)> {
        self.commands_shown().map(|c| (c.name.clone(), c.shortcut.clone(), c.icon.clone())).collect()
    }

    /// Os `id` dos comandos à vista, na ordem da tela (os recentes primeiro, com a busca vazia).
    pub fn shown_ids(&self) -> Vec<String> {
        self.commands_shown().map(|c| c.key().to_string()).collect()
    }

    /// Os `id` dos recentes, do mais novo ao mais velho.
    pub fn recent_ids(&self) -> Vec<String> {
        self.recent.split(';').map(str::trim).filter(|t| !t.is_empty()).map(str::to_string).collect()
    }

    fn commands_shown(&self) -> impl Iterator<Item = &PaletteCommandRecord> {
        self.shown.iter().filter_map(|r| match r {
            PaletteRow::Command(i) => self.commands.get(*i),
            PaletteRow::Header(_) => None,
        })
    }

    /// Mostra a paleta por cima do form, com a busca limpa e o foco (o Ctrl+K do app). Aberta
    /// assim, ela some ao escolher um comando, no Esc com a busca vazia e ao perder o foco.
    pub fn open(ui: &mut Ui, id: ComponentId) -> bool {
        ui.invoke(id, "open", &[]).is_ok()
    }

    /// Esconde a paleta aberta pelo [`open`](Self::open).
    pub fn close(ui: &mut Ui, id: ComponentId) {
        let _ = ui.invoke(id, "close", &[]);
    }

    fn show_opened(&mut self, ctx: &mut EventCtx<'_>) {
        self.opened = true;
        self.query.clear();
        self.active = 0;
        self.first_row.set(0);
        self.refresh();
        self.base.set_visible(true);
        let _ = ctx.ui.set_child_index(ctx.id, usize::MAX);
        ctx.request_focus(ctx.id);
        ctx.ui.invalidate();
        ctx.repaint();
    }

    /// Some (a aberta pelo `open`).
    fn hide(&mut self, ctx: &mut EventCtx<'_>) {
        self.opened = false;
        self.base.set_visible(false);
        ctx.ui.invalidate();
    }

    fn sync(&mut self, name: &str) {
        match name {
            "items" => self.commands = parse_list(&self.items),
            "query" | "filter" | "recent" | "max_recent" => {}
            _ => return,
        }
        self.refresh();
    }

    /// Refaz a lista à vista: com a busca vazia, os recentes e depois todos (os recentes não
    /// repetem embaixo); com texto, os que contêm a busca (sem acento e sem maiúscula). Os
    /// comandos ficam agrupados pela categoria, na ordem em que ela aparece no `items`.
    fn refresh(&mut self) {
        let q = crate::autocomplete::fold(self.query.trim());
        let filtering = self.filter && !q.is_empty();
        let fits = |c: &PaletteCommandRecord| {
            !filtering || [&c.name, &c.description, &c.category].iter().any(|t| crate::autocomplete::fold(t).contains(&q))
        };
        let mut rows = Vec::new();
        let mut taken = vec![false; self.commands.len()];
        if q.is_empty() && self.max_recent > 0 {
            let recent: Vec<usize> = self
                .recent_ids()
                .iter()
                .filter_map(|id| self.commands.iter().position(|c| c.key() == id))
                .take(self.max_recent as usize)
                .collect();
            if !recent.is_empty() {
                rows.push(PaletteRow::Header("Recentes".into()));
                for i in recent {
                    taken[i] = true;
                    rows.push(PaletteRow::Command(i));
                }
            }
        }
        // As categorias na ordem em que aparecem; sem categoria, antes delas (sem título).
        let mut categories: Vec<&str> = Vec::new();
        for c in &self.commands {
            if !categories.contains(&c.category.as_str()) {
                categories.push(&c.category);
            }
        }
        categories.sort_by_key(|c| !c.is_empty());
        for category in categories {
            let members: Vec<usize> = (0..self.commands.len()).filter(|&i| !taken[i] && self.commands[i].category == category && fits(&self.commands[i])).collect();
            if members.is_empty() {
                continue;
            }
            if !category.is_empty() || rows.iter().any(|r| matches!(r, PaletteRow::Header(_))) {
                rows.push(PaletteRow::Header(category.to_string()));
            }
            rows.extend(members.into_iter().map(PaletteRow::Command));
        }
        self.shown = rows;
        let n = self.commands_shown().count();
        self.active = self.active.min(n.saturating_sub(1));
    }

    /// A busca mudou: a lista refaz e o `on_query_change` espera a digitação parar (a tecla
    /// nova cancela o aviso que esperava).
    fn query_changed(&mut self, ctx: &mut EventCtx<'_>) {
        self.active = 0;
        self.first_row.set(0);
        self.refresh();
        if let Some(pending) = self.pending_query.take() {
            ctx.ui.clear_timeout(pending);
        }
        if !ctx.ui.listens(ctx.id, "on_query_change") {
            return;
        }
        let (id, query) = (ctx.id, self.query.clone());
        let timer = ctx.ui.set_timeout(self.query_delay as u64, move |ui| {
            let current = ui.get_mut::<CommandPalette>(id).map(|p| {
                p.pending_query = None;
                p.query.clone()
            });
            // Só a busca que ainda está na tela (um `query` trocado pelo código no meio desiste).
            if current.as_deref() == Some(query.as_str()) {
                ui.fire(id, "on_query_change", &EventArgs::Text(query));
            }
        });
        self.pending_query = Some(timer);
    }

    fn run(&mut self, ctx: &mut EventCtx<'_>, i: usize) {
        let Some(command) = self.commands_shown().nth(i).cloned() else { return };
        let key = command.key().to_string();
        if self.max_recent > 0 {
            let mut recent = self.recent_ids();
            recent.retain(|r| *r != key);
            recent.insert(0, key.clone());
            recent.truncate(self.max_recent as usize);
            let joined = recent.join(";");
            if joined != self.recent {
                self.recent = joined.clone();
                ctx.fire("on_recent_change", EventArgs::Text(joined));
            }
        }
        // Escolhido: a lista volta para a busca vazia com o recente em cima na próxima vez.
        if self.opened {
            self.query.clear();
            self.hide(ctx);
        }
        self.refresh();
        let selection = PaletteSelection { id: key, name: command.name };
        ctx.fire("on_select", EventArgs::Text(to_json(&selection)));
    }
}

/// Um comando do `CommandPalette`: o `id` (vazio: o nome), o nome, a categoria (o grupo), a
/// descrição (embaixo do nome), o atalho mostrado à direita e o ícone.
pub const PALETTE_COMMAND: aylis_core::RecordInfo = aylis_core::RecordInfo {
    name: "PaletteCommand",
    fields: &[
        ("id", aylis_core::PropKind::String),
        ("name", aylis_core::PropKind::String),
        ("category", aylis_core::PropKind::String),
        ("description", aylis_core::PropKind::String),
        ("shortcut", aylis_core::PropKind::String),
        ("icon", aylis_core::PropKind::String),
    ],
};

impl Component for CommandPalette {
    leaf_control!("CommandPalette");

    properties! {
        items: String as RecordList(&PALETTE_COMMAND) => "Data",
        query: String => "Data",
        placeholder: String => "Appearance",
        filter: bool => "Behavior",
        query_delay: u32 => "Behavior",
        max_recent: u32 => "Behavior",
        recent: String as List => "Data";
        events: on_select: PaletteSelection, on_query_change: String, on_recent_change: String;
        on_set: sync
    }

    fn methods(&self) -> &'static [aylis_core::MethodInfo] {
        const METHODS: &[aylis_core::MethodInfo] = &[aylis_core::MethodInfo::new("open", &[], None), aylis_core::MethodInfo::new("close", &[], None)];
        METHODS
    }

    fn invoke(&mut self, ctx: &mut EventCtx<'_>, method: &str, _args: &[aylis_core::PropValue]) -> Result<aylis_core::PropValue, String> {
        match method {
            "open" => self.show_opened(ctx),
            "close" => self.hide(ctx),
            _ => return Err(format!("CommandPalette não tem o método '{method}'")),
        }
        Ok(aylis_core::PropValue::Null)
    }

    fn accepts_text(&self) -> bool {
        true
    }

    fn text_input(&mut self, ctx: &mut EventCtx<'_>, text: &str) {
        let before = self.query.len();
        self.query.extend(text.chars().filter(|c| !c.is_control()));
        if self.query.len() != before {
            self.query_changed(ctx);
        }
        ctx.repaint();
    }

    fn focus_exit(&mut self, ctx: &mut EventCtx<'_>) {
        // Aberta pelo `open`: perder o foco fecha (como o clique fora de um menu).
        if self.opened {
            self.hide(ctx);
        }
    }

    fn key_down(&mut self, ctx: &mut EventCtx<'_>, event: &KeyEvent) {
        let n = self.commands_shown().count();
        match event.key {
            Key::Backspace => {
                if self.query.pop().is_some() {
                    self.query_changed(ctx);
                }
            }
            Key::Down if n > 0 => self.active = (self.active + 1).min(n - 1),
            Key::Up => self.active = self.active.saturating_sub(1),
            Key::Enter if n > 0 => {
                let i = self.active;
                self.run(ctx, i);
            }
            Key::Escape if !self.query.is_empty() => {
                self.query.clear();
                self.query_changed(ctx);
            }
            Key::Escape if self.opened => self.hide(ctx),
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

    fn wheel(&mut self, ctx: &mut EventCtx<'_>, event: &aylis_core::WheelEvent) {
        let rows = self.shown.len();
        let first = self.first_row.get();
        let next = if event.delta.y < 0.0 { (first + 1).min(rows.saturating_sub(1)) } else { first.saturating_sub(1) };
        if next != first {
            self.first_row.set(next);
            ctx.set_handled();
            ctx.repaint();
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
        let top = search.bottom() + 12.0;
        let bottom = r.bottom() - 6.0;
        let height = |row: &PaletteRow| match row {
            PaletteRow::Header(_) => ROW_HEADER,
            PaletteRow::Command(i) if self.commands.get(*i).is_some_and(|c| !c.description.is_empty()) => ROW_TALL,
            PaletteRow::Command(_) => ROW,
        };
        // A linha escolhida sempre à vista: a lista rola até ela (para cima ou para baixo).
        let active_row = self.shown.iter().enumerate().filter(|(_, row)| matches!(row, PaletteRow::Command(_))).nth(self.active).map(|(i, _)| i);
        let mut first = self.first_row.get().min(self.shown.len().saturating_sub(1));
        if let Some(a) = active_row {
            // O título do grupo vem junto quando a escolhida é a primeira dele.
            let a_top = if a > 0 && matches!(self.shown[a - 1], PaletteRow::Header(_)) { a - 1 } else { a };
            if a_top < first {
                first = a_top;
            }
            while first < a && self.shown[first..=a].iter().map(height).sum::<f32>() > bottom - top {
                first += 1;
            }
        }
        self.first_row.set(first);
        let mut rows = Vec::new();
        let mut y = top;
        let mut command_index = self.shown[..first].iter().filter(|row| matches!(row, PaletteRow::Command(_))).count();
        canvas.save();
        canvas.clip_rect(RectF::new(r.x, top, r.width, (bottom - top).max(0.0)));
        for row in &self.shown[first..] {
            if y >= bottom {
                break;
            }
            let h = height(row);
            match row {
                PaletteRow::Header(title) => {
                    let mut ht = ctx.text_style();
                    ht.color = theme.color("text_muted");
                    ht.font.size = (ht.font.size - 1.0).max(9.0);
                    ht.font.weight = FontWeight::SemiBold;
                    canvas.draw_text(&ctx.ui.tr(title), RectF::new(r.x + 16.0, y, r.width - 32.0, h), &ht);
                }
                PaletteRow::Command(c) => {
                    let Some(command) = self.commands.get(*c) else { continue };
                    let i = command_index;
                    command_index += 1;
                    let row = RectF::new(r.x + 6.0, y, r.width - 12.0, h);
                    if i == self.active {
                        canvas.fill_rect(row, 6.0, &Brush::Solid(theme.color("primary").with_opacity(0.14)));
                    }
                    if !command.icon.is_empty() {
                        ctx.draw_icon(canvas, &command.icon, RectF::new(row.x + 10.0, row.y + (h - 18.0) / 2.0, 18.0, 18.0), theme.color("text_muted"));
                    }
                    let mut nt = ctx.text_style();
                    let text_w = row.width - 140.0;
                    if command.description.is_empty() {
                        canvas.draw_text(&command.name, RectF::new(row.x + 38.0, row.y, text_w, h), &nt);
                    } else {
                        canvas.draw_text(&command.name, RectF::new(row.x + 38.0, row.y + 4.0, text_w, h / 2.0), &nt);
                        let mut dt = ctx.text_style();
                        dt.color = theme.color("text_muted");
                        dt.font.size = (dt.font.size - 1.0).max(9.0);
                        dt.trimming = aylis_core::Trimming::Character;
                        canvas.draw_text(&command.description, RectF::new(row.x + 38.0, row.y + h / 2.0 - 2.0, text_w, h / 2.0), &dt);
                    }
                    if !command.shortcut.is_empty() {
                        nt.h_align = TextAlign::Trailing;
                        nt.color = theme.color("text_muted");
                        nt.font = ctx.monospace_font(nt.font.size);
                        nt.font.size = (nt.font.size - 1.0).max(9.0);
                        canvas.draw_text(&command.shortcut, RectF::new(row.x, row.y, row.width - 12.0, h), &nt);
                    }
                    rows.push((row, i));
                }
            }
            y += h;
        }
        canvas.restore();
        *self.rows.borrow_mut() = rows;
    }
}
