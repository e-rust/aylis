# DataRepeater

![DataRepeater](img/DataRepeater.png)

Lista virtual de registros: `data_source` dá a quantidade e os valores; `bindings` (`"lblNome=nome;lblPreco=preco"`) liga os componentes do modelo aos campos.

**Paleta:** Data Controls · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `item_height` | Altura do item | número |  |
| `item_width` | Largura do item | número | Na horizontal: a largura de cada item. |
| `sizes` | Tamanhos da câmera | texto | Tamanhos próprios dos itens, em ordem (`80;120;80`; os que faltam usam `item_height` ou `item_width`). Só na lista (uma coluna). |

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `item_count` | Quantidade de itens | inteiro | Quantos itens (com uma fonte, vem dela). |
| `item_form` | Form de cada item | texto | Um form do projeto como modelo (vazio: o primeiro filho da lista). |
| `bindings` | Ligações | texto | `"lblNome=nome;imgFoto=foto"`: componente do modelo = campo da fonte. |

### Dados

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `data_source` | Fonte de dados | componente |  |

### Posição e tamanho

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `orientation` | Orientação | um de: `vertical`, `horizontal` |  |
| `columns` | Colunas | inteiro | Grade: quantos itens lado a lado (na horizontal, um embaixo do outro). 1 = lista. |

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
