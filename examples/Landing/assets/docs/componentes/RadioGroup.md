# RadioGroup

![RadioGroup](img/RadioGroup.png)

Grupo de opções exclusivas com título e moldura. `items` separados por `;`, `item_index` escolhido, `columns` colunas. `on_change` traz o texto.

**Paleta:** Standard · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `caption` | Título | texto | O título na moldura. |
| `columns` | Colunas | inteiro | Em quantas colunas as opções se distribuem. |

### Dados

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `items` | Itens | texto | As opções, separadas por `;`. |
| `item_index` | Item escolhido | inteiro | A opção escolhida, a partir de 0; -1: nenhuma. |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_change` | o valor mudou (pelo usuário ou pelo código) | — |  |

## Em código

No form, o `RadioGroup1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "RadioGroup", "name": "RadioGroup1", "on_change": "radio_group1_change" }
```

E o handler, um método do form (`self.RadioGroup1` já é um `RadioGroup`):

```rust
impl FrmPrincipal {
    pub fn radio_group1_change(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.RadioGroup1.caption(ui);
        self.RadioGroup1.set_caption(ui, format!("{atual}!"));
    }
}
```
