# Thermometer

![Thermometer](img/Thermometer.png)

Termômetro: o tubo com o bulbo, a coluna subindo com o valor e as marcas.

**Paleta:** Industrial · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `unit` | Unidade | texto | A unidade escrita depois do número (`°C`, `°F`). |

### Dados

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `value` | Valor | número | A temperatura: a altura da coluna entre `min` e `max`. |
| `min` | Mínimo | número | A temperatura do pé da escala (coluna vazia). |
| `max` | Máximo | número | A temperatura do topo da escala (coluna cheia). |
| `alarm` | Alarme | lógico | Em alarme: a coluna fica na cor de perigo e o contorno vermelho pisca. |

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
