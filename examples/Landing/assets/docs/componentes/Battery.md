# Battery

A bateria: `level` (0 a 100) e `charging`. Sempre medindo (a cada `interval`, 5 s).

**Paleta:** Device · **componente não visual**

## Propriedades

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `interval` | Intervalo | número | De quanto em quanto tempo lê a bateria, em segundos. |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_change` | o valor mudou (pelo usuário ou pelo código) | — |  |

## Em código

No form, o `Battery1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "Battery", "name": "Battery1", "on_change": "battery1_change" }
```

E o handler, um método do form (`self.Battery1` já é um `Battery`):

```rust
impl FrmPrincipal {
    pub fn battery1_change(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.Battery1.interval(ui);
        self.Battery1.set_interval(ui, atual + 10.0);
    }
}
```
