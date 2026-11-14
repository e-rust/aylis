# PushTrigger

Dispara uma ação quando chega uma mensagem do push que casa com o filtro.

**Paleta:** Device · **componente não visual**

## Propriedades

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `active` | Ativo | lógico | A regra vale; desligada, as mensagens passam sem disparar a ação. |
| `source` | Origem | componente | O `PushNotifications` de onde vêm as mensagens. |
| `filter` | Filtro | texto | As condições sobre os dados, separadas por `;` (`kind=call`, `kind!=chat`, `caller`: a chave existe); vazio aceita todas. |
| `action` | Ação | componente | O componente executado quando a mensagem casa (qualquer um com o método `execute(dados)`: o `WakeAppAction`, o `NotifyAction` ou um do app). |
| `replace_default` | Substituir a notificação comum | lógico | A notificação comum da mensagem não aparece: a ação cuida dela. |

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
