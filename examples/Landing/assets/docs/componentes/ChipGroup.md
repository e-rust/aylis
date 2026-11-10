# ChipGroup

![ChipGroup](img/ChipGroup.png)

Grupo de chips para escolher (o `v-chip-group`): uma opção ou várias (`multiple`). `selected` são os itens escolhidos, separados por `;`. Quebra a linha sozinho; com `wrap` desligado fica numa linha só e rola para os lados (o `v-slide-group`): roda do mouse, arrasto no toque e setas nas pontas.

**Paleta:** Standard · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `wrap` | Quebra de linha | lógico |
| `chip_height` | Altura dos chips | número |

### Comportamento

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `multiple` | Vários | lógico |

### Dados

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `items` | Itens | texto |
| `selected` | Escolhidos | texto |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_change` | o valor mudou (pelo usuário ou pelo código, como na VCL) | — |  |

## Em código

No form, o `ChipGroup1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "ChipGroup", "name": "ChipGroup1", "on_change": "chip_group1_change" }
```

E o handler, um método do form (`self.ChipGroup1` já é um `ChipGroup`):

```rust
impl FrmPrincipal {
    pub fn chip_group1_change(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.ChipGroup1.items(ui);
        self.ChipGroup1.set_items(ui, format!("{atual}!"));
    }
}
```
