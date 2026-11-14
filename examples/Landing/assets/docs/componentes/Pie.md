# Pie

![Pie](img/Pie.png)

Fatia de pizza da elipse do controle, de `start_angle` a `end_angle` (graus; 0 = em cima, sentido horário, como num relógio).

**Paleta:** Shapes · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `stroke` | Cor do traço | cor | A cor da borda; vazio: sem borda. |
| `stroke_width` | Largura do traço | número | A espessura da borda, em pontos; 0: sem borda. |
| `start_angle` | Ângulo inicial | ângulo (graus) | Onde a fatia começa, em graus (0 = em cima, sentido horário). |
| `end_angle` | Ângulo final | ângulo (graus) | Onde a fatia termina, em graus (0 = em cima, sentido horário). |

## Em código

No form, o `Pie1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "Pie", "name": "Pie1" }
```

E o handler, um método do form (`self.Pie1` já é um `Pie`):

```rust
impl FrmPrincipal {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.Pie1.stroke_width(ui);
        self.Pie1.set_stroke_width(ui, atual + 10.0);
    }
}
```

(Sem eventos próprios: o exemplo usa o `on_show` do form.)
