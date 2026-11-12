# Taskbar

Progresso, selo e botões no ícone do app na barra de tarefas.

**Paleta:** System · **componente não visual**

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `badge` | Selo | texto | Indicador sobre o ícone: vazio, `•` ou um número. |
| `badge_color` | Cor do selo | cor |  |

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `progress` | Progresso | um de: `none`, `normal`, `paused`, `error`, `indeterminate` |  |
| `value` | Valor | número | 0 a 100. |
| `buttons` | Botões | texto | Botões da miniatura (até 7): `icone.png\|Dica` separados por `;`. |
| `tasks` | Tarefas | texto | Tarefas do clique direito: `Nome\|argumentos` separados por `;`. Abrem o próprio app com os argumentos (veja `std::env::args`). |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_button` | — | — |  |

## Em código

No form, o `Taskbar1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "Taskbar", "name": "Taskbar1", "on_button": "taskbar1_button" }
```

E o handler, um método do form (`self.Taskbar1` já é um `Taskbar`):

```rust
impl FrmPrincipal {
    pub fn taskbar1_button(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.Taskbar1.value(ui);
        self.Taskbar1.set_value(ui, atual + 10.0);
    }
}
```
