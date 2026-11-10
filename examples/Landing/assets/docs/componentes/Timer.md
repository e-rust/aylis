# Timer

Dispara on_timer a cada intervalo (como o TTimer).

**Paleta:** System · **componente não visual**

## Propriedades

### Comportamento

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `interval` | Intervalo | duração (ms) |
| `enabled` | Habilitado | lógico |
| `precision` | Precisão | um de: `normal`, `high`, `exact` |
| `background` | Fundo | lógico |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_timer` | a cada `interval` ms, com `enabled` | — |  |

## Em código

No form, o `Timer1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "Timer", "name": "Timer1", "on_timer": "timer1_timer" }
```

E o handler, um método do form (`self.Timer1` já é um `Timer`):

```rust
impl FrmPrincipal {
    pub fn timer1_timer(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.Timer1.enabled(ui);
        self.Timer1.set_enabled(ui, !atual);
    }
}
```
