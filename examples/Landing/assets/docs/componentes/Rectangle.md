# Rectangle

![Rectangle](img/Rectangle.png)

Retângulo. Cada propriedade vazia (`Null`) usa o valor do tema.

**Paleta:** Shapes · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `fill2` | Segunda cor do degradê | cor |
| `fill_mid` | Cor do meio do degradê | cor |
| `fill_mid_at` | Posição da cor do meio | número |
| `gradient_angle` | Ângulo do degradê | ângulo (graus) |
| `gradient_radial` | Degradê circular | lógico |
| `stroke` | Cor do traço | cor |
| `stroke_width` | Largura do traço | número |
| `radius` | Raio dos cantos | número |

## Em código

No form, o `Rectangle1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "Rectangle", "name": "Rectangle1" }
```

E o handler, um método do form (`self.Rectangle1` já é um `Rectangle`):

```rust
impl FrmPrincipal {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.Rectangle1.fill_mid_at(ui);
        self.Rectangle1.set_fill_mid_at(ui, atual + 10.0);
    }
}
```

(Sem eventos próprios: o exemplo usa o `on_show` do form.)
