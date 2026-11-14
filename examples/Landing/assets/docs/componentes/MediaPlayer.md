# MediaPlayer

Toca áudio e vídeo sem tela (sons, música).

**Paleta:** Media · **componente não visual**

## Propriedades

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `file` | Arquivo | texto | Arquivo do aplicativo (`assets/audio/aviso.mp3`) ou caminho no disco. |
| `volume` | Volume | número | O volume, de 0 (mudo) a 1 (o máximo). |
| `looping` | Repetindo | lógico | Volta ao começo ao terminar, sem parar. |
| `autoplay` | Toca sozinho | lógico | Começa a tocar quando o aplicativo abre. |
| `output` | Saída | texto | As saídas: vazio é a padrão; várias tocam o mesmo som sincronizado, cada uma com volume e atraso (`Alto-falantes\|Fone BT;volume=0.8;delay=180`). Veja `crate::AudioOutput`. |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_ready` | — | — |  |
| `on_end` | — | — |  |
| `on_error` | — | — |  |

## Em código

No form, o `MediaPlayer1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "MediaPlayer", "name": "MediaPlayer1", "on_ready": "media_player1_ready" }
```

E o handler, um método do form (`self.MediaPlayer1` já é um `MediaPlayer`):

```rust
impl FrmPrincipal {
    pub fn media_player1_ready(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.MediaPlayer1.file(ui);
        self.MediaPlayer1.set_file(ui, format!("{atual}!"));
    }
}
```
