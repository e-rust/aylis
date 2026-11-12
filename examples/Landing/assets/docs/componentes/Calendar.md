# Calendar

![Calendar](img/Calendar.png)

Calendário de um mês: clique num dia escolhe; ‹ › trocam o mês; setas andam pelos dias, PageUp/PageDown pelos meses. `on_change` traz a data ISO.

**Paleta:** Additional · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `range` | Intervalo de datas | lógico | Intervalo (o `v-date-picker` com `multiple="range"`): o 1º clique é `date`, o 2º `end_date`; `on_change` recebe `início;fim`. |
| `month_mode` | Escolhendo o mês | lógico | Escolhe o mês (o `v-month-picker`): `date` fica no dia 1. |

### Dados

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `date` | Data | texto | Data escolhida (ISO, `2026-10-24`); vazia = nenhuma. |
| `end_date` | Data final | texto |  |
| `events` | Eventos | texto | Dias com eventos (ISO separados por `;`): ganham um ponto embaixo do número. |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_change` | o valor mudou (pelo usuário ou pelo código, como na VCL) | — |  |

## Em código

No form, o `Calendar1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "Calendar", "name": "Calendar1", "on_change": "calendar1_change" }
```

E o handler, um método do form (`self.Calendar1` já é um `Calendar`):

```rust
impl FrmPrincipal {
    pub fn calendar1_change(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.Calendar1.date(ui);
        self.Calendar1.set_date(ui, format!("{atual}!"));
    }
}
```
