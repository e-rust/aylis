# AudioMixer

Mistura microfones, o som das saídas e players: volume, mudo, panorâmica; grava em WAV.

**Paleta:** Media · **componente não visual**

## Propriedades

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `sources` | Fontes | texto |  |
| `output` | Saída | texto |  |
| `active` | Ativo | lógico |  |
| `keep_samples` | Guardar amostras | lógico |  |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_error` | — | — |  |

## Em código

No form, o `AudioMixer1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "AudioMixer", "name": "AudioMixer1", "on_error": "audio_mixer1_error" }
```

E o handler, um método do form (`self.AudioMixer1` já é um `AudioMixer`):

```rust
impl FrmPrincipal {
    pub fn audio_mixer1_error(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.AudioMixer1.sources(ui);
        self.AudioMixer1.set_sources(ui, format!("{atual}!"));
    }
}
```
