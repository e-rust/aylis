# TickerTape

![TickerTape](img/TickerTape.png)

Faixa de cotações rolando (como no rodapé dos canais de notícias). `items` = as cotações em JSON; `speed` em pontos por segundo; para enquanto o mouse está em cima.

**Paleta:** Trader · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `decimals` | Casas decimais | inteiro | Casas decimais dos preços. |

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `speed` | Velocidade | número | A velocidade da rolagem, em pontos por segundo. |

### Dados

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `items` | Itens | lista de `TickerQuote` | As cotações, em JSON (`TICKER_QUOTE`). |

## Em código

No form, o `TickerTape1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "TickerTape", "name": "TickerTape1" }
```

E o handler, um método do form (`self.TickerTape1` já é um `TickerTape`):

```rust
impl FrmPrincipal {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.TickerTape1.speed(ui);
        self.TickerTape1.set_speed(ui, atual + 10.0);
    }
}
```

(Sem eventos próprios: o exemplo usa o `on_show` do form.)
