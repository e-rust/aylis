# DragImage

A imagem que acompanha o mouse durante um arrastar e soltar.

**Paleta:** System · **componente não visual**

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `image` | Imagem | texto |
| `control` | Controle desenhado | componente |
| `opacity` | Opacidade | número |

### Posição e tamanho

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `width` | Largura | número |
| `height` | Altura | número |
| `hot_x` | Ponto no cursor (X) | número |
| `hot_y` | Ponto no cursor (Y) | número |

## Em código

No form, o `DragImage1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "DragImage", "name": "DragImage1" }
```

E o handler, um método do form (`self.DragImage1` já é um `DragImage`):

```rust
impl FrmPrincipal {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.DragImage1.image(ui);
        self.DragImage1.set_image(ui, format!("{atual}!"));
    }
}
```

(Sem eventos próprios: o exemplo usa o `on_show` do form.)
