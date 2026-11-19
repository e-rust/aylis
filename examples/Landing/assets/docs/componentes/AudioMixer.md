# AudioMixer

Mistura microfones, o som das saídas e players: volume, mudo, panorâmica; grava em WAV.

**Paleta:** Media · **componente não visual**

## Propriedades

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `sources` | Fontes | lista de `MixSource` | As fontes, cada uma com `volume`, `pan` e `mute` opcionais (a lista de `MIX_SOURCE_RECORD`; veja `parse_sources`). |
| `output` | Saída | lista de `OutputSpec` | As saídas que tocam a mistura (como no `MediaPlayer`); vazio: não toca em lugar nenhum. |
| `active` | Ativo | lógico | Liga a mistura (abre as fontes e as saídas); desligado, tudo fecha. |
| `keep_samples` | Guardar amostras | lógico | Guarda as amostras da mistura (até 5 s) para o código ler com `take_samples`. |

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
        let atual = self.AudioMixer1.active(ui);
        self.AudioMixer1.set_active(ui, !atual);
    }
}
```
