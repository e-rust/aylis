# Taskbar

Progresso, selo e botões no ícone do app na barra de tarefas.

**Paleta:** System · **componente não visual**

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `badge` | Selo | texto |
| `badge_color` | Cor do selo | cor |

### Comportamento

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `progress` | Progresso | um de: `none`, `normal`, `paused`, `error`, `indeterminate` |
| `value` | Valor | número |
| `buttons` | Botões | texto |
| `tasks` | Tarefas | texto |

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
