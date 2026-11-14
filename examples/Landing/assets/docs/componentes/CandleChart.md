# CandleChart

![CandleChart](img/CandleChart.png)

Gráfico de candles (OHLC) com volume, indicadores e ferramentas de desenho.

**Paleta:** Trader · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `lower` | Indicador de baixo | um de: `none`, `volume`, `rsi`, `macd` | O painel de baixo: `none`, `volume`, `rsi` ou `macd`. |
| `volume_profile` | Perfil de volume | lógico | Mostra o volume por faixa de preço, em barras da direita para dentro. |
| `decimals` | Casas decimais | inteiro | Casas decimais dos preços. |

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `indicators` | Indicadores | texto | Os indicadores sobre o preço, separados por `;`: `"sma:20;ema:9;bollinger:20"`. |
| `draw_mode` | Modo de desenho | um de: `pan`, `trend`, `fibonacci` | O que arrastar faz: `pan` (anda no tempo), `trend` (desenha uma linha de tendência) ou `fibonacci` (desenha uma retração). |

### Dados

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `data` | Dados | texto | Os candles, um por linha: `tempo;abertura;máxima;mínima;fechamento;volume`. |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_drawing` | — | — |  |

## Em código

No form, o `CandleChart1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "CandleChart", "name": "CandleChart1", "on_drawing": "candle_chart1_drawing" }
```

E o handler, um método do form (`self.CandleChart1` já é um `CandleChart`):

```rust
impl FrmPrincipal {
    pub fn candle_chart1_drawing(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.CandleChart1.data(ui);
        self.CandleChart1.set_data(ui, format!("{atual}!"));
    }
}
```
