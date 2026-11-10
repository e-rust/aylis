# DBImage

![DBImage](img/DBImage.png)

Imagem de um campo: campo `blob` com os bytes da imagem (PNG, JPEG...) ou campo de texto com o caminho de um arquivo do projeto.

**Paleta:** Data Controls · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `image` | Imagem | texto |
| `fit` | Ajuste | um de: `contain`, `cover`, `stretch`, `center`, `original` |
| `radius` | Raio dos cantos | número |

### Dados

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `data_source` | Fonte de dados | componente |
| `data_field` | Campo de dados | texto |

## Em código

No form, o `DBImage1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "DBImage", "name": "DBImage1" }
```

E o handler, um método do form (`self.DBImage1` já é um `DBImage`):

```rust
impl FrmPrincipal {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.DBImage1.data_field(ui);
        self.DBImage1.set_data_field(ui, format!("{atual}!"));
    }
}
```

(Sem eventos próprios: o exemplo usa o `on_show` do form.)
