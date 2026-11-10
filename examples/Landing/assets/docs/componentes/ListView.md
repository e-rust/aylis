# ListView

![ListView](img/ListView.png)

Lista de itens com título, detalhe e imagem opcional, pensada também para o celular (linhas altas, rolagem com o dedo). `items` no texto: `Título|detalhe;Outro|mais`. `on_change` traz o índice; `on_item_click` também.

**Paleta:** Additional · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `item_height` | Altura do item | número |
| `accessory` | Seta à direita | lógico |

### Dados

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `items` | Itens | texto |
| `item_index` | Item escolhido | inteiro |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_change` | o valor mudou (pelo usuário ou pelo código, como na VCL) | — |  |
| `on_item_click` | — | — |  |

## Em código

No form, o `ListView1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "ListView", "name": "ListView1", "on_change": "list_view1_change" }
```

E o handler, um método do form (`self.ListView1` já é um `ListView`):

```rust
impl FrmPrincipal {
    pub fn list_view1_change(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.ListView1.items(ui);
        self.ListView1.set_items(ui, format!("{atual}!"));
    }
}
```
