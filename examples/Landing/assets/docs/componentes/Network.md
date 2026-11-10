# Network

A rede: `connected` e `kind` (`wifi`, `cellular`, `ethernet`, `other` ou `none`).

**Paleta:** Device · **componente não visual**

## Propriedades

### Comportamento

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `interval` | Intervalo | número |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_change` | o valor mudou (pelo usuário ou pelo código, como na VCL) | — |  |

## Em código

No form, o `Network1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "Network", "name": "Network1", "on_change": "network1_change" }
```

E o handler, um método do form (`self.Network1` já é um `Network`):

```rust
impl FrmPrincipal {
    pub fn network1_change(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.Network1.interval(ui);
        self.Network1.set_interval(ui, atual + 10.0);
    }
}
```
