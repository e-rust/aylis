# Microphone

![Microphone](img/Microphone.png)

Microfone com medidor de nível (a barra acende com o som). `active` liga; `Microphone::record`/`stop_recording` gravam em WAV.

**Paleta:** Media · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `device` | Dispositivo | texto | O dispositivo (vazio: o padrão do sistema); `capture::input_devices()` lista. |
| `active` | Ativo | lógico |  |
| `monitor` | Retorno | texto | Manda a entrada para uma ou mais saídas (o formato do `output` dos players, com volume e atraso por saída); vazio: não manda. |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_error` | — | — |  |

## Em código

No form, o `Microphone1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "Microphone", "name": "Microphone1", "on_error": "microphone1_error" }
```

E o handler, um método do form (`self.Microphone1` já é um `Microphone`):

```rust
impl FrmPrincipal {
    pub fn microphone1_error(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.Microphone1.device(ui);
        self.Microphone1.set_device(ui, format!("{atual}!"));
    }
}
```
