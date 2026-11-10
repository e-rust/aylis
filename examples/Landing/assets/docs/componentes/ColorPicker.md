# ColorPicker

![ColorPicker](img/ColorPicker.png)

Seletor de cor (o `v-color-picker`): quadro de saturação × brilho, faixa de matiz, faixa de transparência (`alpha`), `#hex` e amostras prontas. `on_change` com a cor.

**Paleta:** Additional · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `alpha` | Transparência (alfa) | lógico |
| `swatches` | Amostras de cor | lógico |

### Dados

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `color` | Cor | cor |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_change` | o valor mudou (pelo usuário ou pelo código, como na VCL) | — |  |

## Em código

No form, o `ColorPicker1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "ColorPicker", "name": "ColorPicker1", "on_change": "color_picker1_change" }
```

E o handler, um método do form (`self.ColorPicker1` já é um `ColorPicker`):

```rust
impl FrmPrincipal {
    pub fn color_picker1_change(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.ColorPicker1.alpha(ui);
        self.ColorPicker1.set_alpha(ui, !atual);
    }
}
```
