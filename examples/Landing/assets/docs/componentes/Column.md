# Column

Uma coluna do DataGrid: campo, título, largura, alinhamento, cores.

**Paleta:** Data Controls · **componente não visual**

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `title` | Título | texto |
| `width` | Largura | número |
| `alignment` | Alinhamento do texto | um de: `leading`, `center`, `trailing`, `justify` |
| `color` | Cor | cor |
| `text_color` | Cor do texto | cor |

### Comportamento

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `visible` | Visível | lógico |
| `read_only` | Somente leitura | lógico |

### Dados

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `field_name` | Nome do campo | texto |

## Em código

No form, o `Column1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "Column", "name": "Column1" }
```

E o handler, um método do form (`self.Column1` já é um `Column`):

```rust
impl FrmPrincipal {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.Column1.field_name(ui);
        self.Column1.set_field_name(ui, format!("{atual}!"));
    }
}
```

(Sem eventos próprios: o exemplo usa o `on_show` do form.)
