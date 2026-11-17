# TreeView

![TreeView](img/TreeView.png)

Árvore, virtualizada: só as linhas visíveis são pintadas.

**Paleta:** Additional · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `item_height` | Altura do item | número | Altura de cada linha, em pontos. |
| `leaf_layout` | Layout das folhas | um de: `aligned`, `icon_in_arrow`, `compact` | Onde ficam o ícone e o texto dos nós sem filhos (que não têm seta): `aligned` (o padrão: o texto na coluna dos irmãos com filhos), `icon_in_arrow` (o ícone no lugar da seta, o texto na coluna dos irmãos) ou `compact` (sem o espaço da seta: o ícone e o texto começam no recuo do nível). |
| `indent` | Recuo | número | O recuo de cada nível, que é também a largura da seta, em pontos. |
| `icon_size` | Tamanho do ícone | número | O tamanho do ícone dos nós, em pontos. |
| `icon_gap` | Espaço do ícone | número | O espaço entre o ícone e o texto, em pontos. |

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `overscroll` | Fim da rolagem | um de: `auto`, `glow`, `stretch`, `bounce`, `none` | O efeito de fim da rolagem no toque (`auto`: o do sistema; veja `aylis_core::overscroll`). |

### Dados

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `items` | Itens | texto | Os nós, separados por `;`, com um `>` na frente por nível e o ícone depois de `\|` (`Brasil\|flag;>São Paulo;>>Campinas;Portugal`). |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_change` | o valor mudou (pelo usuário ou pelo código) | — |  |
| `on_expand` | — | — |  |
| `on_collapse` | — | — |  |

## Em código

No form, o `TreeView1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "TreeView", "name": "TreeView1", "on_change": "tree_view1_change" }
```

E o handler, um método do form (`self.TreeView1` já é um `TreeView`):

```rust
impl FrmPrincipal {
    pub fn tree_view1_change(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.TreeView1.items(ui);
        self.TreeView1.set_items(ui, format!("{atual}!"));
    }
}
```
