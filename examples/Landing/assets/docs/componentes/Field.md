# Field

Um campo do ClientDataSet: nome, tipo, tamanho, rótulo, máscara.

**Paleta:** Data · **componente não visual**

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `display_label` | Rótulo de exibição | texto | O título do campo nas grades e nos rótulos; vazio: o nome do campo. |
| `display_width` | Largura de exibição | número | Largura sugerida para grids, em unidades lógicas. 0 = automática. |
| `display_format` | Formato de exibição | texto | Como o valor aparece (`R$ #,##0.00`, `dd/mm/yyyy`); vazio: o padrão do tipo e do idioma. |
| `edit_format` | Formato de edição | texto | Como um número aparece durante a edição; vazio: o `display_format`. |
| `alignment` | Alinhamento do texto | um de: `leading`, `center`, `trailing`, `justify` | `None` = conforme o tipo (números à direita). |
| `display_values` | Valores exibidos | texto | Textos para verdadeiro;falso (campos lógicos), como `DisplayValues`. |

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `required` | Obrigatório | lógico | Não aceita gravar o registro com o campo vazio. |
| `read_only` | Somente leitura | lógico | Os controles ligados mostram, mas não deixam editar. |
| `visible` | Visível | lógico | Aparece nas grades; desligado, a coluna some (o valor continua no registro). |
| `min` | Mínimo | número | O menor valor aceito (números); vazio: sem limite. |
| `max` | Máximo | número | O maior valor aceito (números); vazio: sem limite. |
| `char_case` | Maiúsculas e minúsculas | um de: `normal`, `upper`, `lower` | Maiúsculas (`upper`) ou minúsculas (`lower`) forçadas no valor; `normal`: como digitado. |
| `edit_mask` | Máscara de edição | texto | Máscara de edição (`000.000.000-00;0`, ver `crate::mask`). |
| `sensitive` | Dado pessoal (LGPD: mascarado nas gravações) | lógico | Dado pessoal (LGPD): os controles ligados ao campo contam como `sensitive` (as gravações do `EventRecorder` mascaram o que é digitado neles). |

### Dados

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `field_name` | Nome do campo | texto | Nome da coluna. Vazio = usa o nome do componente. |
| `type` | Tipo de dado | um de: `string`, `memo`, `integer`, `float`, `currency`, `boolean`, `date`, `time`, `datetime`, `blob`, `auto_inc`, `timestamp` | O tipo do valor: `string`, `memo`, `integer`, `float`, `currency`, `boolean`, `date`, `time`, `datetime`, `blob`, `auto_inc` ou `timestamp`. |
| `size` | Tamanho | inteiro | Tamanho máximo (texto). 0 = sem limite. |
| `scale` | Escala | inteiro | Casas decimais de `currency`. |
| `rounding` | Arredondamento | um de: `half_up`, `half_even`, `truncate`, `up` | Como um valor com mais casas é arredondado para `scale` (ao gravar, vindo da tela, do backend ou de uma expressão, e ao mostrar): metade para cima (o padrão), bancário, truncar ou para cima. |
| `default_value` | Valor padrão | texto | Valor inicial de registros novos, em texto (lido com as regras do tipo). |
| `kind` | Tipo | um de: `data`, `calculated`, `lookup` | De onde vem o valor: `data` (do registro), `calculated` (a `expression`) ou `lookup` (procurado em outro dataset). |
| `expression` | Expressão | texto | Campo calculado: a expressão (`quantidade * preco`, ver `crate::expr`). |

### Pesquisa (lookup)

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `lookup_data_set` | Dados de pesquisa | componente | Lookup: o dataset onde procurar. |
| `key_fields` | Campos chave | texto | Lookup: campo(s) deste dataset com a chave (`cliente_id`). |
| `lookup_key_fields` | Campos chave da pesquisa | texto | Lookup: campo(s) do outro dataset que correspondem (`id`). |
| `lookup_result_field` | Campo do resultado da pesquisa | texto | Lookup: o campo trazido (`nome`). |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_change` | o valor mudou (pelo usuário ou pelo código) | — |  |
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
