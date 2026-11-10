# Image

![Image](img/Image.png)

Mostra uma imagem do aplicativo (`image: "assets/images/logo.png"`, lida pelo serviço de recursos do projeto) ou uma `Image` definida por código (`set_picture`).

**Paleta:** Standard · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `image` | Imagem | texto |
| `fit` | Ajuste | um de: `contain`, `cover`, `stretch`, `center`, `original` |
| `radius` | Raio dos cantos | número |

## Em código

No form, o `Image1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "Image", "name": "Image1" }
```

E o handler, um método do form (`self.Image1` já é um `Image`):

```rust
impl FrmPrincipal {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.Image1.image(ui);
        self.Image1.set_image(ui, format!("{atual}!"));
    }
}
```

(Sem eventos próprios: o exemplo usa o `on_show` do form.)
