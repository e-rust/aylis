# Manometer

![Manometer](img/Manometer.png)

Manômetro: o mostrador redondo com o ponteiro, as marcas e a faixa de alarme.

**Paleta:** Industrial · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `unit` | Unidade | texto |  |
| `decimals` | Casas decimais | inteiro |  |

### Dados

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `value` | Valor | número |  |
| `min` | Mínimo | número |  |
| `max` | Máximo | número |  |
| `alarm_above` | Alarme acima de | número | A partir daqui o arco fica vermelho (0 = sem faixa). |
| `alarm` | Alarme | lógico |  |

## Em código

No form, o `Manometer1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "Manometer", "name": "Manometer1" }
```

E o handler, um método do form (`self.Manometer1` já é um `Manometer`):

```rust
impl FrmPrincipal {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.Manometer1.value(ui);
        self.Manometer1.set_value(ui, atual + 10.0);
    }
}
```

(Sem eventos próprios: o exemplo usa o `on_show` do form.)
