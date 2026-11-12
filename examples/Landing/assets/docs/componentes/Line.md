# Line

![Line](img/Line.png)

Uma linha reta; o clique acerta perto dela (`stroke_width` mais uma folga).

**Paleta:** Shapes · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `stroke` | Cor do traço | cor |  |
| `stroke_width` | Largura do traço | número |  |
| `direction` | Direção | um de: `down`, `up`, `horizontal`, `vertical` |  |

## Em código

No form, o `Line1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "Line", "name": "Line1" }
```

E o handler, um método do form (`self.Line1` já é um `Line`):

```rust
impl FrmPrincipal {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.Line1.stroke_width(ui);
        self.Line1.set_stroke_width(ui, atual + 10.0);
    }
}
```

(Sem eventos próprios: o exemplo usa o `on_show` do form.)
