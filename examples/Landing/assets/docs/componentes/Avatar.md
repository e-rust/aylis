# Avatar

![Avatar](img/Avatar.png)

Foto ou iniciais de uma pessoa, redondo (ou quadrado arredondado com a variante `square`), com a presença no canto. Sem foto, as iniciais de `display_name` sobre uma cor tirada do próprio nome (a mesma pessoa tem sempre a mesma cor).

**Paleta:** Additional · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `display_name` | Nome exibido | texto | Nome da pessoa (as iniciais e a cor saem dele). Não é o `name` do componente. |
| `image` | Imagem | texto | Foto do aplicativo (`assets/...`); vazio = iniciais. |
| `presence` | Presença | um de: `none`, `online`, `away`, `busy`, `offline` |  |

## Em código

No form, o `Avatar1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "Avatar", "name": "Avatar1" }
```

E o handler, um método do form (`self.Avatar1` já é um `Avatar`):

```rust
impl FrmPrincipal {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.Avatar1.display_name(ui);
        self.Avatar1.set_display_name(ui, format!("{atual}!"));
    }
}
```

(Sem eventos próprios: o exemplo usa o `on_show` do form.)
