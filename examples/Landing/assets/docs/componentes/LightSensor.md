# LightSensor

O sensor de luz ambiente: `lux`.

**Paleta:** Device · **componente não visual**

## Propriedades

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `active` | Ativo | lógico | Lê a luz ambiente; desligado, para. |
| `interval` | Intervalo | número | De quanto em quanto tempo lê, em segundos. |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_change` | o valor mudou (pelo usuário ou pelo código) | — |  |

## Em código

No form, o `LightSensor1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "LightSensor", "name": "LightSensor1", "on_change": "light_sensor1_change" }
```

E o handler, um método do form (`self.LightSensor1` já é um `LightSensor`):

```rust
impl FrmPrincipal {
    pub fn light_sensor1_change(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.LightSensor1.active(ui);
        self.LightSensor1.set_active(ui, !atual);
    }
}
```
