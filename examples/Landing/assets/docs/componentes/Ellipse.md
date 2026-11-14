# Ellipse

![Ellipse](img/Ellipse.png)

Elipse que ocupa o controle.

**Paleta:** Shapes · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `stroke` | Cor do traço | cor | A cor da borda; vazio: sem borda. |
| `stroke_width` | Largura do traço | número | A espessura da borda, em pontos; 0: sem borda. |

## Em código

No form, o `Ellipse1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "Ellipse", "name": "Ellipse1" }
```

E o handler, um método do form (`self.Ellipse1` já é um `Ellipse`):

```rust
impl FrmPrincipal {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.Ellipse1.stroke_width(ui);
        self.Ellipse1.set_stroke_width(ui, atual + 10.0);
    }
}
```

(Sem eventos próprios: o exemplo usa o `on_show` do form.)
