# Gamepad

Um controle de jogo (Xbox, PlayStation, genérico): o `index` (0 = o primeiro conectado), os analógicos (`left_x`, `left_y`, `right_x`, `right_y`, de -1 a 1, y positivo para baixo) e os gatilhos (`left_trigger`, `right_trigger`, de 0 a 1), lidos a cada quadro com a `dead_zone` aplicada. `on_button_down`/`on_button_up` trazem o nome do botão (`a`, `b`, `x`, `y`, `lb`, `rb`, `lt`, `rt`, `back`, `start`, `l3`, `r3`, `up`, `down`, `left`, `right`, `guide`; no PlayStation, `a` = ✕); `on_change`, os analógicos mexeram; `on_connect`/`on_disconnect` (`device_name`, o nome do controle). Métodos: `pressed(nome)` e `rumble(força, ms)`.

**Paleta:** Device · **componente não visual**

## Propriedades

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `index` | Índice | inteiro |  |
| `active` | Ativo | lógico |  |
| `dead_zone` | Zona morta | número |  |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_button_down` | — | — |  |
| `on_button_up` | — | — |  |
| `on_change` | o valor mudou (pelo usuário ou pelo código, como na VCL) | — |  |
| `on_connect` | — | — |  |
| `on_disconnect` | — | — |  |

## Em código

No form, o `Gamepad1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "Gamepad", "name": "Gamepad1", "on_button_down": "gamepad1_button_down" }
```

E o handler, um método do form (`self.Gamepad1` já é um `Gamepad`):

```rust
impl FrmPrincipal {
    pub fn gamepad1_button_down(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.Gamepad1.index(ui);
        self.Gamepad1.set_index(ui, atual + 1);
    }
}
```
