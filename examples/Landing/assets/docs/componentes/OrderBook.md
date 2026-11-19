# OrderBook

![OrderBook](img/OrderBook.png)

Livro de ofertas: as vendas (`asks`, do menor preço para cima) em cima, as compras (`bids`) embaixo, o spread no meio; a barra de cada linha mostra o acumulado. Clique numa linha: `on_price_click` com o preço (para a boleta).

**Paleta:** Trader · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `levels` | Níveis | inteiro | Quantos preços aparecem de cada lado. |
| `decimals` | Casas decimais | inteiro | Casas decimais dos preços. |

### Dados

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `bids` | Ofertas de compra | lista de `OrderLevel` | As ofertas de compra, em JSON (`ORDER_LEVEL`): `[{"price": 38.49, "quantity": 1200}]`. |
| `asks` | Ofertas de venda | lista de `OrderLevel` | As ofertas de venda, no mesmo formato das `bids`. |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_price_click` | — | — |  |

## Em código

No form, o `OrderBook1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "OrderBook", "name": "OrderBook1", "on_price_click": "order_book1_price_click" }
```

E o handler, um método do form (`self.OrderBook1` já é um `OrderBook`):

```rust
impl FrmPrincipal {
    pub fn order_book1_price_click(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.OrderBook1.levels(ui);
        self.OrderBook1.set_levels(ui, atual + 1);
    }
}
```
