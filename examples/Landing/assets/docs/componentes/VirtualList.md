# VirtualList

![VirtualList](img/VirtualList.png)

Lista virtual com modelo de item e reciclagem.

**Paleta:** Additional · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

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

No form, o `VirtualList1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "VirtualList", "name": "VirtualList1", "on_bind": "virtual_list1_bind" }
```

E o handler, um método do form (`self.VirtualList1` já é um `VirtualList`):

```rust
impl FrmPrincipal {
    pub fn virtual_list1_bind(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.VirtualList1.item_count(ui);
        self.VirtualList1.set_item_count(ui, atual + 1);
    }
}
```
