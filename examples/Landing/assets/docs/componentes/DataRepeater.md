# DataRepeater

![DataRepeater](img/DataRepeater.png)

Lista virtual de registros: `data_source` dá a quantidade e os valores; `bindings` (`"lblNome=nome;lblPreco=preco"`) liga os componentes do modelo aos campos.

**Paleta:** Data Controls · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `item_height` | Altura do item | número | A altura de cada item, em pontos (na vertical). |
| `item_width` | Largura do item | número | Na horizontal: a largura de cada item. |
| `sizes` | Tamanhos da câmera | texto | Tamanhos próprios dos itens, em ordem (`80;120;80`; os que faltam usam `item_height` ou `item_width`). Só na lista (uma coluna). |

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `overscroll` | Fim da rolagem | um de: `auto`, `glow`, `stretch`, `bounce`, `none` | O efeito de fim da rolagem no toque (`auto`: o do sistema; veja `aylis_core::overscroll`). |
| `item_count` | Quantidade de itens | inteiro | Quantos itens (com uma fonte, vem dela). |
| `item_form` | Form de cada item | texto | Um form do projeto como modelo (vazio: o primeiro filho da lista). |
| `bindings` | Ligações | texto | `"lblNome=nome;imgFoto=foto"`: componente do modelo = campo da fonte. |
| `swipe_actions` | Ações ao deslizar | texto | As ações que aparecem ao deslizar uma linha para a esquerda com o dedo (`Arquivar;Excluir`; vazio: sem). O toque numa dispara `on_swipe_action`. |

### Dados

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `data_source` | Fonte de dados | componente | O `DataSource` dos registros: um item por registro, com os campos de `bindings`. |

### Posição e tamanho

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `orientation` | Orientação | um de: `vertical`, `horizontal` | Para onde a lista rola: `vertical` (um embaixo do outro) ou `horizontal` (lado a lado). |
| `columns` | Colunas | inteiro | Grade: quantos itens lado a lado (na horizontal, um embaixo do outro). 1 = lista. |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_bind` | — | — |  |
| `on_load_more` | — | — |  |
| `on_swipe_action` | o toque numa ação da linha deslizada (`swipe_actions`): o item (`index`) e a ação (`action`) | — |  |

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
