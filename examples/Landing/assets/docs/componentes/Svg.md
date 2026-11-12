# Svg

![Svg](img/Svg.png)

Imagem SVG desenhada como vetor. `source` é o arquivo no projeto (`assets/images/logo.svg`); `color` é o `currentColor` (sem ela, a cor de texto do tema: ícones em SVG acompanham o tema claro e o escuro).

**Paleta:** Additional · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `source` | Origem | texto |  |
| `color` | Cor | cor |  |
| `fit` | Ajuste | um de: `contain`, `cover`, `stretch`, `center`, `original` |  |
| `highlight` | Destacar | texto |  |
| `highlight_color` | Cor de destaque da área | cor |  |
| `zoom_to` | Aproximar até | texto |  |

## Em código

No form, o `Svg1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "Svg", "name": "Svg1" }
```

E o handler, um método do form (`self.Svg1` já é um `Svg`):

```rust
impl FrmPrincipal {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.Svg1.source(ui);
        self.Svg1.set_source(ui, format!("{atual}!"));
    }
}
```

(Sem eventos próprios: o exemplo usa o `on_show` do form.)
