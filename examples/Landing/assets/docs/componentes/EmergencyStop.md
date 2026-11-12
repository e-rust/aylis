# EmergencyStop

![EmergencyStop](img/EmergencyStop.png)

Botão de emergência (cogumelo vermelho sobre fundo amarelo): o clique trava e destrava (`pressed`), com `on_change`.

**Paleta:** Industrial · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Dados

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `pressed` | Pressionado | lógico |  |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_change` | o valor mudou (pelo usuário ou pelo código, como na VCL) | — |  |

## Em código

No form, o `EmergencyStop1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "EmergencyStop", "name": "EmergencyStop1", "on_change": "emergency_stop1_change" }
```

E o handler, um método do form (`self.EmergencyStop1` já é um `EmergencyStop`):

```rust
impl FrmPrincipal {
    pub fn emergency_stop1_change(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.EmergencyStop1.pressed(ui);
        self.EmergencyStop1.set_pressed(ui, !atual);
    }
}
```
