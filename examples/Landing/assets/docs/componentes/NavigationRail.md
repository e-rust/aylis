# NavigationRail

![NavigationRail](img/NavigationRail.png)

Trilho de navegação (o "navigation rail" do Material 3, para telas largas): ícones com o nome embaixo, o escolhido com a pílula; um FAB opcional em cima (`fab_icon`, `on_fab`). `item_index`, `on_change`.

**Paleta:** Layouts · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `icons` | Ícones | texto | Os ícones dos destinos, na mesma ordem dos `items`, separados por `;`. |
| `fab_icon` | Ícone do botão flutuante | texto | O ícone do botão de ação em cima (dispara `on_fab`); vazio: sem botão. |

### Dados

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `items` | Itens | texto | Os nomes dos destinos, separados por `;` (`Início;Buscar;Perfil`). |
| `item_index` | Item escolhido | inteiro | O destino escolhido (0 = o primeiro; -1 = nenhum). |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_change` | o valor mudou (pelo usuário ou pelo código) | — |  |
| `on_fab` | — | — |  |

## Em código

No form, o `NavigationRail1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "NavigationRail", "name": "NavigationRail1", "on_change": "navigation_rail1_change" }
```

E o handler, um método do form (`self.NavigationRail1` já é um `NavigationRail`):

```rust
impl FrmPrincipal {
    pub fn navigation_rail1_change(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.NavigationRail1.items(ui);
        self.NavigationRail1.set_items(ui, format!("{atual}!"));
    }
}
```
