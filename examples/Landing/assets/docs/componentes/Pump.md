# Pump

![Pump](img/Pump.png)

Bomba de supervisório: o símbolo com o rotor girando quando ligada e o estado em cor.

**Paleta:** Industrial · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `label` | Rótulo | texto | O texto do equipamento: no corpo do motor (vazio: "M") e embaixo da bomba. |

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `speed` | Velocidade | número | Voltas por segundo do desenho, ligado. |

### Dados

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `running` | Ligado | lógico | Ligado: gira e fica verde; desligado, parado e cinza. |
| `alarm` | Alarme | lógico | Em alarme: fica vermelho e o contorno pisca. |

## Em código

No form, o `Pump1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "Pump", "name": "Pump1" }
```

E o handler, um método do form (`self.Pump1` já é um `Pump`):

```rust
impl FrmPrincipal {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.Pump1.running(ui);
        self.Pump1.set_running(ui, !atual);
    }
}
```

(Sem eventos próprios: o exemplo usa o `on_show` do form.)
