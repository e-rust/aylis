# VideoPlayer

![VideoPlayer](img/VideoPlayer.png)

Mostra vídeo: **ele mesmo desenha cada quadro**, então dá para ter cantos arredondados, controles por cima e animação.

**Paleta:** Media · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `fit` | Ajuste | um de: `contain`, `cover`, `stretch`, `center`, `original` | Como o quadro ocupa o controle: `contain` (inteiro), `cover` (cobre, cortando as bordas), `stretch`, `center` ou `original`. |
| `radius` | Raio dos cantos | número | O raio dos cantos arredondados, em pontos (0: retos). |
| `show_video` | Mostrar o vídeo | lógico | Pinta o quadro no próprio lugar (o padrão). Desligado, o player só decodifica e segue o relógio: o app pinta o `current_frame`(VideoPlayer::current_frame) onde quiser, na ordem da cena (a letra por cima do vídeo), avisado por `on_frame`. O relatório do Tabernáculo, W12. |

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `file` | Arquivo | texto | Arquivo do aplicativo (`assets/video/abertura.mp4`) ou caminho no disco. |
| `autoplay` | Toca sozinho | lógico | Começa a tocar assim que o arquivo abre. |
| `looping` | Repetindo | lógico | Volta ao começo ao terminar, sem parar. |
| `volume` | Volume | número | O volume, de 0 (mudo) a 1 (o máximo). |
| `muted` | Mudo | lógico | Sem som, sem perder o `volume`. |
| `rate` | Velocidade | número | A velocidade: 1 é a normal, 2 o dobro, 0.5 a metade. Fora de 1 o som fica mudo (volta no lugar certo em 1). Só áudio e o navegador tocam sempre em 1. |
| `output` | Saída | texto | As saídas do som (como no `MediaPlayer`). No navegador, o `<video>` toca na saída padrão. |
| `frames_ahead` | Quadros à frente | inteiro | Quantos quadros o decodificador prepara à frente do relógio (0: o padrão, 6; W13). |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_ready` | — | — |  |
| `on_end` | — | — |  |
| `on_error` | — | — |  |
| `on_frame` | — | — |  |

## Em código

No form, o `VideoPlayer1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "VideoPlayer", "name": "VideoPlayer1", "on_ready": "video_player1_ready" }
```

E o handler, um método do form (`self.VideoPlayer1` já é um `VideoPlayer`):

```rust
impl FrmPrincipal {
    pub fn video_player1_ready(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.VideoPlayer1.file(ui);
        self.VideoPlayer1.set_file(ui, format!("{atual}!"));
    }
}
```
