# Arc

![Arc](img/Arc.png)

Arco (só o traço) da elipse do controle, de `start_angle` a `end_angle`.

**Paleta:** Shapes · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `stroke` | Cor do traço | cor | A cor do arco; vazio: a cor do texto do tema. |
| `stroke_width` | Largura do traço | número | A espessura do arco, em pontos (o traço fica todo dentro do controle). |
| `start_angle` | Ângulo inicial | ângulo (graus) | Onde o arco começa, em graus (0 = em cima, sentido horário). |
| `end_angle` | Ângulo final | ângulo (graus) | Onde o arco termina, em graus (0 = em cima, sentido horário). |

## Em código

No form, o `Arc1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "Arc", "name": "Arc1" }
```

E o handler, um método do form (`self.Arc1` já é um `Arc`):

```rust
impl FrmPrincipal {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.Arc1.stroke_width(ui);
        self.Arc1.set_stroke_width(ui, atual + 10.0);
    }
}
```

(Sem eventos próprios: o exemplo usa o `on_show` do form.)
