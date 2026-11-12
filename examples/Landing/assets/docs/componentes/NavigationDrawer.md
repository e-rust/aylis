# NavigationDrawer

![NavigationDrawer](img/NavigationDrawer.png)

Gaveta de navegação: fica à esquerda (`align` Left), aberta ou fechada (`open`); fechada em modo `rail` mostra só os ícones. Com `items`/`icons` desenha a lista de destinos (`item_index`, `on_change`); também aceita controles dentro.

**Paleta:** Layouts · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `title` | Título | texto |  |
| `icons` | Ícones | texto |  |

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `open` | Aberto | lógico |  |
| `rail` | Só ícones | lógico |  |

### Dados

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `items` | Itens | texto |  |
| `item_index` | Item escolhido | inteiro |  |

### Posição e tamanho

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `drawer_width` | Largura da gaveta | número |  |
| `rail_width` | Largura só com ícones | número |  |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_change` | o valor mudou (pelo usuário ou pelo código, como na VCL) | — |  |

## Em código

No form, o `NavigationDrawer1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "NavigationDrawer", "name": "NavigationDrawer1", "on_change": "navigation_drawer1_change" }
```

E o handler, um método do form (`self.NavigationDrawer1` já é um `NavigationDrawer`):

```rust
impl FrmPrincipal {
    pub fn navigation_drawer1_change(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.NavigationDrawer1.open(ui);
        self.NavigationDrawer1.set_open(ui, !atual);
    }
}
```
