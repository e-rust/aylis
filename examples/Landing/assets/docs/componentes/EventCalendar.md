# EventCalendar

![EventCalendar](img/EventCalendar.png)

Calendário de eventos (o `v-calendar`): vista de mês, semana ou dia (`view`). `events` é a lista de registros `CALENDAR_EVENT` (veja `parse_events`). `date` é o dia de referência (vazio = hoje); as setas andam um mês, uma semana ou um dia. Eventos: `on_day` (a data), `on_event` (o `CalendarEventClick`: posição, dia e título) e `on_slot` (o dia e a hora do horário vazio clicado na semana ou no dia).

**Paleta:** Additional · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `view` | Visão | um de: `month`, `week`, `day` | A vista: `month` (o mês), `week` (a semana com as horas) ou `day` (um dia). |
| `start_hour` | Hora inicial | inteiro | A primeira hora mostrada nas vistas de semana e de dia (0 a 23). |
| `end_hour` | Hora final | inteiro | A hora em que as vistas de semana e de dia terminam (até 24). |

### Dados

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `date` | Data | data (ISO, `2026-10-24`) | O dia de referência (ISO, `2026-10-25`): o mês, a semana ou o dia mostrado. Vazio = hoje. |
| `month` | Mês | texto | Mês mostrado (`2026-10`): o mesmo que `date` no dia 1 (mantido para compatibilidade). |
| `events` | Eventos | lista de `CalendarEvent` | Os eventos, em JSON: `[{"start": "2026-10-25T14:00", "end": "2026-10-25T15:30", "title": "Reunião", "color": "success"}]` (`all_day: true` = o dia todo; a cor é opcional). |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_day` | — | — |  |
| `on_event` | — | — |  |
| `on_slot` | — | — |  |

## Em código

No form, o `EventCalendar1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "EventCalendar", "name": "EventCalendar1", "on_day": "event_calendar1_day" }
```

E o handler, um método do form (`self.EventCalendar1` já é um `EventCalendar`):

```rust
impl FrmPrincipal {
    pub fn event_calendar1_day(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.EventCalendar1.month(ui);
        self.EventCalendar1.set_month(ui, format!("{atual}!"));
    }
}
```
