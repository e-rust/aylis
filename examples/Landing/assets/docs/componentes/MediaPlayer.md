# MediaPlayer

Toca áudio e vídeo sem tela (sons, música).

**Paleta:** Media · **componente não visual**

## Propriedades

### Comportamento

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `file` | Arquivo | texto |
| `volume` | Volume | número |
| `looping` | Repetindo | lógico |
| `autoplay` | Toca sozinho | lógico |
| `output` | Saída | texto |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_end` | — | — |  |
| `on_error` | — | — |  |

## Em código

No form, o `MediaPlayer1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "MediaPlayer", "name": "MediaPlayer1", "on_end": "media_player1_end" }
```

E o handler, um método do form (`self.MediaPlayer1` já é um `MediaPlayer`):

```rust
impl FrmPrincipal {
    pub fn media_player1_end(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.MediaPlayer1.file(ui);
        self.MediaPlayer1.set_file(ui, format!("{atual}!"));
    }
}
```
