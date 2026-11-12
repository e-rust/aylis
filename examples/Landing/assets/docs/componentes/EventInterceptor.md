# EventInterceptor

Recebe num lugar só os eventos de todos os componentes do form ou da aplicação, e pode cancelar.

**Paleta:** System · **componente não visual**

## Propriedades

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `scope` | Alcance | um de: `form`, `application` |  |
| `events` | Eventos | texto |  |
| `classes` | Classes | texto |  |
| `enabled` | Habilitado | lógico |  |

### Outros

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `tag` | Marca (número) | inteiro |  |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_intercept` | — | Ao interceptar | sim: `cancel` |
| `after_intercept` | — | Depois de interceptar |  |

**Veja também:** [Interceptar eventos num lugar só](../guias/interceptador.md)

## Em código

No form, o `EventInterceptor1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "EventInterceptor", "name": "EventInterceptor1", "on_intercept": "event_interceptor1_intercept" }
```

E o handler, um método do form (`self.EventInterceptor1` já é um `EventInterceptor`):

```rust
impl FrmPrincipal {
    pub fn event_interceptor1_intercept(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.EventInterceptor1.events(ui);
        self.EventInterceptor1.set_events(ui, format!("{atual}!"));
    }
}
```
