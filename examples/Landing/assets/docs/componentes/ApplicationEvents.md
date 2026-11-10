# ApplicationEvents

Os eventos da aplicação inteira, num componente não visual de qualquer form: `on_key_down` e `on_key_press` recebem as teclas antes de todos (antes do form com `key_preview` e do controle com foco; engolir ou trocar a tecla vale como lá), e `on_exception` recebe todo erro de handler e de componente: com ele ligado, a caixa de erro padrão não aparece.

**Paleta:** System · **componente não visual**

## Propriedades

### Outros

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `tag` | Marca (número) | inteiro |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_key_down` | uma tecla com o foco no componente | — | sim: `key` |
| `on_key_press` | — | — | sim: `key` |
| `on_exception` | — | — |  |

**Veja também:** [Interceptar eventos num lugar só](../guias/interceptador.md)

## Em código

No form, o `ApplicationEvents1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "ApplicationEvents", "name": "ApplicationEvents1", "on_key_down": "application_events1_key_down" }
```

E o handler, um método do form (`self.ApplicationEvents1` já é um `ApplicationEvents`):

```rust
impl FrmPrincipal {
    pub fn application_events1_key_down(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.ApplicationEvents1.tag(ui);
        self.ApplicationEvents1.set_tag(ui, atual + 1);
    }
}
```
