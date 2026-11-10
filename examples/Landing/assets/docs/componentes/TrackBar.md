# TrackBar

![TrackBar](img/TrackBar.png)

Controle deslizante: arraste o polegar, clique no trilho, use as setas, Home/End e PageUp/PageDown. `on_change` traz o valor (também durante o arrasto).

**Paleta:** Standard · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `vertical` | Vertical | lógico |
| `thumb_size` | Tamanho do botão | número |
| `track_size` | Espessura do trilho | número |
| `solid_thumb` | Botão cheio | lógico |

### Comportamento

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `min` | Mínimo | número |
| `max` | Máximo | número |
| `frequency` | Frequência das marcas | número |
| `limit` | Limite | número |

### Dados

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `value` | Valor | número |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_change` | o valor mudou (pelo usuário ou pelo código, como na VCL) | — |  |

## Em código

No form, o `TrackBar1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "TrackBar", "name": "TrackBar1", "on_change": "track_bar1_change" }
```

E o handler, um método do form (`self.TrackBar1` já é um `TrackBar`):

```rust
impl FrmPrincipal {
    pub fn track_bar1_change(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.TrackBar1.value(ui);
        self.TrackBar1.set_value(ui, atual + 10.0);
    }
}
```
