# Polygon

![Polygon](img/Polygon.png)

Polígono regular de `sides` lados, ou os pontos de `points` (`"0.5,0 1,1 0,1"`, de 0 a 1 na largura e na altura do controle).

**Paleta:** Shapes · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `stroke` | Cor do traço | cor |
| `stroke_width` | Largura do traço | número |
| `sides` | Lados | inteiro |
| `points` | Pontos | texto |

## Em código

No form, o `Polygon1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "Polygon", "name": "Polygon1" }
```

E o handler, um método do form (`self.Polygon1` já é um `Polygon`):

```rust
impl FrmPrincipal {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.Polygon1.stroke_width(ui);
        self.Polygon1.set_stroke_width(ui, atual + 10.0);
    }
}
```

(Sem eventos próprios: o exemplo usa o `on_show` do form.)
