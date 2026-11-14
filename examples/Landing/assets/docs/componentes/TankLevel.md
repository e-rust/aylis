# TankLevel

![TankLevel](img/TankLevel.png)

Nível de tanque: o tanque com o líquido ondulando na altura do valor (em %, ou entre `min` e `max`), as marcas e o número.

**Paleta:** Industrial · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `unit` | Unidade | texto | A unidade escrita depois do número (`%`, `L`). |
| `liquid` | Líquido | cor | A cor do líquido. |

### Dados

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `value` | Valor | número | O nível: a altura do líquido entre `min` e `max`. |
| `min` | Mínimo | número | O valor do tanque vazio. |
| `max` | Máximo | número | O valor do tanque cheio. |
| `running` | Ligado | lógico | O líquido ondula (pede quadros enquanto ligado). |
| `alarm` | Alarme | lógico | Em alarme: o contorno vermelho pisca. |

## Em código

No form, o `TankLevel1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "TankLevel", "name": "TankLevel1" }
```

E o handler, um método do form (`self.TankLevel1` já é um `TankLevel`):

```rust
impl FrmPrincipal {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.TankLevel1.value(ui);
        self.TankLevel1.set_value(ui, atual + 10.0);
    }
}
```

(Sem eventos próprios: o exemplo usa o `on_show` do form.)
