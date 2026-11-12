# Joystick

![Joystick](img/Joystick.png)

Alavanca analógica na tela (jogos no celular, controle de câmera): arrastar o botão dá `x` e `y` de -1 a 1 (y positivo para baixo) com a `dead_zone` aplicada, em `on_change`; soltar dispara `on_release` e, com `spring`, o botão volta ao centro. Para o controle de jogo de verdade, o componente `Gamepad`.

**Paleta:** Additional · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `dead_zone` | Zona morta | número |  |
| `spring` | Volta ao centro | lógico |  |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_change` | o valor mudou (pelo usuário ou pelo código, como na VCL) | — |  |
| `on_release` | — | — |  |

## Em código

No form, o `Joystick1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "Joystick", "name": "Joystick1", "on_change": "joystick1_change" }
```

E o handler, um método do form (`self.Joystick1` já é um `Joystick`):

```rust
impl FrmPrincipal {
    pub fn joystick1_change(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.Joystick1.dead_zone(ui);
        self.Joystick1.set_dead_zone(ui, atual + 10.0);
    }
}
```
