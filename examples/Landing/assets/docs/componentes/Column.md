# Column

Uma coluna do DataGrid: campo, título, largura, alinhamento, cores.

**Paleta:** Data Controls · **componente não visual**

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `title` | Título | texto | O título do cabeçalho (vazio = o rótulo do campo). |
| `width` | Largura | número | A largura (0 = a do campo). |
| `alignment` | Alinhamento do texto | um de: `leading`, `center`, `trailing`, `justify` | O alinhamento do texto (`None` = o do campo). |
| `color` | Cor | cor | O fundo das células. |
| `text_color` | Cor do texto | cor | A cor do texto das células. |

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `visible` | Visível | lógico |  |
| `read_only` | Somente leitura | lógico | Não edita nesta coluna, mesmo com a grade editável. |

### Dados

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `field_name` | Nome do campo | texto | O campo do dataset que a coluna mostra. |

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
