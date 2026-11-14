# FlowMeter

![FlowMeter](img/FlowMeter.png)

Medidor de vazão: a turbina girando na velocidade da vazão e o valor.

**Paleta:** Industrial · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `unit` | Unidade | texto | A unidade escrita depois do número (`m³/h`, `L/min`). |

### Dados

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `value` | Valor | número | A vazão mostrada; a turbina gira mais rápido quanto mais perto de `max` (0: parada). |
| `max` | Máximo | número | A vazão da velocidade máxima da turbina. |
| `running` | Ligado | lógico | Ligado: a turbina gira (verde); desligado, para (cinza). |
| `alarm` | Alarme | lógico | Em alarme: a turbina fica vermelha e o contorno pisca. |

## Em código

No form, o `FlowMeter1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "FlowMeter", "name": "FlowMeter1" }
```

E o handler, um método do form (`self.FlowMeter1` já é um `FlowMeter`):

```rust
impl FrmPrincipal {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.FlowMeter1.value(ui);
        self.FlowMeter1.set_value(ui, atual + 10.0);
    }
}
```

(Sem eventos próprios: o exemplo usa o `on_show` do form.)
