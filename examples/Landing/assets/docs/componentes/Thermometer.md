# Thermometer

![Thermometer](img/Thermometer.png)

Termômetro: o tubo com o bulbo, a coluna subindo com o valor e as marcas.

**Paleta:** Industrial · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `unit` | Unidade | texto |  |

### Dados

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `value` | Valor | número |  |
| `min` | Mínimo | número |  |
| `max` | Máximo | número |  |
| `alarm` | Alarme | lógico |  |

## Em código

No form, o `Thermometer1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "Thermometer", "name": "Thermometer1" }
```

E o handler, um método do form (`self.Thermometer1` já é um `Thermometer`):

```rust
impl FrmPrincipal {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.Thermometer1.value(ui);
        self.Thermometer1.set_value(ui, atual + 10.0);
    }
}
```

(Sem eventos próprios: o exemplo usa o `on_show` do form.)
