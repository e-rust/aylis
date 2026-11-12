# CropBox

![CropBox](img/CropBox.png)

O recorte de uma imagem de `source_width` × `source_height` (pixels da origem; o controle cobre a imagem inteira e escala). A seleção (`sel_x`, `sel_y`, `sel_width`, `sel_height`, em pixels da origem) é o que fica; fora dela a imagem escurece.

**Paleta:** Media · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `caption` | Título | texto |  |
| `framed` | Com moldura | lógico |  |
| `shade` | Sombreamento | cor | A cor de fora da seleção. |

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `min_size` | Tamanho mínimo | número |  |
| `snap` | Passo com Shift | número |  |

### Dados

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `source_width` | Largura da origem | número |  |
| `source_height` | Altura da origem | número |  |
| `sel_x` | X da seleção | número |  |
| `sel_y` | Y da seleção | número |  |
| `sel_width` | Largura da seleção | número |  |
| `sel_height` | Altura da seleção | número |  |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_change` | o valor mudou (pelo usuário ou pelo código, como na VCL) | — |  |
| `on_end` | — | — |  |
| `on_tap` | — | — |  |

## Em código

No form, o `CropBox1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "CropBox", "name": "CropBox1", "on_change": "crop_box1_change" }
```

E o handler, um método do form (`self.CropBox1` já é um `CropBox`):

```rust
impl FrmPrincipal {
    pub fn crop_box1_change(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.CropBox1.source_width(ui);
        self.CropBox1.set_source_width(ui, atual + 10.0);
    }
}
```
