# FlowLayout

![FlowLayout](img/FlowLayout.png)

Os filhos em fila, no tamanho deles, quebrando a linha quando não cabem (chips, etiquetas, botões de filtro). `h_align` alinha cada linha; `justify` espalha as linhas cheias na largura toda (a última fica à esquerda).

**Paleta:** Layouts · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Posição e tamanho

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `gap` | Espaço entre itens | número |
| `line_gap` | Espaço entre linhas | número |
| `h_align` | Alinhamento horizontal | um de: `leading`, `center`, `trailing`, `justify` |

## Em código

No form, o `FlowLayout1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "FlowLayout", "name": "FlowLayout1" }
```

E o handler, um método do form (`self.FlowLayout1` já é um `FlowLayout`):

```rust
impl FrmPrincipal {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.FlowLayout1.gap(ui);
        self.FlowLayout1.set_gap(ui, atual + 10.0);
    }
}
```

(Sem eventos próprios: o exemplo usa o `on_show` do form.)
