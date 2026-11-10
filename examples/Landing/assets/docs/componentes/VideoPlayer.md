# VideoPlayer

![VideoPlayer](img/VideoPlayer.png)

Mostra vídeo: **ele mesmo desenha cada quadro**, então dá para ter cantos arredondados, controles por cima e animação.

**Paleta:** Media · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `fit` | Ajuste | um de: `contain`, `cover`, `stretch`, `center`, `original` |
| `radius` | Raio dos cantos | número |

### Comportamento

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `file` | Arquivo | texto |
| `autoplay` | Toca sozinho | lógico |
| `looping` | Repetindo | lógico |
| `volume` | Volume | número |
| `muted` | Mudo | lógico |
| `rate` | Velocidade | número |
| `output` | Saída | texto |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_end` | — | — |  |
| `on_error` | — | — |  |

## Em código

No form, o `VideoPlayer1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "VideoPlayer", "name": "VideoPlayer1", "on_end": "video_player1_end" }
```

E o handler, um método do form (`self.VideoPlayer1` já é um `VideoPlayer`):

```rust
impl FrmPrincipal {
    pub fn video_player1_end(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.VideoPlayer1.file(ui);
        self.VideoPlayer1.set_file(ui, format!("{atual}!"));
    }
}
```
