# Field

Um campo do ClientDataSet: nome, tipo, tamanho, rótulo, máscara.

**Paleta:** Data · **componente não visual**

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `display_label` | Rótulo de exibição | texto |
| `display_width` | Largura de exibição | número |
| `display_format` | Formato de exibição | texto |
| `edit_format` | Formato de edição | texto |
| `alignment` | Alinhamento do texto | um de: `leading`, `center`, `trailing`, `justify` |
| `display_values` | Valores exibidos | texto |

### Comportamento

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `required` | Obrigatório | lógico |
| `read_only` | Somente leitura | lógico |
| `visible` | Visível | lógico |
| `min` | Mínimo | número |
| `max` | Máximo | número |
| `char_case` | Maiúsculas e minúsculas | um de: `normal`, `upper`, `lower` |
| `edit_mask` | Máscara de edição | texto |
| `sensitive` | Dado pessoal (LGPD: mascarado nas gravações) | lógico |

### Dados

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `field_name` | Nome do campo | texto |
| `type` | Tipo de dado | um de: `string`, `memo`, `integer`, `float`, `currency`, `boolean`, `date`, `time`, `datetime`, `blob`, `auto_inc`, `timestamp` |
| `size` | Tamanho | inteiro |
| `scale` | Escala | inteiro |
| `rounding` | Arredondamento | um de: `half_up`, `half_even`, `truncate`, `up` |
| `default_value` | Valor padrão | texto |
| `kind` | Tipo | um de: `data`, `calculated`, `lookup` |
| `expression` | Expressão | texto |

### Pesquisa (lookup)

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `lookup_data_set` | Dados de pesquisa | componente |
| `key_fields` | Campos chave | texto |
| `lookup_key_fields` | Campos chave da pesquisa | texto |
| `lookup_result_field` | Campo do resultado da pesquisa | texto |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_change` | o valor mudou (pelo usuário ou pelo código, como na VCL) | — |  |
| `on_get_text` | — | — |  |
| `on_set_text` | — | — |  |
| `on_validate` | — | — | sim (recusar com a exceção) |

## Em código

No form, o `Field1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "Field", "name": "Field1", "on_change": "field1_change" }
```

E o handler, um método do form (`self.Field1` já é um `Field`):

```rust
impl FrmPrincipal {
    pub fn field1_change(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.Field1.field_name(ui);
        self.Field1.set_field_name(ui, format!("{atual}!"));
    }
}
```
