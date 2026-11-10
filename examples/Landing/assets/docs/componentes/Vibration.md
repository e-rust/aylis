# Vibration

Vibra o aparelho: `vibrate(ms)` (no Android, a publicação declara a permissão `VIBRATE`).

**Paleta:** Device · **componente não visual**

## Propriedades

Só as comuns.

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_error` | — | — |  |

## Em código

No form, o `Vibration1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "Vibration", "name": "Vibration1", "on_error": "vibration1_error" }
```

E o handler, um método do form (`self.Vibration1` já é um `Vibration`):

```rust
impl FrmPrincipal {
    pub fn vibration1_error(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let _ = &self.Vibration1;
    }
}
```
