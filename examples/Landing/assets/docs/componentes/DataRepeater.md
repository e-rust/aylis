# DataRepeater

![DataRepeater](img/DataRepeater.png)

Lista virtual de registros: `data_source` dá a quantidade e os valores; `item_bindings` (`"lblNome=nome;lblPreco=preco"`) liga os componentes do modelo aos campos.

**Paleta:** Data Controls · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `item_height` | Altura do item | número | A altura de cada item, em pontos (na vertical). |
| `item_width` | Largura do item | número | Na horizontal: a largura de cada item. |
| `sizes` | Tamanhos da câmera | lista de números (separados por `;`) | Tamanhos próprios dos itens, em ordem (`80;120;80`; os que faltam usam `item_height` ou `item_width`). Só na lista (uma coluna). |
| `scroll_bar` | Barra de rolagem | um de: `none`, `auto`, `always` | A barra de rolagem: `none` (como antes), `auto` ou `always`. |

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `overscroll` | Fim da rolagem | um de: `auto`, `glow`, `stretch`, `bounce`, `none` | O efeito de fim da rolagem no toque (`auto`: o do sistema; veja `aylis_core::overscroll`). |
| `item_count` | Quantidade de itens | inteiro | Quantos itens (com uma fonte, vem dela). |
| `item_form` | Form de cada item | nome de um form do projeto | Um form do projeto como modelo (vazio: o primeiro filho da lista). |
| `item_bindings` | Ligações do item | lista de `ItemBinding` | O que cada componente do modelo mostra da fonte: a lista de `ITEM_BINDING` em JSON (`[{"component": "lblNome", "field": "nome"}, {"component": "pinEstado", "property": "fill", "field": "cor"}]`). Sem `property`: o texto, a imagem, o valor ou a marca (a primeira que o componente tiver). |
| `swipe_actions` | Ações ao deslizar | texto | As ações que aparecem ao deslizar uma linha para a esquerda com o dedo (`Arquivar;Excluir`; vazio: sem). O toque numa dispara `on_swipe_action`. |
| `selection` | Seleção | um de: `none`, `single`, `multiple` | Quantas linhas dá para escolher (`none`: nenhuma, como antes). |
| `kinds` | Modelos dos itens | lista de textos (separados por `;`) | O modelo de cada item, pelo nome do filho (`pnlMusica;pnlMusica;pnlSecao`; o que falta e o nome desconhecido: o primeiro). Como o `sizes`, uma lista por item. |
| `kind_field` | Campo do modelo | texto | O campo da fonte (o `DataRepeater`) com o nome do modelo de cada item, no lugar de `kinds`. |

### Dados

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `data_source` | Fonte de dados | componente | O `DataSource` dos registros: um item por registro, com os campos de `item_bindings`. |
| `item_index` | Item escolhido | inteiro | O item escolhido (-1: nenhum); com `multiple`, o último escolhido. |
| `selected` | Escolhidos | lista de números (separados por `;`) | Com `multiple`, os itens escolhidos, em ordem (`2;5;7`). |

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
| `on_change` | o valor mudou (pelo usuário ou pelo código) | — |  |
| `on_item_click` | — | — |  |
| `on_item_dbl_click` | — | — |  |
| `on_item_activate` | — | — |  |

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
