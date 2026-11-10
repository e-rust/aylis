# DataRepeater

![DataRepeater](img/DataRepeater.png)

Lista virtual de registros: `data_source` dá a quantidade e os valores; `bindings` (`"lblNome=nome;lblPreco=preco"`) liga os componentes do modelo aos campos.

**Paleta:** Data Controls · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `item_height` | Altura do item | número |
| `item_width` | Largura do item | número |
| `sizes` | Tamanhos da câmera | texto |

### Comportamento

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `item_count` | Quantidade de itens | inteiro |
| `item_form` | Form de cada item | texto |
| `bindings` | Ligações | texto |

### Dados

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `data_source` | Fonte de dados | componente |

### Posição e tamanho

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `orientation` | Orientação | um de: `vertical`, `horizontal` |
| `columns` | Colunas | inteiro |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_bind` | — | — |  |
| `on_load_more` | — | — |  |

## Em código

No form, o `DataRepeater1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "DataRepeater", "name": "DataRepeater1", "on_bind": "data_repeater1_bind" }
```

E o handler, um método do form (`self.DataRepeater1` já é um `DataRepeater`):

```rust
impl FrmPrincipal {
    pub fn data_repeater1_bind(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.DataRepeater1.item_count(ui);
        self.DataRepeater1.set_item_count(ui, atual + 1);
    }
}
```
