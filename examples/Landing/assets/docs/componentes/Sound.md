# Sound

Som curto (efeito, alerta): toca por cima de si mesmo, na memória.

**Paleta:** Media · **componente não visual**

## Propriedades

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `file` | Arquivo | texto | Arquivo do aplicativo (`assets/audio/clique.wav`) ou caminho no disco. |
| `volume` | Volume | número |  |
| `output` | Saída | texto | As saídas (como no `MediaPlayer`(crate::MediaPlayer)). |
| `polyphony` | Toques ao mesmo tempo | inteiro | Quantos toques ao mesmo tempo (o mais antigo para quando passa). |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_error` | — | — |  |

## Em código

No form, o `Sound1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "Sound", "name": "Sound1", "on_error": "sound1_error" }
```

E o handler, um método do form (`self.Sound1` já é um `Sound`):

```rust
impl FrmPrincipal {
    pub fn sound1_error(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.Sound1.file(ui);
        self.Sound1.set_file(ui, format!("{atual}!"));
    }
}
```
