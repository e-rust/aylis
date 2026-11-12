# AudioOutput

As saídas de áudio do sistema e o aviso quando uma entra ou sai.

**Paleta:** Media · **componente não visual**

## Propriedades

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `watch` | Acompanhar mudanças | lógico |  |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_devices_changed` | — | — |  |

## Em código

No form, o `AudioOutput1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "AudioOutput", "name": "AudioOutput1", "on_devices_changed": "audio_output1_devices_changed" }
```

E o handler, um método do form (`self.AudioOutput1` já é um `AudioOutput`):

```rust
impl FrmPrincipal {
    pub fn audio_output1_devices_changed(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.AudioOutput1.watch(ui);
        self.AudioOutput1.set_watch(ui, !atual);
    }
}
```
