# ProximitySensor

O sensor de proximidade (o rosto encostado no telefone): `near` e `distance` (cm).

**Paleta:** Device · **componente não visual**

## Propriedades

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `active` | Ativo | lógico |  |
| `interval` | Intervalo | número |  |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_change` | o valor mudou (pelo usuário ou pelo código, como na VCL) | — |  |

## Em código

No form, o `ProximitySensor1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "ProximitySensor", "name": "ProximitySensor1", "on_change": "proximity_sensor1_change" }
```

E o handler, um método do form (`self.ProximitySensor1` já é um `ProximitySensor`):

```rust
impl FrmPrincipal {
    pub fn proximity_sensor1_change(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.ProximitySensor1.active(ui);
        self.ProximitySensor1.set_active(ui, !atual);
    }
}
```
