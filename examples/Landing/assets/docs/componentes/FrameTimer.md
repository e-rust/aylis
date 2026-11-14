# FrameTimer

Um evento a cada quadro do monitor (`on_frame`, com os segundos desde o anterior), para a animação que o app desenha:, mas no ritmo da tela, e só enquanto ligado (desligado, o laço dorme; começa desligado). O relatório do Tabernáculo, P05.

**Paleta:** System · **componente não visual**

## Propriedades

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `enabled` | Habilitado | lógico | Dispara o `on_frame` a cada quadro; desligado (o começo), o laço dorme. |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_frame` | — | — |  |

## Em código

No form, o `FrameTimer1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "FrameTimer", "name": "FrameTimer1", "on_frame": "frame_timer1_frame" }
```

E o handler, um método do form (`self.FrameTimer1` já é um `FrameTimer`):

```rust
impl FrmPrincipal {
    pub fn frame_timer1_frame(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.FrameTimer1.enabled(ui);
        self.FrameTimer1.set_enabled(ui, !atual);
    }
}
```
