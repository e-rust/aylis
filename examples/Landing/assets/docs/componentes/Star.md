# Star

![Star](img/Star.png)

Estrela de `points` pontas; `inner` é o raio de dentro em relação ao de fora (0 a 1).

**Paleta:** Shapes · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `stroke` | Cor do traço | cor |
| `stroke_width` | Largura do traço | número |
| `points` | Pontos | inteiro |
| `inner` | Raio interno | número |

## Em código

No form, o `Star1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "Star", "name": "Star1" }
```

E o handler, um método do form (`self.Star1` já é um `Star`):

```rust
impl FrmPrincipal {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.Star1.stroke_width(ui);
        self.Star1.set_stroke_width(ui, atual + 10.0);
    }
}
```

(Sem eventos próprios: o exemplo usa o `on_show` do form.)
