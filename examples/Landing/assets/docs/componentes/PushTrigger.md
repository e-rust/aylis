# PushTrigger

Dispara uma ação quando chega uma mensagem do push que casa com o filtro.

**Paleta:** Device · **componente não visual**

## Propriedades

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `active` | Ativo | lógico |  |
| `source` | Origem | componente |  |
| `filter` | Filtro | texto |  |
| `action` | Ação | componente |  |
| `replace_default` | Substituir a notificação comum | lógico |  |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_trigger` | — | — |  |
| `on_error` | — | — |  |

## Em código

No form, o `PushTrigger1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "PushTrigger", "name": "PushTrigger1", "on_trigger": "push_trigger1_trigger" }
```

E o handler, um método do form (`self.PushTrigger1` já é um `PushTrigger`):

```rust
impl FrmPrincipal {
    pub fn push_trigger1_trigger(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.PushTrigger1.active(ui);
        self.PushTrigger1.set_active(ui, !atual);
    }
}
```
